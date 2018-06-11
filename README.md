# rust-brotli

[![crates.io](http://meritbadge.herokuapp.com/brotli)](https://crates.io/crates/brotli)
[![Build Status](https://travis-ci.org/dropbox/rust-brotli.svg?branch=master)](https://travis-ci.org/dropbox/rust-brotli)

## What's new in 2.4
The callback with the compression intermediate representation now passes a full metablock at a time. Also these items are mutable
in case futher optimization is desired

## What's new in 2.3

Flush now produces output instead of calling finish on the stream. This allows you to use the writer abstraction to
get immediate output without having to resort to the CompressStream internal abstraction

## Project Requirements

Direct no-stdlib port of the C brotli compressor to Rust

no dependency on the Rust stdlib: this library would be ideal for decompressing within a rust kernel among other things.

This is useful to see how C and Rust compare in an apples-to-apples
comparison where the same algorithms and data structures and
optimizations are employed.

## Compression Usage

Rust brotli currently supports compression levels 0 - 11
They should be bitwise identical to the brotli C compression engine at compression levels 0-9
Recommended lg_window_size is between 20 and 22

### With the io::Read abstraction
```rust
let mut input = brotli::CompressorReader::new(&mut io::stdin(), 4096 /* buffer size */,
                                              quality as u32, lg_window_size as u32);
```
then you can simply read input as you would any other io::Read class

### With the io::Write abstraction

```rust
let mut writer = brotli::Compressor::new(&mut io::stdout(), 4096 /* buffer size */,
                                         quality as u32, lg_window_size as u32);
```

### With the Stream Copy abstraction

```rust
match brotli::BrotliCompress(&mut io::stdin(), &mut io::stdout(), &brotli_encoder_params) {
    Ok(_) => {},
    Err(e) => panic!("Error {:?}", e),
}
```

## Decompression Usage

### With the io::Read abstraction

```rust
let mut input = brotli::Decompressor::new(&mut io::stdin(), 4096 /* buffer size */);
```
then you can simply read input as you would any other io::Read class

### With the io::Write abstraction

```rust
let mut writer = brotli::DecompressorWriter::new(&mut io::stdout(), 4096 /* buffer size */);
```

### With the Stream Copy abstraction

```rust
match brotli::BrotliDecompress(&mut io::stdin(), &mut io::stdout()) {
    Ok(_) => {},
    Err(e) => panic!("Error {:?}", e),
}
```

### With manual memory management

There are 3 steps to using brotli without stdlib

1. setup the memory manager
2. setup the BrotliState
3. in a loop, call BrotliDecompressStream

in Detail

```rust
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

// At this point no more syscalls are going to be needed since everything can come from the allocators.

// Feel free to activate SECCOMP jailing or other mechanisms to secure your application if you wish.

// Now it's possible to setup the decompressor state
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
