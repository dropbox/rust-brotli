#![allow(dead_code)]
use super::command::{Command, ComputeDistanceCode, InitCommand, BrotliDistanceParams};
use super::hash_to_binary_tree::{H10, H10Buckets, H10DefaultParams, ZopfliNode};
use super::dictionary_hash::kStaticDictionaryHash;
use super::static_dict::{BROTLI_UNALIGNED_LOAD32, BROTLI_UNALIGNED_LOAD64, FindMatchLengthWithLimit};
use super::static_dict::BrotliDictionary;
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::util::{Log2FloorNonZero, brotli_max_size_t, floatX};
use core;
static kBrotliMinWindowBits: i32 = 10i32;

static kBrotliMaxWindowBits: i32 = 24i32;

pub static kInvalidMatch: u32 = 0xfffffffu32;

static kCutoffTransformsCount: u32 = 10u32;

static kCutoffTransforms: u64 = 0x71b520au64 << 32i32 | 0xda2d3200u32 as (u64);

pub static kHashMul32: u32 = 0x1e35a7bdu32;

pub static kHashMul64: u64 = 0x1e35a7bdu64 << 32i32 | 0x1e35a7bdu64;

pub static kHashMul64Long: u64 = 0x1fe35a7bu32 as (u64) << 32i32 | 0xd3579bd3u32 as (u64);


#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(C)]
pub enum BrotliEncoderMode {
  BROTLI_MODE_GENERIC = 0,
  BROTLI_MODE_TEXT = 1,
  BROTLI_MODE_FONT = 2,
}


#[derive(Clone,Copy)]
pub struct BrotliHasherParams {
  // type of hasher to use (default: type 6, but others have tradeoffs of speed/memory)
  pub type_: i32,
  // number of the number of buckets to have in the hash table (defaults to quality - 1)
  pub bucket_bits: i32,
  // number of potential matches to hold per bucket (hash collisions)
  pub block_bits: i32,
  // number of bytes of a potential match to hash
  pub hash_len: i32,
  // number of previous distance matches to check for future matches (defaults to 16)
  pub num_last_distances_to_check: i32,
  // how much to weigh distance vs an extra byte of copy match when comparing possible copy srcs
  pub literal_byte_score: i32,
}


#[derive(Clone)]
pub struct BrotliEncoderParams {
  pub dist: BrotliDistanceParams,
  // if this brotli file is generic, font or specifically text
  pub mode: BrotliEncoderMode,
  // quality param between 0 and 11 (11 is smallest but takes longest to encode)
  pub quality: i32,
  // log of how big the ring buffer should be for copying prior data
  pub lgwin: i32,
  // log of how often metablocks should be serialized
  pub lgblock: i32,
  // how big the source file is (or 0 if no hint is provided)
  pub size_hint: usize,
  // avoid serializing out priors for literal sections in the favor of decode speed
  pub disable_literal_context_modeling: i32,
  pub hasher: BrotliHasherParams,
  // produce an IR of the compression file
  pub log_meta_block: bool,
  // attempt to detect how many bytes before the current byte generates the best prediction of it
  pub stride_detection_quality: u8, // 0 = off (stride 1 always) 1 = on per 16th of a file 2 = on per block type switch
  // if nonzero, will search for high entropy strings and log them differently to the IR
  pub high_entropy_detection_quality: u8, // search for high entropy literal strings and annotate them differently
  // if nonzero it will search for the temporal locality and effectiveness of the priors
  // for literals. The best adaptation and forgetfulness will be logged per metablock to the IR
  pub cdf_adaptation_detection: u8,
  // whether to search for whether the previous byte or the context_map are better predictors on a per-context-map basis
  pub prior_bitmask_detection: u8,
  // for prior bitmask detection: stride_low, stride_speed, cm_low, cm_speed
  pub literal_adaptation:[(u16, u16); 4],
}

impl Default for BrotliEncoderParams {
   fn default() -> BrotliEncoderParams {
      super::encode::BrotliEncoderInitParams()
   }
}

#[derive(Clone,Copy,Default)]
pub struct H9Opts{
   pub literal_byte_score: u32,
}
pub enum HowPrepared {
  ALREADY_PREPARED,
  NEWLY_PREPARED,
}
pub struct Struct1 {
  pub params: BrotliHasherParams,
  pub is_prepared_: i32,
  pub dict_num_lookups: usize,
  pub dict_num_matches: usize,
}

fn LiteralSpreeLengthForSparseSearch(params: &BrotliEncoderParams) -> usize {
  (if (*params).quality < 9 {
     64i32
   } else {
     512i32
   }) as (usize)
}

fn brotli_min_size_t(a: usize, b: usize) -> usize {
  if a < b { a } else { b }
}


pub struct HasherSearchResult {
  pub len: usize,
  pub len_x_code: usize,
  pub distance: usize,
  pub score: usize,
}

pub trait AnyHasher {
  fn Opts(&self) -> H9Opts;
  fn GetHasherCommon(&mut self) -> &mut Struct1;
  fn HashBytes(&self, data: &[u8]) -> usize;
  fn HashTypeLength(&self) -> usize;
  fn StoreLookahead(&self) -> usize;
  fn PrepareDistanceCache(&self, distance_cache: &mut [i32]);
  fn FindLongestMatch(&mut self,
                      dictionary: &BrotliDictionary,
                      dictionary_hash: &[u16],
                      data: &[u8],
                      ring_buffer_mask: usize,
                      distance_cache: &[i32],
                      cur_ix: usize,
                      max_length: usize,
                      max_backward: usize,
                      out: &mut HasherSearchResult)
                      -> bool;
  fn Store(&mut self, data: &[u8], mask: usize, ix: usize);
  fn StoreRange(&mut self, data: &[u8], mask: usize, ix_start: usize, ix_end: usize);
  fn Prepare(&mut self, one_shot: bool, input_size: usize, data: &[u8]) -> HowPrepared;
  fn StitchToPreviousBlock(&mut self,
                           num_bytes: usize,
                           position: usize,
                           ringbuffer: &[u8],
                           ringbuffer_mask: usize);
}


pub fn StitchToPreviousBlockInternal<T: AnyHasher>(handle: &mut T,
                                                   num_bytes: usize,
                                                   position: usize,
                                                   ringbuffer: &[u8],
                                                   ringbuffer_mask: usize) {
  if num_bytes >= handle.HashTypeLength().wrapping_sub(1) && (position >= 3) {
    handle.Store(ringbuffer, ringbuffer_mask, position.wrapping_sub(3));
    handle.Store(ringbuffer, ringbuffer_mask, position.wrapping_sub(2));
    handle.Store(ringbuffer, ringbuffer_mask, position.wrapping_sub(1));
  }
}

pub fn StoreLookaheadThenStore<T: AnyHasher>(hasher: &mut T, size: usize, dict: &[u8]) {
  let overlap = hasher.StoreLookahead().wrapping_sub(1usize);
  let mut i: usize = 0;
  while i.wrapping_add(overlap) < size {
    hasher.Store(dict, !(0usize), i);
    i = i.wrapping_add(1 as (usize));
  }
}

