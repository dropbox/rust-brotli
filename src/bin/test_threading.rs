#![cfg(test)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
extern crate core;
extern crate brotli_decompressor;
use super::new_brotli_heap_alloc;
use brotli_decompressor::{SliceWrapperMut, SliceWrapper};
use super::brotli::enc::{BrotliEncoderParams, BrotliEncoderMaxCompressedSizeMulti, compress_multi, compress_multi_no_threadpool};
use brotli::enc::threading::{SendAlloc,Owned};

use super::integration_tests::UnlimitedBuffer;
static RANDOM_THEN_UNICODE : &'static [u8] = include_bytes!("../../testdata/random_then_unicode");
static ALICE: &'static[u8]  = include_bytes!("../../testdata/alice29.txt");
use super::Rebox;

struct SliceRef<'a> (&'a [u8]);
impl<'a> SliceWrapper<u8> for SliceRef<'a> {
    fn slice(&self) -> &[u8] { self.0 }
}

fn multi_threaded_split_compression_test(num_threads: usize, quality: i32, catable: bool, expected_size: usize) {
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
        &mut Owned::new(SliceRef(input_data)),
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
fn multi_threaded_split_compression_test_1() {
    multi_threaded_split_compression_test(1, 3, false, 155808)
}
#[test]
fn multi_threaded_split_compression_test_2() {
    multi_threaded_split_compression_test(2, 4, false, 151857)
}
#[test]
fn multi_threaded_split_compression_test_3() {
    multi_threaded_split_compression_test(3, 5, false, 144325)
}
#[test]
fn multi_threaded_split_compression_test_4() {
    multi_threaded_split_compression_test(4, 10, true, 136812)
}
#[test]
fn multi_threaded_split_compression_test_5() {
    multi_threaded_split_compression_test(5, 9, false, 139125)
}



fn thread_spawn_per_job_split_compression_test(num_threads: usize, quality: i32, catable: bool, expected_size: usize) {
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
    let res = compress_multi_no_threadpool(
        &params,
        &mut Owned::new(SliceRef(input_data)),
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
fn thread_spawn_per_job_split_compression_test_1() {
    thread_spawn_per_job_split_compression_test(1, 3, false, 155808)
}
#[test]
fn thread_spawn_per_job_split_compression_test_3() {
    thread_spawn_per_job_split_compression_test(3, 5, false, 144325)
}
