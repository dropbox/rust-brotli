#![allow(dead_code, unused_imports)]
use enc::command::{Command, ComputeDistanceCode, InitCommand, GetInsertLengthCode, GetCopyLengthCode, CombineLengthCodes, PrefixEncodeCopyDistance, CommandCopyLen};
use super::{BrotliEncoderParams, kHashMul32,kHashMul64, kHashMul64Long, BrotliHasherParams, kInvalidMatch, kDistanceCacheIndex, kDistanceCacheOffset, Struct1, H9Opts, HowPrepared, AnyHasher, CloneWithAlloc, HasherSearchResult};
use enc::dictionary_hash::kStaticDictionaryHash;
use enc::static_dict::{BROTLI_UNALIGNED_LOAD32, BROTLI_UNALIGNED_LOAD64, FindMatchLengthWithLimit};
use enc::static_dict::{BrotliDictionary, kBrotliEncDictionary, BrotliFindAllStaticDictionaryMatches};
use enc::literal_cost::BrotliEstimateBitCostsForLiterals;
use enc::constants::{kInsExtra, kCopyExtra};
use alloc;
use alloc::{SliceWrapper, SliceWrapperMut, Allocator};
use enc::util::{Log2FloorNonZero, brotli_max_size_t,FastLog2, floatX};
use core;

pub const kInfinity: floatX = 1.7e38 as floatX;
#[derive(Clone,Copy,Debug)]
pub enum Union1 {
    cost(floatX),
    next(u32),
    shortcut(u32),
}

#[derive(Clone,Copy,Debug)]
pub struct ZopfliNode {
    //highest 7 bit is used to reconstruct the length code
    pub length : u32,
    // distance associated with the length
    pub distance : u32,
    // number of literal inserts before the copy; highest 5 bits contain distance short code + 1 (or zero if no short code)
    pub dcode_insert_length : u32,
    pub u : Union1,
}
impl Default for ZopfliNode {
    fn default() -> Self {
        ZopfliNode{
            length: 1,
            distance: 0,
            dcode_insert_length: 0,
            u: Union1::cost(kInfinity),
        }
    }
}

pub trait Allocable<T:Copy, AllocT: Allocator<T>> {
    fn new(m: &mut AllocT, init:T) -> Self;
    fn new_uninit(m: &mut AllocT) -> Self;
    fn free(&mut self, m: &mut AllocT);
}
pub trait H10Params {
    fn max_tree_search_depth() -> u32;
    fn max_tree_comp_length() -> u32;
}

pub struct H10DefaultParams{}
impl H10Params for H10DefaultParams {
    #[inline(always)]
    fn max_tree_search_depth() -> u32 {
        64
    }
    #[inline(always)]
    fn max_tree_comp_length() -> u32 {
        128
    }
}

const BUCKET_BITS:usize = 17;

pub struct H10Buckets<AllocU32:Allocator<u32>>(AllocU32::AllocatedMemory);


impl<AllocU32:Allocator<u32>> Allocable<u32, AllocU32> for H10Buckets<AllocU32> {
  fn new(m:&mut AllocU32, initializer: u32) -> H10Buckets<AllocU32> {
    let mut ret = m.alloc_cell(1 <<BUCKET_BITS);
    for item in ret.slice_mut().iter_mut() {
      *item = initializer;
    }
    H10Buckets::<AllocU32>(ret)
  }
  fn new_uninit(m:&mut AllocU32) -> H10Buckets<AllocU32> {
    H10Buckets::<AllocU32>(m.alloc_cell(1 <<BUCKET_BITS))
  }
  fn free(&mut self, m:&mut AllocU32) {
    m.free_cell(core::mem::replace(&mut self.0, AllocU32::AllocatedMemory::default()));
  }
}

impl<AllocU32:Allocator<u32>> PartialEq<H10Buckets<AllocU32>> for H10Buckets<AllocU32> {
    fn eq(&self, other: &H10Buckets<AllocU32>) -> bool {
        return self.0.slice() == other.0.slice()
    }
}

