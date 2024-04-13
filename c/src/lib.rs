#![no_std]
#![cfg_attr(not(feature = "std"), feature(lang_items))]
#[cfg(feature = "std")]
extern crate std;

pub extern crate brotli;
pub use brotli::ffi::compressor::*;
pub use brotli::ffi::decompressor::*;
pub use brotli::ffi::multicompress::*;
pub use brotli::*;
use core::ptr::null_mut;
#[cfg(feature = "std")]
unsafe fn std_only_functions() {
    let _ =
        brotli::ffi::decompressor::CBrotliDecoderDecompress(0, null_mut(), null_mut(), null_mut());
}
#[cfg(not(feature = "std"))]
unsafe fn std_only_functions() {}

#[no_mangle]
pub unsafe extern "C" fn instantiate_functions(must_be_null: *const u8) {
    if !must_be_null.is_null() {
        let _ = brotli::ffi::compressor::BrotliEncoderVersion();
        let _ = brotli::ffi::decompressor::CBrotliDecoderCreateInstance(None, None, null_mut());
        let _ = brotli::ffi::decompressor::CBrotliDecoderSetParameter(null_mut(), brotli::ffi::decompressor::ffi::interface::BrotliDecoderParameter::BROTLI_DECODER_PARAM_DISABLE_RING_BUFFER_REALLOCATION, 0);
        let _ = brotli::ffi::decompressor::CBrotliDecoderDecompressStream(
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
        );
        std_only_functions();
        let _ = brotli::ffi::decompressor::CBrotliDecoderMallocU8(null_mut(), 0);
        let _ = brotli::ffi::decompressor::CBrotliDecoderMallocUsize(null_mut(), 0);
        let _ = brotli::ffi::decompressor::CBrotliDecoderFreeU8(null_mut(), null_mut(), 0);
        let _ = brotli::ffi::decompressor::CBrotliDecoderFreeUsize(null_mut(), null_mut(), 0);
        let _ = brotli::ffi::decompressor::CBrotliDecoderDestroyInstance(null_mut());
        let _ = brotli::ffi::decompressor::CBrotliDecoderHasMoreOutput(null_mut());
        let _ = brotli::ffi::decompressor::CBrotliDecoderTakeOutput(null_mut(), null_mut());
        let _ = brotli::ffi::decompressor::CBrotliDecoderIsUsed(null_mut());
        let _ = brotli::ffi::decompressor::CBrotliDecoderIsFinished(null_mut());
        let _ = brotli::ffi::decompressor::CBrotliDecoderGetErrorCode(null_mut());
        let _ = brotli::ffi::decompressor::CBrotliDecoderGetErrorString(null_mut());
        let _ = brotli::ffi::decompressor::CBrotliDecoderErrorString(
            brotli::ffi::decompressor::ffi::BrotliDecoderErrorCode::BROTLI_DECODER_ERROR_UNREACHABLE);
        let _ = BrotliEncoderCreateInstance(None, None, null_mut());
        let _ = BrotliEncoderSetParameter(
            null_mut(),
            brotli::enc::encode::BrotliEncoderParameter::BROTLI_PARAM_MODE,
            0,
        );
        let _ = BrotliEncoderDestroyInstance(null_mut());
        let _ = BrotliEncoderIsFinished(null_mut());
        let _ = BrotliEncoderHasMoreOutput(null_mut());
        let _ = BrotliEncoderTakeOutput(null_mut(), null_mut());
        let _ = BrotliEncoderMaxCompressedSize(0);
        let _ = BrotliEncoderSetCustomDictionary(null_mut(), 0, null_mut());
        let _ = BrotliEncoderCompress(
            0,
            0,
            BrotliEncoderMode::BROTLI_MODE_GENERIC,
            0,
            null_mut(),
            null_mut(),
            null_mut(),
        );
        let _ = BrotliEncoderCompressStream(
            null_mut(),
            BrotliEncoderOperation::BROTLI_OPERATION_FINISH,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
        );
        let _ = BrotliEncoderMallocU8(null_mut(), 0);
        let _ = BrotliEncoderFreeU8(null_mut(), null_mut(), 0);
        let _ = BrotliEncoderMallocUsize(null_mut(), 0);
        let _ = BrotliEncoderFreeUsize(null_mut(), null_mut(), 0);
        let _ = BrotliEncoderMaxCompressedSizeMulti(0, 0);
        let _ = BrotliEncoderCompressMulti(
            0,
            null_mut(),
            null_mut(),
            0,
            null_mut(),
            null_mut(),
            null_mut(),
            0,
            None,
            None,
            null_mut(),
        );
        let _ = BrotliEncoderCreateWorkPool(0, None, None, null_mut());
        let _ = BrotliEncoderDestroyWorkPool(null_mut());
        let _ = BrotliEncoderCompressWorkPool(
            null_mut(),
            0,
            null_mut(),
            null_mut(),
            0,
            null_mut(),
            null_mut(),
            null_mut(),
            0,
            None,
            None,
            null_mut(),
        );
    }
}

#[cfg(not(feature = "std"))]
#[panic_handler]
extern "C" fn panic_impl(_: &::core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(not(feature = "std"))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
