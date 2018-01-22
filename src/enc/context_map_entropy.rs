use core;
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::interface;
use super::input_pair::{InputPair, InputReference};
use super::histogram::ContextType;
use super::constants::{kSigned3BitContextLookup, kUTF8ContextLookup};
use super::util::floatX;
const NUM_SPEEDS_TO_TRY: usize = 32;
const NIBBLE_PRIOR_SIZE: usize = 16 * NUM_SPEEDS_TO_TRY;
// the high nibble, followed by the low nibbles
const CONTEXT_MAP_PRIOR_SIZE: usize = 256 * NIBBLE_PRIOR_SIZE * 17;
const CONTEXT_MAP_COST_SIZE: usize = 256 * NUM_SPEEDS_TO_TRY * 2;
const STRIDE_PRIOR_SIZE: usize = 256 * 256 * NIBBLE_PRIOR_SIZE * 2;
const STRIDE_COST_SIZE: usize = 256 * NUM_SPEEDS_TO_TRY * 2;
fn get_stride_cost_high(data: &mut [floatX], stride_prior: u8, cm_prior: usize) -> &mut [floatX] {
    let index: usize = 2 * stride_prior as usize;
    data.split_at_mut(index * NUM_SPEEDS_TO_TRY).1.split_at_mut(NUM_SPEEDS_TO_TRY).0
}

fn get_stride_cost_low(data: &mut [floatX], stride_prior: u8, cm_prior: usize, high_nibble: u8) -> &mut [floatX] {
    let index: usize = 1 + 2 * (stride_prior as usize & 0xf) + 2 * 16 * high_nibble as usize;// (is_high_nibble as usize) + 2 * (cm_prior as usize | ((stride_prior as usize & 0xf) << 8));
    data.split_at_mut(index * NUM_SPEEDS_TO_TRY).1.split_at_mut(NUM_SPEEDS_TO_TRY).0
}

fn get_cm_cost(data: &mut [floatX], cm_prior: usize, is_high_nibble: bool) -> &mut [floatX] {
    let index = ((is_high_nibble as usize) | ((cm_prior as usize) << 9));
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

fn compute_cost(cost: &mut [floatX],
                cdfs: &[u16],
                nibble_u8: u8) {
    assert_eq!(cost.len(), NUM_SPEEDS_TO_TRY);
    assert_eq!(cdfs.len(), 16 * NUM_SPEEDS_TO_TRY);
    let nibble = nibble_u8 as usize & 0xf;
    
}
fn update_cdf(cdfs: &mut [u16],
              nibble_u8: u8) {
    assert_eq!(cdfs.len(), 16 * NUM_SPEEDS_TO_TRY);
    let nibble = nibble_u8 as usize & 0xf;
    
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
    nop: AllocU32::AllocatedMemory,
    
    cm_priors: AllocU16::AllocatedMemory,
    stride_priors: AllocU16::AllocatedMemory,
    cm_cost: AllocF::AllocatedMemory,
    stride_cost: AllocF::AllocatedMemory,
}
impl<'a,
     AllocU16:alloc::Allocator<u16>,
     AllocU32:alloc::Allocator<u32>,
     AllocF:alloc::Allocator<floatX>,
     > ContextMapEntropy<'a, AllocU16, AllocU32, AllocF> {
   pub fn new(m16: &mut AllocU16,
              m32: &mut AllocU32,
              mf: &mut AllocF,
              input: InputPair<'a>, prediction_mode: interface::PredictionModeContextMap<InputReference<'a>>) -> Self {
       
      ContextMapEntropy::<AllocU16, AllocU32, AllocF>{
         input: input,
         context_map: prediction_mode,
         block_type: 0,
         local_byte_offset: 0,
         nop:  AllocU32::AllocatedMemory::default(),
         cm_priors: m16.alloc_cell(CONTEXT_MAP_PRIOR_SIZE),
         stride_priors: m16.alloc_cell(STRIDE_PRIOR_SIZE),
         cm_cost: mf.alloc_cell(CONTEXT_MAP_COST_SIZE),
         stride_cost: mf.alloc_cell(STRIDE_COST_SIZE),
      }
   }
   pub fn track_cdf_speed(&mut self,
                      data: &[u8],
                      mut prev_byte: u8, mut prev_prev_byte: u8,
                      block_type: u8) {
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
   pub fn free(&mut self, m16: &mut AllocU16, _m32: &mut AllocU32, mf64: &mut AllocF) {
        m16.free_cell(core::mem::replace(&mut self.cm_priors, AllocU16::AllocatedMemory::default()));
        m16.free_cell(core::mem::replace(&mut self.stride_priors, AllocU16::AllocatedMemory::default()));
        mf64.free_cell(core::mem::replace(&mut self.cm_cost, AllocF::AllocatedMemory::default()));
        mf64.free_cell(core::mem::replace(&mut self.stride_cost, AllocF::AllocatedMemory::default()));
   }
   fn update_cost(&mut self, stride_prior: u8, cm_prior: usize, literal: u8) {
       let upper_nibble = (literal >> 4);
       let lower_nibble = literal & 0xf;
       {
           let stride_cdf_high = get_stride_cdf_high(self.stride_priors.slice_mut(), stride_prior, cm_prior);
           compute_cost(get_stride_cost_high(self.stride_cost.slice_mut(), stride_prior, cm_prior), stride_cdf_high, upper_nibble);
           update_cdf(stride_cdf_high, upper_nibble);
       }
       {
           let stride_cdf_low = get_stride_cdf_low(self.stride_priors.slice_mut(), stride_prior, cm_prior, upper_nibble);
           compute_cost(get_stride_cost_low(self.stride_cost.slice_mut(), stride_prior, cm_prior, upper_nibble), stride_cdf_low, lower_nibble);
           update_cdf(stride_cdf_low, lower_nibble);
       }
       {
           let cm_cdf_high = get_cm_cdf_high(self.stride_priors.slice_mut(), cm_prior);
           compute_cost(get_cm_cost(self.cm_cost.slice_mut(), cm_prior, true), cm_cdf_high, upper_nibble);
           update_cdf(cm_cdf_high, upper_nibble);
       }
       {
           let cm_cdf_low = get_cm_cdf_low(self.cm_priors.slice_mut(), cm_prior, upper_nibble);
           compute_cost(get_cm_cost(self.cm_cost.slice_mut(), cm_prior, false), cm_cdf_low, lower_nibble);
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
               let mut priors= [0u8, 0u8];
               if self.local_byte_offset > 1 {
                   priors[0] = self.input[self.local_byte_offset - 2];
                   priors[1] = self.input[self.local_byte_offset - 1];
               }
               for literal in lit.data.slice().iter() {                   
                   let huffman_table_index = compute_huffman_table_index_for_context_map(priors[1], priors[0], self.context_map, self.block_type);
                   self.update_cost(priors[1], huffman_table_index, *literal);
                   // FIXME..... self.entropy_tally.bucket_populations.slice_mut()[((huffman_table_index as usize) << 8) | *literal as usize] += 1;
                    //println!("I {:02x}{:02x} => {:02x} (bt: {}, ind: {} cnt: {})", priors[1], priors[0], *literal, self.block_type, huffman_table_index, self.entropy_tally.bucket_populations.slice_mut()[((huffman_table_index as usize) << 8) | *literal as usize]);
                   priors[0] = priors[1];
                   priors[1] = *literal;
               }
               self.local_byte_offset += lit.data.slice().len();
           }
        }
        let cbval = [val];
        callback(&cbval[..]);
    }
}
