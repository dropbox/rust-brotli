#![allow(dead_code)]

use super::block_split::BlockSplit;
use super::command::{Command, CommandCopyLen, CommandDistanceContext};
use super::constants::{kSigned3BitContextLookup, kUTF8ContextLookup};
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::vectorization::{Mem256i};
use core;
use core::cmp::min;
static kBrotliMinWindowBits: i32 = 10i32;

static kBrotliMaxWindowBits: i32 = 24i32;

//#[derive(Clone)] clone is broken for arrays > 32
pub struct HistogramLiteral {
  pub data_: [u32; 256],
  pub total_count_: usize,
  pub bit_cost_: super::util::floatX,
}
impl Clone for HistogramLiteral {
  #[inline(always)]
  fn clone(&self) -> HistogramLiteral {
    return HistogramLiteral {
             data_: self.data_,
             total_count_: self.total_count_,
             bit_cost_: self.bit_cost_,
           };
  }
}
impl Default for HistogramLiteral {
  #[inline(always)]
  fn default() -> HistogramLiteral {
    return HistogramLiteral {
             data_: [0; 256],
             total_count_: 0,
             bit_cost_: 3.402e+38 as super::util::floatX,
           };
  }
}
//#[derive(Clone)] clone is broken for arrays > 32
pub struct HistogramCommand {
  pub data_: [u32; 704],
  pub total_count_: usize,
  pub bit_cost_: super::util::floatX,
}
impl Clone for HistogramCommand {
  #[inline(always)]
  fn clone(&self) -> HistogramCommand {
    return HistogramCommand {
             data_: self.data_,
             total_count_: self.total_count_,
             bit_cost_: self.bit_cost_,
           };
  }
}
impl Default for HistogramCommand {
  #[inline(always)]
  fn default() -> HistogramCommand {
    return HistogramCommand {
             data_: [0; 704],
             total_count_: 0,
             bit_cost_: 3.402e+38 as super::util::floatX,
           };
  }
}
//#[derive(Clone)] // #derive is broken for arrays > 32

#[cfg(not(feature="disallow_large_window_size"))]
const BROTLI_NUM_HISTOGRAM_DISTANCE_SYMBOLS: usize = 544;
#[cfg(feature="disallow_large_window_size")]
const BROTLI_NUM_HISTOGRAM_DISTANCE_SYMBOLS: usize = 520;

pub struct HistogramDistance {
  pub data_: [u32; BROTLI_NUM_HISTOGRAM_DISTANCE_SYMBOLS],
  pub total_count_: usize,
  pub bit_cost_: super::util::floatX,
}
impl Clone for HistogramDistance {
  fn clone(&self) -> HistogramDistance {
    return HistogramDistance {
             data_: self.data_,
             total_count_: self.total_count_,
             bit_cost_: self.bit_cost_,
           };
  }
}
impl Default for HistogramDistance {
  fn default() -> HistogramDistance {
    return HistogramDistance {
             data_: [0; BROTLI_NUM_HISTOGRAM_DISTANCE_SYMBOLS],
             total_count_: 0,
             bit_cost_: 3.402e+38 as super::util::floatX,
           };
  }
}

pub trait CostAccessors {
  type i32vec : Sized + SliceWrapper<Mem256i>+SliceWrapperMut<Mem256i>;
  fn make_nnz_storage() -> Self::i32vec;
  #[inline(always)]
  fn total_count(&self) -> usize;
  #[inline(always)]
  fn bit_cost(&self) -> super::util::floatX;
  #[inline(always)]
  fn set_bit_cost(&mut self, cost: super::util::floatX);
  #[inline(always)]
  fn set_total_count(&mut self, count: usize);
}
impl SliceWrapper<u32> for HistogramLiteral {
  #[inline(always)]
  fn slice(&self) -> &[u32] {
    return &self.data_[..];
  }
}
impl SliceWrapperMut<u32> for HistogramLiteral {
  #[inline(always)]
  fn slice_mut(&mut self) -> &mut [u32] {
    return &mut self.data_[..];
  }
}
pub struct Array264i([Mem256i;33]);
impl SliceWrapperMut<Mem256i> for Array264i {
  #[inline(always)]
    fn slice_mut(&mut self) -> &mut [Mem256i] {
        return &mut self.0[..]
    }
}

