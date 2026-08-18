use fearless_simd::{Simd, SimdBase, SimdMask, u8x16, u8x32};

use crate::alloc::{Allocator, SliceWrapper, SliceWrapperMut};
use crate::enc::combined_alloc::allocate;
use crate::enc::static_dict::{
    BROTLI_UNALIGNED_LOAD32, BROTLI_UNALIGNED_LOAD64, BrotliDictionary, FindMatchLengthWithLimit,
};
use crate::enc::vectorization::detect_level;

use super::{
    AnyHasher, BackwardReferencePenaltyUsingLastDistance, BackwardReferenceScore,
    BackwardReferenceScoreUsingLastDistance, BrotliHasherParams, CloneWithAlloc,
    HasherSearchResult, HowPrepared, SearchInStaticDictionary, Struct1, fix_unbroken_len,
    kHashMul32, kHashMul64,
};

const TAG_BITS: u32 = 8;
const TAG_MASK: usize = (1 << TAG_BITS) - 1;

pub trait TaggedHashSpecialization: Clone + PartialEq {
    fn hash(&self, data: &[u8]) -> usize;
    fn hash_type_length(&self) -> usize;
    fn block_bits(&self) -> u32;
    fn bucket_bits(&self) -> u32;
    fn compare_from_four(&self) -> bool;

    #[inline(always)]
    fn block_size(&self) -> usize {
        1usize << self.block_bits()
    }

    #[inline(always)]
    fn block_mask(&self) -> usize {
        self.block_size() - 1
    }

    #[inline(always)]
    fn bucket_size(&self) -> usize {
        1usize << self.bucket_bits()
    }
}

#[derive(Clone, PartialEq)]
pub struct H58Sub {
    pub block_bits: u32,
    pub bucket_bits: u32,
}

impl TaggedHashSpecialization for H58Sub {
    #[inline(always)]
    fn hash(&self, data: &[u8]) -> usize {
        let shift = 32 - self.bucket_bits - TAG_BITS;
        (BROTLI_UNALIGNED_LOAD32(data).wrapping_mul(kHashMul32) >> shift) as usize
    }

    #[inline(always)]
    fn hash_type_length(&self) -> usize {
        4
    }

    #[inline(always)]
    fn block_bits(&self) -> u32 {
        self.block_bits
    }

    #[inline(always)]
    fn bucket_bits(&self) -> u32 {
        self.bucket_bits
    }

    #[inline(always)]
    fn compare_from_four(&self) -> bool {
        false
    }
}

#[derive(Clone, PartialEq)]
pub struct H68Sub {
    pub block_bits: u32,
}

impl TaggedHashSpecialization for H68Sub {
    #[inline(always)]
    fn hash(&self, data: &[u8]) -> usize {
        const BUCKET_BITS: u32 = 15;
        let hash_mul = kHashMul64.wrapping_shl(64 - 5 * 8);
        (BROTLI_UNALIGNED_LOAD64(data).wrapping_mul(hash_mul) >> (64 - BUCKET_BITS - TAG_BITS))
            as usize
    }

    #[inline(always)]
    fn hash_type_length(&self) -> usize {
        8
    }

    #[inline(always)]
    fn block_bits(&self) -> u32 {
        self.block_bits
    }

    #[inline(always)]
    fn bucket_bits(&self) -> u32 {
        15
    }

    #[inline(always)]
    fn compare_from_four(&self) -> bool {
        true
    }
}

pub struct TaggedHasher<
    Specialization: TaggedHashSpecialization,
    Alloc: Allocator<u8> + Allocator<u16> + Allocator<u32>,
> {
    pub common: Struct1,
    pub specialization: Specialization,
    pub num: <Alloc as Allocator<u16>>::AllocatedMemory,
    pub tags: <Alloc as Allocator<u8>>::AllocatedMemory,
    pub buckets: <Alloc as Allocator<u32>>::AllocatedMemory,
    pub h9_opts: super::H9Opts,
}

impl<
    Specialization: TaggedHashSpecialization,
    Alloc: Allocator<u8> + Allocator<u16> + Allocator<u32>,
