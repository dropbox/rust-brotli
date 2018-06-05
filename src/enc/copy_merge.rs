use super::NIBBLE_PRIOR_SIZE;

pub struct NumericPrior<AllocU16:alloc::Allocator<u16>> {
    count_small: AllocU16::AllocatedMemory,
    size_beg_nib: AllocU16::AllocatedMemory,
    size_last_nib: AllocU16::AllocatedMemory,
    size_mantissa_nib: AllocU16::AllocatedMemory,
}
pub struct CopyPriors<AllocU16:alloc::Allocator<u16>> {
    count: NumericPrior<AllocU16>,
    dist: NumericPrior<AllocU16>,
}
impl CopyPriors<AllocU16:alloc::Allocator<u16>> {
    fn new(m16: &mut AllocU16) -> Self {
        self.count.count_small = m16.alloc_cell(256 * 16 * NIBBLE_PRIOR_SIZE);
        self.count.size_beg_nib = m16.alloc_cell(256 * NIBBLE_PRIOR_SIZE);
        self.count.size_last_nib = m16.alloc_cell(256 * NIBBLE_PRIOR_SIZE);
        self.count.size_mantissa_nib = m16.alloc_cell(256 * 16 * NIBBLE_PRIOR_SIZE);

        self.dist.count_small = m16.alloc_cell(256 * 8 * NIBBLE_PRIOR_SIZE);
        self.dist.size_beg_nib = m16.alloc_cell(256 * 2 * NIBBLE_PRIOR_SIZE);
        self.dist.size_last_nib = m16.alloc_cell(256 * NIBBLE_PRIOR_SIZE);
        self.dist.size_mantissa_nib = m16.alloc_cell(256 * 5 * NIBBLE_PRIOR_SIZE);
        
    }
    fn free(&mut self, m16: &mut AllocU16) {
        m16.free_cell(self.count.count_small);
        m16.free_cell(self.count.count_beg_nib);
        m16.free_cell(self.count.count_last_nib);
        m16.free_cell(self.count.count_mantissa_nib);

        m16.free_cell(self.dist.count_small);
        m16.free_cell(self.dist.count_beg_nib);
        m16.free_cell(self.dist.count_last_nib);
        m16.free_cell(self.dist.count_mantissa_nib);
    }
}

pub struct LiteralPriors<AllocU16:alloc::Allocator<u16>> {
    count: NumericPrior<AllocU16>,
}

pub struct CopyMerge<'a,
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
     > CopyMerge<'a, AllocU16, AllocU32, AllocF> {
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
          literal_context_map:InputReferenceMut(&mut[]),
          predmode_speed_and_distance_context_map:InputReferenceMut(&mut[]),
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
     AllocF: alloc::Allocator<floatX>> interface::CommandProcessor<'b> for CopyMerge<'a, AllocU16, AllocU32, AllocF> {
    #[inline]
    fn push<Cb: FnMut(&[interface::Command<InputReference>])>(&mut self,
                                                              val: interface::Command<InputReference<'b>>,
                                                              callback: &mut Cb) {
        push_base(self, val, callback)
    }
}

