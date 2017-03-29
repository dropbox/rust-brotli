use super::super::alloc::SliceWrapper;

use super::util::{brotli_max_uint32_t, FastLog2};
use super::histogram::{CostAccessors, HistogramLiteral, HistogramCommand, HistogramDistance};


static mut kCopyBase: [u32; 24] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 18, 22, 30, 38, 54, 70,
                                   102, 134, 198, 326, 582, 1094, 2118];

static mut kCopyExtra: [u32; 24] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 7, 8,
                                    9, 10, 24];

static kBrotliMinWindowBits: i32 = 10i32;

static kBrotliMaxWindowBits: i32 = 24i32;



fn ShannonEntropy(mut population: &[u32], mut size: usize, mut total: &mut usize) -> f64 {
  let mut sum: usize = 0usize;
  let mut retval: f64 = 0i32 as (f64);
  population = &population[..(size as usize)];
  let mut p: usize;
  let mut odd_number_of_elements_left: i32 = 0i32;
  if size & 1usize != 0 {
    odd_number_of_elements_left = 1i32;
  }
  while population.len() != 0 {
    if odd_number_of_elements_left == 0 {
      p = population[0] as usize;
      population = &population[1..];
      sum = sum.wrapping_add(p);
      retval = retval - p as (f64) * FastLog2(p);
    }
    odd_number_of_elements_left = 0i32;
    p = population[0] as usize;
    population = &population[1..];
    sum = sum.wrapping_add(p);
    retval = retval - p as (f64) * FastLog2(p);
  }
  if sum != 0 {
    retval = retval + sum as (f64) * FastLog2(sum);
  }
  *total = sum;
  retval
}

fn BitsEntropy(mut population: &[u32], mut size: usize) -> f64 {
  let mut sum: usize = 0;
  let mut retval: f64 = ShannonEntropy(population, size, &mut sum);
  if retval < sum as (f64) {
    retval = sum as (f64);
  }
  retval
}

