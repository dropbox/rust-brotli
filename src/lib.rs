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
#[macro_use]
mod memory;
mod dictionary;
#[macro_use]
mod bit_reader;
mod huffman;
mod state;
mod prefix;
mod context;
mod transform;
mod test;
mod decode;
pub use huffman::{HuffmanCode, HuffmanTreeGroup};
pub use state::BrotliState;


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


/// this trait does not allow for transient errors: they must be retried in the underlying layer
pub trait CustomWrite<ErrType> {
  fn write(self: &mut Self, data: &[u8]) -> Result<usize, ErrType>;
}
/// this trait does not allow for transient errors: they must be retried in the underlying layer
pub trait CustomRead<ErrType> {
  fn read(self: &mut Self, data: &mut [u8]) -> Result<usize, ErrType>;
}


fn _write_all<ErrType, OutputType>(w: &mut OutputType, buf: &[u8]) -> Result<(), ErrType>
  where OutputType: CustomWrite<ErrType>
{
  let mut total_written: usize = 0;
  while total_written < buf.len() {
    match w.write(&buf[total_written..]) {
      Err(e) => return Result::Err(e),
      // CustomResult::Transient(e) => continue,
      Ok(cur_written) => {
        assert_eq!(cur_written == 0, false); // not allowed by the contract
        total_written += cur_written;
      }
    }
  }
  Ok(())
}

#[cfg(not(feature="no-stdlib"))]
struct IoWriterWrapper<'a, OutputType: Write + 'a> {
  pub writer: &'a mut OutputType,
}
#[cfg(not(feature="no-stdlib"))]
impl<'a, OutputType: Write> CustomWrite<io::Error> for IoWriterWrapper<'a, OutputType> {
  fn write(self: &mut Self, buf: &[u8]) -> Result<usize, io::Error> {
    loop {
      match self.writer.write(buf) {
        Err(e) => {
          match e.kind() {
            ErrorKind::Interrupted => continue,
            _ => return Err(e),
          }
        }
        Ok(cur_written) => return Ok(cur_written),
      }
    }
  }
}


#[cfg(not(feature="no-stdlib"))]
struct IoReaderWrapper<'a, OutputType: Read + 'a> {
  pub reader: &'a mut OutputType,
}
#[cfg(not(feature="no-stdlib"))]
impl<'a, InputType: Read> CustomRead<io::Error> for IoReaderWrapper<'a, InputType> {
  fn read(self: &mut Self, buf: &mut [u8]) -> Result<usize, io::Error> {
    loop {
      match self.reader.read(buf) {
        Err(e) => {
          match e.kind() {
            ErrorKind::Interrupted => continue,
            _ => return Err(e),
          }
        }
        Ok(cur_read) => return Ok(cur_read),
      }
    }
  }
}


#[cfg(not(feature="no-stdlib"))]
pub fn BrotliDecompressFromTo<InputType, OutputType>(r: &mut InputType,
                                                     w: &mut OutputType)
                                                     -> Result<(), io::Error>
  where InputType: Read,
        OutputType: Write
{
  let mut input_buffer: [u8; 4096] = [0; 4096];
  let mut output_buffer: [u8; 4096] = [0; 4096];
  return BrotliDecompressCustomAlloc(r,
                                     w,
                                     &mut input_buffer[..],
                                     &mut output_buffer[..],
                                     HeapAlloc::<u8> { default_value: 0 },
                                     HeapAlloc::<u32> { default_value: 0 },
                                     HeapAlloc::<HuffmanCode> {
                                       default_value: HuffmanCode::default(),
                                     });
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
  return BrotliDecompressCustomIo(&mut IoReaderWrapper::<InputType> { reader: r },
                                  &mut IoWriterWrapper::<OutputType> { writer: w },
                                  input_buffer,
                                  output_buffer,
                                  alloc_u8,
                                  alloc_u32,
                                  alloc_hc,
                                  Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"));
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
  // let mut input = brotli_state.alloc_u8.alloc_cell(input_buffer_lim);
  // let mut output = brotli_state.alloc_u8.alloc_cell(output_buffer_lim);
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
          // Transient(e) => continue,
          Ok(size) => {
            if size == 0 {
              return Err(unexpected_eof_error_constant);
            }
            available_in = size;
          }
        }
      }
      BrotliResult::NeedsMoreOutput => {
        // try!(_write_all(&mut w, &output_buffer[..output_offset]));
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