impl<AllocU32:Allocator<u32>> SliceWrapper<u32> for H10Buckets<AllocU32> {
  #[inline(always)]
  fn slice(&self) -> &[u32] {
     self.0.slice()
  }
}
impl<AllocU32:Allocator<u32>> SliceWrapperMut<u32> for H10Buckets<AllocU32> {
  #[inline(always)]
  fn slice_mut(&mut self) -> &mut [u32] {
     self.0.slice_mut()
  }
}


pub struct H10<AllocU32:Allocator<u32>, Buckets: Allocable<u32, AllocU32>+SliceWrapperMut<u32>+SliceWrapper<u32>, Params:H10Params>
where Buckets:PartialEq<Buckets>
{
    pub window_mask_: usize,
    pub common: Struct1,
    pub buckets_: Buckets,
    pub invalid_pos_:u32,
    pub forest: AllocU32::AllocatedMemory,
    pub _params: core::marker::PhantomData<Params>,
}

impl <AllocU32:Allocator<u32>, Buckets: Allocable<u32, AllocU32>+SliceWrapperMut<u32>+SliceWrapper<u32>, Params:H10Params>
    PartialEq<H10<AllocU32, Buckets, Params>> for H10<AllocU32, Buckets, Params>
    where Buckets:PartialEq<Buckets>
{
    fn eq(&self, other: &H10<AllocU32, Buckets, Params>) -> bool {
        self.window_mask_ == other.window_mask_
            && self.common == other.common
            && self.buckets_ == other.buckets_
            && self.invalid_pos_ == other.invalid_pos_
            && self.forest.slice() == other.forest.slice()
            && self._params == other._params
    }
}

pub fn InitializeH10<AllocU32:Allocator<u32>>(
    m32: &mut AllocU32, one_shot: bool, params: &BrotliEncoderParams, input_size: usize
    ) -> H10<AllocU32, H10Buckets<AllocU32>, H10DefaultParams> {
    initialize_h10::<AllocU32, H10Buckets<AllocU32>>(m32, one_shot, params, input_size)
}
fn initialize_h10<AllocU32:Allocator<u32>, Buckets:SliceWrapperMut<u32>+SliceWrapper<u32>+Allocable<u32, AllocU32> >(
    m32: &mut AllocU32, one_shot: bool, params: &BrotliEncoderParams, input_size: usize
) -> H10<AllocU32, Buckets, H10DefaultParams> where Buckets:PartialEq<Buckets> {
    let mut num_nodes = 1 << params.lgwin;
    if one_shot && input_size < num_nodes {
        num_nodes = input_size;
    }
    let window_mask = (1<<params.lgwin) - 1;
    let invalid_pos = 0u32.wrapping_sub(window_mask) as u32;
    let buckets = <Buckets as Allocable<u32, AllocU32>>::new(m32, invalid_pos);
    H10::<AllocU32, Buckets, H10DefaultParams> {
        common:Struct1{
            params:params.hasher,
            is_prepared_:1,
            dict_num_lookups:0,
            dict_num_matches:0,
        },
        _params: core::marker::PhantomData::<H10DefaultParams>::default(),
        window_mask_: window_mask as usize,
        invalid_pos_: invalid_pos,
        buckets_: buckets,
        forest: m32.alloc_cell(num_nodes * 2),
    }
}

