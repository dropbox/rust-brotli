#![cfg_attr(feature="no-stdlib", allow(unused_imports))]
#[macro_use]
pub mod vectorization;
pub mod input_pair;
pub mod fast_log;
pub mod command;
pub mod block_split;
pub mod brotli_bit_stream;
pub mod constants;
pub mod entropy_encode;
pub mod static_dict;
pub mod combined_alloc;
pub mod static_dict_lut;
pub mod dictionary_hash;
pub mod util;
pub mod utf8_util;
pub mod bit_cost;
pub mod cluster;
pub mod literal_cost;
pub mod histogram;
pub mod hash_to_binary_tree;
pub mod backward_references;
pub mod backward_references_hq;
pub mod block_splitter;
pub mod metablock;
pub mod compress_fragment_two_pass;
pub mod compress_fragment;
pub mod encode;
pub mod reader;
pub mod writer;
pub mod find_stride;
pub mod interface;
pub mod ir_interpret;
pub mod prior_eval;
pub mod stride_eval;
pub mod context_map_entropy;
pub mod pdf;
pub use self::combined_alloc::{CombiningAllocator, BrotliAlloc};
mod compat;
#[cfg(feature="simd")]
use packed_simd::{i16x16, f32x8, i32x8};
#[cfg(feature="simd")]
pub type s16 = i16x16;
#[cfg(feature="simd")]
pub type v8 = f32x8;
#[cfg(feature="simd")]
pub type s8 = i32x8;
#[cfg(not(feature="simd"))]
pub type s16 = compat::Compat16x16;
#[cfg(not(feature="simd"))]
pub type v8 = compat::CompatF8;
#[cfg(not(feature="simd"))]
pub type s8 = compat::Compat32x8;

mod test;
mod weights;
mod parameters;
pub use self::util::floatX;
pub use self::pdf::PDF;
pub use self::hash_to_binary_tree::ZopfliNode;
pub use self::backward_references::BrotliEncoderParams;
pub use self::encode::{BrotliEncoderInitParams, BrotliEncoderSetParameter};
use self::encode::{BrotliEncoderCreateInstance, BrotliEncoderDestroyInstance,
                   BrotliEncoderOperation,
                   BrotliEncoderCompressStream, BrotliEncoderIsFinished};
use self::cluster::{HistogramPair};
pub use self::interface::StaticCommand;
use self::histogram::{ContextType, HistogramLiteral, HistogramCommand, HistogramDistance};
use self::command::{Command};
use self::entropy_encode::{HuffmanTree};
use brotli_decompressor::{CustomRead, CustomWrite};
pub use self::vectorization::{v256,v256i, Mem256f};
pub use interface::{InputReference,InputPair, InputReferenceMut};


#[cfg(not(feature="no-stdlib"))]
use std::io::{Read,Write, Error, ErrorKind};
#[cfg(not(feature="no-stdlib"))]
use std::io;
#[cfg(not(feature="no-stdlib"))]
pub use alloc::HeapAlloc;
pub use alloc::{AllocatedStackMemory, Allocator, SliceWrapper, SliceWrapperMut, StackAllocator};

#[cfg(not(feature="no-stdlib"))]
pub use brotli_decompressor::{IntoIoReader, IoReaderWrapper, IoWriterWrapper};



#[cfg(not(any(feature="no-stdlib")))]
pub fn BrotliCompress<InputType, OutputType>(r: &mut InputType,
                                             w: &mut OutputType,
                                             params: &BrotliEncoderParams)
                                               -> Result<usize, io::Error>
  where InputType: Read,
        OutputType: Write
{
  let mut input_buffer: [u8; 4096] = [0; 4096];
  let mut output_buffer: [u8; 4096] = [0; 4096];
  BrotliCompressCustomAlloc(r,
                            w,
                            &mut input_buffer[..],
                            &mut output_buffer[..],
                            params,
                            CombiningAllocator::new(
                                HeapAlloc::<u8> { default_value: 0 },
                                HeapAlloc::<u16> { default_value: 0 },
                                HeapAlloc::<i32> { default_value: 0 },
                                HeapAlloc::<u32> { default_value: 0 },
                                HeapAlloc::<u64> { default_value: 0 },
                                HeapAlloc::<Command> {
                                    default_value: Command::default(),
                                },
                                HeapAlloc::<floatX> { default_value: 0.0 as floatX },
                                HeapAlloc::<v8> { default_value: v8::default() },
                                HeapAlloc::<s16> { default_value: s16::default() },
                                HeapAlloc::<PDF> { default_value: PDF::default() },
                                HeapAlloc::<StaticCommand> { default_value: StaticCommand::default() },
                                HeapAlloc::<HistogramLiteral>{
                                    default_value: HistogramLiteral::default(),
                                },
                                HeapAlloc::<HistogramCommand>{
                                    default_value: HistogramCommand::default(),
                                },
                                HeapAlloc::<HistogramDistance>{
                                    default_value: HistogramDistance::default(),
                                },
                                HeapAlloc::<HistogramPair>{
                                    default_value: HistogramPair::default(),
                                },
                                HeapAlloc::<ContextType>{
                                    default_value: ContextType::default(),
                                },
                                HeapAlloc::<HuffmanTree>{
                                    default_value: HuffmanTree::default(),
                                },
                                HeapAlloc::<ZopfliNode>{
                                    default_value: ZopfliNode::default(),
                                },
                            ))
}