pub trait BasicHashComputer {
  fn HashBytes(&self, data: &[u8]) -> u32;
  fn BUCKET_BITS(&self) -> i32;
  fn USE_DICTIONARY(&self) -> i32;
  fn BUCKET_SWEEP(&self) -> i32;
}
pub struct BasicHasher<Buckets: SliceWrapperMut<u32> + SliceWrapper<u32> + BasicHashComputer> {
  pub GetHasherCommon: Struct1,
  pub buckets_: Buckets,
  pub h9_opts: H9Opts,
}
pub struct H2Sub<AllocU32: alloc::Allocator<u32>> {
  pub buckets_: AllocU32::AllocatedMemory, // 65537
}
impl<T: SliceWrapperMut<u32> + SliceWrapper<u32> + BasicHashComputer> AnyHasher for BasicHasher<T> {
  #[inline(always)]
  fn Opts(&self) -> H9Opts {
     self.h9_opts
  }
  #[allow(unused_variables)]
  fn PrepareDistanceCache(&self, distance_cache: &mut [i32]) {}
  #[inline(always)]
  fn HashTypeLength(&self) -> usize {
    8
  }
  #[inline(always)]
  fn StoreLookahead(&self) -> usize {
    8
  }
  fn StitchToPreviousBlock(&mut self,
                           num_bytes: usize,
                           position: usize,
                           ringbuffer: &[u8],
                           ringbuffer_mask: usize) {
    StitchToPreviousBlockInternal(self, num_bytes, position, ringbuffer, ringbuffer_mask);
  }
  #[inline(always)]
  fn GetHasherCommon(&mut self) -> &mut Struct1 {
    return &mut self.GetHasherCommon;
  }
  #[inline(always)]
  fn HashBytes(&self, data: &[u8]) -> usize {
    self.buckets_.HashBytes(data) as usize
  }
  fn Store(&mut self, data: &[u8], mask: usize, ix: usize) {
    let (_, data_window) = data.split_at((ix & mask) as (usize));
    let key: u32 = self.HashBytes(data_window) as u32;
    let off: u32 = (ix >> 3i32).wrapping_rem(self.buckets_.BUCKET_SWEEP() as usize) as (u32);
    self.buckets_.slice_mut()[key.wrapping_add(off) as (usize)] = ix as (u32);
  }
  fn StoreRange(&mut self, data: &[u8], mask: usize, ix_start: usize, ix_end: usize) {
    let mut i: usize;
    i = ix_start;
    while i < ix_end {
      {
        self.Store(data, mask, i);
      }
      i = i.wrapping_add(1 as (usize));
    }
  }
  fn Prepare(&mut self, one_shot: bool, input_size: usize, data: &[u8]) -> HowPrepared {
    if self.GetHasherCommon.is_prepared_ != 0 {
      return HowPrepared::ALREADY_PREPARED;
    }
    let partial_prepare_threshold = (4 << self.buckets_.BUCKET_BITS()) >> 7;
    if one_shot && input_size <= partial_prepare_threshold {
      for i in 0..input_size {
        let key = self.HashBytes(&data[i..]) as usize;
        let bs = self.buckets_.BUCKET_SWEEP() as usize;
        for item in self.buckets_.slice_mut()[key..(key + bs)].iter_mut() {
          *item = 0;
        }
      }
    } else {
      for item in self.buckets_.slice_mut().iter_mut() {
        *item = 0;
      }
    }
    self.GetHasherCommon.is_prepared_ = 1;
    HowPrepared::NEWLY_PREPARED
  }

  fn FindLongestMatch(&mut self,
                      dictionary: &BrotliDictionary,
                      dictionary_hash: &[u16],
                      data: &[u8],
                      ring_buffer_mask: usize,
                      distance_cache: &[i32],
                      cur_ix: usize,
                      max_length: usize,
                      max_backward: usize,
                      out: &mut HasherSearchResult)
                      -> bool {
    let opts = self.Opts();
    let best_len_in: usize = (*out).len;
    let cur_ix_masked: usize = cur_ix & ring_buffer_mask;
    let key: u32 = self.HashBytes(&data[(cur_ix_masked as (usize))..]) as u32;
    let mut compare_char: i32 = data[(cur_ix_masked.wrapping_add(best_len_in) as (usize))] as (i32);
    let mut best_score: usize = (*out).score;
    let mut best_len: usize = best_len_in;
    let cached_backward: usize = distance_cache[(0usize)] as (usize);
    let mut prev_ix: usize = cur_ix.wrapping_sub(cached_backward);
    let mut is_match_found: i32 = 0i32;
    (*out).len_x_code = 0usize;
    if prev_ix < cur_ix {
      prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
      if compare_char == data[(prev_ix.wrapping_add(best_len) as (usize))] as (i32) {
        let len: usize = FindMatchLengthWithLimit(&data[(prev_ix as (usize))..],
                                                  &data[(cur_ix_masked as (usize))..],
                                                  max_length);
        if len >= 4usize {
          best_score = BackwardReferenceScoreUsingLastDistance(len, opts);
          best_len = len;
          (*out).len = len;
          (*out).distance = cached_backward;
          (*out).score = best_score;
          compare_char = data[(cur_ix_masked.wrapping_add(best_len) as (usize))] as (i32);
          if self.buckets_.BUCKET_SWEEP() == 1i32 {
            (*self).buckets_.slice_mut()[key as (usize)] = cur_ix as (u32);
            return true;
          } else {
            is_match_found = 1i32;
          }
        }
      }
    }
    let BUCKET_SWEEP = self.buckets_.BUCKET_SWEEP();
    if BUCKET_SWEEP == 1i32 {
      let backward: usize;
      let len: usize;
      prev_ix = (*self).buckets_.slice()[key as (usize)] as (usize);
      (*self).buckets_.slice_mut()[key as (usize)] = cur_ix as (u32);
      backward = cur_ix.wrapping_sub(prev_ix);
      prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
      if compare_char != data[(prev_ix.wrapping_add(best_len_in) as (usize))] as (i32) {
        return false;
      }
      if backward == 0usize || backward > max_backward {
        return false;
      }
      len = FindMatchLengthWithLimit(&data[(prev_ix as (usize))..],
                                     &data[(cur_ix_masked as (usize))..],
                                     max_length);
      if len >= 4usize {
        (*out).len = len;
        (*out).distance = backward;
        (*out).score = BackwardReferenceScore(len, backward, opts);
        return true;
      }
    } else {
      let (old_, mut bucket) = (*self).buckets_.slice_mut()[key as usize..].split_at_mut(1);
      let mut i: i32;
      prev_ix = old_[0] as (usize);
      i = 0i32;

      while i < BUCKET_SWEEP {
        'continue3: loop {
          {
            let backward: usize = cur_ix.wrapping_sub(prev_ix);
            let len: usize;
            prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
            if compare_char != data[(prev_ix.wrapping_add(best_len) as (usize))] as (i32) {
              {
                break 'continue3;
              }
            }
            if backward == 0usize || backward > max_backward {
              {
                break 'continue3;
              }
            }
            len = FindMatchLengthWithLimit(&data[(prev_ix as (usize))..],
                                           &data[(cur_ix_masked as (usize))..],
                                           max_length);
            if len >= 4usize {
              let score: usize = BackwardReferenceScore(len, backward, opts);
              if best_score < score {
                best_score = score;
                best_len = len;
                (*out).len = best_len;
                (*out).distance = backward;
                (*out).score = score;
                compare_char = data[(cur_ix_masked.wrapping_add(best_len) as (usize))] as (i32);
                is_match_found = 1i32;
              }
            }
          }
          break;
        }
        i = i + 1;
        {
          let (_old, new_bucket) = core::mem::replace(&mut bucket, &mut []).split_at_mut(1);
          prev_ix = _old[0] as usize;
          bucket = new_bucket;
        }
      }
    }
    if self.buckets_.USE_DICTIONARY() != 0 && (is_match_found == 0) {
      is_match_found = SearchInStaticDictionary(dictionary,
                                                dictionary_hash,
                                                self,
                                                &data[(cur_ix_masked as (usize))..],
                                                max_length,
                                                max_backward,
                                                out,
                                                1i32);
    }
    (*self).buckets_.slice_mut()[(key as (usize)).wrapping_add((cur_ix >> 3)
                                    .wrapping_rem(BUCKET_SWEEP as usize))] = cur_ix as (u32);
    is_match_found != 0

  }
}
impl<AllocU32: alloc::Allocator<u32>> BasicHashComputer for H2Sub<AllocU32> {
  fn HashBytes(&self, data: &[u8]) -> u32 {
    let h: u64 = (BROTLI_UNALIGNED_LOAD64(data) << 64i32 - 8i32 * 5i32).wrapping_mul(kHashMul64);
    (h >> 64i32 - 16i32) as (u32)
  }
  fn BUCKET_BITS(&self) -> i32 {
    16
  }
  fn BUCKET_SWEEP(&self) -> i32 {
    1
  }
  fn USE_DICTIONARY(&self) -> i32 {
    1
  }
}
impl<AllocU32: alloc::Allocator<u32>> SliceWrapperMut<u32> for H2Sub<AllocU32> {
  fn slice_mut(&mut self) -> &mut [u32] {
    return self.buckets_.slice_mut();
  }
}
impl<AllocU32: alloc::Allocator<u32>> SliceWrapper<u32> for H2Sub<AllocU32> {
  fn slice(&self) -> &[u32] {
    return self.buckets_.slice();
  }
}
pub struct H3Sub<AllocU32: alloc::Allocator<u32>> {
  pub buckets_: AllocU32::AllocatedMemory, // 65538
}
impl<AllocU32: alloc::Allocator<u32>> SliceWrapperMut<u32> for H3Sub<AllocU32> {
  fn slice_mut(&mut self) -> &mut [u32] {
    return self.buckets_.slice_mut();
  }
}
impl<AllocU32: alloc::Allocator<u32>> SliceWrapper<u32> for H3Sub<AllocU32> {
  fn slice(&self) -> &[u32] {
    return self.buckets_.slice();
  }
}
impl<AllocU32: alloc::Allocator<u32>> BasicHashComputer for H3Sub<AllocU32> {
  fn BUCKET_BITS(&self) -> i32 {
    16
  }
  fn BUCKET_SWEEP(&self) -> i32 {
    2
  }
  fn USE_DICTIONARY(&self) -> i32 {
    0
  }
  fn HashBytes(&self, data: &[u8]) -> u32 {
    let h: u64 = (BROTLI_UNALIGNED_LOAD64(data) << 64i32 - 8i32 * 5i32).wrapping_mul(kHashMul64);
    (h >> 64i32 - 16i32) as (u32)
  }
}
pub struct H4Sub<AllocU32: alloc::Allocator<u32>> {
  pub buckets_: AllocU32::AllocatedMemory, // 131076
}
impl<AllocU32: alloc::Allocator<u32>> BasicHashComputer for H4Sub<AllocU32> {
  fn BUCKET_BITS(&self) -> i32 {
    17
  }
  fn BUCKET_SWEEP(&self) -> i32 {
    4
  }
  fn USE_DICTIONARY(&self) -> i32 {
    1
  }
  fn HashBytes(&self, data: &[u8]) -> u32 {
    let h: u64 = (BROTLI_UNALIGNED_LOAD64(data) << 64i32 - 8i32 * 5i32).wrapping_mul(kHashMul64);
    (h >> 64i32 - 17i32) as (u32)
  }
}
impl<AllocU32: alloc::Allocator<u32>> SliceWrapperMut<u32> for H4Sub<AllocU32> {
  fn slice_mut(&mut self) -> &mut [u32] {
    return self.buckets_.slice_mut();
  }
}
impl<AllocU32: alloc::Allocator<u32>> SliceWrapper<u32> for H4Sub<AllocU32> {
  fn slice(&self) -> &[u32] {
    return self.buckets_.slice();
  }
}
pub struct H54Sub<AllocU32: alloc::Allocator<u32>> {
  pub buckets_: AllocU32::AllocatedMemory,
}
impl<AllocU32: alloc::Allocator<u32>> BasicHashComputer for H54Sub<AllocU32> {
  fn BUCKET_BITS(&self) -> i32 {
    20
  }
  fn BUCKET_SWEEP(&self) -> i32 {
    4
  }
  fn USE_DICTIONARY(&self) -> i32 {
    0
  }
  fn HashBytes(&self, data: &[u8]) -> u32 {
    let h: u64 = (BROTLI_UNALIGNED_LOAD64(data) << 64i32 - 8i32 * 7i32).wrapping_mul(kHashMul64);
    (h >> 64i32 - 20i32) as (u32)
  }
}


