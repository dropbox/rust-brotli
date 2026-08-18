use crate::alloc::SliceWrapperMut;
use core::cmp::{max, min};

use fearless_simd::{Simd, SimdBase, SimdMask, u32x16};

use super::super::alloc::SliceWrapper;
use super::histogram::CostAccessors;
use super::util::{FastLog2, FastLog2u16};
use super::vectorization::{Mem256i, detect_level};
use crate::enc::floatX;

const BROTLI_REPEAT_ZERO_CODE_LENGTH: usize = 17;
const BROTLI_CODE_LENGTH_CODES: usize = BROTLI_REPEAT_ZERO_CODE_LENGTH + 1;

pub(crate) fn shannon_entropy(mut population: &[u32], size: usize) -> (floatX, usize) {
    let mut sum: usize = 0;
    let mut retval: floatX = 0.0;

    if (size & 1) != 0 && !population.is_empty() {
        let p = population[0] as usize;
        population = population.split_at(1).1;
        sum = sum.wrapping_add(p);
        retval -= p as floatX * FastLog2u16(p as u16);
    }
    for pop_iter in population.split_at((size >> 1) << 1).0 {
        let p = *pop_iter as usize;
        sum = sum.wrapping_add(p);
        retval -= p as floatX * FastLog2u16(p as u16);
    }
    if sum != 0 {
        retval += sum as floatX * FastLog2(sum as u64); // not sure it's 16 bit
    }

    (retval, sum)
}

#[inline(always)]
pub fn BitsEntropy(population: &[u32], size: usize) -> floatX {
    let (mut retval, sum) = shannon_entropy(population, size);
    if retval < sum as floatX {
        retval = sum as floatX;
    }
    retval
}

#[allow(clippy::excessive_precision)]
fn CostComputation<T: SliceWrapper<Mem256i>>(
    depth_histo: &mut [u32; BROTLI_CODE_LENGTH_CODES],
    nnz_data: &T,
    nnz: usize,
    _total_count: floatX,
    log2total: floatX,
) -> floatX {
    let mut bits: floatX = 0.0;
    let mut max_depth: usize = 1;
    for i in 0..nnz {
        // Compute -log2(P(symbol)) = -log2(count(symbol)/total_count) =
        //                            = log2(total_count) - log2(count(symbol))
        let element = nnz_data.slice()[i >> 3][i & 7];
        let log2p = log2total - FastLog2u16(element as u16);
        // Approximate the bit depth by round(-log2(P(symbol)))
        let depth = min((log2p + 0.5) as u8, 15u8);
        bits += (element as floatX) * log2p;
        if (depth as usize) > max_depth {
            max_depth = depth as usize;
        }
        depth_histo[depth as usize] += 1;
    }

    // Add the estimated encoding cost of the code length code histogram.
    bits += (18 + 2 * max_depth) as floatX;
    // Add the entropy of the code length code histogram.
    bits += BitsEntropy(depth_histo, BROTLI_CODE_LENGTH_CODES);
    //println_stderr!("{:?} {:?}", &depth_histo[..], bits);
    bits
}

/// The bucket values a population cost is charged over.
///
/// Clustering spends nearly all of its population-cost calls on the *sum* of two histograms —
/// [`BrotliHistogramBitCostDistance`](super::cluster::BrotliHistogramBitCostDistance) and the
/// pair queue in [`cluster`](super::cluster). Materializing that sum costs a full copy plus a
/// full add before the cost walk even starts, so [`Sum`] models it instead and lets the walk
/// the cost already makes do the adding.
trait Buckets {
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> u32;
    /// The sixteen buckets starting at `at`, which must leave sixteen in range.
    fn chunk<S: Simd>(&self, simd: S, at: usize) -> u32x16<S>;
}

/// One histogram's own buckets.
struct Own<'a>(&'a [u32]);

/// Two histograms' buckets added lane-wise, never materialized.
///
/// Wrapping addition and the left operand's length match
/// [`HistogramAddHistogram`](super::histogram::HistogramAddHistogram), which is what the
/// materializing form used to call.
struct Sum<'a>(&'a [u32], &'a [u32]);

impl Buckets for Own<'_> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }
    #[inline(always)]
    fn get(&self, i: usize) -> u32 {
        self.0[i]
    }
    #[inline(always)]
    fn chunk<S: Simd>(&self, simd: S, at: usize) -> u32x16<S> {
        u32x16::from_slice(simd, &self.0[at..at + 16])
    }
}

