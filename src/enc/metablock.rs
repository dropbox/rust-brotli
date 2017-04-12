#![allow(dead_code)]
use super::backward_references::{BrotliHasherParams, BrotliEncoderParams, BrotliEncoderMode};
use super::bit_cost::{ShannonEntropy, BitsEntropy};
use super::block_split::BlockSplit;
use super::block_splitter::BrotliSplitBlock;
use super::brotli_bit_stream::MetaBlockSplit;
use super::cluster::{BrotliHistogramBitCostDistance, BrotliHistogramCombine, HistogramPair};
use super::cluster::BrotliClusterHistograms;
use super::command::{Command, CommandCopyLen};
use super::entropy_encode::BrotliOptimizeHuffmanCountsForRle;
use super::histogram::{BrotliBuildHistogramsWithContext, CostAccessors, HistogramLiteral,
                       HistogramCommand, HistogramDistance, HistogramClear, ClearHistograms,
                       ContextType, HistogramAddHistogram, HistogramAddItem, Context};
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::util::{FastLog2, brotli_max_uint8_t, brotli_min_size_t, brotli_max_size_t};
use core;

pub fn BrotliBuildMetaBlock<AllocU8: alloc::Allocator<u8>,
                            AllocU16: alloc::Allocator<u16>,
                            AllocU32: alloc::Allocator<u32>,
                            AllocF64: alloc::Allocator<f64>,
                            AllocHL: alloc::Allocator<HistogramLiteral>,
                            AllocHC: alloc::Allocator<HistogramCommand>,
                            AllocHD: alloc::Allocator<HistogramDistance>,
                            AllocHP: alloc::Allocator<HistogramPair>,
                            AllocCT: alloc::Allocator<ContextType>>
  (mut m8: &mut AllocU8,
   mut m16: &mut AllocU16,
   mut m32: &mut AllocU32,
   mut mf64: &mut AllocF64,
   mut mhl: &mut AllocHL,
   mut mhc: &mut AllocHC,
   mhd: &mut AllocHD,
   mut mhp: &mut AllocHP,
   mut mct: &mut AllocCT,
   ringbuffer: &[u8],
   pos: usize,
   mask: usize,
   params: &BrotliEncoderParams,
   prev_byte: u8,
   prev_byte2: u8,
   cmds: &[Command],
   num_commands: usize,
   literal_context_mode: ContextType,
   mut mb: &mut MetaBlockSplit<AllocU8, AllocU32, AllocHL, AllocHC, AllocHD>) {

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
  ClearHistograms(literal_histograms.slice_mut(), literal_histograms_size);
  distance_histograms_size = (*mb).distance_split.num_types << 2i32;
  distance_histograms = mhd.alloc_cell(distance_histograms_size);
  ClearHistograms(distance_histograms.slice_mut(), distance_histograms_size);
  (*mb).command_histograms_size = (*mb).command_split.num_types;
  (*mb).command_histograms = mhc.alloc_cell((*mb).command_histograms_size);
  ClearHistograms((*mb).command_histograms.slice_mut(),
                  (*mb).command_histograms_size);
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
  pub split_threshold_: f64,
  pub num_blocks_: usize,
  //  pub split_: &'a mut BlockSplit<AllocU8, AllocU32>,
  //  pub histograms_: AllocHT::AllocatedMemory, // FIXME: pull this one out at the end
  //  pub histograms_size_: &'a mut usize, // FIXME: pull this one out at the end
  pub target_block_size_: usize,
  pub block_size_: usize,
  pub curr_histogram_ix_: usize,
  pub last_histogram_ix_: [usize; 2],
  pub last_entropy_: [f64; 2],
  pub merge_last_count_: usize,
}



pub struct ContextBlockSplitter {
  pub alphabet_size_: usize,
  pub num_contexts_: usize,
  pub max_block_types_: usize,
  pub min_block_size_: usize,
  pub split_threshold_: f64,
  pub num_blocks_: usize,
  //  pub split_: &'a mut BlockSplit<AllocU8, AllocU32>,
  //  pub histograms_: AllocHL::AllocatedMemory,
  //  pub histograms_size_: &'a mut usize, // FIXME: pull this one out at the end
  pub target_block_size_: usize,
  pub block_size_: usize,
  pub curr_histogram_ix_: usize,
  pub last_histogram_ix_: [usize; 2],
  pub last_entropy_: [f64; 6],
  pub merge_last_count_: usize,
}



