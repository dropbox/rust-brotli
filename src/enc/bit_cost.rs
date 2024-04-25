#![allow(dead_code)]
use super::super::alloc::SliceWrapper;
use super::histogram::CostAccessors;
use core::cmp::{max, min};

use super::util::{FastLog2, FastLog2u16};
use super::vectorization::Mem256i;

static kCopyBase: [u32; 24] = [
    2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 18, 22, 30, 38, 54, 70, 102, 134, 198, 326, 582, 1094, 2118,
];

static kCopyExtra: [u32; 24] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 7, 8, 9, 10, 24,
];

static kBrotliMinWindowBits: i32 = 10i32;

static kBrotliMaxWindowBits: i32 = 24i32;

pub fn ShannonEntropy(
    mut population: &[u32],
    size: usize,
    total: &mut usize,
) -> super::util::floatX {
    let mut sum: usize = 0usize;
    let mut retval: super::util::floatX = 0i32 as super::util::floatX;
    let mut p: usize;
    if size & 1 != 0 && !population.is_empty() {
        p = population[0] as usize;
        population = population.split_at(1).1;
        sum = sum.wrapping_add(p);
        retval -= p as super::util::floatX * FastLog2u16(p as u16);
    }
    for pop_iter in population.split_at((size >> 1) << 1).0 {
        p = *pop_iter as usize;
        sum = sum.wrapping_add(p);
        retval -= p as super::util::floatX * FastLog2u16(p as u16);
    }
    if sum != 0 {
        retval += sum as super::util::floatX * FastLog2(sum as u64); // not sure it's 16 bit
    }
    *total = sum;
    retval
}

#[inline(always)]
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
/*
use std::io::{self, Error, ErrorKind, Read, Write};

macro_rules! println_stderr(
    ($($val:tt)*) => { {
        writeln!(&mut ::std::io::stderr(), $($val)*).unwrap();
    } }
);
*/

#[cfg(feature = "vector_scratch_space")]
const vectorize_population_cost: bool = true;

#[cfg(not(feature = "vector_scratch_space"))]
const vectorize_population_cost: bool = false;

