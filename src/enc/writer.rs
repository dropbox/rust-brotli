use super::vectorization::Mem256f;
use super::cluster::HistogramPair;
use super::command::Command;
use super::encode::{BrotliEncoderCreateInstance, BrotliEncoderDestroyInstance,
                    BrotliEncoderParameter, BrotliEncoderSetParameter, BrotliEncoderOperation,
                    BrotliEncoderStateStruct, BrotliEncoderCompressStream, BrotliEncoderIsFinished};
use super::entropy_encode::HuffmanTree;
use super::histogram::{ContextType, HistogramLiteral, HistogramCommand, HistogramDistance};
use brotli_decompressor::CustomWrite;
use super::brotli_bit_stream;
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
                                       AllocCommand: Allocator<Command>,
                                       AllocF64: Allocator<super::util::floatX>,
                                       AllocFV: Allocator<Mem256f>,
                                       AllocHL: Allocator<HistogramLiteral>,
                                       AllocHC: Allocator<HistogramCommand>,
                                       AllocHD: Allocator<HistogramDistance>,
                                       AllocHP: Allocator<HistogramPair>,
                                       AllocCT: Allocator<ContextType>,
                                       AllocHT: Allocator<HuffmanTree>> (
    CompressorWriterCustomIo<io::Error,
                             IntoIoWriter<W>,
                             BufferType,
                             AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand, AllocF64, AllocFV,
                             AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT>);


#[cfg(not(feature="no-stdlib"))]
impl<W: Write,
     BufferType : SliceWrapperMut<u8>,
     AllocU8: Allocator<u8>,
     AllocU16: Allocator<u16>,
     AllocI32: Allocator<i32>,
     AllocU32: Allocator<u32>,
     AllocCommand: Allocator<Command>,
     AllocF64: Allocator<super::util::floatX>,
     AllocFV: Allocator<Mem256f>,
     AllocHL: Allocator<HistogramLiteral>,
     AllocHC: Allocator<HistogramCommand>,
     AllocHD: Allocator<HistogramDistance>,
     AllocHP: Allocator<HistogramPair>,
     AllocCT: Allocator<ContextType>,
     AllocHT: Allocator<HuffmanTree>>
    CompressorWriterCustomAlloc<W, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand,
                                AllocF64, AllocFV, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT>
    {

    pub fn new(w: W, buffer : BufferType,
               alloc_u8 : AllocU8,
               alloc_u16 : AllocU16,
               alloc_i32 : AllocI32,
               alloc_u32 : AllocU32,
               alloc_c : AllocCommand,
               alloc_f64 : AllocF64,
               alloc_fv : AllocFV,
               alloc_hl:AllocHL,
               alloc_hc:AllocHC,
               alloc_hd:AllocHD,
               alloc_hp:AllocHP,
               alloc_ct:AllocCT,
               alloc_ht:AllocHT,
               q: u32,
               lgwin: u32) -> Self {
        CompressorWriterCustomAlloc::<W, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand,
                                AllocF64,AllocFV, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT>(
          CompressorWriterCustomIo::<Error,
                                 IntoIoWriter<W>,
                                 BufferType,
                                 AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand,
                                 AllocF64,AllocFV, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT>::new(
              IntoIoWriter::<W>(w),
              buffer,
              alloc_u8, alloc_u16, alloc_i32, alloc_u32, alloc_c,
              alloc_f64, alloc_fv, alloc_hl, alloc_hc, alloc_hd, alloc_hp, alloc_ct,alloc_ht,
              Error::new(ErrorKind::InvalidData,
                         "Invalid Data"),
              q, lgwin))
    }
    }