impl SliceWrapper<Mem256i> for Array264i {
  #[inline(always)]
    fn slice(&self) -> & [Mem256i] {
        return &self.0[..]
    }
}
impl Default for Array264i {
  #[inline(always)]
    fn default() -> Array264i {
        return Array264i([Mem256i::default().clone();33]);
    }
}
pub struct Array528i([Mem256i;66]);
impl SliceWrapperMut<Mem256i> for Array528i {
  #[inline(always)]
    fn slice_mut(&mut self) -> &mut [Mem256i] {
        return &mut self.0[..]
    }
}
impl SliceWrapper<Mem256i> for Array528i {
  #[inline(always)]
    fn slice(&self) -> & [Mem256i] {
        return &self.0[..]
    }
}
impl Default for Array528i {
  #[inline(always)]
    fn default() -> Array528i {
        return Array528i([Mem256i::default();66]);
    }
}

pub struct Array712i([Mem256i;89]);
impl SliceWrapperMut<Mem256i> for Array712i {
  #[inline(always)]
    fn slice_mut(&mut self) -> &mut [Mem256i] {
        return &mut self.0[..]
    }
}
impl SliceWrapper<Mem256i> for Array712i {
  #[inline(always)]
    fn slice(&self) -> & [Mem256i] {
        return &self.0[..]
    }
}
impl Default for Array712i {
  #[inline(always)]
    fn default() -> Array712i {
        return Array712i([Mem256i::default();89]);
    }
}


pub struct EmptyIVec{}

impl SliceWrapperMut<Mem256i> for EmptyIVec {
  #[inline(always)]
    fn slice_mut(&mut self) -> &mut [Mem256i] {
        return &mut []
    }
}
impl SliceWrapper<Mem256i> for EmptyIVec {
  #[inline(always)]
    fn slice(&self) -> & [Mem256i] {
        return & []
    }
}

impl Default for EmptyIVec {
  #[inline(always)]
    fn default() -> EmptyIVec {
        return EmptyIVec{};
    }
}

#[cfg(feature="vector_scratch_space")]
pub type HistogramLiteralScratch = Array264i;

#[cfg(not(feature="vector_scratch_space"))]
pub type HistogramLiteralScratch = EmptyIVec;

impl CostAccessors for HistogramLiteral {
  type i32vec = HistogramLiteralScratch;
  fn make_nnz_storage() -> Self::i32vec {
      return HistogramLiteralScratch::default();
  }
  #[inline(always)]
  fn total_count(&self) -> usize {
    return self.total_count_;
  }
  #[inline(always)]
  fn bit_cost(&self) -> super::util::floatX {
    return self.bit_cost_;
  }
  #[inline(always)]
  fn set_bit_cost(&mut self, data: super::util::floatX) {
    self.bit_cost_ = data;
  }
  #[inline(always)]
  fn set_total_count(&mut self, data: usize) {
    self.total_count_ = data;
  }
}

impl SliceWrapper<u32> for HistogramCommand {
  #[inline(always)]
  fn slice(&self) -> &[u32] {
    return &self.data_[..];
  }
}
impl SliceWrapperMut<u32> for HistogramCommand {
  #[inline(always)]
  fn slice_mut(&mut self) -> &mut [u32] {
    return &mut self.data_[..];
  }
}

#[cfg(feature="vector_scratch_space")]
pub type HistogramCommandScratch = Array712i;

#[cfg(not(feature="vector_scratch_space"))]
pub type HistogramCommandScratch = EmptyIVec;


