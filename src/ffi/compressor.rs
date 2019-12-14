#![cfg(not(feature="safe"))]

#[cfg(feature="std")]
use std::{panic,thread, io};
#[cfg(feature="std")]
use std::io::Write;

#[no_mangle]
use core;
use brotli_decompressor::ffi::alloc_util;
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
use ::enc::encode::BrotliEncoderStateStruct;
use super::alloc_util::BrotliSubclassableAllocator;

#[repr(C)]
#[no_mangle]
pub enum BrotliEncoderOperation {
  BROTLI_OPERATION_PROCESS = 0,
  BROTLI_OPERATION_FLUSH = 1,
  BROTLI_OPERATION_FINISH = 2,
  BROTLI_OPERATION_EMIT_METADATA = 3,
}

#[repr(C)]
#[no_mangle]
pub enum BrotliEncoderMode {
  BROTLI_MODE_GENERIC = 0,
  BROTLI_MODE_TEXT = 1,
  BROTLI_MODE_FONT = 2,
  BROTLI_MODE_FORCE_LSB_PRIOR = 3,
  BROTLI_MODE_FORCE_MSB_PRIOR = 4,
  BROTLI_MODE_FORCE_UTF8_PRIOR = 5,
  BROTLI_MODE_FORCE_SIGNED_PRIOR = 6,
}

#[repr(C)]
#[no_mangle]
pub struct BrotliEncoderState {
  pub custom_allocator: CAllocator,
  pub compressor: BrotliEncoderStateStruct<BrotliSubclassableAllocator>,
}

#[cfg(not(feature="std"))]
fn brotli_new_compressor_without_custom_alloc(_to_box: BrotliEncoderState) -> *mut BrotliEncoderState{
    panic!("Must supply allocators if calling divans when compiled without features=std");
}

#[cfg(feature="std")]
fn brotli_new_compressor_without_custom_alloc(to_box: BrotliEncoderState) -> *mut BrotliEncoderState{
    alloc_util::Box::<BrotliEncoderState>::into_raw(
        alloc_util::Box::<BrotliEncoderState>::new(to_box))
}
#[cfg(feature="std")]
unsafe fn free_compressor_no_custom_alloc(state_ptr: *mut BrotliEncoderState) {
    let _state = alloc_util::Box::from_raw(state_ptr);
}

#[cfg(not(feature="std"))]
unsafe fn free_compressor_no_custom_alloc(_state_ptr: *mut BrotliEncoderState) {
    unreachable!();
}




