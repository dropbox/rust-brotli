#![cfg(test)]
#![cfg(feature = "std")]

use core;

use std::alloc::{alloc, dealloc, Layout};
use std::panic;

use super::*;
use crate::enc::encode::BrotliEncoderParameter;

extern "C" fn failing_alloc(_opaque: *mut c_void, _size: usize) -> *mut c_void {
    core::ptr::null_mut()
}

extern "C" fn failing_free(_opaque: *mut c_void, _address: *mut c_void) {}

#[test]
fn test_create_work_pool_returns_null_on_allocator_failure() {
    let wp = unsafe {
        BrotliEncoderCreateWorkPool(
            8,
            Some(failing_alloc),
            Some(failing_free),
            core::ptr::null_mut(),
        )
    };
    assert!(wp.is_null());
}

#[test]
fn test_compress_workpool() {
    let input = [
        102, 114, 111, 109, 32, 99, 116, 121, 112, 101, 115, 32, 105, 109, 112, 111, 114, 116, 32,
        42, 10, 10, 99, 108, 97, 115, 115, 32, 69, 110, 117, 109, 84, 121, 112, 101, 40, 116, 121,
        112, 101, 40, 99, 95, 117, 105, 110, 116, 41, 41, 58, 10, 32, 32, 32, 32, 100, 101, 102,
        32, 95, 95, 110, 101, 119, 95, 95, 40, 109, 101, 116, 97, 99, 108, 115, 41, 58, 10, 32, 32,
        32, 32, 32, 32, 32, 32, 112, 97, 115, 115, 10,
    ];
    let params = [
        BrotliEncoderParameter::BROTLI_PARAM_QUALITY,
        BrotliEncoderParameter::BROTLI_PARAM_LGWIN,
        BrotliEncoderParameter::BROTLI_PARAM_SIZE_HINT,
        BrotliEncoderParameter::BROTLI_PARAM_CATABLE,
        BrotliEncoderParameter::BROTLI_PARAM_MAGIC_NUMBER,
        BrotliEncoderParameter::BROTLI_PARAM_Q9_5,
    ];
    let values = [11u32, 16, 91, 0, 0, 0];
    let mut encoded_size = BrotliEncoderMaxCompressedSizeMulti(input.len(), 4);
    let mut encoded_backing = [0u8; 145];
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
            core::ptr::null_mut(),
        );
        BrotliEncoderDestroyWorkPool(wp);
        inner_ret
    };
    assert_eq!(ret, 1);
    let mut rt_size = 256;
    let mut rt_buffer = [0u8; 256];
    let ret2 = unsafe {
        super::super::decompressor::CBrotliDecoderDecompress(
            encoded_size,
            encoded.as_ptr(),
            &mut rt_size,
            rt_buffer.as_mut_ptr(),
        )
    };
    match ret2 {
    super::super::decompressor::ffi::interface::BrotliDecoderResult::BROTLI_DECODER_RESULT_SUCCESS => {
    },
    _ => panic!("{}", ret2 as i32),
  }
    assert_eq!(rt_size, input.len());
    assert_eq!(&rt_buffer[..rt_size], &input[..]);
}

#[test]
fn test_compress_empty_workpool() {
    let input = [];
    let params = [
        BrotliEncoderParameter::BROTLI_PARAM_QUALITY,
        BrotliEncoderParameter::BROTLI_PARAM_LGWIN,
        BrotliEncoderParameter::BROTLI_PARAM_SIZE_HINT,
        BrotliEncoderParameter::BROTLI_PARAM_CATABLE,
        BrotliEncoderParameter::BROTLI_PARAM_MAGIC_NUMBER,
        BrotliEncoderParameter::BROTLI_PARAM_Q9_5,
    ];
    let values = [3u32, 16, 91, 0, 0, 0];
    let mut encoded_size = BrotliEncoderMaxCompressedSizeMulti(input.len(), 4);
    let mut encoded_backing = [0u8; 145];
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
            core::ptr::null_mut(),
        );
        BrotliEncoderDestroyWorkPool(wp);
        inner_ret
    };
    assert_eq!(ret, 1);
    let mut rt_size = 256;
    let mut rt_buffer = [0u8; 256];
    assert_ne!(encoded_size, 0);
    let ret2 = unsafe {
        super::super::decompressor::CBrotliDecoderDecompress(
            encoded_size,
            encoded.as_ptr(),
            &mut rt_size,
            rt_buffer.as_mut_ptr(),
        )
    };
    match ret2 {
    super::super::decompressor::ffi::interface::BrotliDecoderResult::BROTLI_DECODER_RESULT_SUCCESS => {
    },
    _ => panic!("{}", ret2 as i32),
  }
    assert_eq!(rt_size, input.len());
    assert_eq!(&rt_buffer[..rt_size], &input[..]);
}