impl<AllocU32: alloc::Allocator<u32>> SliceWrapperMut<u32> for H54Sub<AllocU32> {
  fn slice_mut(&mut self) -> &mut [u32] {
    return self.buckets_.slice_mut();
  }
}
impl<AllocU32: alloc::Allocator<u32>> SliceWrapper<u32> for H54Sub<AllocU32> {
  fn slice(&self) -> &[u32] {
    return self.buckets_.slice();
  }
}
pub const H9_BUCKET_BITS :usize = 15;
pub const H9_BLOCK_BITS :usize =8;
pub const H9_NUM_LAST_DISTANCES_TO_CHECK:usize = 16;
pub const H9_BLOCK_SIZE :usize= 1 << H9_BLOCK_BITS;
const H9_BLOCK_MASK :usize= (1 << H9_BLOCK_BITS) - 1;


impl H9Opts {
   pub fn new(params:&BrotliHasherParams) -> H9Opts {
      H9Opts {
         literal_byte_score: if params.literal_byte_score != 0 { params.literal_byte_score as u32} else {540},
      }
   }
}

pub struct H9<AllocU16: alloc::Allocator<u16>,
              AllocU32: alloc::Allocator<u32>> {
    pub num_:AllocU16::AllocatedMemory,//[u16;1 << H9_BUCKET_BITS],
    pub buckets_:AllocU32::AllocatedMemory,//[u32; H9_BLOCK_SIZE << H9_BUCKET_BITS],
    pub dict_search_stats_:Struct1,
    pub h9_opts: H9Opts,
}

fn adv_prepare_distance_cache(distance_cache: &mut [i32], num_distances: i32) {
        if num_distances > 4i32 {
            let last_distance: i32 = distance_cache[(0usize)];
            distance_cache[(4usize)] = last_distance - 1i32;
            distance_cache[(5usize)] = last_distance + 1i32;
            distance_cache[(6usize)] = last_distance - 2i32;
            distance_cache[(7usize)] = last_distance + 2i32;
            distance_cache[(8usize)] = last_distance - 3i32;
            distance_cache[(9usize)] = last_distance + 3i32;
            if num_distances > 10i32 {
                let next_last_distance: i32 = distance_cache[(1usize)];
                distance_cache[(10usize)] = next_last_distance - 1i32;
                distance_cache[(11usize)] = next_last_distance + 1i32;
                distance_cache[(12usize)] = next_last_distance - 2i32;
                distance_cache[(13usize)] = next_last_distance + 2i32;
                distance_cache[(14usize)] = next_last_distance - 3i32;
                distance_cache[(15usize)] = next_last_distance + 3i32;
            }
        }
}

pub const kDistanceCacheIndex : [u8;16] = [
    0, 1, 2, 3, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1,
];

pub const kDistanceCacheOffset : [i8;16]= [
    0i8,
    0i8,
    0i8,
    0i8,
    -1i8,
    1i8,
    -2i8,
    2i8,
    -3i8,
    3i8,
    -1i8,
    1i8,
    -2i8,
    2i8,
    -3i8,
    3i8];

//const BROTLI_LITERAL_BYTE_SCORE: usize = 540;
const BROTLI_DISTANCE_BIT_PENALTY: usize = 120;


// Score must be positive after applying maximal penalty.
const BROTLI_SCORE_BASE : usize = (BROTLI_DISTANCE_BIT_PENALTY * 8 * 8/* sizeof usize*/);
const kDistanceShortCodeCost : [usize;16] = [
  /* Repeat last */
  BROTLI_SCORE_BASE +  60,
  /* 2nd, 3rd, 4th last */
  BROTLI_SCORE_BASE -  95,
  BROTLI_SCORE_BASE - 117,
  BROTLI_SCORE_BASE - 127,
  /* Last with offset */
  BROTLI_SCORE_BASE -  93,
  BROTLI_SCORE_BASE -  93,
  BROTLI_SCORE_BASE -  96,
  BROTLI_SCORE_BASE -  96,
  BROTLI_SCORE_BASE -  99,
  BROTLI_SCORE_BASE -  99,
  /* 2nd last with offset */
  BROTLI_SCORE_BASE - 105,
  BROTLI_SCORE_BASE - 105,
  BROTLI_SCORE_BASE - 115,
  BROTLI_SCORE_BASE - 115,
  BROTLI_SCORE_BASE - 125,
  BROTLI_SCORE_BASE - 125
];

fn BackwardReferenceScoreH9(copy_length: usize,
                            backward_reference_offset: usize,
                            h9_opts: H9Opts) -> usize {
    (BROTLI_SCORE_BASE.wrapping_add((h9_opts.literal_byte_score as usize).wrapping_mul(copy_length)).wrapping_sub(
        (BROTLI_DISTANCE_BIT_PENALTY as usize).wrapping_mul(Log2FloorNonZero(backward_reference_offset as u64) as usize))) >> 2
}

fn BackwardReferenceScoreUsingLastDistanceH9(
    copy_length : usize, distance_short_code : usize,
    h9_opts: H9Opts) -> usize {
  ((h9_opts.literal_byte_score as usize).wrapping_mul(copy_length).wrapping_add(
      kDistanceShortCodeCost[distance_short_code])) >> 2
}

