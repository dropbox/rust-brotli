use core;

use super::bit_cost::BrotliPopulationCost;
use super::backward_references::{BrotliEncoderParams, BrotliEncoderMode,
};
use super::util::{FastLog2, brotli_max_uint8_t, brotli_max_size_t, brotli_min_size_t};
use super::histogram::{HistogramAddVector, CostAccessors, ClearHistograms, HistogramClear, HistogramAddHistogram, HistogramAddItem};
use super::command::{Command};
use super::cluster::{BrotliHistogramBitCostDistance, BrotliHistogramCombine, HistogramPair};
use super::super::alloc::{SliceWrapper,SliceWrapperMut};
use super::super::alloc;
use super::block_split::BlockSplit;
static kMaxLiteralHistograms: usize = 100usize;

static kMaxCommandHistograms: usize = 50usize;

static kLiteralBlockSwitchCost: f64 = 28.1f64;

static kCommandBlockSwitchCost: f64 = 13.5f64;

static kDistanceBlockSwitchCost: f64 = 14.6f64;

static kLiteralStrideLength: usize = 70usize;

static kCommandStrideLength: usize = 40usize;

static kSymbolsPerLiteralHistogram: usize = 544usize;

static kSymbolsPerCommandHistogram: usize = 530usize;

static kSymbolsPerDistanceHistogram: usize = 544usize;

static kMinLengthForBlockSplitting: usize = 128usize;

static kIterMulForRefining: usize = 2usize;

static kMinItersForRefining: usize = 100usize;


