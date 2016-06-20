# rust-brotli

[![crates.io](http://meritbadge.herokuapp.com/brotli)](https://crates.io/crates/brotli)
[![Build Status](https://travis-ci.org/dropbox/rust-brotli.svg?branch=master)](https://travis-ci.org/dropbox/rust-brotli)
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
### With the io::Read abstraction
```
let mut input = brotli::Decompressor::new(&mut io::stdin(), 4096 /* buffer size */);
```
then you can simply read input as you would any other io::Read class

### With the Stream Copy abstraction
```
  match brotli::BrotliDecompress(&mut io::stdin(), &mut io::stdout(), 65536 /* buffer size */) {
      Ok(_) => {},
      Err(e) => panic!("Error {:?}", e),
  }
```
### With manual memory management
There are 3 steps to using brotli without stdlib
a) setup the memory manager
b) setup the BrotliState
c) in a loop, call BrotliDecompressStream

in Detail

```
// at global scope declare a MemPool type -- in this case we'll choose the heap to
// avoid unsafe code, and avoid restrictions of the stack size

declare_stack_allocator_struct!(MemPool, heap);

  // at local scope, make a heap allocated buffers to hold uint8's uint32's and huffman codes
  let mut u8_buffer = define_allocator_memory_pool!(4096, u8, [0; 32 * 1024 * 1024], heap);
  let mut u32_buffer = define_allocator_memory_pool!(4096, u32, [0; 1024 * 1024], heap);
  let mut hc_buffer = define_allocator_memory_pool!(4096, HuffmanCode, [0; 4 * 1024 * 1024], heap);
  let heap_u8_allocator = HeapPrealloc::<u8>::new_allocator(4096, &mut u8_buffer, bzero);
  let heap_u32_allocator = HeapPrealloc::<u32>::new_allocator(4096, &mut u32_buffer, bzero);
  let heap_hc_allocator = HeapPrealloc::<HuffmanCode>::new_allocator(4096, &mut hc_buffer, bzero);

  // At this point no more syscalls are going to be needed since everything can come from the allocators
  // feel free to activate SECCOMP jailing or other mechanisms to secure your application if you wish


  // now it's possible to setup the decompressor state
  let mut brotli_state = BrotliState::new(heap_u8_allocator, heap_u32_allocator, heap_hc_allocator);


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

