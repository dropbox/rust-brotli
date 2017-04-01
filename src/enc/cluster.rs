use super::util::FastLog2;
use core;
use alloc;
use alloc::{SliceWrapper,SliceWrapperMut};
use super::histogram::{CostAccessors, HistogramSelfAddHistogram, HistogramAddHistogram, HistogramClear};
use super::bit_cost::BrotliPopulationCost;
use super::util::brotli_min_size_t;
#[derive(Clone,Copy)]
pub struct HistogramPair {
  pub idx1: u32,
  pub idx2: u32,
  pub cost_combo: f64,
  pub cost_diff: f64,
}

/* Returns entropy reduction of the context map when we combine two clusters. */
fn ClusterCostDiff(mut size_a: usize, mut size_b: usize) -> f64 {
  let mut size_c: usize = size_a.wrapping_add(size_b);
  size_a as (f64) * FastLog2(size_a) + size_b as (f64) * FastLog2(size_b) -
  size_c as (f64) * FastLog2(size_c)
}

fn brotli_max_double(mut a: f64, mut b: f64) -> f64 {
  if a > b { a } else { b }
}


fn HistogramPairIsLess(mut p1: &HistogramPair, p2: &HistogramPair) -> bool {
  if (*p1).cost_diff != (*p2).cost_diff {
    if !!((*p1).cost_diff > (*p2).cost_diff) {
      true
    } else {
      false
    }
  } else if !!((*p1).idx2.wrapping_sub((*p1).idx1) > (*p2).idx2.wrapping_sub((*p2).idx1)) {
    true
  } else {
    false
  }
}

/* Computes the bit cost reduction by combining out[idx1] and out[idx2] and if
   it is below a threshold, stores the pair (idx1, idx2) in the *pairs queue. */
fn BrotliCompareAndPushToQueue<HistogramType:SliceWrapperMut<u32> + SliceWrapper<u32> + CostAccessors>(
    mut out : &mut[HistogramType],
    mut cluster_size : &[u32],
    mut idx1 : u32,
    mut idx2 : u32,
    mut max_num_pairs : usize,
    mut pairs : &mut [HistogramPair],
    mut num_pairs : &mut usize
){
  let mut is_good_pair: i32 = 0i32;
  let mut p : HistogramPair = HistogramPair{idx1:0,idx2:0,cost_combo:0.0,cost_diff:0.0};
  if idx1 == idx2 {
  } else {
    if idx2 < idx1 {
      let mut t: u32 = idx2;
      idx2 = idx1;
      idx1 = t;
    }
    p.idx1 = idx1;
    p.idx2 = idx2;
    p.cost_diff = 0.5f64 *
                  ClusterCostDiff(cluster_size[idx1 as usize] as (usize), cluster_size[idx2 as usize] as (usize));
    p.cost_diff = p.cost_diff - (out[idx1 as (usize)]).bit_cost();
    p.cost_diff = p.cost_diff - (out[idx2 as (usize)]).bit_cost();
    if (out[idx1 as (usize)]).total_count() == 0i32 as (usize) {
      p.cost_combo = (out[idx2 as (usize)]).bit_cost();
      is_good_pair = 1i32;
    } else if (out[idx2 as (usize)]).total_count() == 0i32 as (usize) {
      p.cost_combo = (out[idx1 as (usize)]).bit_cost();
      is_good_pair = 1i32;
    } else {
      let mut threshold: f64 = if *num_pairs == 0i32 as (usize) {
        1e99f64
      } else {
        brotli_max_double(0.0f64, (pairs[0i32 as (usize)]).cost_diff)
      };
      let mut cost_combo: f64;
      HistogramSelfAddHistogram(out, idx1 as usize, idx2 as usize);
      let mut combo: &HistogramType = &out[idx1 as (usize)];
      cost_combo = BrotliPopulationCost(combo);
      if cost_combo < threshold - p.cost_diff {
        p.cost_combo = cost_combo;
        is_good_pair = 1i32;
      }
    }
    if is_good_pair != 0 {
      p.cost_diff = p.cost_diff + p.cost_combo;
      if *num_pairs > 0i32 as (usize) &&
         (HistogramPairIsLess(&pairs[0i32 as (usize)], &p) != false) {
        /* Replace the top of the queue if needed. */
        if *num_pairs < max_num_pairs {
          pairs[*num_pairs as (usize)] = pairs[0i32 as (usize)];
          *num_pairs = (*num_pairs).wrapping_add(1 as (usize));
        }
        pairs[0i32 as (usize)] = p;
      } else if *num_pairs < max_num_pairs {
        pairs[*num_pairs as (usize)] = p;
        *num_pairs = (*num_pairs).wrapping_add(1 as (usize));
      }
    }
  }
}

