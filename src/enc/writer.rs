use super::vectorization::Mem256f;
use super::cluster::HistogramPair;
use super::command::Command;
use super::input_pair;
use enc::PDF;
use enc::StaticCommand;
use super::hash_to_binary_tree::ZopfliNode;
use super::encode::{BrotliEncoderCreateInstance, BrotliEncoderDestroyInstance,
                    BrotliEncoderParameter, BrotliEncoderSetParameter, BrotliEncoderOperation,
                    BrotliEncoderStateStruct, BrotliEncoderCompressStream, BrotliEncoderIsFinished};
use super::entropy_encode::HuffmanTree;
use super::histogram::{ContextType, HistogramLiteral, HistogramCommand, HistogramDistance};
use brotli_decompressor::CustomWrite;
use super::interface;
#[cfg(not(feature="no-stdlib"))]
pub use brotli_decompressor::{IntoIoWriter, IoWriterWrapper};

pub use alloc::{AllocatedStackMemory, Allocator, SliceWrapper, SliceWrapperMut, StackAllocator};
#[cfg(not(feature="no-stdlib"))]
pub use alloc::HeapAlloc;
#[cfg(not(feature="no-stdlib"))]
use std::io;

#[cfg(not(feature="no-stdlib"))]
use std::io::{Write, Error, ErrorKind};




#[cfg(not(feature="no-stdlib"))]
pub struct CompressorWriterCustomAlloc<W: Write,
                                       BufferType : SliceWrapperMut<u8>,
                                       AllocU8: Allocator<u8>,
                                       AllocU16: Allocator<u16>,
                                       AllocI32: Allocator<i32>,
                                       AllocU32: Allocator<u32>,
                                       AllocU64: Allocator<u64>,
                                       AllocCommand: Allocator<Command>,
                                       AllocF64: Allocator<super::util::floatX>,
                                       AllocFV: Allocator<Mem256f>,
                                       AllocPDF: Allocator<PDF>,
                                       AllocStaticCommand: Allocator<StaticCommand>,
                                       AllocHL: Allocator<HistogramLiteral>,
                                       AllocHC: Allocator<HistogramCommand>,
                                       AllocHD: Allocator<HistogramDistance>,
                                       AllocHP: Allocator<HistogramPair>,
                                       AllocCT: Allocator<ContextType>,
                                       AllocHT: Allocator<HuffmanTree>,
                                       AllocZN: Allocator<ZopfliNode>> (
    CompressorWriterCustomIo<io::Error,
                             IntoIoWriter<W>,
                             BufferType,
                             AllocU8, AllocU16, AllocI32, AllocU32, AllocU64, AllocCommand, AllocF64, AllocFV, AllocPDF, AllocStaticCommand,
                             AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT, AllocZN>);


#[cfg(not(feature="no-stdlib"))]
impl<W: Write,
     BufferType : SliceWrapperMut<u8>,
     AllocU8: Allocator<u8>,
     AllocU16: Allocator<u16>,
     AllocI32: Allocator<i32>,
     AllocU32: Allocator<u32>,
     AllocU64: Allocator<u64>,
     AllocCommand: Allocator<Command>,
     AllocF64: Allocator<super::util::floatX>,
     AllocFV: Allocator<Mem256f>,
     AllocPDF: Allocator<PDF>,
     AllocStaticCommand: Allocator<StaticCommand>,
     AllocHL: Allocator<HistogramLiteral>,
     AllocHC: Allocator<HistogramCommand>,
     AllocHD: Allocator<HistogramDistance>,
     AllocHP: Allocator<HistogramPair>,
     AllocCT: Allocator<ContextType>,
     AllocHT: Allocator<HuffmanTree>,
     AllocZN: Allocator<ZopfliNode>>
    CompressorWriterCustomAlloc<W, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocU64, AllocCommand,
                                AllocF64, AllocFV, AllocPDF, AllocStaticCommand, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT, AllocZN>
    {

    pub fn new(w: W, buffer : BufferType,
               alloc_u8 : AllocU8,
               alloc_u16 : AllocU16,
               alloc_i32 : AllocI32,
               alloc_u32 : AllocU32,
               alloc_u64 : AllocU64,
               alloc_c : AllocCommand,
               alloc_f64 : AllocF64,
               alloc_fv : AllocFV,
               alloc_pdf : AllocPDF,
               alloc_sc : AllocStaticCommand,
               alloc_hl:AllocHL,
               alloc_hc:AllocHC,
               alloc_hd:AllocHD,
               alloc_hp:AllocHP,
               alloc_ct:AllocCT,
               alloc_ht:AllocHT,
               alloc_zn:AllocZN,
               q: u32,
               lgwin: u32) -> Self {
        CompressorWriterCustomAlloc::<W, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocU64, AllocCommand,
                                AllocF64,AllocFV, AllocPDF, AllocStaticCommand,AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT, AllocZN>(
          CompressorWriterCustomIo::<Error,
                                 IntoIoWriter<W>,
                                 BufferType,
                                 AllocU8, AllocU16, AllocI32, AllocU32, AllocU64, AllocCommand,
                                 AllocF64,AllocFV, AllocPDF, AllocStaticCommand, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT, AllocZN>::new(
              IntoIoWriter::<W>(w),
              buffer,
              alloc_u8, alloc_u16, alloc_i32, alloc_u32, alloc_u64, alloc_c,
              alloc_f64, alloc_fv, alloc_pdf, alloc_sc, alloc_hl, alloc_hc, alloc_hd, alloc_hp, alloc_ct,alloc_ht, alloc_zn,
              Error::new(ErrorKind::InvalidData,
                         "Invalid Data"),
              q, lgwin))
    }

    pub fn get_ref(&self) -> &W {
      &self.0.get_ref().0
    }
}