impl<AllocU16: alloc::Allocator<u16>,
              AllocU32: alloc::Allocator<u32>> AnyHasher for H9<AllocU16, AllocU32> {
  #[inline(always)]
    fn Opts(&self) -> H9Opts {
       self.h9_opts
    }
  #[inline(always)]
    fn GetHasherCommon(&mut self) -> &mut Struct1 {
        return &mut self.dict_search_stats_;
    }
  #[inline(always)]
    fn HashBytes(&self, data: &[u8]) -> usize {
        let h: u32 = BROTLI_UNALIGNED_LOAD32(data).wrapping_mul(kHashMul32);
        let thirty_two : usize = 32;
        (h >> (thirty_two.wrapping_sub(H9_BUCKET_BITS))) as usize
    }
  #[inline(always)]
    fn HashTypeLength(&self) -> usize {
        4
    }
  #[inline(always)]
    fn StoreLookahead(&self) -> usize {
        4
    }
    fn PrepareDistanceCache(&self, distance_cache: &mut [i32]) {
        let num_distances = H9_NUM_LAST_DISTANCES_TO_CHECK as i32;
        adv_prepare_distance_cache(distance_cache, num_distances);
    }
    fn FindLongestMatch(&mut self,
                        dictionary: &BrotliDictionary,
                        dictionary_hash: &[u16],
                        data: &[u8],
                        ring_buffer_mask: usize,
                        distance_cache: &[i32],
                        cur_ix: usize,
                        max_length: usize,
                        max_backward: usize,
                        out: &mut HasherSearchResult)
                        -> bool {
        let best_len_in: usize = (*out).len;
        let cur_ix_masked: usize = cur_ix & ring_buffer_mask;
        let mut best_score: usize = (*out).score;
        let mut best_len: usize = best_len_in;
        let mut is_match_found: i32 = 0i32;
        (*out).len_x_code = 0usize;
        for i in 0..H9_NUM_LAST_DISTANCES_TO_CHECK {
            let idx = kDistanceCacheIndex[i] as usize;
            let backward = (distance_cache[idx] as usize).wrapping_add(kDistanceCacheOffset[i] as usize);
            let mut prev_ix = cur_ix.wrapping_sub(backward);
            if prev_ix >= cur_ix {
                continue;
            }
            if backward > max_backward {
                continue;
            }
            prev_ix &= ring_buffer_mask;
            if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask ||
                prev_ix.wrapping_add(best_len) > ring_buffer_mask ||
                data[cur_ix_masked.wrapping_add(best_len)] != data[prev_ix.wrapping_add(best_len)] {
                continue;
            }
            {
                let len: usize = FindMatchLengthWithLimit(&data[(prev_ix as (usize))..],
                                                          &data[(cur_ix_masked as (usize))..],
                                                          max_length);
                if len >= 3 || (len == 2 && i < 2) {
                    let score = BackwardReferenceScoreUsingLastDistanceH9(len, i, self.h9_opts);
                    if best_score < score {
                        best_score = score;
                        best_len = len;
                        out.len = best_len;
                        out.distance = backward;
                        out.score = best_score;
                        is_match_found = 1i32;
                    }
                }
            }
        }
        if max_length >= 4 && cur_ix_masked.wrapping_add(best_len) <= ring_buffer_mask {
            let key = self.HashBytes(&data.split_at(cur_ix_masked).1);
            let bucket = &mut self.buckets_.slice_mut().split_at_mut(key << H9_BLOCK_BITS).1.split_at_mut(H9_BLOCK_SIZE).0;
            assert!(bucket.len() > H9_BLOCK_MASK);
            assert_eq!(bucket.len(), H9_BLOCK_MASK + 1);
            let self_num_key = &mut self.num_.slice_mut()[key];
            let down = if *self_num_key > H9_BLOCK_SIZE as u16 {
                (*self_num_key as usize) - H9_BLOCK_SIZE
            } else {0usize};
            let mut i: usize = *self_num_key as usize;
            let mut prev_best_val = data[cur_ix_masked.wrapping_add(best_len)];
            while i > down {
                i -= 1;
                let mut prev_ix = bucket[i & H9_BLOCK_MASK] as usize;
                let backward = cur_ix.wrapping_sub(prev_ix) as usize;
                if (backward > max_backward) {
                    break;
                }
                prev_ix &= ring_buffer_mask;
                if (prev_ix.wrapping_add(best_len) > ring_buffer_mask ||
                    prev_best_val != data[prev_ix.wrapping_add(best_len) as usize]) {
                    continue;
                }
                {
                    let len = FindMatchLengthWithLimit(&data.split_at(prev_ix).1,
                                                       &data.split_at((cur_ix_masked as usize)).1,
                                                       max_length);
                    if (len >= 4) {
                        /* Comparing for >= 3 does not change the semantics, but just saves
                        for a few unnecessary binary logarithms in backward reference
                        score, since we are not interested in such short matches. */
                        let score = BackwardReferenceScoreH9(len, backward, self.h9_opts);
                        if (best_score < score) {
                            best_score = score;
                            best_len = len;
                            out.len = best_len;
                            out.distance = backward;
                            out.score = best_score;
                            is_match_found = 1;
                            if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask {
                                break
                            }
                            prev_best_val = data[cur_ix_masked.wrapping_add(best_len) as usize];
                        }
                    }
                }
            }
            bucket[*self_num_key as usize & H9_BLOCK_MASK] = cur_ix as u32;
            *self_num_key = self_num_key.wrapping_add(1);
        }
        if (is_match_found == 0) {
            let (_, cur_data) = data.split_at(cur_ix_masked as usize);
            is_match_found = SearchInStaticDictionary(dictionary,
                                                      dictionary_hash,
                                                      self,
                                                      cur_data,
                                                      max_length,
                                                      max_backward,
                                                      out,
                                                      0i32);
        }
        is_match_found != 0
    }

    fn Store(&mut self, data: &[u8], mask: usize, ix: usize) {
        let (_, data_window) = data.split_at((ix & mask) as (usize));
        let key: u32 = self.HashBytes(data_window) as u32;
        let self_num_key = &mut self.num_.slice_mut()[key as usize];
        let minor_ix: usize = (*self_num_key as usize & H9_BLOCK_MASK);
        self.buckets_.slice_mut()[minor_ix.wrapping_add((key as usize) << H9_BLOCK_BITS)] = ix as u32;
        *self_num_key = self_num_key.wrapping_add(1);
    }
    fn StoreRange(&mut self, data: &[u8], mask: usize, ix_start: usize, ix_end: usize) {
        for i in ix_start..ix_end {
            self.Store(data, mask, i);
        }
    }
    fn Prepare(&mut self, _one_shot: bool, _input_size:usize, _data:&[u8]) ->HowPrepared {
        if self.GetHasherCommon().is_prepared_ != 0 {
            return HowPrepared::ALREADY_PREPARED;
        }
        for item in self.num_.slice_mut().iter_mut() {
            *item =0;
        }
        self.GetHasherCommon().is_prepared_ = 1;
        HowPrepared::NEWLY_PREPARED
    }
    fn StitchToPreviousBlock(&mut self,
                             num_bytes: usize,
                             position: usize,
                             ringbuffer: &[u8],
                             ringbuffer_mask: usize) {
        StitchToPreviousBlockInternal(self,
                                      num_bytes,
                                      position,
                                      ringbuffer,
                                      ringbuffer_mask)
    }
}

pub trait AdvHashSpecialization {
  fn get_hash_mask(&self) -> u64;
  fn set_hash_mask(&mut self, params_hash_len: i32);
  fn get_k_hash_mul(&self) -> u64;
  fn HashTypeLength(&self) -> usize;
  fn StoreLookahead(&self) -> usize;
  fn load_and_mix_word(&self, data: &[u8]) -> u64;
}

pub struct AdvHasher<Specialization: AdvHashSpecialization + Sized,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>>
{
  pub GetHasherCommon: Struct1,
  pub bucket_size_: u64,
  pub block_size_: u64,
  pub specialization: Specialization, // contains hash_mask_
  pub hash_shift_: i32,
  pub block_mask_: u32,
  pub num: AllocU16::AllocatedMemory,
  pub buckets: AllocU32::AllocatedMemory,
  pub h9_opts: H9Opts,
}
pub struct H5Sub {}
impl AdvHashSpecialization for H5Sub {
  fn get_hash_mask(&self) -> u64 {
    //return 0xffffffffffffffffu64;
    return 0xffffffffu64; // make it 32 bit
  }
  fn get_k_hash_mul(&self) -> u64 {
    return kHashMul32 as u64;
  }
  fn load_and_mix_word(&self, data: &[u8]) -> u64 {
    return (BROTLI_UNALIGNED_LOAD32(data) as u64 * self.get_k_hash_mul()) & self.get_hash_mask();
  }
  #[allow(unused_variables)]
  fn set_hash_mask(&mut self, params_hash_len: i32) {}
  fn HashTypeLength(&self) -> usize {
    4
  }
  fn StoreLookahead(&self) -> usize {
    4
  }
}

pub struct H6Sub {
  pub hash_mask: u64,
}

