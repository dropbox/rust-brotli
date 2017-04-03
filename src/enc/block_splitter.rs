use core;
use super::bit_cost::BrotliPopulationCost;
use super::backward_references::{BrotliEncoderParams, BrotliEncoderMode,
};
use super::command::{Command};
use super::cluster::{BrotliHistogramBitCostDistance, BrotliHistogramCombine};


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
/*

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
        memcpy(literals[(pos as (usize))..],
               data[(from_pos as (usize))..],
               head_size);
        from_pos = 0usize;
        pos = pos.wrapping_add(head_size);
        insert_len = insert_len.wrapping_sub(head_size);
      }
      if insert_len > 0usize {
        memcpy(literals[(pos as (usize))..],
               data[(from_pos as (usize))..],
               insert_len);
        pos = pos.wrapping_add(insert_len);
      }
      from_pos = from_pos.wrapping_add(insert_len).wrapping_add(CommandCopyLen(&cmds[(i as
                                                                                 (usize))]) as
                                                                (usize)) & mask;
    }
    i = i.wrapping_add(1 as (usize));
  }
}
fn HistogramDataSizeLiteral() -> usize {
  256usize
}



pub struct HistogramLiteral {
  pub data_: [u32; 256],
  pub total_count_: usize,
  pub bit_cost_: f64,
}

fn HistogramClearLiteral(mut xself: &mut HistogramLiteral) {
  memset((*xself).data_.as_mut_ptr(),
         0i32,
         ::std::mem::size_of::<[u32; 256]>());
  (*xself).total_count_ = 0usize;
  (*xself).bit_cost_ = 3.402e+38f64;
}

fn ClearHistogramsLiteral(mut array: &mut [HistogramLiteral], mut length: usize) {
  let mut i: usize;
  i = 0usize;
  while i < length {
    HistogramClearLiteral(array[(i as (usize))..]);
    i = i.wrapping_add(1 as (usize));
  }
}

fn MyRand(mut seed: &mut [u32]) -> u32 {
  *seed = (*seed).wrapping_mul(16807u32);
  if *seed == 0u32 {
    *seed = 1u32;
  }
  *seed
}

fn HistogramAddVectorLiteral(mut xself: &mut HistogramLiteral, mut p: &[u8], mut n: usize) {
  (*xself).total_count_ = (*xself).total_count_.wrapping_add(n);
  n = n.wrapping_add(1usize);
  while {
          n = n.wrapping_sub(1 as (usize));
          n
        } != 0 {
    let _rhs = 1;
    let _lhs = &mut (*xself).data_[*{
                       let _old = p;
                       p = p[(1 as (usize))..];
                       _old
                     } as (usize)];
    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
  }
}

fn InitialEntropyCodesLiteral(mut data: &[u8],
                              mut length: usize,
                              mut stride: usize,
                              mut num_histograms: usize,
                              mut histograms: &mut [HistogramLiteral]) {
  let mut seed: u32 = 7u32;
  let mut block_length: usize = length.wrapping_div(num_histograms);
  let mut i: usize;
  ClearHistogramsLiteral(histograms, num_histograms);
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
      HistogramAddVectorLiteral(&mut histograms[(i as (usize))],
                                data[(pos as (usize))..],
                                stride);
    }
    i = i.wrapping_add(1 as (usize));
  }
}

fn RandomSampleLiteral(mut seed: &mut [u32],
                       mut data: &[u8],
                       mut length: usize,
                       mut stride: usize,
                       mut sample: &mut [HistogramLiteral]) {
  let mut pos: usize = 0usize;
  if stride >= length {
    pos = 0usize;
    stride = length;
  } else {
    pos = (MyRand(seed) as (usize)).wrapping_rem(length.wrapping_sub(stride).wrapping_add(1usize));
  }
  HistogramAddVectorLiteral(sample, data[(pos as (usize))..], stride);
}

fn HistogramAddHistogramLiteral(mut xself: &mut HistogramLiteral, mut v: &[HistogramLiteral]) {
  let mut i: usize;
  (*xself).total_count_ = (*xself).total_count_.wrapping_add((*v).total_count_);
  i = 0usize;
  while i < 256usize {
    {
      let _rhs = (*v).data_[i];
      let _lhs = &mut (*xself).data_[i];
      *_lhs = (*_lhs).wrapping_add(_rhs);
    }
    i = i.wrapping_add(1 as (usize));
  }
}

fn RefineEntropyCodesLiteral(mut data: &[u8],
                             mut length: usize,
                             mut stride: usize,
                             mut num_histograms: usize,
                             mut histograms: &mut [HistogramLiteral]) {
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
      let mut sample: HistogramLiteral;
      HistogramClearLiteral(&mut sample);
      RandomSampleLiteral(&mut seed, data, length, stride, &mut sample);
      HistogramAddHistogramLiteral(&mut histograms[(iter.wrapping_rem(num_histograms) as (usize))],
                                   &mut sample);
    }
    iter = iter.wrapping_add(1 as (usize));
  }
}

fn FastLog2(mut v: usize) -> f64 {
  if v < ::std::mem::size_of::<[f32; 256]>().wrapping_div(::std::mem::size_of::<f32>()) {
    return kLog2Table[v] as (f64);
  }
  log2(v as (f64))
}

fn BitCost(mut count: usize) -> f64 {
  if count == 0usize {
    -2.0f64
  } else {
    FastLog2(count)
  }
}

fn FindBlocksLiteral(mut data: &[u8],
                     length: usize,
                     block_switch_bitcost: f64,
                     num_histograms: usize,
                     mut histograms: &[HistogramLiteral],
                     mut insert_cost: &mut [f64],
                     mut cost: &mut [f64],
                     mut switch_signal: &mut [u8],
                     mut block_id: &mut [u8])
                     -> usize {
  let data_size: usize = HistogramDataSizeLiteral();
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
  memset(insert_cost,
         0i32,
         ::std::mem::size_of::<f64>().wrapping_mul(data_size).wrapping_mul(num_histograms));
  i = 0usize;
  while i < num_histograms {
    {
      insert_cost[(i as (usize))] = FastLog2((histograms[(i as (usize))]).total_count_ as (u32) as
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
          insert_cost[(j as (usize))] - BitCost((histograms[(j as (usize))]).data_[i] as (usize));
      }
      j = j.wrapping_add(1 as (usize));
    }
  }
  memset(cost,
         0i32,
         ::std::mem::size_of::<f64>().wrapping_mul(num_histograms));
  memset(switch_signal,
         0i32,
         ::std::mem::size_of::<u8>().wrapping_mul(length).wrapping_mul(bitmaplen));
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

fn RemapBlockIdsLiteral(mut block_ids: &mut [u8],
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

fn HistogramAddLiteral(mut xself: &mut HistogramLiteral, mut val: usize) {
  {
    let _rhs = 1;
    let _lhs = &mut (*xself).data_[val];
    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
  }
  (*xself).total_count_ = (*xself).total_count_.wrapping_add(1 as (usize));
}

fn BuildBlockHistogramsLiteral(mut data: &[u8],
                               length: usize,
                               mut block_ids: &[u8],
                               num_histograms: usize,
                               mut histograms: &mut [HistogramLiteral]) {
  let mut i: usize;
  ClearHistogramsLiteral(histograms, num_histograms);
  i = 0usize;
  while i < length {
    {
      HistogramAddLiteral(&mut histograms[(block_ids[(i as (usize))] as (usize))],
                          data[(i as (usize))] as (usize));
    }
    i = i.wrapping_add(1 as (usize));
  }
}

fn brotli_min_size_t(mut a: usize, mut b: usize) -> usize {
  if a < b { a } else { b }
}



pub struct HistogramPair {
  pub idx1: u32,
  pub idx2: u32,
  pub cost_combo: f64,
  pub cost_diff: f64,
}

fn brotli_max_uint8_t(mut a: u8, mut b: u8) -> u8 {
  (if a as (i32) > b as (i32) {
     a as (i32)
   } else {
     b as (i32)
   }) as (u8)
}

fn ClusterBlocksLiteral(mut m: &mut [MemoryManager],
                        mut data: &[u8],
                        length: usize,
                        num_blocks: usize,
                        mut block_ids: &mut [u8],
                        mut split: &mut [BlockSplit]) {
  let mut histogram_symbols: *mut u32 = if num_blocks != 0 {
    BrotliAllocate(m, num_blocks.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  let mut block_lengths: *mut u32 = if num_blocks != 0 {
    BrotliAllocate(m, num_blocks.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  let expected_num_clusters: usize = (16usize)
    .wrapping_mul(num_blocks.wrapping_add(64usize).wrapping_sub(1usize))
    .wrapping_div(64usize);
  let mut all_histograms_size: usize = 0usize;
  let mut all_histograms_capacity: usize = expected_num_clusters;
  let mut all_histograms: *mut HistogramLiteral = if all_histograms_capacity != 0 {
    BrotliAllocate(m,
                   all_histograms_capacity.wrapping_mul(::std::mem::size_of::<HistogramLiteral>()))
  } else {
    0i32
  };
  let mut cluster_size_size: usize = 0usize;
  let mut cluster_size_capacity: usize = expected_num_clusters;
  let mut cluster_size: *mut u32 = if cluster_size_capacity != 0 {
    BrotliAllocate(m,
                   cluster_size_capacity.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  let mut num_clusters: usize = 0usize;
  let mut histograms: *mut HistogramLiteral = if brotli_min_size_t(num_blocks, 64usize) != 0 {
    BrotliAllocate(m,
                   brotli_min_size_t(num_blocks, 64usize)
                     .wrapping_mul(::std::mem::size_of::<HistogramLiteral>()))
  } else {
    0i32
  };
  let mut max_num_pairs: usize = (64i32 * 64i32 / 2i32) as (usize);
  let mut pairs_capacity: usize = max_num_pairs.wrapping_add(1usize);
  let mut pairs: *mut HistogramPair = if pairs_capacity != 0 {
    BrotliAllocate(m,
                   pairs_capacity.wrapping_mul(::std::mem::size_of::<HistogramPair>()))
  } else {
    0i32
  };
  let mut pos: usize = 0usize;
  let mut clusters: *mut u32;
  let mut num_final_clusters: usize;
  static kInvalidIndex: u32 = !(0u32);
  let mut new_index: *mut u32;
  let mut i: usize;
  let mut sizes: [u32; 64] = [0;64];
  let mut new_clusters: [u32; 64] = [0;64];
  let mut symbols: [u32; 64] = [0;64];
  let mut remap: [u32; 64] = [0;64];
  
  memset(block_lengths,
         0i32,
         num_blocks.wrapping_mul(::std::mem::size_of::<u32>()));
  {
    let mut block_idx: usize = 0usize;
    i = 0usize;
    while i < length {
      {
        0i32;
        {
          let _rhs = 1;
          let _lhs = &mut block_lengths[(block_idx as (usize))];
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
          HistogramClearLiteral(&mut histograms[(j as (usize))]);
          k = 0usize;
          while k < block_lengths[(i.wrapping_add(j) as (usize))] as (usize) {
            {
              HistogramAddLiteral(&mut histograms[(j as (usize))],
                                  data[({
                                     let _old = pos;
                                     pos = pos.wrapping_add(1 as (usize));
                                     _old
                                   } as (usize))] as (usize));
            }
            k = k.wrapping_add(1 as (usize));
          }
          (histograms[(j as (usize))]).bit_cost_ =
            BrotliPopulationCostLiteral(&mut histograms[(j as (usize))]);
          new_clusters[j] = j as (u32);
          symbols[j] = j as (u32);
          sizes[j] = 1u32;
        }
        j = j.wrapping_add(1 as (usize));
      }
      num_new_clusters = BrotliHistogramCombineLiteral(histograms,
                                                       sizes.as_mut_ptr(),
                                                       symbols.as_mut_ptr(),
                                                       new_clusters.as_mut_ptr(),
                                                       pairs,
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
          let mut new_array: *mut HistogramLiteral;
          while _new_size < all_histograms_size.wrapping_add(num_new_clusters) {
            _new_size = _new_size.wrapping_mul(2usize);
          }
          new_array = if _new_size != 0 {
            BrotliAllocate(m,
                           _new_size.wrapping_mul(::std::mem::size_of::<HistogramLiteral>()))
          } else {
            0i32
          };
          if !!(0i32 == 0) && (all_histograms_capacity != 0usize) {
            memcpy(new_array,
                   all_histograms,
                   all_histograms_capacity.wrapping_mul(::std::mem::size_of::<HistogramLiteral>()));
          }
          {
            BrotliFree(m, all_histograms);
            all_histograms = 0i32;
          }
          all_histograms = new_array;
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
          let mut new_array: *mut u32;
          while _new_size < cluster_size_size.wrapping_add(num_new_clusters) {
            _new_size = _new_size.wrapping_mul(2usize);
          }
          new_array = if _new_size != 0 {
            BrotliAllocate(m, _new_size.wrapping_mul(::std::mem::size_of::<u32>()))
          } else {
            0i32
          };
          if !!(0i32 == 0) && (cluster_size_capacity != 0usize) {
            memcpy(new_array,
                   cluster_size,
                   cluster_size_capacity.wrapping_mul(::std::mem::size_of::<u32>()));
          }
          {
            BrotliFree(m, cluster_size);
            cluster_size = 0i32;
          }
          cluster_size = new_array;
          cluster_size_capacity = _new_size;
        }
      }
      if !(0i32 == 0) {
        return;
      }
      j = 0usize;
      while j < num_new_clusters {
        {
          all_histograms[({
             let _old = all_histograms_size;
             all_histograms_size = all_histograms_size.wrapping_add(1 as (usize));
             _old
           } as (usize))] = histograms[(new_clusters[j] as (usize))];
          cluster_size[({
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
          histogram_symbols[(i.wrapping_add(j) as (usize))] =
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
  {
    BrotliFree(m, histograms);
    histograms = 0i32;
  }
  max_num_pairs = brotli_min_size_t((64usize).wrapping_mul(num_clusters),
                                    num_clusters.wrapping_div(2usize).wrapping_mul(num_clusters));
  if pairs_capacity < max_num_pairs.wrapping_add(1usize) {
    {
      BrotliFree(m, pairs);
      pairs = 0i32;
    }
    pairs = if max_num_pairs.wrapping_add(1usize) != 0 {
      BrotliAllocate(m,
                     max_num_pairs.wrapping_add(1usize)
                       .wrapping_mul(::std::mem::size_of::<HistogramPair>()))
    } else {
      0i32
    };
    if !(0i32 == 0) {
      return;
    }
  }
  clusters = if num_clusters != 0 {
    BrotliAllocate(m, num_clusters.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  if !(0i32 == 0) {
    return;
  }
  i = 0usize;
  while i < num_clusters {
    {
      clusters[(i as (usize))] = i as (u32);
    }
    i = i.wrapping_add(1 as (usize));
  }
  num_final_clusters = BrotliHistogramCombineLiteral(all_histograms,
                                                     cluster_size,
                                                     histogram_symbols,
                                                     clusters,
                                                     pairs,
                                                     num_clusters,
                                                     num_blocks,
                                                     256usize,
                                                     max_num_pairs);
  {
    BrotliFree(m, pairs);
    pairs = 0i32;
  }
  {
    BrotliFree(m, cluster_size);
    cluster_size = 0i32;
  }
  new_index = if num_clusters != 0 {
    BrotliAllocate(m, num_clusters.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  if !(0i32 == 0) {
    return;
  }
  i = 0usize;
  while i < num_clusters {
    new_index[(i as (usize))] = kInvalidIndex;
    i = i.wrapping_add(1 as (usize));
  }
  pos = 0usize;
  {
    let mut next_index: u32 = 0u32;
    i = 0usize;
    while i < num_blocks {
      {
        let mut histo: HistogramLiteral;
        let mut j: usize;
        let mut best_out: u32;
        let mut best_bits: f64;
        HistogramClearLiteral(&mut histo);
        j = 0usize;
        while j < block_lengths[(i as (usize))] as (usize) {
          {
            HistogramAddLiteral(&mut histo,
                                data[({
                                   let _old = pos;
                                   pos = pos.wrapping_add(1 as (usize));
                                   _old
                                 } as (usize))] as (usize));
          }
          j = j.wrapping_add(1 as (usize));
        }
        best_out = if i == 0usize {
          histogram_symbols[(0usize)]
        } else {
          histogram_symbols[(i.wrapping_sub(1usize) as (usize))]
        };
        best_bits = BrotliHistogramBitCostDistanceLiteral(&mut histo,
                                                          &mut all_histograms[(best_out as
                                                                (usize))]);
        j = 0usize;
        while j < num_final_clusters {
          {
            let cur_bits: f64 =
              BrotliHistogramBitCostDistanceLiteral(&mut histo,
                                                    &mut all_histograms[(clusters[(j as (usize))] as
                                                          (usize))]);
            if cur_bits < best_bits {
              best_bits = cur_bits;
              best_out = clusters[(j as (usize))];
            }
          }
          j = j.wrapping_add(1 as (usize));
        }
        histogram_symbols[(i as (usize))] = best_out;
        if new_index[(best_out as (usize))] == kInvalidIndex {
          new_index[(best_out as (usize))] = {
            let _old = next_index;
            next_index = next_index.wrapping_add(1 as (u32));
            _old
          };
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
  }
  {
    BrotliFree(m, clusters);
    clusters = 0i32;
  }
  {
    BrotliFree(m, all_histograms);
    all_histograms = 0i32;
  }
  {
    if (*split).types_alloc_size < num_blocks {
      let mut _new_size: usize = if (*split).types_alloc_size == 0usize {
        num_blocks
      } else {
        (*split).types_alloc_size
      };
      let mut new_array: *mut u8;
      while _new_size < num_blocks {
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
    if (*split).lengths_alloc_size < num_blocks {
      let mut _new_size: usize = if (*split).lengths_alloc_size == 0usize {
        num_blocks
      } else {
        (*split).lengths_alloc_size
      };
      let mut new_array: *mut u32;
      while _new_size < num_blocks {
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
  {
    let mut cur_length: u32 = 0u32;
    let mut block_idx: usize = 0usize;
    let mut max_type: u8 = 0i32 as (u8);
    i = 0usize;
    while i < num_blocks {
      {
        cur_length = cur_length.wrapping_add(block_lengths[(i as (usize))]);
        if i.wrapping_add(1usize) == num_blocks ||
           histogram_symbols[(i as (usize))] !=
           histogram_symbols[(i.wrapping_add(1usize) as (usize))] {
          let id: u8 = new_index[(histogram_symbols[(i as (usize))] as (usize))] as (u8);
          *(*split).types[(block_idx as (usize))..] = id;
          *(*split).lengths[(block_idx as (usize))..] = cur_length;
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
  {
    BrotliFree(m, new_index);
    new_index = 0i32;
  }
  {
    BrotliFree(m, block_lengths);
    block_lengths = 0i32;
  }
  {
    BrotliFree(m, histogram_symbols);
    histogram_symbols = 0i32;
  }
}

fn SplitByteVectorLiteral(mut m: &mut [MemoryManager],
                          mut data: &[u8],
                          length: usize,
                          literals_per_histogram: usize,
                          max_histograms: usize,
                          sampling_stride_length: usize,
                          block_switch_cost: f64,
                          mut params: &[BrotliEncoderParams],
                          mut split: &mut [BlockSplit]) {
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
      if (*split).types_alloc_size < (*split).num_blocks.wrapping_add(1usize) {
        let mut _new_size: usize = if (*split).types_alloc_size == 0usize {
          (*split).num_blocks.wrapping_add(1usize)
        } else {
          (*split).types_alloc_size
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

fn HistogramDataSizeCommand() -> usize {
  704usize
}



pub struct HistogramCommand {
  pub data_: [u32; 704],
  pub total_count_: usize,
  pub bit_cost_: f64,
}

fn HistogramClearCommand(mut xself: &mut HistogramCommand) {
  memset((*xself).data_.as_mut_ptr(),
         0i32,
         ::std::mem::size_of::<[u32; 704]>());
  (*xself).total_count_ = 0usize;
  (*xself).bit_cost_ = 3.402e+38f64;
}

fn ClearHistogramsCommand(mut array: &mut [HistogramCommand], mut length: usize) {
  let mut i: usize;
  i = 0usize;
  while i < length {
    HistogramClearCommand(array[(i as (usize))..]);
    i = i.wrapping_add(1 as (usize));
  }
}

fn HistogramAddVectorCommand(mut xself: &mut HistogramCommand, mut p: &[u16], mut n: usize) {
  (*xself).total_count_ = (*xself).total_count_.wrapping_add(n);
  n = n.wrapping_add(1usize);
  while {
          n = n.wrapping_sub(1 as (usize));
          n
        } != 0 {
    let _rhs = 1;
    let _lhs = &mut (*xself).data_[*{
                       let _old = p;
                       p = p[(1 as (usize))..];
                       _old
                     } as (usize)];
    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
  }
}

fn InitialEntropyCodesCommand(mut data: &[u16],
                              mut length: usize,
                              mut stride: usize,
                              mut num_histograms: usize,
                              mut histograms: &mut [HistogramCommand]) {
  let mut seed: u32 = 7u32;
  let mut block_length: usize = length.wrapping_div(num_histograms);
  let mut i: usize;
  ClearHistogramsCommand(histograms, num_histograms);
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
      HistogramAddVectorCommand(&mut histograms[(i as (usize))],
                                data[(pos as (usize))..],
                                stride);
    }
    i = i.wrapping_add(1 as (usize));
  }
}

fn RandomSampleCommand(mut seed: &mut [u32],
                       mut data: &[u16],
                       mut length: usize,
                       mut stride: usize,
                       mut sample: &mut [HistogramCommand]) {
  let mut pos: usize = 0usize;
  if stride >= length {
    pos = 0usize;
    stride = length;
  } else {
    pos = (MyRand(seed) as (usize)).wrapping_rem(length.wrapping_sub(stride).wrapping_add(1usize));
  }
  HistogramAddVectorCommand(sample, data[(pos as (usize))..], stride);
}

fn HistogramAddHistogramCommand(mut xself: &mut HistogramCommand, mut v: &[HistogramCommand]) {
  let mut i: usize;
  (*xself).total_count_ = (*xself).total_count_.wrapping_add((*v).total_count_);
  i = 0usize;
  while i < 704usize {
    {
      let _rhs = (*v).data_[i];
      let _lhs = &mut (*xself).data_[i];
      *_lhs = (*_lhs).wrapping_add(_rhs);
    }
    i = i.wrapping_add(1 as (usize));
  }
}

fn RefineEntropyCodesCommand(mut data: &[u16],
                             mut length: usize,
                             mut stride: usize,
                             mut num_histograms: usize,
                             mut histograms: &mut [HistogramCommand]) {
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
      let mut sample: HistogramCommand;
      HistogramClearCommand(&mut sample);
      RandomSampleCommand(&mut seed, data, length, stride, &mut sample);
      HistogramAddHistogramCommand(&mut histograms[(iter.wrapping_rem(num_histograms) as (usize))],
                                   &mut sample);
    }
    iter = iter.wrapping_add(1 as (usize));
  }
}

fn FindBlocksCommand(mut data: &[u16],
                     length: usize,
                     block_switch_bitcost: f64,
                     num_histograms: usize,
                     mut histograms: &[HistogramCommand],
                     mut insert_cost: &mut [f64],
                     mut cost: &mut [f64],
                     mut switch_signal: &mut [u8],
                     mut block_id: &mut [u8])
                     -> usize {
  let data_size: usize = HistogramDataSizeCommand();
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
  memset(insert_cost,
         0i32,
         ::std::mem::size_of::<f64>().wrapping_mul(data_size).wrapping_mul(num_histograms));
  i = 0usize;
  while i < num_histograms {
    {
      insert_cost[(i as (usize))] = FastLog2((histograms[(i as (usize))]).total_count_ as (u32) as
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
          insert_cost[(j as (usize))] - BitCost((histograms[(j as (usize))]).data_[i] as (usize));
      }
      j = j.wrapping_add(1 as (usize));
    }
  }
  memset(cost,
         0i32,
         ::std::mem::size_of::<f64>().wrapping_mul(num_histograms));
  memset(switch_signal,
         0i32,
         ::std::mem::size_of::<u8>().wrapping_mul(length).wrapping_mul(bitmaplen));
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

fn RemapBlockIdsCommand(mut block_ids: &mut [u8],
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

fn HistogramAddCommand(mut xself: &mut HistogramCommand, mut val: usize) {
  {
    let _rhs = 1;
    let _lhs = &mut (*xself).data_[val];
    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
  }
  (*xself).total_count_ = (*xself).total_count_.wrapping_add(1 as (usize));
}

fn BuildBlockHistogramsCommand(mut data: &[u16],
                               length: usize,
                               mut block_ids: &[u8],
                               num_histograms: usize,
                               mut histograms: &mut [HistogramCommand]) {
  let mut i: usize;
  ClearHistogramsCommand(histograms, num_histograms);
  i = 0usize;
  while i < length {
    {
      HistogramAddCommand(&mut histograms[(block_ids[(i as (usize))] as (usize))],
                          data[(i as (usize))] as (usize));
    }
    i = i.wrapping_add(1 as (usize));
  }
}

fn ClusterBlocksCommand(mut m: &mut [MemoryManager],
                        mut data: &[u16],
                        length: usize,
                        num_blocks: usize,
                        mut block_ids: &mut [u8],
                        mut split: &mut [BlockSplit]) {
  let mut histogram_symbols: *mut u32 = if num_blocks != 0 {
    BrotliAllocate(m, num_blocks.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  let mut block_lengths: *mut u32 = if num_blocks != 0 {
    BrotliAllocate(m, num_blocks.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  let expected_num_clusters: usize = (16usize)
    .wrapping_mul(num_blocks.wrapping_add(64usize).wrapping_sub(1usize))
    .wrapping_div(64usize);
  let mut all_histograms_size: usize = 0usize;
  let mut all_histograms_capacity: usize = expected_num_clusters;
  let mut all_histograms: *mut HistogramCommand = if all_histograms_capacity != 0 {
    BrotliAllocate(m,
                   all_histograms_capacity.wrapping_mul(::std::mem::size_of::<HistogramCommand>()))
  } else {
    0i32
  };
  let mut cluster_size_size: usize = 0usize;
  let mut cluster_size_capacity: usize = expected_num_clusters;
  let mut cluster_size: *mut u32 = if cluster_size_capacity != 0 {
    BrotliAllocate(m,
                   cluster_size_capacity.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  let mut num_clusters: usize = 0usize;
  let mut histograms: *mut HistogramCommand = if brotli_min_size_t(num_blocks, 64usize) != 0 {
    BrotliAllocate(m,
                   brotli_min_size_t(num_blocks, 64usize)
                     .wrapping_mul(::std::mem::size_of::<HistogramCommand>()))
  } else {
    0i32
  };
  let mut max_num_pairs: usize = (64i32 * 64i32 / 2i32) as (usize);
  let mut pairs_capacity: usize = max_num_pairs.wrapping_add(1usize);
  let mut pairs: *mut HistogramPair = if pairs_capacity != 0 {
    BrotliAllocate(m,
                   pairs_capacity.wrapping_mul(::std::mem::size_of::<HistogramPair>()))
  } else {
    0i32
  };
  let mut pos: usize = 0usize;
  let mut clusters: *mut u32;
  let mut num_final_clusters: usize;
  static kInvalidIndex: u32 = !(0u32);
  let mut new_index: *mut u32;
  let mut i: usize;
  let mut sizes: [u32; 64] = [0; 64];
  let mut new_clusters: [u32; 64] = [0; 64];
  let mut symbols: [u32; 64] = [0u32; 64];
  let mut remap: [u32; 64] = [0u32; 64];
  if !(0i32 == 0) {
    return;
  }
  memset(block_lengths,
         0i32,
         num_blocks.wrapping_mul(::std::mem::size_of::<u32>()));
  {
    let mut block_idx: usize = 0usize;
    i = 0usize;
    while i < length {
      {
        0i32;
        {
          let _rhs = 1;
          let _lhs = &mut block_lengths[(block_idx as (usize))];
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
          HistogramClearCommand(&mut histograms[(j as (usize))]);
          k = 0usize;
          while k < block_lengths[(i.wrapping_add(j) as (usize))] as (usize) {
            {
              HistogramAddCommand(&mut histograms[(j as (usize))],
                                  data[({
                                     let _old = pos;
                                     pos = pos.wrapping_add(1 as (usize));
                                     _old
                                   } as (usize))] as (usize));
            }
            k = k.wrapping_add(1 as (usize));
          }
          (histograms[(j as (usize))]).bit_cost_ =
            BrotliPopulationCostCommand(&mut histograms[(j as (usize))]);
          new_clusters[j] = j as (u32);
          symbols[j] = j as (u32);
          sizes[j] = 1u32;
        }
        j = j.wrapping_add(1 as (usize));
      }
      num_new_clusters = BrotliHistogramCombineCommand(histograms,
                                                       sizes.as_mut_ptr(),
                                                       symbols.as_mut_ptr(),
                                                       new_clusters.as_mut_ptr(),
                                                       pairs,
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
          let mut new_array: *mut HistogramCommand;
          while _new_size < all_histograms_size.wrapping_add(num_new_clusters) {
            _new_size = _new_size.wrapping_mul(2usize);
          }
          new_array = if _new_size != 0 {
            BrotliAllocate(m,
                           _new_size.wrapping_mul(::std::mem::size_of::<HistogramCommand>()))
          } else {
            0i32
          };
          if !!(0i32 == 0) && (all_histograms_capacity != 0usize) {
            memcpy(new_array,
                   all_histograms,
                   all_histograms_capacity.wrapping_mul(::std::mem::size_of::<HistogramCommand>()));
          }
          {
            BrotliFree(m, all_histograms);
            all_histograms = 0i32;
          }
          all_histograms = new_array;
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
          let mut new_array: *mut u32;
          while _new_size < cluster_size_size.wrapping_add(num_new_clusters) {
            _new_size = _new_size.wrapping_mul(2usize);
          }
          new_array = if _new_size != 0 {
            BrotliAllocate(m, _new_size.wrapping_mul(::std::mem::size_of::<u32>()))
          } else {
            0i32
          };
          if !!(0i32 == 0) && (cluster_size_capacity != 0usize) {
            memcpy(new_array,
                   cluster_size,
                   cluster_size_capacity.wrapping_mul(::std::mem::size_of::<u32>()));
          }
          {
            BrotliFree(m, cluster_size);
            cluster_size = 0i32;
          }
          cluster_size = new_array;
          cluster_size_capacity = _new_size;
        }
      }
      if !(0i32 == 0) {
        return;
      }
      j = 0usize;
      while j < num_new_clusters {
        {
          all_histograms[({
             let _old = all_histograms_size;
             all_histograms_size = all_histograms_size.wrapping_add(1 as (usize));
             _old
           } as (usize))] = histograms[(new_clusters[j] as (usize))];
          cluster_size[({
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
          histogram_symbols[(i.wrapping_add(j) as (usize))] =
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
  {
    BrotliFree(m, histograms);
    histograms = 0i32;
  }
  max_num_pairs = brotli_min_size_t((64usize).wrapping_mul(num_clusters),
                                    num_clusters.wrapping_div(2usize).wrapping_mul(num_clusters));
  if pairs_capacity < max_num_pairs.wrapping_add(1usize) {
    {
      BrotliFree(m, pairs);
      pairs = 0i32;
    }
    pairs = if max_num_pairs.wrapping_add(1usize) != 0 {
      BrotliAllocate(m,
                     max_num_pairs.wrapping_add(1usize)
                       .wrapping_mul(::std::mem::size_of::<HistogramPair>()))
    } else {
      0i32
    };
    if !(0i32 == 0) {
      return;
    }
  }
  clusters = if num_clusters != 0 {
    BrotliAllocate(m, num_clusters.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  if !(0i32 == 0) {
    return;
  }
  i = 0usize;
  while i < num_clusters {
    {
      clusters[(i as (usize))] = i as (u32);
    }
    i = i.wrapping_add(1 as (usize));
  }
  num_final_clusters = BrotliHistogramCombineCommand(all_histograms,
                                                     cluster_size,
                                                     histogram_symbols,
                                                     clusters,
                                                     pairs,
                                                     num_clusters,
                                                     num_blocks,
                                                     256usize,
                                                     max_num_pairs);
  {
    BrotliFree(m, pairs);
    pairs = 0i32;
  }
  {
    BrotliFree(m, cluster_size);
    cluster_size = 0i32;
  }
  new_index = if num_clusters != 0 {
    BrotliAllocate(m, num_clusters.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  if !(0i32 == 0) {
    return;
  }
  i = 0usize;
  while i < num_clusters {
    new_index[(i as (usize))] = kInvalidIndex;
    i = i.wrapping_add(1 as (usize));
  }
  pos = 0usize;
  {
    let mut next_index: u32 = 0u32;
    i = 0usize;
    while i < num_blocks {
      {
        let mut histo: HistogramCommand;
        let mut j: usize;
        let mut best_out: u32;
        let mut best_bits: f64;
        HistogramClearCommand(&mut histo);
        j = 0usize;
        while j < block_lengths[(i as (usize))] as (usize) {
          {
            HistogramAddCommand(&mut histo,
                                data[({
                                   let _old = pos;
                                   pos = pos.wrapping_add(1 as (usize));
                                   _old
                                 } as (usize))] as (usize));
          }
          j = j.wrapping_add(1 as (usize));
        }
        best_out = if i == 0usize {
          histogram_symbols[(0usize)]
        } else {
          histogram_symbols[(i.wrapping_sub(1usize) as (usize))]
        };
        best_bits = BrotliHistogramBitCostDistanceCommand(&mut histo,
                                                          &mut all_histograms[(best_out as
                                                                (usize))]);
        j = 0usize;
        while j < num_final_clusters {
          {
            let cur_bits: f64 =
              BrotliHistogramBitCostDistanceCommand(&mut histo,
                                                    &mut all_histograms[(clusters[(j as (usize))] as
                                                          (usize))]);
            if cur_bits < best_bits {
              best_bits = cur_bits;
              best_out = clusters[(j as (usize))];
            }
          }
          j = j.wrapping_add(1 as (usize));
        }
        histogram_symbols[(i as (usize))] = best_out;
        if new_index[(best_out as (usize))] == kInvalidIndex {
          new_index[(best_out as (usize))] = {
            let _old = next_index;
            next_index = next_index.wrapping_add(1 as (u32));
            _old
          };
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
  }
  {
    BrotliFree(m, clusters);
    clusters = 0i32;
  }
  {
    BrotliFree(m, all_histograms);
    all_histograms = 0i32;
  }
  {
    if (*split).types_alloc_size < num_blocks {
      let mut _new_size: usize = if (*split).types_alloc_size == 0usize {
        num_blocks
      } else {
        (*split).types_alloc_size
      };
      let mut new_array: *mut u8;
      while _new_size < num_blocks {
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
    if (*split).lengths_alloc_size < num_blocks {
      let mut _new_size: usize = if (*split).lengths_alloc_size == 0usize {
        num_blocks
      } else {
        (*split).lengths_alloc_size
      };
      let mut new_array: *mut u32;
      while _new_size < num_blocks {
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
  {
    let mut cur_length: u32 = 0u32;
    let mut block_idx: usize = 0usize;
    let mut max_type: u8 = 0i32 as (u8);
    i = 0usize;
    while i < num_blocks {
      {
        cur_length = cur_length.wrapping_add(block_lengths[(i as (usize))]);
        if i.wrapping_add(1usize) == num_blocks ||
           histogram_symbols[(i as (usize))] !=
           histogram_symbols[(i.wrapping_add(1usize) as (usize))] {
          let id: u8 = new_index[(histogram_symbols[(i as (usize))] as (usize))] as (u8);
          *(*split).types[(block_idx as (usize))..] = id;
          *(*split).lengths[(block_idx as (usize))..] = cur_length;
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
  {
    BrotliFree(m, new_index);
    new_index = 0i32;
  }
  {
    BrotliFree(m, block_lengths);
    block_lengths = 0i32;
  }
  {
    BrotliFree(m, histogram_symbols);
    histogram_symbols = 0i32;
  }
}

fn SplitByteVectorCommand(mut m: &mut [MemoryManager],
                          mut data: &[u16],
                          length: usize,
                          literals_per_histogram: usize,
                          max_histograms: usize,
                          sampling_stride_length: usize,
                          block_switch_cost: f64,
                          mut params: &[BrotliEncoderParams],
                          mut split: &mut [BlockSplit]) {
  let data_size: usize = HistogramDataSizeCommand();
  let mut num_histograms: usize = length.wrapping_div(literals_per_histogram).wrapping_add(1usize);
  let mut histograms: *mut HistogramCommand;
  if num_histograms > max_histograms {
    num_histograms = max_histograms;
  }
  if length == 0usize {
    (*split).num_types = 1usize;
    return;
  } else if length < kMinLengthForBlockSplitting {
    {
      if (*split).types_alloc_size < (*split).num_blocks.wrapping_add(1usize) {
        let mut _new_size: usize = if (*split).types_alloc_size == 0usize {
          (*split).num_blocks.wrapping_add(1usize)
        } else {
          (*split).types_alloc_size
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
                   num_histograms.wrapping_mul(::std::mem::size_of::<HistogramCommand>()))
  } else {
    0i32
  };
  if !(0i32 == 0) {
    return;
  }
  InitialEntropyCodesCommand(data,
                             length,
                             sampling_stride_length,
                             num_histograms,
                             histograms);
  RefineEntropyCodesCommand(data,
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
        num_blocks = FindBlocksCommand(data,
                                       length,
                                       block_switch_cost,
                                       num_histograms,
                                       histograms,
                                       insert_cost,
                                       cost,
                                       switch_signal,
                                       block_ids);
        num_histograms = RemapBlockIdsCommand(block_ids, length, new_id, num_histograms);
        BuildBlockHistogramsCommand(data, length, block_ids, num_histograms, histograms);
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
    ClusterBlocksCommand(m, data, length, num_blocks, block_ids, split);
    if !(0i32 == 0) {
      return;
    }
    {
      BrotliFree(m, block_ids);
      block_ids = 0i32;
    }
  }
}

fn HistogramDataSizeDistance() -> usize {
  520usize
}



pub struct HistogramDistance {
  pub data_: [u32; 520],
  pub total_count_: usize,
  pub bit_cost_: f64,
}

fn HistogramClearDistance(mut xself: &mut HistogramDistance) {
  memset((*xself).data_.as_mut_ptr(),
         0i32,
         ::std::mem::size_of::<[u32; 520]>());
  (*xself).total_count_ = 0usize;
  (*xself).bit_cost_ = 3.402e+38f64;
}

fn ClearHistogramsDistance(mut array: &mut [HistogramDistance], mut length: usize) {
  let mut i: usize;
  i = 0usize;
  while i < length {
    HistogramClearDistance(array[(i as (usize))..]);
    i = i.wrapping_add(1 as (usize));
  }
}

fn HistogramAddVectorDistance(mut xself: &mut HistogramDistance, mut p: &[u16], mut n: usize) {
  (*xself).total_count_ = (*xself).total_count_.wrapping_add(n);
  n = n.wrapping_add(1usize);
  while {
          n = n.wrapping_sub(1 as (usize));
          n
        } != 0 {
    let _rhs = 1;
    let _lhs = &mut (*xself).data_[*{
                       let _old = p;
                       p = p[(1 as (usize))..];
                       _old
                     } as (usize)];
    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
  }
}

fn InitialEntropyCodesDistance(mut data: &[u16],
                               mut length: usize,
                               mut stride: usize,
                               mut num_histograms: usize,
                               mut histograms: &mut [HistogramDistance]) {
  let mut seed: u32 = 7u32;
  let mut block_length: usize = length.wrapping_div(num_histograms);
  let mut i: usize;
  ClearHistogramsDistance(histograms, num_histograms);
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
      HistogramAddVectorDistance(&mut histograms[(i as (usize))],
                                 data[(pos as (usize))..],
                                 stride);
    }
    i = i.wrapping_add(1 as (usize));
  }
}

fn RandomSampleDistance(mut seed: &mut [u32],
                        mut data: &[u16],
                        mut length: usize,
                        mut stride: usize,
                        mut sample: &mut [HistogramDistance]) {
  let mut pos: usize = 0usize;
  if stride >= length {
    pos = 0usize;
    stride = length;
  } else {
    pos = (MyRand(seed) as (usize)).wrapping_rem(length.wrapping_sub(stride).wrapping_add(1usize));
  }
  HistogramAddVectorDistance(sample, data[(pos as (usize))..], stride);
}

fn HistogramAddHistogramDistance(mut xself: &mut HistogramDistance, mut v: &[HistogramDistance]) {
  let mut i: usize;
  (*xself).total_count_ = (*xself).total_count_.wrapping_add((*v).total_count_);
  i = 0usize;
  while i < 520usize {
    {
      let _rhs = (*v).data_[i];
      let _lhs = &mut (*xself).data_[i];
      *_lhs = (*_lhs).wrapping_add(_rhs);
    }
    i = i.wrapping_add(1 as (usize));
  }
}

fn RefineEntropyCodesDistance(mut data: &[u16],
                              mut length: usize,
                              mut stride: usize,
                              mut num_histograms: usize,
                              mut histograms: &mut [HistogramDistance]) {
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
      let mut sample: HistogramDistance;
      HistogramClearDistance(&mut sample);
      RandomSampleDistance(&mut seed, data, length, stride, &mut sample);
      HistogramAddHistogramDistance(&mut histograms[(iter.wrapping_rem(num_histograms) as
                                          (usize))],
                                    &mut sample);
    }
    iter = iter.wrapping_add(1 as (usize));
  }
}

fn FindBlocksDistance(mut data: &[u16],
                      length: usize,
                      block_switch_bitcost: f64,
                      num_histograms: usize,
                      mut histograms: &[HistogramDistance],
                      mut insert_cost: &mut [f64],
                      mut cost: &mut [f64],
                      mut switch_signal: &mut [u8],
                      mut block_id: &mut [u8])
                      -> usize {
  let data_size: usize = HistogramDataSizeDistance();
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
  memset(insert_cost,
         0i32,
         ::std::mem::size_of::<f64>().wrapping_mul(data_size).wrapping_mul(num_histograms));
  i = 0usize;
  while i < num_histograms {
    {
      insert_cost[(i as (usize))] = FastLog2((histograms[(i as (usize))]).total_count_ as (u32) as
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
          insert_cost[(j as (usize))] - BitCost((histograms[(j as (usize))]).data_[i] as (usize));
      }
      j = j.wrapping_add(1 as (usize));
    }
  }
  memset(cost,
         0i32,
         ::std::mem::size_of::<f64>().wrapping_mul(num_histograms));
  memset(switch_signal,
         0i32,
         ::std::mem::size_of::<u8>().wrapping_mul(length).wrapping_mul(bitmaplen));
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

fn RemapBlockIdsDistance(mut block_ids: &mut [u8],
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

fn HistogramAddDistance(mut xself: &mut HistogramDistance, mut val: usize) {
  {
    let _rhs = 1;
    let _lhs = &mut (*xself).data_[val];
    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
  }
  (*xself).total_count_ = (*xself).total_count_.wrapping_add(1 as (usize));
}

fn BuildBlockHistogramsDistance(mut data: &[u16],
                                length: usize,
                                mut block_ids: &[u8],
                                num_histograms: usize,
                                mut histograms: &mut [HistogramDistance]) {
  let mut i: usize;
  ClearHistogramsDistance(histograms, num_histograms);
  i = 0usize;
  while i < length {
    {
      HistogramAddDistance(&mut histograms[(block_ids[(i as (usize))] as (usize))],
                           data[(i as (usize))] as (usize));
    }
    i = i.wrapping_add(1 as (usize));
  }
}

fn ClusterBlocksDistance(mut m: &mut [MemoryManager],
                         mut data: &[u16],
                         length: usize,
                         num_blocks: usize,
                         mut block_ids: &mut [u8],
                         mut split: &mut [BlockSplit]) {
  let mut histogram_symbols: *mut u32 = if num_blocks != 0 {
    BrotliAllocate(m, num_blocks.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  let mut block_lengths: *mut u32 = if num_blocks != 0 {
    BrotliAllocate(m, num_blocks.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  let expected_num_clusters: usize = (16usize)
    .wrapping_mul(num_blocks.wrapping_add(64usize).wrapping_sub(1usize))
    .wrapping_div(64usize);
  let mut all_histograms_size: usize = 0usize;
  let mut all_histograms_capacity: usize = expected_num_clusters;
  let mut all_histograms: *mut HistogramDistance = if all_histograms_capacity != 0 {
    BrotliAllocate(m,
                   all_histograms_capacity.wrapping_mul(::std::mem::size_of::<HistogramDistance>()))
  } else {
    0i32
  };
  let mut cluster_size_size: usize = 0usize;
  let mut cluster_size_capacity: usize = expected_num_clusters;
  let mut cluster_size: *mut u32 = if cluster_size_capacity != 0 {
    BrotliAllocate(m,
                   cluster_size_capacity.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  let mut num_clusters: usize = 0usize;
  let mut histograms: *mut HistogramDistance = if brotli_min_size_t(num_blocks, 64usize) != 0 {
    BrotliAllocate(m,
                   brotli_min_size_t(num_blocks, 64usize)
                     .wrapping_mul(::std::mem::size_of::<HistogramDistance>()))
  } else {
    0i32
  };
  let mut max_num_pairs: usize = (64i32 * 64i32 / 2i32) as (usize);
  let mut pairs_capacity: usize = max_num_pairs.wrapping_add(1usize);
  let mut pairs: *mut HistogramPair = if pairs_capacity != 0 {
    BrotliAllocate(m,
                   pairs_capacity.wrapping_mul(::std::mem::size_of::<HistogramPair>()))
  } else {
    0i32
  };
  let mut pos: usize = 0usize;
  let mut clusters: *mut u32;
  let mut num_final_clusters: usize;
  static kInvalidIndex: u32 = !(0u32);
  let mut new_index: *mut u32;
  let mut i: usize;
  let mut sizes: [u32; 64] = [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                              0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                              0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                              0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                              0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                              0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
  let mut new_clusters: [u32; 64] =
    [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 0u32];
  let mut symbols: [u32; 64] = [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                                0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                                0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                                0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                                0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                                0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
  let mut remap: [u32; 64] = [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                              0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                              0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                              0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                              0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                              0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
  if !(0i32 == 0) {
    return;
  }
  memset(block_lengths,
         0i32,
         num_blocks.wrapping_mul(::std::mem::size_of::<u32>()));
  {
    let mut block_idx: usize = 0usize;
    i = 0usize;
    while i < length {
      {
        0i32;
        {
          let _rhs = 1;
          let _lhs = &mut block_lengths[(block_idx as (usize))];
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
          HistogramClearDistance(&mut histograms[(j as (usize))]);
          k = 0usize;
          while k < block_lengths[(i.wrapping_add(j) as (usize))] as (usize) {
            {
              HistogramAddDistance(&mut histograms[(j as (usize))],
                                   data[({
                                      let _old = pos;
                                      pos = pos.wrapping_add(1 as (usize));
                                      _old
                                    } as (usize))] as (usize));
            }
            k = k.wrapping_add(1 as (usize));
          }
          (histograms[(j as (usize))]).bit_cost_ =
            BrotliPopulationCostDistance(&mut histograms[(j as (usize))]);
          new_clusters[j] = j as (u32);
          symbols[j] = j as (u32);
          sizes[j] = 1u32;
        }
        j = j.wrapping_add(1 as (usize));
      }
      num_new_clusters = BrotliHistogramCombineDistance(histograms,
                                                        sizes.as_mut_ptr(),
                                                        symbols.as_mut_ptr(),
                                                        new_clusters.as_mut_ptr(),
                                                        pairs,
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
          let mut new_array: *mut HistogramDistance;
          while _new_size < all_histograms_size.wrapping_add(num_new_clusters) {
            _new_size = _new_size.wrapping_mul(2usize);
          }
          new_array = if _new_size != 0 {
            BrotliAllocate(m,
                           _new_size.wrapping_mul(::std::mem::size_of::<HistogramDistance>()))
          } else {
            0i32
          };
          if !!(0i32 == 0) && (all_histograms_capacity != 0usize) {
            memcpy(
                            new_array ,
                            all_histograms ,
                            all_histograms_capacity.wrapping_mul(
                                ::std::mem::size_of::<HistogramDistance>()
                            )
                        );
          }
          {
            BrotliFree(m, all_histograms);
            all_histograms = 0i32;
          }
          all_histograms = new_array;
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
          let mut new_array: *mut u32;
          while _new_size < cluster_size_size.wrapping_add(num_new_clusters) {
            _new_size = _new_size.wrapping_mul(2usize);
          }
          new_array = if _new_size != 0 {
            BrotliAllocate(m, _new_size.wrapping_mul(::std::mem::size_of::<u32>()))
          } else {
            0i32
          };
          if !!(0i32 == 0) && (cluster_size_capacity != 0usize) {
            memcpy(new_array,
                   cluster_size,
                   cluster_size_capacity.wrapping_mul(::std::mem::size_of::<u32>()));
          }
          {
            BrotliFree(m, cluster_size);
            cluster_size = 0i32;
          }
          cluster_size = new_array;
          cluster_size_capacity = _new_size;
        }
      }
      if !(0i32 == 0) {
        return;
      }
      j = 0usize;
      while j < num_new_clusters {
        {
          all_histograms[({
             let _old = all_histograms_size;
             all_histograms_size = all_histograms_size.wrapping_add(1 as (usize));
             _old
           } as (usize))] = histograms[(new_clusters[j] as (usize))];
          cluster_size[({
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
          histogram_symbols[(i.wrapping_add(j) as (usize))] =
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
  {
    BrotliFree(m, histograms);
    histograms = 0i32;
  }
  max_num_pairs = brotli_min_size_t((64usize).wrapping_mul(num_clusters),
                                    num_clusters.wrapping_div(2usize).wrapping_mul(num_clusters));
  if pairs_capacity < max_num_pairs.wrapping_add(1usize) {
    {
      BrotliFree(m, pairs);
      pairs = 0i32;
    }
    pairs = if max_num_pairs.wrapping_add(1usize) != 0 {
      BrotliAllocate(m,
                     max_num_pairs.wrapping_add(1usize)
                       .wrapping_mul(::std::mem::size_of::<HistogramPair>()))
    } else {
      0i32
    };
    if !(0i32 == 0) {
      return;
    }
  }
  clusters = if num_clusters != 0 {
    BrotliAllocate(m, num_clusters.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  if !(0i32 == 0) {
    return;
  }
  i = 0usize;
  while i < num_clusters {
    {
      clusters[(i as (usize))] = i as (u32);
    }
    i = i.wrapping_add(1 as (usize));
  }
  num_final_clusters = BrotliHistogramCombineDistance(all_histograms,
                                                      cluster_size,
                                                      histogram_symbols,
                                                      clusters,
                                                      pairs,
                                                      num_clusters,
                                                      num_blocks,
                                                      256usize,
                                                      max_num_pairs);
  {
    BrotliFree(m, pairs);
    pairs = 0i32;
  }
  {
    BrotliFree(m, cluster_size);
    cluster_size = 0i32;
  }
  new_index = if num_clusters != 0 {
    BrotliAllocate(m, num_clusters.wrapping_mul(::std::mem::size_of::<u32>()))
  } else {
    0i32
  };
  if !(0i32 == 0) {
    return;
  }
  i = 0usize;
  while i < num_clusters {
    new_index[(i as (usize))] = kInvalidIndex;
    i = i.wrapping_add(1 as (usize));
  }
  pos = 0usize;
  {
    let mut next_index: u32 = 0u32;
    i = 0usize;
    while i < num_blocks {
      {
        let mut histo: HistogramDistance;
        let mut j: usize;
        let mut best_out: u32;
        let mut best_bits: f64;
        HistogramClearDistance(&mut histo);
        j = 0usize;
        while j < block_lengths[(i as (usize))] as (usize) {
          {
            HistogramAddDistance(&mut histo,
                                 data[({
                                    let _old = pos;
                                    pos = pos.wrapping_add(1 as (usize));
                                    _old
                                  } as (usize))] as (usize));
          }
          j = j.wrapping_add(1 as (usize));
        }
        best_out = if i == 0usize {
          histogram_symbols[(0usize)]
        } else {
          histogram_symbols[(i.wrapping_sub(1usize) as (usize))]
        };
        best_bits = BrotliHistogramBitCostDistanceDistance(&mut histo,
                                                           &mut all_histograms[(best_out as
                                                                 (usize))]);
        j = 0usize;
        while j < num_final_clusters {
          {
            let cur_bits: f64 =
              BrotliHistogramBitCostDistanceDistance(&mut histo,
                                                     &mut all_histograms[(clusters[(j as (usize))] as
                                                           (usize))]);
            if cur_bits < best_bits {
              best_bits = cur_bits;
              best_out = clusters[(j as (usize))];
            }
          }
          j = j.wrapping_add(1 as (usize));
        }
        histogram_symbols[(i as (usize))] = best_out;
        if new_index[(best_out as (usize))] == kInvalidIndex {
          new_index[(best_out as (usize))] = {
            let _old = next_index;
            next_index = next_index.wrapping_add(1 as (u32));
            _old
          };
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
  }
  {
    BrotliFree(m, clusters);
    clusters = 0i32;
  }
  {
    BrotliFree(m, all_histograms);
    all_histograms = 0i32;
  }
  {
    if (*split).types_alloc_size < num_blocks {
      let mut _new_size: usize = if (*split).types_alloc_size == 0usize {
        num_blocks
      } else {
        (*split).types_alloc_size
      };
      let mut new_array: *mut u8;
      while _new_size < num_blocks {
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
    if (*split).lengths_alloc_size < num_blocks {
      let mut _new_size: usize = if (*split).lengths_alloc_size == 0usize {
        num_blocks
      } else {
        (*split).lengths_alloc_size
      };
      let mut new_array: *mut u32;
      while _new_size < num_blocks {
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
  {
    let mut cur_length: u32 = 0u32;
    let mut block_idx: usize = 0usize;
    let mut max_type: u8 = 0i32 as (u8);
    i = 0usize;
    while i < num_blocks {
      {
        cur_length = cur_length.wrapping_add(block_lengths[(i as (usize))]);
        if i.wrapping_add(1usize) == num_blocks ||
           histogram_symbols[(i as (usize))] !=
           histogram_symbols[(i.wrapping_add(1usize) as (usize))] {
          let id: u8 = new_index[(histogram_symbols[(i as (usize))] as (usize))] as (u8);
          *(*split).types[(block_idx as (usize))..] = id;
          *(*split).lengths[(block_idx as (usize))..] = cur_length;
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
  {
    BrotliFree(m, new_index);
    new_index = 0i32;
  }
  {
    BrotliFree(m, block_lengths);
    block_lengths = 0i32;
  }
  {
    BrotliFree(m, histogram_symbols);
    histogram_symbols = 0i32;
  }
}

fn SplitByteVectorDistance(mut m: &mut [MemoryManager],
                           mut data: &[u16],
                           length: usize,
                           literals_per_histogram: usize,
                           max_histograms: usize,
                           sampling_stride_length: usize,
                           block_switch_cost: f64,
                           mut params: &[BrotliEncoderParams],
                           mut split: &mut [BlockSplit]) {
  let data_size: usize = HistogramDataSizeDistance();
  let mut num_histograms: usize = length.wrapping_div(literals_per_histogram).wrapping_add(1usize);
  let mut histograms: *mut HistogramDistance;
  if num_histograms > max_histograms {
    num_histograms = max_histograms;
  }
  if length == 0usize {
    (*split).num_types = 1usize;
    return;
  } else if length < kMinLengthForBlockSplitting {
    {
      if (*split).types_alloc_size < (*split).num_blocks.wrapping_add(1usize) {
        let mut _new_size: usize = if (*split).types_alloc_size == 0usize {
          (*split).num_blocks.wrapping_add(1usize)
        } else {
          (*split).types_alloc_size
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
                   num_histograms.wrapping_mul(::std::mem::size_of::<HistogramDistance>()))
  } else {
    0i32
  };
  if !(0i32 == 0) {
    return;
  }
  InitialEntropyCodesDistance(data,
                              length,
                              sampling_stride_length,
                              num_histograms,
                              histograms);
  RefineEntropyCodesDistance(data,
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
        num_blocks = FindBlocksDistance(data,
                                        length,
                                        block_switch_cost,
                                        num_histograms,
                                        histograms,
                                        insert_cost,
                                        cost,
                                        switch_signal,
                                        block_ids);
        num_histograms = RemapBlockIdsDistance(block_ids, length, new_id, num_histograms);
        BuildBlockHistogramsDistance(data, length, block_ids, num_histograms, histograms);
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
    ClusterBlocksDistance(m, data, length, num_blocks, block_ids, split);
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