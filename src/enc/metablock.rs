#![allow(dead_code)]
use super::vectorization::Mem256f;
use super::backward_references::BrotliEncoderParams;
use super::bit_cost::BitsEntropy;
use super::block_split::BlockSplit;
use super::block_splitter::BrotliSplitBlock;
use super::brotli_bit_stream::MetaBlockSplit;
use super::cluster::BrotliClusterHistograms;
use super::cluster::HistogramPair;
use super::command::{Command, CommandCopyLen};
use super::entropy_encode::BrotliOptimizeHuffmanCountsForRle;
use super::histogram::{BrotliBuildHistogramsWithContext, CostAccessors, HistogramLiteral,
                       HistogramCommand, HistogramDistance, HistogramClear, ClearHistograms,
                       ContextType, HistogramAddHistogram, HistogramAddItem, Context};
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::util::{brotli_min_size_t, brotli_max_size_t};
use core;


pub fn BrotliBuildMetaBlock<AllocU8: alloc::Allocator<u8>,
                            AllocU16: alloc::Allocator<u16>,
                            AllocU32: alloc::Allocator<u32>,
                            AllocF64: alloc::Allocator<super::util::floatX>,
                            AllocFV:alloc::Allocator<Mem256f>,
                            AllocHL: alloc::Allocator<HistogramLiteral>,
                            AllocHC: alloc::Allocator<HistogramCommand>,
                            AllocHD: alloc::Allocator<HistogramDistance>,
                            AllocHP: alloc::Allocator<HistogramPair>,
                            AllocCT: alloc::Allocator<ContextType>>
  (m8:   &mut AllocU8,
   m16:  &mut AllocU16,
   m32:  &mut AllocU32,
   mf64: &mut AllocF64,
   mfv:  &mut AllocFV,
   mhl:  &mut AllocHL,
   mhc:  &mut AllocHC,
   mhd:  &mut AllocHD,
   mhp:  &mut AllocHP,
   mct:  &mut AllocCT,
   ringbuffer: &[u8],
   pos: usize,
   mask: usize,
   params: &BrotliEncoderParams,
   prev_byte: u8,
   prev_byte2: u8,
   cmds: &[Command],
   num_commands: usize,
   literal_context_mode: ContextType,
   lit_scratch_space: &mut <HistogramLiteral as CostAccessors>::i32vec,
   cmd_scratch_space: &mut <HistogramCommand as CostAccessors>::i32vec,
   dst_scratch_space: &mut <HistogramDistance as CostAccessors>::i32vec,
   mb: &mut MetaBlockSplit<AllocU8, AllocU32, AllocHL, AllocHC, AllocHD>) {

  static kMaxNumberOfHistograms: usize = 256usize;
  let mut distance_histograms: AllocHD::AllocatedMemory;
  let mut literal_histograms: AllocHL::AllocatedMemory;
  let mut literal_context_modes: AllocCT::AllocatedMemory = AllocCT::AllocatedMemory::default();
  let literal_histograms_size: usize;
  let distance_histograms_size: usize;
  let mut i: usize;
  let mut literal_context_multiplier: usize = 1usize;
  BrotliSplitBlock(m8,
                   m16,
                   m32,
                   mf64,
                   mfv,
                   mhl,
                   mhc,
                   mhd,
                   mhp,
                   cmds,
                   num_commands,
                   ringbuffer,
                   pos,
                   mask,
                   params,
                   lit_scratch_space,
                   cmd_scratch_space,
                   dst_scratch_space,
                   &mut (*mb).literal_split,
                   &mut (*mb).command_split,
                   &mut (*mb).distance_split);
  if (*params).disable_literal_context_modeling == 0 {
    literal_context_multiplier = (1i32 << 6i32) as (usize);
    literal_context_modes = mct.alloc_cell((*mb).literal_split.num_types);
    for item in literal_context_modes.slice_mut().iter_mut() {
      *item = literal_context_mode;
    }
  }
  literal_histograms_size = (*mb).literal_split.num_types.wrapping_mul(literal_context_multiplier);
  literal_histograms = mhl.alloc_cell(literal_histograms_size);
  distance_histograms_size = (*mb).distance_split.num_types << 2i32;
  distance_histograms = mhd.alloc_cell(distance_histograms_size);
  (*mb).command_histograms_size = (*mb).command_split.num_types;
  (*mb).command_histograms = mhc.alloc_cell((*mb).command_histograms_size);
  BrotliBuildHistogramsWithContext(cmds,
                                   num_commands,
                                   &mut (*mb).literal_split,
                                   &mut (*mb).command_split,
                                   &mut (*mb).distance_split,
                                   ringbuffer,
                                   pos,
                                   mask,
                                   prev_byte,
                                   prev_byte2,
                                   literal_context_modes.slice(),
                                   literal_histograms.slice_mut(),
                                   (*mb).command_histograms.slice_mut(),
                                   distance_histograms.slice_mut());
  mct.free_cell(literal_context_modes);
  (*mb).literal_context_map_size = (*mb).literal_split.num_types << 6i32;
  (*mb).literal_context_map = m32.alloc_cell((*mb).literal_context_map_size);
  (*mb).literal_histograms_size = (*mb).literal_context_map_size;
  (*mb).literal_histograms = mhl.alloc_cell((*mb).literal_histograms_size);
  BrotliClusterHistograms(m32,
                          mhp,
                          mhl,
                          literal_histograms.slice(),
                          literal_histograms_size,
                          kMaxNumberOfHistograms,
                          lit_scratch_space,
                          (*mb).literal_histograms.slice_mut(),
                          &mut (*mb).literal_histograms_size,
                          (*mb).literal_context_map.slice_mut());
  mhl.free_cell(literal_histograms);
  if (*params).disable_literal_context_modeling != 0 {
    i = (*mb).literal_split.num_types;
    while i != 0usize {
      let mut j: usize = 0usize;
      i = i.wrapping_sub(1 as (usize));
      while j < (1i32 << 6i32) as (usize) {
        {
          let val = (*mb).literal_context_map.slice()[(i as (usize))];
          (*mb).literal_context_map.slice_mut()[((i << 6i32).wrapping_add(j) as (usize))] = val;
        }
        j = j.wrapping_add(1 as (usize));
      }
    }
  }
  (*mb).distance_context_map_size = (*mb).distance_split.num_types << 2i32;
  (*mb).distance_context_map = m32.alloc_cell((*mb).distance_context_map_size);
  (*mb).distance_histograms_size = (*mb).distance_context_map_size;
  (*mb).distance_histograms = mhd.alloc_cell((*mb).distance_histograms_size);
  BrotliClusterHistograms(m32,
                          mhp,
                          mhd,
                          distance_histograms.slice(),
                          (*mb).distance_context_map_size,
                          kMaxNumberOfHistograms,
                          dst_scratch_space,
                          (*mb).distance_histograms.slice_mut(),
                          &mut (*mb).distance_histograms_size,
                          (*mb).distance_context_map.slice_mut());
  mhd.free_cell(distance_histograms);
}

