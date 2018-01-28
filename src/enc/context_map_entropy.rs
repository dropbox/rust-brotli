use core;
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::interface;
use super::input_pair::{InputPair, InputReference};
use super::histogram::ContextType;
use super::constants::{kSigned3BitContextLookup, kUTF8ContextLookup};
use super::util::{floatX, FastLog2u16};
use super::find_stride;
use super::weights::{Weights, BLEND_FIXED_POINT_PRECISION};
const NUM_SPEEDS_TO_TRY: usize = 32;
const NIBBLE_PRIOR_SIZE: usize = 16 * NUM_SPEEDS_TO_TRY;
// the high nibble, followed by the low nibbles
const CONTEXT_MAP_PRIOR_SIZE: usize = 256 * NIBBLE_PRIOR_SIZE * 17;
const CONTEXT_MAP_COST_SIZE: usize = 256 * NUM_SPEEDS_TO_TRY * 2;
const STRIDE_PRIOR_SIZE: usize = 256 * 256 * NIBBLE_PRIOR_SIZE * 2;
const STRIDE_COST_SIZE: usize = 256 * NUM_SPEEDS_TO_TRY * 2;
const SPEEDS_TO_SEARCH: [u16; NUM_SPEEDS_TO_TRY]= [0,
                                                   1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                                                   2,
                                                   2,
                                                   3,
                                                   3,
                                                   4,
                                                   4,
                                                   5,
                                                   6,
                                                   10,
                                                   10,
                                                   12,
                                                   16,
                                                   24,
                                                   32,
                                                   48,
                                                   64,
                                                   96,
                                                   768,
                                                   1024,
                                                   1280,
                                                   1664,
                                                   ];
const MAXES_TO_SEARCH: [u16; NUM_SPEEDS_TO_TRY] = [32,
                                                   32, 64, 128, 256, 384, 512, 1024, 2048, 4096, 16384,
                                                   512,
                                                   4096,
                                                   512,
                                                   2048,
                                                   512,
                                                   2048,
                                                   8192,
                                                   2048,
                                                   2048,
                                                   4096,
                                                   4096,
                                                   8192,
                                                   16834,
                                                   16384,
                                                   16384,
                                                   16384,
                                                   16384,
                                                   16384,
                                                   16384,
                                                   16384,
                                                   16384,
                                                   ];
#[derive(Clone,Copy, Debug)]
pub struct SpeedAndMax(pub u16, pub u16);
fn get_combined_stride_cost(data: &mut [floatX], cm_prior: usize, is_high_nibble: bool) -> &mut [floatX] {
    //let index: usize = 2 * stride_prior as usize;
    let index: usize = (is_high_nibble as usize) | (cm_prior << 1);
    data.split_at_mut(index * NUM_SPEEDS_TO_TRY).1.split_at_mut(NUM_SPEEDS_TO_TRY).0
}

fn get_stride_cost_low(data: &mut [floatX], stride_prior: u8, high_nibble: u8) -> &mut [floatX] {
    let index: usize = 1 + 2 * (stride_prior as usize & 0xf) + 2 * 16 * high_nibble as usize;
    data.split_at_mut(index * NUM_SPEEDS_TO_TRY).1.split_at_mut(NUM_SPEEDS_TO_TRY).0
}

fn get_stride_cost_high(data: &mut [floatX], stride_prior: u8) -> &mut [floatX] {
    let index: usize = 2 * stride_prior as usize;
    data.split_at_mut(index * NUM_SPEEDS_TO_TRY).1.split_at_mut(NUM_SPEEDS_TO_TRY).0
}

fn get_cm_cost(data: &mut [floatX], cm_prior: usize, is_high_nibble: bool) -> &mut [floatX] {
    let index = ((is_high_nibble as usize) | ((cm_prior as usize) << 1));
    data.split_at_mut(index * NUM_SPEEDS_TO_TRY).1.split_at_mut(NUM_SPEEDS_TO_TRY).0
}

