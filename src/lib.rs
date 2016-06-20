#![no_std]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//#[macro_use] //<-- for debugging, remove xprintln from bit_reader and replace with println
//extern crate std;

#[macro_use]
extern crate alloc_no_stdlib as alloc;
pub use alloc::{Allocator, SliceWrapperMut, SliceWrapper, StackAllocator, AllocatedStackMemory};

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
pub use decode::{BrotliResult, BrotliDecompressStream};

