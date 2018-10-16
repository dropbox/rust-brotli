#![cfg(not(feature="safe"))]

#[no_mangle]
use core;
use core::slice;
use brotli_decompressor::ffi::alloc_util::SubclassableAllocator;
use brotli_decompressor::ffi::interface::{brotli_alloc_func, brotli_free_func, CAllocator, c_void};
use super::alloc_util::BrotliSubclassableAllocator;
use ::enc;
use ::enc::backward_references::BrotliEncoderParams;
use ::enc::encode::{BrotliEncoderParameter, set_parameter};
use ::enc::threading::{SendAlloc,Owned};
use alloc::SliceWrapper;
pub const MAX_THREADS: usize = 16;

struct SliceRef<'a> (&'a [u8]);
impl<'a> SliceWrapper<u8> for SliceRef<'a> {
    fn slice(&self) -> &[u8] { self.0 }
}


#[no_mangle]
pub unsafe extern fn BrotliEncoderCompressMulti(
  num_params: usize,
  param_keys: *const BrotliEncoderParameter,
  param_values: *const u32,
  input_size: usize,
  input: *const u8,
  encoded_size: *mut usize,
  encoded: *mut u8,
  desired_num_threads: usize,
  alloc_func: brotli_alloc_func,
  free_func: brotli_free_func,
  alloc_opaque_per_thread: *mut*mut c_void,
) -> i32 {
  if desired_num_threads == 0 {
    return 0;
  }
  let null_opaques = [core::ptr::null_mut::<c_void>();MAX_THREADS];
  let param_keys_slice = slice::from_raw_parts(param_keys, num_params);
  let param_values_slice = slice::from_raw_parts(param_values, num_params);
  let alloc_opaque = if alloc_opaque_per_thread.is_null() {
    &null_opaques[..]
  } else {
    slice::from_raw_parts(alloc_opaque_per_thread, desired_num_threads)
  };
  let mut params = BrotliEncoderParams::default();
  for (k,v) in param_keys_slice.iter().zip(param_values_slice.iter()) {
    if set_parameter(&mut params, *k, *v) == 0 {
      return 0;
    }
  }
  let num_threads = core::cmp::min(desired_num_threads, MAX_THREADS);
  let mut alloc_array:[_;MAX_THREADS] = [
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[0],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[1%desired_num_threads],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[2%desired_num_threads],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[3%desired_num_threads],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[4%desired_num_threads],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[5%desired_num_threads],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[6%desired_num_threads],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[7%desired_num_threads],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[8%desired_num_threads],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[9%desired_num_threads],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[10%desired_num_threads],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[11%desired_num_threads],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[12%desired_num_threads],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[13%desired_num_threads],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[14%desired_num_threads],
        }))),
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:alloc_func,
          free_func:free_func,
          opaque:alloc_opaque[15%desired_num_threads],
        }))),
    ];
    let res = enc::compress_multi(
        &params,
        &mut Owned::new(SliceRef(slice::from_raw_parts(input, input_size))),
        slice::from_raw_parts_mut(encoded, *encoded_size),
        &mut alloc_array[..num_threads],
    );
    match res {
      Ok(size) => {
        *encoded_size = size;
        return 1;
      },
      Err(_err) => {
        return 0;
      }
    }
}