#[cfg(not(feature="no-stdlib"))]
impl<W: Write,
     BufferType: SliceWrapperMut<u8>,
     AllocU8: Allocator<u8>,
     AllocU16: Allocator<u16>,
     AllocI32: Allocator<i32>,
     AllocU32: Allocator<u32>,
     AllocU64: Allocator<u64>,
     AllocCommand: Allocator<Command>,
     AllocF64: Allocator<super::util::floatX>,
     AllocFV: Allocator<Mem256f>,
     AllocPDF: Allocator<PDF>,
     AllocStaticCommand: Allocator<StaticCommand>,
     AllocHL: Allocator<HistogramLiteral>,
     AllocHC: Allocator<HistogramCommand>,
     AllocHD: Allocator<HistogramDistance>,
     AllocHP: Allocator<HistogramPair>,
     AllocCT: Allocator<ContextType>,
     AllocHT: Allocator<HuffmanTree>,
     AllocZN: Allocator<ZopfliNode>>
    Write for CompressorWriterCustomAlloc<W, BufferType,
                                         AllocU8, AllocU16, AllocI32, AllocU32, AllocU64, AllocCommand, AllocF64, AllocFV, AllocPDF, AllocStaticCommand,
                                         AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT, AllocZN> {
  	fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
       self.0.write(buf)
    }
    fn flush(&mut self) -> Result<(), Error> {
       self.0.flush()
    }
}


#[cfg(not(any(feature="no-stdlib")))]
pub struct CompressorWriter<W: Write>(CompressorWriterCustomAlloc<W,
                                     <HeapAlloc<u8>
                                      as Allocator<u8>>::AllocatedMemory,
                                     HeapAlloc<u8>,
                                     HeapAlloc<u16>,
                                     HeapAlloc<i32>,
                                     HeapAlloc<u32>,
                                     HeapAlloc<u64>,
                                     HeapAlloc<Command>,
                                     HeapAlloc<super::util::floatX>,
                                     HeapAlloc<Mem256f>,
                                     HeapAlloc<PDF>,
                                     HeapAlloc<StaticCommand>,
                                     HeapAlloc<HistogramLiteral>,
                                     HeapAlloc<HistogramCommand>,
                                     HeapAlloc<HistogramDistance>,
                                     HeapAlloc<HistogramPair>,
                                     HeapAlloc<ContextType>,
                                     HeapAlloc<HuffmanTree>,
                                     HeapAlloc<ZopfliNode>>);