impl CostAccessors for HistogramCommand {
  type i32vec = HistogramCommandScratch;
  fn make_nnz_storage() -> Self::i32vec {
      return HistogramCommandScratch::default();
  }
  #[inline(always)]
  fn total_count(&self) -> usize {
    return self.total_count_;
  }
  #[inline(always)]
  fn bit_cost(&self) -> super::util::floatX {
    return self.bit_cost_;
  }
  #[inline(always)]
  fn set_bit_cost(&mut self, data: super::util::floatX) {
    self.bit_cost_ = data;
  }
  #[inline(always)]
  fn set_total_count(&mut self, data: usize) {
    self.total_count_ = data;
  }
}

impl SliceWrapper<u32> for HistogramDistance {
  #[inline(always)]
  fn slice(&self) -> &[u32] {
    return &self.data_[..];
  }
}
impl SliceWrapperMut<u32> for HistogramDistance {
  #[inline(always)]
  fn slice_mut(&mut self) -> &mut [u32] {
    return &mut self.data_[..];
  }
}

#[cfg(feature="vector_scratch_space")]
pub type HistogramDistanceScratch = Array528i;

#[cfg(not(feature="vector_scratch_space"))]
pub type HistogramDistanceScratch = EmptyIVec;


impl CostAccessors for HistogramDistance {
  type i32vec = HistogramDistanceScratch;
  fn make_nnz_storage() -> Self::i32vec {
      return HistogramDistanceScratch::default();
  }

  #[inline(always)]
  fn total_count(&self) -> usize {
    return self.total_count_;
  }
  #[inline(always)]
  fn bit_cost(&self) -> super::util::floatX {
    return self.bit_cost_;
  }
  #[inline(always)]
  fn set_bit_cost(&mut self, data: super::util::floatX) {
    self.bit_cost_ = data;
  }
  #[inline(always)]
  fn set_total_count(&mut self, data: usize) {
    self.total_count_ = data;
  }
}



#[derive(Copy,Clone)]
pub enum ContextType {
  CONTEXT_LSB6 = 0,
  CONTEXT_MSB6 = 1,
  CONTEXT_UTF8 = 2,
  CONTEXT_SIGNED = 3,
}

impl Default for ContextType {
  #[inline(always)]
  fn default() -> ContextType {
    ContextType::CONTEXT_LSB6
  }
}


pub struct BlockSplitIterator<'a,
                              Alloc: alloc::Allocator<u8> + 'a + alloc::Allocator<u32> + 'a>
{
  pub split_: &'a BlockSplit<Alloc>,
  pub idx_: usize,
  pub type_: usize,
  pub length_: usize,
}



fn NewBlockSplitIterator<'a, Alloc: alloc::Allocator<u8> +  alloc::Allocator<u32>>
  (split: &'a BlockSplit<Alloc>)
   -> BlockSplitIterator<'a, Alloc> {
  return BlockSplitIterator::<'a> {
           split_: split,
           idx_: 0i32 as (usize),
           type_: 0i32 as (usize),
           length_: if (*split).lengths.slice().len() != 0 {
             (*split).lengths.slice()[0] as usize
           } else {
             0i32 as (usize)
           },
         };
}