#[cfg(not(feature="no-stdlib"))]
pub fn BrotliCompressCustomAlloc<InputType,
                                 OutputType,
                                 Alloc: BrotliAlloc>
  (r: &mut InputType,
   w: &mut OutputType,
   input_buffer: &mut [u8],
   output_buffer: &mut [u8],
   params: &BrotliEncoderParams,
   alloc: Alloc)
   -> Result<usize, io::Error>
  where InputType: Read,
        OutputType: Write
{
  let mut nop_callback = |_data:&mut interface::PredictionModeContextMap<InputReferenceMut>,
                          _cmds: &mut [interface::StaticCommand],
                          _mb: interface::InputPair, _m: &mut Alloc|();
  BrotliCompressCustomIo(&mut IoReaderWrapper::<InputType>(r),
                           &mut IoWriterWrapper::<OutputType>(w),
                           input_buffer,
                           output_buffer,
                           params,
                           alloc,
                           &mut nop_callback,
                           Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"))
}

pub fn BrotliCompressCustomIo<ErrType,
                              InputType,
                              OutputType,
                              Alloc: BrotliAlloc,
                              MetablockCallback: FnMut(&mut interface::PredictionModeContextMap<InputReferenceMut>,
                                                       &mut [interface::StaticCommand],
                                                       interface::InputPair, &mut Alloc)>
  (r: &mut InputType,
   w: &mut OutputType,
   input_buffer: &mut [u8],
   output_buffer: &mut [u8],
   params: &BrotliEncoderParams,
   alloc: Alloc,
   metablock_callback: &mut MetablockCallback,
   unexpected_eof_error_constant: ErrType)
   -> Result<usize, ErrType>
  where InputType: CustomRead<ErrType>,
        OutputType: CustomWrite<ErrType>
{
  assert!(input_buffer.len() != 0);
  assert!(output_buffer.len() != 0);
  let mut s_orig = BrotliEncoderCreateInstance(alloc);
  s_orig.params = params.clone();
  let mut next_in_offset: usize = 0;  
  let mut next_out_offset: usize = 0;
  let mut total_out = Some(0usize);
  {
      let s = &mut s_orig;
      
      //BrotliEncoderSetParameter(s, BrotliEncoderParameter::BROTLI_PARAM_MODE, 0 as (u32)); // gen, text, font
      //BrotliEncoderSetParameter(s,
      //                          BrotliEncoderParameter::BROTLI_PARAM_SIZE_HINT,
      //                          input.len() as (u32));
      let mut available_in: usize = 0;
      let mut available_out: usize = output_buffer.len();
      let mut eof = false;
      loop {
          if available_in == 0 && !eof {
              next_in_offset = 0;
              match r.read(input_buffer) {
                  Err(_) => {
                      available_in = 0;
                      eof = true;
                  },
                  Ok(size) => {
                      if size == 0 {
                          eof = true;
                      }
                      available_in = size;
                  }
              }
          }
          let op : BrotliEncoderOperation;
          if available_in == 0 {
              op = BrotliEncoderOperation::BROTLI_OPERATION_FINISH;
          } else {
              op = BrotliEncoderOperation::BROTLI_OPERATION_PROCESS;
          }
          let result = BrotliEncoderCompressStream(s,
                                                   op,
                                                   &mut available_in,
                                                   input_buffer,
                                                   &mut next_in_offset,  
                                                   &mut available_out,
                                                   output_buffer,
                                                   &mut next_out_offset,
                                                   &mut total_out,
                                                   metablock_callback);
          let fin = BrotliEncoderIsFinished(s);
          if available_out == 0 || fin != 0 {
              let lim = output_buffer.len() - available_out;
              assert_eq!(next_out_offset, lim);
              next_out_offset = 0;
              while next_out_offset < lim {
                  match w.write(&mut output_buffer[next_out_offset..lim]) {
                      Err(e) => return Err(e),
                      Ok(size) => {
                          next_out_offset += size;
                      }
                  }
              }
              available_out = output_buffer.len();
              next_out_offset = 0;
          }
          if result <= 0 {
              return Err(unexpected_eof_error_constant);
          }
          if fin != 0 {
              break;
          }
      }
      BrotliEncoderDestroyInstance(s);
  }
  Ok(total_out.unwrap())
}

