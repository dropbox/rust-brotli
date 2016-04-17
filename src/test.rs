#![cfg(test)]

extern crate alloc_no_stdlib as alloc;
use alloc::{Allocator, SliceWrapperMut, SliceWrapper,
            StackAllocator, AllocatedStackMemory};

use core::ops;

#[allow(unused_imports)] // this is actually used, not sure why warn
use core::cell;


pub use super::{BrotliDecompressStream, BrotliState, BrotliResult, HuffmanCode};

declare_stack_allocator_struct!(MemPool, 4096, stack);



fn oneshot(mut input : &mut [u8], mut output : &mut [u8]) -> (BrotliResult, usize, usize) {
  let mut available_out : usize = output.len();
  define_allocator_memory_pool!(stack_u8_buffer, 4096, u8, [0; 400 * 1024], stack);
  define_allocator_memory_pool!(stack_u32_buffer, 4096, u32, [0; 48 * 1024], stack);
  define_allocator_memory_pool!(stack_hc_buffer, 4096, super::HuffmanCode, [HuffmanCode::default(); 48 * 1024], stack);
  let stack_u8_allocator = MemPool::<u8>::new_allocator(&mut stack_u8_buffer);
  let stack_u32_allocator = MemPool::<u32>::new_allocator(&mut stack_u32_buffer);
  let stack_hc_allocator = MemPool::<HuffmanCode>::new_allocator(&mut stack_hc_buffer);
  let mut available_in : usize = input.len();
  let mut input_offset : usize = 0;
  let mut output_offset : usize = 0;
  let mut written : usize = 0;
  let mut scratch = [0u8;8];
  let input_cell = cell::RefCell::new(&mut input[..]);
  let scratch_cell = cell::RefCell::new(&mut scratch[..]);
  let mut brotli_state = BrotliState::new(stack_u8_allocator, stack_u32_allocator, stack_hc_allocator);
  let result = BrotliDecompressStream(&mut available_in, &mut input_offset, &input_cell,
                                      &mut available_out, &mut output_offset, &mut output,
                                      &mut written, &mut brotli_state, &scratch_cell);
  brotli_state.BrotliStateCleanup();
  return (result, input_offset, output_offset);
}

#[test]
fn test_10x10y() {
  const BUFFER_SIZE : usize = 16384;
  let mut input : [u8;12] = [0x1b, 0x13, 0x00, 0x00, 0xa4, 0xb0, 0xb2, 0xea, 0x81, 0x47, 0x02, 0x8a];
  let mut output = [0u8;BUFFER_SIZE];
  let (result, input_offset, output_offset) = oneshot(&mut input[..], &mut output[..]);
  match result {
     BrotliResult::ResultSuccess => {},
     _ => assert!(false),
  }
  let mut i : usize = 0;
  while i < 10 {
     assert_eq!(output[i], 'X' as u8);
     assert_eq!(output[i + 10], 'Y' as u8);
     i += 1;
  }
  assert_eq!(output_offset, 20);
  assert_eq!(input_offset, input.len());
}



#[test]
fn test_x() {
  const BUFFER_SIZE : usize = 16384;
  let mut input : [u8;5] = [0x0b, 0x00, 0x80, 0x58, 0x03];
  let mut output = [0u8;BUFFER_SIZE];
  let (result, input_offset, output_offset) = oneshot(&mut input[..], &mut output[..]);
  match result {
     BrotliResult::ResultSuccess => {},
     _ => assert!(false),
  }
  assert_eq!(output[0], 'X' as u8);
  assert_eq!(output_offset, 1);
  assert_eq!(input_offset, input.len());
}

#[test]
fn test_empty() {
  const BUFFER_SIZE : usize = 16384;
  let mut input : [u8;1] = [0x06];
  let mut output = [0u8;BUFFER_SIZE];
  let (result, input_offset, output_offset) = oneshot(&mut input[..], &mut output[..]);
  match result {
     BrotliResult::ResultSuccess => {},
     _ => assert!(false),
  }
  assert_eq!(output_offset, 0);
  assert_eq!(input_offset, input.len());
}

#[test]
#[should_panic] // <-- remove when we get this test operational
fn test_quickfox_repeated() {
  const BUFFER_SIZE : usize = 180 * 1024;
  let mut input : [u8;58] = [0x5B, 0xFF, 0xAF, 0x02, 0xC0, 0x22, 0x79, 0x5C, 0xFB, 0x5A, 0x8C, 0x42, 0x3B, 0xF4, 0x25, 0x55,
    0x19, 0x5A, 0x92, 0x99, 0xB1, 0x35, 0xC8, 0x19, 0x9E, 0x9E, 0x0A, 0x7B, 0x4B, 0x90, 0xB9, 0x3C,
    0x98, 0xC8, 0x09, 0x40, 0xF3, 0xE6, 0xD9, 0x4D, 0xE4, 0x6D, 0x65, 0x1B, 0x27, 0x87, 0x13, 0x5F,
    0xA6, 0xE9, 0x30, 0x96, 0x7B, 0x3C, 0x15, 0xD8, 0x53, 0x1C];

  let mut output = [0u8;BUFFER_SIZE];
  let (result, input_offset, output_offset) = oneshot(&mut input[..], &mut output[..]);
  match result {
     BrotliResult::ResultSuccess => {},
     _ => assert!(false),
  }
  assert_eq!(output_offset, 176128);
  assert_eq!(input_offset, input.len());

}