/*
pub struct BlockSplitter<'a, HistogramType:SliceWrapper<u32>+SliceWrapperMut<u32> +CostAccessors,
                         AllocU8:alloc::Allocator<u8>+'a,
                         AllocU32:alloc::Allocator<u32>+'a,
                         AllocHT:alloc::Allocator<HistogramType>+'a > {
                         */
pub struct BlockSplitter {
  pub alphabet_size_: usize,
  pub min_block_size_: usize,
  pub split_threshold_: super::util::floatX,
  pub num_blocks_: usize,
  //  pub split_: &'a mut BlockSplit<AllocU8, AllocU32>,
  //  pub histograms_: AllocHT::AllocatedMemory, // FIXME: pull this one out at the end
  //  pub histograms_size_: &'a mut usize, // FIXME: pull this one out at the end
  pub target_block_size_: usize,
  pub block_size_: usize,
  pub curr_histogram_ix_: usize,
  pub last_histogram_ix_: [usize; 2],
  pub last_entropy_: [super::util::floatX; 2],
  pub merge_last_count_: usize,
}



pub struct ContextBlockSplitter {
  pub alphabet_size_: usize,
  pub num_contexts_: usize,
  pub max_block_types_: usize,
  pub min_block_size_: usize,
  pub split_threshold_: super::util::floatX,
  pub num_blocks_: usize,
  //  pub split_: &'a mut BlockSplit<AllocU8, AllocU32>,
  //  pub histograms_: AllocHL::AllocatedMemory,
  //  pub histograms_size_: &'a mut usize, // FIXME: pull this one out at the end
  pub target_block_size_: usize,
  pub block_size_: usize,
  pub curr_histogram_ix_: usize,
  pub last_histogram_ix_: [usize; 2],
  pub last_entropy_: [super::util::floatX; 2 * BROTLI_MAX_STATIC_CONTEXTS],
  pub merge_last_count_: usize,
}



enum LitBlocks {
  plain(BlockSplitter), //<'a, HistogramLiteral, AllocU8, AllocU32, AllocHL>,
  ctx(ContextBlockSplitter), //<'a, AllocU8, AllocU32, AllocHL>,
}

/*

pub struct BlockSplitterCommand {
  pub alphabet_size_: usize,
  pub min_block_size_: usize,
  pub split_threshold_: super::util::floatX,
  pub num_blocks_: usize,
  pub split_: *mut BlockSplit,
  pub histograms_: *mut HistogramCommand,
  pub histograms_size_: *mut usize,
  pub target_block_size_: usize,
  pub block_size_: usize,
  pub curr_histogram_ix_: usize,
  pub last_histogram_ix_: [usize; 2],
  pub last_entropy_: [super::util::floatX; 2],
  pub merge_last_count_: usize,
}



pub struct BlockSplitterDistance {
  pub alphabet_size_: usize,
  pub min_block_size_: usize,
  pub split_threshold_: super::util::floatX,
  pub num_blocks_: usize,
  pub split_: *mut BlockSplit,
  pub histograms_: *mut HistogramDistance,
  pub histograms_size_: *mut usize,
  pub target_block_size_: usize,
  pub block_size_: usize,
  pub curr_histogram_ix_: usize,
  pub last_histogram_ix_: [usize; 2],
  pub last_entropy_: [super::util::floatX; 2],
  pub merge_last_count_: usize,
}
*/