> PartialEq for TaggedHasher<Specialization, Alloc>
{
    fn eq(&self, other: &Self) -> bool {
        self.common == other.common
            && self.specialization == other.specialization
            && self.num.slice() == other.num.slice()
            && self.tags.slice() == other.tags.slice()
            && self.buckets.slice() == other.buckets.slice()
            && self.h9_opts == other.h9_opts
    }
}

impl<
    Specialization: TaggedHashSpecialization,
    Alloc: Allocator<u8> + Allocator<u16> + Allocator<u32>,
> TaggedHasher<Specialization, Alloc>
{
    pub fn new(
        alloc: &mut Alloc,
        params: &BrotliHasherParams,
        specialization: Specialization,
    ) -> Self {
        let bucket_size = specialization.bucket_size();
        let table_size = bucket_size * specialization.block_size();
        let mut num = allocate::<u16, _>(alloc, bucket_size);
        num.slice_mut().fill(u16::MAX);
        Self {
            common: Struct1 {
                params: *params,
                is_prepared_: 1,
                dict_num_lookups: 0,
                dict_num_matches: 0,
            },
            specialization,
            num,
            tags: allocate::<u8, _>(alloc, table_size),
            buckets: allocate::<u32, _>(alloc, table_size),
            h9_opts: super::H9Opts::new(params),
        }
    }

    #[inline(always)]
    fn hash_parts(&self, data: &[u8]) -> (usize, u8) {
        let hash = self.specialization.hash(data);
        (hash >> TAG_BITS, (hash & TAG_MASK) as u8)
    }

    #[inline(always)]
    fn store(&mut self, data: &[u8], mask: usize, ix: usize) {
        let (key, tag) = self.hash_parts(&data[ix & mask..]);
        let block_mask = self.specialization.block_mask();
        let offset = (key << self.specialization.block_bits())
            + (self.num.slice()[key] as usize & block_mask);
        self.num.slice_mut()[key] = self.num.slice()[key].wrapping_sub(1);
        self.buckets.slice_mut()[offset] = ix as u32;
        self.tags.slice_mut()[offset] = tag;
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn find_longest_match_simd<S: Simd>(
        &mut self,
        simd: S,
        dictionary: Option<&BrotliDictionary>,
        dictionary_hash: &[u16],
        data: &[u8],
        ring_buffer_mask: usize,
        ring_buffer_break: Option<core::num::NonZeroUsize>,
        distance_cache: &[i32],
        cur_ix: usize,
        max_length: usize,
        max_backward: usize,
        gap: usize,
        max_distance: usize,
        out: &mut HasherSearchResult,
    ) -> bool {
        let cur_ix_masked = cur_ix & ring_buffer_mask;
        let cur_data = &data[cur_ix_masked..];
        let min_score = out.score;
        let mut best_score = out.score;
        let mut best_len = out.len;
        let mut is_match_found = false;
        out.len = 0;
        out.len_x_code = 0;

        for i in 0..self.common.params.num_last_distances_to_check as usize {
            let backward = distance_cache[i] as usize;
            let mut prev_ix = cur_ix.wrapping_sub(backward);
            if prev_ix >= cur_ix || backward > max_backward {
                continue;
            }
            prev_ix &= ring_buffer_mask;
            if cur_ix_masked + best_len > ring_buffer_mask
                || prev_ix + best_len > ring_buffer_mask
                || cur_data[best_len] != data[prev_ix + best_len]
            {
                continue;
            }
            let unbroken_len = FindMatchLengthWithLimit(&data[prev_ix..], cur_data, max_length);
            if unbroken_len >= 3 || (unbroken_len == 2 && i < 2) {
                let len = fix_unbroken_len(unbroken_len, prev_ix, cur_ix_masked, ring_buffer_break);
                let mut score = BackwardReferenceScoreUsingLastDistance(len, self.h9_opts);
                if best_score < score {
                    if i != 0 {
                        score = score.wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i));
                    }
                    if best_score < score {
                        best_score = score;
                        best_len = len;
                        out.len = len;
                        out.distance = backward;
                        out.score = score;
                        is_match_found = true;
                    }
                }
            }
        }

        best_len = best_len.max(3);
        let (key, tag) = self.hash_parts(cur_data);
        let block_bits = self.specialization.block_bits();
        let block_size = self.specialization.block_size();
        let block_mask = self.specialization.block_mask();
        let block_start = key << block_bits;
        let num = self.num.slice()[key];
        let head = num.wrapping_add(1) as usize & block_mask;
        let mut matches = matching_tag_mask(
            simd,
            tag,
            &self.tags.slice()[block_start..block_start + block_size],
            head,
        );
        let stored = u16::MAX.wrapping_sub(num) as usize;
        if stored < block_size {
            matches &= (1u64 << stored) - 1;
        }

        while matches != 0 {
            let rb_index = (head + matches.trailing_zeros() as usize) & block_mask;
            matches &= matches - 1;
            let mut prev_ix = self.buckets.slice()[block_start + rb_index] as usize;
            let backward = cur_ix.wrapping_sub(prev_ix);
            if backward > max_backward {
                break;
            }
            prev_ix &= ring_buffer_mask;
            if cur_ix_masked + best_len > ring_buffer_mask {
                break;
            }
            if prev_ix + best_len > ring_buffer_mask
                || BROTLI_UNALIGNED_LOAD32(&cur_data[best_len - 3..])
                    != BROTLI_UNALIGNED_LOAD32(&data[prev_ix + best_len - 3..])
            {
                continue;
            }

            let unbroken_len = if self.specialization.compare_from_four() {
                if BROTLI_UNALIGNED_LOAD32(cur_data) != BROTLI_UNALIGNED_LOAD32(&data[prev_ix..]) {
                    continue;
                }
                FindMatchLengthWithLimit(&data[prev_ix + 4..], &cur_data[4..], max_length - 4) + 4
            } else {
                FindMatchLengthWithLimit(&data[prev_ix..], cur_data, max_length)
            };
            if unbroken_len >= 4 {
                let len = fix_unbroken_len(unbroken_len, prev_ix, cur_ix_masked, ring_buffer_break);
                let score = BackwardReferenceScore(len, backward, self.h9_opts);
                if best_score < score {
                    best_score = score;
                    best_len = len;
                    out.len = len;
                    out.distance = backward;
                    out.score = score;
                    is_match_found = true;
                }
            }
        }

        let offset = block_start + (num as usize & block_mask);
        self.buckets.slice_mut()[offset] = cur_ix as u32;
        self.tags.slice_mut()[offset] = tag;
        self.num.slice_mut()[key] = num.wrapping_sub(1);

        if min_score == out.score && dictionary.is_some() {
            is_match_found = SearchInStaticDictionary(
                dictionary.unwrap(),
                dictionary_hash,
                self,
                cur_data,
                max_length,
                max_backward.wrapping_add(gap),
                max_distance,
                out,
                false,
            );
        }
        is_match_found
    }
}