#[test]
fn test_compress_empty_multi_raw() {
    let input = [];
    let params = [
        BrotliEncoderParameter::BROTLI_PARAM_QUALITY,
        BrotliEncoderParameter::BROTLI_PARAM_LGWIN,
        BrotliEncoderParameter::BROTLI_PARAM_SIZE_HINT,
        BrotliEncoderParameter::BROTLI_PARAM_CATABLE,
        BrotliEncoderParameter::BROTLI_PARAM_MAGIC_NUMBER,
        BrotliEncoderParameter::BROTLI_PARAM_Q9_5,
    ];
    let values = [3u32, 16, 0, 0, 0, 0];
    let mut encoded_size = BrotliEncoderMaxCompressedSizeMulti(input.len(), 4);
    let mut encoded_backing = [0u8; 145];
    let encoded = &mut encoded_backing[..encoded_size];
    let ret = unsafe {
        BrotliEncoderCompressMulti(
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
            core::ptr::null_mut(),
        )
    };
    assert_eq!(ret, 1);
    let mut rt_size = 256;
    let mut rt_buffer = [0u8; 256];
    assert_ne!(encoded_size, 0);
    let ret2 = unsafe {
        super::super::decompressor::CBrotliDecoderDecompress(
            encoded_size,
            encoded.as_ptr(),
            &mut rt_size,
            rt_buffer.as_mut_ptr(),
        )
    };
    match ret2 {
    super::super::decompressor::ffi::interface::BrotliDecoderResult::BROTLI_DECODER_RESULT_SUCCESS => {
    },
    _ => panic!("{}", ret2 as i32),
  }
    assert_eq!(rt_size, input.len());
    assert_eq!(&rt_buffer[..rt_size], &input[..]);
}

#[test]
fn test_compress_null_multi_raw() {
    let params = [
        BrotliEncoderParameter::BROTLI_PARAM_QUALITY,
        BrotliEncoderParameter::BROTLI_PARAM_LGWIN,
        BrotliEncoderParameter::BROTLI_PARAM_SIZE_HINT,
        BrotliEncoderParameter::BROTLI_PARAM_CATABLE,
        BrotliEncoderParameter::BROTLI_PARAM_MAGIC_NUMBER,
        BrotliEncoderParameter::BROTLI_PARAM_Q9_5,
    ];
    let values = [3u32, 16, 0, 0, 0, 0];
    let mut encoded_size = BrotliEncoderMaxCompressedSizeMulti(0, 4);
    let mut encoded_backing = [0u8; 145];
    let encoded = &mut encoded_backing[..encoded_size];
    let ret = unsafe {
        BrotliEncoderCompressMulti(
            params.len(),
            params[..].as_ptr(),
            values[..].as_ptr(),
            0,
            core::ptr::null(),
            &mut encoded_size,
            encoded.as_mut_ptr(),
            4,
            None,
            None,
            core::ptr::null_mut(),
        )
    };
    assert_eq!(ret, 1);
    let mut rt_size = 256;
    let mut rt_buffer = [0u8; 256];
    assert_ne!(encoded_size, 0);
    let ret2 = unsafe {
        super::super::decompressor::CBrotliDecoderDecompress(
            encoded_size,
            encoded.as_ptr(),
            &mut rt_size,
            rt_buffer.as_mut_ptr(),
        )
    };
    match ret2 {
    super::super::decompressor::ffi::interface::BrotliDecoderResult::BROTLI_DECODER_RESULT_SUCCESS => {
    },
    _ => panic!("{}", ret2 as i32),
  }
    assert_eq!(rt_size, 0);
}

#[test]
fn test_compress_empty_multi_raw_one_thread() {
    let input = [];
    let params = [
        BrotliEncoderParameter::BROTLI_PARAM_QUALITY,
        BrotliEncoderParameter::BROTLI_PARAM_Q9_5,
        BrotliEncoderParameter::BROTLI_PARAM_CATABLE,
        BrotliEncoderParameter::BROTLI_PARAM_APPENDABLE,
        BrotliEncoderParameter::BROTLI_PARAM_MAGIC_NUMBER,
    ];
    let values = [10u32, 1, 1, 1, 1];
    let mut encoded_size = BrotliEncoderMaxCompressedSizeMulti(input.len(), 1);
    let mut encoded_backing = [0u8; 25];
    let encoded = &mut encoded_backing[..encoded_size];
    assert_eq!(params.len(), 5);
    assert_eq!(encoded_size, 25);
    let ret = unsafe {
        BrotliEncoderCompressMulti(
            params.len(),
            params[..].as_ptr(),
            values[..].as_ptr(),
            input.len(),
            input[..].as_ptr(),
            &mut encoded_size,
            encoded.as_mut_ptr(),
            1,
            None,
            None,
            core::ptr::null_mut(),
        )
    };
    assert_eq!(ret, 1);
    let mut rt_size = 256;
    let mut rt_buffer = [0u8; 256];
    assert_ne!(encoded_size, 0);
    let ret2 = unsafe {
        super::super::decompressor::CBrotliDecoderDecompress(
            encoded_size,
            encoded.as_ptr(),
            &mut rt_size,
            rt_buffer.as_mut_ptr(),
        )
    };
    match ret2 {
    super::super::decompressor::ffi::interface::BrotliDecoderResult::BROTLI_DECODER_RESULT_SUCCESS => {
    },
    _ => panic!("{}", ret2 as i32),
  }
    assert_eq!(rt_size, input.len());
    assert_eq!(&rt_buffer[..rt_size], &input[..]);
}