#[allow(clippy::excessive_precision)]
fn CostComputation<T: SliceWrapper<Mem256i>>(
    depth_histo: &mut [u32; BROTLI_CODE_LENGTH_CODES],
    nnz_data: &T,
    nnz: usize,
    _total_count: super::util::floatX,
    log2total: super::util::floatX,
) -> super::util::floatX {
    let mut bits: super::util::floatX = 0.0 as super::util::floatX;
    let mut max_depth: usize = 1;
    for i in 0..nnz {
        // Compute -log2(P(symbol)) = -log2(count(symbol)/total_count) =
        //                            = log2(total_count) - log2(count(symbol))
        let element = nnz_data.slice()[i >> 3][i & 7];
        let log2p = log2total - FastLog2u16(element as u16);
        // Approximate the bit depth by round(-log2(P(symbol)))
        let depth = min((log2p + 0.5) as u8, 15u8);
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
    //println_stderr!("{:?} {:?}", &depth_histo[..], bits);
    bits
}

use alloc::SliceWrapperMut;

pub fn BrotliPopulationCost<HistogramType: SliceWrapper<u32> + CostAccessors>(
    histogram: &HistogramType,
    nnz_data: &mut HistogramType::i32vec,
) -> super::util::floatX {
    static kOneSymbolHistogramCost: super::util::floatX = 12i32 as super::util::floatX;
    static kTwoSymbolHistogramCost: super::util::floatX = 20i32 as super::util::floatX;
    static kThreeSymbolHistogramCost: super::util::floatX = 28i32 as super::util::floatX;
    static kFourSymbolHistogramCost: super::util::floatX = 37i32 as super::util::floatX;
    let data_size: usize = histogram.slice().len();
    let mut count: i32 = 0i32;
    let mut s: [usize; 5] = [0; 5];

    let mut bits: super::util::floatX = 0.0 as super::util::floatX;
    let mut i: usize;
    if histogram.total_count() == 0usize {
        return kOneSymbolHistogramCost;
    }
    i = 0usize;
    'break1: while i < data_size {
        {
            if histogram.slice()[i] > 0u32 {
                s[count as usize] = i;
                count += 1;
                if count > 4i32 {
                    break 'break1;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if count == 1i32 {
        return kOneSymbolHistogramCost;
    }
    if count == 2i32 {
        return kTwoSymbolHistogramCost + histogram.total_count() as super::util::floatX;
    }
    if count == 3i32 {
        let histo0: u32 = histogram.slice()[s[0]];
        let histo1: u32 = histogram.slice()[s[1]];
        let histo2: u32 = histogram.slice()[s[2]];
        let histomax: u32 = max(histo0, max(histo1, histo2));
        return kThreeSymbolHistogramCost
            + (2u32).wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2))
                as super::util::floatX
            - histomax as super::util::floatX;
    }
    if count == 4i32 {
        let mut histo: [u32; 4] = [0; 4];

        for i in 0usize..4usize {
            histo[i] = histogram.slice()[s[i]];
        }
        for i in 0..4 {
            for j in i + 1..4 {
                if histo[j] > histo[i] {
                    histo.swap(j, i);
                }
            }
        }
        let h23: u32 = histo[2].wrapping_add(histo[3]);
        let histomax: u32 = max(h23, histo[0]);
        return kFourSymbolHistogramCost
            + (3u32).wrapping_mul(h23) as super::util::floatX
            + (2u32).wrapping_mul(histo[0].wrapping_add(histo[1])) as super::util::floatX
            - histomax as super::util::floatX;
    }
    if vectorize_population_cost {
        // vectorization failed: it's faster to do things inline than split into two loops
        let mut nnz: usize = 0;
        let mut depth_histo = [0u32; 18];
        let total_count = histogram.total_count() as super::util::floatX;
        let log2total = FastLog2(histogram.total_count() as u64);
        i = 0usize;
        while i < data_size {
            if histogram.slice()[i] > 0u32 {
                let nnz_val = &mut nnz_data.slice_mut()[nnz >> 3];
                nnz_val[nnz & 7] = histogram.slice()[i] as i32;
                i += 1;
                nnz += 1;
            } else {
                let mut reps: u32 = 1;
                for hd in histogram.slice()[i + 1..data_size].iter() {
                    if *hd != 0 {
                        break;
                    }
                    reps += 1
                }
                i += reps as usize;
                if i == data_size {
                    break;
                }
                if reps < 3 {
                    depth_histo[0] += reps
                } else {
                    reps -= 2;
                    let mut depth_histo_adds: u32 = 0;
                    while reps > 0u32 {
                        depth_histo_adds += 1;
                        bits += 3i32 as super::util::floatX;
                        reps >>= 3i32;
                    }
                    depth_histo[BROTLI_REPEAT_ZERO_CODE_LENGTH] += depth_histo_adds;
                }
            }
        }
        bits += CostComputation(&mut depth_histo, nnz_data, nnz, total_count, log2total);
    } else {
        let mut max_depth: usize = 1;
        let mut depth_histo = [0u32; 18];
        let log2total: super::util::floatX = FastLog2(histogram.total_count() as u64); // 64 bit here
        let mut reps: u32 = 0;
        for histo in histogram.slice()[..data_size].iter() {
            if *histo != 0 {
                if reps != 0 {
                    if reps < 3 {
                        depth_histo[0] += reps;
                    } else {
                        reps -= 2;
                        while reps > 0u32 {
                            depth_histo[17] += 1;
                            bits += 3 as super::util::floatX;
                            reps >>= 3;
                        }
                    }
                    reps = 0;
                }
                let log2p: super::util::floatX = log2total - FastLog2u16(*histo as u16);
                let mut depth: usize = (log2p + 0.5 as super::util::floatX) as usize;
                bits += *histo as super::util::floatX * log2p;
                depth = min(depth, 15);
                max_depth = max(depth, max_depth);
                depth_histo[depth] += 1;
            } else {
                reps += 1;
            }
        }
        bits += (18usize).wrapping_add((2usize).wrapping_mul(max_depth)) as super::util::floatX;
        bits += BitsEntropy(&depth_histo[..], 18usize);
    }
    bits
}
/*
fn HistogramDataSizeCommand() -> usize {
    704usize
}*/

/*
fn HistogramDataSizeDistance() -> usize {
    520usize
}
*/
