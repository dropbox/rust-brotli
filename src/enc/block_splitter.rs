#![allow(dead_code)]
use super::vectorization::{v128,v128i,v256,v256i, Mem256f};
use super::backward_references::BrotliEncoderParams;

use super::bit_cost::BrotliPopulationCost;
use super::block_split::BlockSplit;
use super::cluster::{BrotliHistogramBitCostDistance, BrotliHistogramCombine, HistogramPair};
use super::command::Command;
use super::histogram::{HistogramAddVector, CostAccessors, ClearHistograms, HistogramClear,
                       HistogramAddHistogram, HistogramAddItem, HistogramLiteral, HistogramCommand,
                       HistogramDistance};
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::util::{FastLog2, brotli_max_uint8_t, brotli_min_size_t};
use core;
static kMaxLiteralHistograms: usize = 100usize;

static kMaxCommandHistograms: usize = 50usize;

static kLiteralBlockSwitchCost: super::util::floatX = 28.1 as super::util::floatX;

static kCommandBlockSwitchCost: super::util::floatX = 13.5 as super::util::floatX;

static kDistanceBlockSwitchCost: super::util::floatX = 14.6 as super::util::floatX;

static kLiteralStrideLength: usize = 70usize;

static kCommandStrideLength: usize = 40usize;

static kSymbolsPerLiteralHistogram: usize = 544usize;

static kSymbolsPerCommandHistogram: usize = 530usize;

static kSymbolsPerDistanceHistogram: usize = 544usize;

static kMinLengthForBlockSplitting: usize = 128usize;

static kIterMulForRefining: usize = 2usize;

static kMinItersForRefining: usize = 100usize;

fn update_cost_and_signal(num_histograms32: u32,
                          ix: usize,
                          min_cost: super::util::floatX,
                          block_switch_cost: super::util::floatX,
                          mut cost: &mut [Mem256f],
                          mut switch_signal: &mut [u8]) {
    let ymm_min_cost = bcast256!(min_cost);
    let round_num_histograms = (((num_histograms32 as usize) + 7) >> 3) << 3;
    let ymm_block_switch_cost = bcast256!(block_switch_cost);
    let ymm_and_mask = v256i{hi:v128i{x3:1<<7,
                                      x2:1<<6,
                                      x1:1<<5,
                                      x0:1<<4},
                             lo:v128i{x3:1<<3,
                                      x2:1<<2,
                                      x1:1<<1,
                                      x0:1<<0}};
    
    for (index, cost_it) in cost.iter_mut().enumerate() {
        let mut ymm_cost = v256::new(cost_it);
        let costk_minus_min_cost = sub256!(ymm_cost, ymm_min_cost);
        let ymm_cmpge = cmpge256!(costk_minus_min_cost, ymm_block_switch_cost);
        let ymm_bits = and256i!(ymm_cmpge, ymm_and_mask);
        let result = super::vectorization::sum8(ymm_bits) as u8;
        switch_signal[ix + index] |= result;
        ymm_cost = min256!(costk_minus_min_cost, ymm_block_switch_cost);
        *cost_it = Mem256f::new(ymm_cost);
    }
}
fn CountLiterals(cmds: &[Command], num_commands: usize) -> usize {
  let mut total_length: usize = 0usize;
  let mut i: usize;
  i = 0usize;
  while i < num_commands {
    {
      total_length = total_length.wrapping_add((cmds[(i as (usize))]).insert_len_ as (usize));
    }
    i = i.wrapping_add(1 as (usize));
  }
  total_length
}

fn CommandCopyLen(xself: &Command) -> u32 {
  (*xself).copy_len_ & 0xffffffu32
}

fn CopyLiteralsToByteArray(cmds: &[Command],
                           num_commands: usize,
                           data: &[u8],
                           offset: usize,
                           mask: usize,
                           mut literals: &mut [u8]) {
  let mut pos: usize = 0usize;
  let mut from_pos: usize = offset & mask;
  let mut i: usize;
  i = 0usize;
  while i < num_commands {
    {
      let mut insert_len: usize = (cmds[(i as (usize))]).insert_len_ as (usize);
      if from_pos.wrapping_add(insert_len) > mask {
        let head_size: usize = mask.wrapping_add(1usize).wrapping_sub(from_pos);
        literals[(pos as (usize))..((pos + head_size) as usize)].clone_from_slice(&data[(from_pos as
                                                                                    (usize))..
                                                                                   ((from_pos +
                                                                                     head_size) as
                                                                                    usize)]);
        from_pos = 0usize;
        pos = pos.wrapping_add(head_size);
        insert_len = insert_len.wrapping_sub(head_size);
      }
      if insert_len > 0usize {
        literals[(pos as (usize))..(pos as usize + insert_len)].clone_from_slice(&data
                                                                                    [(from_pos as
                                                                                   (usize))..
                                                                                  (from_pos as
                                                                                   usize +
                                                                                   insert_len)]);
        pos = pos.wrapping_add(insert_len);
      }
      from_pos = from_pos.wrapping_add(insert_len).wrapping_add(CommandCopyLen(&cmds[(i as
                                                                                 (usize))]) as
                                                                (usize)) & mask;
    }
    i = i.wrapping_add(1 as (usize));
  }
}