fn CountLiterals(mut cmds: &[Command], num_commands: usize) -> usize {
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

fn CommandCopyLen(mut xself: &Command) -> u32 {
  (*xself).copy_len_ & 0xffffffu32
}

fn CopyLiteralsToByteArray(mut cmds: &[Command],
                           num_commands: usize,
                           mut data: &[u8],
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
        let mut head_size: usize = mask.wrapping_add(1usize).wrapping_sub(from_pos);
        literals[(pos as (usize))..((pos + head_size) as usize)].clone_from_slice(
                &data[(from_pos as (usize))..((from_pos + head_size) as usize)]);
        from_pos = 0usize;
        pos = pos.wrapping_add(head_size);
        insert_len = insert_len.wrapping_sub(head_size);
      }
      if insert_len > 0usize {
        literals[(pos as (usize))..(pos as usize + insert_len)].clone_from_slice(
                &data[(from_pos as (usize))..(from_pos as usize + insert_len)]);
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



fn InitialEntropyCodes<HistogramType:SliceWrapper<u32>+SliceWrapperMut<u32>+CostAccessors>(mut data: &[u8],
                              mut length: usize,
                              mut stride: usize,
                              mut num_histograms: usize,
                              mut histograms: &mut [HistogramType]) {
  let mut seed: u32 = 7u32;
  let mut block_length: usize = length.wrapping_div(num_histograms);
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

fn RandomSample<HistogramType:SliceWrapper<u32>+SliceWrapperMut<u32>+CostAccessors>(mut seed: &mut u32,
                       mut data: &[u8],
                       mut length: usize,
                       mut stride: usize,
                       mut sample: &mut HistogramType) {
  let mut pos: usize = 0usize;
  if stride >= length {
    pos = 0usize;
    stride = length;
  } else {
    pos = (MyRand(seed) as (usize)).wrapping_rem(length.wrapping_sub(stride).wrapping_add(1usize));
  }
  HistogramAddVector(sample, &data[(pos as (usize))..], stride);
}

fn RefineEntropyCodes<HistogramType:SliceWrapper<u32>+SliceWrapperMut<u32>+CostAccessors+core::default::Default>(mut data: &[u8],
                             mut length: usize,
                             mut stride: usize,
                             mut num_histograms: usize,
                             mut histograms: &mut [HistogramType]) {
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

fn BitCost(mut count: usize) -> f64 {
  if count == 0usize {
    -2.0f64
  } else {
    FastLog2(count)
  }
}
fn FindBlocks<HistogramType:SliceWrapper<u32>+SliceWrapperMut<u32>+CostAccessors>(mut data: &[u8],
                     length: usize,
                     block_switch_bitcost: f64,
                     num_histograms: usize,
                     mut histograms: &[HistogramType],
                     mut insert_cost: &mut [f64],
                     mut cost: &mut [f64],
                     mut switch_signal: &mut [u8],
                     mut block_id: &mut [u8])
                     -> usize {
  if num_histograms == 0 {
     return 0
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
     *item = 0.0f64;
  }
  i = 0usize;
  while i < num_histograms {
    {
      insert_cost[(i as (usize))] = FastLog2((histograms[(i as (usize))]).total_count() as (u32) as
                                             (usize));
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
  for item in cost[..num_histograms].iter_mut() {
    *item = 0.0f64;
  }
  for item in switch_signal[..(length * bitmaplen)].iter_mut() {
    *item = 0;
  }
  i = 0usize;
  while i < length {
    {
      let byte_ix: usize = i;
      let mut ix: usize = byte_ix.wrapping_mul(bitmaplen);
      let mut insert_cost_ix: usize = (data[(byte_ix as (usize))] as (usize))
        .wrapping_mul(num_histograms);
      let mut min_cost: f64 = 1e99f64;
      let mut block_switch_cost: f64 = block_switch_bitcost;
      let mut k: usize;
      k = 0usize;
      while k < num_histograms {
        {
          {
            let _rhs = insert_cost[(insert_cost_ix.wrapping_add(k) as (usize))];
            let _lhs = &mut cost[(k as (usize))];
            *_lhs = *_lhs + _rhs;
          }
          if cost[(k as (usize))] < min_cost {
            min_cost = cost[(k as (usize))];
            block_id[(byte_ix as (usize))] = k as (u8);
          }
        }
        k = k.wrapping_add(1 as (usize));
      }
      if byte_ix < 2000usize {
        block_switch_cost = block_switch_cost *
                            (0.77f64 + 0.07f64 * byte_ix as (f64) / 2000i32 as (f64));
      }
      k = 0usize;
      while k < num_histograms {
        {
          {
            let _rhs = min_cost;
            let _lhs = &mut cost[(k as (usize))];
            *_lhs = *_lhs - _rhs;
          }
          if cost[(k as (usize))] >= block_switch_cost {
            let mask: u8 = (1u32 << (k & 7usize)) as (u8);
            cost[(k as (usize))] = block_switch_cost;
            0i32;
            {
              let _rhs = mask;
              let _lhs = &mut switch_signal[(ix.wrapping_add(k >> 3i32) as (usize))];
              *_lhs = (*_lhs as (i32) | _rhs as (i32)) as (u8);
            }
          }
        }
        k = k.wrapping_add(1 as (usize));
      }
    }
    i = i.wrapping_add(1 as (usize));
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


fn BuildBlockHistograms<HistogramType:SliceWrapper<u32>+SliceWrapperMut<u32>+CostAccessors>(mut data: &[u8],
                               length: usize,
                               mut block_ids: &[u8],
                               num_histograms: usize,
                               mut histograms: &mut [HistogramType]) {
  let mut i: usize;
  ClearHistograms(histograms, num_histograms);
  i = 0usize;
  while i < length {
    {
      HistogramAddItem(&mut histograms[(block_ids[(i as (usize))] as (usize))],
                          data[(i as (usize))] as (usize));
    }
    i = i.wrapping_add(1 as (usize));
  }
}


fn ClusterBlocks<HistogramType:SliceWrapper<u32>+SliceWrapperMut<u32>+CostAccessors+core::default::Default+Clone,
                        AllocU8:alloc::Allocator<u8>,
                        AllocU32:alloc::Allocator<u32>,
                        AllocHT:alloc::Allocator<HistogramType>,
                        AllocHP:alloc::Allocator<HistogramPair>>(mut m8: &mut AllocU8,
                        mut m32:&mut AllocU32,
                        mut mht:&mut AllocHT,
                        mut mhp:&mut AllocHP,
                        mut data: &[u8],
                        length: usize,
                        num_blocks: usize,
                        mut block_ids: &mut [u8],
                        mut split: &mut BlockSplit<AllocU8,AllocU32>) {

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
  let mut pairs_capacity: usize = max_num_pairs.wrapping_add(1usize);
  let mut pairs = mhp.alloc_cell(pairs_capacity);
  let mut pos: usize = 0usize;
  let mut clusters: AllocU32::AllocatedMemory;
  let mut num_final_clusters: usize;
  static kInvalidIndex: u32 = !(0u32);
  let mut i: usize;
  let mut sizes: [u32; 64] = [0;64];
  let mut new_clusters: [u32; 64] = [0;64];
  let mut symbols: [u32; 64] = [0;64];
  let mut remap: [u32; 64] = [0;64];
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
      let mut num_new_clusters: usize;
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
                                  data[({
                                     let _old = pos;
                                     pos = pos.wrapping_add(1 as (usize));
                                     _old
                                   } as (usize))] as (usize));
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
          new_array.slice_mut()[..all_histograms_capacity].clone_from_slice(&all_histograms.slice()[..all_histograms_capacity]);
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
          new_array.slice_mut()[..cluster_size_capacity].clone_from_slice(&cluster_size.slice()[..cluster_size_capacity]);
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
    mhp.free_cell(core::mem::replace(&mut pairs,
                                     new_cell));
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
  i = 0usize;
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
        let mut best_bits: f64;
        HistogramClear(&mut histo);
        j = 0usize;
        while j < block_lengths.slice()[(i as (usize))] as (usize) {
          {
            HistogramAddItem(&mut histo,
                                data[({
                                   let _old = pos;
                                   pos = pos.wrapping_add(1 as (usize));
                                   _old
                                 } as (usize))] as (usize));
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
            let cur_bits: f64 =
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
      new_array.slice_mut()[..(*split).types_alloc_size()].clone_from_slice(
          &(*split).types.slice()[..(*split).types_alloc_size()]);
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
      new_array.slice_mut()[..(*split).lengths_alloc_size()].clone_from_slice(
         (*split).lengths.slice());
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
          let id: u8 = new_index.slice()[(histogram_symbols.slice()[(i as (usize))] as (usize))] as (u8);
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
/*
fn SplitByteVectorLiteral(mut m: &mut [MemoryManager],
                          mut data: &[u8],
                          length: usize,
                          literals_per_histogram: usize,
                          max_histograms: usize,
                          sampling_stride_length: usize,
                          block_switch_cost: f64,
                          mut params: &[BrotliEncoderParams],
                          mut split: &mut BlockSplit) {
  let data_size: usize = HistogramDataSizeLiteral();
  let mut num_histograms: usize = length.wrapping_div(literals_per_histogram).wrapping_add(1usize);
  let mut histograms: *mut HistogramLiteral;
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
        let mut new_array: *mut u8;
        while _new_size < (*split).num_blocks.wrapping_add(1usize) {
          _new_size = _new_size.wrapping_mul(2usize);
        }
        new_array = if _new_size != 0 {
          BrotliAllocate(m, _new_size.wrapping_mul(::std::mem::size_of::<u8>()))
        } else {
          0i32
        };
        if !!(0i32 == 0) && ((*split).types_alloc_size != 0usize) {
          memcpy(new_array,
                 (*split).types,
                 (*split).types_alloc_size.wrapping_mul(::std::mem::size_of::<u8>()));
        }
        {
          BrotliFree(m, (*split).types);
          (*split).types = 0i32;
        }
        (*split).types = new_array;
        (*split).types_alloc_size = _new_size;
      }
    }
    {
      if (*split).lengths_alloc_size < (*split).num_blocks.wrapping_add(1usize) {
        let mut _new_size: usize = if (*split).lengths_alloc_size == 0usize {
          (*split).num_blocks.wrapping_add(1usize)
        } else {
          (*split).lengths_alloc_size
        };
        let mut new_array: *mut u32;
        while _new_size < (*split).num_blocks.wrapping_add(1usize) {
          _new_size = _new_size.wrapping_mul(2usize);
        }
        new_array = if _new_size != 0 {
          BrotliAllocate(m, _new_size.wrapping_mul(::std::mem::size_of::<u32>()))
        } else {
          0i32
        };
        if !!(0i32 == 0) && ((*split).lengths_alloc_size != 0usize) {
          memcpy(new_array,
                 (*split).lengths,
                 (*split).lengths_alloc_size.wrapping_mul(::std::mem::size_of::<u32>()));
        }
        {
          BrotliFree(m, (*split).lengths);
          (*split).lengths = 0i32;
        }
        (*split).lengths = new_array;
        (*split).lengths_alloc_size = _new_size;
      }
    }
    if !(0i32 == 0) {
      return;
    }
    (*split).num_types = 1usize;
    *(*split).types[((*split).num_blocks as (usize))..] = 0i32 as (u8);
    *(*split).lengths[((*split).num_blocks as (usize))..] = length as (u32);
    (*split).num_blocks = (*split).num_blocks.wrapping_add(1 as (usize));
    return;
  }
  histograms = if num_histograms != 0 {
    BrotliAllocate(m,
                   num_histograms.wrapping_mul(::std::mem::size_of::<HistogramLiteral>()))
  } else {
    0i32
  };
  if !(0i32 == 0) {
    return;
  }
  InitialEntropyCodesLiteral(data,
                             length,
                             sampling_stride_length,
                             num_histograms,
                             histograms);
  RefineEntropyCodesLiteral(data,
                            length,
                            sampling_stride_length,
                            num_histograms,
                            histograms);
  {
    let mut block_ids: *mut u8 = if length != 0 {
      BrotliAllocate(m, length.wrapping_mul(::std::mem::size_of::<u8>()))
    } else {
      0i32
    };
    let mut num_blocks: usize = 0usize;
    let bitmaplen: usize = num_histograms.wrapping_add(7usize) >> 3i32;
    let mut insert_cost: *mut f64 = if data_size.wrapping_mul(num_histograms) != 0 {
      BrotliAllocate(m,
                     data_size.wrapping_mul(num_histograms)
                       .wrapping_mul(::std::mem::size_of::<f64>()))
    } else {
      0i32
    };
    let mut cost: *mut f64 = if num_histograms != 0 {
      BrotliAllocate(m, num_histograms.wrapping_mul(::std::mem::size_of::<f64>()))
    } else {
      0i32
    };
    let mut switch_signal: *mut u8 = if length.wrapping_mul(bitmaplen) != 0 {
      BrotliAllocate(m,
                     length.wrapping_mul(bitmaplen).wrapping_mul(::std::mem::size_of::<u8>()))
    } else {
      0i32
    };
    let mut new_id: *mut u16 = if num_histograms != 0 {
      BrotliAllocate(m, num_histograms.wrapping_mul(::std::mem::size_of::<u16>()))
    } else {
      0i32
    };
    let iters: usize = (if (*params).quality < 11i32 {
                          3i32
                        } else {
                          10i32
                        }) as (usize);
    let mut i: usize;
    if !(0i32 == 0) {
      return;
    }
    i = 0usize;
    while i < iters {
      {
        num_blocks = FindBlocksLiteral(data,
                                       length,
                                       block_switch_cost,
                                       num_histograms,
                                       histograms,
                                       insert_cost,
                                       cost,
                                       switch_signal,
                                       block_ids);
        num_histograms = RemapBlockIdsLiteral(block_ids, length, new_id, num_histograms);
        BuildBlockHistogramsLiteral(data, length, block_ids, num_histograms, histograms);
      }
      i = i.wrapping_add(1 as (usize));
    }
    {
      BrotliFree(m, insert_cost);
      insert_cost = 0i32;
    }
    {
      BrotliFree(m, cost);
      cost = 0i32;
    }
    {
      BrotliFree(m, switch_signal);
      switch_signal = 0i32;
    }
    {
      BrotliFree(m, new_id);
      new_id = 0i32;
    }
    {
      BrotliFree(m, histograms);
      histograms = 0i32;
    }
    ClusterBlocksLiteral(m, data, length, num_blocks, block_ids, split);
    if !(0i32 == 0) {
      return;
    }
    {
      BrotliFree(m, block_ids);
      block_ids = 0i32;
    }
  }
}


pub fn BrotliSplitBlock(mut m: &mut [MemoryManager],
                        mut cmds: &[Command],
                        num_commands: usize,
                        mut data: &[u8],
                        pos: usize,
                        mask: usize,
                        mut params: &[BrotliEncoderParams],
                        mut literal_split: &mut [BlockSplit],
                        mut insert_and_copy_split: &mut [BlockSplit],
                        mut dist_split: &mut [BlockSplit]) {
  {
    let mut literals_count: usize = CountLiterals(cmds, num_commands);
    let mut literals: *mut u8 = if literals_count != 0 {
      BrotliAllocate(m, literals_count.wrapping_mul(::std::mem::size_of::<u8>()))
    } else {
      0i32
    };
    if !(0i32 == 0) {
      return;
    }
    CopyLiteralsToByteArray(cmds, num_commands, data, pos, mask, literals);
    SplitByteVectorLiteral(m,
                           literals,
                           literals_count,
                           kSymbolsPerLiteralHistogram,
                           kMaxLiteralHistograms,
                           kLiteralStrideLength,
                           kLiteralBlockSwitchCost,
                           params,
                           literal_split);
    if !(0i32 == 0) {
      return;
    }
    {
      BrotliFree(m, literals);
      literals = 0i32;
    }
  }
  {
    let mut insert_and_copy_codes: *mut u16 = if num_commands != 0 {
      BrotliAllocate(m, num_commands.wrapping_mul(::std::mem::size_of::<u16>()))
    } else {
      0i32
    };
    let mut i: usize;
    if !(0i32 == 0) {
      return;
    }
    i = 0usize;
    while i < num_commands {
      {
        insert_and_copy_codes[(i as (usize))] = (cmds[(i as (usize))]).cmd_prefix_;
      }
      i = i.wrapping_add(1 as (usize));
    }
    SplitByteVectorCommand(m,
                           insert_and_copy_codes,
                           num_commands,
                           kSymbolsPerCommandHistogram,
                           kMaxCommandHistograms,
                           kCommandStrideLength,
                           kCommandBlockSwitchCost,
                           params,
                           insert_and_copy_split);
    if !(0i32 == 0) {
      return;
    }
    {
      BrotliFree(m, insert_and_copy_codes);
      insert_and_copy_codes = 0i32;
    }
  }
  {
    let mut distance_prefixes: *mut u16 = if num_commands != 0 {
      BrotliAllocate(m, num_commands.wrapping_mul(::std::mem::size_of::<u16>()))
    } else {
      0i32
    };
    let mut j: usize = 0usize;
    let mut i: usize;
    if !(0i32 == 0) {
      return;
    }
    i = 0usize;
    while i < num_commands {
      {
        let mut cmd: *const Command = &cmds[(i as (usize))];
        if CommandCopyLen(cmd) != 0 && ((*cmd).cmd_prefix_ as (i32) >= 128i32) {
          distance_prefixes[({
             let _old = j;
             j = j.wrapping_add(1 as (usize));
             _old
           } as (usize))] = (*cmd).dist_prefix_;
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
    SplitByteVectorDistance(m,
                            distance_prefixes,
                            j,
                            kSymbolsPerDistanceHistogram,
                            kMaxCommandHistograms,
                            kCommandStrideLength,
                            kDistanceBlockSwitchCost,
                            params,
                            dist_split);
    if !(0i32 == 0) {
      return;
    }
    {
      BrotliFree(m, distance_prefixes);
      distance_prefixes = 0i32;
    }
  }
}
*/