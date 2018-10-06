pub use brotli_decompressor::ffi;
pub use brotli_decompressor::ffi::interface::{
  brotli_alloc_func,
  brotli_free_func,
  c_void,
};

pub unsafe extern fn BrotliDecoderCreateInstance(
    alloc_func: brotli_alloc_func,
    free_func: brotli_free_func,
    opaque: *mut c_void,
) -> *mut ffi::BrotliDecoderState {
   ffi::BrotliDecoderCreateInstance(alloc_func, free_func, opaque)
}

pub unsafe extern fn BrotliDecoderSetParameter(state_ptr: *mut ffi::BrotliDecoderState,
                                       selector: ffi::interface::BrotliDecoderParameter,
                                       value: u32) {
    ffi::BrotliDecoderSetParameter(state_ptr,  selector, value)
} 
     
#[cfg(not(feature="no-stdlib"))] // this requires a default allocator
pub unsafe extern fn BrotliDecoderDecompress(
  encoded_size: usize,
  encoded_buffer: *const u8,
  decoded_size: *mut usize,
  decoded_buffer: *mut u8) -> ffi::interface::BrotliDecoderResult {
    ffi::BrotliDecoderDecompress(encoded_size, encoded_buffer, decoded_size, decoded_buffer)
}

pub unsafe extern fn BrotliDecoderDecompressStream(
    state_ptr: *mut ffi::BrotliDecoderState,
    available_in: *mut usize,
    input_buf_ptr: *mut*const u8,
    available_out: *mut usize,
    output_buf_ptr: *mut*mut u8,
    total_out: *mut usize,
) -> ffi::interface::BrotliDecoderResult {
ffi::BrotliDecoderDecompressStream(
  state_ptr,
  available_in,
  input_buf_ptr,
  available_out,
  output_buf_ptr,
  total_out)
}

pub unsafe extern fn BrotliDecoderMallocU8(state_ptr: *mut ffi::BrotliDecoderState, size: usize) -> *mut u8 {
  ffi::BrotliDecoderMallocU8(state_ptr, size)
}

pub unsafe extern fn BrotliDecoderFreeU8(state_ptr: *mut ffi::BrotliDecoderState, data: *mut u8, size: usize) {
  ffi::BrotliDecoderFreeU8(state_ptr, data, size)
}

pub unsafe extern fn BrotliDecoderMallocUsize(state_ptr: *mut ffi::BrotliDecoderState, size: usize) -> *mut usize {
  ffi::BrotliDecoderMallocUsize(state_ptr, size)
}

pub unsafe extern fn BrotliDecoderFreeUsize(state_ptr: *mut ffi::BrotliDecoderState, data: *mut usize, size: usize) {
  ffi::BrotliDecoderFreeUsize(state_ptr, data, size)
}

pub unsafe extern fn BrotliDecoderDestroyInstance(state_ptr: *mut ffi::BrotliDecoderState) {
  ffi::BrotliDecoderDestroyInstance(state_ptr)
}

pub extern fn BrotliDecoderVersion() -> u32 {
  ffi::BrotliDecoderVersion()
}

#[no_mangle]
pub extern fn BrotliDecoderErrorString(c: ffi::BrotliDecoderErrorCode) -> *const u8 {
  ffi::BrotliDecoderErrorString(c)
}

#[no_mangle]
pub unsafe extern fn BrotliDecoderHasMoreOutput(state_ptr: *const ffi::BrotliDecoderState) -> i32 {
  ffi::BrotliDecoderHasMoreOutput(state_ptr)
}

#[no_mangle]
pub unsafe extern fn BrotliDecoderTakeOutput(state_ptr: *mut ffi::BrotliDecoderState, size: *mut usize) -> *const u8 {
  ffi::BrotliDecoderTakeOutput(state_ptr, size)
}



#[no_mangle]
pub unsafe extern fn BrotliDecoderIsUsed(state_ptr: *const ffi::BrotliDecoderState) -> i32 {
  ffi::BrotliDecoderIsUsed(state_ptr)
}
#[no_mangle]
pub unsafe extern fn BrotliDecoderIsFinished(state_ptr: *const ffi::BrotliDecoderState) -> i32 {
  ffi::BrotliDecoderIsFinished(state_ptr)
}
#[no_mangle]
pub unsafe extern fn BrotliDecoderGetErrorCode(state_ptr: *const ffi::BrotliDecoderState) -> ffi::BrotliDecoderErrorCode {
  ffi::BrotliDecoderGetErrorCode(state_ptr)
}
