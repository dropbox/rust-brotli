#![no_std]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[macro_use] //<-- for debugging, remove xprintln from bit_reader and replace with println
#[cfg(not(feature="no-stdlib"))]
extern crate std;
#[cfg(not(feature="no-stdlib"))]
use std::io::{self, Read, Write, ErrorKind, Error};

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

#[cfg(not(feature="no-stdlib"))]
fn _write_all<OutputType> (w : &mut OutputType, buf : &[u8]) -> Result<(), io::Error>
where OutputType: Write {
    let mut total_written : usize = 0;
    while total_written < buf.len() {
        match w.write(&buf[total_written..]) {
            Err(e) => {
                match e.kind() {
                    ErrorKind::Interrupted => continue,
                    _ => return Err(e),
                }
            },
            Ok(cur_written) => {
                if cur_written == 0 {
                     return Err(Error::new(ErrorKind::UnexpectedEof, "Write EOF"));
                }
                total_written += cur_written;
            }
        }
    }
    Ok(())
}

#[cfg(not(feature="no-stdlib"))]
pub fn BrotliDecompressFromTo<InputType, OutputType> (r : &mut InputType,
                                                      w : &mut OutputType) -> Result<(), io::Error>
where InputType: Read, OutputType: Write {
    let mut input_buffer : [u8;4096] = [0; 4096];
    let mut output_buffer : [u8;4096] = [0; 4096];
    return BrotliDecompressOpt(r, w,
                               &mut input_buffer[..],
                               &mut output_buffer[..],
                               HeapAlloc::<u8>{default_value:0},
                               HeapAlloc::<u32>{default_value:0},
                               HeapAlloc::<HuffmanCode>{default_value:HuffmanCode::default()});
}

#[cfg(not(feature="no-stdlib"))]
fn BrotliDecompressOpt<InputType : Read, OutputType : Write, AllocU8 : Allocator<u8>, AllocU32 : Allocator<u32>, AllocHC : Allocator<HuffmanCode> > (r : &mut InputType,
                                               mut w : &mut OutputType,
                                               input_buffer: &mut[u8],
                                               output_buffer: &mut[u8],
                                               alloc_u8 : AllocU8,
                                               alloc_u32 : AllocU32,
                                               alloc_hc : AllocHC) -> Result<(), io::Error> {
  let mut brotli_state = BrotliState::new(alloc_u8,
                                          alloc_u32,
                                          alloc_hc);
  //let mut input = brotli_state.alloc_u8.alloc_cell(input_buffer_lim);
  //let mut output = brotli_state.alloc_u8.alloc_cell(output_buffer_lim);
  let mut available_out : usize = output_buffer.len();

  let mut available_in : usize = 0;
  let mut input_offset : usize = 0;
  let mut output_offset : usize = 0;
  let mut result : BrotliResult = BrotliResult::NeedsMoreInput;
  loop {
      match result {
          BrotliResult::NeedsMoreInput => {
              input_offset = 0;
              match r.read(input_buffer) {
                  Err(e) => {
                      match e.kind() {
                          ErrorKind::Interrupted => continue,
                          _ => return Err(e),
                      }
                  },
                  Ok(size) => {
                      if size == 0 {
                          return Err(Error::new(ErrorKind::UnexpectedEof, "Read EOF"));
                      }
                      available_in = size;
                  },
              }
          },
          BrotliResult::NeedsMoreOutput => {
              try!(_write_all(&mut w, &output_buffer[..output_offset]));
              output_offset = 0;
          },
          BrotliResult::ResultSuccess => break,
          BrotliResult::ResultFailure => panic!("FAILURE"),
      }
      let mut written :usize = 0;
      result = BrotliDecompressStream(&mut available_in, &mut input_offset, input_buffer,
                                      &mut available_out, &mut output_offset, output_buffer,
                                      &mut written, &mut brotli_state);

      if output_offset != 0 {
          try!(_write_all(&mut w, &output_buffer[..output_offset]));
          output_offset = 0;
          available_out = output_buffer.len()
      }
  }
  brotli_state.BrotliStateCleanup();
  Ok(())
}