impl AdvHashSpecialization for H6Sub {
  fn get_hash_mask(&self) -> u64 {
    self.hash_mask
  }
  fn set_hash_mask(&mut self, params_hash_len: i32) {
    self.hash_mask = !(0u32 as (u64)) >> 64i32 - 8i32 * params_hash_len;
  }
  fn get_k_hash_mul(&self) -> u64 {
    kHashMul64Long
  }
  fn load_and_mix_word(&self, data: &[u8]) -> u64 {
    return (BROTLI_UNALIGNED_LOAD64(data) & self.get_hash_mask())
             .wrapping_mul(self.get_k_hash_mul());
  }
  fn HashTypeLength(&self) -> usize {
    8
  }
  fn StoreLookahead(&self) -> usize {
    8
  }
}

fn BackwardReferencePenaltyUsingLastDistance(distance_short_code: usize) -> usize {
  (39usize).wrapping_add((0x1ca10i32 >> (distance_short_code & 0xeusize) & 0xei32) as (usize))
}


impl<Specialization: AdvHashSpecialization, AllocU16: alloc::Allocator<u16>, AllocU32: alloc::Allocator<u32>> AnyHasher
  for AdvHasher<Specialization, AllocU16, AllocU32> {
  fn Opts(&self) -> H9Opts {
     self.h9_opts
  }
  fn PrepareDistanceCache(&self, distance_cache: &mut [i32]){
    let num_distances = self.GetHasherCommon.params.num_last_distances_to_check;
    adv_prepare_distance_cache(distance_cache, num_distances);
  }
  fn StitchToPreviousBlock(&mut self,
                           num_bytes: usize,
                           position: usize,
                           ringbuffer: &[u8],
                           ringbuffer_mask: usize) {
      StitchToPreviousBlockInternal(self,
                                    num_bytes,
                                    position,
                                    ringbuffer,
                                    ringbuffer_mask);
  }
  fn Prepare(&mut self, one_shot: bool, input_size:usize, data:&[u8]) ->HowPrepared {
      if self.GetHasherCommon.is_prepared_ != 0 {
          return HowPrepared::ALREADY_PREPARED;
      }
      let partial_prepare_threshold = self.bucket_size_ as usize >> 6;
      if one_shot && input_size <= partial_prepare_threshold {
        for i in 0..input_size {
          let key = self.HashBytes(&data[i..]);
          self.num.slice_mut()[key] = 0;
        }
      } else {
        for item in self.num.slice_mut()[..(self.bucket_size_ as usize)].iter_mut() {
          *item =0;
        }
      }
      self.GetHasherCommon.is_prepared_ = 1;
      HowPrepared::NEWLY_PREPARED
  }

  fn GetHasherCommon(&mut self) -> &mut Struct1 {
    &mut self.GetHasherCommon
  }
  fn HashTypeLength(&self) -> usize {
     self.specialization.HashTypeLength()
  }
  fn StoreLookahead(&self) -> usize {
     self.specialization.StoreLookahead()
  }
  fn HashBytes(&self, data: &[u8]) -> usize {
    let shift = self.hash_shift_;
    let h: u64 = self.specialization.load_and_mix_word(data);
    (h >> shift) as (u32) as usize
  }
  fn Store(&mut self, data: &[u8], mask: usize, ix: usize) {
    let (_, data_window) = data.split_at((ix & mask) as (usize));
    let key: u32 = self.HashBytes(data_window) as u32;
    let minor_ix: usize = (self.num.slice()[(key as (usize))] as (u32) & (*self).block_mask_) as (usize);
    let offset: usize = minor_ix.wrapping_add((key << (self.GetHasherCommon).params.block_bits) as
                                              (usize));
    self.buckets.slice_mut()[offset] = ix as (u32);
    {
      let _lhs = &mut self.num.slice_mut()[(key as (usize))];
      *_lhs = (*_lhs as (i32) + 1) as (u16);
    }
  }
  fn StoreRange(&mut self, data: &[u8], mask: usize, ix_start: usize, ix_end: usize) {
    for i in ix_start..ix_end {
      self.Store(data, mask, i);
    }
  }

  fn FindLongestMatch(&mut self,
                      dictionary: &BrotliDictionary,
                      dictionary_hash: &[u16],
                      data: &[u8],
                      ring_buffer_mask: usize,
                      distance_cache: &[i32],
                      cur_ix: usize,
                      max_length: usize,
                      max_backward: usize,
                      out: &mut HasherSearchResult)
                      -> bool {
    let opts = self.Opts();
    let cur_ix_masked: usize = cur_ix & ring_buffer_mask;
    let mut is_match_found: i32 = 0i32;
    let mut best_score: usize = (*out).score;
    let mut best_len: usize = (*out).len;
    let mut i: usize;
    (*out).len = 0usize;
    (*out).len_x_code = 0usize;
    i = 0usize;
    while i < self.GetHasherCommon.params.num_last_distances_to_check as (usize) {
      'continue45: loop {
        {
          let backward: usize = distance_cache[(i as (usize))] as (usize);
          let mut prev_ix: usize = cur_ix.wrapping_sub(backward);
          if prev_ix >= cur_ix {
            {
              break 'continue45;
            }
          }
          if backward > max_backward {
            {
              break 'continue45;
            }
          }
          prev_ix = prev_ix & ring_buffer_mask;
          if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask || prev_ix.wrapping_add(best_len) > ring_buffer_mask ||
             data[(cur_ix_masked.wrapping_add(best_len) as (usize))] as (i32) !=
             data[(prev_ix.wrapping_add(best_len) as (usize))] as (i32) {
            {
              break 'continue45;
            }
          }
          {
            let (_, prev_data) = data.split_at(prev_ix as usize);
            let (_, cur_data) = data.split_at(cur_ix_masked as usize);

            let len: usize = FindMatchLengthWithLimit(&prev_data,
                                                      &cur_data,
                                                      max_length);
            if len >= 3usize || len == 2usize && (i < 2usize) {
              let mut score: usize = BackwardReferenceScoreUsingLastDistance(len, opts);
              if best_score < score {
                if i != 0usize {
                  score = score.wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i));
                }
                if best_score < score {
                  best_score = score;
                  best_len = len;
                  (*out).len = best_len;
                  (*out).distance = backward;
                  (*out).score = best_score;
                  is_match_found = 1i32;
                }
              }
            }
          }
        }
        break;
      }
      i = i.wrapping_add(1 as (usize));
    }
    {
      let key: u32 = self.HashBytes(&data[(cur_ix_masked as (usize))..]) as u32;
      let common_block_bits = self.GetHasherCommon.params.block_bits;
        if (key << common_block_bits) as usize >= self.buckets.slice().len() {
            let key2: u32 = self.HashBytes(&data[(cur_ix_masked as (usize))..]) as u32;
            let key3: u32 = self.HashBytes(&data[(cur_ix_masked as (usize))..]) as u32;
            assert_eq!(key2, key3 + 1);
        }
      let bucket: &mut [u32] = &mut self.buckets.slice_mut()[((key << common_block_bits) as (usize))..];
      let down: usize = if self.num.slice()[(key as (usize))] as (u64) > (*self).block_size_ {
        (self.num.slice()[(key as (usize))] as (u64)).wrapping_sub((*self).block_size_) as usize
      } else {
        0u32 as (usize)
      };
      i = self.num.slice()[(key as (usize))] as (usize);
      while i > down {
        let mut prev_ix: usize = bucket[(({
            i = i.wrapping_sub(1 as (usize));
            i
          } & (*self).block_mask_ as (usize)) as (usize))] as (usize);
        let backward: usize = cur_ix.wrapping_sub(prev_ix);
        if backward > max_backward {
          {
            break;
          }
        }
        prev_ix = prev_ix & ring_buffer_mask;
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask || prev_ix.wrapping_add(best_len) > ring_buffer_mask ||
           data[(cur_ix_masked.wrapping_add(best_len) as (usize))] as (i32) !=
           data[(prev_ix.wrapping_add(best_len) as (usize))] as (i32) {
          {
            continue;
          }
        }
        {
          let (_, prev_data) = data.split_at(prev_ix as usize);
          let (_, cur_data) = data.split_at(cur_ix_masked as usize);
          let len: usize = FindMatchLengthWithLimit(&prev_data,
                                                    &cur_data,
                                                    max_length);
          if len >= 4usize {
            let score: usize = BackwardReferenceScore(len, backward, opts);
            if best_score < score {
              best_score = score;
              best_len = len;
              (*out).len = best_len;
              (*out).distance = backward;
              (*out).score = best_score;
              is_match_found = 1i32;
            }
          }
        }
      }
      bucket[((self.num.slice()[(key as (usize))] as (u32) & (self).block_mask_) as (usize))] = cur_ix as (u32);
      {
        let _rhs = 1;
        let _lhs = &mut self.num.slice_mut()[(key as (usize))];
        *_lhs = (*_lhs as (i32) + _rhs) as (u16);
      }
    }
    if is_match_found == 0 {
      let (_, cur_data) = data.split_at(cur_ix_masked as usize);
      is_match_found = SearchInStaticDictionary(dictionary,
                                                dictionary_hash,
                                                self,
                                                cur_data,
                                                max_length,
                                                max_backward,
                                                out,
                                                0i32);
    }
    is_match_found != 0

  }
}