#[cfg(not(feature="no-stdlib"))]
impl<W: Write,
     BufferType: SliceWrapperMut<u8>,
     AllocU8: Allocator<u8>,
     AllocU16: Allocator<u16>,
     AllocI32: Allocator<i32>,
     AllocU32: Allocator<u32>,
     AllocCommand: Allocator<Command>,
     AllocF64: Allocator<super::util::floatX>,
     AllocFV: Allocator<Mem256f>,
     AllocHL: Allocator<HistogramLiteral>,
     AllocHC: Allocator<HistogramCommand>,
     AllocHD: Allocator<HistogramDistance>,
     AllocHP: Allocator<HistogramPair>,
     AllocCT: Allocator<ContextType>,
     AllocHT: Allocator<HuffmanTree>>
    Write for CompressorWriterCustomAlloc<W, BufferType,
                                         AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand, AllocF64, AllocFV,
                                         AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT> {
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
                                     HeapAlloc<Command>,
                                     HeapAlloc<super::util::floatX>,
                                     HeapAlloc<Mem256f>,
                                     HeapAlloc<HistogramLiteral>,
                                     HeapAlloc<HistogramCommand>,
                                     HeapAlloc<HistogramDistance>,
                                     HeapAlloc<HistogramPair>,
                                     HeapAlloc<ContextType>,
                                     HeapAlloc<HuffmanTree>>);