#[test]
fn test_compress_empty_multi_catable() {
    let input = [];
    let params = [
        BrotliEncoderParameter::BROTLI_PARAM_QUALITY,
        BrotliEncoderParameter::BROTLI_PARAM_LGWIN,
        BrotliEncoderParameter::BROTLI_PARAM_SIZE_HINT,
        BrotliEncoderParameter::BROTLI_PARAM_CATABLE,
        BrotliEncoderParameter::BROTLI_PARAM_MAGIC_NUMBER,
        BrotliEncoderParameter::BROTLI_PARAM_Q9_5,
    ];
    let values = [3u32, 16, 0, 1, 1, 0];
    let mut encoded_size = BrotliEncoderMaxCompressedSizeMulti(input.len(), 4);
    let mut encoded_backing = [0u8; 145];
    let encoded = &mut encoded_backing[..encoded_size];
    let ret = unsafe {
        BrotliEncoderCompressMulti(
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
            core::ptr::null_mut(),
        )
    };
    assert_eq!(ret, 1);
    let mut rt_size = 256;
    let mut rt_buffer = [0u8; 256];
    assert_ne!(encoded_size, 0);
    let ret2 = unsafe {
        super::super::decompressor::CBrotliDecoderDecompress(
            encoded_size,
            encoded.as_ptr(),
            &mut rt_size,
            rt_buffer.as_mut_ptr(),
        )
    };
    match ret2 {
    super::super::decompressor::ffi::interface::BrotliDecoderResult::BROTLI_DECODER_RESULT_SUCCESS => {
    },
    _ => panic!("{:?}", ret2 as i32),
  }
    assert_eq!(rt_size, input.len());
    assert_eq!(&rt_buffer[..rt_size], &input[..]);
}

// Number of blocks a C allocator pair has handed out and not yet taken back.
struct LiveBlocks(usize);

extern "C" fn tracking_alloc(opaque: *mut c_void, size: usize) -> *mut c_void {
    // The size goes in a header so tracking_free can rebuild the layout.
    let layout = Layout::from_size_align(size + 16, 16).unwrap();
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            return core::ptr::null_mut();
        }
        *(ptr as *mut usize) = size;
        (*(opaque as *mut LiveBlocks)).0 += 1;
        ptr.add(16) as *mut c_void
    }
}

extern "C" fn tracking_free(opaque: *mut c_void, address: *mut c_void) {
    if address.is_null() {
        return;
    }
    unsafe {
        let ptr = (address as *mut u8).sub(16);
        let size = *(ptr as *const usize);
        dealloc(ptr, Layout::from_size_align(size + 16, 16).unwrap());
        (*(opaque as *mut LiveBlocks)).0 -= 1;
    }
}

// With one thread, BrotliEncoderCompressMulti runs the encoder on the calling
// thread with a state of its own; every block that state took from the
// caller's allocator must be back before the call returns.
#[test]
fn test_compress_multi_one_thread_frees_encoder_state() {
    let input = b"the quick brown fox jumps over the lazy dog, the quick brown fox";
    let params = [
        BrotliEncoderParameter::BROTLI_PARAM_QUALITY,
        BrotliEncoderParameter::BROTLI_PARAM_LGWIN,
        BrotliEncoderParameter::BROTLI_PARAM_SIZE_HINT,
    ];
    let values = [11u32, 16, input.len() as u32];
    let mut encoded_size = BrotliEncoderMaxCompressedSizeMulti(input.len(), 1);
    let mut encoded = vec![0u8; encoded_size];
    let mut live = LiveBlocks(0);
    let mut opaque = &mut live as *mut LiveBlocks as *mut c_void;
    let ret = unsafe {
        BrotliEncoderCompressMulti(
            params.len(),
            params[..].as_ptr(),
            values[..].as_ptr(),
            input.len(),
            input[..].as_ptr(),
            &mut encoded_size,
            encoded.as_mut_ptr(),
            1,
            Some(tracking_alloc),
            Some(tracking_free),
            &mut opaque,
        )
    };
    assert_eq!(ret, 1);
    assert_ne!(encoded_size, 0);
    assert_eq!(live.0, 0);
}