pub struct LitBlocks {
  pub plain: BlockSplitter, //<'a, HistogramLiteral, AllocU8, AllocU32, AllocHL>,
  pub ctx: ContextBlockSplitter, //<'a, AllocU8, AllocU32, AllocHL>,
}

/*

pub struct BlockSplitterCommand {
  pub alphabet_size_: usize,
  pub min_block_size_: usize,
  pub split_threshold_: f64,
  pub num_blocks_: usize,
  pub split_: *mut BlockSplit,
  pub histograms_: *mut HistogramCommand,
  pub histograms_size_: *mut usize,
  pub target_block_size_: usize,
  pub block_size_: usize,
  pub curr_histogram_ix_: usize,
  pub last_histogram_ix_: [usize; 2],
  pub last_entropy_: [f64; 2],
  pub merge_last_count_: usize,
}



pub struct BlockSplitterDistance {
  pub alphabet_size_: usize,
  pub min_block_size_: usize,
  pub split_threshold_: f64,
  pub num_blocks_: usize,
  pub split_: *mut BlockSplit,
  pub histograms_: *mut HistogramDistance,
  pub histograms_size_: *mut usize,
  pub target_block_size_: usize,
  pub block_size_: usize,
  pub curr_histogram_ix_: usize,
  pub last_histogram_ix_: [usize; 2],
  pub last_entropy_: [f64; 2],
  pub merge_last_count_: usize,
}
*/

fn InitBlockSplitter<HistogramType: SliceWrapper<u32> + SliceWrapperMut<u32> + CostAccessors,
                     AllocU8: alloc::Allocator<u8>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocHT: alloc::Allocator<HistogramType>>
  (mut m8: &mut AllocU8,
   mut m32: &mut AllocU32,
   mut mht: &mut AllocHT,
   mut xself: &mut BlockSplitter, //<HistogramType, AllocU8, AllocU32, AllocHT>,
   alphabet_size: usize,
   min_block_size: usize,
   split_threshold: f64,
   num_symbols: usize,
   mut split: &mut BlockSplit<AllocU8, AllocU32>,
   mut histograms: &mut AllocHT::AllocatedMemory,
   mut histograms_size: &mut usize) {
  let max_num_blocks: usize = num_symbols.wrapping_div(min_block_size).wrapping_add(1usize);
  let max_num_types: usize = brotli_min_size_t(max_num_blocks, (256i32 + 1i32) as (usize));
  (*xself).alphabet_size_ = alphabet_size;
  (*xself).min_block_size_ = min_block_size;
  (*xself).split_threshold_ = split_threshold;
  (*xself).num_blocks_ = 0usize;
  //(*xself).split_ = split;
  //(*xself).histograms_size_ = histograms_size;
  (*xself).target_block_size_ = min_block_size;
  (*xself).block_size_ = 0usize;
  (*xself).curr_histogram_ix_ = 0usize;
  (*xself).merge_last_count_ = 0usize;
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
  (*xself).last_histogram_ix_[0] = 0;
  (*xself).last_histogram_ix_[1] = 0;
}
fn InitContextBlockSplitter<AllocU8: alloc::Allocator<u8>,
                            AllocU32: alloc::Allocator<u32>,
                            AllocHL: alloc::Allocator<HistogramLiteral>>
  (mut m8: &mut AllocU8,
   mut m32: &mut AllocU32,
   mut mhl: &mut AllocHL,
   mut xself: &mut ContextBlockSplitter,
   alphabet_size: usize,
   num_contexts: usize,
   min_block_size: usize,
   split_threshold: f64,
   num_symbols: usize,
   mut split: &mut BlockSplit<AllocU8, AllocU32>,
   mut histograms: &mut AllocHL::AllocatedMemory,
   mut histograms_size: &mut usize) {
  let max_num_blocks: usize = num_symbols.wrapping_div(min_block_size).wrapping_add(1usize);
  let max_num_types: usize;
  (*xself).alphabet_size_ = alphabet_size;
  (*xself).num_contexts_ = num_contexts;
  (*xself).max_block_types_ = (256usize).wrapping_div(num_contexts);
  (*xself).min_block_size_ = min_block_size;
  (*xself).split_threshold_ = split_threshold;
  (*xself).num_blocks_ = 0usize;
  //(*xself).histograms_size_ = histograms_size;
  (*xself).target_block_size_ = min_block_size;
  (*xself).block_size_ = 0usize;
  (*xself).curr_histogram_ix_ = 0usize;
  (*xself).merge_last_count_ = 0usize;
  max_num_types = brotli_min_size_t(max_num_blocks,
                                    (*xself).max_block_types_.wrapping_add(1usize));
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
  (*xself).last_histogram_ix_[0] = 0;
  (*xself).last_histogram_ix_[1] = 0;
}

