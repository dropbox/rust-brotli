use super::vectorization::Mem256f;
use super::cluster::HistogramPair;
use enc::PDF;
use enc::StaticCommand;
use super::command::Command;
use super::combined_alloc::{CombiningAllocator, BrotliAlloc};
use super::hash_to_binary_tree::ZopfliNode;
use super::encode::{BrotliEncoderCreateInstance, BrotliEncoderDestroyInstance,
                    BrotliEncoderParameter, BrotliEncoderSetParameter, BrotliEncoderOperation,
                    BrotliEncoderStateStruct, BrotliEncoderCompressStream, BrotliEncoderIsFinished};
use super::entropy_encode::HuffmanTree;
use super::histogram::{ContextType, HistogramLiteral, HistogramCommand, HistogramDistance};
use super::interface;
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
use enc::{s16, v8};



#[cfg(not(feature="no-stdlib"))]
pub struct CompressorReaderCustomAlloc<R: Read,
                                       BufferType : SliceWrapperMut<u8>,
                                       Alloc: BrotliAlloc> (
    CompressorReaderCustomIo<io::Error,
                             IntoIoReader<R>,
                             BufferType,
                             Alloc>);


#[cfg(not(feature="no-stdlib"))]
impl<R: Read,
     BufferType : SliceWrapperMut<u8>,
     Alloc: BrotliAlloc>
    CompressorReaderCustomAlloc<R, BufferType, Alloc>
    {

    pub fn new(r: R, buffer : BufferType,
               alloc: Alloc,
               q: u32,
               lgwin: u32) -> Self {
        CompressorReaderCustomAlloc::<R, BufferType, Alloc>(
          CompressorReaderCustomIo::<Error,
                                 IntoIoReader<R>,
                                 BufferType,
                                 Alloc>::new(
              IntoIoReader::<R>(r),
              buffer,
              alloc,
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
     Alloc: BrotliAlloc>
    Read for CompressorReaderCustomAlloc<R, BufferType,
                                         Alloc> {
  	fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
       self.0.read(buf)
    }
}


#[cfg(not(any(feature="no-stdlib")))]
pub struct CompressorReader<R: Read>(CompressorReaderCustomAlloc<R,
                                     <HeapAlloc<u8>
                                      as Allocator<u8>>::AllocatedMemory,
                                     CombiningAllocator<HeapAlloc<u8>,
                                                        HeapAlloc<u16>,
                                                        HeapAlloc<i32>,
                                                        HeapAlloc<u32>,
                                                        HeapAlloc<u64>,
                                                        HeapAlloc<Command>,
                                                        HeapAlloc<super::util::floatX>,
                                                        HeapAlloc<v8>,
                                                        HeapAlloc<s16>,
                                                        HeapAlloc<PDF>,
                                                        HeapAlloc<StaticCommand>,
                                                        HeapAlloc<HistogramLiteral>,
                                                        HeapAlloc<HistogramCommand>,
                                                        HeapAlloc<HistogramDistance>,
                                                        HeapAlloc<HistogramPair>,
                                                        HeapAlloc<ContextType>,
                                                        HeapAlloc<HuffmanTree>,
                                                        HeapAlloc<ZopfliNode>>>);


#[cfg(not(any(feature="no-stdlib")))]
impl<R: Read> CompressorReader<R> {
  pub fn new(r: R, buffer_size: usize, q: u32, lgwin: u32) -> Self {
    let mut alloc_u8 = HeapAlloc::<u8> { default_value: 0 };
    let buffer = alloc_u8.alloc_cell(if buffer_size == 0 {4096} else {buffer_size});
    let alloc_u16 = HeapAlloc::<u16> { default_value: 0 };
    let alloc_i32 = HeapAlloc::<i32> { default_value: 0 };
    let alloc_u32 = HeapAlloc::<u32> { default_value: 0 };
    let alloc_u64 = HeapAlloc::<u64> { default_value: 0 };
    let alloc_c = HeapAlloc::<Command> { default_value: Command::default() };
    let alloc_f64 = HeapAlloc::<super::util::floatX> { default_value: 0.0 as super::util::floatX };
    let alloc_fv = HeapAlloc::<Mem256f> { default_value: Mem256f::default() };
    let alloc_f8 = HeapAlloc::<v8> { default_value: v8::default() };
    let alloc_s16 = HeapAlloc::<s16> { default_value: s16::default() };
    let alloc_pdf = HeapAlloc::<PDF> { default_value: PDF::default() };
    let alloc_sc = HeapAlloc::<StaticCommand> { default_value: StaticCommand::default() };
    let alloc_hl = HeapAlloc::<HistogramLiteral> { default_value: HistogramLiteral::default() };
    let alloc_hc = HeapAlloc::<HistogramCommand> { default_value: HistogramCommand::default() };
    let alloc_hd = HeapAlloc::<HistogramDistance> { default_value: HistogramDistance::default() };
    let alloc_hp = HeapAlloc::<HistogramPair> { default_value: HistogramPair::default() };
    let alloc_ct = HeapAlloc::<ContextType> { default_value: ContextType::default() };
    let alloc_ht = HeapAlloc::<HuffmanTree> { default_value: HuffmanTree::default() };
    let alloc_zn = HeapAlloc::<ZopfliNode> { default_value: ZopfliNode::default() };
    CompressorReader::<R>(CompressorReaderCustomAlloc::new(r,
                                                           buffer,
                                                           CombiningAllocator::new(alloc_u8,
                                                                                   alloc_u16,
                                                                                   alloc_i32,
                                                                                   alloc_u32,
                                                                                   alloc_u64,
                                                                                   alloc_c,
                                                                                   alloc_f64,
                                                                                   alloc_f8,
                                                                                   alloc_s16,
                                                                                   alloc_pdf,
                                                                                   alloc_sc,
                                                                                   alloc_hl,
                                                                                   alloc_hc,
                                                                                   alloc_hd,
                                                                                   alloc_hp,
                                                                                   alloc_ct,
                                                                                   alloc_ht,
                                                                                   alloc_zn,
                                                           ), q, lgwin))
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
                                    Alloc: BrotliAlloc>
{
  input_buffer: BufferType,
  total_out: Option<usize>,
  input_offset: usize,
  input_len: usize,
  input_eof: bool,
  input: R,
  error_if_invalid_data: Option<ErrType>,
  read_error: Option<ErrType>,
  state: BrotliEncoderStateStruct<Alloc>,
}

impl<ErrType,
     R: CustomRead<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     Alloc: BrotliAlloc>
CompressorReaderCustomIo<ErrType, R, BufferType, Alloc>
{

    pub fn new(r: R, buffer : BufferType,
                              alloc : Alloc,
               invalid_data_error_type : ErrType,
               q: u32,
               lgwin: u32) -> Self {
        let mut ret = CompressorReaderCustomIo{
            input_buffer : buffer,
            total_out : Some(0),
            input_offset : 0,
            input_len : 0,
            input_eof : false,
            input: r,
            state : BrotliEncoderCreateInstance(alloc),
            error_if_invalid_data : Some(invalid_data_error_type),
            read_error : None,
        };
        BrotliEncoderSetParameter(&mut ret.state,
                                  BrotliEncoderParameter::BROTLI_PARAM_QUALITY,
                                  q as (u32));
        BrotliEncoderSetParameter(&mut ret.state,
                                  BrotliEncoderParameter::BROTLI_PARAM_LGWIN,
                                  lgwin as (u32));

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
     Alloc: BrotliAlloc> Drop for
CompressorReaderCustomIo<ErrType, R, BufferType, Alloc> {
    fn drop(&mut self) {
        BrotliEncoderDestroyInstance(&mut self.state);
    }
}
impl<ErrType,
     R: CustomRead<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     Alloc: BrotliAlloc> CustomRead<ErrType> for
CompressorReaderCustomIo<ErrType, R, BufferType, Alloc> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize, ErrType > {
        let mut nop_callback = |_data:&mut interface::PredictionModeContextMap<interface::InputReferenceMut>,
                                _cmds: &mut [interface::StaticCommand],
                                _mb: interface::InputPair, _mfv: &mut Alloc|();
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



/*

/////////////////BOGUS////////////////////////////////////
pub struct SimpleReader<R: Read,
                        BufferType: SliceWrapperMut<u8>>
{
  input_buffer: BufferType,
  total_out: Option<usize>,
  input_offset: usize,
  input_len: usize,
  input_eof: bool,
  input: R,
  error_if_invalid_data: Option<io::Error>,
  read_error: Option<io::Error>,
  state: BrotliEncoderStateStruct<HeapAlloc<u8>, HeapAlloc<u16>, HeapAlloc<u32>, HeapAlloc<i32>, HeapAlloc<Command>>,
}

impl<R: Read,
     BufferType : SliceWrapperMut<u8>>
SimpleReader<R, BufferType>
{

    pub fn new(r: R, buffer : BufferType,
               q: u32,
               lgwin: u32) -> Option<Self> {
        let mut ret = SimpleReader{
            input_buffer : buffer,
            total_out : Some(0),
            input_offset : 0,
            input_len : 0,
            input_eof : false,
            input: r,
            state : BrotliEncoderCreateInstance(HeapAlloc{default_value:0},
                                                HeapAlloc{default_value:0},
                                                HeapAlloc{default_value:0},
                                                HeapAlloc{default_value:0},
                                                HeapAlloc{default_value:Command::default()}),
            error_if_invalid_data : Some(Error::new(ErrorKind::InvalidData,
                         "Invalid Data")),
            read_error : None,
        };
        BrotliEncoderSetParameter(&mut ret.state,
                                  BrotliEncoderParameter::BROTLI_PARAM_QUALITY,
                                  q as (u32));
        BrotliEncoderSetParameter(&mut ret.state,
                                  BrotliEncoderParameter::BROTLI_PARAM_LGWIN,
                                  lgwin as (u32));

        Some(ret)
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

impl<R: Read,
     BufferType : SliceWrapperMut<u8>> Drop for
SimpleReader<R, BufferType> {
    fn drop(&mut self) {
        BrotliEncoderDestroyInstance(&mut self.state);
    }
}
impl<R: Read,
     BufferType : SliceWrapperMut<u8>> Read for
SimpleReader<R, BufferType> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let mut nop_callback = |_data:&[interface::Command<input_pair::InputReferenceMut>]|();
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
                &mut HeapAlloc{default_value:0},
                &mut HeapAlloc{default_value:0.0},
                &mut HeapAlloc{default_value:Mem256f::default()},
                &mut HeapAlloc{default_value:HistogramLiteral::default()},
                &mut HeapAlloc{default_value:HistogramCommand::default()},
                &mut HeapAlloc{default_value:HistogramDistance::default()},
                &mut HeapAlloc{default_value:HistogramPair::default()},
                &mut HeapAlloc{default_value:ContextType::default()},
                &mut HeapAlloc{default_value:HuffmanTree::default()},
                &mut HeapAlloc{default_value:ZopfliNode::default()},
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
 */
