use core;
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::interface;
use super::input_pair::{InputPair, InputReference, InputReferenceMut};
use super::ir_interpret::{IRInterpreter, push_base};
use super::util::{floatX, FastLog2u16};
use super::find_stride;

const NIBBLE_PRIOR_SIZE: usize = 16;
// the high nibble, followed by the low nibbles
const CONTEXT_MAP_PRIOR_SIZE: usize = 256 * NIBBLE_PRIOR_SIZE * 17;
const STRIDE_PRIOR_SIZE: usize = 256 * 256 * NIBBLE_PRIOR_SIZE * 2;
pub enum WhichPrior {
    CM = 0,
    STRIDE = 1,
    // future ideas
}

pub trait Prior {
    fn lookup_lin(stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize;
    fn lookup_mut(data:&mut [u16], stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> CDF {
        CDF::from(data.split_at_mut(
            Self::lookup_lin(stride_byte, selected_context, actual_context,
                             high_nibble) * NIBBLE_PRIOR_SIZE).1.split_at_mut(16).0)
    }
    fn lookup(data:&[u16], stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> &[u16] {
        data.split_at(
            Self::lookup_lin(stride_byte, selected_context, actual_context,
                             high_nibble) * NIBBLE_PRIOR_SIZE).1.split_at(16).0
    }
    fn which() -> WhichPrior;
}

pub struct StridePrior {
}
impl Prior for StridePrior {
    #[allow(unused_variables)]
    fn lookup_lin(stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize {
        if let Some(nibble) = high_nibble {
            1 + 2 * (actual_context as usize
                     | ((stride_byte as usize & 0xf) << 8)
                     | ((nibble as usize) << 12))
        } else {
            2 * (actual_context as usize | ((stride_byte as usize) << 8))
        }
    }
    fn which() -> WhichPrior {
        WhichPrior::STRIDE
    }
}

pub struct CMPrior {
}
impl Prior for CMPrior {
    #[allow(unused_variables)]
    fn lookup_lin(stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize {
        if let Some(nibble) = high_nibble {
            (nibble as usize + 1) + 17 * actual_context
        } else {
            17 * actual_context as usize
        }
    }
    fn which() -> WhichPrior {
        WhichPrior::CM
    }
}

pub struct CDF<'a> {
    cdf:&'a mut [u16],
}

impl<'a> CDF<'a> {
    fn cost(&self, nibble_u8:u8) -> floatX {
        assert_eq!(self.cdf.len(), 16);
        let nibble = nibble_u8 as usize & 0xf;
        let mut pdf = self.cdf[nibble];
        if nibble_u8 != 0 {
            pdf -= self.cdf[nibble - 1];
        }
        FastLog2u16(self.cdf[15]) - FastLog2u16(pdf)
    }
    fn update(&mut self, nibble_u8:u8, speed: (u16, u16)) {
        assert_eq!(self.cdf.len(), 16);
        for nib_range in (nibble_u8 as usize & 0xf) .. 16 {
            self.cdf[nib_range] += speed.0;
        }
        if self.cdf[15] >= speed.1 {
            const CDF_BIAS:[u16;16] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
            for nibble_index in 0..16  {
                let tmp = &mut self.cdf[nibble_index];
                *tmp = (tmp.wrapping_add(CDF_BIAS[nibble_index])).wrapping_sub(
                    tmp.wrapping_add(CDF_BIAS[nibble_index]) >> 2);
            }
        }
    }
}

impl<'a> From<&'a mut[u16]> for CDF<'a> {
    fn from(cdf: &'a mut[u16]) -> CDF<'a> {
        assert_eq!(cdf.len(), 16);
        CDF {
            cdf:cdf,
        }
    }
}

fn init_cdfs(cdfs: &mut [u16]) {
    for (index, item) in cdfs.iter_mut().enumerate() {
        *item = 4 + 4 * (index as u16 & 0xf);
    }
}

#[allow(unused_variables)]
fn score_index(stride_byte: u8, selected_context: u8, actual_context: usize, high_nibble: Option<u8>, which: WhichPrior) -> usize {
    if let Some(nibble) = high_nibble {
        actual_context + 256 * nibble as usize + (which as usize * 8192)
    } else {
        actual_context + 256 * (stride_byte >> 4) as usize + (which as usize * 8192)
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
    score: AllocF::AllocatedMemory,
    cm_speed: [(u16, u16);2],
    stride_speed: [(u16, u16);2],
}

impl<'a,
     AllocU16:alloc::Allocator<u16>,
     AllocU32:alloc::Allocator<u32>,
     AllocF:alloc::Allocator<floatX>,
     > PriorEval<'a, AllocU16, AllocU32, AllocF> {
   pub fn new(m16: &mut AllocU16,
              _m32: &mut AllocU32,
              mf: &mut AllocF,
              input: InputPair<'a>,
              stride: [u8; find_stride::NUM_LEAF_NODES],
              prediction_mode: interface::PredictionModeContextMap<InputReferenceMut<'a>>,
              prior_bitmask_detection: u8) -> Self {
      let do_alloc = prior_bitmask_detection != 0;
      let cm_speed = prediction_mode.context_map_speed();
      let stride_speed = prediction_mode.stride_context_speed();
      let mut ret = PriorEval::<AllocU16, AllocU32, AllocF>{
         input: input,
         context_map: prediction_mode,
         block_type: 0,
         local_byte_offset: 0,
         _nop:  AllocU32::AllocatedMemory::default(),
         cm_priors: if do_alloc {m16.alloc_cell(CONTEXT_MAP_PRIOR_SIZE)} else {
             AllocU16::AllocatedMemory::default()},
         stride_priors: if do_alloc {m16.alloc_cell(STRIDE_PRIOR_SIZE)} else {
             AllocU16::AllocatedMemory::default()},
         stride_pyramid_leaves: stride,
         score: if do_alloc {mf.alloc_cell(16834)} else {
             AllocF::AllocatedMemory::default()},
         cm_speed: cm_speed,
         stride_speed: stride_speed,
      };
      init_cdfs(ret.cm_priors.slice_mut());
      init_cdfs(ret.stride_priors.slice_mut());
      ret
   }
   pub fn choose_bitmask(&mut self) {
       let epsilon = 1.5;
       let max = 8192;
       let mut bitmask = [0u8; super::interface::NUM_MIXING_VALUES];
       for i in 0..max {
           if self.score.slice()[i + WhichPrior::CM as usize] > epsilon + self.score.slice()[max * WhichPrior::CM as usize + i] {
               bitmask[i] = 1;
           } else {
               bitmask[i] = 0;
           }
       }
       self.context_map.set_mixing_values(&bitmask);
   }
   pub fn free(&mut self,
               m16: &mut AllocU16,
               _m32: &mut AllocU32,
               mf: &mut AllocF) {
       mf.free_cell(core::mem::replace(&mut self.score, AllocF::AllocatedMemory::default()));
       m16.free_cell(core::mem::replace(&mut self.cm_priors, AllocU16::AllocatedMemory::default()));
       m16.free_cell(core::mem::replace(&mut self.stride_priors, AllocU16::AllocatedMemory::default()));
   }
                
   pub fn take_prediction_mode(&mut self) -> interface::PredictionModeContextMap<InputReferenceMut<'a>> {
       core::mem::replace(&mut self.context_map, interface::PredictionModeContextMap::<InputReferenceMut<'a>>{
          literal_context_map:InputReferenceMut(&mut[]),
          predmode_speed_and_distance_context_map:InputReferenceMut(&mut[]),
       })
   }
   fn update_cost_base(&mut self, stride_prior: u8, selected_bits: u8, cm_prior: usize, literal: u8) {
       {
           type CurPrior = CMPrior;
           let score_index = score_index(stride_prior, selected_bits, cm_prior, None, CurPrior::which());
           let mut cdf = CurPrior::lookup_mut(self.cm_priors.slice_mut(),
                                              stride_prior, selected_bits, cm_prior, None);
           self.score.slice_mut()[score_index] += cdf.cost(literal>>4);
           cdf.update(literal >> 4, self.cm_speed[1]);
       }
       {
           type CurPrior = CMPrior;
           let score_index = score_index(stride_prior, selected_bits, cm_prior, Some(literal >> 4), CurPrior::which());
           let mut cdf = CurPrior::lookup_mut(self.cm_priors.slice_mut(),
                                              stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           self.score.slice_mut()[score_index] += cdf.cost(literal&0xf);
           cdf.update(literal&0xf, self.cm_speed[0]);
       }
       type CurPrior = StridePrior;
       {
           let score_index = score_index(stride_prior, selected_bits, cm_prior, None, CurPrior::which());
           let mut cdf = CurPrior::lookup_mut(self.stride_priors.slice_mut(),
                                              stride_prior, selected_bits, cm_prior, None);
           self.score.slice_mut()[score_index] += cdf.cost(literal>>4);
           cdf.update(literal >> 4, self.stride_speed[1]);
       }
       {
           let score_index = score_index(stride_prior, selected_bits, cm_prior, Some(literal >> 4), CurPrior::which());
           let mut cdf = CurPrior::lookup_mut(self.stride_priors.slice_mut(),
                                              stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           self.score.slice_mut()[score_index] += cdf.cost(literal&0xf);
           cdf.update(literal&0xf, self.stride_speed[0]);
       }
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


impl<'a, 'b, AllocU16: alloc::Allocator<u16>,
     AllocU32:alloc::Allocator<u32>,
     AllocF: alloc::Allocator<floatX>> interface::CommandProcessor<'b> for PriorEval<'a, AllocU16, AllocU32, AllocF> {
    fn push<Cb: FnMut(&[interface::Command<InputReference>])>(&mut self,
                                                              val: interface::Command<InputReference<'b>>,
                                                              callback: &mut Cb) {
        push_base(self, val, callback)
    }
}

