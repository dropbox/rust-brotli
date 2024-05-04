#![no_std]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![cfg_attr(feature = "benchmark", feature(test))]
#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(
    feature = "no-stdlib-ffi-binding",
    cfg_attr(not(feature = "std"), feature(lang_items))
)]
#[macro_use]
// <-- for debugging, remove xprintln from bit_reader and replace with println
#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
extern crate alloc_stdlib;
#[allow(unused_imports)]
#[macro_use]
extern crate alloc_no_stdlib as alloc;
extern crate brotli_decompressor;
pub use alloc::{AllocatedStackMemory, Allocator, SliceWrapper, SliceWrapperMut, StackAllocator};
pub const VERSION: u8 = 1;
#[cfg(feature = "std")]
pub use alloc_stdlib::HeapAlloc;
pub mod enc;
pub use self::enc::combined_alloc::CombiningAllocator;
pub mod concat;
pub use brotli_decompressor::dictionary;
pub use brotli_decompressor::reader;
#[cfg(feature = "std")]
pub use brotli_decompressor::reader::Decompressor;
pub use brotli_decompressor::reader::DecompressorCustomIo;
pub use brotli_decompressor::transform;
pub use brotli_decompressor::transform::TransformDictionaryWord;
pub use brotli_decompressor::writer;
pub use brotli_decompressor::BrotliState;
pub use brotli_decompressor::HuffmanCode; // so we can make custom allocator for decompression

pub use brotli_decompressor::writer::DecompressorWriterCustomIo;

#[cfg(feature = "std")]
pub use brotli_decompressor::writer::DecompressorWriter;

pub use brotli_decompressor::io_wrappers::{CustomRead, CustomWrite};

#[cfg(feature = "std")]
pub use brotli_decompressor::io_wrappers::{IntoIoReader, IoReaderWrapper, IoWriterWrapper};
pub use enc::input_pair::InputPair;
pub use enc::input_pair::InputReference;
pub use enc::input_pair::InputReferenceMut;
pub use enc::interface;
pub use enc::interface::SliceOffset;
#[cfg(feature = "ffi-api")]
pub mod ffi;
// interface
// pub fn BrotliDecompressStream(mut available_in: &mut usize,
//                               input_offset: &mut usize,
//                               input: &[u8],
//                               mut available_out: &mut usize,
//                               mut output_offset: &mut usize,
//                               mut output: &mut [u8],
//                               mut total_out: &mut usize,
//                               mut s: &mut BrotliState<AllocU8, AllocU32, AllocHC>);

pub use brotli_decompressor::{BrotliDecompressStream, BrotliResult};
#[cfg(feature = "std")]
pub use enc::{BrotliCompress, BrotliCompressCustomAlloc};
pub use enc::{BrotliCompressCustomIo, BrotliCompressCustomIoCustomDict};

#[cfg(feature = "std")]
pub use enc::reader::CompressorReader;
pub use enc::reader::CompressorReaderCustomIo;

#[cfg(feature = "std")]
pub use enc::writer::CompressorWriter;
pub use enc::writer::CompressorWriterCustomIo;

#[cfg(feature = "std")]
pub use brotli_decompressor::BrotliDecompress;

#[cfg(feature = "std")]
pub use brotli_decompressor::BrotliDecompressCustomAlloc;

pub use brotli_decompressor::BrotliDecompressCustomIo;
pub use brotli_decompressor::BrotliDecompressCustomIoCustomDict;

#[cfg(feature = "std")]
pub use brotli_decompressor::copy_from_to;
