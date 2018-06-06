use core;
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::interface;
use super::backward_references::BrotliEncoderParams;
use super::input_pair::{InputPair, InputReference, InputReferenceMut};
use super::ir_interpret::{IRInterpreter, push_base};
use super::util::{floatX};

use super::prior_eval::{Prior,Stride1Prior,init_cdfs, DEFAULT_SPEED, STRIDE_PRIOR_SIZE};

pub struct StrideEval<'a,
                     AllocU16:alloc::Allocator<u16> + 'a,
                     AllocU32:alloc::Allocator<u32> + 'a,
                     AllocF:alloc::Allocator<floatX> +'a,
                     > {
    input: InputPair<'a>,
    mf: &'a mut AllocF,
    m16: &'a mut AllocU16,
    _m32: &'a mut AllocU32,
    context_map: &'a interface::PredictionModeContextMap<InputReferenceMut<'a>>,
    block_type: u8,
    local_byte_offset: usize,
    _nop: AllocU32::AllocatedMemory,    
    stride_priors: [AllocU16::AllocatedMemory;8],
    score: AllocF::AllocatedMemory,
    cur_score_epoch: usize,
    stride_speed: [(u16, u16);2],
    cur_stride: u8,
}

impl<'a,
     AllocU16:alloc::Allocator<u16>,
     AllocU32:alloc::Allocator<u32>,
     AllocF:alloc::Allocator<floatX>+'a,
     > StrideEval<'a, AllocU16, AllocU32, AllocF> {
   pub fn new(m16: &'a mut AllocU16,
              _m32: &'a mut AllocU32,
              mf: &'a mut AllocF,
              input: InputPair<'a>,
              prediction_mode: &'a interface::PredictionModeContextMap<InputReferenceMut<'a>>,
              params: &BrotliEncoderParams,
              ) -> Self {
      let do_alloc = true;
      let mut stride_speed = prediction_mode.stride_context_speed();
      if stride_speed[0] == (0, 0) {
          stride_speed[0] = params.literal_adaptation[0]
      }
      if stride_speed[0] == (0, 0) {
          stride_speed[0] = DEFAULT_SPEED;
      }
      if stride_speed[1] == (0, 0) {
          stride_speed[1] = params.literal_adaptation[1]
      }
      if stride_speed[1] == (0, 0) {
          stride_speed[1] = stride_speed[0];
      }
      let score = if do_alloc {
          mf.alloc_cell(8 * 4) // FIXME make this bigger than just 4
      } else {
          AllocF::AllocatedMemory::default()
      };
      let stride_priors = if do_alloc {
          [m16.alloc_cell(STRIDE_PRIOR_SIZE),
           m16.alloc_cell(STRIDE_PRIOR_SIZE),
           m16.alloc_cell(STRIDE_PRIOR_SIZE),
           m16.alloc_cell(STRIDE_PRIOR_SIZE),
           m16.alloc_cell(STRIDE_PRIOR_SIZE),
           m16.alloc_cell(STRIDE_PRIOR_SIZE),
           m16.alloc_cell(STRIDE_PRIOR_SIZE),
           m16.alloc_cell(STRIDE_PRIOR_SIZE),
          ]
      } else {
          [AllocU16::AllocatedMemory::default(),
           AllocU16::AllocatedMemory::default(),
           AllocU16::AllocatedMemory::default(),
           AllocU16::AllocatedMemory::default(),
           AllocU16::AllocatedMemory::default(),
           AllocU16::AllocatedMemory::default(),
           AllocU16::AllocatedMemory::default(),
           AllocU16::AllocatedMemory::default(),
              ]
      };
      let mut ret = StrideEval::<AllocU16, AllocU32, AllocF>{
         input: input,
         context_map: prediction_mode,
          block_type: 0,
          m16: m16,
          _m32: _m32,
          mf: mf,
          cur_stride: 1,
          cur_score_epoch: 0,
         local_byte_offset: 0,
         _nop:  AllocU32::AllocatedMemory::default(),
         stride_priors: stride_priors,
         score: score,
         stride_speed: stride_speed,
      };
      for stride_prior in ret.stride_priors.iter_mut() {
          init_cdfs(stride_prior.slice_mut());
      }
      ret
   }
   pub fn choose_stride(&self, stride_data: &mut[u8]) {
       assert_eq!(stride_data.len(), self.cur_score_epoch);
       assert!(self.score.slice().len() > stride_data.len());
       assert!(self.score.slice().len() > (stride_data.len() << 3) + 7 + 8);
       for (index, choice) in stride_data.iter_mut().enumerate() {
           let choices = self.score.slice().split_at((1 + index) << 3).1.split_at(8).0;
           let mut best_choice: u8 = 0;
           let mut best_score = choices[0];
           for (cur_index, cur_score) in choices.iter().enumerate() {
               if *cur_score + 2.0 < best_score { // needs to be 2 bits better to be worth the type switch
                   best_score = *cur_score;
                   best_choice = cur_index as u8;
               }
           }
           *choice = best_choice;
       }
   }
   pub fn num_types(&self) -> usize {
       self.cur_score_epoch
   }
   fn update_cost_base(&mut self, stride_prior: [u8;8], selected_bits: u8, cm_prior: usize, literal: u8) {
       type CurPrior = Stride1Prior;
       {
           for i in 0..8 {
               let mut cdf = CurPrior::lookup_mut(self.stride_priors[i].slice_mut(),
                                                  stride_prior[i], selected_bits, cm_prior, None);
               self.score.slice_mut()[self.cur_score_epoch * 8 + i] += cdf.cost(literal>>4);
               cdf.update(literal >> 4, self.stride_speed[1]);
           }
       }
       {
           for i in 0..8 {
               let mut cdf = CurPrior::lookup_mut(self.stride_priors[i].slice_mut(),
                                                  stride_prior[i], selected_bits, cm_prior, Some(literal >> 4));
               self.score.slice_mut()[self.cur_score_epoch * 8 + i] += cdf.cost(literal&0xf);
               cdf.update(literal&0xf, self.stride_speed[0]);
           }
       }
   }
}
impl<'a, AllocU16: alloc::Allocator<u16>,
     AllocU32:alloc::Allocator<u32>,
     AllocF: alloc::Allocator<floatX>> Drop for StrideEval<'a, AllocU16, AllocU32, AllocF> {
   fn drop(&mut self) {
       self.mf.free_cell(core::mem::replace(&mut self.score, AllocF::AllocatedMemory::default()));
       for i in 0..8 {
           self.m16.free_cell(core::mem::replace(&mut self.stride_priors[i], AllocU16::AllocatedMemory::default()));
       }
   }
}

