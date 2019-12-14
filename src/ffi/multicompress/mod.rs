#![cfg(not(feature="safe"))]
#[cfg(feature="std")]
use std::{panic,thread, io};
#[cfg(feature="std")]
use std::io::Write;
mod test;
#[no_mangle]
use core;
#[allow(unused_imports)]
use brotli_decompressor;
use super::compressor;
use ::enc::encode::{BrotliEncoderCreateInstance,
                    BrotliEncoderOperation,
                    BrotliEncoderSetParameter,
                    BrotliEncoderCompressStream,
                    BrotliEncoderIsFinished,
                    BrotliEncoderDestroyInstance,
};
use brotli_decompressor::ffi::alloc_util::SubclassableAllocator;
use brotli_decompressor::ffi::interface::{
  brotli_alloc_func,
  brotli_free_func,
  CAllocator,
  c_void,
};
use brotli_decompressor::ffi::{
  slice_from_raw_parts_or_nil,
  slice_from_raw_parts_or_nil_mut,
};

use super::alloc_util::BrotliSubclassableAllocator;
use ::enc;
use ::enc::backward_references::{BrotliEncoderParams, UnionHasher};
use ::enc::encode::{BrotliEncoderParameter, set_parameter};
use ::enc::threading::{SendAlloc,Owned};
use alloc::SliceWrapper;
pub const MAX_THREADS: usize = 16;

struct SliceRef<'a> (&'a [u8]);
impl<'a> SliceWrapper<u8> for SliceRef<'a> {
    fn slice(&self) -> &[u8] { self.0 }
}

macro_rules! make_send_alloc {
  ($alloc_func: expr, $free_func: expr, $opaque: expr) => (
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:$alloc_func,
          free_func:$free_func,
          opaque:$opaque,
        })), UnionHasher::Uninit)
  )
}
#[no_mangle]
pub extern fn BrotliEncoderMaxCompressedSizeMulti(input_size: usize, num_threads: usize) -> usize {
  ::enc::encode::BrotliEncoderMaxCompressedSizeMulti(input_size, num_threads)
}

