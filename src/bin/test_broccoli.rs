#![cfg(test)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
use std::fs::File;
extern crate core;
extern crate brotli_decompressor;
use brotli_decompressor::{CustomRead, CustomWrite};
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
          window_override:Option<u8>,
          bs: usize) {
  let mut obuffer = vec![0u8; bs];
  let mut ibuffer = vec![0u8; bs];
  let mut ooffset = 0usize;
  let mut ioffset = 0usize;
  let mut uboutput = UnlimitedBuffer::new(&[]);
  {
    let mut output = super::IoWriterWrapper(&mut uboutput);
    let mut bro_cat_li = match window_override {
      Some(ws) => BroCatli::new_with_window_size(ws),
      None => BroCatli::new(),
    };
    for (index, brotli) in brotli_files.iter_mut().enumerate() {
        {
            let dfn = index.to_string();
            
            let mut debug_file = File::create("/tmp/".to_owned() + &dfn).unwrap();
            debug_file.write_all(brotli.data());
        }

        bro_cat_li.new_brotli_file();
        {
            let mut input = super::IoReaderWrapper(brotli);
            loop {
                ioffset = 0;
                match input.read(&mut ibuffer[..]) {
                    Err(e) => panic!(e),
                    Ok(cur_read) => {
                        if cur_read == 0 {
                            break;
                        }
                        loop {
                            match bro_cat_li.stream(&ibuffer[..cur_read], &mut ioffset,
                                                    &mut obuffer[..], &mut ooffset) {
                                BrotliResult::ResultFailure => {
                                    panic!(index);
                },
                BrotliResult::NeedsMoreOutput => {
                  match output.write(&obuffer[..ooffset]) {
                    Err(why) => panic!("couldn't write: {:}", why),
                    Ok(count) => {assert_eq!(count, ooffset);},
                  }
                  ooffset = 0;
                },
                BrotliResult::NeedsMoreInput => {
                  break;
                },
                BrotliResult::ResultSuccess => {
                  panic!("Unexpected state: Success when streaming before finish");
                }
              }
            }
          }
        }
      }
        }
      brotli.reset_read();
    }
    loop {
      match bro_cat_li.finish(&mut obuffer[..], &mut ooffset) {
        BrotliResult::NeedsMoreOutput => {
          match output.write(&obuffer[..ooffset]) {
            Err(why) => panic!("couldn't write\n{:}", why),
            Ok(count) => {assert_eq!(count, ooffset);},
          }
          ooffset = 0;
        },
        BrotliResult::NeedsMoreInput => {
          panic!("Unexpected EOF");
        },
        BrotliResult::ResultSuccess => {
          if ooffset != 0 {
            match output.write(&obuffer[..ooffset]) {
              Err(why) => panic!("couldn't write\n{:}", why),
              Ok(count) => {assert_eq!(count, ooffset);},
            }
          }
          break;
        }
        BrotliResult::ResultFailure => {
          panic!("Failed to terminate concatenation")
        }
      }
    }
  }
    let mut rt = UnlimitedBuffer::new(&[]);
    {
        let mut debug_file = File::create("/tmp/concatted".to_owned() + &files.len().to_string()+&".br".to_owned()).unwrap();
        debug_file.write_all(uboutput.data());
    }
  match super::decompress(&mut uboutput, &mut rt, 65536) {
    Ok(_) => {},
    Err(e) => panic!("Error {:?}", e),
  }
  let mut offset = 0;
  for file in files {
    assert_eq!(&rt.data()[offset..offset+file.data().len()],
               file.data());
    offset += file.data().len();
  }
  assert_eq!(offset, rt.data().len());
}

fn concat_many_subsets(files:&mut [UnlimitedBuffer],
                       brotli_files:&mut [UnlimitedBuffer],
                       window_override:Option<u8>) {
  let test_plans:[(usize,usize);4] = [(brotli_files.len(), 4096 * 1024), (4, 1), (3, 3),  (2, 4096)];
  for plan_bs in test_plans.iter() {
    let files_len = files.len();
    for index in 0..(brotli_files.len() - core::cmp::min((plan_bs.0 - 1), files_len)) {
      let file_subset = &mut files[index..core::cmp::min((index+plan_bs.0), files_len)];
      let brotli_subset = &mut brotli_files[index..core::cmp::min((index+plan_bs.0), files_len)];
      concat(file_subset, brotli_subset, window_override, plan_bs.1);
    }
  }
}