fn MyRand(mut seed: &mut u32) -> u32 {
  *seed = (*seed).wrapping_mul(16807u32);
  if *seed == 0u32 {
    *seed = 1u32;
  }
  *seed
}



fn InitialEntropyCodes<HistogramType: SliceWrapper<u32> + SliceWrapperMut<u32> + CostAccessors,
                       IntegerType: Sized + Clone>
  (data: &[IntegerType],
   length: usize,
   stride: usize,
   num_histograms: usize,
   mut histograms: &mut [HistogramType])
  where u64: core::convert::From<IntegerType>
{
  let mut seed: u32 = 7u32;
  let block_length: usize = length.wrapping_div(num_histograms);
  let mut i: usize;
  ClearHistograms(histograms, num_histograms);
  i = 0usize;
  while i < num_histograms {
    {
      let mut pos: usize = length.wrapping_mul(i).wrapping_div(num_histograms);
      if i != 0usize {
        pos = pos.wrapping_add((MyRand(&mut seed) as (usize)).wrapping_rem(block_length));
      }
      if pos.wrapping_add(stride) >= length {
        pos = length.wrapping_sub(stride).wrapping_sub(1usize);
      }
      HistogramAddVector(&mut histograms[(i as (usize))],
                         &data[(pos as (usize))..],
                         stride);
    }
    i = i.wrapping_add(1 as (usize));
  }
}

fn RandomSample<HistogramType: SliceWrapper<u32> + SliceWrapperMut<u32> + CostAccessors,
                IntegerType: Sized + Clone>
  (mut seed: &mut u32,
   data: &[IntegerType],
   length: usize,
   mut stride: usize,
   mut sample: &mut HistogramType)
  where u64: core::convert::From<IntegerType>
{
  let pos: usize;
  if stride >= length {
    pos = 0usize;
    stride = length;
  } else {
    pos = (MyRand(seed) as (usize)).wrapping_rem(length.wrapping_sub(stride).wrapping_add(1usize));
  }
  HistogramAddVector(sample, &data[(pos as (usize))..], stride);
}

fn RefineEntropyCodes<HistogramType:SliceWrapper<u32>+SliceWrapperMut<u32>+CostAccessors+core::default::Default, IntegerType:Sized+Clone>(data: &[IntegerType],
                             length: usize,
                             stride: usize,
                             num_histograms: usize,
mut histograms: &mut [HistogramType]) where u64: core::convert::From<IntegerType>{
  let mut iters: usize = kIterMulForRefining.wrapping_mul(length)
    .wrapping_div(stride)
    .wrapping_add(kMinItersForRefining);
  let mut seed: u32 = 7u32;
  let mut iter: usize;
  iters = iters.wrapping_add(num_histograms)
    .wrapping_sub(1usize)
    .wrapping_div(num_histograms)
    .wrapping_mul(num_histograms);
  iter = 0usize;
  while iter < iters {
    {
      let mut sample = HistogramType::default();
      HistogramClear(&mut sample);
      RandomSample(&mut seed, data, length, stride, &mut sample);
      HistogramAddHistogram(&mut histograms[(iter.wrapping_rem(num_histograms) as (usize))],
                            &mut sample);
    }
    iter = iter.wrapping_add(1 as (usize));
  }
}