impl<AllocU32: Allocator<u32>,
     Buckets: Allocable<u32, AllocU32>+SliceWrapperMut<u32>+SliceWrapper<u32>,
     Params:H10Params> H10<AllocU32, Buckets, Params> where Buckets:PartialEq<Buckets> {
    pub fn free(&mut self, m32: &mut AllocU32) {
        m32.free_cell(core::mem::replace(&mut self.forest, AllocU32::AllocatedMemory::default()));
        self.buckets_.free(m32);
    }
}
impl<Alloc: alloc::Allocator<u16> + alloc::Allocator<u32>, Buckets:Allocable<u32, Alloc>+SliceWrapperMut<u32>+SliceWrapper<u32>, Params:H10Params,
     > CloneWithAlloc<Alloc> for H10<Alloc, Buckets, Params> where Buckets:PartialEq<Buckets> {
  fn clone_with_alloc(&self, m: &mut Alloc) -> Self {
      let mut ret = H10::<Alloc, Buckets, Params> {
          window_mask_: self.window_mask_,
          common: self.common.clone(),
          buckets_:Buckets::new_uninit(m),
          invalid_pos_:self.invalid_pos_,
          forest:<Alloc as Allocator<u32>>::alloc_cell(m, self.forest.len()),
          _params: core::marker::PhantomData::<Params>::default(),
      };
      ret.buckets_.slice_mut().clone_from_slice(self.buckets_.slice());
      ret.forest.slice_mut().clone_from_slice(self.forest.slice());
      ret
  }
}

impl<AllocU32: Allocator<u32>,
     Buckets: Allocable<u32, AllocU32>+SliceWrapperMut<u32>+SliceWrapper<u32>,
     Params:H10Params> AnyHasher for H10<AllocU32, Buckets, Params> where Buckets:PartialEq<Buckets> {
/*  fn GetH10Tree(&mut self) -> Option<&mut H10<AllocU32, Buckets, H10Params>> {
    Some(self)
  }*/
  #[inline(always)]
  fn Opts(&self) -> H9Opts {
      H9Opts{literal_byte_score:340}
  }
  #[inline(always)]
  fn PrepareDistanceCache(&self, _distance_cache: &mut [i32]) {}
  #[inline(always)]   
  fn HashTypeLength(&self) -> usize {
    4
  }
  #[inline(always)]
  fn StoreLookahead(&self) -> usize {
      Params::max_tree_comp_length() as usize
  }
  fn StitchToPreviousBlock(&mut self,
                           num_bytes: usize,
                           position: usize,
                           ringbuffer: &[u8],
                           ringbuffer_mask: usize) {
      super::hq::StitchToPreviousBlockH10(self, num_bytes, position, ringbuffer, ringbuffer_mask)
  }
  #[inline(always)]
  fn GetHasherCommon(&mut self) -> &mut Struct1 {
    &mut self.common
  }
  #[inline(always)]
  fn HashBytes(&self, data: &[u8]) -> usize {
    let h = BROTLI_UNALIGNED_LOAD32(
              data
          ).wrapping_mul(
              kHashMul32
          );
    (h >> 32i32 - BUCKET_BITS as i32) as usize
  }
  #[inline(always)]
  fn Store(&mut self, data: &[u8], mask: usize, ix: usize) {
    let max_backward
        : usize
        = (*self).window_mask_.wrapping_sub(16usize).wrapping_add(
              1usize
          );
    StoreAndFindMatchesH10(
        self,
        data,
        ix,
        mask,
        Params::max_tree_comp_length() as usize,
        max_backward,
        &mut 0,
        &mut[], 
    );
  }
  fn StoreRange(&mut self, data: &[u8], mask: usize, ix_start: usize, ix_end: usize) {
    let mut i : usize = ix_start;
    let mut j : usize = ix_start;
    if ix_start.wrapping_add(63usize) <= ix_end {
        i = ix_end.wrapping_sub(63usize);
    }
    if ix_start.wrapping_add(512usize) <= i {
        while j < i {
            {
                self.Store(data,mask,j);
            }
            j = j.wrapping_add(8usize);
        }
    }
    while i < ix_end {
        {
            self.Store(data,mask,i);
        }
        i = i.wrapping_add(1 as (usize));
    }
  }
  fn BulkStoreRange(&mut self, data: &[u8], mask: usize, ix_start: usize, ix_end: usize) {
    for i in ix_start..ix_end {
      self.Store(data, mask, i);
    }
  }
  fn Prepare(&mut self, _one_shot: bool, _input_size: usize, _data: &[u8]) -> HowPrepared {
    if self.common.is_prepared_ != 0 {
      return HowPrepared::ALREADY_PREPARED;
    }
    let invalid_pos = self.invalid_pos_;
    for bucket in self.buckets_.slice_mut().iter_mut() {
        *bucket = invalid_pos;
    }
    self.common.is_prepared_ = 1;
    HowPrepared::NEWLY_PREPARED
  }

  fn FindLongestMatch(&mut self,
                      _dictionary: Option<&BrotliDictionary>,
                      _dictionary_hash: &[u16],
                      _data: &[u8],
                      _ring_buffer_mask: usize,
                      _distance_cache: &[i32],
                      _cur_ix: usize,
                      _max_length: usize,
                      _max_backward: usize,
                      _gap: usize,
                      _max_distance: usize,
                      _out: &mut HasherSearchResult)
                      -> bool {
      unimplemented!();
  }
}