fn BrotliHistogramCombine<HistogramType:SliceWrapperMut<u32> + SliceWrapper<u32> + CostAccessors>
    (mut out: &mut [HistogramType],
     mut cluster_size: &mut [u32],
     mut symbols: &mut [u32],
     mut clusters: &mut [u32],
     mut pairs: &mut [HistogramPair],
     mut num_clusters: usize,
     mut symbols_size: usize,
     mut max_clusters: usize,
     mut max_num_pairs: usize) -> usize {
  let mut cost_diff_threshold: f64 = 0.0f64;
  let mut min_cluster_size: usize = 1usize;
  let mut num_pairs: usize = 0usize;
  {
    /* We maintain a vector of histogram pairs, with the property that the pair
       with the maximum bit cost reduction is the first. */
    let mut idx1: usize;
    idx1 = 0usize;
    while idx1 < num_clusters {
      {
        let mut idx2: usize;
        idx2 = idx1.wrapping_add(1usize);
        while idx2 < num_clusters {
          {
            BrotliCompareAndPushToQueue(out,
                                               cluster_size,
                                               clusters[(idx1 as (usize))],
                                               clusters[(idx2 as (usize))],
                                               max_num_pairs,
                                               pairs,
                                               &mut num_pairs);
          }
          idx2 = idx2.wrapping_add(1 as (usize));
        }
      }
      idx1 = idx1.wrapping_add(1 as (usize));
    }
  }
  while num_clusters > min_cluster_size {
    let mut best_idx1: u32;
    let mut best_idx2: u32;
    let mut i: usize;
    if (pairs[(0usize)]).cost_diff >= cost_diff_threshold {
      cost_diff_threshold = 1e99f64;
      min_cluster_size = max_clusters;
      {
        {
          continue;
        }
      }
    }
        /* Take the best pair from the top of heap. */
    best_idx1 = (pairs[(0usize)]).idx1;
    best_idx2 = (pairs[(0usize)]).idx2;
    HistogramSelfAddHistogram(&mut out,
                                   (best_idx1 as (usize)),
                                 (best_idx2 as (usize)));
    (out[(best_idx1 as (usize))]).set_bit_cost((pairs[(0usize)]).cost_combo);
    {
      let _rhs = cluster_size[(best_idx2 as (usize))];
      let _lhs = &mut cluster_size[(best_idx1 as (usize))];
      *_lhs = (*_lhs).wrapping_add(_rhs);
    }
    i = 0usize;
    while i < symbols_size {
      {
        if symbols[(i as (usize))] == best_idx2 {
          symbols[(i as (usize))] = best_idx1;
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
    i = 0usize;
    'break9: while i < num_clusters {
      {
        if clusters[(i as (usize))] == best_idx2 {
          for offset in 0..(num_clusters - i - 1) {
              clusters[i + offset] = clusters[i + 1 + offset];
          }
          break 'break9;
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
    num_clusters = num_clusters.wrapping_sub(1 as (usize));
    {
      /* Remove pairs intersecting the just combined best pair. */
      let mut copy_to_idx: usize = 0usize;
      i = 0usize;
      while i < num_pairs {
        'continue12: loop {
          {
            let mut p: HistogramPair = pairs[(i as (usize))];
            if (p).idx1 == best_idx1 || (p).idx2 == best_idx1 || (p).idx1 == best_idx2 ||
               (p).idx2 == best_idx2 {
              /* Remove invalid pair from the queue. */
              {
                break 'continue12;
              }
            }
            if HistogramPairIsLess(&pairs[(0usize)], &p) != false {
              /* Replace the top of the queue if needed. */
              let mut front: HistogramPair = pairs[(0usize)];
              pairs[(0usize)] = p;
              pairs[(copy_to_idx as (usize))] = front;
            } else {
              pairs[(copy_to_idx as (usize))] = p;
            }
            copy_to_idx = copy_to_idx.wrapping_add(1 as (usize));
          }
          break;
        }
        i = i.wrapping_add(1 as (usize));
      }
      num_pairs = copy_to_idx;
    }
    i = 0usize;
        /* Push new pairs formed with the combined histogram to the heap. */
    while i < num_clusters {
      {
        BrotliCompareAndPushToQueue(out,
                                           cluster_size,
                                           best_idx1,
                                           clusters[(i as (usize))],
                                           max_num_pairs,
                                           &mut pairs,
                                           &mut num_pairs);
      }
      i = i.wrapping_add(1 as (usize));
    }
  }
  num_clusters
}

/* What is the bit cost of moving histogram from cur_symbol to candidate. */
pub fn BrotliHistogramBitCostDistance<HistogramType:SliceWrapperMut<u32> + SliceWrapper<u32> + CostAccessors + Clone>
                                             (mut histogram: &HistogramType,
                                             mut candidate: &HistogramType)
                                             -> f64 {
  if (*histogram).total_count() == 0usize {
    0.0f64
  } else {
    let mut tmp: HistogramType = histogram.clone();
    HistogramAddHistogram(&mut tmp, candidate);
    BrotliPopulationCost(&tmp) - (*candidate).bit_cost()
  }
}

/* Find the best 'out' histogram for each of the 'in' histograms.
   When called, clusters[0..num_clusters) contains the unique values from
   symbols[0..in_size), but this property is not preserved in this function.
   Note: we assume that out[]->bit_cost_ is already up-to-date. */

pub fn BrotliHistogramRemap<HistogramType:SliceWrapperMut<u32> + SliceWrapper<u32> + CostAccessors + Clone>
                                  (mut inp: &[HistogramType],
                                   mut in_size: usize,
                                   mut clusters: &[u32],
                                   mut num_clusters: usize,
                                   mut out: &mut [HistogramType],
                                   mut symbols: &mut [u32]) {
  let mut i: usize;
  i = 0usize;
  while i < in_size {
    {
      let mut best_out: u32 = if i == 0usize {
        symbols[(0usize)]
      } else {
        symbols[(i.wrapping_sub(1usize) as (usize))]
      };
      let mut best_bits: f64 = BrotliHistogramBitCostDistance(&inp[(i as (usize))],
                                                                     &mut out[(best_out as
                                                                           (usize))]);
      let mut j: usize;
      j = 0usize;
      while j < num_clusters {
        {
          let cur_bits: f64 =
            BrotliHistogramBitCostDistance(&inp[(i as (usize))],
                                           &mut out[(clusters[(j as (usize))] as (usize))]);
          if cur_bits < best_bits {
            best_bits = cur_bits;
            best_out = clusters[(j as (usize))];
          }
        }
        j = j.wrapping_add(1 as (usize));
      }
      symbols[(i as (usize))] = best_out;
    }
    i = i.wrapping_add(1 as (usize));
  }
  i = 0usize;
  /* Recompute each out based on raw and symbols. */
  while i < num_clusters {
    {
      HistogramClear(&mut out[(clusters[(i as (usize))] as (usize))]);
    }
    i = i.wrapping_add(1 as (usize));
  }
  i = 0usize;
  while i < in_size {
    {
      HistogramAddHistogram(&mut out[(symbols[(i as (usize))] as (usize))],
                                   &inp[(i as (usize))]);
    }
    i = i.wrapping_add(1 as (usize));
  }
}



/* Reorders elements of the out[0..length) array and changes values in
   symbols[0..length) array in the following way:
     * when called, symbols[] contains indexes into out[], and has N unique
       values (possibly N < length)
     * on return, symbols'[i] = f(symbols[i]) and
                  out'[symbols'[i]] = out[symbols[i]], for each 0 <= i < length,
       where f is a bijection between the range of symbols[] and [0..N), and
       the first occurrences of values in symbols'[i] come in consecutive
       increasing order.
   Returns N, the number of unique values in symbols[]. */
pub fn BrotliHistogramReindex<HistogramType:SliceWrapperMut<u32> + SliceWrapper<u32> + CostAccessors+Clone,
                            AllocU32:alloc::Allocator<u32>,
                            AllocH:alloc::Allocator<HistogramType> >
                            (mut m: &mut AllocU32,
                             mut mh: &mut AllocH,
                                     mut out: &mut [HistogramType],
                                     mut symbols: &mut [u32],
                                     mut length: usize)
                                     -> usize {
  static kInvalidIndex: u32 = !(0u32);
  let mut new_index: AllocU32::AllocatedMemory = if length != 0 {
    m.alloc_cell(length)
  } else {
    AllocU32::AllocatedMemory::default()
  };
  let mut next_index: u32;
  let mut tmp: AllocH::AllocatedMemory;
  let mut i: usize;
  i = 0usize;
  while i < length {
    {
      new_index.slice_mut()[(i as (usize))] = kInvalidIndex;
    }
    i = i.wrapping_add(1 as (usize));
  }
  next_index = 0u32;
  i = 0usize;
  while i < length {
    {
      if new_index.slice()[(symbols[(i as (usize))] as (usize))] == kInvalidIndex {
        new_index.slice_mut()[(symbols[(i as (usize))] as (usize))] = next_index;
        next_index = next_index.wrapping_add(1 as (u32));
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
    /* TODO: by using idea of "cycle-sort" we can avoid allocation of
     tmp and reduce the number of copying by the factor of 2. */
  tmp = if next_index != 0 {
    mh.alloc_cell(next_index as usize)
  } else {
    AllocH::AllocatedMemory::default()
  };
  next_index = 0u32;
  i = 0usize;
  while i < length {
    {
      if new_index.slice()[(symbols[(i as (usize))] as (usize))] == next_index {
        tmp.slice_mut()[(next_index as (usize))] = out[(symbols[(i as (usize))] as (usize))].clone();
        next_index = next_index.wrapping_add(1 as (u32));
      }
      symbols[(i as (usize))] = new_index.slice()[(symbols[(i as (usize))] as (usize))];
    }
    i = i.wrapping_add(1 as (usize));
  }
  {
    m.free_cell(new_index);
  }
  i = 0usize;
  while i < next_index as (usize) {
    {
      out[(i as (usize))] = tmp.slice()[(i as (usize))].clone();
    }
    i = i.wrapping_add(1 as (usize));
  }
  {
    mh.free_cell(tmp)
  }
  next_index as (usize)
}

pub fn BrotliClusterHistogramsLiteral<HistogramType:SliceWrapperMut<u32> + SliceWrapper<u32> + CostAccessors+Clone,
                                      AllocU32:alloc::Allocator<u32>,
                                      AllocHP:alloc::Allocator<HistogramPair>,
                                      AllocH:alloc::Allocator<HistogramType> >
                                     (mut m32: &mut AllocU32,
                                      mut mhp: &mut AllocHP,
                                      mut mh: &mut AllocH,
                                      mut inp: &[HistogramType],
                                      in_size: usize,
                                      mut max_histograms: usize,
                                      mut out: &mut [HistogramType],
                                      mut out_size: &mut usize,
                                      mut histogram_symbols: &mut [u32]) {
  let mut cluster_size = if in_size != 0 {
    m32.alloc_cell(in_size)
  } else {
    AllocU32::AllocatedMemory::default()
  };
  let mut clusters = if in_size != 0 {
    m32.alloc_cell(in_size)
  } else {
    AllocU32::AllocatedMemory::default()
  };
  let mut num_clusters: usize = 0usize;
  let max_input_histograms: usize = 64usize;
  let mut pairs_capacity: usize = max_input_histograms.wrapping_mul(max_input_histograms)
    .wrapping_div(2usize);
  let mut pairs = if pairs_capacity.wrapping_add(1usize) != 0 {
    mhp.alloc_cell(pairs_capacity.wrapping_add(1usize))
  } else {
    AllocHP::AllocatedMemory::default()
  };
  let mut i: usize;
  i = 0usize;
  while i < in_size {
    {
      cluster_size.slice_mut()[(i as (usize))] = 1u32;
    }
    i = i.wrapping_add(1 as (usize));
  }
  i = 0usize;
  while i < in_size {
    {
      out[(i as (usize))] = inp[(i as (usize))].clone();
      (out[(i as (usize))]).set_bit_cost(BrotliPopulationCost(&inp[(i as (usize))]));
      histogram_symbols[(i as (usize))] = i as (u32);
    }
    i = i.wrapping_add(1 as (usize));
  }
  i = 0usize;
  while i < in_size {
    {
      let mut num_to_combine: usize = brotli_min_size_t(in_size.wrapping_sub(i),
                                                        max_input_histograms);
      let mut num_new_clusters: usize;
      let mut j: usize;
      j = 0usize;
      while j < num_to_combine {
        {
          clusters.slice_mut()[(num_clusters.wrapping_add(j) as (usize))] = i.wrapping_add(j) as (u32);
        }
        j = j.wrapping_add(1 as (usize));
      }
      num_new_clusters = BrotliHistogramCombine(out,
                                                       cluster_size.slice_mut(),
                                                       &mut histogram_symbols[(i as (usize))..],
                                                       &mut clusters.slice_mut()[(num_clusters as (usize))..],
                                                       pairs.slice_mut(),
                                                       num_to_combine,
                                                       num_to_combine,
                                                       max_histograms,
                                                       pairs_capacity);
      num_clusters = num_clusters.wrapping_add(num_new_clusters);
    }
    i = i.wrapping_add(max_input_histograms);
  }
  {
    let mut max_num_pairs: usize = brotli_min_size_t((64usize).wrapping_mul(num_clusters),
                                                     num_clusters.wrapping_div(2usize)
                                                       .wrapping_mul(num_clusters));
    {
      if pairs_capacity < max_num_pairs.wrapping_add(1usize) {
        let mut _new_size: usize = if pairs_capacity == 0usize {
          max_num_pairs.wrapping_add(1usize)
        } else {
          pairs_capacity
        };
        let mut new_array: AllocHP::AllocatedMemory;
        while _new_size < max_num_pairs.wrapping_add(1usize) {
          _new_size = _new_size.wrapping_mul(2usize);
        }
        new_array = if _new_size != 0 {
          mhp.alloc_cell(_new_size)
        } else {
          AllocHP::AllocatedMemory::default()
        };
        new_array.slice_mut()[..pairs_capacity].clone_from_slice(&pairs.slice()[..pairs_capacity]);
        mhp.free_cell(core::mem::replace(&mut pairs, new_array));
        pairs_capacity = _new_size;
      }
    }
    num_clusters = BrotliHistogramCombine(out,
                                                 cluster_size.slice_mut(),
                                                 histogram_symbols,
                                                 clusters.slice_mut(),
                                                 pairs.slice_mut(),
                                                 num_clusters,
                                                 in_size,
                                                 max_histograms,
                                                 max_num_pairs);
  }
  mhp.free_cell(pairs);
  m32.free_cell(cluster_size);
  BrotliHistogramRemap(inp, in_size, clusters.slice(), num_clusters, out, histogram_symbols);
  m32.free_cell(clusters);
  *out_size = BrotliHistogramReindex(m32, mh, out, histogram_symbols, in_size);
}



/////////// DONE //////////////////////////
