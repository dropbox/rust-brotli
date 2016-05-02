extern crate brotli_no_stdlib as brotli;
extern crate core;

#[macro_use]
extern crate alloc_no_stdlib;

use core::ops;
use alloc_no_stdlib::{Allocator, SliceWrapperMut, SliceWrapper,
            StackAllocator, AllocatedStackMemory};

//use alloc::{SliceWrapper,SliceWrapperMut, StackAllocator, AllocatedStackMemory, Allocator};
use brotli::{BrotliDecompressStream, BrotliState, BrotliResult, FILE_BUFFER_SIZE, HuffmanCode};
use std::io::{self, Read, Write, ErrorKind, Error};
extern {
  fn calloc(n_elem : usize, el_size : usize) -> *mut u8;
}


declare_stack_allocator_struct!(MemPool, 4096, calloc);


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

//trace_macros!(true);

define_allocator_memory_pool!(global_buffer, 16, u8, [0; 1024 * 1024 * 100], global);

fn decompress<InputType, OutputType> (r : &mut InputType, mut w : &mut OutputType) -> Result<(), io::Error>
where InputType: Read, OutputType: Write {
  let mut available_out : usize = FILE_BUFFER_SIZE;
  define_allocator_memory_pool!(calloc_u8_buffer, 4096, u8, [0; 32 * 1024 * 1024], calloc);
  define_allocator_memory_pool!(calloc_u32_buffer, 4096, u32, [0; 1024 * 1024], calloc);
  define_allocator_memory_pool!(calloc_hc_buffer, 4096, HuffmanCode, [0; 4 * 1024 * 1024], calloc);
  let calloc_u8_allocator = MemPool::<u8>::new_allocator(&mut calloc_u8_buffer);
  let calloc_u32_allocator = MemPool::<u32>::new_allocator(&mut calloc_u32_buffer);
  let calloc_hc_allocator = MemPool::<HuffmanCode>::new_allocator(&mut calloc_hc_buffer);
  //test(calloc_u8_allocator);
  let mut input = [0u8;FILE_BUFFER_SIZE];
  let mut output = [0u8;FILE_BUFFER_SIZE];
  let mut brotli_state = BrotliState::new(calloc_u8_allocator, calloc_u32_allocator, calloc_hc_allocator);

  //let amount = try!(r.read(&mut buf));
  let mut available_in : usize = 0;
  let mut input_offset : usize = 0;
  let mut output_offset : usize = 0;
  let mut result : BrotliResult = BrotliResult::NeedsMoreInput;
  loop {
      match result {
          BrotliResult::NeedsMoreInput => {
              match r.read(&mut input[..]) {
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
              try!(_write_all(&mut w, &output[..output_offset]));
              output_offset = 0;
          },
          BrotliResult::ResultSuccess => break,
          BrotliResult::ResultFailure => panic!("FAILURE"),
      }
      let mut written :usize = 0;
      result = BrotliDecompressStream(&mut available_in, &mut input_offset, &input[..],
                                      &mut available_out, &mut output_offset, &mut output,
                                      &mut written, &mut brotli_state);

  }
  if output_offset != 0 {
      try!(_write_all(&mut w, &output[..output_offset]));
  }
  brotli_state.BrotliStateCleanup();
  Ok(())
}

fn main() {
    match decompress(&mut io::stdin(), &mut io::stdout()) {
        Ok(_) => return,
        Err(e) => panic!("Error {:?}", e),
    }
}
