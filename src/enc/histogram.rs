use core::cmp::min;
use super::constants::{kSigned3BitContextLookup, kUTF8ContextLookup};
use super::super::alloc::{SliceWrapper,SliceWrapperMut};
static kBrotliMinWindowBits: i32 = 10i32;

static kBrotliMaxWindowBits: i32 = 24i32;

//#[derive(Clone)] clone is broken for arrays > 32
pub struct HistogramLiteral {
  pub data_: [u32; 256],
  pub total_count_: usize,
  pub bit_cost_: f64,
}
impl Clone for HistogramLiteral {
     fn clone(&self) -> HistogramLiteral {
         return HistogramLiteral{data_:self.data_, total_count_:self.total_count_, bit_cost_:self.bit_cost_};
     }
}
impl Default for HistogramLiteral {
     fn default() -> HistogramLiteral {
         return HistogramLiteral{data_:[0;256], total_count_:0, bit_cost_:3.402e+38f64};
     }
}
//#[derive(Clone)] clone is broken for arrays > 32
pub struct HistogramCommand {
  pub data_: [u32; 704],
  pub total_count_: usize,
  pub bit_cost_: f64,
}
impl Clone for HistogramCommand {
     fn clone(&self) -> HistogramCommand {
         return HistogramCommand{data_:self.data_, total_count_:self.total_count_, bit_cost_:self.bit_cost_};
     }
}
impl Default for HistogramCommand {
     fn default() -> HistogramCommand {
         return HistogramCommand{data_:[0;704], total_count_:0, bit_cost_:3.402e+38f64};
     }
}
//#[derive(Clone)] // #derive is broken for arrays > 32
pub struct HistogramDistance {
  pub data_: [u32; 520],
  pub total_count_: usize,
  pub bit_cost_: f64,
}
impl Clone for HistogramDistance {
     fn clone(&self) -> HistogramDistance {
         return HistogramDistance{data_:self.data_, total_count_:self.total_count_, bit_cost_:self.bit_cost_};
     }
}
impl Default for HistogramDistance {
     fn default() -> HistogramDistance {
         return HistogramDistance{data_:[0;520], total_count_:0, bit_cost_:3.402e+38f64};
     }
}

pub trait CostAccessors {
  fn total_count(&self) -> usize;
  fn bit_cost(&self) -> f64;
  fn set_bit_cost(&mut self, cost: f64);
  fn set_total_count(&mut self, count: usize);
}
impl SliceWrapper<u32> for HistogramLiteral {
  fn slice(&self) -> &[u32] {
    return &self.data_[..];
  }
}
impl SliceWrapperMut<u32> for HistogramLiteral {
  fn slice_mut(&mut self) -> &mut [u32] {
    return &mut self.data_[..];
  }
}
impl CostAccessors for HistogramLiteral {
  fn total_count(&self) -> usize {
    return self.total_count_;
  }
  fn bit_cost(&self) -> f64 {
    return self.bit_cost_;
  }
  fn set_bit_cost(&mut self, data: f64) {
    self.bit_cost_ = data;
  }
  fn set_total_count(&mut self, data: usize) {
    self.total_count_ = data;
  }
}

impl SliceWrapper<u32> for HistogramCommand {
  fn slice(&self) -> &[u32] {
    return &self.data_[..];
  }
}
impl SliceWrapperMut<u32> for HistogramCommand {
  fn slice_mut(&mut self) -> &mut [u32] {
    return &mut self.data_[..];
  }
}

impl CostAccessors for HistogramCommand {
  fn total_count(&self) -> usize {
    return self.total_count_;
  }
  fn bit_cost(&self) -> f64 {
    return self.bit_cost_;
  }
  fn set_bit_cost(&mut self, data: f64) {
    self.bit_cost_ = data;
  }
  fn set_total_count(&mut self, data: usize) {
    self.total_count_ = data;
  }
}

impl SliceWrapper<u32> for HistogramDistance {
  fn slice(&self) -> &[u32] {
    return &self.data_[..];
  }
}
impl SliceWrapperMut<u32> for HistogramDistance {
  fn slice_mut(&mut self) -> &mut [u32] {
    return &mut self.data_[..];
  }
}
impl CostAccessors for HistogramDistance {
  fn total_count(&self) -> usize {
    return self.total_count_;
  }
  fn bit_cost(&self) -> f64 {
    return self.bit_cost_;
  }
  fn set_bit_cost(&mut self, data: f64) {
    self.bit_cost_ = data;
  }
  fn set_total_count(&mut self, data: usize) {
    self.total_count_ = data;
  }
}