fn InitBlockSplitIterator<'a,
                          Alloc:alloc::Allocator<u8> + alloc::Allocator<u32>>(
          xself: &'a mut BlockSplitIterator<'a, Alloc>,
split: &'a BlockSplit<Alloc>){
  (*xself).split_ = split;
  (*xself).idx_ = 0i32 as (usize);
  (*xself).type_ = 0i32 as (usize);
  (*xself).length_ = if (*split).lengths.slice().len() != 0 {
    (*split).lengths.slice()[0] as u32
  } else {
    0i32 as (u32)
  } as (usize);
}
fn BlockSplitIteratorNext<'a,
                          Alloc: alloc::Allocator<u8> + alloc::Allocator<u32>>(xself: &mut BlockSplitIterator<Alloc>){
  if (*xself).length_ == 0i32 as (usize) {
    (*xself).idx_ = (*xself).idx_.wrapping_add(1 as (usize));
    (*xself).type_ = (*(*xself).split_).types.slice()[(*xself).idx_ as (usize)] as (usize);
    (*xself).length_ = (*(*xself).split_).lengths.slice()[(*xself).idx_ as (usize)] as (usize);
  }
  (*xself).length_ = (*xself).length_.wrapping_sub(1 as (usize));
}
pub fn HistogramAddItem<HistogramType: SliceWrapper<u32> + SliceWrapperMut<u32> + CostAccessors>
  (xself: &mut HistogramType,
   val: usize) {
  {
    let _rhs = 1;
    let _lhs = &mut (*xself).slice_mut()[val];
    let val = (*_lhs).wrapping_add(_rhs as (u32));
    *_lhs = val;
  }
  let new_count = (*xself).total_count().wrapping_add(1 as (usize));
  (*xself).set_total_count(new_count);
}
pub fn HistogramAddVector<HistogramType: SliceWrapper<u32> + SliceWrapperMut<u32> + CostAccessors,
                          IntegerType: Sized + Clone>
  (xself: &mut HistogramType,
   p: &[IntegerType],
   n: usize)
  where u64: core::convert::From<IntegerType>
{
  let new_tc = (*xself).total_count().wrapping_add(n);
  (*xself).set_total_count(new_tc);
  for p_item in p[..n].iter() {
    let _rhs = 1;
    let index: usize = u64::from(p_item.clone()) as usize;
    let _lhs = &mut (*xself).slice_mut()[index];
    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
  }
}

#[inline(always)]
pub fn HistogramClear<HistogramType:SliceWrapperMut<u32>+CostAccessors>(xself: &mut HistogramType){
  for data_elem in xself.slice_mut().iter_mut() {
    *data_elem = 0;
  }
  (*xself).set_total_count(0);
  (*xself).set_bit_cost(3.402e+38 as super::util::floatX);
}
pub fn ClearHistograms<HistogramType:SliceWrapperMut<u32>+CostAccessors>(array: &mut [HistogramType], length: usize){
  for item in array[..length].iter_mut() {
    HistogramClear(item)
  }
}

#[inline(always)]
pub fn HistogramAddHistogram<HistogramType:SliceWrapperMut<u32> + SliceWrapper<u32> + CostAccessors>(
    xself : &mut HistogramType, v : &HistogramType
){
  let old_total_count = (*xself).total_count();
  (*xself).set_total_count(old_total_count + (*v).total_count());
  let h0 = xself.slice_mut();
  let h1 = v.slice();
  let n = min(h0.len(), h1.len());
  for i in 0..n {
    let mut h0val = &mut h0[i];
    let val = h0val.wrapping_add(h1[i]);
    *h0val = val;
  }
}
pub fn HistogramSelfAddHistogram<HistogramType:SliceWrapperMut<u32> + SliceWrapper<u32> + CostAccessors>(
    xself : &mut [HistogramType], i0 : usize, i1 : usize
){
  let tc_new = xself[i1].total_count();
  let tc_old = xself[i0].total_count();
  xself[i0].set_total_count(tc_old.wrapping_add(tc_new));
  let h0 = xself[i0].slice().len();
  let h0a = xself[i0].slice().len();
  let h1 = xself[i1].slice().len();
  let n = min(h0, min(h0a, h1));
  for h_index in 0..n {
    let val = xself[i0].slice()[h_index].wrapping_add(xself[i1].slice()[h_index]);
    xself[i0].slice_mut()[h_index] = val;
  }
}