fn get_stride_cdf_low(data: &mut [u16], stride_prior: u8, cm_prior: usize, high_nibble: u8) -> &mut [u16] {
    let index: usize =  1 + 2 * (cm_prior as usize | ((stride_prior as usize & 0xf) << 8) | ((high_nibble as usize) << 12));
    data.split_at_mut(NUM_SPEEDS_TO_TRY * index << 4).1.split_at_mut(16 * NUM_SPEEDS_TO_TRY).0
}

fn get_stride_cdf_high(data: &mut [u16], stride_prior: u8, cm_prior: usize) -> &mut [u16] {
    let index: usize = 2 * (cm_prior as usize | ((stride_prior as usize) << 8));
    data.split_at_mut(NUM_SPEEDS_TO_TRY * index << 4).1.split_at_mut(16 * NUM_SPEEDS_TO_TRY).0
}

fn get_cm_cdf_low(data: &mut [u16], cm_prior: usize, high_nibble: u8) -> &mut [u16] {
    let index: usize = (high_nibble as usize + 1) + 17 * cm_prior as usize;
    data.split_at_mut(NUM_SPEEDS_TO_TRY * index << 4).1.split_at_mut(16 * NUM_SPEEDS_TO_TRY).0
}

fn get_cm_cdf_high(data: &mut [u16], cm_prior: usize) -> &mut [u16] {
    let index: usize = 17 * cm_prior as usize;
    data.split_at_mut(NUM_SPEEDS_TO_TRY * index << 4).1.split_at_mut(16 * NUM_SPEEDS_TO_TRY).0
}
fn init_cdfs(cdfs: &mut [u16]) {
    assert_eq!(cdfs.len() % (16 * NUM_SPEEDS_TO_TRY), 0);
    let mut total_index = 0usize;
    let len = cdfs.len();
    loop {
        for cdf_index in 0..16 {
            let mut vec = cdfs.split_at_mut(total_index).1.split_at_mut(NUM_SPEEDS_TO_TRY).0;
            for item in vec {
                *item = 1 + cdf_index as u16;
            }
            total_index += NUM_SPEEDS_TO_TRY;
        }
        if total_index == len {
            break;
        }
    }
}
fn compute_combined_cost(cost: &mut [floatX],
                cdfs: &[u16],
                mixing_cdf: [u16;16],
                nibble_u8: u8,
                _weights: &mut [Weights; NUM_SPEEDS_TO_TRY]) {
    assert_eq!(cost.len(), NUM_SPEEDS_TO_TRY);
    assert_eq!(cdfs.len(), 16 * NUM_SPEEDS_TO_TRY);
    let nibble = nibble_u8 as usize & 0xf;
    let mut stride_pdf = [0u16; NUM_SPEEDS_TO_TRY];
    stride_pdf.clone_from_slice(cdfs.split_at(NUM_SPEEDS_TO_TRY * nibble).1.split_at(NUM_SPEEDS_TO_TRY).0);
    let mut cm_pdf:u16 = mixing_cdf[nibble];
    if nibble_u8 != 0 {
        let mut tmp = [0u16; NUM_SPEEDS_TO_TRY];
        tmp.clone_from_slice(cdfs.split_at(NUM_SPEEDS_TO_TRY * (nibble - 1)).1.split_at(NUM_SPEEDS_TO_TRY).0);
        for i in 0..NUM_SPEEDS_TO_TRY {
            stride_pdf[i] -= tmp[i];
        }
        cm_pdf -= mixing_cdf[nibble - 1]
    }
    let mut stride_max = [0u16; NUM_SPEEDS_TO_TRY];
    stride_max.clone_from_slice(cdfs.split_at(NUM_SPEEDS_TO_TRY * 15).1);
    let cm_max = mixing_cdf[15];
    for i in 0..NUM_SPEEDS_TO_TRY {
        if stride_pdf[i] == 0 { 
            assert!(stride_pdf[i] != 0);
        }
        if stride_max[i] == 0 {
            assert!(stride_max[i] != 0);
        }
        let w;
        w = (1<<(BLEND_FIXED_POINT_PRECISION - 2)) ; // a quarter of weight to stride
        let combined_pdf = w * u32::from(stride_pdf[i]) + ((1<<BLEND_FIXED_POINT_PRECISION) - w) * u32::from(cm_pdf);
        let combined_max = w * u32::from(stride_max[i]) + ((1<<BLEND_FIXED_POINT_PRECISION) - w) * u32::from(cm_max);
        cost[i] -= FastLog2u16((combined_pdf >> BLEND_FIXED_POINT_PRECISION) as u16) - FastLog2u16((combined_max >> BLEND_FIXED_POINT_PRECISION) as u16);
    }
}
fn compute_cost(cost: &mut [floatX],
                cdfs: &[u16],
                nibble_u8: u8) {
    assert_eq!(cost.len(), NUM_SPEEDS_TO_TRY);
    assert_eq!(cdfs.len(), 16 * NUM_SPEEDS_TO_TRY);
    let nibble = nibble_u8 as usize & 0xf;
    let mut pdf = [0u16; NUM_SPEEDS_TO_TRY];
    pdf.clone_from_slice(cdfs.split_at(NUM_SPEEDS_TO_TRY * nibble).1.split_at(NUM_SPEEDS_TO_TRY).0);
    if nibble_u8 != 0 {
        let mut tmp = [0u16; NUM_SPEEDS_TO_TRY];
        tmp.clone_from_slice(cdfs.split_at(NUM_SPEEDS_TO_TRY * (nibble - 1)).1.split_at(NUM_SPEEDS_TO_TRY).0);
        for i in 0..NUM_SPEEDS_TO_TRY {
            pdf[i] -= tmp[i];
        }
    }
    let mut max = [0u16; NUM_SPEEDS_TO_TRY];
    max.clone_from_slice(cdfs.split_at(NUM_SPEEDS_TO_TRY * 15).1);
    for i in 0..NUM_SPEEDS_TO_TRY {
        if pdf[i] == 0 { 
            assert!(pdf[i] != 0);
        }
        if max[i] == 0 {
            assert!(max[i] != 0);
        }
        cost[i] -= FastLog2u16(pdf[i]) - FastLog2u16(max[i]);
    }
}
fn update_cdf(cdfs: &mut [u16],
              nibble_u8: u8) {
    assert_eq!(cdfs.len(), 16 * NUM_SPEEDS_TO_TRY);
    let mut overall_index = nibble_u8 as usize * NUM_SPEEDS_TO_TRY;
    for _nibble in (nibble_u8 as usize & 0xf) .. 16 {
        for speed_index in 0..NUM_SPEEDS_TO_TRY {
            cdfs[overall_index + speed_index] += SPEEDS_TO_SEARCH[speed_index];
        }
        overall_index += NUM_SPEEDS_TO_TRY;
    }
    overall_index = 0;
    for nibble in 0 .. 16 {
        for speed_index in 0..NUM_SPEEDS_TO_TRY {
            if nibble == 0 {
                assert!(cdfs[overall_index + speed_index] != 0);
            } else {
                assert!(cdfs[overall_index + speed_index]  - cdfs[overall_index + speed_index - NUM_SPEEDS_TO_TRY]  != 0);
            }
        }
        overall_index += NUM_SPEEDS_TO_TRY;
    }
    for max_index in 0..NUM_SPEEDS_TO_TRY {
        if cdfs[15 * NUM_SPEEDS_TO_TRY + max_index] >= MAXES_TO_SEARCH[max_index] {
            const CDF_BIAS:[u16;16] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
            for nibble_index in 0..16  {
                let tmp = &mut cdfs[nibble_index * NUM_SPEEDS_TO_TRY + max_index];
                *tmp = (tmp.wrapping_add(CDF_BIAS[nibble_index])).wrapping_sub(tmp.wrapping_add(CDF_BIAS[nibble_index]) >> 1);
            }
        }
    }
    overall_index = 0;
    for nibble in 0 .. 16 {
        for speed_index in 0..NUM_SPEEDS_TO_TRY {
            if nibble == 0 {
                assert!(cdfs[overall_index + speed_index] != 0);
            } else {
                assert!(cdfs[overall_index + speed_index]  - cdfs[overall_index + speed_index - NUM_SPEEDS_TO_TRY]  != 0);
            }
        }
        overall_index += NUM_SPEEDS_TO_TRY;
    }
}

