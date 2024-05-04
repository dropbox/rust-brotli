#![cfg(test)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
extern crate brotli_decompressor;
extern crate core;
use super::brotli::concat::{BroCatli, BroCatliResult};
use super::brotli::enc::BrotliEncoderParams;
use super::integration_tests::UnlimitedBuffer;
use std::io::{Read, Write};
static RANDOM_THEN_UNICODE: &[u8] = include_bytes!("../../testdata/random_then_unicode");
static ALICE: &[u8] = include_bytes!("../../testdata/alice29.txt");
use super::Rebox;

#[test]
fn test_custom_dict() {
    let mut raw = UnlimitedBuffer::new(ALICE);
    let mut params = BrotliEncoderParams::default();
    params.quality = 10;
    let mut br = UnlimitedBuffer::new(&[]);
    let mut rt = UnlimitedBuffer::new(&[]);
    let dict = &ALICE[12515..23411];
    super::compress(&mut raw, &mut br, 4096, &params, dict, 1).unwrap();
    raw.reset_read();
    let mut vec = Vec::<u8>::new();
    vec.extend(dict);
    super::decompress(&mut br, &mut rt, 4096, Rebox::from(vec)).unwrap();
    assert_eq!(rt.data(), raw.data());
    if br.data().len() != 43654 {
        assert_eq!(br.data().len(), 43636);
    }
}

#[test]
fn test_custom_wrong_dict_fails() {
    let mut raw = UnlimitedBuffer::new(ALICE);
    let mut params = BrotliEncoderParams::default();
    params.quality = 10;
    let mut br = UnlimitedBuffer::new(&[]);
    let mut rt = UnlimitedBuffer::new(&[]);
    let dict = &ALICE[12515..19515];
    super::compress(&mut raw, &mut br, 4096, &params, dict, 1).unwrap();
    raw.reset_read();
    let mut vec = Vec::<u8>::new();
    vec.extend(&dict[1..]); // slightly offset dictionary to be wrong, and ensure the dict was being used above
    match super::decompress(&mut br, &mut rt, 4096, Rebox::from(vec)) {
        Ok(_) => panic!("Decompression should have failed"),
        Err(_) => {}
    }
    if rt.data() == raw.data() {
        panic!("they should be unequal");
    }
}

#[test]
fn test_custom_wrong_dict_fails_but_doesnt_disrupt_compression_strategy() {
    let mut raw = UnlimitedBuffer::new(ALICE);
    let mut params = BrotliEncoderParams::default();
    params.quality = 6;
    let mut br = UnlimitedBuffer::new(&[]);
    let mut rt = UnlimitedBuffer::new(&[]);
    let dict = &ALICE[12515..19515];
    super::compress(&mut raw, &mut br, 4096, &params, dict, 1).unwrap();
    raw.reset_read();
    let mut vec = Vec::<u8>::new();
    vec.extend(&dict[1..]); // slightly offset dictionary to be wrong, and ensure the dict was being used above
    super::decompress(&mut br, &mut rt, 4096, Rebox::from(vec)).unwrap();
    if rt.data() == raw.data() {
        panic!("they should be unequal");
    }
}

#[test]
fn test_custom_dict_for_multithreading() {
    let mut raws = [
        UnlimitedBuffer::new(&ALICE[..ALICE.len() / 3]),
        UnlimitedBuffer::new(&ALICE[ALICE.len() / 3..2 * ALICE.len() / 3]),
        UnlimitedBuffer::new(&ALICE[2 * ALICE.len() / 3..]),
    ];
    let mut params = BrotliEncoderParams::default();
    params.quality = 10;
    params.appendable = true;
    let mut brs = [
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
    ];
    let mut rts = [
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
    ];
    let dicts = [
        &ALICE[..0],
        &ALICE[..ALICE.len() / 3],
        &ALICE[..2 * ALICE.len() / 3],
    ];
    for (raw, (br, (rt, dict))) in raws
        .iter_mut()
        .zip(brs.iter_mut().zip(rts.iter_mut().zip(dicts.iter())))
    {
        super::compress(raw, br, 4096, &params, dict, 1).unwrap();
        raw.reset_read();
        let mut vec = Vec::<u8>::new();
        vec.extend(*dict);
        super::decompress(br, rt, 4096, Rebox::from(vec)).unwrap();
        assert_eq!(rt.data(), raw.data());
        params.catable = true;
        params.use_dictionary = false;
    }
    let mut bro_cat_li = BroCatli::new();
    let mut output = UnlimitedBuffer::new(&[]);
    let mut ibuffer = [0u8; 1];
    let mut obuffer = [0u8; 1];
    let mut ooffset = 0usize;
    for brotli in brs.iter_mut() {
        brotli.reset_read();
        bro_cat_li.new_brotli_file();
        let input = brotli;
        loop {
            let mut ioffset = 0usize;
            match input.read(&mut ibuffer[..]) {
                Err(e) => panic!("{:?}", e),
                Ok(cur_read) => {
                    if cur_read == 0 {
                        break;
                    }
                    loop {
                        match bro_cat_li.stream(
                            &ibuffer[..cur_read],
                            &mut ioffset,
                            &mut obuffer[..],
                            &mut ooffset,
                        ) {
                            BroCatliResult::NeedsMoreOutput => {
                                match output.write(&obuffer[..ooffset]) {
                                    Err(why) => panic!("couldn't write: {:}", why),
                                    Ok(count) => {
                                        assert_eq!(count, ooffset);
                                    }
                                }
                                ooffset = 0;
                            }
                            BroCatliResult::NeedsMoreInput => {
                                break;
                            }
                            BroCatliResult::Success => {
                                panic!("Unexpected state: Success when streaming before finish");
                            }
                            failure => {
                                panic!("{:?}", failure);
                            }
                        }
                    }
                }
            }
        }
    }
    loop {
        match bro_cat_li.finish(&mut obuffer[..], &mut ooffset) {
            BroCatliResult::NeedsMoreOutput => {
                match output.write(&obuffer[..ooffset]) {
                    Err(why) => panic!("couldn't write\n{:}", why),
                    Ok(count) => {
                        assert_eq!(count, ooffset);
                    }
                }
                ooffset = 0;
            }
            BroCatliResult::NeedsMoreInput => {
                panic!("Unexpected EOF");
            }
            BroCatliResult::Success => {
                if ooffset != 0 {
                    match output.write(&obuffer[..ooffset]) {
                        Err(why) => panic!("couldn't write\n{:}", why),
                        Ok(count) => {
                            assert_eq!(count, ooffset);
                        }
                    }
                }
                break;
            }
            failure => {
                panic!("{}", failure as i32)
            }
        }
    }
    let mut rt = UnlimitedBuffer::new(&[]);
    output.reset_read();
    super::decompress(&mut output, &mut rt, 4096, Rebox::default()).unwrap();
    assert_eq!(rt.data(), ALICE);
    // without setting std flag: approximation make it 4 bytes bigger
    if output.data().len() != 48568 {
        assert_eq!(output.data().len(), 48563); // as opposed to 46487 with standard settings
    }
}
