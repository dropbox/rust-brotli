#![cfg(test)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
extern crate core;
extern crate brotli_decompressor;
use super::new_brotli_heap_alloc;
use brotli_decompressor::{SliceWrapperMut, SliceWrapper};
use super::brotli::enc::{BrotliEncoderParams, BrotliEncoderMaxCompressedSizeMulti, compress_multi};
use brotli::enc::threading::{SendAlloc,Owned};

use super::integration_tests::UnlimitedBuffer;
static RANDOM_THEN_UNICODE : &'static [u8] = include_bytes!("../../testdata/random_then_unicode");
static ALICE: &'static[u8]  = include_bytes!("../../testdata/alice29.txt");
use super::Rebox;


fn single_threaded_split_compression_test(num_threads: usize, quality: i32, catable: bool, expected_size: usize) {
    let mut params = BrotliEncoderParams::default();
    let input_data = &RANDOM_THEN_UNICODE[..];
    params.quality = quality;
    params.magic_number = true;
    if catable {
        params.catable = true;
    }
    let mut output = Rebox::from(vec![0u8;BrotliEncoderMaxCompressedSizeMulti(RANDOM_THEN_UNICODE.len(), num_threads)]);
    let mut alloc_per_thread = [
        SendAlloc::new(new_brotli_heap_alloc()),
        SendAlloc::new(new_brotli_heap_alloc()),
        SendAlloc::new(new_brotli_heap_alloc()),
        SendAlloc::new(new_brotli_heap_alloc()),
        SendAlloc::new(new_brotli_heap_alloc()),
        SendAlloc::new(new_brotli_heap_alloc()),
        SendAlloc::new(new_brotli_heap_alloc()),
        SendAlloc::new(new_brotli_heap_alloc()),
        SendAlloc::new(new_brotli_heap_alloc()),
        SendAlloc::new(new_brotli_heap_alloc()),
        SendAlloc::new(new_brotli_heap_alloc()),
        SendAlloc::new(new_brotli_heap_alloc()),
        SendAlloc::new(new_brotli_heap_alloc()),
        SendAlloc::new(new_brotli_heap_alloc()),
    ];
    if num_threads > alloc_per_thread.len() {
        panic!("Too many threads requested {} > {}", num_threads, alloc_per_thread.len());
    }
    let res = compress_multi(
        &params,
        &mut Owned::new(super::SliceRef(input_data)),
        output.slice_mut(),
        &mut alloc_per_thread[..num_threads],
    );
    let observed_size = res.unwrap();
    if observed_size > expected_size {
        assert_eq!(observed_size, expected_size);
    }
    let mut compressed_version = UnlimitedBuffer::new(&output.slice()[..observed_size]);
    let mut rt = UnlimitedBuffer::new(&[]);
    match super::decompress(&mut compressed_version, &mut rt, 65536, Rebox::default()) {
        Ok(_) => {}
        Err(e) => panic!("Error {:?}", e),
    }
    assert_eq!(rt.data(), input_data);
}
#[test]
fn single_threaded_split_compression_test_1() {
    single_threaded_split_compression_test(1, 3, false, 155808)
}
#[test]
fn single_threaded_split_compression_test_2() {
    single_threaded_split_compression_test(2, 4, false, 151857)
}
#[test]
fn single_threaded_split_compression_test_3() {
    single_threaded_split_compression_test(3, 5, false, 144325)
}
#[test]
fn single_threaded_split_compression_test_4() {
    single_threaded_split_compression_test(4, 10, true, 136812)
}
#[test]
fn single_threaded_split_compression_test_5() {
    single_threaded_split_compression_test(5, 9, false, 139125)
}