fn BitCost(count: usize) -> super::util::floatX {
  if count == 0usize {
    -2.0 as super::util::floatX
  } else {
    FastLog2(count as u64)
  }
}
fn FindBlocks<HistogramType: SliceWrapper<u32> + SliceWrapperMut<u32> + CostAccessors,
              IntegerType: Sized + Clone>
  (data: &[IntegerType],
   length: usize,
   block_switch_bitcost: super::util::floatX,
   num_histograms: usize,
   histograms: &[HistogramType],
   mut insert_cost: &mut [super::util::floatX],
   mut cost: &mut [Mem256f],
   mut switch_signal: &mut [u8],
   mut block_id: &mut [u8])
   -> usize
  where u64: core::convert::From<IntegerType>
{
  if num_histograms == 0 {
    return 0;
  }
  let data_size: usize = histograms[0].slice().len();
  let bitmaplen: usize = num_histograms.wrapping_add(7usize) >> 3i32;
  let mut num_blocks: usize = 1usize;
  let mut i: usize;
  let mut j: usize;
  0i32;
  if num_histograms <= 1usize {
    i = 0usize;
    while i < length {
      {
        block_id[(i as (usize))] = 0i32 as (u8);
      }
      i = i.wrapping_add(1 as (usize));
    }
    return 1usize;
  }
  for item in insert_cost[..(data_size * num_histograms)].iter_mut() {
    *item = 0.0 as super::util::floatX;
  }
  i = 0usize;
  while i < num_histograms {
    {
      insert_cost[(i as (usize))] = FastLog2((histograms[(i as (usize))]).total_count() as (u32) as
                                             (u64));
    }
    i = i.wrapping_add(1 as (usize));
  }
  i = data_size;
  while i != 0usize {
    i = i.wrapping_sub(1 as (usize));
    j = 0usize;
    while j < num_histograms {
      {
        insert_cost[(i.wrapping_mul(num_histograms).wrapping_add(j) as (usize))] =
          insert_cost[(j as (usize))] - BitCost((histograms[(j as (usize))]).slice()[i] as (usize));
      }
      j = j.wrapping_add(1 as (usize));
    }
  }
  for item in cost.iter_mut() {
    *item = Mem256f::default();
  }
  for item in switch_signal[..(length * bitmaplen)].iter_mut() {
    *item = 0;
  }
  i = 0usize;
  for (byte_ix, data_byte_ix) in data[..length].iter().enumerate() {
    {
      let ix: usize = byte_ix.wrapping_mul(bitmaplen);
      let insert_cost_ix: usize = u64::from(data[(byte_ix as (usize))].clone())
        .wrapping_mul(num_histograms as u64) as usize;
      let mut min_cost: super::util::floatX = 1e38 as super::util::floatX;
      let mut block_switch_cost: super::util::floatX = block_switch_bitcost;
      for (k, insert_cost_iter) in insert_cost[insert_cost_ix..(insert_cost_ix + num_histograms)].iter().enumerate() {
          let cost_iter = &mut cost[(k >> 3)].0[k&7];
          *cost_iter += *insert_cost_iter;
          if *cost_iter < min_cost {
              min_cost = *cost_iter;
              block_id[byte_ix] = k as u8;
          }
      }
      if byte_ix < 2000usize {
        block_switch_cost = block_switch_cost *
                            (0.77 as super::util::floatX + 0.07 as super::util::floatX * byte_ix as (super::util::floatX) / 2000i32 as (super::util::floatX));
      }
      update_cost_and_signal(num_histograms as u32, ix, min_cost, block_switch_cost, cost, switch_signal);
    }
  }
  {
    let mut byte_ix: usize = length.wrapping_sub(1usize);
    let mut ix: usize = byte_ix.wrapping_mul(bitmaplen);
    let mut cur_id: u8 = block_id[(byte_ix as (usize))];
    while byte_ix > 0usize {
      let mask: u8 = (1u32 << (cur_id as (i32) & 7i32)) as (u8);
      0i32;
      byte_ix = byte_ix.wrapping_sub(1 as (usize));
      ix = ix.wrapping_sub(bitmaplen);
      if switch_signal[(ix.wrapping_add((cur_id as (i32) >> 3i32) as (usize)) as
          (usize))] as (i32) & mask as (i32) != 0 {
        if cur_id as (i32) != block_id[(byte_ix as (usize))] as (i32) {
          cur_id = block_id[(byte_ix as (usize))];
          num_blocks = num_blocks.wrapping_add(1 as (usize));
        }
      }
      block_id[(byte_ix as (usize))] = cur_id;
    }
  }
  num_blocks
}

