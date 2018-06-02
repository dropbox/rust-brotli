
use super::super::alloc::{SliceWrapper, SliceWrapperMut, Allocator};
use super::util::floatX;
use super::vectorization::Mem256f;
use super::interface;
use super::interface::CommandProcessor;
use super::ir_interpret::{IRInterpreter, push_base};
use super::pdf::{PDF, similar};
use interface::{PredictionModeContextMap, MAX_ADV_LITERAL_CONTEXT_MAP_SIZE, MAX_LITERAL_CONTEXT_MAP_SIZE};
use core::marker::PhantomData;
use enc::backward_references::BrotliEncoderParams;
use super::input_pair::{InputPair, InputReference, InputReferenceMut};
pub struct Processor<'a,
                 AllocU16:Allocator<u16>,
                 AllocU32:Allocator<u32>,
                 AllocF:Allocator<floatX>,
                 AllocFV:Allocator<Mem256f>,
                 AllocPDF:Allocator<PDF>> {
    p0: PhantomData<AllocU16>,
    p1: PhantomData<AllocU32>,
    p2: PhantomData<AllocF>,
    p3: PhantomData<AllocFV>,
    p4: PhantomData<AllocPDF>,
    pub context_map: PredictionModeContextMap<InputReferenceMut<'a>>,
    input: InputPair<'a>,
    local_byte_offset: usize,
    block_type: u8,
    cur_stride: u8,
    context_histograms: AllocPDF::AllocatedMemory,
    htable_histograms: AllocPDF::AllocatedMemory,
}

impl<'a,
     AllocU16:Allocator<u16>,
     AllocU32:Allocator<u32>,
     AllocF:Allocator<floatX>,
     AllocFV:Allocator<Mem256f>,
     AllocPDF:Allocator<PDF>> Processor<'a,
                                        AllocU16,
                                        AllocU32,
                                        AllocF,
                                        AllocFV,
                                        AllocPDF> {
    pub fn new(m16: &mut AllocU16,
               m32: &mut AllocU32,
               mf: &mut AllocF,
               mfv: &mut AllocFV,
               mpdf: &mut AllocPDF,
               ip: InputPair<'a>,
               cm: PredictionModeContextMap<InputReferenceMut<'a>>,
               params:&BrotliEncoderParams) -> Self {
        Self{
            p0: PhantomData::<AllocU16>::default(),
            p1: PhantomData::<AllocU32>::default(),
            p2: PhantomData::<AllocF>::default(),
            p3: PhantomData::<AllocFV>::default(),
            p4: PhantomData::<AllocPDF>::default(),
            context_map:cm,
            input: ip,
            local_byte_offset:0,
            block_type:0,
            cur_stride:0,
            context_histograms: mpdf.alloc_cell(MAX_ADV_LITERAL_CONTEXT_MAP_SIZE),
            htable_histograms: mpdf.alloc_cell(256 * 2),
        }
    }
    pub fn merge(&mut self) {
        let mut merge_map = [[0u8;256];2];
        for i in 0..256 {
            merge_map[0][i] = i as u8;
            merge_map[1][i] = i as u8;
        }
        for i in 0..256 {
            for j in (i + 1)..256 {
                if similar(&self.htable_histograms.slice()[i],&self.htable_histograms.slice()[j]) {
                    merge_map[0][j] = merge_map[0][i];
                }
                if similar(&self.htable_histograms.slice()[i + 256],&self.htable_histograms.slice()[j + 256]) {
                    merge_map[1][j] = merge_map[1][i];
                }
            }
        }
        eprintln!("Merging map {:?} and {:?}\n", &merge_map[0][..],&merge_map[1][..]);
        for (index, item) in self.context_map.literal_context_map.slice_mut().iter_mut().enumerate() {
            if index < MAX_LITERAL_CONTEXT_MAP_SIZE {
                *item = merge_map[0][usize::from(*item)];
            } else {
                *item = merge_map[1][usize::from(*item)];
            }
        }
    }
}

impl<'a,
     AllocU16:Allocator<u16>,
     AllocU32:Allocator<u32>,
     AllocF:Allocator<floatX>,
     AllocFV:Allocator<Mem256f>,
     AllocPDF:Allocator<PDF>> IRInterpreter for Processor <'a, AllocU16, AllocU32, AllocF, AllocFV, AllocPDF> {
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
        //self.update_cost_base(stride_prior, stride_prior_offset, selected_bits, cm_prior, literal)
        let high_bit = literal >> 4;
        let low_bit = literal & 0xf;
        let context_map_low_index = usize::from(selected_bits) + 64 * usize::from(self.block_type);
        let context_map_high_index = (usize::from(selected_bits) + 64 * usize::from(self.block_type)) + (64 * 256 * (high_bit as usize + 1));
        self.context_histograms.slice_mut()[context_map_low_index].add_sample(low_bit);
        self.context_histograms.slice_mut()[context_map_high_index].add_sample(high_bit);
        let high_indirect_index = usize::from(self.context_map.literal_context_map.slice()[context_map_high_index]);
        let low_indirect_index = usize::from(self.context_map.literal_context_map.slice()[context_map_low_index]) + 256;
        assert_eq!(low_indirect_index, high_indirect_index + 256);
        self.htable_histograms.slice_mut()[low_indirect_index].add_sample(low_bit);
        self.htable_histograms.slice_mut()[high_indirect_index].add_sample(high_bit);
    }

    
}



impl<'a, 'b,
     AllocU16:Allocator<u16>,
     AllocU32:Allocator<u32>,
     AllocF:Allocator<floatX>,
     AllocFV:Allocator<Mem256f>,
     AllocPDF:Allocator<PDF>> CommandProcessor<'b> for Processor <'a, AllocU16, AllocU32, AllocF, AllocFV, AllocPDF> {
    fn push<Cb: FnMut(&[interface::Command<InputReference>])>(&mut self,
                                                              val: interface::Command<InputReference<'b>>,
                                                              callback: &mut Cb) {
        push_base(self, val, callback)
    }

}
