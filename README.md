# rust-brotli

[![crates.io](http://meritbadge.herokuapp.com/brotli-no-stdlib)](https://crates.io/crates/brotli-no-stdlib)
[![Build Status](https://travis-ci.org/dropbox/rust-brotli-no-stdlib.svg?branch=master)](https://travis-ci.org/dropbox/rust-brotli-no-stdlib)
## Project Requirements

    Direct no-stdlib port of the C brotli decompressor to Rust
    
    no dependency on the Rust stdlib: this library would be ideal for decompressing within a rust kernel among other things.
    
    This will be useful to see how C and Rust compare in an apples-to-apples
    comparison where the same algorithms and data structures and
    optimizations are employed.
    
    The current expected performance losses come from
    a) an extra indirection in the hgroups
    b) array bounds checks on every access
    c) no ability to load a full aligned 64 bit or 128 bit item from a [u8]

    the system also enables all syscalls to be "frontloaded" in the initial generation
    of a memory pool for the allocator. Afterwards, SECCOMP can be activated or
    other mechanisms can be used to secure the application, if desired


## Usage

There are 3 steps to using brotli
a) setup the memory manager
b) setup the BrotliState
c) in a loop, call BrotliDecompressStream

in Detail

```
// at global scope declare a MemPool type -- in this case we'll choose the heap to
// avoid unsafe code, and avoid restrictions of the stack size

declare_stack_allocator_struct!(MemPool, heap);

  // at local scope, make a heap allocated buffers to hold uint8's uint32's and huffman codes
  define_allocator_memory_pool!(u8_buffer, 4096, u8, [0; 32 * 1024 * 1024], heap);
  define_allocator_memory_pool!(u32_buffer, 4096, u32, [0; 1024 * 1024], heap);
  define_allocator_memory_pool!(hc_buffer, 4096, HuffmanCode, [0; 4 * 1024 * 1024], heap);
  let heap_u8_allocator = MemPool::<u8>::new_allocator(u8_buffer, bzero);
  let heap_u32_allocator = MemPool::<u32>::new_allocator(u32_buffer, bzero);
  let heap_hc_allocator = MemPool::<HuffmanCode>::new_allocator(hc_buffer, bzero);


  // At this point no more syscalls are going to be needed since everything can come from the allocators
  // feel free to activate SECCOMP jailing or other mechanisms to secure your application if you wish


  // now it's possible to setup the decompressor state
  let mut brotli_state = BrotliState::new(calloc_u8_allocator, calloc_u32_allocator, calloc_hc_allocator);


  // at this point the decompressor simply needs an input and output buffer and the ability to track
  // the available data left in each buffer
  loop {

      result = BrotliDecompressStream(&mut available_in, &mut input_offset, &input.slice(),
                                      &mut available_out, &mut output_offset, &mut output.slice_mut(),
                                      &mut written, &mut brotli_state);

      // just end the decompression if result is BrotliResult::ResultSuccess or BrotliResult::ResultFailure
  }
```

This interface is the same interface that the C brotli decompressor uses

Also feel free to use custom allocators that invoke Box directly.
This example illustrates a mechanism to avoid subsequent syscalls after the initial allocation

