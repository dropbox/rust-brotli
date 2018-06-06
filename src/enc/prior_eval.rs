use core;
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::interface;
use super::backward_references::BrotliEncoderParams;
use super::input_pair::{InputPair, InputReference, InputReferenceMut};
use super::ir_interpret::{IRInterpreter, push_base};
use super::util::{floatX, FastLog2u16};
use super::find_stride;

const NIBBLE_PRIOR_SIZE: usize = 16;
// the high nibble, followed by the low nibbles
pub const CONTEXT_MAP_PRIOR_SIZE: usize = 256 * NIBBLE_PRIOR_SIZE * 17;
pub const STRIDE_PRIOR_SIZE: usize = 256 * 256 * NIBBLE_PRIOR_SIZE * 2;
pub const ADV_PRIOR_SIZE: usize = 256 * 256 * 16 * 2 * NIBBLE_PRIOR_SIZE;
pub const DEFAULT_SPEED: (u16, u16) = (8, 8192);
pub enum WhichPrior {
    CM = 0,
    ADV = 1,
    SLOW_CM = 2,
    FAST_CM = 3,
    STRIDE1 = 4,
    STRIDE2 = 5,
    STRIDE3 = 6,
    STRIDE4 = 7,
    STRIDE8 = 8,
    NUM_PRIORS = 9,
    // future ideas
}

