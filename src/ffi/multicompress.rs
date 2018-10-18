#![cfg(not(feature="safe"))]

#[no_mangle]
use core;
use core::slice;
#[allow(unused_imports)]
use brotli_decompressor;
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

#[cfg(feature="no-stdlib")]
fn brotli_new_work_pool_without_custom_alloc(_to_box: BrotliEncoderWorkPool) -> *mut BrotliEncoderWorkPool{
    panic!("Must supply allocators if calling divans when compiled with features=no-stdlib");
}

#[cfg(not(feature="no-stdlib"))]
fn brotli_new_work_pool_without_custom_alloc(to_box: BrotliEncoderWorkPool) -> *mut BrotliEncoderWorkPool{
    brotli_decompressor::ffi::alloc_util::Box::<BrotliEncoderWorkPool>::into_raw(
        brotli_decompressor::ffi::alloc_util::Box::<BrotliEncoderWorkPool>::new(to_box))
}
#[no_mangle]
pub unsafe extern fn BrotliEncoderMakeWorkPool(
  num_threads: usize,
  alloc_func: brotli_alloc_func,
  free_func: brotli_free_func,
  opaque: *mut c_void,
) -> *mut BrotliEncoderWorkPool {
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
}
#[cfg(not(feature="no-stdlib"))]
unsafe fn free_work_pool_no_custom_alloc(_work_pool: *mut BrotliEncoderWorkPool) {
    let _state = brotli_decompressor::ffi::alloc_util::Box::from_raw(_work_pool);
}

#[cfg(feature="no-stdlib")]
unsafe fn free_work_pool_no_custom_alloc(_work_pool: *mut BrotliEncoderWorkPool) {
    unreachable!();
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderDestroyWorkPool(work_pool_ptr: *mut BrotliEncoderWorkPool) {
  if let Some(_) = (*work_pool_ptr).custom_allocator.alloc_func {
    if let Some(free_fn) = (*work_pool_ptr).custom_allocator.free_func {
      {
        let _to_be_dropped = core::ptr::read(work_pool_ptr);
      }
      let _to_free = core::ptr::read(work_pool_ptr);
      let ptr = core::mem::transmute::<*mut BrotliEncoderWorkPool, *mut c_void>(work_pool_ptr);
      free_fn((*work_pool_ptr).custom_allocator.opaque, ptr);
    }
  } else {
    free_work_pool_no_custom_alloc(work_pool_ptr);
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
    return BrotliEncoderCompressMulti(
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
      alloc_opaque_per_thread);
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
    let res = enc::compress_worker_pool(
      &params,
      &mut Owned::new(SliceRef(slice::from_raw_parts(input, input_size))),
      slice::from_raw_parts_mut(encoded, *encoded_size),
      &mut alloc_array[..num_threads],
      &mut (*work_pool).work_pool,
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
