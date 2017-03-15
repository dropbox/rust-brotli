#![no_std]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[macro_use]
// <-- for debugging, remove xprintln from bit_reader and replace with println
#[cfg(not(feature="no-stdlib"))]
extern crate std;
#[cfg(not(feature="no-stdlib"))]
use std::io::{self, Error, ErrorKind, Read, Write};

#[macro_use]
extern crate alloc_no_stdlib as alloc;
pub use alloc::{AllocatedStackMemory, Allocator, SliceWrapper, SliceWrapperMut, StackAllocator};

#[cfg(not(feature="no-stdlib"))]
pub use alloc::HeapAlloc;
#[cfg(all(feature="unsafe",not(feature="no-stdlib")))]
pub use alloc::HeapAllocUninitialized;
#[macro_use]
mod memory;
mod dictionary;
mod enc;
#[macro_use]
mod bit_reader;
mod huffman;
mod state;
mod prefix;
mod context;
mod transform;
mod test;
mod decode;
mod io_wrappers;
pub use huffman::{HuffmanCode, HuffmanTreeGroup};
pub use state::BrotliState;
// use io_wrappers::write_all;
pub use io_wrappers::{CustomRead, CustomWrite};
#[cfg(not(feature="no-stdlib"))]
pub use io_wrappers::{IntoIoReader, IoReaderWrapper, IoWriterWrapper};

// interface
// pub fn BrotliDecompressStream(mut available_in: &mut usize,
//                               input_offset: &mut usize,
//                               input: &[u8],
//                               mut available_out: &mut usize,
//                               mut output_offset: &mut usize,
//                               mut output: &mut [u8],
//                               mut total_out: &mut usize,
//                               mut s: &mut BrotliState<AllocU8, AllocU32, AllocHC>);

pub use decode::{BrotliDecompressStream, BrotliResult};




#[cfg(not(any(feature="unsafe", feature="no-stdlib")))]
pub fn BrotliDecompress<InputType, OutputType>(r: &mut InputType,
                                               w: &mut OutputType)
                                               -> Result<(), io::Error>
  where InputType: Read,
        OutputType: Write
{
  let mut input_buffer: [u8; 4096] = [0; 4096];
  let mut output_buffer: [u8; 4096] = [0; 4096];
  BrotliDecompressCustomAlloc(r,
                              w,
                              &mut input_buffer[..],
                              &mut output_buffer[..],
                              HeapAlloc::<u8> { default_value: 0 },
                              HeapAlloc::<u32> { default_value: 0 },
                              HeapAlloc::<HuffmanCode> {
                                default_value: HuffmanCode::default(),
                              })
}

#[cfg(all(feature="unsafe",not(feature="no-stdlib")))]
pub fn BrotliDecompress<InputType, OutputType>(r: &mut InputType,
                                               w: &mut OutputType)
                                               -> Result<(), io::Error>
  where InputType: Read,
        OutputType: Write
{
  let mut input_buffer: [u8; 4096] = [0; 4096];
  let mut output_buffer: [u8; 4096] = [0; 4096];
  BrotliDecompressCustomAlloc(r,
                              w,
                              &mut input_buffer[..],
                              &mut output_buffer[..],
                              unsafe { HeapAllocUninitialized::<u8>::new() },
                              unsafe { HeapAllocUninitialized::<u32>::new() },
                              unsafe { HeapAllocUninitialized::<HuffmanCode>::new() })
}


