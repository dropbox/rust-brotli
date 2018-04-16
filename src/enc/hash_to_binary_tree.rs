#![allow(dead_code, unused_imports)]
use super::command::{Command, ComputeDistanceCode, InitCommand, GetInsertLengthCode, GetCopyLengthCode, CombineLengthCodes, PrefixEncodeCopyDistance, CommandCopyLen};
use super::backward_references::{BrotliEncoderParams, kHashMul32,kHashMul64, kHashMul64Long, BrotliHasherParams, kInvalidMatch, kDistanceCacheIndex, kDistanceCacheOffset, Struct1, H9Opts, HowPrepared, AnyHasher, HasherSearchResult};
use super::dictionary_hash::kStaticDictionaryHash;
use super::static_dict::{BROTLI_UNALIGNED_LOAD32, BROTLI_UNALIGNED_LOAD64, FindMatchLengthWithLimit};
use super::static_dict::{BrotliDictionary, kBrotliEncDictionary, BrotliFindAllStaticDictionaryMatches};
use super::literal_cost::BrotliEstimateBitCostsForLiterals;
use super::constants::{kInsExtra, kCopyExtra};
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut, Allocator};
use super::util::{Log2FloorNonZero, brotli_max_size_t,FastLog2, floatX};
use core;


pub trait Allocable<T:Copy, AllocT: Allocator<T>> {
    fn new(m: &mut AllocT, init:T) -> Self;
    fn free(m: &mut AllocT, data: Self);
}
pub trait H10Params {
    fn max_tree_search_depth() -> u32;
    fn max_tree_comp_length() -> u32;
}

pub struct H10DefaultParams{}
impl H10Params for H10DefaultParams {
    fn max_tree_search_depth() -> u32 {
        64
    }
    fn max_tree_comp_length() -> u32 {
        128
    }
}

const BUCKET_BITS:usize = 17;

pub struct H10Buckets([u32;1 << BUCKET_BITS]);

pub struct H10<AllocU32:Allocator<u32>, Buckets: Allocable<u32, AllocU32>+SliceWrapperMut<u32>+SliceWrapper<u32>, Params:H10Params> {
    pub window_mask_: usize,
    pub common: Struct1,
    pub buckets_: Buckets,
    pub invalid_pos_:u32,
    pub forest: AllocU32::AllocatedMemory,
    pub _params: core::marker::PhantomData<Params>,
}



impl<AllocU32: Allocator<u32>,
     Buckets: Allocable<u32, AllocU32>+SliceWrapperMut<u32>+SliceWrapper<u32>,
     Params:H10Params> AnyHasher for H10<AllocU32, Buckets, Params> {
/*  fn GetH10Tree(&mut self) -> Option<&mut H10<AllocU32, Buckets, H10Params>> {
    Some(self)
  }*/
  #[inline(always)]
  fn Opts(&self) -> H9Opts {
      H9Opts{literal_byte_score:340}
  }
  fn PrepareDistanceCache(&self, _distance_cache: &mut [i32]) {}
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
    unimplemented!();
  }
  #[inline(always)]
  fn GetHasherCommon(&mut self) -> &mut Struct1 {
    &mut self.common
  }
  #[inline(always)]
  fn HashBytes(&self, data: &[u8]) -> usize {
    let mut h
        : u32
        = BROTLI_UNALIGNED_LOAD32(
              data
          ).wrapping_mul(
              kHashMul32
          );
    (h >> 32i32 - BUCKET_BITS as i32) as usize
  }
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
  fn Prepare(&mut self, one_shot: bool, input_size: usize, data: &[u8]) -> HowPrepared {
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
      unimplemented!();
  }
}

pub struct BackwardMatch(pub u64);

//    pub distance : u32,
//    pub length_and_code : u32,
impl BackwardMatch {
    pub fn distance(&self) -> u32 {
        self.0 as u32
    }
    pub fn length_and_code(&self) -> u32 {
        (self.0 >> 32) as u32
    }
}
pub struct BackwardMatchMut<'a>(pub &'a mut u64);

//    pub distance : u32,
//    pub length_and_code : u32,
impl<'a> BackwardMatchMut<'a> {
    pub fn distance(&self) -> u32 {
        *self.0 as u32
    }
    pub fn length_and_code(&self) -> u32 {
        (*self.0 >> 32) as u32
    }
    pub fn set_distance(&mut self, data: u32) {
        *self.0 &= 0xffffffff00000000;
        *self.0 |= u64::from(data)
    }
    pub fn set_length_and_code(&mut self, data: u32) {
        *self.0 = u64::from((*self.0) as u32) | (u64::from(data) << 32);
    }
}

pub fn InitBackwardMatch(
    mut xself : &mut BackwardMatchMut, mut dist : usize, mut len : usize
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
    mut xself : &mut H10<AllocU32, Buckets, Params>,
    data : & [u8],
    cur_ix : usize,
    ring_buffer_mask : usize,
    max_length : usize,
    max_backward : usize,
    best_len : &mut usize,
    mut matches : &mut [u64]) -> usize {
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
    let mut forest : &mut [u32] = xself.forest.slice_mut();
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
                let mut len : usize;
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


