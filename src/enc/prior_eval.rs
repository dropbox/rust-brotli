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
    fn lookup_lin(stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize;
    fn lookup_mut(data:&mut [u16], stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> &mut[u16] {
        data.split_at_mut(
            Self::lookup_lin(stride_byte, selected_context, actual_context,
                             high_nibble) * NIBBLE_PRIOR_SIZE).1.split_at_mut(16).0
    }
    fn lookup(data:&[u16], stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> &[u16] {
        data.split_at(
            Self::lookup_lin(stride_byte, selected_context, actual_context,
                             high_nibble) * NIBBLE_PRIOR_SIZE).1.split_at(16).0
    }
}

pub struct StridePrior {
}
impl Prior for StridePrior {
    fn lookup_lin(stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize {
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
    fn lookup_lin(stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize {
        if let Some(nibble) = high_nibble {
            (nibble as usize + 1) + 17 * actual_context
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
    cm_priors: AllocU16::AllocatedMemory,
    stride_priors: AllocU16::AllocatedMemory,
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
              prediction_mode: interface::PredictionModeContextMap<InputReferenceMut<'a>>,
              prior_bitmask_detection: u8) -> Self {
      let do_alloc = prior_bitmask_detection != 0;
      let mut ret = PriorEval::<AllocU16, AllocU32, AllocF>{
         phantom:core::marker::PhantomData::<AllocF>::default(),
         input: input,
         context_map: prediction_mode,
         block_type: 0,
         local_byte_offset: 0,
         _nop:  AllocU32::AllocatedMemory::default(),
         cm_priors: if do_alloc {m16.alloc_cell(PRIOR_SIZES[0])} else {AllocU16::AllocatedMemory::default()},
         stride_priors: if do_alloc {m16.alloc_cell(PRIOR_SIZES[1])} else {AllocU16::AllocatedMemory::default()},
         stride_pyramid_leaves: stride,
      };
      init_cdfs(ret.cm_priors.slice_mut());
      init_cdfs(ret.stride_priors.slice_mut());
      ret
   }
   pub fn take_prediction_mode(&mut self) -> interface::PredictionModeContextMap<InputReferenceMut<'a>> {
       core::mem::replace(&mut self.context_map, interface::PredictionModeContextMap::<InputReferenceMut<'a>>{
          literal_context_map:InputReferenceMut(&mut[]),
          predmode_speed_and_distance_context_map:InputReferenceMut(&mut[]),
       })
   }
   fn update_cost_base(&mut self, stride_prior: u8, selected_bits: u8, cm_prior: usize, literal: u8) {
       <CMPrior as Prior>::lookup_mut(self.cm_priors.slice_mut(), stride_prior, selected_bits, cm_prior, None);
       <CMPrior as Prior>::lookup_mut(self.cm_priors.slice_mut(), stride_prior, selected_bits, cm_prior, Some(literal >> 4));
       <StridePrior as Prior>::lookup_mut(self.stride_priors.slice_mut(), stride_prior, selected_bits, cm_prior, None);
       <StridePrior as Prior>::lookup_mut(self.stride_priors.slice_mut(), stride_prior, selected_bits, cm_prior, Some(literal >> 4));
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
    fn update_cost(&mut self, stride_prior: u8, selected_bits: u8, cm_prior: usize, literal: u8) {
        self.update_cost_base(stride_prior, selected_bits, cm_prior, literal)
    }
}