#[cfg(not(any(feature="no-stdlib")))]
impl<W: Write> CompressorWriter<W> {
  pub fn new(w: W, buffer_size: usize, q: u32, lgwin: u32) -> Self {
    let mut alloc_u8 = HeapAlloc::<u8> { default_value: 0 };
    let buffer = alloc_u8.alloc_cell(buffer_size);
    let alloc_u16 = HeapAlloc::<u16> { default_value: 0 };
    let alloc_i32 = HeapAlloc::<i32> { default_value: 0 };
    let alloc_u32 = HeapAlloc::<u32> { default_value: 0 };
    let alloc_c = HeapAlloc::<Command> { default_value: Command::default() };
    let alloc_f64 = HeapAlloc::<super::util::floatX> { default_value: 0.0 as super::util::floatX };
    let alloc_fv = HeapAlloc::<Mem256f> { default_value: Mem256f::default() };
    let alloc_hl = HeapAlloc::<HistogramLiteral> { default_value: HistogramLiteral::default() };
    let alloc_hc = HeapAlloc::<HistogramCommand> { default_value: HistogramCommand::default() };
    let alloc_hd = HeapAlloc::<HistogramDistance> { default_value: HistogramDistance::default() };
    let alloc_hp = HeapAlloc::<HistogramPair> { default_value: HistogramPair::default() };
    let alloc_ct = HeapAlloc::<ContextType> { default_value: ContextType::default() };
    let alloc_ht = HeapAlloc::<HuffmanTree> { default_value: HuffmanTree::default() };
    CompressorWriter::<W>(CompressorWriterCustomAlloc::new(w,
                                                           buffer,
                                                           alloc_u8,
                                                           alloc_u16,
                                                           alloc_i32,
                                                           alloc_u32,
                                                           alloc_c,
                                                           alloc_f64,
                                                           alloc_fv,
                                                           alloc_hl,
                                                           alloc_hc,
                                                           alloc_hd,
                                                           alloc_hp,
                                                           alloc_ct,
                                                           alloc_ht,
                                                           q,
                                                           lgwin))
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
                                    AllocCommand: Allocator<Command>,
                                    AllocF64: Allocator<super::util::floatX>,
                                    AllocFV: Allocator<Mem256f>,
                                    AllocHL: Allocator<HistogramLiteral>,
                                    AllocHC: Allocator<HistogramCommand>,
                                    AllocHD: Allocator<HistogramDistance>,
                                    AllocHP: Allocator<HistogramPair>,
                                    AllocCT: Allocator<ContextType>,
                                    AllocHT: Allocator<HuffmanTree>>
{
  output_buffer: BufferType,
  total_out: Option<usize>,
  output: W,
  error_if_invalid_data: Option<ErrType>,
  alloc_f64: AllocF64,
  alloc_fv: AllocFV,
  alloc_hl: AllocHL,
  alloc_hc: AllocHC,
  alloc_hd: AllocHD,
  alloc_hp: AllocHP,
  alloc_ct: AllocCT,
  alloc_ht: AllocHT,
  state: BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
}
pub fn write_all<ErrType, W: CustomWrite<ErrType>>(mut writer: &mut W, mut buf : &[u8]) -> Result<(), ErrType> {
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
     AllocCommand: Allocator<Command>,
     AllocF64: Allocator<super::util::floatX>,
     AllocFV: Allocator<Mem256f>,
     AllocHL: Allocator<HistogramLiteral>,
     AllocHC: Allocator<HistogramCommand>,
     AllocHD: Allocator<HistogramDistance>,
     AllocHP: Allocator<HistogramPair>,
     AllocCT: Allocator<ContextType>,
     AllocHT: Allocator<HuffmanTree>>
CompressorWriterCustomIo<ErrType, W, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand,
                         AllocF64, AllocFV, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT>
{

    pub fn new(w: W, buffer : BufferType,
                              alloc_u8 : AllocU8,
               alloc_u16 : AllocU16,
               alloc_i32 : AllocI32,
               alloc_u32 : AllocU32,
               alloc_c : AllocCommand,
               alloc_f64 : AllocF64,
               alloc_fv: AllocFV,
               alloc_hl:AllocHL,
               alloc_hc:AllocHC,
               alloc_hd:AllocHD,
               alloc_hp:AllocHP,
               alloc_ct:AllocCT,
               alloc_ht:AllocHT,
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
            alloc_f64:alloc_f64,
            alloc_fv:alloc_fv,
            alloc_hl:alloc_hl,
            alloc_hc:alloc_hc,
            alloc_hd:alloc_hd,
            alloc_hp:alloc_hp,
            alloc_ct:alloc_ct,
            alloc_ht:alloc_ht,
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
        let mut nop_callback = |_data:&[interface::Command<brotli_bit_stream::InputReference>]|();
        loop {
            let mut avail_in : usize = 0;
            let mut input_offset : usize = 0;
            let mut avail_out : usize = self.output_buffer.slice_mut().len();
            let mut output_offset : usize = 0;
            let ret = BrotliEncoderCompressStream(
                &mut self.state,
                &mut self.alloc_f64,
                &mut self.alloc_fv,
                &mut self.alloc_hl,
                &mut self.alloc_hc,
                &mut self.alloc_hd,
                &mut self.alloc_hp,
                &mut self.alloc_ct,
                &mut self.alloc_ht,
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

           if BrotliEncoderIsFinished(&mut self.state) != 0 {
              return Ok(());
           }
        }        
    }
}

impl<ErrType,
     W: CustomWrite<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     AllocU8: Allocator<u8>,
     AllocU16: Allocator<u16>,
     AllocI32: Allocator<i32>,
     AllocU32: Allocator<u32>,
     AllocCommand: Allocator<Command>,
     AllocF64: Allocator<super::util::floatX>,
     AllocFV: Allocator<Mem256f>,
     AllocHL: Allocator<HistogramLiteral>,
     AllocHC: Allocator<HistogramCommand>,
     AllocHD: Allocator<HistogramDistance>,
     AllocHP: Allocator<HistogramPair>,
     AllocCT: Allocator<ContextType>,
     AllocHT: Allocator<HuffmanTree>> Drop for
CompressorWriterCustomIo<ErrType, W, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand,
                         AllocF64, AllocFV, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT> {
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
     AllocCommand: Allocator<Command>,
     AllocF64: Allocator<super::util::floatX>,
     AllocFV: Allocator<Mem256f>,
     AllocHL: Allocator<HistogramLiteral>,
     AllocHC: Allocator<HistogramCommand>,
     AllocHD: Allocator<HistogramDistance>,
     AllocHP: Allocator<HistogramPair>,
     AllocCT: Allocator<ContextType>,
     AllocHT: Allocator<HuffmanTree>> CustomWrite<ErrType> for
CompressorWriterCustomIo<ErrType, W, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand,
                         AllocF64, AllocFV, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT> {
	fn write(&mut self, buf: & [u8]) -> Result<usize, ErrType > {
        let mut nop_callback = |_data:&[interface::Command<brotli_bit_stream::InputReference>]|();
        let mut avail_in = buf.len();
        let mut input_offset : usize = 0;
        while avail_in != 0 {
            let mut output_offset = 0;
            let mut avail_out = self.output_buffer.slice_mut().len();
            let ret = BrotliEncoderCompressStream(
                &mut self.state,
                &mut self.alloc_f64,
                &mut self.alloc_fv,
                &mut self.alloc_hl,
                &mut self.alloc_hc,
                &mut self.alloc_hd,
                &mut self.alloc_hp,
                &mut self.alloc_ct,
                &mut self.alloc_ht,
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