pub trait Prior {
    fn lookup_lin(stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize;
    #[inline]
    fn lookup_mut(data:&mut [u16], stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> CDF {
        let index = Self::lookup_lin(stride_byte, selected_context, actual_context,
                             high_nibble) * NIBBLE_PRIOR_SIZE;
        CDF::from(data.split_at_mut(index).1.split_at_mut(16).0)
    }
    #[inline]
    fn lookup(data:&[u16], stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> &[u16] {
        let index = Self::lookup_lin(stride_byte, selected_context, actual_context,
                             high_nibble) * NIBBLE_PRIOR_SIZE;
        data.split_at(index).1.split_at(16).0
    }
    #[allow(unused_variables)]
    #[inline]
    fn score_index(stride_byte: u8, selected_context: u8, actual_context: usize, high_nibble: Option<u8>) -> usize {
        let which: WhichPrior = Self::which();
        if let Some(nibble) = high_nibble {
        actual_context + 4096 + 256 * nibble as usize + (which as usize * 8192)
        } else {
            actual_context + 256 * (stride_byte >> 4) as usize + (which as usize * 8192)
        }
    }
    fn which() -> WhichPrior;
}

#[allow(unused_variables)]
#[inline]
fn stride_lookup_lin(stride_byte:u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize {
    if let Some(nibble) = high_nibble {
        1 + 2 * (actual_context as usize
                 | ((stride_byte as usize & 0xf) << 8)
                 | ((nibble as usize) << 12))
    } else {
        2 * (actual_context as usize | ((stride_byte as usize) << 8))
    }
}
pub struct Stride1Prior{
}
impl Stride1Prior {
    pub fn offset() -> usize{
        0
    }
}

impl Prior for Stride1Prior {
    #[inline]
    fn lookup_lin(stride_byte:u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize {
        stride_lookup_lin(stride_byte, selected_context, actual_context, high_nibble)
    }
    #[inline]
    fn which() -> WhichPrior {
        WhichPrior::STRIDE1
    }
}
/*impl StridePrior for Stride1Prior {
    const STRIDE_OFFSET:usize = 0;
}*/
pub struct Stride2Prior{
}
impl Stride2Prior {
    #[inline(always)]
    pub fn offset() -> usize{
        1
    }
}

impl Prior for Stride2Prior {
    #[inline(always)]
    fn lookup_lin(stride_byte:u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize {
        stride_lookup_lin(stride_byte, selected_context, actual_context, high_nibble)
    }
    #[inline]
    fn which() -> WhichPrior {
        WhichPrior::STRIDE2
    }
}
/*impl StridePrior for Stride2Prior {
    const STRIDE_OFFSET:usize = 1;
}*/
pub struct Stride3Prior{
}
impl Stride3Prior {
    #[inline(always)]
    pub fn offset() -> usize{
        2
    }
}

impl Prior for Stride3Prior {
    #[inline(always)]
    fn lookup_lin(stride_byte:u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize {
        stride_lookup_lin(stride_byte, selected_context, actual_context, high_nibble)
    }
    #[inline]
    fn which() -> WhichPrior {
        WhichPrior::STRIDE3
    }
}

/*impl StridePrior for Stride3Prior {
    const STRIDE_OFFSET:usize = 2;
}*/
pub struct Stride4Prior{
}
impl Stride4Prior {
    #[inline(always)]
    pub fn offset() -> usize{
        3
    }
}
impl Prior for Stride4Prior {
    #[inline(always)]
    fn lookup_lin(stride_byte:u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize {
        stride_lookup_lin(stride_byte, selected_context, actual_context, high_nibble)
    }
    #[inline]
    fn which() -> WhichPrior {
        WhichPrior::STRIDE4
    }
}

/*impl StridePrior for Stride4Prior {
    const STRIDE_OFFSET:usize = 3;
}*/
pub struct Stride8Prior{
}/*
impl StridePrior for Stride8Prior {
    const STRIDE_OFFSET:usize = 7;
}*/
impl Stride8Prior {
    #[inline(always)]
    pub fn offset() -> usize{
        7
    }
}
impl Prior for Stride8Prior {
    fn lookup_lin(stride_byte:u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize {
        stride_lookup_lin(stride_byte, selected_context, actual_context, high_nibble)
    }
    #[inline]
    fn which() -> WhichPrior {
        WhichPrior::STRIDE8
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
    #[inline]
    fn which() -> WhichPrior {
        WhichPrior::CM
    }
}
pub struct FastCMPrior {
}
impl Prior for FastCMPrior {
    #[allow(unused_variables)]
    fn lookup_lin(stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize {
        if let Some(nibble) = high_nibble {
            2 * actual_context
        } else {
            2 * actual_context + 1
        }
    }
    #[inline]
    fn which() -> WhichPrior {
        WhichPrior::FAST_CM
    }
}

pub struct SlowCMPrior {
}
impl Prior for SlowCMPrior {
    #[allow(unused_variables)]
    fn lookup_lin(stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize {
        if let Some(nibble) = high_nibble {
            (nibble as usize + 1) + 17 * actual_context
        } else {
            17 * actual_context as usize
        }
    }
    #[inline]
    fn which() -> WhichPrior {
        WhichPrior::SLOW_CM
    }
}

pub struct AdvPrior {
}
impl Prior for AdvPrior {
    #[allow(unused_variables)]
    fn lookup_lin(stride_byte: u8, selected_context:u8, actual_context:usize, high_nibble: Option<u8>) -> usize {
        if let Some(nibble) = high_nibble {
            65536 + ((actual_context as usize)
                  | ((stride_byte as usize) << 8)
                  | ((nibble as usize & 0xf) << 16))
        } else {
            (actual_context as usize)
             | ((stride_byte as usize & 0xf0) << 8)
        }
    }
    fn which() -> WhichPrior {
        WhichPrior::ADV
    }
}

pub struct CDF<'a> {
    cdf:&'a mut [u16],
}

impl<'a> CDF<'a> {
    #[inline(always)]
    pub fn cost(&self, nibble_u8:u8) -> floatX {
        assert_eq!(self.cdf.len(), 16);
        let nibble = nibble_u8 as usize & 0xf;
        let mut pdf = self.cdf[nibble];
        if nibble_u8 != 0 {
            pdf -= self.cdf[nibble - 1];
        }
        FastLog2u16(self.cdf[15]) - FastLog2u16(pdf)
    }
    #[inline(always)]
    pub fn update(&mut self, nibble_u8:u8, speed: (u16, u16)) {
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
    #[inline]
    fn from(cdf: &'a mut[u16]) -> CDF<'a> {
        assert_eq!(cdf.len(), 16);
        CDF {
            cdf:cdf,
        }
    }
}


pub fn init_cdfs(cdfs: &mut [u16]) {
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
    slow_cm_priors: AllocU16::AllocatedMemory,
    fast_cm_priors: AllocU16::AllocatedMemory,
    stride_priors: [AllocU16::AllocatedMemory; 5],
    adv_priors: AllocU16::AllocatedMemory,
    _stride_pyramid_leaves: [u8; find_stride::NUM_LEAF_NODES],
    score: AllocF::AllocatedMemory,
    cm_speed: [(u16, u16);2],
    stride_speed: [(u16, u16);2],
    cur_stride: u8,
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
              params: &BrotliEncoderParams,
              ) -> Self {
      let do_alloc = params.prior_bitmask_detection != 0;
      let mut cm_speed = prediction_mode.context_map_speed();
      let mut stride_speed = prediction_mode.stride_context_speed();
      if cm_speed[0] == (0,0) {
          cm_speed[0] = params.literal_adaptation[2]
      }
      if cm_speed[0] == (0,0) {
          cm_speed[0] = DEFAULT_SPEED;
      }
      if cm_speed[1] == (0,0) {
          cm_speed[1] = params.literal_adaptation[3]
      }
      if cm_speed[1] == (0,0) {
          cm_speed[1] = cm_speed[0];
      }
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
      let mut ret = PriorEval::<AllocU16, AllocU32, AllocF>{
         input: input,
         context_map: prediction_mode,
         block_type: 0,
         cur_stride: 1,
         local_byte_offset: 0,
         _nop:  AllocU32::AllocatedMemory::default(),
         cm_priors: if do_alloc {m16.alloc_cell(CONTEXT_MAP_PRIOR_SIZE)} else {
             AllocU16::AllocatedMemory::default()},
         slow_cm_priors: if do_alloc {m16.alloc_cell(CONTEXT_MAP_PRIOR_SIZE)} else {
             AllocU16::AllocatedMemory::default()},
         fast_cm_priors: if do_alloc {m16.alloc_cell(CONTEXT_MAP_PRIOR_SIZE)} else {
             AllocU16::AllocatedMemory::default()},
         stride_priors: [
             if do_alloc {m16.alloc_cell(STRIDE_PRIOR_SIZE)} else {
                 AllocU16::AllocatedMemory::default()},
             if do_alloc {m16.alloc_cell(STRIDE_PRIOR_SIZE)} else {
                 AllocU16::AllocatedMemory::default()},
             if do_alloc {m16.alloc_cell(STRIDE_PRIOR_SIZE)} else {
                 AllocU16::AllocatedMemory::default()},
             if do_alloc {m16.alloc_cell(STRIDE_PRIOR_SIZE)} else {
                 AllocU16::AllocatedMemory::default()},
             if do_alloc {m16.alloc_cell(STRIDE_PRIOR_SIZE)} else {
                 AllocU16::AllocatedMemory::default()},],
         adv_priors: if do_alloc {m16.alloc_cell(ADV_PRIOR_SIZE)} else {
             AllocU16::AllocatedMemory::default()},
         _stride_pyramid_leaves: stride,
         score: if do_alloc {mf.alloc_cell(8192 * WhichPrior::NUM_PRIORS as usize )} else {
             AllocF::AllocatedMemory::default()},
         cm_speed: cm_speed,
         stride_speed: stride_speed,
      };
      init_cdfs(ret.cm_priors.slice_mut());
      init_cdfs(ret.slow_cm_priors.slice_mut());
      init_cdfs(ret.fast_cm_priors.slice_mut());
      init_cdfs(ret.stride_priors[0].slice_mut());
      init_cdfs(ret.stride_priors[1].slice_mut());
      init_cdfs(ret.stride_priors[2].slice_mut());
      init_cdfs(ret.stride_priors[3].slice_mut());
      init_cdfs(ret.stride_priors[4].slice_mut());
      init_cdfs(ret.adv_priors.slice_mut());
      ret
   }
   pub fn choose_bitmask(&mut self) {
       let epsilon = 6.0;
       let max = 8192;
       let mut bitmask = [0u8; super::interface::NUM_MIXING_VALUES];
       for i in 0..max {
           let cm_index = i + max * WhichPrior::CM as usize;
           let slow_cm_index = i + max * WhichPrior::SLOW_CM as usize;
           let fast_cm_index = i + max * WhichPrior::FAST_CM as usize;
           let stride_index1 = i + max * WhichPrior::STRIDE1 as usize;
           let stride_index2 = i + max * WhichPrior::STRIDE2 as usize;
           let stride_index3 = i + max * WhichPrior::STRIDE3 as usize;
           let stride_index4 = i + max * WhichPrior::STRIDE4 as usize;
           let stride_index8 = i + max * WhichPrior::STRIDE8 as usize;
           let adv_index = i + max * WhichPrior::ADV as usize;
           let cm_score = self.score.slice()[cm_index];
           let slow_cm_score = self.score.slice()[slow_cm_index];
           let fast_cm_score = self.score.slice()[fast_cm_index] + 16.0;
           let stride1_score = self.score.slice()[stride_index1];
           let stride2_score = self.score.slice()[stride_index2];
           let stride3_score = self.score.slice()[stride_index3] + 16.0;
           let stride4_score = self.score.slice()[stride_index4] + 16.0;
           let stride8_score = self.score.slice()[stride_index8] * 1.125 + 16.0;
           let stride_score = core::cmp::min(stride1_score as u64,
                                             core::cmp::min(stride2_score as u64,
                                                            core::cmp::min(stride3_score as u64,
                                                                           core::cmp::min(stride4_score as u64,
                                                                                          stride8_score as u64))));
                                  
           let adv_score = self.score.slice()[adv_index];
           if adv_score + epsilon < stride_score as floatX && adv_score + epsilon < cm_score && adv_score + epsilon < slow_cm_score && adv_score + epsilon < fast_cm_score {
               bitmask[i] = 1;
           } else if slow_cm_score + epsilon < stride_score as floatX && slow_cm_score + epsilon < cm_score && slow_cm_score + epsilon < fast_cm_score {
               bitmask[i] = 2;
           } else if fast_cm_score + epsilon < stride_score as floatX && fast_cm_score + epsilon < cm_score {
               bitmask[i] = 3;
           } else if epsilon + (stride_score as floatX) < cm_score {
               bitmask[i] = WhichPrior::STRIDE1 as u8;
               if stride_score == stride8_score as u64 {
                   bitmask[i] = WhichPrior::STRIDE8 as u8;
               }
               if stride_score == stride4_score as u64 {
                   bitmask[i] = WhichPrior::STRIDE4 as u8;
               }
               if stride_score == stride3_score as u64 {
                   bitmask[i] = WhichPrior::STRIDE3 as u8;
               }
               if stride_score == stride2_score as u64 {
                   bitmask[i] = WhichPrior::STRIDE2 as u8;
               }
               if stride_score == stride1_score as u64 {
                   bitmask[i] = WhichPrior::STRIDE1 as u8;
               }
           } else {
               bitmask[i] = 0;
           }
           //eprintln!("Score {} {}: {}", cm_score, stride_score, bitmask[i]);
       }
       self.context_map.set_mixing_values(&bitmask);
   }
   pub fn free(&mut self,
               m16: &mut AllocU16,
               _m32: &mut AllocU32,
               mf: &mut AllocF) {
       mf.free_cell(core::mem::replace(&mut self.score, AllocF::AllocatedMemory::default()));
       m16.free_cell(core::mem::replace(&mut self.cm_priors, AllocU16::AllocatedMemory::default()));
       m16.free_cell(core::mem::replace(&mut self.slow_cm_priors, AllocU16::AllocatedMemory::default()));
       m16.free_cell(core::mem::replace(&mut self.fast_cm_priors, AllocU16::AllocatedMemory::default()));
       m16.free_cell(core::mem::replace(&mut self.stride_priors[0], AllocU16::AllocatedMemory::default()));
       m16.free_cell(core::mem::replace(&mut self.stride_priors[1], AllocU16::AllocatedMemory::default()));
       m16.free_cell(core::mem::replace(&mut self.stride_priors[2], AllocU16::AllocatedMemory::default()));
       m16.free_cell(core::mem::replace(&mut self.stride_priors[3], AllocU16::AllocatedMemory::default()));
       m16.free_cell(core::mem::replace(&mut self.stride_priors[4], AllocU16::AllocatedMemory::default()));
       m16.free_cell(core::mem::replace(&mut self.adv_priors, AllocU16::AllocatedMemory::default()));
   }
                
   pub fn take_prediction_mode(&mut self) -> interface::PredictionModeContextMap<InputReferenceMut<'a>> {
       core::mem::replace(&mut self.context_map, interface::PredictionModeContextMap::<InputReferenceMut<'a>>{
          literal_context_map:InputReferenceMut::default(),
          predmode_speed_and_distance_context_map:InputReferenceMut::default(),
       })
   }
   fn update_cost_base(&mut self, stride_prior: [u8;8], stride_prior_offset:usize, selected_bits: u8, cm_prior: usize, literal: u8) {
       let base_stride_prior = stride_prior[stride_prior_offset.wrapping_sub(self.cur_stride as usize) & 7];
       {
           type CurPrior = CMPrior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, None);
           let mut cdf = CurPrior::lookup_mut(self.cm_priors.slice_mut(),
                                              base_stride_prior, selected_bits, cm_prior, None);
           self.score.slice_mut()[score_index] += cdf.cost(literal>>4);
           cdf.update(literal >> 4, self.cm_speed[1]);
       }
       {
           type CurPrior = CMPrior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           let mut cdf = CurPrior::lookup_mut(self.cm_priors.slice_mut(),
                                              base_stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           self.score.slice_mut()[score_index] += cdf.cost(literal&0xf);
           cdf.update(literal&0xf, self.cm_speed[0]);
       }
       {
           type CurPrior = SlowCMPrior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, None);
           let mut cdf = CurPrior::lookup_mut(self.slow_cm_priors.slice_mut(),
                                              base_stride_prior, selected_bits, cm_prior, None);
           self.score.slice_mut()[score_index] += cdf.cost(literal>>4);
           cdf.update(literal >> 4, (0,1024));
       }
       {
           type CurPrior = SlowCMPrior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           let mut cdf = CurPrior::lookup_mut(self.slow_cm_priors.slice_mut(),
                                              base_stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           self.score.slice_mut()[score_index] += cdf.cost(literal&0xf);
           cdf.update(literal&0xf, (0,1024));
       }
       {
           type CurPrior = FastCMPrior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, None);
           let mut cdf = CurPrior::lookup_mut(self.fast_cm_priors.slice_mut(),
                                              base_stride_prior, selected_bits, cm_prior, None);
           self.score.slice_mut()[score_index] += cdf.cost(literal>>4);
           cdf.update(literal >> 4, self.cm_speed[0]);
       }
       {
           type CurPrior = FastCMPrior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           let mut cdf = CurPrior::lookup_mut(self.fast_cm_priors.slice_mut(),
                                              base_stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           self.score.slice_mut()[score_index] += cdf.cost(literal&0xf);
           cdf.update(literal&0xf, self.cm_speed[0]);
       }
       {
           type CurPrior = Stride1Prior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, None);
           let mut cdf = CurPrior::lookup_mut(self.stride_priors[0].slice_mut(),
                                              stride_prior[stride_prior_offset.wrapping_sub(CurPrior::offset())&7], selected_bits, cm_prior, None);
           self.score.slice_mut()[score_index] += cdf.cost(literal>>4);
           cdf.update(literal >> 4, self.stride_speed[1]);
       }
       {
           type CurPrior = Stride1Prior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           let mut cdf = CurPrior::lookup_mut(self.stride_priors[0].slice_mut(),
                                              stride_prior[stride_prior_offset.wrapping_sub(CurPrior::offset())&7],
                                              selected_bits,
                                              cm_prior,
                                              Some(literal >> 4));
           self.score.slice_mut()[score_index] += cdf.cost(literal&0xf);
           cdf.update(literal&0xf, self.stride_speed[0]);
       }
       {
           type CurPrior = Stride2Prior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, None);
           let mut cdf = CurPrior::lookup_mut(self.stride_priors[1].slice_mut(),
                                              stride_prior[stride_prior_offset.wrapping_sub(CurPrior::offset())&7], selected_bits, cm_prior, None);
           self.score.slice_mut()[score_index] += cdf.cost(literal>>4);
           cdf.update(literal >> 4, self.stride_speed[1]);
       }
       {
           type CurPrior = Stride2Prior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           let mut cdf = CurPrior::lookup_mut(self.stride_priors[1].slice_mut(),
                                              stride_prior[stride_prior_offset.wrapping_sub(CurPrior::offset())&7],
                                              selected_bits,
                                              cm_prior,
                                              Some(literal >> 4));
           self.score.slice_mut()[score_index] += cdf.cost(literal&0xf);
           cdf.update(literal&0xf, self.stride_speed[0]);
       }
       {
           type CurPrior = Stride3Prior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, None);
           let mut cdf = CurPrior::lookup_mut(self.stride_priors[2].slice_mut(),
                                              stride_prior[stride_prior_offset.wrapping_sub(CurPrior::offset())&7], selected_bits, cm_prior, None);
           self.score.slice_mut()[score_index] += cdf.cost(literal>>4);
           cdf.update(literal >> 4, self.stride_speed[1]);
       }
       {
           type CurPrior = Stride3Prior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           let mut cdf = CurPrior::lookup_mut(self.stride_priors[2].slice_mut(),
                                              stride_prior[stride_prior_offset.wrapping_sub(CurPrior::offset())&7],
                                              selected_bits,
                                              cm_prior,
                                              Some(literal >> 4));
           self.score.slice_mut()[score_index] += cdf.cost(literal&0xf);
           cdf.update(literal&0xf, self.stride_speed[0]);
       }
       {
           type CurPrior = Stride4Prior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, None);
           let mut cdf = CurPrior::lookup_mut(self.stride_priors[3].slice_mut(),
                                              stride_prior[stride_prior_offset.wrapping_sub(CurPrior::offset())&7], selected_bits, cm_prior, None);
           self.score.slice_mut()[score_index] += cdf.cost(literal>>4);
           cdf.update(literal >> 4, self.stride_speed[1]);
       }
       {
           type CurPrior = Stride4Prior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           let mut cdf = CurPrior::lookup_mut(self.stride_priors[3].slice_mut(),
                                              stride_prior[stride_prior_offset.wrapping_sub(CurPrior::offset())&7],
                                              selected_bits,
                                              cm_prior,
                                              Some(literal >> 4));
           self.score.slice_mut()[score_index] += cdf.cost(literal&0xf);
           cdf.update(literal&0xf, self.stride_speed[0]);
       }
       {
           type CurPrior = Stride8Prior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, None);
           let mut cdf = CurPrior::lookup_mut(self.stride_priors[4].slice_mut(),
                                              stride_prior[stride_prior_offset.wrapping_sub(CurPrior::offset())&7], selected_bits, cm_prior, None);
           self.score.slice_mut()[score_index] += cdf.cost(literal>>4);
           cdf.update(literal >> 4, self.stride_speed[1]);
       }
       {
           type CurPrior = Stride8Prior;
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           let mut cdf = CurPrior::lookup_mut(self.stride_priors[4].slice_mut(),
                                              stride_prior[stride_prior_offset.wrapping_sub(CurPrior::offset()) & 7],
                                              selected_bits,
                                              cm_prior,
                                              Some(literal >> 4));
           self.score.slice_mut()[score_index] += cdf.cost(literal&0xf);
           cdf.update(literal&0xf, self.stride_speed[0]);
       }

       type CurPrior = AdvPrior;
       {
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, None);
           let mut cdf = CurPrior::lookup_mut(self.adv_priors.slice_mut(),
                                              base_stride_prior, selected_bits, cm_prior, None);
           self.score.slice_mut()[score_index] += cdf.cost(literal>>4);
           cdf.update(literal >> 4, self.stride_speed[1]);
       }
       {
           let score_index = CurPrior::score_index(base_stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           let mut cdf = CurPrior::lookup_mut(self.adv_priors.slice_mut(),
                                              base_stride_prior, selected_bits, cm_prior, Some(literal >> 4));
           self.score.slice_mut()[score_index] += cdf.cost(literal&0xf);
           cdf.update(literal&0xf, self.stride_speed[0]);
       }
   }
}
impl<'a, AllocU16: alloc::Allocator<u16>,
     AllocU32:alloc::Allocator<u32>,
     AllocF: alloc::Allocator<floatX>> IRInterpreter for PriorEval<'a, AllocU16, AllocU32, AllocF> {
    #[inline]
    fn inc_local_byte_offset(&mut self, inc: usize) {
        self.local_byte_offset += inc;
    }
    #[inline]
    fn local_byte_offset(&self) -> usize {
        self.local_byte_offset
    }
    #[inline]
    fn update_block_type(&mut self, new_type: u8, stride: u8) {
        self.block_type = new_type;
        self.cur_stride = stride;
    }
    #[inline]
    fn block_type(&self) -> u8 {
        self.block_type
    }
    #[inline]
    fn literal_data_at_offset(&self, index:usize) -> u8 {
        self.input[index]
    }
    #[inline]
    fn literal_context_map(&self) -> &[u8] {
        self.context_map.literal_context_map.slice()
    }
    #[inline]
    fn prediction_mode(&self) -> ::interface::LiteralPredictionModeNibble {
        self.context_map.literal_prediction_mode()
    }
    #[inline]
    fn update_cost(&mut self, stride_prior: [u8;8], stride_prior_offset: usize, selected_bits: u8, cm_prior: usize, literal: u8) {
        //let stride = self.cur_stride as usize;
        self.update_cost_base(stride_prior, stride_prior_offset, selected_bits, cm_prior, literal)
    }
}


impl<'a, 'b, AllocU16: alloc::Allocator<u16>,
     AllocU32:alloc::Allocator<u32>,
     AllocF: alloc::Allocator<floatX>> interface::CommandProcessor<'b> for PriorEval<'a, AllocU16, AllocU32, AllocF> {
    #[inline]
    fn push(&mut self,
            val: interface::Command<InputReference<'b>>) {
        push_base(self, val)
    }
}

