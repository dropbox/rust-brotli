#![cfg(test)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
extern crate core;
extern crate brotli_decompressor;
use super::brotli::BrotliResult;
use super::brotli::BrotliState;
use super::brotli::enc::BrotliEncoderParams;
use super::brotli::concat::BroCatli;
use brotli::BrotliDecompressStream;
use std::io::{Read, Write};
use super::integration_tests::{Buffer, UnlimitedBuffer};
static RANDOM_THEN_UNICODE : &'static [u8] = include_bytes!("../../testdata/random_then_unicode");
static ALICE: &'static[u8]  = include_bytes!("../../testdata/alice29.txt");
static UKKONOOA: &'static[u8]  = include_bytes!("../../testdata/ukkonooa");
static ASYOULIKE: &'static[u8] = include_bytes!("../../testdata/asyoulik.txt");
static BACKWARD65536: &'static [u8] = include_bytes!("../../testdata/backward65536");
static DICTWORD: &'static [u8] = include_bytes!("../../testdata/ends_with_truncated_dictionary");
static RANDOM10K: &'static [u8] = include_bytes!("../../testdata/random_org_10k.bin");
static RANDOMTHENUNICODE: &'static [u8] = include_bytes!("../../testdata/random_then_unicode");
static QUICKFOX: &'static [u8] = include_bytes!("../../testdata/quickfox_repeated");
static EMPTY: &'static [u8] = &[];
fn concat(files:&mut [UnlimitedBuffer],
          brotli_files:&mut [UnlimitedBuffer],
          window_override:Option<u8>) {
    
}

#[cfg(debug_assertions)]
fn light_debug_test(params: &mut BrotliEncoderParams) {
    params.quality = 5;
}

#[cfg(not(debug_assertions))]
fn light_debug_test(params: &mut BrotliEncoderParams) {
}

#[test]
fn test_concat() {
    let mut files = [
        UnlimitedBuffer::new(ALICE),
        UnlimitedBuffer::new(RANDOM_THEN_UNICODE),
        UnlimitedBuffer::new(UKKONOOA),
        UnlimitedBuffer::new(ASYOULIKE),
        UnlimitedBuffer::new(BACKWARD65536),
        UnlimitedBuffer::new(DICTWORD),
        UnlimitedBuffer::new(RANDOM10K),
        UnlimitedBuffer::new(RANDOMTHENUNICODE),
        UnlimitedBuffer::new(QUICKFOX),
        UnlimitedBuffer::new(EMPTY),
    ];
    let mut params0 = BrotliEncoderParams::default();
    light_debug_test(&mut params0);
    let mut params1 = params0.clone();
    params1.quality = 9;
    params1.q9_5 = true;
    params1.magic_number = true;
    let mut params2 = params0.clone();
    params2.quality = 9;
    let mut params3 = params0.clone();
    params3.quality = 9;
    params3.lgwin = 16;
    params3.magic_number = true;
    let mut params4 = params0.clone();
    params4.quality = 8;
    params4.lgwin = 14;
    let mut params4 = params0.clone();
    params4.quality = 1;
    params4.lgwin = 10;
    let mut params5 = params0.clone();
    params5.quality = 0;
    params5.lgwin = 10;
    params5.magic_number = true;
    params0.lgwin = 30;
    params0.large_window = true;
    
    let mut options = [
        params0,
        params1,
        params2,
        params3,
        params4,
        params5,
        ];
    for option in options.iter_mut() {
        let mut ufiles = [
            UnlimitedBuffer::new(&[]),
/*            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),*/
        ];
        let mut first = true;
        for (src, dst) in files.iter_mut().zip(ufiles.iter_mut()) {
            if first {
                option.appendable = true;
            } else {
                option.appendable = false;
                option.catable = true;
            }
            super::compress(src, dst, 4096, option).unwrap();
        }
    }
            
        
}
