#![cfg(not(feature="safe"))]
#[cfg(feature="std")]
use std::{panic,thread, io};
#[cfg(feature="std")]
use std::io::Write;

#[no_mangle]
use core;
use core::slice;
#[allow(unused_imports)]
use brotli_decompressor;
use super::compressor;
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

macro_rules! make_send_alloc {
  ($alloc_func: expr, $free_func: expr, $opaque: expr) => (
    SendAlloc::new(BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(
        CAllocator{
          alloc_func:$alloc_func,
          free_func:$free_func,
          opaque:$opaque,
        })))
  )
}
#[no_mangle]
pub extern fn BrotliEncoderMaxCompressedSizeMulti(input_size: usize, num_threads: usize) -> usize {
  ::enc::encode::BrotliEncoderMaxCompressedSizeMulti(input_size, num_threads)
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
  let alloc_opaque = if alloc_opaque_per_thread.is_null() {
    &null_opaques[..]
  } else {
    slice::from_raw_parts(alloc_opaque_per_thread, desired_num_threads)
  };
  let param_keys_slice = slice::from_raw_parts(param_keys, num_params);
  let param_values_slice = slice::from_raw_parts(param_values, num_params);
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
    let res = enc::compress_multi_no_threadpool(
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

#[no_mangle]
#[repr(C)]
pub struct BrotliEncoderWorkPool {
  custom_allocator: CAllocator,
  work_pool: enc::WorkerPool<enc::CompressionThreadResult<BrotliSubclassableAllocator>,
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
#[cfg(test)]
#[cfg(feature="std")]
mod test {
    use super::*;
    use ::enc::encode::BrotliEncoderParameter;
    use core;
    #[test]
    fn test_compress_workpool() {
        let input = [102, 114, 111, 109, 32, 99, 116, 121, 112, 101, 115, 32, 105, 109, 112, 111, 114, 116, 32, 42, 10, 10, 99, 108, 97, 115, 115, 32, 69, 110, 117, 109, 84, 121, 112, 101, 40, 116, 121, 112, 101, 40, 99, 95, 117, 105, 110, 116, 41, 41, 58, 10, 32, 32, 32, 32, 100, 101, 102, 32, 95, 95, 110, 101, 119, 95, 95, 40, 109, 101, 116, 97, 99, 108, 115, 41, 58, 10, 32, 32, 32, 32, 32, 32, 32, 32, 112, 97, 115, 115, 10];
        let params = [BrotliEncoderParameter::BROTLI_PARAM_QUALITY, BrotliEncoderParameter::BROTLI_PARAM_LGWIN, BrotliEncoderParameter::BROTLI_PARAM_SIZE_HINT, BrotliEncoderParameter::BROTLI_PARAM_CATABLE, BrotliEncoderParameter::BROTLI_PARAM_MAGIC_NUMBER, BrotliEncoderParameter::BROTLI_PARAM_Q9_5];
        let values = [11u32,16u32,91u32,0u32,0u32,0u32];
        let mut encoded_size = BrotliEncoderMaxCompressedSizeMulti(input.len(), 4);
        let mut encoded_backing = [0u8;145];
        let encoded = &mut encoded_backing[..encoded_size];
        let ret = unsafe {
            let wp = BrotliEncoderCreateWorkPool(8, None, None, core::ptr::null_mut());
            let inner_ret = BrotliEncoderCompressWorkPool(
                wp,
                params.len(),
                params[..].as_ptr(),
                values[..].as_ptr(),
                input.len(),
                input[..].as_ptr(),
                &mut encoded_size,
                encoded.as_mut_ptr(),
                4,
                None,
                None,
                core::ptr::null_mut());
            BrotliEncoderDestroyWorkPool(wp);
            inner_ret
        };
        assert_eq!(ret, 1);
        let mut rt_size = 256;
        let mut rt_buffer = [0u8;256];
        let ret2 = unsafe {
            super::super::decompressor::CBrotliDecoderDecompress(encoded_size, encoded.as_ptr(),
                                                                 &mut rt_size, rt_buffer.as_mut_ptr())
        };
        match ret2 {
            super::super::decompressor::ffi::interface::BrotliDecoderResult::BROTLI_DECODER_RESULT_SUCCESS => {
            },
            _ => panic!(ret2),
        }
        assert_eq!(rt_size, input.len());
        assert_eq!(&rt_buffer[..rt_size], &input[..]);
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
      slice::from_raw_parts(alloc_opaque_per_thread, desired_num_threads)
    };
    let param_keys_slice = slice::from_raw_parts(param_keys, num_params);
    let param_values_slice = slice::from_raw_parts(param_values, num_params);
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
      &mut Owned::new(SliceRef(slice::from_raw_parts(input, input_size))),
      slice::from_raw_parts_mut(encoded, *encoded_size),
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
