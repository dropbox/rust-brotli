use super::util::FastLog2;
use alloc::{SliceWrapper,SliceWrapperMut};
use super::histogram::{CostAccessors, HistogramSelfAddHistogram};
use super::bit_cost::BrotliPopulationCost;

#[derive(Clone,Copy)]
pub struct HistogramPair {
  pub idx1: u32,
  pub idx2: u32,
  pub cost_combo: f64,
  pub cost_diff: f64,
}

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
      let mut copy_to_idx: usize = 0usize;
      i = 0usize;
      while i < num_pairs {
        'continue12: loop {
          {
            let mut p: HistogramPair = pairs[(i as (usize))];
            if (p).idx1 == best_idx1 || (p).idx2 == best_idx1 || (p).idx1 == best_idx2 ||
               (p).idx2 == best_idx2 {
              {
                break 'continue12;
              }
            }
            if HistogramPairIsLess(&pairs[(0usize)], &p) != false {
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
/*
#[no_mangle]
pub unsafe extern fn BrotliHistogramBitCostDistanceLiteral(
    mut histogram : *const HistogramLiteral,
    mut candidate : *const HistogramLiteral
) -> f64{
  if (*histogram).total_count_ == 0i32 as (usize) {
    0.0f64
  } else {
    let mut tmp: HistogramLiteral = *histogram;
    HistogramAddHistogramLiteral(&mut tmp as (*mut HistogramLiteral), candidate);
    BrotliPopulationCostLiteral(&mut tmp as (*mut HistogramLiteral) as (*const HistogramLiteral)) -
    (*candidate).bit_cost_
  }
}

unsafe extern "C" fn HistogramClearLiteral(mut xself: *mut HistogramLiteral) {
  memset((*xself).data_.as_mut_ptr() as (*mut ::std::os::raw::c_void),
         0i32,
         ::std::mem::size_of::<[u32; 256]>());
  (*xself).total_count_ = 0i32 as (usize);
  (*xself).bit_cost_ = 3.402e+38f64;
}

#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramRemapLiteral(mut in_: *const HistogramLiteral,
                                                     mut in_size: usize,
                                                     mut clusters: *const u32,
                                                     mut num_clusters: usize,
                                                     mut out: *mut HistogramLiteral,
                                                     mut symbols: *mut u32) {
  let mut i: usize;
  i = 0i32 as (usize);
  'loop1: loop {
    if i < in_size {
      let mut best_out: u32 = if i == 0i32 as (usize) {
        *symbols.offset(0i32 as (isize))
      } else {
        *symbols.offset(i.wrapping_sub(1i32 as (usize)) as (isize))
      };
      let mut best_bits: f64 = BrotliHistogramBitCostDistanceLiteral(&*in_.offset(i as (isize)) as
                                                                     (*const HistogramLiteral),
                                                                     &mut *out.offset(best_out as
                                                                                      (isize)) as
                                                                     (*mut HistogramLiteral) as
                                                                     (*const HistogramLiteral));
      let mut j: usize;
      j = 0i32 as (usize);
      'loop12: loop {
        if j < num_clusters {
          let cur_bits: f64 =
            BrotliHistogramBitCostDistanceLiteral(&*in_.offset(i as (isize)) as
                                                  (*const HistogramLiteral),
                                                  &mut *out.offset(*clusters.offset(j as (isize)) as
                                                                   (isize)) as
                                                  (*mut HistogramLiteral) as
                                                  (*const HistogramLiteral));
          if cur_bits < best_bits {
            best_bits = cur_bits;
            best_out = *clusters.offset(j as (isize));
          }
          j = j.wrapping_add(1 as (usize));
          continue 'loop12;
        } else {
          break 'loop12;
        }
      }
      *symbols.offset(i as (isize)) = best_out;
      i = i.wrapping_add(1 as (usize));
      continue 'loop1;
    } else {
      break 'loop1;
    }
  }
  i = 0i32 as (usize);
  'loop3: loop {
    if i < num_clusters {
      HistogramClearLiteral(&mut *out.offset(*clusters.offset(i as (isize)) as (isize)) as
                            (*mut HistogramLiteral));
      i = i.wrapping_add(1 as (usize));
      continue 'loop3;
    } else {
      break 'loop3;
    }
  }
  i = 0i32 as (usize);
  'loop5: loop {
    if i < in_size {
      HistogramAddHistogramLiteral(&mut *out.offset(*symbols.offset(i as (isize)) as (isize)) as
                                   (*mut HistogramLiteral),
                                   &*in_.offset(i as (isize)) as (*const HistogramLiteral));
      i = i.wrapping_add(1 as (usize));
      continue 'loop5;
    } else {
      break 'loop5;
    }
  }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryManager {
  pub alloc_func: unsafe extern "C" fn(*mut ::std::os::raw::c_void, usize)
                                       -> *mut ::std::os::raw::c_void,
  pub free_func: unsafe extern "C" fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void),
  pub opaque: *mut ::std::os::raw::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramReindexLiteral(mut m: *mut MemoryManager,
                                                       mut out: *mut HistogramLiteral,
                                                       mut symbols: *mut u32,
                                                       mut length: usize)
                                                       -> usize {
  static kInvalidIndex: u32 = !(0i32 as (u32));
  let mut new_index: *mut u32 = if length != 0 {
    BrotliAllocate(m, length.wrapping_mul(::std::mem::size_of::<u32>())) as (*mut u32)
  } else {
    0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
  };
  let mut next_index: u32;
  let mut tmp: *mut HistogramLiteral;
  let mut i: usize;
  if !(0i32 == 0) {
    0i32 as (usize)
  } else {
    i = 0i32 as (usize);
    'loop2: loop {
      if i < length {
        *new_index.offset(i as (isize)) = kInvalidIndex;
        i = i.wrapping_add(1 as (usize));
        continue 'loop2;
      } else {
        break 'loop2;
      }
    }
    next_index = 0i32 as (u32);
    i = 0i32 as (usize);
    'loop4: loop {
      if i < length {
        if *new_index.offset(*symbols.offset(i as (isize)) as (isize)) == kInvalidIndex {
          *new_index.offset(*symbols.offset(i as (isize)) as (isize)) = next_index;
          next_index = next_index.wrapping_add(1 as (u32));
        }
        i = i.wrapping_add(1 as (usize));
        continue 'loop4;
      } else {
        break 'loop4;
      }
    }
    tmp = if next_index != 0 {
      BrotliAllocate(m,
                     (next_index as (usize))
                       .wrapping_mul(::std::mem::size_of::<HistogramLiteral>())) as
      (*mut HistogramLiteral)
    } else {
      0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral)
    };
    if !(0i32 == 0) {
      0i32 as (usize)
    } else {
      next_index = 0i32 as (u32);
      i = 0i32 as (usize);
      'loop7: loop {
        if i < length {
          if *new_index.offset(*symbols.offset(i as (isize)) as (isize)) == next_index {
            *tmp.offset(next_index as (isize)) = *out.offset(*symbols.offset(i as (isize)) as
                                                             (isize));
            next_index = next_index.wrapping_add(1 as (u32));
          }
          *symbols.offset(i as (isize)) = *new_index.offset(*symbols.offset(i as (isize)) as
                                                            (isize));
          i = i.wrapping_add(1 as (usize));
          continue 'loop7;
        } else {
          break 'loop7;
        }
      }
      BrotliFree(m, new_index as (*mut ::std::os::raw::c_void));
      new_index = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
      i = 0i32 as (usize);
      'loop9: loop {
        if i < next_index as (usize) {
          *out.offset(i as (isize)) = *tmp.offset(i as (isize));
          i = i.wrapping_add(1 as (usize));
          continue 'loop9;
        } else {
          break 'loop9;
        }
      }
      BrotliFree(m, tmp as (*mut ::std::os::raw::c_void));
      tmp = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral);
      next_index as (usize)
    }
  }
}

unsafe extern "C" fn brotli_min_size_t(mut a: usize, mut b: usize) -> usize {
  if a < b { a } else { b }
}

#[no_mangle]
pub unsafe extern "C" fn BrotliClusterHistogramsLiteral(mut m: *mut MemoryManager,
                                                        mut in_: *const HistogramLiteral,
                                                        in_size: usize,
                                                        mut max_histograms: usize,
                                                        mut out: *mut HistogramLiteral,
                                                        mut out_size: *mut usize,
                                                        mut histogram_symbols: *mut u32) {
  let mut cluster_size: *mut u32 = if in_size != 0 {
    BrotliAllocate(m, in_size.wrapping_mul(::std::mem::size_of::<u32>())) as (*mut u32)
  } else {
    0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
  };
  let mut clusters: *mut u32 = if in_size != 0 {
    BrotliAllocate(m, in_size.wrapping_mul(::std::mem::size_of::<u32>())) as (*mut u32)
  } else {
    0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
  };
  let mut num_clusters: usize = 0i32 as (usize);
  let max_input_histograms: usize = 64i32 as (usize);
  let mut pairs_capacity: usize = max_input_histograms.wrapping_mul(max_input_histograms)
    .wrapping_div(2i32 as (usize));
  let mut pairs: *mut HistogramPair = if pairs_capacity.wrapping_add(1i32 as (usize)) != 0 {
    BrotliAllocate(m,
                   pairs_capacity.wrapping_add(1i32 as (usize))
                     .wrapping_mul(::std::mem::size_of::<HistogramPair>())) as
    (*mut HistogramPair)
  } else {
    0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair)
  };
  let mut i: usize;
  if !(0i32 == 0) {
  } else {
    i = 0i32 as (usize);
    'loop2: loop {
      if i < in_size {
        *cluster_size.offset(i as (isize)) = 1i32 as (u32);
        i = i.wrapping_add(1 as (usize));
        continue 'loop2;
      } else {
        break 'loop2;
      }
    }
    i = 0i32 as (usize);
    'loop4: loop {
      if i < in_size {
        *out.offset(i as (isize)) = *in_.offset(i as (isize));
        (*out.offset(i as (isize))).bit_cost_ =
          BrotliPopulationCostLiteral(&*in_.offset(i as (isize)) as (*const HistogramLiteral));
        *histogram_symbols.offset(i as (isize)) = i as (u32);
        i = i.wrapping_add(1 as (usize));
        continue 'loop4;
      } else {
        break 'loop4;
      }
    }
    i = 0i32 as (usize);
    'loop6: loop {
      if i < in_size {
        let mut num_to_combine: usize = brotli_min_size_t(in_size.wrapping_sub(i),
                                                          max_input_histograms);
        let mut num_new_clusters: usize;
        let mut j: usize;
        j = 0i32 as (usize);
        'loop20: loop {
          if j < num_to_combine {
            *clusters.offset(num_clusters.wrapping_add(j) as (isize)) = i.wrapping_add(j) as (u32);
            j = j.wrapping_add(1 as (usize));
            continue 'loop20;
          } else {
            break 'loop20;
          }
        }
        num_new_clusters =
          BrotliHistogramCombineLiteral(out,
                                        cluster_size,
                                        &mut *histogram_symbols.offset(i as (isize)) as (*mut u32),
                                        &mut *clusters.offset(num_clusters as (isize)) as
                                        (*mut u32),
                                        pairs,
                                        num_to_combine,
                                        num_to_combine,
                                        max_histograms,
                                        pairs_capacity);
        num_clusters = num_clusters.wrapping_add(num_new_clusters);
        i = i.wrapping_add(max_input_histograms);
        continue 'loop6;
      } else {
        break 'loop6;
      }
    }
    let mut max_num_pairs: usize = brotli_min_size_t((64i32 as (usize)).wrapping_mul(num_clusters),
                                                     num_clusters.wrapping_div(2i32 as (usize))
                                                       .wrapping_mul(num_clusters));
    if pairs_capacity < max_num_pairs.wrapping_add(1i32 as (usize)) {
      let mut _new_size: usize = if pairs_capacity == 0i32 as (usize) {
        max_num_pairs.wrapping_add(1i32 as (usize))
      } else {
        pairs_capacity
      };
      let mut new_array: *mut HistogramPair;
      'loop9: loop {
        if _new_size < max_num_pairs.wrapping_add(1i32 as (usize)) {
          _new_size = _new_size.wrapping_mul(2i32 as (usize));
          continue 'loop9;
        } else {
          break 'loop9;
        }
      }
      new_array = if _new_size != 0 {
        BrotliAllocate(m,
                       _new_size.wrapping_mul(::std::mem::size_of::<HistogramPair>())) as
        (*mut HistogramPair)
      } else {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair)
      };
      if !!(0i32 == 0) && (pairs_capacity != 0i32 as (usize)) {
        memcpy(new_array as (*mut ::std::os::raw::c_void),
               pairs as (*const ::std::os::raw::c_void),
               pairs_capacity.wrapping_mul(::std::mem::size_of::<HistogramPair>()));
      }
      BrotliFree(m, pairs as (*mut ::std::os::raw::c_void));
      pairs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair);
      pairs = new_array;
      pairs_capacity = _new_size;
    }
    if !(0i32 == 0) {
    } else {
      num_clusters = BrotliHistogramCombineLiteral(out,
                                                   cluster_size,
                                                   histogram_symbols,
                                                   clusters,
                                                   pairs,
                                                   num_clusters,
                                                   in_size,
                                                   max_histograms,
                                                   max_num_pairs);
      BrotliFree(m, pairs as (*mut ::std::os::raw::c_void));
      pairs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair);
      BrotliFree(m, cluster_size as (*mut ::std::os::raw::c_void));
      cluster_size = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
      BrotliHistogramRemapLiteral(in_,
                                  in_size,
                                  clusters as (*const u32),
                                  num_clusters,
                                  out,
                                  histogram_symbols);
      BrotliFree(m, clusters as (*mut ::std::os::raw::c_void));
      clusters = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
      *out_size = BrotliHistogramReindexLiteral(m, out, histogram_symbols, in_size);
      if !(0i32 == 0) {}
    }
  }
}
/////////// DONE //////////////////////////