#[cfg(debug_assertions)]
fn light_debug_test(params: &mut BrotliEncoderParams) {
    params.quality = 5;
}

#[cfg(not(debug_assertions))]
fn light_debug_test(params: &mut BrotliEncoderParams) {
}

#[test]
#[should_panic]
fn test_appendonly_twice_fails() {
  let mut files = [
    UnlimitedBuffer::new(UKKONOOA),
    UnlimitedBuffer::new(QUICKFOX),
  ];
  let mut ufiles = [
    UnlimitedBuffer::new(&[]),
    UnlimitedBuffer::new(&[]),
  ];
  for (src, dst) in files.iter_mut().zip(ufiles.iter_mut()) {
    let mut params0 = BrotliEncoderParams::default();
    params0.appendable = true;
    super::compress(src, dst, 4096, &params0).unwrap();
  }
  concat(&mut files[..], &mut ufiles[..], None, 2);
}

#[test]
fn test_append_then_cat_works() {
  let mut files = [
    UnlimitedBuffer::new(UKKONOOA),
    UnlimitedBuffer::new(QUICKFOX),
  ];
  let mut ufiles = [
    UnlimitedBuffer::new(&[]),
    UnlimitedBuffer::new(&[]),
  ];
  let mut first = true;
  for (src, dst) in files.iter_mut().zip(ufiles.iter_mut()) {
    let mut params0 = BrotliEncoderParams::default();
    params0.appendable = first;
    params0.catable = !first;
    super::compress(src, dst, 4096, &params0).unwrap();
    first = false;
  }
  concat(&mut files[..], &mut ufiles[..], None, 2);
}

#[test]
fn test_concat() {
    let mut files = [
      /*  UnlimitedBuffer::new(ALICE),
      UnlimitedBuffer::new(RANDOM_THEN_UNICODE),*/
      
        UnlimitedBuffer::new(UKKONOOA),
        //UnlimitedBuffer::new(ASYOULIKE),
        //UnlimitedBuffer::new(BACKWARD65536),
        UnlimitedBuffer::new(DICTWORD),/*
        UnlimitedBuffer::new(RANDOM10K),
        UnlimitedBuffer::new(RANDOMTHENUNICODE),
        UnlimitedBuffer::new(QUICKFOX),
        UnlimitedBuffer::new(EMPTY),*/
    ];
    let mut params0 = BrotliEncoderParams::default();
    light_debug_test(&mut params0);
    let mut params1 = params0.clone();
    params1.quality = 9;
    params1.q9_5 = true;
    params1.lgwin=22;
    params1.magic_number = true;
    let mut params2 = params0.clone();
    params2.quality = 9;
    params2.lgwin=19;
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
    params0.lgwin = 23;
    //params0.large_window = true; (FIXME: turn this challenge back on)
    
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
            UnlimitedBuffer::new(&[]),
         /*   UnlimitedBuffer::new(&[]),
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
            src.reset_read();
            first = false;
        }
        concat_many_subsets(&mut files[..], &mut ufiles[..], None);
    }
    let mut ufiles = [
      UnlimitedBuffer::new(&[]),
      UnlimitedBuffer::new(&[]),/*
      UnlimitedBuffer::new(&[]),
      UnlimitedBuffer::new(&[]),
      UnlimitedBuffer::new(&[]),
      UnlimitedBuffer::new(&[]),
      UnlimitedBuffer::new(&[]),
      UnlimitedBuffer::new(&[]),
      UnlimitedBuffer::new(&[]),
      UnlimitedBuffer::new(&[]),*/
    ];
    let options_len = options.len();
    for (index, (src, dst)) in files.iter_mut().zip(ufiles.iter_mut()).enumerate() {
      options[core::cmp::min(index, options_len - 1)].catable = true;
      options[core::cmp::min(index, options_len - 1)].appendable = false;
      options[core::cmp::min(index, options_len - 1)].quality = core::cmp::max(
        2, options[core::cmp::min(index, options_len - 1)].quality);
      // ^^^ there's an artificial limitation of using 18 as the minimum window size for quality 0,1
      // since this test depends on different window sizes for each stream, exclude q={0,1}  
      super::compress(src, dst, 4096, &options[core::cmp::min(index, options_len - 1)]).unwrap();
      src.reset_read();
    }
    //concat_many_subsets(&mut files[..], &mut ufiles[..], None);
    //concat_many_subsets(&mut files[..], &mut ufiles[..], Some(28));
}