fn RemapBlockIds(mut block_ids: &mut [u8],
                 length: usize,
                 mut new_id: &mut [u16],
                 num_histograms: usize)
                 -> usize {
  static kInvalidId: u16 = 256i32 as (u16);
  let mut next_id: u16 = 0i32 as (u16);
  let mut i: usize;
  i = 0usize;
  while i < num_histograms {
    {
      new_id[(i as (usize))] = kInvalidId;
    }
    i = i.wrapping_add(1 as (usize));
  }
  i = 0usize;
  while i < length {
    {
      0i32;
      if new_id[(block_ids[(i as (usize))] as (usize))] as (i32) == kInvalidId as (i32) {
        new_id[(block_ids[(i as (usize))] as (usize))] = {
          let _old = next_id;
          next_id = (next_id as (i32) + 1) as (u16);
          _old
        };
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  i = 0usize;
  while i < length {
    {
      block_ids[(i as (usize))] = new_id[(block_ids[(i as (usize))] as (usize))] as (u8);
      0i32;
    }
    i = i.wrapping_add(1 as (usize));
  }
  0i32;
  next_id as (usize)
}


fn BuildBlockHistograms<HistogramType: SliceWrapper<u32> + SliceWrapperMut<u32> + CostAccessors,
                        IntegerType: Sized + Clone>
  (data: &[IntegerType],
   length: usize,
   block_ids: &[u8],
   num_histograms: usize,
   mut histograms: &mut [HistogramType])
  where u64: core::convert::From<IntegerType>
{
  let mut i: usize;
  ClearHistograms(histograms, num_histograms);
  i = 0usize;
  while i < length {
    {
      HistogramAddItem(&mut histograms[(block_ids[(i as (usize))] as (usize))],
                       u64::from(data[(i as (usize))].clone()) as usize);
    }
    i = i.wrapping_add(1 as (usize));
  }
}


fn ClusterBlocks<HistogramType:SliceWrapper<u32>+SliceWrapperMut<u32>+CostAccessors+core::default::Default+Clone,
                        AllocU8:alloc::Allocator<u8>,
                        AllocU32:alloc::Allocator<u32>,
                        AllocHT:alloc::Allocator<HistogramType>,
                        AllocHP:alloc::Allocator<HistogramPair>,
                        IntegerType:Sized+Clone>(mut m8: &mut AllocU8,
                        mut m32:&mut AllocU32,
                        mut mht:&mut AllocHT,
                        mut mhp:&mut AllocHP,
                        data: &[IntegerType],
                        length: usize,
                        num_blocks: usize,
                        block_ids: &mut [u8],
mut split: &mut BlockSplit<AllocU8,AllocU32>) where u64: core::convert::From<IntegerType>{

  let mut histogram_symbols = m32.alloc_cell(num_blocks);
  let mut block_lengths = m32.alloc_cell(num_blocks);
  let expected_num_clusters: usize = (16usize)
    .wrapping_mul(num_blocks.wrapping_add(64usize).wrapping_sub(1usize))
    .wrapping_div(64usize);
  let mut all_histograms_size: usize = 0usize;
  let mut all_histograms_capacity: usize = expected_num_clusters;
  let mut all_histograms = mht.alloc_cell(all_histograms_capacity);
  let mut cluster_size_size: usize = 0usize;
  let mut cluster_size_capacity: usize = expected_num_clusters;
  let mut cluster_size = m32.alloc_cell(cluster_size_capacity);
  let mut num_clusters: usize = 0usize;
  let mut histograms = mht.alloc_cell(brotli_min_size_t(num_blocks, 64usize));
  let mut max_num_pairs: usize = (64i32 * 64i32 / 2i32) as (usize);
  let pairs_capacity: usize = max_num_pairs.wrapping_add(1usize);
  let mut pairs = mhp.alloc_cell(pairs_capacity);
  let mut pos: usize = 0usize;
  let mut clusters: AllocU32::AllocatedMemory;
  let num_final_clusters: usize;
  static kInvalidIndex: u32 = !(0u32);
  let mut i: usize;
  let mut sizes: [u32; 64] = [0; 64];
  let mut new_clusters: [u32; 64] = [0; 64];
  let mut symbols: [u32; 64] = [0; 64];
  let mut remap: [u32; 64] = [0; 64];
  for item_mut in block_lengths.slice_mut()[..num_blocks].iter_mut() {
    *item_mut = 0;
  }
  {
    let mut block_idx: usize = 0usize;
    i = 0usize;
    while i < length {
      {
        0i32;
        {
          let _rhs = 1;
          let _lhs = &mut block_lengths.slice_mut()[(block_idx as (usize))];
          *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        if i.wrapping_add(1usize) == length ||
           block_ids[(i as (usize))] as (i32) !=
           block_ids[(i.wrapping_add(1usize) as (usize))] as (i32) {
          block_idx = block_idx.wrapping_add(1 as (usize));
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
    0i32;
  }
  i = 0usize;
  while i < num_blocks {
    {
      let num_to_combine: usize = brotli_min_size_t(num_blocks.wrapping_sub(i), 64usize);
      let num_new_clusters: usize;
      let mut j: usize;
      j = 0usize;
      while j < num_to_combine {
        {
          let mut k: usize;
          HistogramClear(&mut histograms.slice_mut()[(j as (usize))]);
          k = 0usize;
          while k < block_lengths.slice()[(i.wrapping_add(j) as (usize))] as (usize) {
            {
              HistogramAddItem(&mut histograms.slice_mut()[(j as (usize))],
                               u64::from(data[{
                                             let _old = pos;
                                             pos = pos.wrapping_add(1 as (usize));
                                             _old
                                           }]
                                           .clone()) as usize);
            }
            k = k.wrapping_add(1 as (usize));
          }
          let new_cost = BrotliPopulationCost(&histograms.slice()[(j as (usize))]);
          (histograms.slice_mut()[(j as (usize))]).set_bit_cost(new_cost);

          new_clusters[j] = j as (u32);
          symbols[j] = j as (u32);
          sizes[j] = 1u32;
        }
        j = j.wrapping_add(1 as (usize));
      }
      num_new_clusters = BrotliHistogramCombine(histograms.slice_mut(),
                                                &mut sizes[..],
                                                &mut symbols[..],
                                                &mut new_clusters[..],
                                                pairs.slice_mut(),
                                                num_to_combine,
                                                num_to_combine,
                                                64usize,
                                                max_num_pairs);
      {
        if all_histograms_capacity < all_histograms_size.wrapping_add(num_new_clusters) {
          let mut _new_size: usize = if all_histograms_capacity == 0usize {
            all_histograms_size.wrapping_add(num_new_clusters)
          } else {
            all_histograms_capacity
          };
          while _new_size < all_histograms_size.wrapping_add(num_new_clusters) {
            _new_size = _new_size.wrapping_mul(2usize);
          }
          let mut new_array = mht.alloc_cell(_new_size);
          new_array.slice_mut()[..all_histograms_capacity]
            .clone_from_slice(&all_histograms.slice()[..all_histograms_capacity]);
          mht.free_cell(core::mem::replace(&mut all_histograms, new_array));
          all_histograms_capacity = _new_size;
        }
      }
      {
        if cluster_size_capacity < cluster_size_size.wrapping_add(num_new_clusters) {
          let mut _new_size: usize = if cluster_size_capacity == 0usize {
            cluster_size_size.wrapping_add(num_new_clusters)
          } else {
            cluster_size_capacity
          };
          while _new_size < cluster_size_size.wrapping_add(num_new_clusters) {
            _new_size = _new_size.wrapping_mul(2usize);
          }
          let mut new_array = m32.alloc_cell(_new_size);
          new_array.slice_mut()[..cluster_size_capacity]
            .clone_from_slice(&cluster_size.slice()[..cluster_size_capacity]);
          m32.free_cell(core::mem::replace(&mut cluster_size, new_array));
          cluster_size_capacity = _new_size;
        }
      }
      j = 0usize;
      while j < num_new_clusters {
        {
          all_histograms.slice_mut()[({
             let _old = all_histograms_size;
             all_histograms_size = all_histograms_size.wrapping_add(1 as (usize));
             _old
           } as (usize))] = histograms.slice()[(new_clusters[j] as (usize))].clone();
          cluster_size.slice_mut()[({
             let _old = cluster_size_size;
             cluster_size_size = cluster_size_size.wrapping_add(1 as (usize));
             _old
           } as (usize))] = sizes[new_clusters[j] as (usize)];
          remap[new_clusters[j] as (usize)] = j as (u32);
        }
        j = j.wrapping_add(1 as (usize));
      }
      j = 0usize;
      while j < num_to_combine {
        {
          histogram_symbols.slice_mut()[(i.wrapping_add(j) as (usize))] =
            (num_clusters as (u32)).wrapping_add(remap[symbols[j] as (usize)]);
        }
        j = j.wrapping_add(1 as (usize));
      }
      num_clusters = num_clusters.wrapping_add(num_new_clusters);
      0i32;
      0i32;
    }
    i = i.wrapping_add(64usize);
  }
  mht.free_cell(core::mem::replace(&mut histograms, AllocHT::AllocatedMemory::default()));
  max_num_pairs = brotli_min_size_t((64usize).wrapping_mul(num_clusters),
                                    num_clusters.wrapping_div(2usize).wrapping_mul(num_clusters));
  if pairs_capacity < max_num_pairs.wrapping_add(1usize) {
    let new_cell = mhp.alloc_cell(max_num_pairs.wrapping_add(1usize));
    mhp.free_cell(core::mem::replace(&mut pairs, new_cell));
  }
  clusters = m32.alloc_cell(num_clusters);
  i = 0usize;
  for item in clusters.slice_mut()[..num_clusters].iter_mut() {
    *item = i as u32;
    i = i.wrapping_add(1 as (usize));
  }
  num_final_clusters = BrotliHistogramCombine(all_histograms.slice_mut(),
                                              cluster_size.slice_mut(),
                                              histogram_symbols.slice_mut(),
                                              clusters.slice_mut(),
                                              pairs.slice_mut(),
                                              num_clusters,
                                              num_blocks,
                                              256usize,
                                              max_num_pairs);
  mhp.free_cell(core::mem::replace(&mut pairs, AllocHP::AllocatedMemory::default()));
  m32.free_cell(core::mem::replace(&mut cluster_size, AllocU32::AllocatedMemory::default()));

  let mut new_index = m32.alloc_cell(num_clusters);
  for item in new_index.slice_mut().iter_mut() {
    *item = kInvalidIndex;
  }
  pos = 0usize;
  {
    let mut next_index: u32 = 0u32;
    i = 0usize;
    while i < num_blocks {
      {
        let mut histo: HistogramType = HistogramType::default();
        let mut j: usize;
        let mut best_out: u32;
        let mut best_bits: super::util::floatX;
        HistogramClear(&mut histo);
        j = 0usize;
        while j < block_lengths.slice()[(i as (usize))] as (usize) {
          {
            HistogramAddItem(&mut histo,
                             u64::from(data[({
                                          let _old = pos;
                                          pos = pos.wrapping_add(1 as (usize));
                                          _old
                                        } as
                                        (usize))]
                                           .clone()) as (usize));
          }
          j = j.wrapping_add(1 as (usize));
        }
        best_out = if i == 0usize {
          histogram_symbols.slice()[(0usize)]
        } else {
          histogram_symbols.slice()[(i.wrapping_sub(1usize) as (usize))]
        };
        best_bits = BrotliHistogramBitCostDistance(&mut histo,
                                                   &mut all_histograms.slice_mut()[(best_out as
                                                         (usize))]);
        j = 0usize;
        while j < num_final_clusters {
          {
            let cur_bits: super::util::floatX =
              BrotliHistogramBitCostDistance(&mut histo,
                                             &mut all_histograms.slice_mut()[(clusters.slice()[(j as (usize))] as
                                                   (usize))]);
            if cur_bits < best_bits {
              best_bits = cur_bits;
              best_out = clusters.slice()[(j as (usize))];
            }
          }
          j = j.wrapping_add(1 as (usize));
        }
        histogram_symbols.slice_mut()[(i as (usize))] = best_out;
        if new_index.slice()[(best_out as (usize))] == kInvalidIndex {
          new_index.slice_mut()[(best_out as (usize))] = {
            let _old = next_index;
            next_index = next_index.wrapping_add(1 as (u32));
            _old
          };
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
  }
  m32.free_cell(core::mem::replace(&mut clusters, AllocU32::AllocatedMemory::default()));
  mht.free_cell(core::mem::replace(&mut all_histograms, AllocHT::AllocatedMemory::default()));
  {
    if (*split).types_alloc_size() < num_blocks {
      let mut _new_size: usize = if (*split).types_alloc_size() == 0usize {
        num_blocks
      } else {
        (*split).types_alloc_size()
      };
      while _new_size < num_blocks {
        _new_size = _new_size.wrapping_mul(2usize);
      }
      let mut new_array = m8.alloc_cell(_new_size);
      new_array.slice_mut()[..(*split).types_alloc_size()]
        .clone_from_slice(&(*split).types.slice()[..(*split).types_alloc_size()]);
      m8.free_cell(core::mem::replace(&mut (*split).types, new_array));
    }
  }
  {
    if (*split).lengths_alloc_size() < num_blocks {
      let mut _new_size: usize = if (*split).lengths_alloc_size() == 0usize {
        num_blocks
      } else {
        (*split).lengths_alloc_size()
      };
      while _new_size < num_blocks {
        _new_size = _new_size.wrapping_mul(2usize);
      }
      let mut new_array = m32.alloc_cell(_new_size);
      new_array.slice_mut()[..(*split).lengths_alloc_size()].clone_from_slice((*split)
                                                                                .lengths
                                                                                .slice());
      m32.free_cell(core::mem::replace(&mut (*split).lengths, new_array));
    }
  }
  {
    let mut cur_length: u32 = 0u32;
    let mut block_idx: usize = 0usize;
    let mut max_type: u8 = 0i32 as (u8);
    i = 0usize;
    while i < num_blocks {
      {
        cur_length = cur_length.wrapping_add(block_lengths.slice()[(i as (usize))]);
        if i.wrapping_add(1usize) == num_blocks ||
           histogram_symbols.slice()[(i as (usize))] !=
           histogram_symbols.slice()[(i.wrapping_add(1usize) as (usize))] {
          let id: u8 = new_index.slice()[(histogram_symbols.slice()[(i as (usize))] as
           (usize))] as (u8);
          (*split).types.slice_mut()[(block_idx as (usize))] = id;
          (*split).lengths.slice_mut()[(block_idx as (usize))] = cur_length;
          max_type = brotli_max_uint8_t(max_type, id);
          cur_length = 0u32;
          block_idx = block_idx.wrapping_add(1 as (usize));
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
    (*split).num_blocks = block_idx;
    (*split).num_types = (max_type as (usize)).wrapping_add(1usize);
  }
  m32.free_cell(new_index);
  m32.free_cell(block_lengths);
  m32.free_cell(histogram_symbols);
}

fn SplitByteVector<HistogramType:SliceWrapper<u32>+SliceWrapperMut<u32>+CostAccessors+core::default::Default+Clone,
                        AllocU8:alloc::Allocator<u8>,
                        AllocU16:alloc::Allocator<u16>,
                        AllocU32:alloc::Allocator<u32>,
                        AllocF64:alloc::Allocator<super::util::floatX>,
                        AllocFV:alloc::Allocator<Mem256f>,
                        AllocHT:alloc::Allocator<HistogramType>,
                        AllocHP:alloc::Allocator<HistogramPair>,
                        IntegerType:Sized+Clone>(mut m8: &mut AllocU8,
                        mut m16:&mut AllocU16,
                        mut m32:&mut AllocU32,
                        mut mf64:&mut AllocF64,
                        mut mfv:&mut AllocFV,
                        mut mht:&mut AllocHT,
                        mut mhp:&mut AllocHP,
                          data: &[IntegerType],
                          length: usize,
                          literals_per_histogram: usize,
                          max_histograms: usize,
                          sampling_stride_length: usize,
                          block_switch_cost: super::util::floatX,
                          params: &BrotliEncoderParams,
mut split: &mut BlockSplit<AllocU8, AllocU32>) where u64: core::convert::From<IntegerType>{
  let data_size: usize = HistogramType::default().slice().len();
  let mut num_histograms: usize = length.wrapping_div(literals_per_histogram).wrapping_add(1usize);
  if num_histograms > max_histograms {
    num_histograms = max_histograms;
  }
  if length == 0usize {
    (*split).num_types = 1usize;
    return;
  } else if length < kMinLengthForBlockSplitting {
    {
      if (*split).types_alloc_size() < (*split).num_blocks.wrapping_add(1usize) {
        let mut _new_size: usize = if (*split).types_alloc_size() == 0usize {
          (*split).num_blocks.wrapping_add(1usize)
        } else {
          (*split).types_alloc_size()
        };

        while _new_size < (*split).num_blocks.wrapping_add(1usize) {
          _new_size = _new_size.wrapping_mul(2usize);
        }
        let mut new_array = m8.alloc_cell(_new_size);
        new_array.slice_mut()[..(*split).types_alloc_size()]
          .clone_from_slice(&(*split).types.slice()[..(*split).types_alloc_size()]);
        m8.free_cell(core::mem::replace(&mut (*split).types, new_array));
      }
    }
    {
      if (*split).lengths_alloc_size() < (*split).num_blocks.wrapping_add(1usize) {
        let mut _new_size: usize = if (*split).lengths_alloc_size() == 0usize {
          (*split).num_blocks.wrapping_add(1usize)
        } else {
          (*split).lengths_alloc_size()
        };
        while _new_size < (*split).num_blocks.wrapping_add(1usize) {
          _new_size = _new_size.wrapping_mul(2usize);
        }
        let mut new_array = m32.alloc_cell(_new_size);
        new_array.slice_mut()[..(*split).lengths_alloc_size()]
          .clone_from_slice(&(*split).lengths.slice()[..(*split).lengths_alloc_size()]);
        m32.free_cell(core::mem::replace(&mut (*split).lengths, new_array));
      }
    }
    (*split).num_types = 1usize;
    (*split).types.slice_mut()[((*split).num_blocks as (usize))] = 0i32 as (u8);
    (*split).lengths.slice_mut()[((*split).num_blocks as (usize))] = length as (u32);
    (*split).num_blocks = (*split).num_blocks.wrapping_add(1 as (usize));
    return;
  }
  let mut histograms = mht.alloc_cell(num_histograms);

  InitialEntropyCodes(data,
                      length,
                      sampling_stride_length,
                      num_histograms,
                      histograms.slice_mut());
  RefineEntropyCodes(data,
                     length,
                     sampling_stride_length,
                     num_histograms,
                     histograms.slice_mut());
  {
    let mut block_ids = m8.alloc_cell(length);
    let mut num_blocks: usize = 0usize;
    let bitmaplen: usize = num_histograms.wrapping_add(7usize) >> 3i32;
    let mut insert_cost = mf64.alloc_cell(data_size.wrapping_mul(num_histograms));
    let mut cost = mfv.alloc_cell(((num_histograms + 7) >> 3));
    let mut switch_signal = m8.alloc_cell(length.wrapping_mul(bitmaplen));
    let mut new_id = m16.alloc_cell(num_histograms);
    let iters: usize = (if (*params).quality < 11 {
                          3i32
                        } else {
                          10i32
                        }) as (usize);
    let mut i: usize;
    i = 0usize;
    while i < iters {
      {
        num_blocks = FindBlocks(data,
                                length,
                                block_switch_cost,
                                num_histograms,
                                histograms.slice_mut(),
                                insert_cost.slice_mut(),
                                cost.slice_mut(),
                                switch_signal.slice_mut(),
                                block_ids.slice_mut());
        num_histograms = RemapBlockIds(block_ids.slice_mut(),
                                       length,
                                       new_id.slice_mut(),
                                       num_histograms);
        BuildBlockHistograms(data,
                             length,
                             block_ids.slice(),
                             num_histograms,
                             histograms.slice_mut());
      }
      i = i.wrapping_add(1 as (usize));
    }
    mf64.free_cell(insert_cost);
    mfv.free_cell(cost);
    m8.free_cell(switch_signal);
    m16.free_cell(new_id);
    mht.free_cell(histograms);
    ClusterBlocks(m8,
                  m32,
                  mht,
                  mhp,
                  data,
                  length,
                  num_blocks,
                  block_ids.slice_mut(),
                  split);
    m8.free_cell(block_ids);
  }
}

pub fn BrotliSplitBlock<AllocU8: alloc::Allocator<u8>,
                        AllocU16: alloc::Allocator<u16>,
                        AllocU32: alloc::Allocator<u32>,
                        AllocF64: alloc::Allocator<super::util::floatX>,
                        AllocFV:alloc::Allocator<Mem256f>,
                        AllocHL: alloc::Allocator<HistogramLiteral>,
                        AllocHC: alloc::Allocator<HistogramCommand>,
                        AllocHD: alloc::Allocator<HistogramDistance>,
                        AllocHP: alloc::Allocator<HistogramPair>>
  (mut m8: &mut AllocU8,
   mut m16: &mut AllocU16,
   mut m32: &mut AllocU32,
   mut mf64: &mut AllocF64,
   mut mfv: &mut AllocFV,
   mut mhl: &mut AllocHL,
   mut mhc: &mut AllocHC,
   mhd: &mut AllocHD,
   mut mhp: &mut AllocHP,
   cmds: &[Command],
   num_commands: usize,
   data: &[u8],
   pos: usize,
   mask: usize,
   params: &BrotliEncoderParams,
   mut literal_split: &mut BlockSplit<AllocU8, AllocU32>,
   mut insert_and_copy_split: &mut BlockSplit<AllocU8, AllocU32>,
   mut dist_split: &mut BlockSplit<AllocU8, AllocU32>) {
  {
    let literals_count: usize = CountLiterals(cmds, num_commands);
    let mut literals = m8.alloc_cell(literals_count);
    CopyLiteralsToByteArray(cmds, num_commands, data, pos, mask, literals.slice_mut());
    SplitByteVector(m8,
                    m16,
                    m32,
                    mf64,
                    mfv,
                    mhl,
                    mhp,
                    literals.slice(),
                    literals_count,
                    kSymbolsPerLiteralHistogram,
                    kMaxLiteralHistograms,
                    kLiteralStrideLength,
                    kLiteralBlockSwitchCost,
                    params,
                    literal_split);
    m8.free_cell(literals);
  }
  {
    let mut insert_and_copy_codes = m16.alloc_cell(num_commands);
    for i in 0..core::cmp::min(num_commands, cmds.len()) {
      insert_and_copy_codes.slice_mut()[(i as (usize))] = (cmds[(i as (usize))]).cmd_prefix_;
    }
    SplitByteVector(m8,
                    m16,
                    m32,
                    mf64,
                    mfv,
                    mhc,
                    mhp,
                    insert_and_copy_codes.slice(),
                    num_commands,
                    kSymbolsPerCommandHistogram,
                    kMaxCommandHistograms,
                    kCommandStrideLength,
                    kCommandBlockSwitchCost,
                    params,
                    insert_and_copy_split);
    m16.free_cell(insert_and_copy_codes);
  }
  {
    let mut distance_prefixes = m16.alloc_cell(num_commands);
    let mut j: usize = 0usize;
    let mut i: usize;
    i = 0usize;
    while i < num_commands {
      {
        let cmd = &cmds[(i as (usize))];
        if CommandCopyLen(cmd) != 0 && ((*cmd).cmd_prefix_ as (i32) >= 128i32) {
          distance_prefixes.slice_mut()[({
             let _old = j;
             j = j.wrapping_add(1 as (usize));
             _old
           } as (usize))] = (*cmd).dist_prefix_;
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
    SplitByteVector(m8,
                    m16,
                    m32,
                    mf64,
                    mfv,
                    mhd,
                    mhp,
                    distance_prefixes.slice(),
                    j,
                    kSymbolsPerDistanceHistogram,
                    kMaxCommandHistograms,
                    kCommandStrideLength,
                    kDistanceBlockSwitchCost,
                    params,
                    dist_split);
    m16.free_cell(distance_prefixes);
  }
}
