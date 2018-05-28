
use super::super::alloc::{SliceWrapper, SliceWrapperMut, Allocator};
use super::util::floatX;
use super::vectorization::Mem256f;
use super::interface;
use super::interface::CommandProcessor;
use super::ir_interpret::{IRInterpreter, push_base};
use super::pdf::PDF;
use interface::PredictionModeContextMap;
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