pub struct BankH40 {
  pub slots: [SlotH40; 65536],
}

pub struct BankH41 {
  pub slots: [SlotH41; 65536],
}

pub struct BankH42 {
  pub slots: [SlotH42; 512],
}


pub struct SlotH40 {
  pub delta: u16,
  pub next: u16,
}
pub struct SlotH41 {
  pub delta: u16,
  pub next: u16,
}

pub struct SlotH42 {
  pub delta: u16,
  pub next: u16,
}

// UNSUPPORTED, for now.
pub struct H40 {
  pub common: Struct1,
  pub addr: [u32; 32768],
  pub head: [u16; 32768],
  pub tiny_hash: [u8; 65536],
  pub banks: [BankH40; 1],
  pub free_slot_idx: [u16; 1],
  pub max_hops: usize,
}


pub struct H41 {
  pub common: Struct1,
  pub addr: [u32; 32768],
  pub head: [u16; 32768],
  pub tiny_hash: [u8; 65536],
  pub banks: [BankH41; 1],
  pub free_slot_idx: [u16; 1],
  pub max_hops: usize,
}

pub struct H42 {
  pub common: Struct1,
  pub addr: [u32; 32768],
  pub head: [u16; 32768],
  pub tiny_hash: [u8; 65536],
  pub banks: [BankH42; 512],
  free_slot_idx: [u16; 512],
  pub max_hops: usize,
}




fn unopt_ctzll(mut val: usize) -> u8 {
  let mut cnt: u8 = 0i32 as (u8);
  while val & 1usize == 0usize {
    val = val >> 1i32;
    cnt = (cnt as (i32) + 1) as (u8);
  }
  cnt
}


fn BackwardReferenceScoreUsingLastDistance(copy_length: usize, h9_opts: H9Opts) -> usize {
  ((h9_opts.literal_byte_score as usize) >> 2)
    .wrapping_mul(copy_length)
    .wrapping_add(((30i32 * 8i32) as (usize)).wrapping_mul(::core::mem::size_of::<usize>()))
    .wrapping_add(15usize)
}


fn BackwardReferenceScore(copy_length: usize, backward_reference_offset: usize, h9_opts: H9Opts) -> usize {
  ((30i32 * 8i32) as (usize))
    .wrapping_mul(::core::mem::size_of::<usize>())
    .wrapping_add(((h9_opts.literal_byte_score as usize) >> 2).wrapping_mul(copy_length))
    .wrapping_sub((30u32).wrapping_mul(Log2FloorNonZero(backward_reference_offset as u64)) as
                  (usize))
}

fn Hash14(data: &[u8]) -> u32 {
  let h: u32 = BROTLI_UNALIGNED_LOAD32(data).wrapping_mul(kHashMul32);
  h >> 32i32 - 14i32
}

fn TestStaticDictionaryItem(dictionary: &BrotliDictionary,
                            item: usize,
                            data: &[u8],
                            max_length: usize,
                            max_backward: usize,
                            h9_opts: H9Opts,
                            out: &mut HasherSearchResult)
                            -> i32 {
  let len: usize;
  let dist: usize;
  let offset: usize;
  let matchlen: usize;
  let backward: usize;
  let score: usize;
  len = item & 0x1fusize;
  dist = item >> 5i32;
  offset = ((*dictionary).offsets_by_length[len] as (usize)).wrapping_add(len.wrapping_mul(dist));
  if len > max_length {
    return 0i32;
  }
  matchlen = FindMatchLengthWithLimit(data, &(*dictionary).data[offset..], len);
  if matchlen.wrapping_add(kCutoffTransformsCount as usize) <= len || matchlen == 0usize {
    return 0i32;
  }
  {
    let cut: u64 = len.wrapping_sub(matchlen) as u64;
    let transform_id: usize =
      (cut << 2i32).wrapping_add(kCutoffTransforms as u64 >> cut.wrapping_mul(6) & 0x3f) as usize;
    backward = max_backward.wrapping_add(dist)
      .wrapping_add(1usize)
      .wrapping_add(transform_id << (*dictionary).size_bits_by_length[len] as (i32));
  }
  score = BackwardReferenceScore(matchlen, backward, h9_opts);
  if score < (*out).score {
    return 0i32;
  }
  (*out).len = matchlen;
  (*out).len_x_code = len ^ matchlen;
  (*out).distance = backward;
  (*out).score = score;
  1i32
}

fn SearchInStaticDictionary<HasherType: AnyHasher>(dictionary: &BrotliDictionary,
                                                   dictionary_hash: &[u16],
                                                   handle: &mut HasherType,
                                                   data: &[u8],
                                                   max_length: usize,
                                                   max_backward: usize,
                                                   out: &mut HasherSearchResult,
                                                   shallow: i32)
                                                   -> i32 {
  let mut key: usize;
  let mut i: usize;
  let mut is_match_found: i32 = 0i32;
  let opts = handle.Opts();
  let xself: &mut Struct1 = handle.GetHasherCommon();
  if (*xself).dict_num_matches < (*xself).dict_num_lookups >> 7i32 {
    return 0i32;
  }
  key = (Hash14(data) << 1i32) as (usize); //FIXME: works for any kind of hasher??
  i = 0usize;
  while i < if shallow != 0 { 1u32 } else { 2u32 } as (usize) {
    {
      let item: usize = dictionary_hash[(key as (usize))] as (usize);
      (*xself).dict_num_lookups = (*xself).dict_num_lookups.wrapping_add(1 as (usize));
      if item != 0usize {
        let item_matches: i32 =
          TestStaticDictionaryItem(dictionary, item, data, max_length, max_backward, opts, out);
        if item_matches != 0 {
          (*xself).dict_num_matches = (*xself).dict_num_matches.wrapping_add(1 as (usize));
          is_match_found = 1i32;
        }
      }
    }
    i = i.wrapping_add(1 as (usize));
    key = key.wrapping_add(1 as (usize));
  }
  is_match_found
}

