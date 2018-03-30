use core;
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::interface;
use super::input_pair::{InputPair, InputReference, InputReferenceMut};
use super::ir_interpret::{IRInterpreter};
use super::histogram::ContextType;
use super::constants::{kSigned3BitContextLookup, kUTF8ContextLookup};
use super::util::{floatX, FastLog2u16};
use super::find_stride;
use super::weights::{Weights, BLEND_FIXED_POINT_PRECISION};

use super::context_map_entropy::SpeedAndMax;

const NUM_PRIORS_TO_TRY: usize = 2;
const NIBBLE_PRIOR_SIZE: usize = 16;
// the high nibble, followed by the low nibbles
const CONTEXT_MAP_PRIOR_SIZE: usize = 256 * NIBBLE_PRIOR_SIZE * 17;
const STRIDE_PRIOR_SIZE: usize = 256 * 256 * NIBBLE_PRIOR_SIZE * 2;
const PRIOR_SIZES : [usize;2] = [CONTEXT_MAP_PRIOR_SIZE, STRIDE_PRIOR_SIZE];
pub trait Prior {
    fn lookup_lin(actual_context:u8, selected_context:u8, stride_byte: u8, high_nibble: Option<u8>) -> usize;
    fn lookup_mut(data:&mut [u8], actual_context:u8, selected_context:u8, stride_byte: u8, high_nibble: Option<u8>) -> &mut[u8] {
        data.split_at_mut(
            Self::lookup_lin(actual_context,
                             selected_context,
                             stride_byte,
                             high_nibble) * NIBBLE_PRIOR_SIZE).1.split_at_mut(16).0
    }
    fn lookup(data:&[u8], actual_context:u8, selected_context:u8, stride_byte: u8, high_nibble: Option<u8>) -> &[u8] {
        data.split_at(
            Self::lookup_lin(actual_context,
                             selected_context,
                             stride_byte,
                             high_nibble) * NIBBLE_PRIOR_SIZE).1.split_at(16).0
    }
}

pub struct StridePrior {
}
impl Prior for StridePrior {
    fn lookup_lin(actual_context:u8, selected_context:u8, stride_byte: u8, high_nibble: Option<u8>) -> usize {
        if let Some(nibble) = high_nibble {
            1 + 2 * (actual_context as usize
                     | ((stride_byte as usize & 0xf) << 8)
                     | ((nibble as usize) << 12))
        } else {
            2 * (actual_context as usize | ((stride_byte as usize) << 8))
        }
    }
}

pub struct CMPrior {
}
impl Prior for CMPrior {
    fn lookup_lin(actual_context:u8, selected_context:u8, stride_byte: u8, high_nibble: Option<u8>) -> usize {
        if let Some(nibble) = high_nibble {
            (nibble as usize + 1) + 17 * actual_context as usize
        } else {
            17 * actual_context as usize
        }
    }
}


fn init_cdfs(cdfs: &mut [u16]) {
    assert_eq!(cdfs.len() % 16, 0);
    for (index, item) in cdfs.iter_mut().enumerate() {
        *item = 4 + 4 * (index as u16 & 0xf);
    }
}

pub struct PriorEval<'a,
                     AllocU16:alloc::Allocator<u16>,
                     AllocU32:alloc::Allocator<u32>,
                     AllocF:alloc::Allocator<floatX>,
                     > {
    input: InputPair<'a>,
    context_map: interface::PredictionModeContextMap<InputReferenceMut<'a>>,
    block_type: u8,
    local_byte_offset: usize,
    _nop: AllocU32::AllocatedMemory,    
    priors: [AllocU16::AllocatedMemory; NUM_PRIORS_TO_TRY],
    stride_pyramid_leaves: [u8; find_stride::NUM_LEAF_NODES],
    phantom: core::marker::PhantomData<AllocF>,
}

impl<'a,
     AllocU16:alloc::Allocator<u16>,
     AllocU32:alloc::Allocator<u32>,
     AllocF:alloc::Allocator<floatX>,
     > PriorEval<'a, AllocU16, AllocU32, AllocF> {
   pub fn new(m16: &mut AllocU16,
              _m32: &mut AllocU32,
              _mf: &mut AllocF,
              input: InputPair<'a>,
              stride: [u8; find_stride::NUM_LEAF_NODES],
              prediction_mode: interface::PredictionModeContextMap<InputReferenceMut<'a>>) -> Self {
      let mut ret = PriorEval::<AllocU16, AllocU32, AllocF>{
         phantom:core::marker::PhantomData::<AllocF>::default(),
         input: input,
         context_map: prediction_mode,
         block_type: 0,
         local_byte_offset: 0,
         _nop:  AllocU32::AllocatedMemory::default(),
         priors: [m16.alloc_cell(PRIOR_SIZES[0]), m16.alloc_cell(PRIOR_SIZES[1])],
         stride_pyramid_leaves: stride,
      };
      for prior in ret.priors.iter_mut() {
          init_cdfs(prior.slice_mut());
      }
      ret
   }
   fn update_cost_base(&mut self, stride_prior: u8, cm_prior: usize, literal: u8) {
   }
}
impl<'a, AllocU16: alloc::Allocator<u16>,
     AllocU32:alloc::Allocator<u32>,
     AllocF: alloc::Allocator<floatX>> IRInterpreter for PriorEval<'a, AllocU16, AllocU32, AllocF> {
    fn inc_local_byte_offset(&mut self, inc: usize) {
        self.local_byte_offset += inc;
    }
    fn get_stride(&self, local_byte_offset: usize) -> u8 {
        self.stride_pyramid_leaves[local_byte_offset * 8 / self.input.len()]
    }
    fn local_byte_offset(&self) -> usize {
        self.local_byte_offset
    }
    fn update_block_type(&mut self, new_type: u8) {
        self.block_type = new_type;
    }
    fn block_type(&self) -> u8 {
        self.block_type
    }
    fn literal_data_at_offset(&self, index:usize) -> u8 {
        self.input[index]
    }
    fn literal_context_map(&self) -> &[u8] {
        self.context_map.literal_context_map.slice()
    }
    fn prediction_mode(&self) -> ::interface::LiteralPredictionModeNibble {
        self.context_map.literal_prediction_mode()
    }
    fn update_cost(&mut self, stride_prior: u8, cm_prior: usize, literal: u8) {
        self.update_cost_base(stride_prior, cm_prior, literal)
    }
}