#[inline(always)]
pub fn Context(p1: u8, p2: u8, mode: ContextType) -> u8 {
  match mode {
    ContextType::CONTEXT_SIGNED => {
      ((kSigned3BitContextLookup[p1 as (usize)] as (i32) << 3i32) +
       kSigned3BitContextLookup[p2 as (usize)] as (i32)) as (u8)
    }
    ContextType::CONTEXT_UTF8 => {
      (kUTF8ContextLookup[p1 as (usize)] as (i32) |
       kUTF8ContextLookup[(p2 as (i32) + 256i32) as (usize)] as (i32)) as (u8)
    }
    ContextType::CONTEXT_MSB6 => (p1 as (i32) >> 2i32) as (u8),
    ContextType::CONTEXT_LSB6 => (p1 as (i32) & 0x3fi32) as (u8),/* else {
    0i32 as (u8)
    }*/
  }
}

pub fn BrotliBuildHistogramsWithContext<'a,
                                        Alloc: alloc::Allocator<u8> + alloc::Allocator<u32>>
  (cmds: &[Command],
   num_commands: usize,
   literal_split: &BlockSplit<Alloc>,
   insert_and_copy_split: &BlockSplit<Alloc>,
   dist_split: &BlockSplit<Alloc>,
   ringbuffer: &[u8],
   start_pos: usize,
   mask: usize,
   mut prev_byte: u8,
   mut prev_byte2: u8,
   context_modes: &[ContextType],
   literal_histograms: &mut [HistogramLiteral],
   insert_and_copy_histograms: &mut [HistogramCommand],
   copy_dist_histograms: &mut [HistogramDistance]) {
  let mut pos: usize = start_pos;
  let mut literal_it: BlockSplitIterator<Alloc>;
  let mut insert_and_copy_it: BlockSplitIterator<Alloc>;
  let mut dist_it: BlockSplitIterator<Alloc>;
  let mut i: usize;
  literal_it = NewBlockSplitIterator(literal_split);
  insert_and_copy_it = NewBlockSplitIterator(insert_and_copy_split);
  dist_it = NewBlockSplitIterator(dist_split);
  i = 0usize;
  while i < num_commands {
    {
      let cmd = &cmds[(i as (usize))];
      let mut j: usize;
      BlockSplitIteratorNext(&mut insert_and_copy_it);
      HistogramAddItem(&mut insert_and_copy_histograms[(insert_and_copy_it.type_ as (usize))],
                       (*cmd).cmd_prefix_ as (usize));
      j = (*cmd).insert_len_ as (usize);
      while j != 0usize {
        {
          let context: usize;
          BlockSplitIteratorNext(&mut literal_it);
          context = if context_modes.len() != 0 {
            (literal_it.type_ << 6i32).wrapping_add(Context(prev_byte,
                                                            prev_byte2,
                                                            context_modes[(literal_it.type_ as
                                                             (usize))]) as
                                                    (usize))
          } else {
            literal_it.type_
          };
          HistogramAddItem(&mut literal_histograms[(context as (usize))],
                           ringbuffer[((pos & mask) as (usize))] as (usize));
          prev_byte2 = prev_byte;
          prev_byte = ringbuffer[((pos & mask) as (usize))];
          pos = pos.wrapping_add(1 as (usize));
        }
        j = j.wrapping_sub(1 as (usize));
      }
      pos = pos.wrapping_add(CommandCopyLen(cmd) as (usize));
      if CommandCopyLen(cmd) != 0 {
        prev_byte2 = ringbuffer[((pos.wrapping_sub(2usize) & mask) as (usize))];
        prev_byte = ringbuffer[((pos.wrapping_sub(1usize) & mask) as (usize))];
        if (*cmd).cmd_prefix_ as (i32) >= 128i32 {
          let context: usize;
          BlockSplitIteratorNext(&mut dist_it);
          context = (dist_it.type_ << 2i32).wrapping_add(CommandDistanceContext(cmd) as (usize));
          HistogramAddItem(&mut copy_dist_histograms[(context as (usize))],
                           (*cmd).dist_prefix_ as (usize) & 0x3ff);
        }
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
}