#[cfg(not(any(feature="no-stdlib")))]
impl<W: Write> CompressorWriter<W> {
  pub fn new(w: W, buffer_size: usize, q: u32, lgwin: u32) -> Self {
    let mut alloc_u8 = HeapAlloc::<u8> { default_value: 0 };
    let buffer = alloc_u8.alloc_cell(if buffer_size == 0 { 4096} else {buffer_size});
    let alloc_u16 = HeapAlloc::<u16> { default_value: 0 };
    let alloc_i32 = HeapAlloc::<i32> { default_value: 0 };
    let alloc_u32 = HeapAlloc::<u32> { default_value: 0 };
    let alloc_u64 = HeapAlloc::<u64> { default_value: 0 };
    let alloc_c = HeapAlloc::<Command> { default_value: Command::default() };
    let alloc_f64 = HeapAlloc::<super::util::floatX> { default_value: 0.0 as super::util::floatX };
    let alloc_fv = HeapAlloc::<Mem256f> { default_value: Mem256f::default() };
    let alloc_pdf = HeapAlloc::<PDF> { default_value: PDF::default() };
    let alloc_sc = HeapAlloc::<StaticCommand> { default_value: StaticCommand::default() };
    let alloc_hl = HeapAlloc::<HistogramLiteral> { default_value: HistogramLiteral::default() };
    let alloc_hc = HeapAlloc::<HistogramCommand> { default_value: HistogramCommand::default() };
    let alloc_hd = HeapAlloc::<HistogramDistance> { default_value: HistogramDistance::default() };
    let alloc_hp = HeapAlloc::<HistogramPair> { default_value: HistogramPair::default() };
    let alloc_ct = HeapAlloc::<ContextType> { default_value: ContextType::default() };
    let alloc_ht = HeapAlloc::<HuffmanTree> { default_value: HuffmanTree::default() };
    let alloc_zn = HeapAlloc::<ZopfliNode> { default_value: ZopfliNode::default() };
    CompressorWriter::<W>(CompressorWriterCustomAlloc::new(w,
                                                           buffer,
                                                           alloc_u8,
                                                           alloc_u16,
                                                           alloc_i32,
                                                           alloc_u32,
                                                           alloc_u64,
                                                           alloc_c,
                                                           alloc_f64,
                                                           alloc_fv,
                                                           alloc_pdf,
                                                           alloc_sc,
                                                           alloc_hl,
                                                           alloc_hc,
                                                           alloc_hd,
                                                           alloc_hp,
                                                           alloc_ct,
                                                           alloc_ht,
                                                           alloc_zn,
                                                           q,
                                                           lgwin))
  }

  pub fn get_ref(&self) -> &W {
    self.0.get_ref()
  }
}



#[cfg(not(feature="no-stdlib"))]
impl<W: Write> Write for CompressorWriter<W> {
  fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
    self.0.write(buf)
  }
  fn flush(&mut self) -> Result<(), Error> {
    self.0.flush()
  }
}

pub struct CompressorWriterCustomIo<ErrType,
                                    W: CustomWrite<ErrType>,
                                    BufferType: SliceWrapperMut<u8>,
                                    AllocU8: Allocator<u8>,
                                    AllocU16: Allocator<u16>,
                                    AllocI32: Allocator<i32>,
                                    AllocU32: Allocator<u32>,
                                    AllocU64: Allocator<u64>,
                                    AllocCommand: Allocator<Command>,
                                    AllocF64: Allocator<super::util::floatX>,
                                    AllocFV: Allocator<Mem256f>,
                                    AllocPDF: Allocator<PDF>,
                                    AllocStaticCommand: Allocator<StaticCommand>,
                                    AllocHL: Allocator<HistogramLiteral>,
                                    AllocHC: Allocator<HistogramCommand>,
                                    AllocHD: Allocator<HistogramDistance>,
                                    AllocHP: Allocator<HistogramPair>,
                                    AllocCT: Allocator<ContextType>,
                                    AllocHT: Allocator<HuffmanTree>,
                                    AllocZN: Allocator<ZopfliNode>>
{
  output_buffer: BufferType,
  total_out: Option<usize>,
  output: W,
  error_if_invalid_data: Option<ErrType>,
  alloc_u64: AllocU64,
  alloc_f64: AllocF64,
  alloc_fv: AllocFV,
  alloc_pdf: AllocPDF,
  alloc_sc: AllocStaticCommand,
  alloc_hl: AllocHL,
  alloc_hc: AllocHC,
  alloc_hd: AllocHD,
  alloc_hp: AllocHP,
  alloc_ct: AllocCT,
  alloc_ht: AllocHT,
  alloc_zn: AllocZN,
  state: BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
}
pub fn write_all<ErrType, W: CustomWrite<ErrType>>(writer: &mut W, mut buf : &[u8]) -> Result<(), ErrType> {
    while buf.len() != 0 {
          match writer.write(buf) {
                Ok(bytes_written) => buf = &buf[bytes_written..],
                Err(e) => return Err(e),
          }
    }
    Ok(())
}
impl<ErrType,
     W: CustomWrite<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     AllocU8: Allocator<u8>,
     AllocU16: Allocator<u16>,
     AllocI32: Allocator<i32>,
     AllocU32: Allocator<u32>,
     AllocU64: Allocator<u64>,
     AllocCommand: Allocator<Command>,
     AllocF64: Allocator<super::util::floatX>,
     AllocFV: Allocator<Mem256f>,
     AllocPDF: Allocator<PDF>,
     AllocStaticCommand: Allocator<StaticCommand>,
     AllocHL: Allocator<HistogramLiteral>,
     AllocHC: Allocator<HistogramCommand>,
     AllocHD: Allocator<HistogramDistance>,
     AllocHP: Allocator<HistogramPair>,
     AllocCT: Allocator<ContextType>,
     AllocHT: Allocator<HuffmanTree>,
     AllocZN: Allocator<ZopfliNode>>
