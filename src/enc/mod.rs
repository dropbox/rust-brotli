#![cfg_attr(not(feature="std"), allow(unused_imports))]
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
pub mod backward_references;
pub use self::backward_references::hash_to_binary_tree;
pub use self::backward_references::hq as backward_references_hq;
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
pub mod threading;
pub mod singlethreading;
pub mod multithreading;
pub mod fixed_queue;
pub mod worker_pool;
#[cfg(feature="simd")]
use packed_simd_2::{i16x16, f32x8, i32x8};
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
pub use self::backward_references::{BrotliEncoderParams, UnionHasher};
pub use self::encode::{
    BrotliEncoderInitParams,
    BrotliEncoderSetParameter,
    BrotliEncoderMaxCompressedSizeMulti,
    BrotliEncoderMaxCompressedSize,
};
use self::encode::{BrotliEncoderCreateInstance, BrotliEncoderDestroyInstance,
                   BrotliEncoderOperation,
                   BrotliEncoderSetCustomDictionary,
                   BrotliEncoderCompressStream, BrotliEncoderIsFinished};
pub use self::interface::StaticCommand;
use brotli_decompressor::{CustomRead, CustomWrite};
pub use self::vectorization::{v256,v256i, Mem256f};
pub use interface::{InputReference,InputPair, InputReferenceMut};


#[cfg(feature="std")]
use std::io::{Read,Write, Error, ErrorKind};
#[cfg(feature="std")]
use std::io;
#[cfg(feature="std")]
pub use alloc_stdlib::StandardAlloc;
pub use alloc::{AllocatedStackMemory, Allocator, SliceWrapper, SliceWrapperMut, StackAllocator};

#[cfg(feature="std")]
pub use brotli_decompressor::{IntoIoReader, IoReaderWrapper, IoWriterWrapper};

pub use self::threading::{SendAlloc,
                          Owned,
                          BrotliEncoderThreadError,
                          BatchSpawnableLite,
                          CompressionThreadResult,
};
#[cfg(feature="std")]
pub use self::worker_pool::{
  compress_worker_pool,
  new_work_pool,
  WorkerPool,
};
#[cfg(not(feature="std"))]
pub use self::singlethreading::{
  compress_worker_pool,
  new_work_pool,
  WorkerPool,
};
#[cfg(feature="std")]
pub fn compress_multi<Alloc:BrotliAlloc+Send+'static,
                      SliceW: SliceWrapper<u8>+Send+'static+Sync> (
  params:&BrotliEncoderParams,
  owned_input: &mut Owned<SliceW>,
  output: &mut [u8],
  alloc_per_thread:&mut [SendAlloc<CompressionThreadResult<Alloc>,
                                   backward_references::UnionHasher<Alloc>,
                                   Alloc,
                                   <WorkerPool<CompressionThreadResult<Alloc>, backward_references::UnionHasher<Alloc>, Alloc, (SliceW, BrotliEncoderParams)> as BatchSpawnableLite<CompressionThreadResult<Alloc>, backward_references::UnionHasher<Alloc>, Alloc, (SliceW, BrotliEncoderParams)>>::JoinHandle>],
) -> Result<usize, BrotliEncoderThreadError>
  where <Alloc as Allocator<u8>>::AllocatedMemory: Send,
        <Alloc as Allocator<u16>>::AllocatedMemory: Send+Sync,
        <Alloc as Allocator<u32>>::AllocatedMemory: Send+Sync {
  let mut work_pool = self::worker_pool::new_work_pool(alloc_per_thread.len() - 1);
  compress_worker_pool(params, owned_input,output, alloc_per_thread,&mut work_pool)
}

#[cfg(not(feature="std"))]
pub use self::singlethreading::compress_multi;
#[cfg(not(feature="std"))]
pub use self::singlethreading::compress_multi as compress_multi_no_threadpool;
#[cfg(feature="std")]
pub use self::multithreading::compress_multi as compress_multi_no_threadpool;


#[cfg(feature="std")]
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
                            StandardAlloc::default(),
                           ) 
}

#[cfg(feature="std")]
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
  BrotliCompressCustomIoCustomDict(r,w,input_buffer,output_buffer, params, alloc, metablock_callback, &[], unexpected_eof_error_constant)
}
pub fn BrotliCompressCustomIoCustomDict<ErrType,
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
   dict: &[u8],
   unexpected_eof_error_constant: ErrType)
   -> Result<usize, ErrType>
  where InputType: CustomRead<ErrType>,
        OutputType: CustomWrite<ErrType>
{
  assert!(input_buffer.len() != 0);
  assert!(output_buffer.len() != 0);
  let mut s_orig = BrotliEncoderCreateInstance(alloc);
  s_orig.params = params.clone();
  if dict.len() != 0 {
    BrotliEncoderSetCustomDictionary(&mut s_orig, dict.len(), dict);
  }
  let mut next_in_offset: usize = 0;  
  let mut next_out_offset: usize = 0;
  let mut total_out = Some(0usize);
  let mut read_err: Result<(), ErrType> = Ok(());
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
                  Err(e) => {
                      read_err = Err(e);
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
                      Err(e) => {
                          BrotliEncoderDestroyInstance(s);
                          if let Err(err) = read_err {
                              return Err(err);
                          }
                          return Err(e);
                      }
                      Ok(size) => {
                          next_out_offset += size;
                      }
                  }
              }
              available_out = output_buffer.len();
              next_out_offset = 0;
          }
          if result <= 0 {
              if let Ok(_) = read_err {
                  read_err = Err(unexpected_eof_error_constant);
              }
              break;
          }
          if fin != 0 {
              break;
          }
      }
      BrotliEncoderDestroyInstance(s);
  }
  if let Err(err) = read_err {
    return Err(err)
  }
  Ok(total_out.unwrap())
}