#[no_mangle]
pub unsafe extern fn BrotliEncoderCreateInstance(
    alloc_func: brotli_alloc_func,
    free_func: brotli_free_func,
    opaque: *mut c_void,
) -> *mut BrotliEncoderState {
  match catch_panic_cstate(|| {
    let allocators = CAllocator {
      alloc_func:alloc_func,
      free_func:free_func,
      opaque:opaque,
    };
    let to_box = BrotliEncoderState {
      custom_allocator: allocators.clone(),
      compressor: ::enc::encode::BrotliEncoderCreateInstance(
        BrotliSubclassableAllocator::new(
          SubclassableAllocator::new(allocators.clone())),
      ),
    };
    if let Some(alloc) = alloc_func {
      if free_func.is_none() {
          panic!("either both alloc and free must exist or neither");
      }
     let ptr = alloc(allocators.opaque, core::mem::size_of::<BrotliEncoderState>());
      let brotli_decoder_state_ptr = core::mem::transmute::<*mut c_void, *mut BrotliEncoderState>(ptr);
      core::ptr::write(brotli_decoder_state_ptr, to_box);
      brotli_decoder_state_ptr
    } else {
      brotli_new_compressor_without_custom_alloc(to_box)
    }
  }) {
    Ok(ret) => ret,
    Err(err) => {
      error_print(err);
      core::ptr::null_mut()
    }
  }
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderSetParameter(
  state_ptr: *mut BrotliEncoderState,
  param: ::enc::encode::BrotliEncoderParameter,
  value: u32,
) -> i32 {
  ::enc::encode::BrotliEncoderSetParameter(&mut (*state_ptr).compressor, param, value)
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderDestroyInstance(state_ptr: *mut BrotliEncoderState) {
  ::enc::encode::BrotliEncoderDestroyInstance(&mut (*state_ptr).compressor);
  if let Some(_) = (*state_ptr).custom_allocator.alloc_func {
    if let Some(free_fn) = (*state_ptr).custom_allocator.free_func {
      let _to_free = core::ptr::read(state_ptr);
      let ptr = core::mem::transmute::<*mut BrotliEncoderState, *mut c_void>(state_ptr);
      free_fn((*state_ptr).custom_allocator.opaque, ptr);
    }
  } else {
    free_compressor_no_custom_alloc(state_ptr);
  }
}
#[no_mangle]
pub unsafe extern fn BrotliEncoderIsFinished(
  state_ptr: *mut BrotliEncoderState,
) -> i32 {
  ::enc::encode::BrotliEncoderIsFinished(&mut (*state_ptr).compressor)
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderHasMoreOutput(
  state_ptr: *mut BrotliEncoderState,
) -> i32 {
  ::enc::encode::BrotliEncoderHasMoreOutput(&mut (*state_ptr).compressor)
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderSetCustomDictionary(
  state_ptr: *mut BrotliEncoderState,
  size: usize,
  dict: *const u8,
) {
  if let Err(panic_err) = catch_panic(|| {
      let dict_slice = slice_from_raw_parts_or_nil(dict, size);
      ::enc::encode::BrotliEncoderSetCustomDictionary(&mut (*state_ptr).compressor, size, dict_slice);
      0
  }) {
      error_print(panic_err);
  }
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderTakeOutput(
  state_ptr: *mut BrotliEncoderState,
  size: *mut usize,
) -> *const u8 {
  ::enc::encode::BrotliEncoderTakeOutput(&mut (*state_ptr).compressor, &mut *size).as_ptr()
}
#[no_mangle]
pub extern fn BrotliEncoderVersion() -> u32 {
  ::enc::encode::BrotliEncoderVersion()
}
#[no_mangle]
pub extern fn BrotliEncoderMaxCompressedSize(input_size: usize) -> usize {
  ::enc::encode::BrotliEncoderMaxCompressedSize(input_size)
}
#[no_mangle]
pub unsafe extern fn BrotliEncoderCompress(
  quality: i32,
  lgwin: i32,
  mode: BrotliEncoderMode,
  input_size: usize,
  input_buffer: *const u8,
  encoded_size: *mut usize,
  encoded_buffer: *mut u8) -> i32 {
  match catch_panic(|| {
    let input_buf = slice_from_raw_parts_or_nil(input_buffer, input_size);
    let encoded_buf = slice_from_raw_parts_or_nil_mut(encoded_buffer, *encoded_size);
    let allocators = CAllocator {
        alloc_func:None,
        free_func:None,
        opaque:core::ptr::null_mut(),
    };
    let translated_mode = match mode {
      BrotliEncoderMode::BROTLI_MODE_GENERIC =>
        ::enc::backward_references::BrotliEncoderMode::BROTLI_MODE_GENERIC,
      BrotliEncoderMode::BROTLI_MODE_TEXT =>
        ::enc::backward_references::BrotliEncoderMode::BROTLI_MODE_TEXT,
      BrotliEncoderMode::BROTLI_MODE_FONT =>
        ::enc::backward_references::BrotliEncoderMode::BROTLI_MODE_FONT,
      BrotliEncoderMode::BROTLI_MODE_FORCE_LSB_PRIOR =>
        ::enc::backward_references::BrotliEncoderMode::BROTLI_FORCE_LSB_PRIOR,
      BrotliEncoderMode::BROTLI_MODE_FORCE_MSB_PRIOR =>
        ::enc::backward_references::BrotliEncoderMode::BROTLI_FORCE_MSB_PRIOR,
      BrotliEncoderMode::BROTLI_MODE_FORCE_UTF8_PRIOR =>
        ::enc::backward_references::BrotliEncoderMode::BROTLI_FORCE_UTF8_PRIOR,
      BrotliEncoderMode::BROTLI_MODE_FORCE_SIGNED_PRIOR =>
        ::enc::backward_references::BrotliEncoderMode::BROTLI_FORCE_SIGNED_PRIOR,
    };
    let mut m8 = BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(allocators.clone()));
    let empty_m8 = BrotliSubclassableAllocator::new(
      SubclassableAllocator::new(allocators.clone()));
  
    ::enc::encode::BrotliEncoderCompress(
      empty_m8,
      &mut m8,
      quality,
      lgwin,
      translated_mode,
      input_size,
      input_buf,
      &mut *encoded_size,
      encoded_buf,
      &mut |_a,_b,_c,_d|(),
      )

  }) {
    Ok(ret) => ret,
    Err(panic_err) => {
      error_print(panic_err);
      0
    },
  }
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderCompressStreaming(
  state_ptr: *mut BrotliEncoderState,
  op: BrotliEncoderOperation,
  available_in: *mut usize,
  mut input_buf: *const u8,
  available_out: *mut usize,
  mut output_buf: *mut u8,
) -> i32 {
  BrotliEncoderCompressStream(state_ptr,
                              op,
                              available_in,
                              &mut input_buf,
                              available_out,
                              &mut output_buf,
                              core::ptr::null_mut())
                              
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderCompressStream(
  state_ptr: *mut BrotliEncoderState,
  op: BrotliEncoderOperation,
  available_in: *mut usize,
  input_buf_ptr: *mut*const u8,
  available_out: *mut usize,
  output_buf_ptr: *mut*mut u8,
  total_out: *mut usize) -> i32 {
  match catch_panic(|| {
    let mut input_offset = 0usize;
    let mut output_offset = 0usize;
    let result;
    let translated_op = match op {
      BrotliEncoderOperation::BROTLI_OPERATION_PROCESS =>
        ::enc::encode::BrotliEncoderOperation::BROTLI_OPERATION_PROCESS,
      BrotliEncoderOperation::BROTLI_OPERATION_FLUSH =>
        ::enc::encode::BrotliEncoderOperation::BROTLI_OPERATION_FLUSH,
      BrotliEncoderOperation::BROTLI_OPERATION_FINISH =>
        ::enc::encode::BrotliEncoderOperation::BROTLI_OPERATION_FINISH,
      BrotliEncoderOperation::BROTLI_OPERATION_EMIT_METADATA =>
        ::enc::encode::BrotliEncoderOperation::BROTLI_OPERATION_EMIT_METADATA,
    };
    {
      let input_buf = slice_from_raw_parts_or_nil(*input_buf_ptr, *available_in);
      let output_buf = slice_from_raw_parts_or_nil_mut(*output_buf_ptr, *available_out);
      let mut to = Some(0usize);
      result = ::enc::encode::BrotliEncoderCompressStream(
        &mut (*state_ptr).compressor,
        translated_op,
        &mut *available_in,
        input_buf,
        &mut input_offset,
        &mut *available_out,
        output_buf,
        &mut output_offset,
        &mut to,
        &mut |_a,_b,_c,_d|(),
      );
      if !total_out.is_null() {
        *total_out = to.unwrap_or(0);
      }
    }
    *input_buf_ptr = (*input_buf_ptr).offset(input_offset as isize);
    *output_buf_ptr = (*output_buf_ptr).offset(output_offset as isize);
    result
  }) {
    Ok(ret) => ret,
    Err(panic_err) => {
      error_print(panic_err);
      0
    },
  }
}


#[no_mangle]
pub unsafe extern fn BrotliEncoderMallocU8(state_ptr: *mut BrotliEncoderState, size: usize) -> *mut u8 {
    if let Some(alloc_fn) = (*state_ptr).custom_allocator.alloc_func {
        return core::mem::transmute::<*mut c_void, *mut u8>(alloc_fn((*state_ptr).custom_allocator.opaque, size));
    } else {
        return alloc_util::alloc_stdlib(size);
    }
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderFreeU8(state_ptr: *mut BrotliEncoderState, data: *mut u8, size: usize) {
    if let Some(free_fn) = (*state_ptr).custom_allocator.free_func {
        free_fn((*state_ptr).custom_allocator.opaque, core::mem::transmute::<*mut u8, *mut c_void>(data));
    } else {
        alloc_util::free_stdlib(data, size);
    }
}

#[no_mangle]
pub unsafe extern fn BrotliEncoderMallocUsize(state_ptr: *mut BrotliEncoderState, size: usize) -> *mut usize {
    if let Some(alloc_fn) = (*state_ptr).custom_allocator.alloc_func {
        return core::mem::transmute::<*mut c_void, *mut usize>(alloc_fn((*state_ptr).custom_allocator.opaque,
                                                                         size * core::mem::size_of::<usize>()));
    } else {
        return alloc_util::alloc_stdlib(size);
    }
}
#[no_mangle]
pub unsafe extern fn BrotliEncoderFreeUsize(state_ptr: *mut BrotliEncoderState, data: *mut usize, size: usize) {
    if let Some(free_fn) = (*state_ptr).custom_allocator.free_func {
        free_fn((*state_ptr).custom_allocator.opaque, core::mem::transmute::<*mut usize, *mut c_void>(data));
    } else {
        alloc_util::free_stdlib(data, size);
    }
}



#[cfg(all(feature="std", not(feature="pass-through-ffi-panics")))]
pub fn catch_panic<F:FnOnce()->i32+panic::UnwindSafe>(f: F) -> thread::Result<i32> {
    panic::catch_unwind(f)
}

#[cfg(all(feature="std", not(feature="pass-through-ffi-panics")))]
fn catch_panic_cstate<F:FnOnce()->*mut BrotliEncoderState+panic::UnwindSafe>(f: F) -> thread::Result<*mut BrotliEncoderState> {
    panic::catch_unwind(f)
}

#[cfg(all(feature="std", not(feature="pass-through-ffi-panics")))]
fn error_print<Err:core::fmt::Debug>(err: Err) {
    let _ign = writeln!(&mut io::stderr(), "Internal Error {:?}", err);
}

// can't catch panics in a reliable way without std:: configure with panic=abort. These shouldn't happen
#[cfg(any(not(feature="std"), feature="pass-through-ffi-panics"))]
pub fn catch_panic<F:FnOnce()->i32>(f: F) -> Result<i32, ()> {
    Ok(f())
}

#[cfg(any(not(feature="std"), feature="pass-through-ffi-panics"))]
fn catch_panic_cstate<F:FnOnce()->*mut BrotliEncoderState>(f: F) -> Result<*mut BrotliEncoderState, ()> {
    Ok(f())
}

#[cfg(any(not(feature="std"), feature="pass-through-ffi-panics"))]
fn error_print<Err>(_err: Err) {
}