#[cfg_attr(feature = "hotpath", hotpath::measure)]
fn matching_tag_mask<S: Simd>(simd: S, tag: u8, tags: &[u8], head: usize) -> u64 {
    match tags.len() {
        16 => {
            let mask = u8x16::from_slice(simd, tags)
                .simd_eq(u8x16::splat(simd, tag))
                .to_bitmask() as u16;
            mask.rotate_right(head as u32) as u64
        }
        32 => {
            let mask = u8x32::from_slice(simd, tags)
                .simd_eq(u8x32::splat(simd, tag))
                .to_bitmask() as u32;
            mask.rotate_right(head as u32) as u64
        }
        _ => {
            let mut mask = 0u64;
            for (index, &candidate) in tags.iter().enumerate() {
                mask |= u64::from(candidate == tag) << index;
            }
            let width_mask = (1u64 << tags.len()) - 1;
            ((mask >> head) | (mask << (tags.len() - head))) & width_mask
        }
    }
}

impl<
    Specialization: TaggedHashSpecialization,
    Alloc: Allocator<u8> + Allocator<u16> + Allocator<u32>,
> AnyHasher for TaggedHasher<Specialization, Alloc>
{
    #[inline(always)]
    fn Opts(&self) -> super::H9Opts {
        self.h9_opts
    }

    #[inline(always)]
    fn GetHasherCommon(&mut self) -> &mut Struct1 {
        &mut self.common
    }

    #[inline(always)]
    fn HashBytes(&self, data: &[u8]) -> usize {
        self.specialization.hash(data) >> TAG_BITS
    }

    #[inline(always)]
    fn HashTypeLength(&self) -> usize {
        self.specialization.hash_type_length()
    }

    #[inline(always)]
    fn StoreLookahead(&self) -> usize {
        self.specialization.hash_type_length()
    }

    fn PrepareDistanceCache(&self, distance_cache: &mut [i32]) {
        super::adv_prepare_distance_cache(
            distance_cache,
            self.common.params.num_last_distances_to_check,
        );
    }

    fn FindLongestMatch(
        &mut self,
        dictionary: Option<&BrotliDictionary>,
        dictionary_hash: &[u16],
        data: &[u8],
        ring_buffer_mask: usize,
        ring_buffer_break: Option<core::num::NonZeroUsize>,
        distance_cache: &[i32],
        cur_ix: usize,
        max_length: usize,
        max_backward: usize,
        gap: usize,
        max_distance: usize,
        out: &mut HasherSearchResult,
    ) -> bool {
        dispatch!(detect_level(), simd => self.find_longest_match_simd(
            simd, dictionary, dictionary_hash, data, ring_buffer_mask, ring_buffer_break,
            distance_cache, cur_ix, max_length, max_backward, gap, max_distance, out
        ))
    }

    fn Store(&mut self, data: &[u8], mask: usize, ix: usize) {
        self.store(data, mask, ix);
    }

    fn StoreRange(&mut self, data: &[u8], mask: usize, ix_start: usize, ix_end: usize) {
        for ix in ix_start..ix_end {
            self.store(data, mask, ix);
        }
    }

    fn BulkStoreRange(&mut self, data: &[u8], mask: usize, ix_start: usize, ix_end: usize) {
        self.StoreRange(data, mask, ix_start, ix_end);
    }

    fn Prepare(&mut self, one_shot: bool, input_size: usize, data: &[u8]) -> HowPrepared {
        if self.common.is_prepared_ != 0 {
            return HowPrepared::ALREADY_PREPARED;
        }
        let partial_prepare_threshold = self.specialization.bucket_size() >> 6;
        if one_shot && input_size <= partial_prepare_threshold {
            for ix in 0..input_size {
                let key = self.HashBytes(&data[ix..]);
                self.num.slice_mut()[key] = u16::MAX;
            }
        } else {
            self.num.slice_mut().fill(u16::MAX);
        }
        self.common.is_prepared_ = 1;
        HowPrepared::NEWLY_PREPARED
    }

    fn StitchToPreviousBlock(
        &mut self,
        num_bytes: usize,
        position: usize,
        ringbuffer: &[u8],
        ringbuffer_mask: usize,
    ) {
        super::StitchToPreviousBlockInternal(
            self,
            num_bytes,
            position,
            ringbuffer,
            ringbuffer_mask,
        );
    }
}