impl Buckets for Sum<'_> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }
    #[inline(always)]
    fn get(&self, i: usize) -> u32 {
        self.0[i].wrapping_add(self.1[i])
    }
    #[inline(always)]
    fn chunk<S: Simd>(&self, simd: S, at: usize) -> u32x16<S> {
        u32x16::from_slice(simd, &self.0[at..at + 16])
            + u32x16::from_slice(simd, &self.1[at..at + 16])
    }
}

pub fn BrotliPopulationCost<HistogramType: SliceWrapper<u32> + CostAccessors>(
    histogram: &HistogramType,
    nnz_data: &mut HistogramType::i32vec,
) -> floatX {
    population_cost(Own(histogram.slice()), histogram.total_count(), nnz_data)
}

/// Cost of the histogram that adding `b` into `a` would produce, without building it.
///
/// Bit-for-bit identical to cloning `a`, calling
/// [`HistogramAddHistogram`](super::histogram::HistogramAddHistogram) with `b` and costing the
/// result: the walk sees the same bucket values in the same order, so the float accumulation is
/// unchanged. It just skips the copy and the separate add pass.
pub fn BrotliPopulationCostOfSum<HistogramType: SliceWrapper<u32> + CostAccessors>(
    a: &HistogramType,
    b: &HistogramType,
    nnz_data: &mut HistogramType::i32vec,
) -> floatX {
    debug_assert_eq!(a.slice().len(), b.slice().len());
    population_cost(
        Sum(a.slice(), b.slice()),
        a.total_count() + b.total_count(),
        nnz_data,
    )
}

fn population_cost<B: Buckets, Scratch: SliceWrapper<Mem256i> + SliceWrapperMut<Mem256i>>(
    buckets: B,
    total_count: usize,
    nnz_data: &mut Scratch,
) -> floatX {
    static kOneSymbolHistogramCost: floatX = 12.0;
    static kTwoSymbolHistogramCost: floatX = 20.0;
    static kThreeSymbolHistogramCost: floatX = 28.0;
    static kFourSymbolHistogramCost: floatX = 37.0;

    let data_size: usize = buckets.len();
    let mut count = 0;
    let mut s: [usize; 5] = [0; 5];
    let mut bits: floatX = 0.0;

    if total_count == 0 {
        return kOneSymbolHistogramCost;
    }
    for i in 0..data_size {
        if buckets.get(i) > 0 {
            s[count] = i;
            count += 1;
            if count > 4 {
                break;
            }
        }
    }
    match count {
        1 => return kOneSymbolHistogramCost,
        2 => return kTwoSymbolHistogramCost + total_count as floatX,
        3 => {
            let histo0: u32 = buckets.get(s[0]);
            let histo1: u32 = buckets.get(s[1]);
            let histo2: u32 = buckets.get(s[2]);
            let histomax: u32 = max(histo0, max(histo1, histo2));
            return kThreeSymbolHistogramCost
                + (2u32).wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2)) as floatX
                - histomax as floatX;
        }
        4 => {
            let mut histo: [u32; 4] = [0; 4];

            for i in 0..4 {
                histo[i] = buckets.get(s[i]);
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
                + (3u32).wrapping_mul(h23) as floatX
                + (2u32).wrapping_mul(histo[0].wrapping_add(histo[1])) as floatX
                - histomax as floatX;
        }
        _ => {}
    }

    if cfg!(feature = "vector_scratch_space") {
        // vectorization failed: it's faster to do things inline than split into two loops
        let mut nnz: usize = 0;
        let mut depth_histo = [0u32; 18];
        let total_count_f = total_count as floatX;
        let log2total = FastLog2(total_count as u64);
        let mut i: usize = 0;
        while i < data_size {
            if buckets.get(i) > 0 {
                let histo = buckets.get(i);
                let nnz_val = &mut nnz_data.slice_mut()[nnz >> 3];
                nnz_val[nnz & 7] = histo as i32;
                i += 1;
                nnz += 1;
            } else {
                let mut reps: u32 = 1;
                for j in i + 1..data_size {
                    if buckets.get(j) != 0 {
                        break;
                    }
                    reps += 1
                }
                i += reps as usize;
                if i == data_size {
                    break;
                }
                if reps < 3 {
                    depth_histo[0] += reps;
                } else {
                    reps -= 2;
                    let mut depth_histo_adds: u32 = 0;
                    while reps > 0 {
                        depth_histo_adds += 1;
                        bits += 3.0;
                        reps >>= 3;
                    }
                    depth_histo[BROTLI_REPEAT_ZERO_CODE_LENGTH] += depth_histo_adds;
                }
            }
        }
        bits += CostComputation(&mut depth_histo, nnz_data, nnz, total_count_f, log2total);
    } else {
        let mut depth_histo = [0u32; BROTLI_CODE_LENGTH_CODES];
        let log2total: floatX = FastLog2(total_count as u64); // 64 bit here
        let max_depth = dispatch!(detect_level(), simd => accumulate_symbol_costs(
            simd,
            &buckets,
            log2total,
            &mut bits,
            &mut depth_histo,
        ));
        bits += (18usize).wrapping_add((2usize).wrapping_mul(max_depth)) as floatX;
        bits += BitsEntropy(&depth_histo[..], BROTLI_CODE_LENGTH_CODES);
    }
    bits
}