fn BlockSplitterFinishBlock<HistogramType:SliceWrapper<u32>
                                          +SliceWrapperMut<u32>
                                          +CostAccessors
                                          +Clone,
                            AllocU8:alloc::Allocator<u8>,
                            AllocU32:alloc::Allocator<u32>>(mut xself: &mut BlockSplitter,
                                                                     mut split: &mut BlockSplit<AllocU8, AllocU32>,
                                                                     mut histograms: &mut [HistogramType],
                                                                     mut histograms_size: &mut usize,
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
    let entropy: f64 = BitsEntropy((histograms[((*xself).curr_histogram_ix_ as (usize))]).slice(),
                                   (*xself).alphabet_size_);
    let mut combined_histo: [HistogramType; 2] = [histograms[(*xself).curr_histogram_ix_].clone(),
                                                  histograms[(*xself).curr_histogram_ix_].clone()];

    let mut combined_entropy: [f64; 2] = [0.0f64, 0.0f64];
    let mut diff: [f64; 2] = [0.0f64, 0.0f64];
    let mut j: usize;
    j = 0usize;
    while j < 2usize {
      {
        let last_histogram_ix: usize = (*xself).last_histogram_ix_[j];
        //combined_histo[j] = histograms[((*xself).curr_histogram_ix_ as (usize))].clone();
        HistogramAddHistogram(&mut combined_histo[j],
                              &histograms[(last_histogram_ix as (usize))]);
        combined_entropy[j] = BitsEntropy(&mut combined_histo[j].slice_mut()[0usize..],
                                          (*xself).alphabet_size_);
        diff[j] = combined_entropy[j] - entropy - (*xself).last_entropy_[(j as (usize))];
      }
      j = j.wrapping_add(1 as (usize));
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
    } else if diff[1usize] < diff[0usize] - 20.0f64 {
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
/*
fn BlockSplitterAddSymbolCommand<AllocU8:alloc::Allocator<u8>,
                            AllocU32:alloc::Allocator<u32> >(mut xself: &mut BlockSplitter,
                                                                        mut split: &mut BlockSplit<AllocU8, AllocU32>,
                                                                        mut histograms: &mut [HistogramCommand],
                                                                        mut histograms_size: &mut usize,
                                                                        mut symbol: usize) {
  HistogramAddItem(&mut histograms[((*xself).curr_histogram_ix_ as (usize))],
                      symbol);
  (*xself).block_size_ = (*xself).block_size_.wrapping_add(1 as (usize));
  if (*xself).block_size_ == (*xself).target_block_size_ {
    BlockSplitterFinishBlock(xself, split, histograms, histograms_size, 0i32);
  }
}

*/

fn ContextBlockSplitterFinishBlock<AllocU8: alloc::Allocator<u8>, AllocU32: alloc::Allocator<u32>>
  (mut xself: &mut ContextBlockSplitter,
   mut split: &mut BlockSplit<AllocU8, AllocU32>,
   mut histograms: &mut [HistogramLiteral],
   mut histograms_size: &mut usize,
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
    let mut entropy: [f64; 3] = [0.0f64; 3];
    let mut combined_histo: [HistogramLiteral; 6] = [HistogramLiteral::default(),
                                                     HistogramLiteral::default(),
                                                     HistogramLiteral::default(),
                                                     HistogramLiteral::default(),
                                                     HistogramLiteral::default(),
                                                     HistogramLiteral::default()]; //FIXME; slow
    let mut combined_entropy: [f64; 6] = [0.0f64; 6];
    let mut diff: [f64; 2] = [0.0f64, 0.0f64];
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
            combined_histo[jx] = histograms[(curr_histo_ix as (usize))].clone();
            HistogramAddHistogram(&mut combined_histo[jx],
                                  &mut histograms[(last_histogram_ix as (usize))]);
            combined_entropy[jx] = BitsEntropy(combined_histo[jx].slice(), (*xself).alphabet_size_);
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
    } else if diff[1usize] < diff[0usize] - 20.0f64 {
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
            combined_histo[num_contexts.wrapping_add(i)].clone();
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
            combined_histo[i].clone();
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
                            AllocU32:alloc::Allocator<u32> >(mut xself: &mut BlockSplitter,
                                                                        mut split: &mut BlockSplit<AllocU8, AllocU32>,
                                                                        mut histograms: &mut [HistogramType],
                                                                        mut histograms_size: &mut usize,
symbol: usize){
  HistogramAddItem(&mut histograms[((*xself).curr_histogram_ix_ as (usize))],
                   symbol);
  (*xself).block_size_ = (*xself).block_size_.wrapping_add(1 as (usize));
  if (*xself).block_size_ == (*xself).target_block_size_ {
    BlockSplitterFinishBlock(xself, split, histograms, histograms_size, 0i32);
  }
}

fn ContextBlockSplitterAddSymbol<AllocU8: alloc::Allocator<u8>, AllocU32: alloc::Allocator<u32>>
  (mut xself: &mut ContextBlockSplitter,
   mut split: &mut BlockSplit<AllocU8, AllocU32>,
   mut histograms: &mut [HistogramLiteral],
   mut histograms_size: &mut usize,
   symbol: usize,
   context: usize) {
  HistogramAddItem(&mut histograms[((*xself).curr_histogram_ix_.wrapping_add(context) as (usize))],
                   symbol);
  (*xself).block_size_ = (*xself).block_size_.wrapping_add(1 as (usize));
  if (*xself).block_size_ == (*xself).target_block_size_ {
    ContextBlockSplitterFinishBlock(xself, split, histograms, histograms_size, 0i32);
  }
}

fn MapStaticContexts<AllocU8: alloc::Allocator<u8>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocHL: alloc::Allocator<HistogramLiteral>,
                     AllocHC: alloc::Allocator<HistogramCommand>,
                     AllocHD: alloc::Allocator<HistogramDistance>>
  (mut m32: &mut AllocU32,
   num_contexts: usize,
   static_context_map: &[u32],
   mut mb: &mut MetaBlockSplit<AllocU8, AllocU32, AllocHL, AllocHC, AllocHD>) {
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
/*
fn BrotliBuildMetaBlockGreedyInternal(mut m: &mut [MemoryManager],
                                      mut ringbuffer: &[u8],
                                      mut pos: usize,
                                      mut mask: usize,
                                      mut prev_byte: u8,
                                      mut prev_byte2: u8,
                                      mut literal_context_mode: ContextType,
                                      num_contexts: usize,
                                      mut static_context_map: &[u32],
                                      mut commands: &[Command],
                                      mut n_commands: usize,
                                      mut mb: &mut [MetaBlockSplit]) {
  let mut lit_blocks: LitBlocks;
  let mut cmd_blocks: BlockSplitterCommand;
  let mut dist_blocks: BlockSplitterDistance;
  let mut num_literals: usize = 0usize;
  let mut i: usize;
  i = 0usize;
  while i < n_commands {
    {
      num_literals = num_literals.wrapping_add((commands[(i as (usize))]).insert_len_ as (usize));
    }
    i = i.wrapping_add(1 as (usize));
  }
  if num_contexts == 1usize {
    InitBlockSplitterLiteral(m,
                             &mut lit_blocks.plain,
                             256usize,
                             512usize,
                             400.0f64,
                             num_literals,
                             &mut (*mb).literal_split,
                             &mut (*mb).literal_histograms,
                             &mut (*mb).literal_histograms_size);
  } else {
    InitContextBlockSplitter(m,
                             &mut lit_blocks.ctx,
                             256usize,
                             num_contexts,
                             512usize,
                             400.0f64,
                             num_literals,
                             &mut (*mb).literal_split,
                             &mut (*mb).literal_histograms,
                             &mut (*mb).literal_histograms_size);
  }
  if !(0i32 == 0) {
    return;
  }
  InitBlockSplitterCommand(m,
                           &mut cmd_blocks,
                           704usize,
                           1024usize,
                           500.0f64,
                           n_commands,
                           &mut (*mb).command_split,
                           &mut (*mb).command_histograms,
                           &mut (*mb).command_histograms_size);
  if !(0i32 == 0) {
    return;
  }
  InitBlockSplitterDistance(m,
                            &mut dist_blocks,
                            64usize,
                            512usize,
                            100.0f64,
                            n_commands,
                            &mut (*mb).distance_split,
                            &mut (*mb).distance_histograms,
                            &mut (*mb).distance_histograms_size);
  if !(0i32 == 0) {
    return;
  }
  i = 0usize;
  while i < n_commands {
    {
      let cmd: Command = commands[(i as (usize))];
      let mut j: usize;
      BlockSplitterAddSymbolCommand(&mut cmd_blocks, cmd.cmd_prefix_ as (usize));
      j = cmd.insert_len_ as (usize);
      while j != 0usize {
        {
          let mut literal: u8 = ringbuffer[((pos & mask) as (usize))];
          if num_contexts == 1usize {
            BlockSplitterAddSymbolLiteral(&mut lit_blocks.plain, literal as (usize));
          } else {
            let mut context: usize = Context(prev_byte, prev_byte2, literal_context_mode) as
                                     (usize);
            ContextBlockSplitterAddSymbol(&mut lit_blocks.ctx,
                                          literal as (usize),
                                          static_context_map[(context as (usize))] as (usize));
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
          BlockSplitterAddSymbolDistance(&mut dist_blocks, cmd.dist_prefix_ as (usize));
        }
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  if num_contexts == 1usize {
    BlockSplitterFinishBlockLiteral(&mut lit_blocks.plain, 1i32);
  } else {
    ContextBlockSplitterFinishBlock(&mut lit_blocks.ctx, 1i32);
  }
  BlockSplitterFinishBlockCommand(&mut cmd_blocks, 1i32);
  BlockSplitterFinishBlockDistance(&mut dist_blocks, 1i32);
  if num_contexts > 1usize {
    MapStaticContexts(m, num_contexts, static_context_map, mb);
  }
}


pub fn BrotliBuildMetaBlockGreedy(mut m: &mut [MemoryManager],
                                  mut ringbuffer: &[u8],
                                  mut pos: usize,
                                  mut mask: usize,
                                  mut prev_byte: u8,
                                  mut prev_byte2: u8,
                                  mut literal_context_mode: ContextType,
                                  mut num_contexts: usize,
                                  mut static_context_map: &[u32],
                                  mut commands: &[Command],
                                  mut n_commands: usize,
                                  mut mb: &mut [MetaBlockSplit]) {
  if num_contexts == 1usize {
    BrotliBuildMetaBlockGreedyInternal(m,
                                       ringbuffer,
                                       pos,
                                       mask,
                                       prev_byte,
                                       prev_byte2,
                                       literal_context_mode,
                                       1usize,
                                       0i32,
                                       commands,
                                       n_commands,
                                       mb);
  } else {
    BrotliBuildMetaBlockGreedyInternal(m,
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


pub fn BrotliOptimizeHistograms(mut num_direct_distance_codes: usize,
                                mut distance_postfix_bits: usize,
                                mut mb: &mut [MetaBlockSplit]) {
  let mut good_for_rle: [u8; 704];
  let mut num_distance_codes: usize;
  let mut i: usize;
  i = 0usize;
  while i < (*mb).literal_histograms_size {
    {
      BrotliOptimizeHuffmanCountsForRle(256usize,
                                        (*(*mb).literal_histograms[(i as (usize))..])
                                          .data_
                                          .as_mut_ptr(),
                                        good_for_rle.as_mut_ptr());
    }
    i = i.wrapping_add(1 as (usize));
  }
  i = 0usize;
  while i < (*mb).command_histograms_size {
    {
      BrotliOptimizeHuffmanCountsForRle(704usize,
                                        (*(*mb).command_histograms[(i as (usize))..])
                                          .data_
                                          .as_mut_ptr(),
                                        good_for_rle.as_mut_ptr());
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
                                        (*(*mb).distance_histograms[(i as (usize))..])
                                          .data_
                                          .as_mut_ptr(),
                                        good_for_rle.as_mut_ptr());
    }
    i = i.wrapping_add(1 as (usize));
  }
}
*/