impl<'a, AllocU16: alloc::Allocator<u16>,
     AllocU32:alloc::Allocator<u32>,
     AllocF: alloc::Allocator<floatX>> IRInterpreter for StrideEval<'a, AllocU16, AllocU32, AllocF> {
    fn inc_local_byte_offset(&mut self, inc: usize) {
        self.local_byte_offset += inc;
    }
    fn local_byte_offset(&self) -> usize {
        self.local_byte_offset
    }
    fn update_block_type(&mut self, new_type: u8, stride: u8) {
        self.block_type = new_type;
        self.cur_stride = stride;
        self.cur_score_epoch += 1;
        if self.cur_score_epoch * 8 + 7 >= self.score.slice().len() {
            let new_len = self.score.slice().len() * 2;
            let mut new_score = self.mf.alloc_cell(new_len);
            for (src, dst) in self.score.slice().iter().zip(new_score.slice_mut().split_at_mut(self.score.slice().len()).0.iter_mut()) {
                *dst = *src;
            }
            self.mf.free_cell(core::mem::replace(&mut self.score, new_score));
        }
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
    fn update_cost(&mut self, stride_prior: [u8;8], stride_prior_offset: usize , selected_bits: u8, cm_prior: usize, literal: u8) {
        let reversed_stride_priors = [stride_prior[stride_prior_offset&7],
                                      stride_prior[stride_prior_offset.wrapping_sub(1)&7],
                                      stride_prior[stride_prior_offset.wrapping_sub(2)&7],
                                      stride_prior[stride_prior_offset.wrapping_sub(3)&7],
                                      stride_prior[stride_prior_offset.wrapping_sub(4)&7],
                                      stride_prior[stride_prior_offset.wrapping_sub(5)&7],
                                      stride_prior[stride_prior_offset.wrapping_sub(6)&7],
                                      stride_prior[stride_prior_offset.wrapping_sub(7)&7]];
        self.update_cost_base(reversed_stride_priors, selected_bits, cm_prior, literal)
    }
}


impl<'a, 'b, AllocU16: alloc::Allocator<u16>,
     AllocU32:alloc::Allocator<u32>,
     AllocF: alloc::Allocator<floatX>> interface::CommandProcessor<'b> for StrideEval<'a, AllocU16, AllocU32, AllocF> {
    fn push(&mut self,
            val: interface::Command<InputReference<'b>>) {
        push_base(self, val)
    }
}