pub struct TaggedHasherSimd<'a, S: Simd, Specialization, Alloc>
where
    Specialization: TaggedHashSpecialization,
    Alloc: Allocator<u8> + Allocator<u16> + Allocator<u32>,
{
    simd: S,
    hasher: &'a mut TaggedHasher<Specialization, Alloc>,
}

impl<'a, S: Simd, Specialization, Alloc> TaggedHasherSimd<'a, S, Specialization, Alloc>
where
    Specialization: TaggedHashSpecialization,
    Alloc: Allocator<u8> + Allocator<u16> + Allocator<u32>,
{
    pub fn new(simd: S, hasher: &'a mut TaggedHasher<Specialization, Alloc>) -> Self {
        Self { simd, hasher }
    }
}

impl<S: Simd, Specialization, Alloc> AnyHasher for TaggedHasherSimd<'_, S, Specialization, Alloc>
where
    Specialization: TaggedHashSpecialization,
    Alloc: Allocator<u8> + Allocator<u16> + Allocator<u32>,
{
    fn Opts(&self) -> super::H9Opts {
        self.hasher.Opts()
    }
    fn GetHasherCommon(&mut self) -> &mut Struct1 {
        self.hasher.GetHasherCommon()
    }
    fn HashBytes(&self, data: &[u8]) -> usize {
        self.hasher.HashBytes(data)
    }
    fn HashTypeLength(&self) -> usize {
        self.hasher.HashTypeLength()
    }
    fn StoreLookahead(&self) -> usize {
        self.hasher.StoreLookahead()
    }
    fn PrepareDistanceCache(&self, distance_cache: &mut [i32]) {
        self.hasher.PrepareDistanceCache(distance_cache)
    }
    fn FindLongestMatch(
        &mut self,
        dictionary: Option<&BrotliDictionary>,
        dictionary_hash: &[u16],
        data: &[u8],
        ring_buffer_mask: usize,
        ring_buffer_break: Option<core::num::NonZeroUsize>,
        distance_cache: &[i32],
        cur_ix: usize,
        max_length: usize,
        max_backward: usize,
        gap: usize,
        max_distance: usize,
        out: &mut HasherSearchResult,
    ) -> bool {
        self.hasher.find_longest_match_simd(
            self.simd,
            dictionary,
            dictionary_hash,
            data,
            ring_buffer_mask,
            ring_buffer_break,
            distance_cache,
            cur_ix,
            max_length,
            max_backward,
            gap,
            max_distance,
            out,
        )
    }
    fn Store(&mut self, data: &[u8], mask: usize, ix: usize) {
        self.hasher.Store(data, mask, ix)
    }
    fn StoreRange(&mut self, data: &[u8], mask: usize, start: usize, end: usize) {
        self.hasher.StoreRange(data, mask, start, end)
    }
    fn BulkStoreRange(&mut self, data: &[u8], mask: usize, start: usize, end: usize) {
        self.hasher.BulkStoreRange(data, mask, start, end)
    }
    fn Prepare(&mut self, one_shot: bool, input_size: usize, data: &[u8]) -> HowPrepared {
        self.hasher.Prepare(one_shot, input_size, data)
    }
    fn StitchToPreviousBlock(&mut self, bytes: usize, pos: usize, data: &[u8], mask: usize) {
        self.hasher.StitchToPreviousBlock(bytes, pos, data, mask)
    }
}

impl<Specialization, Alloc> CloneWithAlloc<Alloc> for TaggedHasher<Specialization, Alloc>
where
    Specialization: TaggedHashSpecialization,
    Alloc: Allocator<u8> + Allocator<u16> + Allocator<u32>,
{
    fn clone_with_alloc(&self, alloc: &mut Alloc) -> Self {
        let mut num = allocate::<u16, _>(alloc, self.num.len());
        num.slice_mut().copy_from_slice(self.num.slice());
        let mut tags = allocate::<u8, _>(alloc, self.tags.len());
        tags.slice_mut().copy_from_slice(self.tags.slice());
        let mut buckets = allocate::<u32, _>(alloc, self.buckets.len());
        buckets.slice_mut().copy_from_slice(self.buckets.slice());
        Self {
            common: self.common.clone(),
            specialization: self.specialization.clone(),
            num,
            tags,
            buckets,
            h9_opts: self.h9_opts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_tag_mask_rotates_newest_candidate_to_bit_zero() {
        let mut tags = [0u8; 32];
        tags[3] = 7;
        tags[17] = 7;
        dispatch!(detect_level(), simd => {
            assert_eq!(matching_tag_mask(simd, 7, &tags, 3), 1 | (1 << 14));
        });
    }
}