pub struct Command {
  pub insert_len_: u32,
  pub copy_len_: u32,
  pub dist_extra_: u32,
  pub cmd_prefix_: u16,
  pub dist_prefix_: u16,
}

pub struct BlockSplit<'a> {
  pub num_types: usize,
  pub num_blocks: usize,
  pub types: &'a mut [u8],
  pub lengths: &'a mut [u32],
  pub types_alloc_size: usize,
  pub lengths_alloc_size: usize,
}

#[derive(Copy,Clone)]
pub enum ContextType {
  CONTEXT_LSB6 = 0,
  CONTEXT_MSB6 = 1,
  CONTEXT_UTF8 = 2,
  CONTEXT_SIGNED = 3,
}



pub struct BlockSplitIterator<'a> {
  pub split_: &'a BlockSplit<'a>,
  pub idx_: usize,
  pub type_: usize,
  pub length_: usize,
}



fn NewBlockSplitIterator<'a>(mut split: &'a BlockSplit) -> BlockSplitIterator<'a> {
  return BlockSplitIterator::<'a> {
           split_: split,
           idx_: 0i32 as (usize),
           type_: 0i32 as (usize),
           length_: if (*split).lengths.len() != 0 {
             (*split).lengths[0] as usize
           } else {
             0i32 as (usize)
           },
         };
}


fn InitBlockSplitIterator<'a>(mut xself: &'a mut BlockSplitIterator<'a>,
                              mut split: &'a BlockSplit) {
  (*xself).split_ = split;
  (*xself).idx_ = 0i32 as (usize);
  (*xself).type_ = 0i32 as (usize);
  (*xself).length_ = if (*split).lengths.len() != 0 {
    (*split).lengths[0] as u32
  } else {
    0i32 as (u32)
  } as (usize);
}

fn BlockSplitIteratorNext(mut xself: &mut BlockSplitIterator) {
  if (*xself).length_ == 0i32 as (usize) {
    (*xself).idx_ = (*xself).idx_.wrapping_add(1 as (usize));
    (*xself).type_ = (*(*xself).split_).types[(*xself).idx_ as (usize)] as (usize);
    (*xself).length_ = (*(*xself).split_).lengths[(*xself).idx_ as (usize)] as (usize);
  }
  (*xself).length_ = (*xself).length_.wrapping_sub(1 as (usize));
}
pub fn HistogramAddItem<HistogramType:SliceWrapper<u32>+SliceWrapperMut<u32> +CostAccessors>(mut xself: &mut HistogramType, mut val: usize) {
  {
    let _rhs = 1;
    let _lhs = &mut (*xself).slice_mut()[val];
    let val = (*_lhs).wrapping_add(_rhs as (u32));
    *_lhs = val;
  }
  let new_count = (*xself).total_count().wrapping_add(1 as (usize));
  (*xself).set_total_count(new_count);
}
pub fn HistogramClear<HistogramType:SliceWrapperMut<u32>+CostAccessors>(mut xself: &mut HistogramType) {
  for data_elem in xself.slice_mut().iter_mut() {
      *data_elem = 0;
  }
  (*xself).set_total_count(0);
  (*xself).set_bit_cost(3.402e+38f64);
}