pub enum UnionHasher<AllocU16: alloc::Allocator<u16>, AllocU32: alloc::Allocator<u32>> {
  Uninit,
  H2(BasicHasher<H2Sub<AllocU32>>),
  H3(BasicHasher<H3Sub<AllocU32>>),
  H4(BasicHasher<H4Sub<AllocU32>>),
  H54(BasicHasher<H54Sub<AllocU32>>),
  H5(AdvHasher<H5Sub, AllocU16, AllocU32>),
  H6(AdvHasher<H6Sub, AllocU16, AllocU32>),
  H9(H9<AllocU16, AllocU32>),
  H10(H10<AllocU32, H10Buckets<AllocU32>, H10DefaultParams>),
}
macro_rules! match_all_hashers_mut {
    ($xself : expr, $func_call : ident, $( $args:expr),*) => {
        match $xself {
     &mut UnionHasher::H2(ref mut hasher) => hasher.$func_call($($args),*),
     &mut UnionHasher::H3(ref mut hasher) => hasher.$func_call($($args),*),
     &mut UnionHasher::H4(ref mut hasher) => hasher.$func_call($($args),*),
     &mut UnionHasher::H5(ref mut hasher) => hasher.$func_call($($args),*),
     &mut UnionHasher::H6(ref mut hasher) => hasher.$func_call($($args),*),
     &mut UnionHasher::H54(ref mut hasher) => hasher.$func_call($($args),*),
     &mut UnionHasher::H9(ref mut hasher) => hasher.$func_call($($args),*),
     &mut UnionHasher::H10(ref mut hasher) => hasher.$func_call($($args),*),
     &mut UnionHasher::Uninit => panic!("UNINTIALIZED"),
        }
    };
}
macro_rules! match_all_hashers {
    ($xself : expr, $func_call : ident, $( $args:expr),*) => {
        match $xself {
     &UnionHasher::H2(ref hasher) => hasher.$func_call($($args),*),
     &UnionHasher::H3(ref hasher) => hasher.$func_call($($args),*),
     &UnionHasher::H4(ref hasher) => hasher.$func_call($($args),*),
     &UnionHasher::H5(ref hasher) => hasher.$func_call($($args),*),
     &UnionHasher::H6(ref hasher) => hasher.$func_call($($args),*),
     &UnionHasher::H54(ref hasher) => hasher.$func_call($($args),*),
     &UnionHasher::H9(ref hasher) => hasher.$func_call($($args),*),
     &UnionHasher::H10(ref hasher) => hasher.$func_call($($args),*),
     &UnionHasher::Uninit => panic!("UNINTIALIZED"),
        }
    };
}
impl<AllocU16: alloc::Allocator<u16>, AllocU32: alloc::Allocator<u32>> AnyHasher
  for UnionHasher<AllocU16, AllocU32> {
  fn Opts(&self) -> H9Opts {
    return match_all_hashers!(self, Opts,);
  }
  fn GetHasherCommon(&mut self) -> &mut Struct1 {
    return match_all_hashers_mut!(self, GetHasherCommon,);
  }/*
  fn GetH10Tree(&mut self) -> Option<&mut H10<AllocU32, H10Buckets, H10DefaultParams>> {
    return match_all_hashers_mut!(self, GetH10Tree,);
  }*/
  fn Prepare(&mut self, one_shot: bool, input_size: usize, data: &[u8]) -> HowPrepared {
    return match_all_hashers_mut!(self, Prepare, one_shot, input_size, data);
  }
  fn HashBytes(&self, data: &[u8]) -> usize {
    return match_all_hashers!(self, HashBytes, data);
  }
  fn HashTypeLength(&self) -> usize {
    return match_all_hashers!(self, HashTypeLength,);
  }
  fn StoreLookahead(&self) -> usize {
    return match_all_hashers!(self, StoreLookahead,);
  }
  fn PrepareDistanceCache(&self, distance_cache: &mut [i32]) {
    return match_all_hashers!(self, PrepareDistanceCache, distance_cache);
  }
  fn StitchToPreviousBlock(&mut self,
                           num_bytes: usize,
                           position: usize,
                           ringbuffer: &[u8],
                           ringbuffer_mask: usize) {
    return match_all_hashers_mut!(self,
                                  StitchToPreviousBlock,
                                  num_bytes,
                                  position,
                                  ringbuffer,
                                  ringbuffer_mask);
  }
  fn FindLongestMatch(&mut self,
                      dictionary: &BrotliDictionary,
                      dictionary_hash: &[u16],
                      data: &[u8],
                      ring_buffer_mask: usize,
                      distance_cache: &[i32],
                      cur_ix: usize,
                      max_length: usize,
                      max_backward: usize,
                      out: &mut HasherSearchResult)
                      -> bool {
    return match_all_hashers_mut!(self,
                                  FindLongestMatch,
                                  dictionary,
                                  dictionary_hash,
                                  data,
                                  ring_buffer_mask,
                                  distance_cache,
                                  cur_ix,
                                  max_length,
                                  max_backward,
                                  out);
  }
  fn Store(&mut self, data: &[u8], mask: usize, ix: usize) {
    return match_all_hashers_mut!(self, Store, data, mask, ix);
  }
  fn StoreRange(&mut self, data: &[u8], mask: usize, ix_start: usize, ix_end: usize) {
    return match_all_hashers_mut!(self, StoreRange, data, mask, ix_start, ix_end);
  }
}

impl<AllocU16: alloc::Allocator<u16>, AllocU32: alloc::Allocator<u32>> UnionHasher<AllocU16, AllocU32> {
  pub fn free (&mut self, m16: &mut AllocU16, m32: &mut AllocU32) {
    match self {
      &mut UnionHasher::H2(ref mut hasher) => {
        m32.free_cell(core::mem::replace(&mut hasher.buckets_.buckets_, AllocU32::AllocatedMemory::default()));
      }
      &mut UnionHasher::H3(ref mut hasher) => {
        m32.free_cell(core::mem::replace(&mut hasher.buckets_.buckets_, AllocU32::AllocatedMemory::default()));
      }
      &mut UnionHasher::H4(ref mut hasher) => {
        m32.free_cell(core::mem::replace(&mut hasher.buckets_.buckets_, AllocU32::AllocatedMemory::default()));
      }
      &mut UnionHasher::H54(ref mut hasher) => {
        m32.free_cell(core::mem::replace(&mut hasher.buckets_.buckets_, AllocU32::AllocatedMemory::default()));
      }
      &mut UnionHasher::H5(ref mut hasher) => {
        m16.free_cell(core::mem::replace(&mut hasher.num, AllocU16::AllocatedMemory::default()));
        m32.free_cell(core::mem::replace(&mut hasher.buckets, AllocU32::AllocatedMemory::default()));
      }
      &mut UnionHasher::H6(ref mut hasher) => {
        m16.free_cell(core::mem::replace(&mut hasher.num, AllocU16::AllocatedMemory::default()));
        m32.free_cell(core::mem::replace(&mut hasher.buckets, AllocU32::AllocatedMemory::default()));
      }
      &mut UnionHasher::H9(ref mut hasher) => {
        m16.free_cell(core::mem::replace(&mut hasher.num_, AllocU16::AllocatedMemory::default()));
        m32.free_cell(core::mem::replace(&mut hasher.buckets_, AllocU32::AllocatedMemory::default()));
      }
      _ => {}
    }
    *self = UnionHasher::<AllocU16, AllocU32>::default();
  }
}


impl<AllocU16: alloc::Allocator<u16>, AllocU32: alloc::Allocator<u32>> Default
  for UnionHasher<AllocU16, AllocU32> {
  fn default() -> Self {
    UnionHasher::Uninit
  }
}

/*UnionHasher::H2(BasicHasher {
          GetHasherCommon:Struct1{params:BrotliHasherParams{
           type_:2,
           block_bits: 8,
           bucket_bits:16,
           hash_len: 4,
           num_last_distances_to_check:0},
          is_prepared_:0,
          dict_num_lookups:0,
          dict_num_matches:0,
          },
          buckets_:H2Sub{
          buckets_:[0;65537],
          },
          })
          */
