#![no_std]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[macro_use]
// <-- for debugging, remove xprintln from bit_reader and replace with println
#[cfg(not(feature="no-stdlib"))]
extern crate std;

#[allow(unused_imports)]
#[macro_use]
extern crate alloc_no_stdlib as alloc;
extern crate brotli_decompressor;
pub use alloc::{AllocatedStackMemory, Allocator, SliceWrapper, SliceWrapperMut, StackAllocator};

#[cfg(not(feature="no-stdlib"))]
pub use alloc::HeapAlloc;
pub mod enc;
pub use brotli_decompressor::transform;
pub use brotli_decompressor::dictionary;
pub use brotli_decompressor::reader;
pub use brotli_decompressor::writer;
pub use brotli_decompressor::BrotliState;
pub use brotli_decompressor::reader::{DecompressorCustomIo};
pub use brotli_decompressor::HuffmanCode; // so we can make custom allocator for decompression
pub use brotli_decompressor::transform::TransformDictionaryWord;
#[cfg(not(feature="no-stdlib"))]
pub use brotli_decompressor::reader::{Decompressor};

pub use brotli_decompressor::writer::{DecompressorWriterCustomIo};

#[cfg(not(feature="no-stdlib"))]
pub use brotli_decompressor::writer::{DecompressorWriter};

pub use brotli_decompressor::io_wrappers::{CustomRead, CustomWrite};

#[cfg(not(feature="no-stdlib"))]
pub use brotli_decompressor::io_wrappers::{IntoIoReader, IoReaderWrapper, IoWriterWrapper};
pub use enc::interface;
pub use enc::input_pair::InputReference;
pub use enc::input_pair::InputReferenceMut;
pub use enc::input_pair::InputPair;
pub use enc::interface::SliceOffset;
pub use enc::interface::thaw;
pub use enc::interface::thaw_pair;
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

#[cfg(not(feature="no-stdlib"))]
pub use enc::{BrotliCompress, BrotliCompressCustomAlloc};
pub use enc::{BrotliCompressCustomIo};

#[cfg(not(feature="no-stdlib"))]
pub use enc::reader::{CompressorReader};
pub use enc::reader::{CompressorReaderCustomIo};


#[cfg(not(feature="no-stdlib"))]
pub use enc::writer::{CompressorWriter};
pub use enc::writer::{CompressorWriterCustomIo};


#[cfg(not(feature="no-stdlib"))]
pub use brotli_decompressor::BrotliDecompress;



#[cfg(not(feature="no-stdlib"))]
pub use brotli_decompressor::BrotliDecompressCustomAlloc;

pub use brotli_decompressor::BrotliDecompressCustomIo;


#[cfg(not(feature="no-stdlib"))]
pub use brotli_decompressor::copy_from_to;
