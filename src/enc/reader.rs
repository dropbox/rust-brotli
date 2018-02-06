use super::vectorization::Mem256f;
use super::cluster::HistogramPair;
use super::command::Command;
use super::encode::{BrotliEncoderCreateInstance, BrotliEncoderDestroyInstance,
                    BrotliEncoderParameter, BrotliEncoderSetParameter, BrotliEncoderOperation, BrotliEncoderSetCustomDictionary,
                    BrotliEncoderStateStruct, BrotliEncoderCompressStream, BrotliEncoderIsFinished};
use super::entropy_encode::HuffmanTree;
use super::histogram::{ContextType, HistogramLiteral, HistogramCommand, HistogramDistance};
use super::interface;
use super::brotli_bit_stream;
use brotli_decompressor::CustomRead;
#[cfg(not(feature="no-stdlib"))]
pub use brotli_decompressor::{IntoIoReader, IoReaderWrapper, IoWriterWrapper};

pub use alloc::{AllocatedStackMemory, Allocator, SliceWrapper, SliceWrapperMut, StackAllocator};
#[cfg(not(feature="no-stdlib"))]
pub use alloc::HeapAlloc;
#[cfg(not(feature="no-stdlib"))]
use std::io;

#[cfg(not(feature="no-stdlib"))]
use std::io::{Read, Error, ErrorKind};




#[cfg(not(feature="no-stdlib"))]
pub struct CompressorReaderCustomAlloc<R: Read,
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
    CompressorReaderCustomIo<io::Error,
                             IntoIoReader<R>,
                             BufferType,
                             AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand, AllocF64, AllocFV,
                             AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT>);


#[cfg(not(feature="no-stdlib"))]
impl<R: Read,
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
    CompressorReaderCustomAlloc<R, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand,
                                AllocF64, AllocFV, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT>
    {

    pub fn new(r: R, buffer : BufferType,
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
        CompressorReaderCustomAlloc::<R, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand,
                                AllocF64, AllocFV, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT>(
          CompressorReaderCustomIo::<Error,
                                 IntoIoReader<R>,
                                 BufferType,
                                 AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand,
                                 AllocF64, AllocFV, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT>::new(
              IntoIoReader::<R>(r),
              buffer,
              alloc_u8, alloc_u16, alloc_i32, alloc_u32, alloc_c,
              alloc_f64, alloc_fv, alloc_hl, alloc_hc, alloc_hd, alloc_hp, alloc_ct,alloc_ht,
              Error::new(ErrorKind::InvalidData,
                         "Invalid Data"),
              q, lgwin))
    }

    pub fn get_ref(&self) -> &R {
        &self.0.get_ref().0
    }
}

#[cfg(not(feature="no-stdlib"))]
impl<R: Read,
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
    Read for CompressorReaderCustomAlloc<R, BufferType,
                                         AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand, AllocF64, AllocFV,
                                         AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT> {
  	fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
       self.0.read(buf)
    }
}