CompressorWriterCustomIo<ErrType, W, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocU64, AllocCommand,
                         AllocF64, AllocFV, AllocPDF, AllocStaticCommand, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT, AllocZN>
{

    pub fn new(w: W, buffer : BufferType,
                              alloc_u8 : AllocU8,
               alloc_u16 : AllocU16,
               alloc_i32 : AllocI32,
               alloc_u32 : AllocU32,
               alloc_u64 : AllocU64,
               alloc_c : AllocCommand,
               alloc_f64 : AllocF64,
               alloc_fv: AllocFV,
               alloc_pdf: AllocPDF,
               alloc_sc: AllocStaticCommand,
               alloc_hl:AllocHL,
               alloc_hc:AllocHC,
               alloc_hd:AllocHD,
               alloc_hp:AllocHP,
               alloc_ct:AllocCT,
               alloc_ht:AllocHT,
               alloc_zn:AllocZN,
               invalid_data_error_type : ErrType,
               q: u32,
               lgwin: u32) -> Self {
        let mut ret = CompressorWriterCustomIo{
            output_buffer : buffer,
            total_out : Some(0),
            output: w,
            state : BrotliEncoderCreateInstance(alloc_u8,
                                     alloc_u16,
                                     alloc_i32,
                                     alloc_u32,
                                     alloc_c),
            alloc_u64:alloc_u64,
            alloc_f64:alloc_f64,
            alloc_fv:alloc_fv,
            alloc_pdf:alloc_pdf,
            alloc_sc:alloc_sc,
            alloc_hl:alloc_hl,
            alloc_hc:alloc_hc,
            alloc_hd:alloc_hd,
            alloc_hp:alloc_hp,
            alloc_ct:alloc_ct,
            alloc_ht:alloc_ht,
            alloc_zn:alloc_zn,
            error_if_invalid_data : Some(invalid_data_error_type),
        };
        BrotliEncoderSetParameter(&mut ret.state,
                                  BrotliEncoderParameter::BROTLI_PARAM_QUALITY,
                                  q as (u32));
        BrotliEncoderSetParameter(&mut ret.state,
                                  BrotliEncoderParameter::BROTLI_PARAM_LGWIN,
                                  lgwin as (u32));

        ret
    }
    fn flush_or_close(&mut self, op:BrotliEncoderOperation) -> Result<(), ErrType>{
       let mut nop_callback = |_pm:&mut interface::PredictionModeContextMap<input_pair::InputReferenceMut>,
                               _queue:&mut [interface::Command<interface::SliceOffset>],
                               _mb:interface::InputPair|();

        loop {
            let mut avail_in : usize = 0;
            let mut input_offset : usize = 0;
            let mut avail_out : usize = self.output_buffer.slice_mut().len();
            let mut output_offset : usize = 0;
            let ret = BrotliEncoderCompressStream(
                &mut self.state,
                &mut self.alloc_u64,
                &mut self.alloc_f64,
                &mut self.alloc_fv,
                &mut self.alloc_pdf,
                &mut self.alloc_sc,
                &mut self.alloc_hl,
                &mut self.alloc_hc,
                &mut self.alloc_hd,
                &mut self.alloc_hp,
                &mut self.alloc_ct,
                &mut self.alloc_ht,
                &mut self.alloc_zn,
                op,
                &mut avail_in,
                &[],
                &mut input_offset,
                &mut avail_out,
                self.output_buffer.slice_mut(),
                &mut output_offset,
                &mut self.total_out,
                &mut nop_callback);
           if output_offset > 0 {
             match write_all(&mut self.output, &self.output_buffer.slice_mut()[..output_offset]) {
               Ok(_) => {},
               Err(e) => return Err(e),
             }
           }
           if ret <= 0 {
              return Err(self.error_if_invalid_data.take().unwrap());
           }
           if let BrotliEncoderOperation::BROTLI_OPERATION_FLUSH = op {
              return Ok(());
           }
           if BrotliEncoderIsFinished(&mut self.state) != 0 {
              return Ok(());
           }
        }        
    }
    
    pub fn get_ref(&self) -> &W {
      &self.output
    }
}

