#![cfg_attr(not(feature="std"), allow(unused_imports))]
use core;
use super::encode::{BrotliEncoderCreateInstance, BrotliEncoderDestroyInstance,
                    BrotliEncoderParameter, BrotliEncoderSetParameter, BrotliEncoderOperation,
                    BrotliEncoderStateStruct, BrotliEncoderCompressStream, BrotliEncoderIsFinished};
use super::backward_references::BrotliEncoderParams;
use brotli_decompressor::CustomWrite;
use super::interface;
#[cfg(feature="std")]
pub use brotli_decompressor::{IntoIoWriter, IoWriterWrapper};
use super::combined_alloc::BrotliAlloc;
pub use alloc::{AllocatedStackMemory, Allocator, SliceWrapper, SliceWrapperMut, StackAllocator};
#[cfg(feature="std")]
pub use alloc_stdlib::StandardAlloc;
#[cfg(feature="std")]
use std::io;

#[cfg(feature="std")]
use std::io::{Write, Error, ErrorKind};



#[cfg(feature="std")]
pub struct CompressorWriterCustomAlloc<W: Write,
                                       BufferType : SliceWrapperMut<u8>,
                                       Alloc: BrotliAlloc> (
    CompressorWriterCustomIo<io::Error,
                             IntoIoWriter<W>,
                             BufferType,
                             Alloc>);


#[cfg(feature="std")]
impl<W: Write,
     BufferType : SliceWrapperMut<u8>,
     Alloc: BrotliAlloc>
    CompressorWriterCustomAlloc<W, BufferType, Alloc>
    {

    pub fn new(w: W, buffer : BufferType,
               alloc: Alloc,
               q: u32,
               lgwin: u32) -> Self {
        CompressorWriterCustomAlloc::<W, BufferType, Alloc>(
          CompressorWriterCustomIo::<Error,
                                 IntoIoWriter<W>,
                                 BufferType,
                                 Alloc>::new(
              IntoIoWriter::<W>(w),
              buffer,
              alloc,
              Error::new(ErrorKind::InvalidData,
                         "Invalid Data"),
              q, lgwin))
    }

    pub fn get_ref(&self) -> &W {
      &self.0.get_ref().0
    }
    pub fn get_mut(&mut self) -> &mut W {
      &mut self.0.get_mut().0
    }
    pub fn into_inner(self) -> W {
      self.0.into_inner().0
    }
}

#[cfg(feature="std")]
impl<W: Write,
     BufferType: SliceWrapperMut<u8>,
     Alloc: BrotliAlloc>
    Write for CompressorWriterCustomAlloc<W, BufferType,
                                         Alloc> {
  	fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
       self.0.write(buf)
    }
    fn flush(&mut self) -> Result<(), Error> {
       self.0.flush()
    }
}


#[cfg(feature="std")]
pub struct CompressorWriter<W: Write>(CompressorWriterCustomAlloc<W,
                                     <StandardAlloc
                                      as Allocator<u8>>::AllocatedMemory,
                                     StandardAlloc>);


#[cfg(feature="std")]
impl<W: Write> CompressorWriter<W> {
  pub fn new(w: W, buffer_size: usize, q: u32, lgwin: u32) -> Self {
    let mut alloc = StandardAlloc::default();
    let buffer = <StandardAlloc as Allocator<u8>>::alloc_cell(&mut alloc, if buffer_size == 0 { 4096} else {buffer_size});
    CompressorWriter::<W>(CompressorWriterCustomAlloc::new(w,
                                                           buffer,
                                                           alloc,
                                                           q,
                                                           lgwin))
  }

  pub fn with_params(w: W, buffer_size: usize, params: &BrotliEncoderParams) -> Self {
    let mut writer = Self::new(w, buffer_size, params.quality as u32, params.lgwin as u32);
    (writer.0).0.state.params = params.clone();
    writer
  }

  pub fn get_ref(&self) -> &W {
    self.0.get_ref()
  }
  pub fn get_mut(&mut self) -> &mut W {
    self.0.get_mut()
  }
  pub fn into_inner(self) -> W {
    self.0.into_inner()
  }
}



#[cfg(feature="std")]
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
                                    Alloc: BrotliAlloc>
{
  output_buffer: BufferType,
  total_out: Option<usize>,
  output: Option<W>,
  error_if_invalid_data: Option<ErrType>,
  state: BrotliEncoderStateStruct<Alloc>,
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
     Alloc: BrotliAlloc>
CompressorWriterCustomIo<ErrType, W, BufferType, Alloc>
{

    pub fn new(w: W, buffer : BufferType,
               alloc : Alloc,
               invalid_data_error_type : ErrType,
               q: u32,
               lgwin: u32) -> Self {
        let mut ret = CompressorWriterCustomIo{
            output_buffer : buffer,
            total_out : Some(0),
            output: Some(w),
            state : BrotliEncoderCreateInstance(alloc),
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
       let mut nop_callback = |_data:&mut interface::PredictionModeContextMap<interface::InputReferenceMut>,
                               _cmds: &mut [interface::StaticCommand],
                               _mb: interface::InputPair, _mfv: &mut Alloc|();

        loop {
            let mut avail_in : usize = 0;
            let mut input_offset : usize = 0;
            let mut avail_out : usize = self.output_buffer.slice_mut().len();
            let mut output_offset : usize = 0;
            let ret = BrotliEncoderCompressStream(
                &mut self.state,
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
             match write_all(self.output.as_mut().unwrap(), &self.output_buffer.slice_mut()[..output_offset]) {
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
      self.output.as_ref().unwrap()
    }
    pub fn get_mut(&mut self) -> &mut W {
      self.output.as_mut().unwrap()
    }
    pub fn into_inner(mut self) -> W {
      match self.flush_or_close(BrotliEncoderOperation::BROTLI_OPERATION_FINISH) {
        Ok(_) => {},
        Err(_) => {},
      }
      core::mem::replace(&mut self.output, None).unwrap()
    }
}

impl<ErrType,
     W: CustomWrite<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     Alloc: BrotliAlloc> Drop for
CompressorWriterCustomIo<ErrType, W, BufferType, Alloc> {
    fn drop(&mut self) {
        if self.output.is_some() {
            match self.flush_or_close(BrotliEncoderOperation::BROTLI_OPERATION_FINISH) {
                Ok(_) => {},
              Err(_) => {},
            }
        }
        BrotliEncoderDestroyInstance(&mut self.state);
    }
}
impl<ErrType,
     W: CustomWrite<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     Alloc: BrotliAlloc> CustomWrite<ErrType> for
CompressorWriterCustomIo<ErrType, W, BufferType, Alloc> {
	fn write(&mut self, buf: & [u8]) -> Result<usize, ErrType > {
        let mut nop_callback = |_data:&mut interface::PredictionModeContextMap<interface::InputReferenceMut>,
                                _cmds: &mut [interface::StaticCommand],
                                _mb: interface::InputPair, _mfv: &mut Alloc|();
        let mut avail_in = buf.len();
        let mut input_offset : usize = 0;
        while avail_in != 0 {
            let mut output_offset = 0;
            let mut avail_out = self.output_buffer.slice_mut().len();
            let ret = BrotliEncoderCompressStream(
                &mut self.state,
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
             match write_all(self.output.as_mut().unwrap(), &self.output_buffer.slice_mut()[..output_offset]) {
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
        self.output.as_mut().unwrap().flush()
      }
}
