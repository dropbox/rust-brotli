// Shared regression/fuzz harness. Built against rust-brotli 9.1.1 with std.
// No external random-number or testing dependencies are required.
#![allow(dead_code)]

use brotli::enc::encode::{BrotliEncoderOperation, BrotliEncoderParameter, BrotliEncoderStateStruct};
use brotli::enc::threading::{Owned, SendAlloc};
use brotli::enc::{
    compress_multi_no_threadpool, BrotliEncoderMaxCompressedSize,
    BrotliEncoderMaxCompressedSizeMulti, BrotliEncoderParams, StandardAlloc, UnionHasher,
};
use brotli::{Allocator, Decompressor, SliceWrapper, SliceWrapperMut};
use std::io::Read;

const MULTIPLIER: u64 = 0x9E37_79B9_7F4A_7C15;

// This generator is intentionally identical to issue #280 for shape=0.
// All branches are deterministic and do not require rand or proptest.
pub fn corpus(seed: u64, dict_len: usize, data_len: usize, shape: u8) -> (Vec<u8>, Vec<u8>) {
    let mut x = seed.wrapping_mul(MULTIPLIER) | 1;
    let total = dict_len + data_len;
    let mut all = Vec::with_capacity(total);
    for i in 0..total {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let v = match shape % 6 {
            0 => b'a' + ((x >> 32) % 3) as u8, // exact issue #280 alphabet
            1 => b'a' + ((x >> 32) % 2) as u8,
            2 => b'a' + (i.wrapping_add(seed as usize) % 4) as u8,
            3 => (x >> 32) as u8, // high entropy control
            4 => b'a' + ((i / 7).wrapping_add(seed as usize) % 3) as u8,
            _ => b'a' + ((i / 3).wrapping_add((x >> 32) as usize) % 3) as u8,
        };
        all.push(v);
    }
    (all[..dict_len].to_vec(), all[dict_len..].to_vec())
}

// This makes a match that starts on the final dictionary byte and continues
// into the payload appear attractive to the match finder.
pub fn boundary_run(dict_len: usize, data_len: usize, period: usize) -> (Vec<u8>, Vec<u8>) {
    assert!(dict_len >= 2 && data_len >= 4 && period >= 1);
    let mut dictionary = vec![b'a'; dict_len];
    dictionary[dict_len - 1] = b'b';
    let mut payload = vec![b'b'; data_len];
    for i in 0..data_len {
        if period > 1 && i % period == period - 1 {
            payload[i] = b'a';
        }
    }
    (dictionary, payload)
}

fn decompress_with_dict(compressed: &[u8], dictionary: &[u8]) -> Result<Vec<u8>, String> {
    let mut alloc = StandardAlloc::default();
    let mut dict_mem = <StandardAlloc as Allocator<u8>>::alloc_cell(&mut alloc, dictionary.len());
    dict_mem.slice_mut().copy_from_slice(dictionary);
    let mut decoder = Decompressor::new_with_custom_dict(compressed, 4096, dict_mem);
    let mut plain = Vec::new();
    decoder.read_to_end(&mut plain).map_err(|e| format!("decompress: {}", e))?;
    Ok(plain)
}

pub fn compress_dict(
    dictionary: &[u8], input: &[u8], quality: u32, catable: bool,
) -> Result<Vec<u8>, String> {
    let mut state = BrotliEncoderStateStruct::new(StandardAlloc::default());
    let params = [
        (BrotliEncoderParameter::BROTLI_PARAM_QUALITY, quality),
        (BrotliEncoderParameter::BROTLI_PARAM_LGWIN, 22),
        (BrotliEncoderParameter::BROTLI_PARAM_CATABLE, catable as u32),
        (BrotliEncoderParameter::BROTLI_PARAM_APPENDABLE, catable as u32),
    ];
    for &(key, value) in &params {
        if !state.set_parameter(key, value) {
            return Err(format!("set_parameter failed for q={}", quality));
        }
    }
    state.set_custom_dictionary(dictionary.len(), dictionary);
    let capacity = BrotliEncoderMaxCompressedSize(input.len()) + 1024;
    let mut output = vec![0u8; capacity];
    let (mut in_offset, mut out_offset) = (0usize, 0usize);
    for _ in 0..16 {
        let mut avail_in = input.len() - in_offset;
        let mut avail_out = output.len() - out_offset;
        let mut total_out = Some(out_offset);
        let ok = state.compress_stream(
            BrotliEncoderOperation::BROTLI_OPERATION_FINISH,
            &mut avail_in, input, &mut in_offset,
            &mut avail_out, &mut output, &mut out_offset, &mut total_out,
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
    dictionary: &[u8], input: &[u8], quality: u32, catable: bool,
) -> Result<usize, String> {
    let compressed = compress_dict(dictionary, input, quality, catable)?;
    let plain = decompress_with_dict(&compressed, dictionary)?;
    if plain != input {
        return Err(format!("mismatch: decoded {} bytes, expected {}", plain.len(), input.len()));
    }
    Ok(compressed.len())
}

struct OwnedBytes(Vec<u8>);
impl SliceWrapper<u8> for OwnedBytes {
    fn slice(&self) -> &[u8] { &self.0 }
}

pub fn check_multi(input: &[u8], quality: i32, threads: usize) -> Result<usize, String> {
    assert!(threads >= 1 && threads <= 8);
    let mut params = BrotliEncoderParams::default();
    params.quality = quality;
    params.catable = true;
    params.appendable = true;
    params.magic_number = true;
    params.use_dictionary = false;
    params.size_hint = input.len();

    let mut allocs = Vec::new();
    for _ in 0..threads {
        allocs.push(SendAlloc::new(StandardAlloc::default(), UnionHasher::Uninit));
    }
    let mut owned = Owned::new(OwnedBytes(input.to_vec()));
    let mut output = vec![0u8; BrotliEncoderMaxCompressedSizeMulti(input.len(), threads)];
    let written = compress_multi_no_threadpool(&params, &mut owned, &mut output, &mut allocs)
        .map_err(|err| format!("compress_multi_no_threadpool: {:?}", err))?;
    output.truncate(written);
    let mut decoder = Decompressor::new(output.as_slice(), 4096);
    let mut plain = Vec::new();
    decoder.read_to_end(&mut plain).map_err(|err| format!("multithread decode: {}", err))?;
    if plain != input {
        return Err(format!("multi mismatch: decoded {} bytes, expected {}", plain.len(), input.len()));
    }
    Ok(written)
}

pub fn check_multi_pattern(seed: u64, size: usize, shape: u8, quality: i32, threads: usize)
    -> Result<usize, String> {
    let (_prefix, input) = corpus(seed, 0, size, shape);
    check_multi(&input, quality, threads)
}