fn InitBlockSplitter<HistogramType: SliceWrapper<u32> + SliceWrapperMut<u32> + CostAccessors,
                     AllocU8: alloc::Allocator<u8>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocHT: alloc::Allocator<HistogramType>>
  (m8:  &mut AllocU8,
   m32: &mut AllocU32,
   mht: &mut AllocHT,
   alphabet_size: usize,
   min_block_size: usize,
   split_threshold: super::util::floatX,
   num_symbols: usize,
   split: &mut BlockSplit<AllocU8, AllocU32>,
   histograms: &mut AllocHT::AllocatedMemory,
   histograms_size: &mut usize)
   -> BlockSplitter {
  let max_num_blocks: usize = num_symbols.wrapping_div(min_block_size).wrapping_add(1usize);
  let max_num_types: usize = brotli_min_size_t(max_num_blocks, (256i32 + 1i32) as (usize));
  let mut xself = BlockSplitter {
    last_entropy_: [0.0 as super::util::floatX; 2],
    alphabet_size_: alphabet_size,
    min_block_size_: min_block_size,
    split_threshold_: split_threshold,
    num_blocks_: 0usize,
    //(*xself).split_ : split,
    //(*xself).histograms_size_ : histograms_size,
    target_block_size_: min_block_size,
    block_size_: 0usize,
    curr_histogram_ix_: 0usize,
    merge_last_count_: 0usize,
    last_histogram_ix_: [0usize; 2],
  };
  {
    if (*split).types.slice().len() < max_num_blocks {
      let mut _new_size: usize = if (*split).types.slice().len() == 0usize {
        max_num_blocks
      } else {
        (*split).types.slice().len()
      };
      let mut new_array: AllocU8::AllocatedMemory;
      while _new_size < max_num_blocks {
        _new_size = _new_size.wrapping_mul(2usize);
      }
      new_array = m8.alloc_cell(_new_size);
      if ((*split).types.slice().len() != 0usize) {
        new_array.slice_mut()[..(*split).types.slice().len()].clone_from_slice((*split)
                                                                                 .types
                                                                                 .slice());
      }
      m8.free_cell(core::mem::replace(&mut (*split).types, new_array));
    }
  }
  {
    if (*split).lengths.slice().len() < max_num_blocks {
      let mut _new_size: usize = if (*split).lengths.slice().len() == 0usize {
        max_num_blocks
      } else {
        (*split).lengths.slice().len()
      };
      while _new_size < max_num_blocks {
        _new_size = _new_size.wrapping_mul(2usize);
      }
      let mut new_array = m32.alloc_cell(_new_size);
      new_array.slice_mut()[..(*split).lengths.slice().len()].clone_from_slice((*split)
                                                                                 .lengths
                                                                                 .slice());
      m32.free_cell(core::mem::replace(&mut (*split).lengths, new_array));
    }
  }
  (*split).num_blocks = max_num_blocks;
  *histograms_size = max_num_types;
  let hlocal = mht.alloc_cell(*histograms_size);
  mht.free_cell(core::mem::replace(&mut *histograms, hlocal));
  HistogramClear(&mut histograms.slice_mut()[0]);
  xself.last_histogram_ix_[0] = 0;
  xself.last_histogram_ix_[1] = 0;
  return xself;
}
fn InitContextBlockSplitter<AllocU8: alloc::Allocator<u8>,
                            AllocU32: alloc::Allocator<u32>,
                            AllocHL: alloc::Allocator<HistogramLiteral>>
  (m8:  &mut AllocU8,
   m32: &mut AllocU32,
   mhl: &mut AllocHL,
   alphabet_size: usize,
   num_contexts: usize,
   min_block_size: usize,
   split_threshold: super::util::floatX,
   num_symbols: usize,
   split: &mut BlockSplit<AllocU8, AllocU32>,
   histograms: &mut AllocHL::AllocatedMemory,
   histograms_size: &mut usize)
   -> ContextBlockSplitter {
  let max_num_blocks: usize = num_symbols.wrapping_div(min_block_size).wrapping_add(1usize);
  let max_num_types: usize;
  assert!(num_contexts <= BROTLI_MAX_STATIC_CONTEXTS);
  let mut xself = ContextBlockSplitter {
    alphabet_size_: alphabet_size,
    num_contexts_: num_contexts,
    max_block_types_: (256usize).wrapping_div(num_contexts),
    min_block_size_: min_block_size,
    split_threshold_: split_threshold,
    num_blocks_: 0usize,
    //        histograms_size_: histograms_size,
    target_block_size_: min_block_size,
    block_size_: 0usize,
    curr_histogram_ix_: 0usize,
    merge_last_count_: 0usize,
    last_histogram_ix_: [0; 2],
    last_entropy_: [0.0 as super::util::floatX; 2 * BROTLI_MAX_STATIC_CONTEXTS],
  };
  max_num_types = brotli_min_size_t(max_num_blocks, xself.max_block_types_.wrapping_add(1usize));
  {
    if (*split).types.slice().len() < max_num_blocks {
      let mut _new_size: usize = if (*split).types.slice().len() == 0usize {
        max_num_blocks
      } else {
        (*split).types.slice().len()
      };
      while _new_size < max_num_blocks {
        _new_size = _new_size.wrapping_mul(2usize);
      }
      let mut new_array = m8.alloc_cell(_new_size);
      if ((*split).types.slice().len() != 0usize) {
        new_array.slice_mut()[..(*split).types.slice().len()].clone_from_slice((*split)
                                                                                 .types
                                                                                 .slice());
      }
      m8.free_cell(core::mem::replace(&mut (*split).types, new_array));
    }
  }
  {
    if (*split).lengths.slice().len() < max_num_blocks {
      let mut _new_size: usize = if (*split).lengths.slice().len() == 0usize {
        max_num_blocks
      } else {
        (*split).lengths.slice().len()
      };
      while _new_size < max_num_blocks {
        _new_size = _new_size.wrapping_mul(2usize);
      }
      let mut new_array = m32.alloc_cell(_new_size);
      if ((*split).lengths.slice().len() != 0usize) {
        new_array.slice_mut()[..(*split).lengths.slice().len()].clone_from_slice((*split)
                                                                                   .lengths
                                                                                   .slice());
      }
      m32.free_cell(core::mem::replace(&mut (*split).lengths, new_array));
    }
  }
  (*split).num_blocks = max_num_blocks;
  *histograms_size = max_num_types.wrapping_mul(num_contexts);
  *histograms = mhl.alloc_cell(*histograms_size);
  //(*xself).histograms_ = *histograms;
  ClearHistograms(&mut histograms.slice_mut()[(0usize)..], num_contexts);
  xself.last_histogram_ix_[0] = 0;
  xself.last_histogram_ix_[1] = 0;
  return xself;
}

