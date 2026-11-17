#![cfg(test)]
#![cfg(feature = "std")]

use core;

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

static WORKER_STARTED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
static CALL_RETURNED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
static LATE_WORKER_ALLOCS: core::sync::atomic::AtomicUsize =
    core::sync::atomic::AtomicUsize::new(0);
static WORKER_OPAQUE: u8 = 0;

// A null opaque is the calling thread's allocator: it fails, but only once the
// pool worker is busy with its part, so the call panics while that part is running.
extern "C" fn worker_tracking_alloc(opaque: *mut c_void, size: usize) -> *mut c_void {
    use core::sync::atomic::Ordering::SeqCst;
    if opaque.is_null() {
        while !WORKER_STARTED.load(SeqCst) {
            std::thread::yield_now();
        }
        return core::ptr::null_mut();
    }
    if CALL_RETURNED.load(SeqCst) {
        LATE_WORKER_ALLOCS.fetch_add(1, SeqCst);
    }
    if !WORKER_STARTED.swap(true, SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    let layout = std::alloc::Layout::from_size_align(size + 16, 16).unwrap();
    unsafe {
        let base = std::alloc::alloc(layout);
        assert!(!base.is_null());
        (base as *mut usize).write(size);
        base.add(16) as *mut c_void
    }
}

extern "C" fn worker_tracking_free(_opaque: *mut c_void, address: *mut c_void) {
    if address.is_null() {
        return;
    }
    unsafe {
        let base = (address as *mut u8).sub(16);
        let size = (base as *mut usize).read();
        std::alloc::dealloc(
            base,
            std::alloc::Layout::from_size_align(size + 16, 16).unwrap(),
        );
    }
}

// The calling thread compresses the last part itself. When that panics, the
// parts already queued on the pool still read the caller's input and use its
// allocator, so they must be finished before the call returns.
#[cfg(not(feature = "pass-through-ffi-panics"))]
#[test]
fn test_compress_workpool_panic_waits_for_pool_jobs() {
    use core::sync::atomic::Ordering::SeqCst;
    let input: std::vec::Vec<u8> = (0..65536u32).map(|i| (i * 7 + (i >> 5)) as u8).collect();
    let params = [BrotliEncoderParameter::BROTLI_PARAM_QUALITY];
    let values = [5u32];
    let mut encoded = std::vec![0u8; BrotliEncoderMaxCompressedSizeMulti(input.len(), 2)];
    let mut encoded_size = encoded.len();
    let mut opaques = [
        &WORKER_OPAQUE as *const u8 as *mut c_void,
        core::ptr::null_mut(),
    ];
    unsafe {
        let wp = BrotliEncoderCreateWorkPool(2, None, None, core::ptr::null_mut());
        let ret = BrotliEncoderCompressWorkPool(
            wp,
            params.len(),
            params.as_ptr(),
            values.as_ptr(),
            input.len(),
            input.as_ptr(),
            &mut encoded_size,
            encoded.as_mut_ptr(),
            2,
            Some(worker_tracking_alloc),
            Some(worker_tracking_free),
            opaques.as_mut_ptr(),
        );
        CALL_RETURNED.store(true, SeqCst);
        assert_eq!(ret, 0);
        BrotliEncoderDestroyWorkPool(wp);
    }
    assert_eq!(LATE_WORKER_ALLOCS.load(SeqCst), 0);
}