pub fn BrotliPopulationCost<HistogramType:SliceWrapper<u32>+CostAccessors>(
    mut histogram : &HistogramLiteral
) -> f64{
  static kOneSymbolHistogramCost: f64 = 12i32 as (f64);
  static kTwoSymbolHistogramCost: f64 = 20i32 as (f64);
  static kThreeSymbolHistogramCost: f64 = 28i32 as (f64);
  static kFourSymbolHistogramCost: f64 = 37i32 as (f64);
  let data_size: usize = (*histogram).slice().len();
  let mut count: i32 = 0i32;
  let mut s: [usize; 5] = [0; 5];
  let mut bits: f64 = 0.0f64;
  let mut i: usize;
  if (*histogram).total_count() == 0i32 as (usize) {
    kOneSymbolHistogramCost
  } else {
    i = 0i32 as (usize);
    'loop2: loop {
      if i < data_size {
        if (*histogram).slice()[i] > 0 {
          s[count as (usize)] = i;
          count = count + 1;
          if count > 4i32 {
            break 'loop2;
          }
        }
        i = i.wrapping_add(1 as (usize));
        continue 'loop2;
      } else {
        break 'loop2;
      }
    }
    if count == 1i32 {
      kOneSymbolHistogramCost
    } else if count == 2i32 {
      kTwoSymbolHistogramCost + (*histogram).total_count() as (f64)
    } else if count == 3i32 {
      let histo0: u32 = (*histogram).slice()[s[0i32 as (usize)]];
      let histo1: u32 = (*histogram).slice()[s[1i32 as (usize)]];
      let histo2: u32 = (*histogram).slice()[s[2i32 as (usize)]];
      let histomax: u32 = brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2));
      kThreeSymbolHistogramCost +
      (2i32 as (u32)).wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2)) as (f64) -
      histomax as (f64)
    } else if count == 4i32 {
      let mut histo: [u32; 4] = [0; 4];
      let mut h23: u32;
      let mut histomax: u32;
      i = 0i32 as (usize);
      'loop30: loop {
        if i < 4i32 as (usize) {
          histo[i] = (*histogram).slice()[s[i]];
          i = i.wrapping_add(1 as (usize));
          continue 'loop30;
        } else {
          break 'loop30;
        }
      }
      i = 0i32 as (usize);
      'loop32: loop {
        if i < 4i32 as (usize) {
          let mut j: usize;
          j = i.wrapping_add(1i32 as (usize));
          'loop35: loop {
            if j < 4i32 as (usize) {
              if histo[j] > histo[i] {
                let mut __brotli_swap_tmp: u32 = histo[j];
                histo[j] = histo[i];
                histo[i] = __brotli_swap_tmp;
              }
              j = j.wrapping_add(1 as (usize));
              continue 'loop35;
            } else {
              break 'loop35;
            }
          }
          i = i.wrapping_add(1 as (usize));
          continue 'loop32;
        } else {
          break 'loop32;
        }
      }
      h23 = histo[2i32 as (usize)].wrapping_add(histo[3i32 as (usize)]);
      histomax = brotli_max_uint32_t(h23, histo[0i32 as (usize)]);
      kFourSymbolHistogramCost + (3i32 as (u32)).wrapping_mul(h23) as (f64) +
      (2i32 as (u32)).wrapping_mul(histo[0i32 as (usize)].wrapping_add(histo[1i32 as
                                                                       (usize)])) as (f64) -
      histomax as (f64)
    } else {
      let mut max_depth: usize = 1i32 as (usize);
      let mut depth_histo: [u32; 18] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
      let log2total: f64 = FastLog2((*histogram).total_count());
      i = 0i32 as (usize);
      'loop11: loop {
        if i < data_size {
          if (*histogram).slice()[i] > 0 {
            let mut log2p: f64 = log2total - FastLog2((*histogram).slice()[i] as (usize));
            let mut depth: usize = (log2p + 0.5f64) as (usize);
            bits = bits + (*histogram).slice()[i] as (f64) * log2p;
            if depth > 15i32 as (usize) {
              depth = 15i32 as (usize);
            }
            if depth > max_depth {
              max_depth = depth;
            }
            {
              let _rhs = 1;
              let _lhs = &mut depth_histo[depth];
              *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop11;
          } else {
            let mut reps: u32 = 1;
            let mut k: usize;
            k = i.wrapping_add(1i32 as (usize));
            'loop14: loop {
              if k < data_size && ((*histogram).slice()[k] == 0) {
                reps = reps.wrapping_add(1 as (u32));
                k = k.wrapping_add(1 as (usize));
                continue 'loop14;
              } else {
                break 'loop14;
              }
            }
            i = i.wrapping_add(reps as (usize));
            if i == data_size {
              break 'loop11;
            } else if reps < 3 {
              {
                let _rhs = reps;
                let _lhs = &mut depth_histo[0i32 as (usize)];
                *_lhs = (*_lhs).wrapping_add(_rhs);
              }
              continue 'loop11;
            } else {
              reps = reps.wrapping_sub(2);
              'loop18: loop {
                if reps > 0 {
                  {
                    let _rhs = 1;
                    let _lhs = &mut depth_histo[17i32 as (usize)];
                    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                  }
                  bits = bits + 3i32 as (f64);
                  reps = reps >> 3i32;
                  continue 'loop18;
                } else {
                  continue 'loop11;
                }
              }
            }
          }
        } else {
          break 'loop11;
        }
      }
      bits = bits +
             (18i32 as (usize)).wrapping_add((2i32 as (usize)).wrapping_mul(max_depth)) as (f64);
      bits = bits + BitsEntropy(&depth_histo[..], 18i32 as (usize));
      bits
    }
  }
}
/*
fn HistogramDataSizeCommand() -> usize {
    704i32 as (usize)
}*/


/*
fn HistogramDataSizeDistance() -> usize {
    520i32 as (usize)
}
*/