#[cfg(not(any(feature="no-stdlib")))]
pub struct CompressorReader<R: Read>(CompressorReaderCustomAlloc<R,
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
impl<R: Read> CompressorReader<R> {
  pub fn new(r: R, buffer_size: usize, q: u32, lgwin: u32) -> Self {
    let mut alloc_u8 = HeapAlloc::<u8> { default_value: 0 };
    let buffer = alloc_u8.alloc_cell(if buffer_size == 0 {4096} else {buffer_size});
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
    CompressorReader::<R>(CompressorReaderCustomAlloc::new(r,
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

  pub fn get_ref(&self) -> &R {
      self.0.get_ref()
  }
}



#[cfg(not(feature="no-stdlib"))]
impl<R: Read> Read for CompressorReader<R> {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
    self.0.read(buf)
  }
}

pub struct CompressorReaderCustomIo<ErrType,
                                    R: CustomRead<ErrType>,
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
  input_buffer: BufferType,
  total_out: Option<usize>,
  input_offset: usize,
  input_len: usize,
  input_eof: bool,
  input: R,
  error_if_invalid_data: Option<ErrType>,
  read_error: Option<ErrType>,
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

impl<ErrType,
     R: CustomRead<ErrType>,
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
CompressorReaderCustomIo<ErrType, R, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand,
                         AllocF64, AllocFV, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT>
{

    pub fn new(r: R, buffer : BufferType,
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
               invalid_data_error_type : ErrType,
               q: u32,
               lgwin: u32) -> Self {
         Self::new_with_custom_dictionary(r, buffer, alloc_u8, alloc_u16, alloc_i32, alloc_u32, alloc_c, alloc_f64, alloc_fv,
                                    alloc_hl, alloc_hc, alloc_hd, alloc_hp, alloc_ct, alloc_ht,
                                    invalid_data_error_type, q, lgwin,
                                    &[], &[])
    }
    pub fn new_with_custom_dictionary(r: R, buffer : BufferType,
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
               invalid_data_error_type : ErrType,
               q: u32,
               lgwin: u32,
               dict: &[u8],
               dict_invalid: &[u8]) -> Self {
        let mut ret = CompressorReaderCustomIo{
            input_buffer : buffer,
            total_out : Some(0),
            input_offset : 0,
            input_len : 0,
            input_eof : false,
            input: r,
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
            read_error : None,
        };
        BrotliEncoderSetParameter(&mut ret.state,
                                  BrotliEncoderParameter::BROTLI_PARAM_QUALITY,
                                  q as (u32));
        BrotliEncoderSetParameter(&mut ret.state,
                                  BrotliEncoderParameter::BROTLI_PARAM_LGWIN,
                                  lgwin as (u32));
        if dict.len() != 0 {
            match BrotliEncoderSetCustomDictionary(&mut ret.state,
                                             dict.len(),
                                             dict,
                                             dict_invalid) {
               Ok(_) => {},
               Err(_) => ret.read_error = Some(ret.error_if_invalid_data.take().unwrap()),
            }
        }
        ret
    }
    pub fn copy_to_front(&mut self) {
        let avail_in = self.input_len - self.input_offset;
        if self.input_offset == self.input_buffer.slice_mut().len() {
            self.input_offset = 0;
            self.input_len = 0;
        } else if self.input_offset + 256 > self.input_buffer.slice_mut().len() && avail_in < self.input_offset {
            let (first, second) = self.input_buffer.slice_mut().split_at_mut(self.input_offset);
            first[0..avail_in].clone_from_slice(&second[0..avail_in]);
            self.input_len -= self.input_offset;
            self.input_offset = 0;
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.input
    }
}

impl<ErrType,
     R: CustomRead<ErrType>,
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
CompressorReaderCustomIo<ErrType, R, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand,
                         AllocF64, AllocFV, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT> {
    fn drop(&mut self) {
        BrotliEncoderDestroyInstance(&mut self.state);
    }
}
impl<ErrType,
     R: CustomRead<ErrType>,
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
     AllocHT: Allocator<HuffmanTree>> CustomRead<ErrType> for
CompressorReaderCustomIo<ErrType, R, BufferType, AllocU8, AllocU16, AllocI32, AllocU32, AllocCommand,
                         AllocF64, AllocFV, AllocHL, AllocHC, AllocHD, AllocHP, AllocCT, AllocHT> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize, ErrType > {
        let mut nop_callback = |_data:&[interface::Command<brotli_bit_stream::InputReference>]|();
        let mut output_offset : usize = 0;
        let mut avail_out = buf.len() - output_offset;
        let mut avail_in = self.input_len - self.input_offset;
        let mut needs_input = false;
        while avail_out == buf.len() && (!needs_input || !self.input_eof) {
            if self.input_len < self.input_buffer.slice_mut().len() && !self.input_eof {
                match self.input.read(&mut self.input_buffer.slice_mut()[self.input_len..]) {
                    Err(e) => {
                        self.read_error = Some(e);
                        self.input_eof = true;
                    },
                    Ok(size) => if size == 0 {
                        self.input_eof = true;
                    }else {
                        needs_input = false;
                        self.input_len += size;
                        avail_in = self.input_len - self.input_offset;
                    },
                }
            }
            let op : BrotliEncoderOperation;
            if avail_in == 0 {
                op = BrotliEncoderOperation::BROTLI_OPERATION_FINISH;
            } else {
                op = BrotliEncoderOperation::BROTLI_OPERATION_PROCESS;
            }
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
                &self.input_buffer.slice_mut()[..],
                &mut self.input_offset,
                &mut avail_out,
                buf,
                &mut output_offset,
                &mut self.total_out,
                &mut nop_callback);
          if avail_in == 0 {
            match self.read_error.take() {
              Some(err) => return Err(err),
              None => {
                needs_input = true;
                self.copy_to_front();
              },
            }
          }
          if ret <= 0 {
              return Err(self.error_if_invalid_data.take().unwrap());
          }
          let fin = BrotliEncoderIsFinished(&mut self.state);
          if fin != 0 {
              break;
          }
        }
        Ok(output_offset)
      }
}