fn BlockSplitterFinishBlock<HistogramType:SliceWrapper<u32>
                                          +SliceWrapperMut<u32>
                                          +CostAccessors
                                          +Clone,
                            AllocU8:alloc::Allocator<u8>,
                            AllocU32:alloc::Allocator<u32>>(xself: &mut BlockSplitter,
                                                            split: &mut BlockSplit<AllocU8,
                                                                                       AllocU32>,
                                                            histograms: &mut [HistogramType],
                                                            histograms_size: &mut usize,
is_final: i32){
  (*xself).block_size_ = brotli_max_size_t((*xself).block_size_, (*xself).min_block_size_);
  if (*xself).num_blocks_ == 0usize {
    (*split).lengths.slice_mut()[(0usize)] = (*xself).block_size_ as (u32);
    (*split).types.slice_mut()[(0usize)] = 0i32 as (u8);
    (*xself).last_entropy_[(0usize)] = BitsEntropy((histograms[(0usize)]).slice(),
                                                   (*xself).alphabet_size_);
    (*xself).last_entropy_[(1usize)] = (*xself).last_entropy_[(0usize)];
    (*xself).num_blocks_ = (*xself).num_blocks_.wrapping_add(1 as (usize));
    (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
    (*xself).curr_histogram_ix_ = (*xself).curr_histogram_ix_.wrapping_add(1 as (usize));
    if (*xself).curr_histogram_ix_ < *histograms_size {
      HistogramClear(&mut histograms[((*xself).curr_histogram_ix_ as (usize))]);
    }
    (*xself).block_size_ = 0usize;
  } else if (*xself).block_size_ > 0usize {
    let entropy: super::util::floatX = BitsEntropy((histograms[((*xself).curr_histogram_ix_ as (usize))]).slice(),
                                   (*xself).alphabet_size_);
    let mut combined_histo: [HistogramType; 2] = [histograms[(*xself).curr_histogram_ix_].clone(),
                                                  histograms[(*xself).curr_histogram_ix_].clone()];

    let mut combined_entropy: [super::util::floatX; 2] = [0.0 as super::util::floatX, 0.0 as super::util::floatX];
    let mut diff: [super::util::floatX; 2] = [0.0 as super::util::floatX, 0.0 as super::util::floatX];
    for j in 0..2 {
      {
        let last_histogram_ix: usize = (*xself).last_histogram_ix_[j];
        HistogramAddHistogram(&mut combined_histo[j],
                              &histograms[(last_histogram_ix as (usize))]);
        combined_entropy[j] = BitsEntropy(&mut combined_histo[j].slice_mut()[0usize..],
                                          (*xself).alphabet_size_);
        diff[j] = combined_entropy[j] - entropy - (*xself).last_entropy_[(j as (usize))];
      }
    }
    if (*split).num_types < 256usize && (diff[0usize] > (*xself).split_threshold_) &&
       (diff[1usize] > (*xself).split_threshold_) {
      (*split).lengths.slice_mut()[((*xself).num_blocks_ as (usize))] = (*xself).block_size_ as
                                                                        (u32);
      (*split).types.slice_mut()[((*xself).num_blocks_ as (usize))] = (*split).num_types as (u8);
      (*xself).last_histogram_ix_[1usize] = (*xself).last_histogram_ix_[0usize];
      (*xself).last_histogram_ix_[0usize] = (*split).num_types as (u8) as (usize);
      (*xself).last_entropy_[(1usize)] = (*xself).last_entropy_[(0usize)];
      (*xself).last_entropy_[(0usize)] = entropy;
      (*xself).num_blocks_ = (*xself).num_blocks_.wrapping_add(1 as (usize));
      (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
      (*xself).curr_histogram_ix_ = (*xself).curr_histogram_ix_.wrapping_add(1 as (usize));
      if (*xself).curr_histogram_ix_ < *histograms_size {
        HistogramClear(&mut histograms[((*xself).curr_histogram_ix_ as (usize))]);
      }
      (*xself).block_size_ = 0usize;
      (*xself).merge_last_count_ = 0usize;
      (*xself).target_block_size_ = (*xself).min_block_size_;
    } else if diff[1usize] < diff[0usize] - 20.0 as super::util::floatX {
      (*split).lengths.slice_mut()[((*xself).num_blocks_ as (usize))] = (*xself).block_size_ as
                                                                        (u32);
      (*split).types.slice_mut()[((*xself).num_blocks_ as (usize))] = (*split).types.slice()
        [((*xself).num_blocks_.wrapping_sub(2usize) as (usize))]; //FIXME: investigate copy?
      {
        let mut __brotli_swap_tmp: usize = (*xself).last_histogram_ix_[0usize];
        (*xself).last_histogram_ix_[0usize] = (*xself).last_histogram_ix_[1usize];
        (*xself).last_histogram_ix_[1usize] = __brotli_swap_tmp;
      }
      histograms[((*xself).last_histogram_ix_[0usize] as (usize))] = combined_histo[1usize].clone();
      (*xself).last_entropy_[(1usize)] = (*xself).last_entropy_[(0usize)];
      (*xself).last_entropy_[(0usize)] = combined_entropy[1usize];
      (*xself).num_blocks_ = (*xself).num_blocks_.wrapping_add(1 as (usize));
      (*xself).block_size_ = 0usize;
      HistogramClear(&mut histograms[((*xself).curr_histogram_ix_ as (usize))]);
      (*xself).merge_last_count_ = 0usize;
      (*xself).target_block_size_ = (*xself).min_block_size_;
    } else {
      {
        let _rhs = (*xself).block_size_ as (u32);
        let _lhs = &mut (*split).lengths.slice_mut()[((*xself).num_blocks_.wrapping_sub(1usize) as
                         (usize))];
        *_lhs = (*_lhs).wrapping_add(_rhs);
      }
      histograms[((*xself).last_histogram_ix_[0usize] as (usize))] = combined_histo[0usize].clone();
      (*xself).last_entropy_[(0usize)] = combined_entropy[0usize];
      if (*split).num_types == 1usize {
        (*xself).last_entropy_[(1usize)] = (*xself).last_entropy_[(0usize)];
      }
      (*xself).block_size_ = 0usize;
      HistogramClear(&mut histograms[((*xself).curr_histogram_ix_ as (usize))]);
      if {
           (*xself).merge_last_count_ = (*xself).merge_last_count_.wrapping_add(1 as (usize));
           (*xself).merge_last_count_
         } > 1usize {
        (*xself).target_block_size_ =
          (*xself).target_block_size_.wrapping_add((*xself).min_block_size_);
      }
    }
  }
  if is_final != 0 {
    *histograms_size = (*split).num_types;
    (*split).num_blocks = (*xself).num_blocks_;
  }
}
const BROTLI_MAX_STATIC_CONTEXTS: usize = 13;

fn ContextBlockSplitterFinishBlock<AllocU8: alloc::Allocator<u8>, AllocU32: alloc::Allocator<u32>, AllocHL: alloc::Allocator<HistogramLiteral>>
  (xself: &mut ContextBlockSplitter,
   m : &mut AllocHL,
   split: &mut BlockSplit<AllocU8, AllocU32>,
   histograms: &mut [HistogramLiteral],
   histograms_size: &mut usize,
   is_final: i32) {
  let num_contexts: usize = (*xself).num_contexts_;
  if (*xself).block_size_ < (*xself).min_block_size_ {
    (*xself).block_size_ = (*xself).min_block_size_;
  }
  if (*xself).num_blocks_ == 0usize {
    let mut i: usize;
    (*split).lengths.slice_mut()[(0usize)] = (*xself).block_size_ as (u32);
    (*split).types.slice_mut()[(0usize)] = 0i32 as (u8);
    i = 0usize;
    while i < num_contexts {
      {
        (*xself).last_entropy_[(i as (usize))] = BitsEntropy((histograms[(i as (usize))]).slice(),
                                                             (*xself).alphabet_size_);
        (*xself).last_entropy_[(num_contexts.wrapping_add(i) as (usize))] = (*xself).last_entropy_
          [(i as (usize))];
      }
      i = i.wrapping_add(1 as (usize));
    }
    (*xself).num_blocks_ = (*xself).num_blocks_.wrapping_add(1 as (usize));
    (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
    (*xself).curr_histogram_ix_ = (*xself).curr_histogram_ix_.wrapping_add(num_contexts);
    if (*xself).curr_histogram_ix_ < *histograms_size {
      ClearHistograms(&mut histograms[((*xself).curr_histogram_ix_ as (usize))..],
                      (*xself).num_contexts_);
    }
    (*xself).block_size_ = 0usize;
  } else if (*xself).block_size_ > 0usize {
    let mut entropy = [0.0 as super::util::floatX; BROTLI_MAX_STATIC_CONTEXTS];
    let mut combined_histo = m.alloc_cell(2 * num_contexts);
      let mut combined_entropy = [0.0 as super::util::floatX; 2 * BROTLI_MAX_STATIC_CONTEXTS];
    let mut diff: [super::util::floatX; 2] = [0.0 as super::util::floatX; 2];
    let mut i: usize;
    i = 0usize;
    while i < num_contexts {
      {
        let curr_histo_ix: usize = (*xself).curr_histogram_ix_.wrapping_add(i);
        let mut j: usize;
        entropy[i] = BitsEntropy(&(histograms[(curr_histo_ix as (usize))]).slice(),
                                 (*xself).alphabet_size_);
        j = 0usize;
        while j < 2usize {
          {
            let jx: usize = j.wrapping_mul(num_contexts).wrapping_add(i);
            let last_histogram_ix: usize = (*xself).last_histogram_ix_[j].wrapping_add(i);
            combined_histo.slice_mut()[jx] = histograms[(curr_histo_ix as (usize))].clone();
            HistogramAddHistogram(&mut combined_histo.slice_mut()[jx],
                                  &mut histograms[(last_histogram_ix as (usize))]);
            combined_entropy[jx] = BitsEntropy(combined_histo.slice()[jx].slice(), (*xself).alphabet_size_);
            {
              let _rhs = combined_entropy[jx] - entropy[i] -
                         (*xself).last_entropy_[(jx as (usize))];
              let _lhs = &mut diff[j];
              *_lhs = *_lhs + _rhs;
            }
          }
          j = j.wrapping_add(1 as (usize));
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
    if (*split).num_types < (*xself).max_block_types_ &&
       (diff[0usize] > (*xself).split_threshold_) &&
       (diff[1usize] > (*xself).split_threshold_) {
      (*split).lengths.slice_mut()[((*xself).num_blocks_ as (usize))] = (*xself).block_size_ as
                                                                        (u32);
      (*split).types.slice_mut()[((*xself).num_blocks_ as (usize))] = (*split).num_types as (u8);
      (*xself).last_histogram_ix_[1usize] = (*xself).last_histogram_ix_[0usize];
      (*xself).last_histogram_ix_[0usize] = (*split).num_types.wrapping_mul(num_contexts);
      i = 0usize;
      while i < num_contexts {
        {
          (*xself).last_entropy_[(num_contexts.wrapping_add(i) as (usize))] =
            (*xself).last_entropy_[(i as (usize))];
          (*xself).last_entropy_[(i as (usize))] = entropy[i];
        }
        i = i.wrapping_add(1 as (usize));
      }
      (*xself).num_blocks_ = (*xself).num_blocks_.wrapping_add(1 as (usize));
      (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
      (*xself).curr_histogram_ix_ = (*xself).curr_histogram_ix_.wrapping_add(num_contexts);
      if (*xself).curr_histogram_ix_ < *histograms_size {
        ClearHistograms(&mut histograms[((*xself).curr_histogram_ix_ as (usize))..],
                        (*xself).num_contexts_);
      }
      (*xself).block_size_ = 0usize;
      (*xself).merge_last_count_ = 0usize;
      (*xself).target_block_size_ = (*xself).min_block_size_;
    } else if diff[1usize] < diff[0usize] - 20.0 as super::util::floatX {
      (*split).lengths.slice_mut()[((*xself).num_blocks_ as (usize))] = (*xself).block_size_ as
                                                                        (u32);
      let nbm2 = (*split).types.slice()[((*xself).num_blocks_.wrapping_sub(2usize) as (usize))];
      (*split).types.slice_mut()[((*xself).num_blocks_ as (usize))] = nbm2;

      {
        let mut __brotli_swap_tmp: usize = (*xself).last_histogram_ix_[0usize];
        (*xself).last_histogram_ix_[0usize] = (*xself).last_histogram_ix_[1usize];
        (*xself).last_histogram_ix_[1usize] = __brotli_swap_tmp;
      }
      i = 0usize;
      while i < num_contexts {
        {
          histograms[((*xself).last_histogram_ix_[0usize].wrapping_add(i) as (usize))] =
            combined_histo.slice()[num_contexts.wrapping_add(i)].clone();
          (*xself).last_entropy_[(num_contexts.wrapping_add(i) as (usize))] =
            (*xself).last_entropy_[(i as (usize))];
          (*xself).last_entropy_[(i as (usize))] = combined_entropy[num_contexts.wrapping_add(i)];
          HistogramClear(&mut histograms[((*xself).curr_histogram_ix_.wrapping_add(i) as (usize))]);
        }
        i = i.wrapping_add(1 as (usize));
      }
      (*xself).num_blocks_ = (*xself).num_blocks_.wrapping_add(1 as (usize));
      (*xself).block_size_ = 0usize;
      (*xself).merge_last_count_ = 0usize;
      (*xself).target_block_size_ = (*xself).min_block_size_;
    } else {
      {
        let _rhs = (*xself).block_size_ as (u32);
        let _lhs = &mut (*split).lengths.slice_mut()[((*xself).num_blocks_.wrapping_sub(1usize) as
                         (usize))];
        let old_split_length = *_lhs;
        *_lhs = old_split_length.wrapping_add(_rhs);
      }
      i = 0usize;
      while i < num_contexts {
        {
          histograms[((*xself).last_histogram_ix_[0usize].wrapping_add(i) as (usize))] =
            combined_histo.slice()[i].clone();
          (*xself).last_entropy_[(i as (usize))] = combined_entropy[i];
          if (*split).num_types == 1usize {
            (*xself).last_entropy_[(num_contexts.wrapping_add(i) as (usize))] =
              (*xself).last_entropy_[(i as (usize))];
          }
          HistogramClear(&mut histograms[((*xself).curr_histogram_ix_.wrapping_add(i) as (usize))]);
        }
        i = i.wrapping_add(1 as (usize));
      }
      (*xself).block_size_ = 0usize;
      if {
           (*xself).merge_last_count_ = (*xself).merge_last_count_.wrapping_add(1 as (usize));
           (*xself).merge_last_count_
         } > 1usize {
        (*xself).target_block_size_ =
          (*xself).target_block_size_.wrapping_add((*xself).min_block_size_);
      }
    }
    m.free_cell(combined_histo);
  }
  if is_final != 0 {
    *histograms_size = (*split).num_types.wrapping_mul(num_contexts);
    (*split).num_blocks = (*xself).num_blocks_;
  }
}

fn BlockSplitterAddSymbol<HistogramType:SliceWrapper<u32>
                                          +SliceWrapperMut<u32>
                                          +CostAccessors
                                          +Clone,
                            AllocU8:alloc::Allocator<u8>,
                            AllocU32:alloc::Allocator<u32>>(xself: &mut BlockSplitter,
                                                            split: &mut BlockSplit<AllocU8,
                                                                                   AllocU32>,
                                                            histograms: &mut [HistogramType],
                                                            histograms_size: &mut usize,
                                                            symbol: usize) {
  HistogramAddItem(&mut histograms[((*xself).curr_histogram_ix_ as (usize))],
                   symbol);
  (*xself).block_size_ = (*xself).block_size_.wrapping_add(1 as (usize));
  if (*xself).block_size_ == (*xself).target_block_size_ {
    BlockSplitterFinishBlock(xself, split, histograms, histograms_size, 0i32);
  }
}

fn ContextBlockSplitterAddSymbol<AllocU8: alloc::Allocator<u8>, AllocU32: alloc::Allocator<u32>,
                            AllocHL:alloc::Allocator<HistogramLiteral>>
  (xself: &mut ContextBlockSplitter,
   m : &mut AllocHL,
   split: &mut BlockSplit<AllocU8, AllocU32>,
   histograms: &mut [HistogramLiteral],
   histograms_size: &mut usize,
   symbol: usize,
   context: usize) {
  HistogramAddItem(&mut histograms[((*xself).curr_histogram_ix_.wrapping_add(context) as (usize))],
                   symbol);
  (*xself).block_size_ = (*xself).block_size_.wrapping_add(1 as (usize));
  if (*xself).block_size_ == (*xself).target_block_size_ {
    ContextBlockSplitterFinishBlock(xself, m, split, histograms, histograms_size, 0i32);
  }
}

fn MapStaticContexts<AllocU8: alloc::Allocator<u8>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocHL: alloc::Allocator<HistogramLiteral>,
                     AllocHC: alloc::Allocator<HistogramCommand>,
                     AllocHD: alloc::Allocator<HistogramDistance>>
  (m32: &mut AllocU32,
   num_contexts: usize,
   static_context_map: &[u32],
   mb: &mut MetaBlockSplit<AllocU8, AllocU32, AllocHL, AllocHC, AllocHD>) {
  let mut i: usize;
  (*mb).literal_context_map_size = (*mb).literal_split.num_types << 6i32;
  let new_literal_context_map = m32.alloc_cell((*mb).literal_context_map_size);
  m32.free_cell(core::mem::replace(&mut (*mb).literal_context_map, new_literal_context_map));
  i = 0usize;
  while i < (*mb).literal_split.num_types {
    {
      let offset: u32 = i.wrapping_mul(num_contexts) as (u32);
      let mut j: usize;
      j = 0usize;
      while j < (1u32 << 6i32) as (usize) {
        {
          (*mb).literal_context_map.slice_mut()[((i << 6i32).wrapping_add(j) as (usize))] =
            offset.wrapping_add(static_context_map[(j as (usize))]);
        }
        j = j.wrapping_add(1 as (usize));
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
}
pub fn BrotliBuildMetaBlockGreedyInternal<AllocU8: alloc::Allocator<u8>,
                                          AllocU32: alloc::Allocator<u32>,
                                          AllocHL: alloc::Allocator<HistogramLiteral>,
                                          AllocHC: alloc::Allocator<HistogramCommand>,
                                          AllocHD: alloc::Allocator<HistogramDistance>>
  (m8:  &mut AllocU8,
   m32: &mut AllocU32,
   mhl: &mut AllocHL,
   mhc: &mut AllocHC,
   mhd: &mut AllocHD,
   ringbuffer: &[u8],
   mut pos: usize,
   mask: usize,
   mut prev_byte: u8,
   mut prev_byte2: u8,
   literal_context_mode: ContextType,
   num_contexts: usize,
   static_context_map: &[u32],
   commands: &[Command],
   n_commands: usize,
   mb: &mut MetaBlockSplit<AllocU8, AllocU32, AllocHL, AllocHC, AllocHD>) {
  let mut lit_blocks: LitBlocks;
  let mut cmd_blocks: BlockSplitter;
  let mut dist_blocks: BlockSplitter;
  let mut num_literals: usize = 0usize;
  let mut i: usize;
  i = 0usize;
  while i < n_commands {
    {
      num_literals = num_literals.wrapping_add((commands[(i as (usize))]).insert_len_ as (usize));
    }
    i = i.wrapping_add(1 as (usize));
  }
  lit_blocks = if num_contexts == 1usize {
    LitBlocks::plain(InitBlockSplitter(m8,
                                       m32,
                                       mhl,
                                       256usize,
                                       512usize,
                                       400.0 as super::util::floatX,
                                       num_literals,
                                       &mut (*mb).literal_split,
                                       &mut (*mb).literal_histograms,
                                       &mut (*mb).literal_histograms_size))
  } else {
    LitBlocks::ctx(InitContextBlockSplitter(m8,
                                            m32,
                                            mhl,
                                            256usize,
                                            num_contexts,
                                            512usize,
                                            400.0 as super::util::floatX,
                                            num_literals,
                                            &mut (*mb).literal_split,
                                            &mut (*mb).literal_histograms,
                                            &mut (*mb).literal_histograms_size))
  };
  cmd_blocks = InitBlockSplitter(m8,
                                 m32,
                                 mhc,
                                 704usize,
                                 1024usize,
                                 500.0 as super::util::floatX,
                                 n_commands,
                                 &mut (*mb).command_split,
                                 &mut (*mb).command_histograms,
                                 &mut (*mb).command_histograms_size);
  dist_blocks = InitBlockSplitter(m8,
                                  m32,
                                  mhd,
                                  64usize,
                                  512usize,
                                  100.0 as super::util::floatX,
                                  n_commands,
                                  &mut (*mb).distance_split,
                                  &mut (*mb).distance_histograms,
                                  &mut (*mb).distance_histograms_size);

  i = 0usize;
  while i < n_commands {
    {
      let cmd: Command = commands[(i as (usize))].clone();
      let mut j: usize;
      BlockSplitterAddSymbol(&mut cmd_blocks,
                             &mut (*mb).command_split,
                             &mut (*mb).command_histograms.slice_mut(),
                             &mut (*mb).command_histograms_size,
                             cmd.cmd_prefix_ as (usize));
      j = cmd.insert_len_ as (usize);
      while j != 0usize {
        {
          let literal: u8 = ringbuffer[((pos & mask) as (usize))];
          match (&mut lit_blocks) {
            &mut LitBlocks::plain(ref mut lit_blocks_plain) => {
              BlockSplitterAddSymbol(lit_blocks_plain,
                                     &mut (*mb).literal_split,
                                     &mut (*mb).literal_histograms.slice_mut(),
                                     &mut (*mb).literal_histograms_size,
                                     literal as (usize))
            }
            &mut LitBlocks::ctx(ref mut lit_blocks_ctx) => {
              let context: usize = Context(prev_byte, prev_byte2, literal_context_mode) as (usize);
              ContextBlockSplitterAddSymbol(lit_blocks_ctx,
                                            mhl,
                                            &mut (*mb).literal_split,
                                            &mut (*mb).literal_histograms.slice_mut(),
                                            &mut (*mb).literal_histograms_size,
                                            literal as (usize),
                                            static_context_map[(context as (usize))] as (usize));
            }
          }
          prev_byte2 = prev_byte;
          prev_byte = literal;
          pos = pos.wrapping_add(1 as (usize));
        }
        j = j.wrapping_sub(1 as (usize));
      }
      pos = pos.wrapping_add(CommandCopyLen(&cmd) as (usize));
      if CommandCopyLen(&cmd) != 0 {
        prev_byte2 = ringbuffer[((pos.wrapping_sub(2usize) & mask) as (usize))];
        prev_byte = ringbuffer[((pos.wrapping_sub(1usize) & mask) as (usize))];
        if cmd.cmd_prefix_ as (i32) >= 128i32 {
          BlockSplitterAddSymbol(&mut dist_blocks,
                                 &mut (*mb).distance_split,
                                 &mut (*mb).distance_histograms.slice_mut(),
                                 &mut (*mb).distance_histograms_size,
                                 cmd.dist_prefix_ as (usize));
        }
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  match (&mut lit_blocks) {
    &mut LitBlocks::plain(ref mut lit_blocks_plain) => {
      BlockSplitterFinishBlock(lit_blocks_plain,
                               &mut (*mb).literal_split,
                               &mut (*mb).literal_histograms.slice_mut(),
                               &mut (*mb).literal_histograms_size,
                               1i32)
    }
    &mut LitBlocks::ctx(ref mut lit_blocks_ctx) => {
      ContextBlockSplitterFinishBlock(lit_blocks_ctx,
                                      mhl,
                                      &mut (*mb).literal_split,
                                      &mut (*mb).literal_histograms.slice_mut(),
                                      &mut (*mb).literal_histograms_size,
                                      1i32)
    }
  }
  BlockSplitterFinishBlock(&mut cmd_blocks,
                           &mut (*mb).command_split,
                           &mut (*mb).command_histograms.slice_mut(),
                           &mut (*mb).command_histograms_size,
                           1i32);
  BlockSplitterFinishBlock(&mut dist_blocks,
                           &mut (*mb).distance_split,
                           &mut (*mb).distance_histograms.slice_mut(),
                           &mut (*mb).distance_histograms_size,
                           1i32);
  if num_contexts > 1usize {
    MapStaticContexts(m32, num_contexts, static_context_map, mb);
  }
}
pub fn BrotliBuildMetaBlockGreedy<AllocU8: alloc::Allocator<u8>,
                                  AllocU32: alloc::Allocator<u32>,
                                  AllocHL: alloc::Allocator<HistogramLiteral>,
                                  AllocHC: alloc::Allocator<HistogramCommand>,
                                  AllocHD: alloc::Allocator<HistogramDistance>>
  (m8:  &mut AllocU8,
   m32: &mut AllocU32,
   mhl: &mut AllocHL,
   mhc: &mut AllocHC,
   mhd: &mut AllocHD,
   ringbuffer: &[u8],
   pos: usize,
   mask: usize,
   prev_byte: u8,
   prev_byte2: u8,
   literal_context_mode: ContextType,
   num_contexts: usize,
   static_context_map: &[u32],
   commands: &[Command],
   n_commands: usize,
   mb: &mut MetaBlockSplit<AllocU8, AllocU32, AllocHL, AllocHC, AllocHD>) {
  if num_contexts == 1usize {
    BrotliBuildMetaBlockGreedyInternal(m8,
                                       m32,
                                       mhl,
                                       mhc,
                                       mhd,
                                       ringbuffer,
                                       pos,
                                       mask,
                                       prev_byte,
                                       prev_byte2,
                                       literal_context_mode,
                                       1usize,
                                       &[],
                                       commands,
                                       n_commands,
                                       mb);
  } else {
    BrotliBuildMetaBlockGreedyInternal(m8,
                                       m32,
                                       mhl,
                                       mhc,
                                       mhd,
                                       ringbuffer,
                                       pos,
                                       mask,
                                       prev_byte,
                                       prev_byte2,
                                       literal_context_mode,
                                       num_contexts,
                                       static_context_map,
                                       commands,
                                       n_commands,
                                       mb);
  }
}


pub fn BrotliOptimizeHistograms<AllocU8: alloc::Allocator<u8>,
                                AllocU32: alloc::Allocator<u32>,
                                AllocHL: alloc::Allocator<HistogramLiteral>,
                                AllocHC: alloc::Allocator<HistogramCommand>,
                                AllocHD: alloc::Allocator<HistogramDistance>>
  (num_direct_distance_codes: usize,
   distance_postfix_bits: usize,
   mb: &mut MetaBlockSplit<AllocU8, AllocU32, AllocHL, AllocHC, AllocHD>) {
  let mut good_for_rle: [u8; 704] = [0; 704];
  let num_distance_codes: usize;
  let mut i: usize;
  i = 0usize;
  while i < (*mb).literal_histograms_size {
    {
      BrotliOptimizeHuffmanCountsForRle(256usize,
                                        (*mb).literal_histograms.slice_mut()[(i as (usize))]
                                          .slice_mut(),
                                        &mut good_for_rle[..]);
    }
    i = i.wrapping_add(1 as (usize));
  }
  i = 0usize;
  while i < (*mb).command_histograms_size {
    {
      BrotliOptimizeHuffmanCountsForRle(704usize,
                                        (*mb).command_histograms.slice_mut()[(i as (usize))]
                                          .slice_mut(),
                                        &mut good_for_rle[..]);
    }
    i = i.wrapping_add(1 as (usize));
  }
  num_distance_codes =
    (16usize).wrapping_add(num_direct_distance_codes).wrapping_add(((2u32).wrapping_mul(24u32) <<
                                                                    distance_postfix_bits) as
                                                                   (usize));
  i = 0usize;
  while i < (*mb).distance_histograms_size {
    {
      BrotliOptimizeHuffmanCountsForRle(num_distance_codes,
                                        (*mb).distance_histograms.slice_mut()[(i as (usize))]
                                          .slice_mut(),
                                        &mut good_for_rle[..]);
    }
    i = i.wrapping_add(1 as (usize));
  }
}
