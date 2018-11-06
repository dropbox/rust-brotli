# rust-brotli

[![crates.io](http://meritbadge.herokuapp.com/brotli)](https://crates.io/crates/brotli)
[![Build Status](https://travis-ci.org/dropbox/rust-brotli.svg?branch=master)](https://travis-ci.org/dropbox/rust-brotli)

* A fully compatible FFI for drop-in compatibiltiy with the https://github.com/google/brotli binaries
  * custom allocators fully supported
* Multithreaded compression so multiple threads can operate in unison on a single file
* Concatenatability mode to add the feature requested in https://github.com/google/brotli/issues/628
  * binary tool catbrotli can accomplish this if the first file was specified with -apendable and the second with -catable
* validation mode where a file is double-checked to be able to be decompressed with the same settings; useful for benchmarking or fuzzing
* Magic Number: where the brotli file can have a useful header with a few magic bytes, concatability info and a final output size for pre-allocating memory

## Project Requirements

Direct no-stdlib port of the C brotli compressor to Rust

no dependency on the Rust stdlib: this library would be ideal for decompressing within a rust kernel among other things.

This is useful to see how C and Rust compare in an apples-to-apples
comparison where the same algorithms and data structures and
optimizations are employed.

## Using the C interface

rust-brotli is a drop-in replacement for the official http://github.com/google/brotli/ C
implementation. That means you can use it from any place that supports that library.
To build rust-brotli in this manner enter the c subdirectory and run make there

cd c && make

this should build c/target/release/libbrotli.so and should build the vanilla
command line tool in C for compressing and decompressing any brotli file.

the libbrotli.so in c/target/release should be able to replace any other libbrotli.so
file, but with all the advantages of using safe rust (except in the FFI bindings)

The code also allows a wider range of options, including forcing the prediction mode
(eg UTF8 vs signed vs MSB vs LSB) and changing the weight of the literal cost from 540
 to other values.

Additionally the CATABLE and APPENDABLE options are exposed and allow concatenation of files
created in this manner.

Specifically CATABLE files can be concatenated in any order using the catbrotli tool
and APPENDABLE files can be the first file in a sequence of catable files...
eg you can combine
appendable.br catable1.br catable2.br catable3.br

or simply
catable0.br catable1.br catable2.br catable3.br

# Multithreaded Compression
The C FFI allows you to create a workpool which may be used to compress multiple files without recreating threads on each compression
```rust
    BrotliEncoderWorkPool *work_pool = BrotliEncoderCreateWorkPool(num_threads != 0 ? num_threads - 1 : 0, NULL /* custom allocator */, NULL, NULL);
    if (!work_pool) {
      return 0;
    }
    size_t out_len = BrotliEncoderMaxCompressedSizeMulti(len, num_threads);
    reinit_vec_u8(ret_buffer, out_len);
    ret = BrotliEncoderCompressWorkPool(
        work_pool,
        num_params,
        param_keys,
        param_values,
        len,
        data,
        &out_len,
        ret_buffer->data,
        num_threads,
        NULL /* custom allocator*/, NULL, NULL);
        
    BrotliEncoderDestroyWorkPool(work_pool);
```

An example can be seen in multiexample.c
