#![allow(dead_code)]
use core;
use super::histogram::CostAccessors;
use super::super::alloc::SliceWrapper;

use super::util::{brotli_max_uint32_t, FastLog2};


static mut kCopyBase: [u32; 24] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 18, 22, 30, 38, 54, 70,
                                   102, 134, 198, 326, 582, 1094, 2118];

static mut kCopyExtra: [u32; 24] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 7, 8,
                                    9, 10, 24];

static kBrotliMinWindowBits: i32 = 10i32;

static kBrotliMaxWindowBits: i32 = 24i32;



pub fn ShannonEntropy(mut population: &[u32], size: usize, mut total: &mut usize) -> super::util::floatX {
  let mut sum: usize = 0usize;
  let mut retval: super::util::floatX = 0i32 as super::util::floatX;
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
      retval = retval - p as super::util::floatX * FastLog2(p as u64);
    }
    odd_number_of_elements_left = 0i32;
    p = population[0] as usize;
    population = &population[1..];
    sum = sum.wrapping_add(p);
    retval = retval - p as super::util::floatX * FastLog2(p as u64);
  }
  if sum != 0 {
    retval = retval + sum as super::util::floatX * FastLog2(sum as u64);
  }
  *total = sum;
  retval
}

pub fn BitsEntropy(population: &[u32], size: usize) -> super::util::floatX {
  let mut sum: usize = 0;
  let mut retval: super::util::floatX = ShannonEntropy(population, size, &mut sum);
  if retval < sum as super::util::floatX {
    retval = sum as super::util::floatX;
  }
  retval
}

const BROTLI_REPEAT_ZERO_CODE_LENGTH: usize = 17;
const BROTLI_CODE_LENGTH_CODES: usize = BROTLI_REPEAT_ZERO_CODE_LENGTH + 1;

fn CostComputation(depth_histo: &mut [u32;BROTLI_CODE_LENGTH_CODES],
                   nnz_data: &[super::vectorization::Mem256i],
                   nnz: usize,
                   total_count: super::util::floatX,
                   log2total: super::util::floatX) -> super::util::floatX {
    let mut bits : super::util::floatX = 0 as super::util::floatX;
    if (true) {
      let mut max_depth : usize = 1;
      for i in 0..nnz {
          // Compute -log2(P(symbol)) = -log2(count(symbol)/total_count) =
          //                            = log2(total_count) - log2(count(symbol))
         let element = nnz_data[i>>3].0[i&7];
         let log2p = log2total - FastLog2(element as u64);
         // Approximate the bit depth by round(-log2(P(symbol)))
         let mut depth = core::cmp::min((log2p + 0.5) as u8, 15u8);
         bits += element as super::util::floatX * log2p;
         if (depth as usize > max_depth) {
            max_depth = depth as usize;
         }
         depth_histo[depth as usize] += 1;
      }

      // Add the estimated encoding cost of the code length code histogram.
      bits += (18 + 2 * max_depth) as super::util::floatX;
      // Add the entropy of the code length code histogram.
      bits += BitsEntropy(depth_histo, BROTLI_CODE_LENGTH_CODES);
      return bits;
    }
    
    return bits;
}
use alloc::SliceWrapperMut;

pub fn BrotliPopulationCost<HistogramType:SliceWrapper<u32>+CostAccessors>(
    histogram : &HistogramType
) -> super::util::floatX{
  static kOneSymbolHistogramCost: super::util::floatX = 12i32 as super::util::floatX;
  static kTwoSymbolHistogramCost: super::util::floatX = 20i32 as super::util::floatX;
  static kThreeSymbolHistogramCost: super::util::floatX = 28i32 as super::util::floatX;
  static kFourSymbolHistogramCost: super::util::floatX = 37i32 as super::util::floatX;
  let data_size: usize = (*histogram).slice().len();
  let mut count: i32 = 0i32;
  let mut s: [usize; 5] = [0; 5];

  let mut bits: super::util::floatX = 0.0 as super::util::floatX;
  let mut i: usize;
  if (*histogram).total_count() == 0usize {
    return kOneSymbolHistogramCost;
  }
  i = 0usize;
  'break1: while i < data_size {
    {
      if (*histogram).slice()[i] > 0u32 {
        s[count as (usize)] = i;
        count = count + 1;
        if count > 4i32 {
          {
            break 'break1;
          }
        }
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  if count == 1i32 {
    return kOneSymbolHistogramCost;
  }
  if count == 2i32 {
    return kTwoSymbolHistogramCost + (*histogram).total_count() as super::util::floatX;
  }
  if count == 3i32 {
    let histo0: u32 = (*histogram).slice()[s[0usize]];
    let histo1: u32 = (*histogram).slice()[s[1usize]];
    let histo2: u32 = (*histogram).slice()[s[2usize]];
    let histomax: u32 = brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2));
    return kThreeSymbolHistogramCost +
           (2u32).wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2)) as super::util::floatX -
           histomax as super::util::floatX;
  }
  if count == 4i32 {
    let mut histo: [u32; 4] = [0; 4];
    let h23: u32;
    let histomax: u32;
    i = 0usize;
    while i < 4usize {
      {
        histo[i] = (*histogram).slice()[s[i]];
      }
      i = i.wrapping_add(1 as (usize));
    }
    i = 0usize;
    while i < 4usize {
      {
        let mut j: usize;
        j = i.wrapping_add(1usize);
        while j < 4usize {
          {
            if histo[j] > histo[i] {
              let mut __brotli_swap_tmp: u32 = histo[j];
              histo[j] = histo[i];
              histo[i] = __brotli_swap_tmp;
            }
          }
          j = j.wrapping_add(1 as (usize));
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
    h23 = histo[2usize].wrapping_add(histo[3usize]);
    histomax = brotli_max_uint32_t(h23, histo[0usize]);
    return kFourSymbolHistogramCost + (3u32).wrapping_mul(h23) as super::util::floatX +
           (2u32).wrapping_mul(histo[0usize].wrapping_add(histo[1usize])) as super::util::floatX -
           histomax as super::util::floatX;
  }
  {
    let mut nnz: usize = 0;
    let mut max_depth: usize = 1usize;
    let mut depth_histo: [u32; 18] = [0u32; 18];
    let mut nnz_data = HistogramType::make_nnz_storage();
    let total_count = (*histogram).total_count() as super::util::floatX;
    let log2total = FastLog2((*histogram).total_count() as u64);
    i = 0usize;
    while i < data_size {
      if (*histogram).slice()[i] > 0u32 {
        nnz_data.slice_mut()[nnz>>3].0[nnz&7] = histogram.slice()[i] as i32;
        i += 1;
        nnz += 1;
      } else {
        let mut reps: u32 = 1;
        let mut k: usize;
        for hd in (*histogram).slice()[i+1..(data_size as usize)].iter() {
            if *hd != 0 {
               break
            }
            reps += 1
        }
        i += reps as usize;
        if i == data_size {
          {
            break;
          }
        }
        if reps < 3 {
          depth_histo[0] += reps
        } else {
          reps -= 2;
          let mut depth_histo_adds : u32 = 0;
          while reps > 0u32 {
            depth_histo_adds += 1;
            bits = bits + 3i32 as super::util::floatX;
            reps = reps >> 3i32;
          }
          depth_histo[BROTLI_REPEAT_ZERO_CODE_LENGTH] += depth_histo_adds;
        }
      }
    }
    bits += CostComputation(&mut depth_histo, nnz_data.slice(), nnz, total_count, log2total);
  }
  bits
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