fn extract_single_cdf(cdf_bundle:&[u16], index:usize) -> [u16;16] {
    assert_eq!(cdf_bundle.len(), 16 * NUM_SPEEDS_TO_TRY);
    assert!(index < NUM_SPEEDS_TO_TRY);
    [
        cdf_bundle[index + 0 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 1 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 2 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 3 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 4 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 5 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 6 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 7 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 8 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 9 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 10 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 11 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 12 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 13 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 14 * NUM_SPEEDS_TO_TRY],
        cdf_bundle[index + 15 * NUM_SPEEDS_TO_TRY],
        ]
}

fn min_cost_index_for_speed(cost: &[floatX]) -> usize {
    assert_eq!(cost.len(), NUM_SPEEDS_TO_TRY);
    let mut min_cost = cost[0];
    let mut best_choice = 0;
    for i in 1..NUM_SPEEDS_TO_TRY {
        if cost[i] < min_cost {
            best_choice = i;
            min_cost = cost[i];
        }
    }
    best_choice
}
fn min_cost_speed_max(cost: &[floatX]) -> SpeedAndMax {
    let best_choice = min_cost_index_for_speed(cost);
    SpeedAndMax(
        SPEEDS_TO_SEARCH[best_choice],
        MAXES_TO_SEARCH[best_choice])
}