pub struct BackwardMatch(pub u64);

//    pub distance : u32,
//    pub length_and_code : u32,
impl BackwardMatch {
    #[inline(always)]
    pub fn distance(&self) -> u32 {
        self.0 as u32
    }
    #[inline(always)]
    pub fn length_and_code(&self) -> u32 {
        (self.0 >> 32) as u32
    }
}
pub struct BackwardMatchMut<'a>(pub &'a mut u64);

//    pub distance : u32,
//    pub length_and_code : u32,
impl<'a> BackwardMatchMut<'a> {
    #[inline(always)]
    pub fn distance(&self) -> u32 {
        *self.0 as u32
    }
    #[inline(always)]
    pub fn length_and_code(&self) -> u32 {
        (*self.0 >> 32) as u32
    }
    #[inline(always)]
    pub fn set_distance(&mut self, data: u32) {
        *self.0 &= 0xffffffff00000000;
        *self.0 |= u64::from(data)
    }
    #[inline(always)]
    pub fn set_length_and_code(&mut self, data: u32) {
        *self.0 = u64::from((*self.0) as u32) | (u64::from(data) << 32);
    }
}

#[inline(always)]
pub fn InitBackwardMatch(
    xself : &mut BackwardMatchMut, dist : usize, len : usize
) {
    (*xself).set_distance(dist as (u32));
    (*xself).set_length_and_code((len << 5i32) as (u32));
}


macro_rules! LeftChildIndexH10 {
    ($xself: expr, $pos: expr) => {
        (2usize).wrapping_mul($pos & (*$xself).window_mask_)    
    };
}
macro_rules! RightChildIndexH10 {
    ($xself: expr, $pos: expr) => {
        (2usize).wrapping_mul(
            $pos & (*$xself).window_mask_
        ).wrapping_add(
            1usize
        )
    }
}
/*
fn LeftChildIndexH10<AllocU32: Allocator<u32>,
     Buckets: Allocable<u32, AllocU32>+SliceWrapperMut<u32>+SliceWrapper<u32>,
     Params:H10Params>(
    mut xself : &mut H10<AllocU32, Buckets, Params>, pos : usize
) -> usize {
    (2usize).wrapping_mul(pos & (*xself).window_mask_)
}

fn RightChildIndexH10<AllocU32: Allocator<u32>,
     Buckets: Allocable<u32, AllocU32>+SliceWrapperMut<u32>+SliceWrapper<u32>,
     Params:H10Params>(
    mut xself : &mut H10<AllocU32, Buckets, Params>, pos : usize
) -> usize {
    (2usize).wrapping_mul(
        pos & (*xself).window_mask_
    ).wrapping_add(
        1usize
    )
}
*/