impl<ErrType,
     W: CustomWrite<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     AllocU8: Allocator<u8>,
     AllocU16: Allocator<u16>,
     AllocI32: Allocator<i32>,
     AllocU32: Allocator<u32>,
     AllocU64: Allocator<u64>,
     AllocCommand: Allocator<Command>,
     AllocF64: Allocator<super::util::floatX>,
     AllocFV: Allocator<Mem256f>,
     AllocPDF: Allocator<PDF>,
     AllocStaticCommand: Allocator<StaticCommand>,
     AllocHL: Allocator<HistogramLiteral>,
     AllocHC: Allocator<HistogramCommand>,
     AllocHD: Allocator<HistogramDistance>,
     AllocHP: Allocator<HistogramPair>,
     AllocCT: Allocator<ContextType>,
     AllocHT: Allocator<HuffmanTree>,
     AllocZN: Allocator<ZopfliNode>> Drop for
CompressorWriterCustomIo<ErrType, W, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocU64, AllocCommand,
                         AllocF64, AllocFV, AllocPDF, AllocStaticCommand, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT, AllocZN> {
    fn drop(&mut self) {
        match self.flush_or_close(BrotliEncoderOperation::BROTLI_OPERATION_FINISH) {
              Ok(_) => {},
              Err(_) => {},
        }
        BrotliEncoderDestroyInstance(&mut self.state);
    }
}
impl<ErrType,
     W: CustomWrite<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     AllocU8: Allocator<u8>,
     AllocU16: Allocator<u16>,
     AllocI32: Allocator<i32>,
     AllocU32: Allocator<u32>,
     AllocU64: Allocator<u64>,
     AllocCommand: Allocator<Command>,
     AllocF64: Allocator<super::util::floatX>,
     AllocFV: Allocator<Mem256f>,
     AllocPDF: Allocator<PDF>,
     AllocStaticCommand: Allocator<StaticCommand>,
     AllocHL: Allocator<HistogramLiteral>,
     AllocHC: Allocator<HistogramCommand>,
     AllocHD: Allocator<HistogramDistance>,
     AllocHP: Allocator<HistogramPair>,
     AllocCT: Allocator<ContextType>,
     AllocHT: Allocator<HuffmanTree>,
     AllocZN: Allocator<ZopfliNode>> CustomWrite<ErrType> for
CompressorWriterCustomIo<ErrType, W, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocU64, AllocCommand,
                         AllocF64, AllocFV, AllocPDF, AllocStaticCommand, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT, AllocZN> {
	fn write(&mut self, buf: & [u8]) -> Result<usize, ErrType > {
        let mut nop_callback = |_pm:&mut interface::PredictionModeContextMap<input_pair::InputReferenceMut>,
                                _queue:&mut [interface::Command<interface::SliceOffset>],
                                _mb:interface::InputPair|();
        let mut avail_in = buf.len();
        let mut input_offset : usize = 0;
        while avail_in != 0 {
            let mut output_offset = 0;
            let mut avail_out = self.output_buffer.slice_mut().len();
            let ret = BrotliEncoderCompressStream(
                &mut self.state,
                &mut self.alloc_u64,
                &mut self.alloc_f64,
                &mut self.alloc_fv,
                &mut self.alloc_pdf,
                &mut self.alloc_sc,
                &mut self.alloc_hl,
                &mut self.alloc_hc,
                &mut self.alloc_hd,
                &mut self.alloc_hp,
                &mut self.alloc_ct,
                &mut self.alloc_ht,
                &mut self.alloc_zn,
                BrotliEncoderOperation::BROTLI_OPERATION_PROCESS,
                &mut avail_in,
                &buf[..],
                &mut input_offset,
                &mut avail_out,
                self.output_buffer.slice_mut(),
                &mut output_offset,
                &mut self.total_out,
                &mut nop_callback);
           if output_offset > 0 {
             match write_all(&mut self.output, &self.output_buffer.slice_mut()[..output_offset]) {
              Ok(_) => {},
              Err(e) => return Err(e),
             }
           }
           if ret <= 0 {
              return Err(self.error_if_invalid_data.take().unwrap());
           }
        }
        Ok(buf.len())
      }
      fn flush(&mut self) -> Result<(), ErrType > {
        match self.flush_or_close(BrotliEncoderOperation::BROTLI_OPERATION_FLUSH) {
              Ok(_) => {},
              Err(e) => return Err(e),
        }
        self.output.flush()
      }
}
