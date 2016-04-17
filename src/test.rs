#[cfg(test)]

extern crate alloc_no_stdlib as alloc;
use alloc::{Allocator, SliceWrapperMut, SliceWrapper,
            StackAllocator, AllocatedStackMemory};

use core::ops;

#[allow(unused_imports)] // this is actually used, not sure why warn
use core::cell;


pub use super::{BrotliDecompressStream, BrotliState, BrotliResult, HuffmanCode};

#[allow(dead_code)]
declare_stack_allocator_struct!(MemPool, 4096, stack);

#[test]
fn test_10x10y() {
  const BUFFER_SIZE : usize = 16384;
  let mut available_out : usize = BUFFER_SIZE;
  define_allocator_memory_pool!(stack_u8_buffer, 4096, u8, [0; 64 * 1024], stack);
  define_allocator_memory_pool!(stack_u32_buffer, 4096, u32, [0; 64 * 1024], stack);
  define_allocator_memory_pool!(stack_hc_buffer, 4096, super::HuffmanCode, [HuffmanCode::default(); 64 * 1024], stack);
  let stack_u8_allocator = MemPool::<u8>::new_allocator(&mut stack_u8_buffer);
  let stack_u32_allocator = MemPool::<u32>::new_allocator(&mut stack_u32_buffer);
  let stack_hc_allocator = MemPool::<HuffmanCode>::new_allocator(&mut stack_hc_buffer);
  let mut input : [u8;12] = [0x1b, 0x13, 0x00, 0x00, 0xa4, 0xb0, 0xb2, 0xea, 0x81, 0x47, 0x02, 0x8a];
  let mut available_in : usize = input.len();
  let mut input_offset : usize = 0;
  let mut output_offset : usize = 0;
  let mut written : usize = 0;
  let mut scratch = [0u8;8];
  let input_cell = cell::RefCell::new(&mut input[..]);
  let scratch_cell = cell::RefCell::new(&mut scratch[..]);
  let mut output = [0u8;BUFFER_SIZE];
  let mut brotli_state = BrotliState::new(stack_u8_allocator, stack_u32_allocator, stack_hc_allocator);
  let result = BrotliDecompressStream(&mut available_in, &mut input_offset, &input_cell,
                                      &mut available_out, &mut output_offset, &mut output,
                                      &mut written, &mut brotli_state, &scratch_cell);
  match result {
     BrotliResult::ResultSuccess => {},
     _ => assert!(false),
  }
  brotli_state.BrotliStateCleanup();
  let mut i : usize = 0;
  while i < 10 {
     assert_eq!(output[i], 'X' as u8);
     assert_eq!(output[i + 10], 'Y' as u8);
     i += 1;
  }
  assert_eq!(output_offset, 20);
  assert_eq!(input_offset, input_cell.borrow().len());
}