fn help_brotli_encoder_compress_single(
  param_keys: &[BrotliEncoderParameter],
  param_values: &[u32],
  input: &[u8],
  output: &mut[u8],
  encoded_size: &mut usize,
  m8: BrotliSubclassableAllocator,
) -> i32 {
  let mut encoder = BrotliEncoderCreateInstance(m8);
  for (p, v) in param_keys.iter().zip(param_values.iter()) {
    BrotliEncoderSetParameter(&mut encoder, *p, *v);
  }
  let mut result;
  let mut available_in = input.len();
  let mut next_in_offset = 0usize;
  let mut available_out = output.len();
  let mut next_out_offset = 0usize;
  let mut total_out = Some(0usize);
  result = BrotliEncoderCompressStream(&mut encoder,
                                       BrotliEncoderOperation::BROTLI_OPERATION_FINISH,
                                       &mut available_in,
                                       input,
                                       &mut next_in_offset,  
                                       &mut available_out,
                                       output,
                                       &mut next_out_offset,
                                       &mut total_out,
                                       &mut |_a,_b,_c,_d|());
  if BrotliEncoderIsFinished(&encoder) == 0 {
    result = 0i32;
  }
  *encoded_size = total_out.unwrap();
  BrotliEncoderDestroyInstance(&mut encoder);
  result
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
  let num_threads = core::cmp::min(desired_num_threads, MAX_THREADS);
  match compressor::catch_panic(|| {
    let param_keys_slice = slice_from_raw_parts_or_nil(param_keys, num_params);
    let param_values_slice = slice_from_raw_parts_or_nil(param_values, num_params);
    let input_slice = slice_from_raw_parts_or_nil(input, input_size);
    let output_slice = slice_from_raw_parts_or_nil_mut(encoded, *encoded_size);
    if num_threads == 1 {
      let allocators = CAllocator {
        alloc_func:alloc_func,
        free_func:free_func,
        opaque:if alloc_opaque_per_thread.is_null() {core::ptr::null_mut()} else {*alloc_opaque_per_thread},
      };
      let m8 = BrotliSubclassableAllocator::new(
        SubclassableAllocator::new(allocators.clone()));
      return help_brotli_encoder_compress_single(
        param_keys_slice,
        param_values_slice,
        input_slice,
        output_slice,
        &mut *encoded_size,
        m8,
      )
    }
    let null_opaques = [core::ptr::null_mut::<c_void>();MAX_THREADS];
    let alloc_opaque = if alloc_opaque_per_thread.is_null() {
      &null_opaques[..]
    } else {
      slice_from_raw_parts_or_nil(alloc_opaque_per_thread, desired_num_threads)
    };
    let mut params = BrotliEncoderParams::default();
    for (k,v) in param_keys_slice.iter().zip(param_values_slice.iter()) {
      if set_parameter(&mut params, *k, *v) == 0 {
        return 0;
      }
    }
    let mut alloc_array:[_;MAX_THREADS] = [
      make_send_alloc!(alloc_func, free_func, alloc_opaque[0]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[1%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[2%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[3%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[4%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[5%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[6%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[7%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[8%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[9%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[10%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[11%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[12%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[13%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[14%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[15%desired_num_threads]),
    ];

    let owned_input = &mut Owned::new(SliceRef(input_slice));
    let res = enc::compress_multi_no_threadpool(
      &params,
      owned_input,
      output_slice,
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
  }) {
    Ok(ret) => return ret,
    Err(panic_err) => {
      error_print(panic_err);
      return 0;
    },
  }
}

#[no_mangle]
#[repr(C)]
pub struct BrotliEncoderWorkPool {
  custom_allocator: CAllocator,
  work_pool: enc::WorkerPool<enc::CompressionThreadResult<BrotliSubclassableAllocator>,
                             UnionHasher<BrotliSubclassableAllocator>,
                             BrotliSubclassableAllocator,
                             (SliceRef<'static>, BrotliEncoderParams)>,
}

#[cfg(not(feature="std"))]
fn brotli_new_work_pool_without_custom_alloc(_to_box: BrotliEncoderWorkPool) -> *mut BrotliEncoderWorkPool{
    panic!("Must supply allocators if calling divans when compiled without features=std");
}

#[cfg(feature="std")]
fn brotli_new_work_pool_without_custom_alloc(to_box: BrotliEncoderWorkPool) -> *mut BrotliEncoderWorkPool{
    brotli_decompressor::ffi::alloc_util::Box::<BrotliEncoderWorkPool>::into_raw(
        brotli_decompressor::ffi::alloc_util::Box::<BrotliEncoderWorkPool>::new(to_box))
}
#[no_mangle]
pub unsafe extern fn BrotliEncoderCreateWorkPool(
  num_threads: usize,
  alloc_func: brotli_alloc_func,
  free_func: brotli_free_func,
  opaque: *mut c_void,
) -> *mut BrotliEncoderWorkPool {
  match catch_panic_wstate(|| {
    let allocators = CAllocator {
    
      alloc_func:alloc_func,
      free_func:free_func,
      opaque:opaque,
    };
    let to_box = BrotliEncoderWorkPool {
      custom_allocator: allocators.clone(),
      work_pool: enc::new_work_pool(core::cmp::min(num_threads, MAX_THREADS)),
    };
    if let Some(alloc) = alloc_func {
      if free_func.is_none() {
        panic!("either both alloc and free must exist or neither");
      }
      let ptr = alloc(allocators.opaque, core::mem::size_of::<BrotliEncoderWorkPool>());
      let brotli_work_pool_ptr = core::mem::transmute::<*mut c_void, *mut BrotliEncoderWorkPool>(ptr);
      core::ptr::write(brotli_work_pool_ptr, to_box);
      brotli_work_pool_ptr
    } else {
      brotli_new_work_pool_without_custom_alloc(to_box)
    }
  }) {
      Ok(ret) => ret,
      Err(err) => {
          error_print(err);
          core::ptr::null_mut()
      }
  }
}
#[cfg(feature="std")]
unsafe fn free_work_pool_no_custom_alloc(_work_pool: *mut BrotliEncoderWorkPool) {
    let _state = brotli_decompressor::ffi::alloc_util::Box::from_raw(_work_pool);
}

#[cfg(not(feature="std"))]
unsafe fn free_work_pool_no_custom_alloc(_work_pool: *mut BrotliEncoderWorkPool) {
    unreachable!();
}
struct UnsafeUnwindBox(*mut BrotliEncoderWorkPool);
#[cfg(all(feature="std", not(feature="pass-through-ffi-panics")))]
impl panic::RefUnwindSafe for UnsafeUnwindBox{}

#[no_mangle]
pub unsafe extern fn BrotliEncoderDestroyWorkPool(work_pool_ptr: *mut BrotliEncoderWorkPool) {
  let wpp = UnsafeUnwindBox(work_pool_ptr);
  if let Err(panic_err) = compressor::catch_panic(|| {
  if let Some(_) = (*wpp.0).custom_allocator.alloc_func {
    if let Some(free_fn) = (*wpp.0).custom_allocator.free_func {
      let _to_free = core::ptr::read(wpp.0);
      let ptr = core::mem::transmute::<*mut BrotliEncoderWorkPool, *mut c_void>(wpp.0);
      free_fn((*wpp.0).custom_allocator.opaque, ptr);
    }
  } else {
    free_work_pool_no_custom_alloc(wpp.0);
  }
  0
  }) {
      error_print(panic_err);
  }
}
#[no_mangle]
pub unsafe extern fn BrotliEncoderCompressWorkPool(
  work_pool: *mut BrotliEncoderWorkPool,
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
  if work_pool.is_null() {
    match compressor::catch_panic(|| BrotliEncoderCompressMulti(
      num_params,
      param_keys,
      param_values,
      input_size,
      input,
      encoded_size,
      encoded,
      desired_num_threads,
      alloc_func,
      free_func,
      alloc_opaque_per_thread)) {
        Ok(ret) => return ret, // no panic
        Err(panic_err) => {
          error_print(panic_err); // print panic
          return 0; // fail
      }
    }
  }
  let work_pool_wrapper = UnsafeUnwindBox(work_pool);
  match compressor::catch_panic(|| {
    let null_opaques = [core::ptr::null_mut::<c_void>();MAX_THREADS];
    let alloc_opaque = if alloc_opaque_per_thread.is_null() {
      &null_opaques[..]
    } else {
      slice_from_raw_parts_or_nil(alloc_opaque_per_thread, desired_num_threads)
    };
    let param_keys_slice = slice_from_raw_parts_or_nil(param_keys, num_params);
    let param_values_slice = slice_from_raw_parts_or_nil(param_values, num_params);
    let mut params = BrotliEncoderParams::default();
    for (k,v) in param_keys_slice.iter().zip(param_values_slice.iter()) {
      if set_parameter(&mut params, *k, *v) == 0 {
        return 0;
      }
    }
    let num_threads = core::cmp::min(desired_num_threads, MAX_THREADS);
    let mut alloc_array:[_;MAX_THREADS] = [
      make_send_alloc!(alloc_func, free_func, alloc_opaque[0]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[1%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[2%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[3%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[4%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[5%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[6%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[7%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[8%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[9%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[10%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[11%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[12%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[13%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[14%desired_num_threads]),
      make_send_alloc!(alloc_func, free_func, alloc_opaque[15%desired_num_threads]),
    ];
    let res = enc::compress_worker_pool(
      &params,
      &mut Owned::new(SliceRef(slice_from_raw_parts_or_nil(input, input_size))),
      slice_from_raw_parts_or_nil_mut(encoded, *encoded_size),
      &mut alloc_array[..num_threads],
      &mut (*work_pool_wrapper.0).work_pool,
    );
    match res {
      Ok(size) => {
        *encoded_size = size;
        return 1;
      },
      Err(_err) => {
        return 0;
      },
    }
  }) {
    Ok(ret) => ret, // no panic
    Err(panic_err) => {
      error_print(panic_err); // print panic
      0 // fail
    },
  }
}

#[cfg(all(feature="std", not(feature="pass-through-ffi-panics")))]
fn catch_panic_wstate<F:FnOnce()->*mut BrotliEncoderWorkPool+panic::UnwindSafe>(f: F) -> thread::Result<*mut BrotliEncoderWorkPool> {
    panic::catch_unwind(f)
}

#[cfg(all(feature="std", not(feature="pass-through-ffi-panics")))]
fn error_print<Err:core::fmt::Debug>(err: Err) {
    let _ign = writeln!(&mut io::stderr(), "Internal Error {:?}", err);
}

#[cfg(any(not(feature="std"), feature="pass-through-ffi-panics"))]
fn catch_panic_wstate<F:FnOnce()->*mut BrotliEncoderWorkPool>(f: F) -> Result<*mut BrotliEncoderWorkPool, ()> {
    Ok(f())
}

#[cfg(any(not(feature="std"), feature="pass-through-ffi-panics"))]
fn error_print<Err>(_err: Err) {
}