pub fn StoreAndFindMatchesH10<AllocU32: Allocator<u32>,
     Buckets: Allocable<u32, AllocU32>+SliceWrapperMut<u32>+SliceWrapper<u32>,
     Params:H10Params>(
    xself : &mut H10<AllocU32, Buckets, Params>,
    data : & [u8],
    cur_ix : usize,
    ring_buffer_mask : usize,
    max_length : usize,
    max_backward : usize,
    best_len : &mut usize,
    matches : &mut [u64]) -> usize where Buckets:PartialEq<Buckets> {
    let mut matches_offset = 0usize;
    let cur_ix_masked : usize = cur_ix & ring_buffer_mask;
    let max_comp_len
        : usize
        = core::cmp::min(max_length,128usize);
    let should_reroot_tree
        : i32
        = if !!(max_length >= 128usize) { 1i32 } else { 0i32 };
    let key
        = xself.HashBytes(
              &data[(cur_ix_masked as (usize)).. ]
          );
    let forest : &mut [u32] = xself.forest.slice_mut();
    let mut prev_ix
        : usize
        = (*xself).buckets_.slice()[key] as (usize);
    let mut node_left : usize = LeftChildIndexH10!(xself,cur_ix);
    let mut node_right : usize = RightChildIndexH10!(xself,cur_ix);
    let mut best_len_left : usize = 0usize;
    let mut best_len_right : usize = 0usize;
    let mut depth_remaining : usize;
    if should_reroot_tree != 0 {
        (*xself).buckets_.slice_mut()[(key as (usize))]= cur_ix as (u32);
    }
    depth_remaining = 64usize;
    'break16: loop {
        {
            let backward : usize = cur_ix.wrapping_sub(prev_ix);
            let prev_ix_masked : usize = prev_ix & ring_buffer_mask;
            if backward == 0usize || backward > max_backward || depth_remaining == 0usize {
                if should_reroot_tree != 0 {
                    forest[(node_left as (usize)) ]= (*xself).invalid_pos_;
                    forest[(node_right as (usize)) ]= (*xself).invalid_pos_;
                }
                break 'break16;
            }
            {
                let cur_len
                    : usize
                    = core::cmp::min(best_len_left,best_len_right);
                let len : usize;
                len = cur_len.wrapping_add(
                          FindMatchLengthWithLimit(
                              &data[(
                                    cur_ix_masked.wrapping_add(cur_len) as (usize)
                                )..],
                              &data[(
                                    prev_ix_masked.wrapping_add(cur_len) as (usize)
                                )..],
                              max_length.wrapping_sub(cur_len)
                          )
                      );
                if matches_offset != matches.len() && (len > *best_len) {
                    *best_len = len;
                    InitBackwardMatch(
                        &mut BackwardMatchMut(&mut matches[matches_offset]),
                        backward,
                        len
                    );
                    matches_offset += 1;
                }
                if len >= max_comp_len {
                    if should_reroot_tree != 0 {
                        forest[(node_left as (usize)) ]= forest[(
                                                                    LeftChildIndexH10!(
                                                                        xself,
                                                                        prev_ix
                                                                    ) as (usize)
                                                                )];
                        forest[(node_right as (usize)) ]= forest[(
                                                                     RightChildIndexH10!(
                                                                         xself,
                                                                         prev_ix
                                                                     ) as (usize)
                                                                 )];
                    }
                    break 'break16;
                }
                if data[(
                        cur_ix_masked.wrapping_add(len) as (usize)
                    ) ]as (i32) > data[(
                                      prev_ix_masked.wrapping_add(len) as (usize)
                                  ) ]as (i32) {
                    best_len_left = len;
                    if should_reroot_tree != 0 {
                        forest[(node_left as (usize)) ]= prev_ix as (u32);
                    }
                    node_left = RightChildIndexH10!(xself,prev_ix);
                    prev_ix = forest[(node_left as (usize)) ]as (usize);
                } else {
                    best_len_right = len;
                    if should_reroot_tree != 0 {
                        forest[(node_right as (usize)) ]= prev_ix as (u32);
                    }
                    node_right = LeftChildIndexH10!(xself,prev_ix);
                    prev_ix = forest[(node_right as (usize)) ]as (usize);
                }
            }
        }
        depth_remaining = depth_remaining.wrapping_sub(1 as (usize));
    }
    matches_offset
}