#[cfg(not(feature="no-stdlib"))]
pub fn BrotliDecompressCustomAlloc<InputType,
                                   OutputType,
                                   AllocU8: Allocator<u8>,
                                   AllocU32: Allocator<u32>,
                                   AllocHC: Allocator<HuffmanCode>>
  (r: &mut InputType,
   mut w: &mut OutputType,
   input_buffer: &mut [u8],
   output_buffer: &mut [u8],
   alloc_u8: AllocU8,
   alloc_u32: AllocU32,
   alloc_hc: AllocHC)
   -> Result<(), io::Error>
  where InputType: Read,
        OutputType: Write
{
  BrotliDecompressCustomIo(&mut IoReaderWrapper::<InputType>(r),
                           &mut IoWriterWrapper::<OutputType>(w),
                           input_buffer,
                           output_buffer,
                           alloc_u8,
                           alloc_u32,
                           alloc_hc,
                           Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"))
}

pub fn BrotliDecompressCustomIo<ErrType,
                                InputType,
                                OutputType,
                                AllocU8: Allocator<u8>,
                                AllocU32: Allocator<u32>,
                                AllocHC: Allocator<HuffmanCode>>
  (r: &mut InputType,
   mut w: &mut OutputType,
   input_buffer: &mut [u8],
   output_buffer: &mut [u8],
   alloc_u8: AllocU8,
   alloc_u32: AllocU32,
   alloc_hc: AllocHC,
   unexpected_eof_error_constant: ErrType)
   -> Result<(), ErrType>
  where InputType: CustomRead<ErrType>,
        OutputType: CustomWrite<ErrType>
{
  let mut brotli_state = BrotliState::new(alloc_u8, alloc_u32, alloc_hc);

  let mut available_out: usize = output_buffer.len();

  let mut available_in: usize = 0;
  let mut input_offset: usize = 0;
  let mut output_offset: usize = 0;
  let mut result: BrotliResult = BrotliResult::NeedsMoreInput;
  loop {
    match result {
      BrotliResult::NeedsMoreInput => {
        input_offset = 0;
        match r.read(input_buffer) {
          Err(e) => return Err(e),
          Ok(size) => {
            if size == 0 {
              return Err(unexpected_eof_error_constant);
            }
            available_in = size;
          }
        }
      }
      BrotliResult::NeedsMoreOutput => {
        let mut total_written: usize = 0;
        while total_written < output_offset {
          // this would be a call to write_all
          match w.write(&output_buffer[total_written..output_offset]) {
            Err(e) => return Result::Err(e),
            Ok(cur_written) => {
              assert_eq!(cur_written == 0, false); // not allowed by the contract
              total_written += cur_written;
            }
          }
        }

        output_offset = 0;
      }
      BrotliResult::ResultSuccess => break,
      BrotliResult::ResultFailure => panic!("FAILURE"),
    }
    let mut written: usize = 0;
    result = BrotliDecompressStream(&mut available_in,
                                    &mut input_offset,
                                    input_buffer,
                                    &mut available_out,
                                    &mut output_offset,
                                    output_buffer,
                                    &mut written,
                                    &mut brotli_state);

    if output_offset != 0 {
      let mut total_written: usize = 0;
      while total_written < output_offset {
        match w.write(&output_buffer[total_written..output_offset]) {
          Err(e) => return Result::Err(e),
          // CustomResult::Transient(e) => continue,
          Ok(cur_written) => {
            assert_eq!(cur_written == 0, false); // not allowed by the contract
            total_written += cur_written;
          }
        }
      }
      output_offset = 0;
      available_out = output_buffer.len()
    }
  }
  brotli_state.BrotliStateCleanup();
  Ok(())
}





#[cfg(not(feature="no-stdlib"))]
pub struct DecompressorCustomAlloc<R: Read,
     BufferType : SliceWrapperMut<u8>,
     AllocU8 : Allocator<u8>,
     AllocU32 : Allocator<u32>,
     AllocHC : Allocator<HuffmanCode> >(DecompressorCustomIo<io::Error,
                                                             IntoIoReader<R>,
                                                             BufferType,
                                                             AllocU8, AllocU32, AllocHC>);


#[cfg(not(feature="no-stdlib"))]
impl<R: Read,
     BufferType : SliceWrapperMut<u8>,
     AllocU8,
     AllocU32,
     AllocHC> DecompressorCustomAlloc<R, BufferType, AllocU8, AllocU32, AllocHC>
 where AllocU8 : Allocator<u8>, AllocU32 : Allocator<u32>, AllocHC : Allocator<HuffmanCode>
    {

    pub fn new(r: R, buffer : BufferType,
               alloc_u8 : AllocU8, alloc_u32 : AllocU32, alloc_hc : AllocHC) -> Self {
        DecompressorCustomAlloc::<R, BufferType, AllocU8, AllocU32, AllocHC>(
          DecompressorCustomIo::<Error,
                                 IntoIoReader<R>,
                                 BufferType,
                                 AllocU8, AllocU32, AllocHC>::new(IntoIoReader::<R>(r),
                                                                  buffer,
                                                                  alloc_u8, alloc_u32, alloc_hc,
                                                                  Error::new(ErrorKind::InvalidData,
                                                                             "Invalid Data")))
    }
}
#[cfg(not(feature="no-stdlib"))]
impl<R: Read,
     BufferType : SliceWrapperMut<u8>,
     AllocU8 : Allocator<u8>,
     AllocU32 : Allocator<u32>,
     AllocHC : Allocator<HuffmanCode> > Read for DecompressorCustomAlloc<R,
                                                                         BufferType,
                                                                         AllocU8,
                                                                         AllocU32,
                                                                         AllocHC> {
  	fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, Error> {
       self.0.read(buf)
    }
}


#[cfg(not(any(feature="unsafe", feature="no-stdlib")))]
pub struct Decompressor<R: Read>(DecompressorCustomAlloc<R,
                                                         <alloc::HeapAlloc<u8>
                                                          as Allocator<u8>>::AllocatedMemory,
                                                         HeapAlloc<u8>,
                                                         HeapAlloc<u32>,
                                                         HeapAlloc<HuffmanCode> >);


#[cfg(not(any(feature="unsafe", feature="no-stdlib")))]
impl<R: Read> Decompressor<R> {
  pub fn new(r: R, buffer_size: usize) -> Self {
    let mut alloc_u8 = HeapAlloc::<u8> { default_value: 0 };
    let buffer = alloc_u8.alloc_cell(buffer_size);
    let alloc_u32 = HeapAlloc::<u32> { default_value: 0 };
    let alloc_hc = HeapAlloc::<HuffmanCode> { default_value: HuffmanCode::default() };
    Decompressor::<R>(DecompressorCustomAlloc::<R,
                                                <alloc::HeapAlloc<u8>
                                                 as Allocator<u8>>::AllocatedMemory,
                                                HeapAlloc<u8>,
                                                HeapAlloc<u32>,
                                                HeapAlloc<HuffmanCode> >::new(r,
                                                                              buffer,
                                                                              alloc_u8,
                                                                              alloc_u32,
                                                                              alloc_hc))
  }
}


#[cfg(all(feature="unsafe", not(feature="no-stdlib")))]
pub struct Decompressor<R: Read>(DecompressorCustomAlloc<R,
                                                         <alloc::HeapAllocUninitialized<u8>
                                                          as Allocator<u8>>::AllocatedMemory,
                                                         HeapAllocUninitialized<u8>,
                                                         HeapAllocUninitialized<u32>,
                                                         HeapAllocUninitialized<HuffmanCode> >);


#[cfg(all(feature="unsafe", not(feature="no-stdlib")))]
impl<R: Read> Decompressor<R> {
  pub fn new(r: R, buffer_size: usize) -> Self {
    let mut alloc_u8 = unsafe { HeapAllocUninitialized::<u8>::new() };
    let buffer = alloc_u8.alloc_cell(buffer_size);
    let alloc_u32 = unsafe { HeapAllocUninitialized::<u32>::new() };
    let alloc_hc = unsafe { HeapAllocUninitialized::<HuffmanCode>::new() };
    Decompressor::<R>(DecompressorCustomAlloc::<R,
                                                <alloc::HeapAllocUninitialized<u8>
                                                 as Allocator<u8>>::AllocatedMemory,
                                                HeapAllocUninitialized<u8>,
                                                HeapAllocUninitialized<u32>,
                                                HeapAllocUninitialized<HuffmanCode> >
      ::new(r, buffer, alloc_u8, alloc_u32, alloc_hc))
  }
}


#[cfg(not(feature="no-stdlib"))]
impl<R: Read> Read for Decompressor<R> {
  fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, Error> {
    self.0.read(buf)
  }
}

pub struct DecompressorCustomIo<ErrType,
                                R: CustomRead<ErrType>,
                                BufferType: SliceWrapperMut<u8>,
                                AllocU8: Allocator<u8>,
                                AllocU32: Allocator<u32>,
                                AllocHC: Allocator<HuffmanCode>>
{
  input_buffer: BufferType,
  total_out: usize,
  input_offset: usize,
  input_len: usize,
  input_eof: bool,
  input: R,
  error_if_invalid_data: Option<ErrType>,
  read_error: Option<ErrType>,
  state: BrotliState<AllocU8, AllocU32, AllocHC>,
}

impl<ErrType,
     R: CustomRead<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     AllocU8,
     AllocU32,
     AllocHC> DecompressorCustomIo<ErrType, R, BufferType, AllocU8, AllocU32, AllocHC>
 where AllocU8 : Allocator<u8>, AllocU32 : Allocator<u32>, AllocHC : Allocator<HuffmanCode>
{

    pub fn new(r: R, buffer : BufferType,
               alloc_u8 : AllocU8, alloc_u32 : AllocU32, alloc_hc : AllocHC,
               invalid_data_error_type : ErrType) -> Self {
        DecompressorCustomIo::<ErrType, R, BufferType, AllocU8, AllocU32, AllocHC>{
            input_buffer : buffer,
            total_out : 0,
            input_offset : 0,
            input_len : 0,
            input_eof : false,
            input: r,
            state : BrotliState::new(alloc_u8,
                                     alloc_u32,
                                     alloc_hc),
            error_if_invalid_data : Some(invalid_data_error_type),
            read_error : None,
        }
    }

    pub fn copy_to_front(&mut self) {
        if self.input_offset == self.input_buffer.slice_mut().len() {
            self.input_offset = 0;
            self.input_len = 0;
        } else if self.input_offset + 256 > self.input_buffer.slice_mut().len() {
            let (mut first, second) = self.input_buffer.slice_mut().split_at_mut(self.input_offset);
            let avail_in = self.input_len - self.input_offset;
            first[0..avail_in].clone_from_slice(&second[0..avail_in]);
            self.input_offset = 0;
        }
    }
}
impl<ErrType,
     R: CustomRead<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     AllocU8 : Allocator<u8>,
     AllocU32 : Allocator<u32>,
     AllocHC : Allocator<HuffmanCode> > CustomRead<ErrType> for DecompressorCustomIo<ErrType,
                                                                                     R,
                                                                                     BufferType,
                                                                                     AllocU8,
                                                                                     AllocU32,
                                                                                     AllocHC> {
	fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, ErrType > {
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
        match BrotliDecompressStream(&mut avail_in,
                                     &mut self.input_offset,
                                     &self.input_buffer.slice_mut()[..],
                                     &mut avail_out,
                                     &mut output_offset,
                                     buf,
                                     &mut self.total_out,
                                     &mut self.state) {
          BrotliResult::NeedsMoreInput => {
            match self.read_error.take() {
              Some(err) => return Err(err),
              None => {
                needs_input = true;
                self.copy_to_front();
              },
            }
          },
          BrotliResult::NeedsMoreOutput => {},
          BrotliResult::ResultSuccess => break,
          BrotliResult::ResultFailure => return Err(self.error_if_invalid_data.take().unwrap()),
        }
      }
      Ok(output_offset)
    }
}

#[cfg(not(feature="no-stdlib"))]
pub fn copy_from_to<R: io::Read, W: io::Write>(mut r: R, mut w: W) -> io::Result<usize> {
  let mut buffer: [u8; 65536] = [0; 65536];
  let mut out_size: usize = 0;
  loop {
    match r.read(&mut buffer[..]) {
      Err(e) => {
        if let io::ErrorKind::Interrupted =  e.kind() {
          continue
        }
        return Err(e);
      }
      Ok(size) => {
        if size == 0 {
          break;
        } else {
          match w.write_all(&buffer[..size]) {
            Err(e) => {
              if let io::ErrorKind::Interrupted = e.kind() {
                continue
              }
              return Err(e);
            }
            Ok(_) => out_size += size,
          }
        }
      }
    }
  }
  Ok(out_size)
}