fn min_cost_value(cost: &[floatX]) -> floatX {
    let best_choice = min_cost_index_for_speed(cost);
    cost[best_choice]
}

    
pub struct ContextMapEntropy<'a,
                             AllocU16:alloc::Allocator<u16>,
                             AllocU32:alloc::Allocator<u32>,
                             AllocF:alloc::Allocator<floatX>,
                             > {
    input: InputPair<'a>,
    context_map: interface::PredictionModeContextMap<InputReference<'a>>,
    block_type: u8,
    local_byte_offset: usize,
    weight: [[Weights; NUM_SPEEDS_TO_TRY];2],
    _nop: AllocU32::AllocatedMemory,
    
    cm_priors: AllocU16::AllocatedMemory,
    stride_priors: AllocU16::AllocatedMemory,
    cm_cost: AllocF::AllocatedMemory,
    stride_cost: AllocF::AllocatedMemory,
    combined_stride_cost: AllocF::AllocatedMemory,
    stride_pyramid_leaves: [u8; find_stride::NUM_LEAF_NODES],
}
impl<'a,
     AllocU16:alloc::Allocator<u16>,
     AllocU32:alloc::Allocator<u32>,
     AllocF:alloc::Allocator<floatX>,
     > ContextMapEntropy<'a, AllocU16, AllocU32, AllocF> {
   pub fn new(m16: &mut AllocU16,
              _m32: &mut AllocU32,
              mf: &mut AllocF,
              input: InputPair<'a>,
              stride: [u8; find_stride::NUM_LEAF_NODES],
              prediction_mode: interface::PredictionModeContextMap<InputReference<'a>>) -> Self {
       
      let mut ret = ContextMapEntropy::<AllocU16, AllocU32, AllocF>{
         input: input,
         context_map: prediction_mode,
         block_type: 0,
         local_byte_offset: 0,
         _nop:  AllocU32::AllocatedMemory::default(),
         cm_priors: m16.alloc_cell(CONTEXT_MAP_PRIOR_SIZE),
         stride_priors: m16.alloc_cell(STRIDE_PRIOR_SIZE),
         cm_cost: mf.alloc_cell(CONTEXT_MAP_COST_SIZE),
         stride_cost: mf.alloc_cell(STRIDE_COST_SIZE),
         combined_stride_cost: mf.alloc_cell(STRIDE_COST_SIZE),
         stride_pyramid_leaves: stride,
         weight:[[Weights::new(); NUM_SPEEDS_TO_TRY],
                 [Weights::new(); NUM_SPEEDS_TO_TRY]],
      };
      init_cdfs(ret.cm_priors.slice_mut());
      init_cdfs(ret.stride_priors.slice_mut());
      ret
   }
   #[inline]
   pub fn track_cdf_speed(&mut self,
                      _data: &[u8],
                      mut _prev_byte: u8, mut _prev_prev_byte: u8,
                          _block_type: u8) {
       /*
       scratch.bucket_populations.slice_mut().clone_from_slice(self.entropy_tally.bucket_populations.slice());
       scratch.bucket_populations.slice_mut()[65535] += 1; // to demonstrate that we have
       scratch.bucket_populations.slice_mut()[65535] -= 1; // to demonstrate that we have write capability
       let mut stray_count = 0 as find_stride::floatY;
       for val in data.iter() {
           let huffman_table_index = compute_huffman_table_index_for_context_map(prev_byte, prev_prev_byte, self.context_map, block_type);
           let loc = &mut scratch.bucket_populations.slice_mut()[huffman_table_index * 256 + *val as usize];
           //let mut stray = false;
           if *loc == 0 {
               stray_count += 1.0;
               //stray = true;
           } else {
               *loc -= 1;
           }
           //println!("{} {:02x}{:02x} => {:02x} (bt: {}, ind: {}, cnt: {})", if stray {"S"} else {"L"}, prev_byte, prev_prev_byte, *val, block_type, huffman_table_index, *loc);
           prev_prev_byte = prev_byte;
           prev_byte = *val;
       }
       if self.entropy_tally.cached_bit_entropy == 0.0 as find_stride::floatY {
           self.entropy_tally.cached_bit_entropy = find_stride::HuffmanCost(self.entropy_tally.bucket_populations.slice());
       }
       debug_assert_eq!(find_stride::HuffmanCost(self.entropy_tally.bucket_populations.slice()),
                        self.entropy_tally.cached_bit_entropy);

       scratch.cached_bit_entropy = find_stride::HuffmanCost(scratch.bucket_populations.slice());
       self.entropy_tally.cached_bit_entropy - scratch.cached_bit_entropy + stray_count * 8.0
*/
   }
    pub fn best_speeds(&mut self, // mut due to helpers
                       cm:bool,
                       combined: bool) -> [[SpeedAndMax;2]; 256] { 
       let mut ret = [[SpeedAndMax(SPEEDS_TO_SEARCH[0],MAXES_TO_SEARCH[0]); 2]; 256];
       if cm && !combined{
           for prior in 0..256 {
               for high in 0..2 {
                   let cost = get_cm_cost(self.cm_cost.slice_mut(), prior, high != 0);
                   ret[prior][high] = min_cost_speed_max(cost);
               }
           }
       } else {
           for stride_prior in 0..256 {
               for high in 0..2 {
                   let cost;
                   if combined {
                       cost = get_combined_stride_cost(self.combined_stride_cost.slice_mut(), stride_prior as usize, high != 0);
                   } else {
                       if high == 1 {
                           cost = get_stride_cost_high(self.stride_cost.slice_mut(), stride_prior as u8);
                       } else {
                           cost = get_stride_cost_low(self.stride_cost.slice_mut(), stride_prior as u8 & 0xf, stride_prior as u8 >> 4);
                       }
                   }
                   ret[stride_prior][high] = min_cost_speed_max(cost);
                   if combined && (ret[stride_prior][high].0 == 0 || ret[stride_prior][high].1 == 0) {
                       // make sure no nonzeros
                       //ret[stride_prior][high] = SpeedAndMax(SPEEDS_TO_SEARCH[1], MAXES_TO_SEARCH[1]);
                   }
               }
           }
       }
       ret
   }
   pub fn best_speeds_costs(&mut self, // mut due to helpers
                            cm:bool,
                            combined: bool) -> [[floatX;2]; 256] { 
       let mut ret = [[0.0 as floatX; 2]; 256];
       if cm && !combined {
           for prior in 0..256 {
               for high in 0..2 {
                   let cost = get_cm_cost(self.cm_cost.slice_mut(), prior, high != 0);
                   ret[prior][high] = min_cost_value(cost);
               }
           }
       } else {
           for stride_prior in 0..256 {
               for high in 0..2 {
                   let cost;
                   if combined {
                       cost = get_combined_stride_cost(self.combined_stride_cost.slice_mut(), stride_prior as usize, high != 0);
                   } else {
                       if high == 1 {
                           cost = get_stride_cost_high(self.stride_cost.slice_mut(), stride_prior as u8);
                       } else {
                           cost = get_stride_cost_low(self.stride_cost.slice_mut(), stride_prior as u8 & 0xf, stride_prior as u8 >> 4);
                       }
                   }
                   ret[stride_prior][high] = min_cost_value(cost);
               }
           }
       }
       ret
   }
   pub fn free(&mut self, m16: &mut AllocU16, _m32: &mut AllocU32, mf64: &mut AllocF) {
        m16.free_cell(core::mem::replace(&mut self.cm_priors, AllocU16::AllocatedMemory::default()));
        m16.free_cell(core::mem::replace(&mut self.stride_priors, AllocU16::AllocatedMemory::default()));
        mf64.free_cell(core::mem::replace(&mut self.cm_cost, AllocF::AllocatedMemory::default()));
        mf64.free_cell(core::mem::replace(&mut self.stride_cost, AllocF::AllocatedMemory::default()));
        mf64.free_cell(core::mem::replace(&mut self.combined_stride_cost, AllocF::AllocatedMemory::default()));
   }
   fn update_cost(&mut self, stride_prior: u8, cm_prior: usize, literal: u8) {
       let upper_nibble = (literal >> 4);
       let lower_nibble = literal & 0xf;
       let provisional_cm_high_cdf: [u16; 16];
       let provisional_cm_low_cdf: [u16; 16];
       {
           let cm_cdf_high = get_cm_cdf_high(self.cm_priors.slice_mut(), cm_prior);
           let cm_high_cost = get_cm_cost(self.cm_cost.slice_mut(), cm_prior, true);
           compute_cost(cm_high_cost, cm_cdf_high, upper_nibble);
           let best_cm_index = min_cost_index_for_speed(cm_high_cost);
           provisional_cm_high_cdf = extract_single_cdf(cm_cdf_high, best_cm_index);
       }
       {
           let cm_cdf_low = get_cm_cdf_low(self.cm_priors.slice_mut(), cm_prior, upper_nibble);
           let cm_low_cost = get_cm_cost(self.cm_cost.slice_mut(), cm_prior, false);
           compute_cost(cm_low_cost, cm_cdf_low, lower_nibble);
           let best_cm_index = min_cost_index_for_speed(cm_low_cost);
           provisional_cm_low_cdf = extract_single_cdf(cm_cdf_low, best_cm_index);
       }
       {
           let stride_cdf_high = get_stride_cdf_high(self.stride_priors.slice_mut(), stride_prior, cm_prior);
           compute_combined_cost(get_combined_stride_cost(self.combined_stride_cost.slice_mut(), cm_prior, true), stride_cdf_high, provisional_cm_high_cdf, upper_nibble, &mut self.weight[1]);
           compute_cost(get_stride_cost_high(self.stride_cost.slice_mut(), stride_prior), stride_cdf_high, upper_nibble);
           update_cdf(stride_cdf_high, upper_nibble);
       }
       {
           let stride_cdf_low = get_stride_cdf_low(self.stride_priors.slice_mut(), stride_prior, cm_prior, upper_nibble);
           compute_combined_cost(get_combined_stride_cost(self.combined_stride_cost.slice_mut(), cm_prior, false), stride_cdf_low, provisional_cm_low_cdf, lower_nibble, &mut self.weight[0]);
           compute_cost(get_stride_cost_low(self.stride_cost.slice_mut(), stride_prior, upper_nibble), stride_cdf_low, lower_nibble);
           update_cdf(stride_cdf_low, lower_nibble);
       }
       {
           let cm_cdf_high = get_cm_cdf_high(self.cm_priors.slice_mut(), cm_prior);
           update_cdf(cm_cdf_high, upper_nibble);
       }
       {
           let cm_cdf_low = get_cm_cdf_low(self.cm_priors.slice_mut(), cm_prior, upper_nibble);
           update_cdf(cm_cdf_low, lower_nibble);
       }
   }
}
fn Context(p1: u8, p2: u8, mode: ContextType) -> u8 {
  match mode {
    ContextType::CONTEXT_LSB6 => {
      return (p1 as (i32) & 0x3fi32) as (u8);
    }
    ContextType::CONTEXT_MSB6 => {
      return (p1 as (i32) >> 2i32) as (u8);
    }
    ContextType::CONTEXT_UTF8 => {
      return (kUTF8ContextLookup[p1 as (usize)] as (i32) |
              kUTF8ContextLookup[(p2 as (i32) + 256i32) as (usize)] as (i32)) as (u8);
    }
    ContextType::CONTEXT_SIGNED => {
      return ((kSigned3BitContextLookup[p1 as (usize)] as (i32) << 3i32) +
              kSigned3BitContextLookup[p2 as (usize)] as (i32)) as (u8);
    }
  }
  //  0i32 as (u8)
}