/// Charges one populated bucket, first flushing the run of empty buckets before it.
///
/// Split out of [`accumulate_symbol_costs`] so the vectorized and remainder walks share it,
/// which keeps `bits` accumulating in bucket order and therefore bit-identical.
#[inline(always)]
fn accumulate_one_symbol(
    histo: u32,
    log2total: floatX,
    bits: &mut floatX,
    max_depth: &mut usize,
    reps: &mut u32,
    depth_histo: &mut [u32; BROTLI_CODE_LENGTH_CODES],
) {
    if *reps != 0 {
        if *reps < 3 {
            depth_histo[0] += *reps;
        } else {
            let mut remaining = *reps - 2;
            while remaining > 0 {
                depth_histo[BROTLI_REPEAT_ZERO_CODE_LENGTH] += 1;
                *bits += 3.0;
                remaining >>= 3;
            }
        }
        *reps = 0;
    }
    let log2p = log2total - FastLog2u16(histo as u16);
    let depth = min((log2p + 0.5) as usize, 15);
    *bits += histo as floatX * log2p;
    *max_depth = max(depth, *max_depth);
    depth_histo[depth] += 1;
}

/// Charges every populated bucket of `buckets`, returning the deepest code length seen.
///
/// Bucket occupancy is bimodal rather than uniformly sparse: measured over a mixed 3.9 MB
/// corpus at q10, 43% of buckets are populated overall, but they cluster — the 256-bucket
/// literal alphabet is dense over the ASCII range and empty above it, while the 704-bucket
/// command and 544-bucket distance alphabets are mostly empty. So the walk tests sixteen
/// buckets per compare and takes a straight-line branch for each extreme, falling back to the
/// bit-at-a-time scan only for mixed chunks. Both were measured to beat a plain scalar walk
/// and an eight-wide test. Empty runs still land in `depth_histo` exactly as a
/// bucket-at-a-time scan would leave them.
#[inline(always)]
fn accumulate_symbol_costs<S: Simd, B: Buckets>(
    simd: S,
    buckets: &B,
    log2total: floatX,
    bits: &mut floatX,
    depth_histo: &mut [u32; BROTLI_CODE_LENGTH_CODES],
) -> usize {
    const LANES: usize = 16;
    let mut max_depth: usize = 1;
    let mut reps: u32 = 0;
    let empty = u32x16::splat(simd, 0);

    let data_size = buckets.len();
    // Every histogram alphabet is a multiple of `LANES`, so the scalar tail below is normally
    // dead; it is there to keep the walk correct for any bucket count.
    let vectorized = data_size & !(LANES - 1);
    let mut at = 0;
    while at < vectorized {
        let chunk = buckets.chunk(simd, at);
        let mut populated = !chunk.simd_eq(empty).to_bitmask() & 0xffff;
        if populated == 0 {
            reps += LANES as u32;
            at += LANES;
            continue;
        }
        if populated == 0xffff {
            for lane in 0..LANES {
                accumulate_one_symbol(
                    chunk[lane],
                    log2total,
                    bits,
                    &mut max_depth,
                    &mut reps,
                    depth_histo,
                );
            }
            at += LANES;
            continue;
        }
        let mut scanned = 0u32;
        while populated != 0 {
            let lane = populated.trailing_zeros();
            reps += lane - scanned;
            scanned = lane + 1;
            populated &= populated - 1;
            // Read the lane back off `chunk` rather than through `buckets`: for `Sum` that is
            // the difference between one register extract and re-loading both operands.
            accumulate_one_symbol(
                chunk[lane as usize],
                log2total,
                bits,
                &mut max_depth,
                &mut reps,
                depth_histo,
            );
        }
        reps += LANES as u32 - scanned;
        at += LANES;
    }
    for i in vectorized..data_size {
        let histo = buckets.get(i);
        if histo != 0 {
            accumulate_one_symbol(
                histo,
                log2total,
                bits,
                &mut max_depth,
                &mut reps,
                depth_histo,
            );
        } else {
            reps += 1;
        }
    }
    max_depth
}
