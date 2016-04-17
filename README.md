# rust-brotli

    Direct no-stdlib port of the C brotli decompressor to Rust
    
    no dependency on the Rust stdlib: this library would be ideal for decompressing within a rust kernel among other things.
    
    This will be useful to see how C and Rust compare in an apples-to-apples
    comparison where the same algorithms and data structures and
    optimizations are employed.
    
    The current expected performance losses come from
    a) an extra indirection in the hgroups
    b) array bounds checks on every access
    c) no ability to load a full aligned 64 bit or 128 bit item from a [u8]