fn compute_huffman_table_index_for_context_map<SliceType: alloc::SliceWrapper<u8> > (
    prev_byte: u8,
    prev_prev_byte: u8,
    context_map: interface::PredictionModeContextMap<SliceType>,
    block_type: u8,
) -> usize {
    let prior = Context(prev_byte, prev_prev_byte, context_map.literal_prediction_mode.to_context_enum().unwrap());
    assert!(prior < 64);
    let context_map_index = ((block_type as usize)<< 6) | prior as usize;
    if context_map_index < context_map.literal_context_map.slice().len() {
        context_map.literal_context_map.slice()[context_map_index] as usize
    } else {
        prior as usize
    }
}



impl<'a, 'b, AllocU16: alloc::Allocator<u16>,
     AllocU32:alloc::Allocator<u32>,
     AllocF: alloc::Allocator<floatX>> interface::CommandProcessor<'b> for ContextMapEntropy<'a, AllocU16, AllocU32, AllocF> {
    fn push<Cb: FnMut(&[interface::Command<InputReference>])>(&mut self,
                                                              val: interface::Command<InputReference<'b>>,
                                                              callback: &mut Cb) {
        match val {
           interface::Command::BlockSwitchCommand(_) |
           interface::Command::BlockSwitchDistance(_) |
           interface::Command::PredictionMode(_) => {}
           interface::Command::Copy(ref copy) => {
             self.local_byte_offset += copy.num_bytes as usize;
           },
           interface::Command::Dict(ref dict) => {
             self.local_byte_offset += dict.final_size as usize;
           },
           interface::Command::BlockSwitchLiteral(block_type) => self.block_type = block_type.block_type(),
           interface::Command::Literal(ref lit) => {
               let stride = self.stride_pyramid_leaves[self.local_byte_offset * 8 / self.input.len()] as usize;
               let mut priors= [0u8; 8];
               for poffset in 0..core::cmp::max((stride & 7) + 1, 2) {
                   if self.local_byte_offset > poffset {
                       priors[7 - poffset] = self.input[self.local_byte_offset - poffset -  1];
                   }
               }
               let mut cur = 0usize;
               for literal in lit.data.slice().iter() {
                   let huffman_table_index = compute_huffman_table_index_for_context_map(priors[(cur + 7)&7], priors[(cur + 6) &7], self.context_map, self.block_type);
                   self.update_cost(priors[(cur + 7 - stride) & 7], huffman_table_index, *literal);
                   // FIXME..... self.entropy_tally.bucket_populations.slice_mut()[((huffman_table_index as usize) << 8) | *literal as usize] += 1;
                    //println!("I {:02x}{:02x} => {:02x} (bt: {}, ind: {} cnt: {})", priors[1], priors[0], *literal, self.block_type, huffman_table_index, self.entropy_tally.bucket_populations.slice_mut()[((huffman_table_index as usize) << 8) | *literal as usize]);
                   priors[cur & 7] = *literal;
                   cur += 1;
                   cur &= 7;
               }
               self.local_byte_offset += lit.data.slice().len();
           }
        }
        let cbval = [val];
        callback(&cbval[..]);
    }
}