fn CreateBackwardReferences<AH: AnyHasher>(dictionary: &BrotliDictionary,
                                           dictionary_hash: &[u16],
                                           num_bytes: usize,
                                           mut position: usize,
                                           ringbuffer: &[u8],
                                           ringbuffer_mask: usize,
                                           params: &BrotliEncoderParams,
                                           hasher: &mut AH,
                                           dist_cache: &mut [i32],
                                           last_insert_len: &mut usize,
                                           mut commands: &mut [Command],
                                           num_commands: &mut usize,
                                           num_literals: &mut usize) {
  let max_backward_limit: usize = (1usize << (*params).lgwin).wrapping_sub(16usize);
  let mut new_commands_count: usize = 0;
  let mut insert_length: usize = *last_insert_len;
  let pos_end: usize = position.wrapping_add(num_bytes);
  let store_end: usize = if num_bytes >= hasher.StoreLookahead() {
    position.wrapping_add(num_bytes).wrapping_sub(hasher.StoreLookahead()).wrapping_add(1usize)
  } else {
    position
  };
  let random_heuristics_window_size: usize = LiteralSpreeLengthForSparseSearch(params);
  let mut apply_random_heuristics: usize = position.wrapping_add(random_heuristics_window_size);
  let kMinScore: usize = ((30i32 * 8i32) as (usize))
    .wrapping_mul(::core::mem::size_of::<usize>())
    .wrapping_add(100usize);
  hasher.PrepareDistanceCache(dist_cache);
  while position.wrapping_add(hasher.HashTypeLength()) < pos_end {
    let mut max_length: usize = pos_end.wrapping_sub(position);
    let mut max_distance: usize = brotli_min_size_t(position, max_backward_limit);
    let mut sr = HasherSearchResult {
      len: 0,
      len_x_code: 0,
      distance: 0,
      score: 0,
    };
    sr.len = 0usize;
    sr.len_x_code = 0usize;
    sr.distance = 0usize;
    sr.score = kMinScore;
    if hasher.FindLongestMatch(dictionary,
                               dictionary_hash,
                               ringbuffer,
                               ringbuffer_mask,
                               dist_cache,
                               position,
                               max_length,
                               max_distance,
                               &mut sr) {
      let mut delayed_backward_references_in_row: i32 = 0i32;
      max_length = max_length.wrapping_sub(1 as (usize));
      'break6: loop {
        'continue7: loop {
          let cost_diff_lazy: usize = 175usize;
          let is_match_found: bool;
          let mut sr2 = HasherSearchResult {
            len: 0,
            len_x_code: 0,
            distance: 0,
            score: 0,
          };
          sr2.len = if (*params).quality < 5 {
            brotli_min_size_t(sr.len.wrapping_sub(1usize), max_length)
          } else {
            0usize
          };
          sr2.len_x_code = 0usize;
          sr2.distance = 0usize;
          sr2.score = kMinScore;
          max_distance = brotli_min_size_t(position.wrapping_add(1usize), max_backward_limit);
          is_match_found = hasher.FindLongestMatch(dictionary,
                                                   dictionary_hash,
                                                   ringbuffer,
                                                   ringbuffer_mask,
                                                   dist_cache,
                                                   position.wrapping_add(1usize),
                                                   max_length,
                                                   max_distance,
                                                   &mut sr2);
          if is_match_found && (sr2.score >= sr.score.wrapping_add(cost_diff_lazy)) {
            position = position.wrapping_add(1 as (usize));
            insert_length = insert_length.wrapping_add(1 as (usize));
            sr = sr2;
            if {
                 delayed_backward_references_in_row = delayed_backward_references_in_row + 1;
                 delayed_backward_references_in_row
               } < 4i32 &&
               (position.wrapping_add(hasher.HashTypeLength()) < pos_end) {
              {
                break 'continue7;
              }
            }
          }
          break 'break6;
        }
        max_length = max_length.wrapping_sub(1 as (usize));
      }
      apply_random_heuristics = position.wrapping_add((2usize).wrapping_mul(sr.len))
        .wrapping_add(random_heuristics_window_size);
      max_distance = brotli_min_size_t(position, max_backward_limit);
      {
        let distance_code: usize = ComputeDistanceCode(sr.distance, max_distance, dist_cache);
        if sr.distance <= max_distance && (distance_code > 0usize) {
          dist_cache[(3usize)] = dist_cache[(2usize)];
          dist_cache[(2usize)] = dist_cache[(1usize)];
          dist_cache[(1usize)] = dist_cache[(0usize)];
          dist_cache[(0usize)] = sr.distance as (i32);
          hasher.PrepareDistanceCache(dist_cache);
        }
        new_commands_count += 1;
        InitCommand({
                      let (mut _old, new_commands) = core::mem::replace(&mut commands, &mut []).split_at_mut(1);
                      commands = new_commands;
                      &mut _old[0]
                    },
                    &params.dist,
                    insert_length,
                    sr.len,
                    sr.len ^ sr.len_x_code,
                    distance_code);
      }
      *num_literals = (*num_literals).wrapping_add(insert_length);
      insert_length = 0usize;
      hasher.StoreRange(ringbuffer,
                        ringbuffer_mask,
                        position.wrapping_add(2usize),
                        brotli_min_size_t(position.wrapping_add(sr.len), store_end));
      position = position.wrapping_add(sr.len);
    } else {
      insert_length = insert_length.wrapping_add(1 as (usize));
      position = position.wrapping_add(1 as (usize));
      if position > apply_random_heuristics {
        if position >
           apply_random_heuristics.wrapping_add((4usize)
                                                  .wrapping_mul(random_heuristics_window_size)) {
          let kMargin: usize = brotli_max_size_t(hasher.StoreLookahead().wrapping_sub(1usize),
                                                 4usize);
          let pos_jump: usize = brotli_min_size_t(position.wrapping_add(16usize),
                                                  pos_end.wrapping_sub(kMargin));
          while position < pos_jump {
            {
              hasher.Store(ringbuffer, ringbuffer_mask, position);
              insert_length = insert_length.wrapping_add(4usize);
            }
            position = position.wrapping_add(4usize);
          }
        } else {
          let kMargin: usize = brotli_max_size_t(hasher.StoreLookahead().wrapping_sub(1usize),
                                                 2usize);
          let pos_jump: usize = brotli_min_size_t(position.wrapping_add(8usize),
                                                  pos_end.wrapping_sub(kMargin));
          while position < pos_jump {
            {
              hasher.Store(ringbuffer, ringbuffer_mask, position);
              insert_length = insert_length.wrapping_add(2usize);
            }
            position = position.wrapping_add(2usize);
          }
        }
      }
    }
  }
  insert_length = insert_length.wrapping_add(pos_end.wrapping_sub(position));
  *last_insert_len = insert_length;
  *num_commands = (*num_commands).wrapping_add(new_commands_count);
}
pub fn BrotliCreateBackwardReferences<AllocU16: alloc::Allocator<u16>,
                                      AllocU32: alloc::Allocator<u32>,
                                      AllocU64: alloc::Allocator<u64>,
                                      AllocF: alloc::Allocator<floatX>,
                                      AllocZN:alloc::Allocator<ZopfliNode>>
  (m32 : &mut AllocU32,
   m64 : &mut AllocU64,
   mf : &mut AllocF,
   mz : &mut AllocZN,
   dictionary: &BrotliDictionary,
   num_bytes: usize,
   position: usize,
   ringbuffer: &[u8],
   ringbuffer_mask: usize,
   params: &BrotliEncoderParams,
   hasher_union: &mut UnionHasher<AllocU16, AllocU32>,
   dist_cache: &mut [i32],
   last_insert_len: &mut usize,
   commands: &mut [Command],
   num_commands: &mut usize,
   num_literals: &mut usize) {
  match (hasher_union) {
    &mut UnionHasher::Uninit => panic!("working with uninitialized hash map"),
    &mut UnionHasher::H10(ref mut hasher) => {
      super::backward_references_hq::BrotliCreateHqZopfliBackwardReferences(
          m32, m64, mf, mz, dictionary,
          num_bytes,
          position,
          ringbuffer,
          ringbuffer_mask,
          params,
          hasher,
          dist_cache,
          last_insert_len,
          commands,
          num_commands,
          num_literals)
    }
    &mut UnionHasher::H2(ref mut hasher) => {
      CreateBackwardReferences(dictionary,
                               &kStaticDictionaryHash[..],
                               num_bytes,
                               position,
                               ringbuffer,
                               ringbuffer_mask,
                               params,
                               hasher,
                               dist_cache,
                               last_insert_len,
                               commands,
                               num_commands,
                               num_literals)
    }
    &mut UnionHasher::H3(ref mut hasher) => {
      CreateBackwardReferences(dictionary,
                               &kStaticDictionaryHash[..],
                               num_bytes,
                               position,
                               ringbuffer,
                               ringbuffer_mask,
                               params,
                               hasher,
                               dist_cache,
                               last_insert_len,
                               commands,
                               num_commands,
                               num_literals)
    }
    &mut UnionHasher::H4(ref mut hasher) => {
      CreateBackwardReferences(dictionary,
                               &kStaticDictionaryHash[..],
                               num_bytes,
                               position,
                               ringbuffer,
                               ringbuffer_mask,
                               params,
                               hasher,
                               dist_cache,
                               last_insert_len,
                               commands,
                               num_commands,
                               num_literals)
    }
    &mut UnionHasher::H5(ref mut hasher) => {
      CreateBackwardReferences(dictionary,
                               &kStaticDictionaryHash[..],
                               num_bytes,
                               position,
                               ringbuffer,
                               ringbuffer_mask,
                               params,
                               hasher,
                               dist_cache,
                               last_insert_len,
                               commands,
                               num_commands,
                               num_literals)
    }
    &mut UnionHasher::H6(ref mut hasher) => {
      CreateBackwardReferences(dictionary,
                               &kStaticDictionaryHash[..],
                               num_bytes,
                               position,
                               ringbuffer,
                               ringbuffer_mask,
                               params,
                               hasher,
                               dist_cache,
                               last_insert_len,
                               commands,
                               num_commands,
                               num_literals)
    }
    &mut UnionHasher::H9(ref mut hasher) => {
      CreateBackwardReferences(dictionary,
                               &kStaticDictionaryHash[..],
                               num_bytes,
                               position,
                               ringbuffer,
                               ringbuffer_mask,
                               params,
                               hasher,
                               dist_cache,
                               last_insert_len,
                               commands,
                               num_commands,
                               num_literals)
    }
    &mut UnionHasher::H54(ref mut hasher) => {
      CreateBackwardReferences(dictionary,
                               &kStaticDictionaryHash[..],
                               num_bytes,
                               position,
                               ringbuffer,
                               ringbuffer_mask,
                               params,
                               hasher,
                               dist_cache,
                               last_insert_len,
                               commands,
                               num_commands,
                               num_literals)
    }
  }
}