pub fn HistogramAddHistogram<HistogramType:SliceWrapperMut<u32> + SliceWrapper<u32> + CostAccessors>(
    mut xself : &mut HistogramType, mut v : &HistogramType
) {
    let old_total_count = (*xself).total_count();
    (*xself).set_total_count(old_total_count + (*v).total_count());
    let mut h0 = xself.slice_mut();
    let h1 = v.slice();
    let n = min(h0.len(), h1.len());
    for i in 0..n {
        let mut h0val = &mut h0[i];
        let val = h0val.wrapping_add(h1[i]);
        *h0val = val;
    }
}
pub fn HistogramSelfAddHistogram<HistogramType:SliceWrapperMut<u32> + SliceWrapper<u32> + CostAccessors>(
    mut xself : &mut [HistogramType], i0 : usize, i1 : usize
) {
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

fn Context(mut p1: u8, mut p2: u8, mode: ContextType) -> u8 {
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

fn CommandCopyLen(mut xself: &Command) -> u32 {
  (*xself).copy_len_ & 0xffffffi32 as (u32)
}

fn CommandDistanceContext(mut xself: &Command) -> u32 {
  let mut r: u32 = ((*xself).cmd_prefix_ as (i32) >> 6i32) as (u32);
  let mut c: u32 = ((*xself).cmd_prefix_ as (i32) & 7i32) as (u32);
  if (r == 0i32 as (u32) || r == 2i32 as (u32) || r == 4i32 as (u32) || r == 7i32 as (u32)) &&
     (c <= 2i32 as (u32)) {
    c
  } else {
    3i32 as (u32)
  }
}

#[no_mangle]
extern fn BrotliBuildHistogramsWithContext(
    mut cmds : &[Command],
    num_commands : usize,
    mut literal_split : &BlockSplit,
    mut insert_and_copy_split : &BlockSplit,
    mut dist_split : &BlockSplit,
    mut ringbuffer : &[u8],
    mut start_pos : usize,
    mut mask : usize,
    mut prev_byte : u8,
    mut prev_byte2 : u8,
    mut context_modes : &[ContextType],
    mut literal_histograms : &mut [HistogramLiteral],
    mut insert_and_copy_histograms : &mut [HistogramCommand],
    mut copy_dist_histograms : &mut [HistogramDistance]
){
  let mut pos: usize = start_pos;
  let mut literal_it: BlockSplitIterator;
  let mut insert_and_copy_it: BlockSplitIterator;
  let mut dist_it: BlockSplitIterator;
  let mut i: usize;
  literal_it = NewBlockSplitIterator(literal_split);
  insert_and_copy_it = NewBlockSplitIterator(insert_and_copy_split);
  dist_it = NewBlockSplitIterator(dist_split);
  /*
    InitBlockSplitIterator(
        &mut literal_it,
        literal_split
    );
    InitBlockSplitIterator(
        &mut insert_and_copy_it,
        insert_and_copy_split
    );
    InitBlockSplitIterator(
        &mut dist_it,
        dist_split
    );*/
    /*FIXME
  i = 0i32 as (usize);
  'loop1: loop {
    if i < num_commands {
      let cmd = &cmds[i as (usize)];
      let mut j: usize;
      BlockSplitIteratorNext(&mut insert_and_copy_it);
      HistogramAddItem(&mut insert_and_copy_histograms[insert_and_copy_it.type_ as (usize)],
                          (*cmd).cmd_prefix_ as (usize));
      j = (*cmd).insert_len_ as (usize);
      'loop4: loop {
        if j != 0i32 as (usize) {
          let mut context: usize;
          BlockSplitIteratorNext(&mut literal_it);
          context = if context_modes.len() != 0 {
            (literal_it.type_ << 6i32).wrapping_add(Context(prev_byte,
                                                            prev_byte2,
                                                            context_modes[literal_it.type_ as
                                                            (usize)]) as
                                                    (usize))
          } else {
            literal_it.type_
          };
          HistogramAddItem(&mut literal_histograms[context as (usize)],
                              ringbuffer[(pos & mask) as (usize)] as (usize));
          prev_byte2 = prev_byte;
          prev_byte = ringbuffer[(pos & mask) as (usize)];
          pos = pos.wrapping_add(1 as (usize));
          j = j.wrapping_sub(1 as (usize));
          continue 'loop4;
        } else {
          break 'loop4;
        }
      }
      pos = pos.wrapping_add(CommandCopyLen(cmd) as (usize));
      if CommandCopyLen(cmd) != 0 {
        prev_byte2 = ringbuffer[(pos.wrapping_sub(2i32 as (usize)) & mask) as (usize)];
        prev_byte = ringbuffer[(pos.wrapping_sub(1i32 as (usize)) & mask) as (usize)];
        if (*cmd).cmd_prefix_ as (i32) >= 128i32 {
          let mut context: usize;
          BlockSplitIteratorNext(&mut dist_it);
          context = (dist_it.type_ << 2i32).wrapping_add(CommandDistanceContext(cmd) as (usize));
          HistogramAddItem(&mut copy_dist_histograms[context as (usize)],
                               (*cmd).dist_prefix_ as (usize));
        }
      }
      i = i.wrapping_add(1 as (usize));
      continue 'loop1;
    } else {
      break 'loop1;
    }
  }*/
}
