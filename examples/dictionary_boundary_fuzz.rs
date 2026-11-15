extern crate brotli;

// Run: cargo run --release --example dictionary_boundary_fuzz -- cases=10000 seed=0
// Reproduce issue #280: cases=1 seed=50 quality=9 shape=0 dict=16 len=4096 catable=0

#[cfg(feature = "std")]
mod harness {
    use brotli::enc::encode::{
        BrotliEncoderOperation, BrotliEncoderParameter, BrotliEncoderStateStruct,
    };
    use brotli::enc::{BrotliEncoderMaxCompressedSize, StandardAlloc};
    use brotli::{Allocator, Decompressor, SliceWrapperMut};
    use std::io::Read;

    const MULTIPLIER: u64 = 0x9E37_79B9_7F4A_7C15;

    pub fn corpus(seed: u64, dict_len: usize, data_len: usize, shape: u8) -> (Vec<u8>, Vec<u8>) {
        let mut x = seed.wrapping_mul(MULTIPLIER) | 1;
        let total = dict_len + data_len;
        let mut all = Vec::with_capacity(total);
        for i in 0..total {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            let v = match shape % 6 {
                0 => b'a' + ((x >> 32) % 3) as u8,
                1 => b'a' + ((x >> 32) % 2) as u8,
                2 => b'a' + (i.wrapping_add(seed as usize) % 4) as u8,
                3 => (x >> 32) as u8,
                4 => b'a' + ((i / 7).wrapping_add(seed as usize) % 3) as u8,
                _ => b'a' + ((i / 3).wrapping_add((x >> 32) as usize) % 3) as u8,
            };
            all.push(v);
        }
        (all[..dict_len].to_vec(), all[dict_len..].to_vec())
    }

    fn decompress_with_dict(compressed: &[u8], dictionary: &[u8]) -> Result<Vec<u8>, String> {
        let mut alloc = StandardAlloc::default();
        let mut dict_mem =
            <StandardAlloc as Allocator<u8>>::alloc_cell(&mut alloc, dictionary.len());
        dict_mem.slice_mut().copy_from_slice(dictionary);
        let mut decoder = Decompressor::new_with_custom_dict(compressed, 4096, dict_mem);
        let mut plain = Vec::new();
        decoder
            .read_to_end(&mut plain)
            .map_err(|err| format!("decompress: {}", err))?;
        Ok(plain)
    }

    fn compress_dict(
        dictionary: &[u8],
        input: &[u8],
        quality: u32,
        catable: bool,
    ) -> Result<Vec<u8>, String> {
        let mut state = BrotliEncoderStateStruct::new(StandardAlloc::default());
        let params = [
            (BrotliEncoderParameter::BROTLI_PARAM_QUALITY, quality),
            (BrotliEncoderParameter::BROTLI_PARAM_LGWIN, 22),
            (BrotliEncoderParameter::BROTLI_PARAM_CATABLE, catable as u32),
            (
                BrotliEncoderParameter::BROTLI_PARAM_APPENDABLE,
                catable as u32,
            ),
        ];
        for &(key, value) in &params {
            if !state.set_parameter(key, value) {
                return Err(format!("set_parameter failed for q={}", quality));
            }
        }
        state.set_custom_dictionary(dictionary.len(), dictionary);
        let mut output = vec![0u8; BrotliEncoderMaxCompressedSize(input.len()) + 1024];
        let (mut in_offset, mut out_offset) = (0usize, 0usize);
        for _ in 0..16 {
            let mut avail_in = input.len() - in_offset;
            let mut avail_out = output.len() - out_offset;
            let mut total_out = Some(out_offset);
            let ok = state.compress_stream(
                BrotliEncoderOperation::BROTLI_OPERATION_FINISH,
                &mut avail_in,
                input,
                &mut in_offset,
                &mut avail_out,
                &mut output,
                &mut out_offset,
                &mut total_out,
                &mut |_, _, _, _| (),
            );
            if !ok {
                return Err(format!("compress_stream returned false q={}", quality));
            }
            if state.is_finished() {
                output.truncate(out_offset);
                return Ok(output);
            }
            if avail_out == 0 {
                return Err("output buffer exhausted".to_owned());
            }
        }
        Err("compression failed to finish in 16 iterations".to_owned())
    }

    pub fn check_dict(
        dictionary: &[u8],
        input: &[u8],
        quality: u32,
        catable: bool,
    ) -> Result<usize, String> {
        let compressed = compress_dict(dictionary, input, quality, catable)?;
        let plain = decompress_with_dict(&compressed, dictionary)?;
        if plain != input {
            return Err(format!(
                "mismatch: decoded {} bytes, expected {}",
                plain.len(),
                input.len()
            ));
        }
        Ok(compressed.len())
    }
}

#[cfg(feature = "std")]
use std::env;
#[cfg(feature = "std")]
use std::panic::{catch_unwind, AssertUnwindSafe};

#[cfg(feature = "std")]
fn number(arg: &str, default: u64) -> u64 {
    env::args()
        .find_map(|a| {
            let (k, v) = a.split_once('=')?;
            if k == arg {
                v.parse::<u64>().ok()
            } else {
                None
            }
        })
        .unwrap_or(default)
}

#[cfg(feature = "std")]
fn main() {
    let cases = number("cases", 100) as usize;
    let seed0 = number("seed", 0);
    let quality_arg = number("quality", 99);
    let shape_arg = number("shape", 99);
    let dict_arg = number("dict", 0);
    let len_arg = number("len", 0);
    let catable_arg = number("catable", 99);
    let mut failures = 0usize;
    for i in 0..cases {
        let seed = seed0.wrapping_add(i as u64);
        let dict_len = if dict_arg != 0 {
            dict_arg as usize
        } else {
            [2usize, 4, 8, 16, 31, 63, 127, 1024][(seed % 8) as usize]
        };
        let data_len = if len_arg != 0 {
            len_arg as usize
        } else {
            [128usize, 1024, 4096, 8192][((seed / 8) % 4) as usize]
        };
        let q = if quality_arg <= 11 {
            quality_arg as u32
        } else {
            (seed % 12) as u32
        };
        let shape = if shape_arg <= 5 {
            shape_arg as u8
        } else {
            ((seed / 3) % 6) as u8
        };
        let catable = if catable_arg <= 1 {
            catable_arg == 1
        } else {
            seed % 2 == 0
        };
        let (dictionary, input) = harness::corpus(seed, dict_len, data_len, shape);
        let result = catch_unwind(AssertUnwindSafe(|| {
            harness::check_dict(&dictionary, &input, q, catable)
        }));
        match result {
            Ok(Ok(_)) => {}
            Ok(Err(message)) => {
                failures += 1;
                eprintln!(
                    "FAIL seed={} dict={} len={} shape={} q={} catable={}: {}",
                    seed, dict_len, data_len, shape, q, catable, message
                );
            }
            Err(_) => {
                failures += 1;
                eprintln!(
                    "PANIC seed={} dict={} len={} shape={} q={} catable={}",
                    seed, dict_len, data_len, shape, q, catable
                );
            }
        }
        if i % 100 == 0 {
            eprintln!("checked {} / {}; failures={}", i + 1, cases, failures);
        }
    }
    println!("{} cases, {} failures", cases, failures);
    if failures != 0 {
        std::process::exit(1);
    }
}

#[cfg(not(feature = "std"))]
fn main() {
    panic!("dictionary_boundary_fuzz requires the std feature");
}
