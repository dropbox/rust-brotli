#![cfg(test)]

extern crate alloc_no_stdlib;
extern crate brotli_decompressor;
extern "C" {
    fn calloc(n_elem: usize, el_size: usize) -> *mut u8;
}
extern "C" {
    fn free(ptr: *mut u8);
}

// FIXME: Remove this after https://github.com/dropbox/rust-alloc-no-stdlib/issues/19 is fixed
use alloc::{
    declare_stack_allocator_struct, define_allocator_memory_pool, define_stack_allocator_traits,
    static_array,
};
use core;
use core::cmp::min;
use core::ops;

use brotli_decompressor::HuffmanCode;

use super::super::alloc::{
    bzero, AllocatedStackMemory, Allocator, SliceWrapper, SliceWrapperMut, StackAllocator,
};
pub use super::super::{BrotliDecompressStream, BrotliResult, BrotliState};
use super::cluster::HistogramPair;
use super::combined_alloc::CombiningAllocator;
use super::command::Command;
use super::encode::{BrotliEncoderOperation, BrotliEncoderParameter};
use super::entropy_encode::HuffmanTree;
use super::histogram::{ContextType, HistogramCommand, HistogramDistance, HistogramLiteral};
use super::pdf::PDF;
use super::{interface, s16, v8, StaticCommand, ZopfliNode};
use crate::enc::encode::BrotliEncoderStateStruct;

declare_stack_allocator_struct!(MemPool, 128, stack);
declare_stack_allocator_struct!(CallocatedFreelist4096, 128, calloc);
declare_stack_allocator_struct!(CallocatedFreelist2048, 64, calloc);
declare_stack_allocator_struct!(CallocatedFreelist1024, 32, calloc);
declare_stack_allocator_struct!(StackAllocatedFreelist64, 64, stack);

fn oneshot_compress(
    input: &[u8],
    output: &mut [u8],
    quality: u32,
    lgwin: u32,
    magic: bool,
    in_batch_size: usize,
    out_batch_size: usize,
) -> (bool, usize) {
    let stack_u8_buffer =
        unsafe { define_allocator_memory_pool!(96, u8, [0; 24 * 1024 * 1024], calloc) };
    let stack_u16_buffer =
        unsafe { define_allocator_memory_pool!(96, u16, [0; 128 * 1024], calloc) };
    let stack_i32_buffer =
        unsafe { define_allocator_memory_pool!(96, i32, [0; 128 * 1024], calloc) };
    let stack_u32_buffer =
        unsafe { define_allocator_memory_pool!(96, u32, [0; 32 * 1024 * 1024], calloc) };
    let stack_u64_buffer =
        unsafe { define_allocator_memory_pool!(96, u64, [0; 32 * 1024], calloc) };
    let stack_f64_buffer =
        unsafe { define_allocator_memory_pool!(48, super::util::floatX, [0; 128 * 1024], calloc) };
    let mut stack_global_buffer_v8 =
        define_allocator_memory_pool!(64, v8, [v8::default(); 1024 * 16], stack);
    let mf8 = StackAllocatedFreelist64::<v8>::new_allocator(&mut stack_global_buffer_v8, bzero);
    let mut stack_16x16_buffer =
        define_allocator_memory_pool!(64, s16, [s16::default(); 1024 * 16], stack);
    let m16x16 = StackAllocatedFreelist64::<s16>::new_allocator(&mut stack_16x16_buffer, bzero);

    let stack_hl_buffer =
        unsafe { define_allocator_memory_pool!(48, HistogramLiteral, [0; 128 * 1024], calloc) };
    let stack_hc_buffer =
        unsafe { define_allocator_memory_pool!(48, HistogramCommand, [0; 128 * 1024], calloc) };
    let stack_hd_buffer =
        unsafe { define_allocator_memory_pool!(48, HistogramDistance, [0; 128 * 1024], calloc) };
    let stack_hp_buffer =
        unsafe { define_allocator_memory_pool!(48, HistogramPair, [0; 128 * 1024], calloc) };
    let stack_ct_buffer =
        unsafe { define_allocator_memory_pool!(48, ContextType, [0; 128 * 1024], calloc) };
    let stack_ht_buffer =
        unsafe { define_allocator_memory_pool!(48, HuffmanTree, [0; 128 * 1024], calloc) };
    let stack_zn_buffer =
        unsafe { define_allocator_memory_pool!(48, ZopfliNode, [0; 1024], calloc) };
    let stack_mc_buffer =
        unsafe { define_allocator_memory_pool!(48, Command, [0; 128 * 1024], calloc) };
    let stack_pdf_buffer = unsafe { define_allocator_memory_pool!(48, PDF, [0; 1], calloc) };
    let stack_sc_buffer =
        unsafe { define_allocator_memory_pool!(48, StaticCommand, [0; 100], calloc) };
    let stack_u8_allocator =
        CallocatedFreelist4096::<u8>::new_allocator(stack_u8_buffer.data, bzero);
    let stack_u16_allocator =
        CallocatedFreelist4096::<u16>::new_allocator(stack_u16_buffer.data, bzero);
    let stack_i32_allocator =
        CallocatedFreelist1024::<i32>::new_allocator(stack_i32_buffer.data, bzero);
    let stack_u32_allocator =
        CallocatedFreelist4096::<u32>::new_allocator(stack_u32_buffer.data, bzero);
    let stack_u64_allocator =
        CallocatedFreelist1024::<u64>::new_allocator(stack_u64_buffer.data, bzero);
    let stack_zn_allocator =
        CallocatedFreelist1024::<ZopfliNode>::new_allocator(stack_zn_buffer.data, bzero);
    let mf64 =
        CallocatedFreelist2048::<super::util::floatX>::new_allocator(stack_f64_buffer.data, bzero);

    let mpdf = CallocatedFreelist2048::<PDF>::new_allocator(stack_pdf_buffer.data, bzero);
    let msc = CallocatedFreelist2048::<StaticCommand>::new_allocator(stack_sc_buffer.data, bzero);
    let stack_mc_allocator =
        CallocatedFreelist2048::<Command>::new_allocator(stack_mc_buffer.data, bzero);
    let mhl =
        CallocatedFreelist2048::<HistogramLiteral>::new_allocator(stack_hl_buffer.data, bzero);
    let mhc =
        CallocatedFreelist2048::<HistogramCommand>::new_allocator(stack_hc_buffer.data, bzero);
    let mhd =
        CallocatedFreelist2048::<HistogramDistance>::new_allocator(stack_hd_buffer.data, bzero);
    let mhp = CallocatedFreelist2048::<HistogramPair>::new_allocator(stack_hp_buffer.data, bzero);
    let mct = CallocatedFreelist2048::<ContextType>::new_allocator(stack_ct_buffer.data, bzero);
    let mht = CallocatedFreelist2048::<HuffmanTree>::new_allocator(stack_ht_buffer.data, bzero);
    let mut s_orig = BrotliEncoderStateStruct::new(CombiningAllocator::new(
        stack_u8_allocator,
        stack_u16_allocator,
        stack_i32_allocator,
        stack_u32_allocator,
        stack_u64_allocator,
        stack_mc_allocator,
        mf64,
        mf8,
        m16x16,
        mpdf,
        msc,
        mhl,
        mhc,
        mhd,
        mhp,
        mct,
        mht,
        stack_zn_allocator,
    ));
    let mut next_in_offset: usize = 0;
    let mut next_out_offset: usize = 0;
    {
        let s = &mut s_orig;

        s.set_parameter(BrotliEncoderParameter::BROTLI_PARAM_QUALITY, quality);
        if magic {
            s.set_parameter(
                BrotliEncoderParameter::BROTLI_PARAM_MAGIC_NUMBER,
                magic as u32,
            );
        }
        if quality >= 10 {
            s.set_parameter(BrotliEncoderParameter::BROTLI_PARAM_Q9_5, 1);
        }
        s.set_parameter(BrotliEncoderParameter::BROTLI_PARAM_LGWIN, lgwin);
        s.set_parameter(BrotliEncoderParameter::BROTLI_PARAM_MODE, 0); // gen, text, font
        s.set_parameter(
            BrotliEncoderParameter::BROTLI_PARAM_SIZE_HINT,
            input.len() as u32,
        );
        loop {
            let mut available_in: usize = min(input.len() - next_in_offset, in_batch_size);
            let mut available_out: usize = min(output.len() - next_out_offset, out_batch_size);
            if available_out == 0 {
                panic!("No output buffer space");
            }
            let mut total_out = Some(0);
            let op: BrotliEncoderOperation;
            if available_in == input.len() - next_in_offset {
                op = BrotliEncoderOperation::BROTLI_OPERATION_FINISH;
            } else {
                op = BrotliEncoderOperation::BROTLI_OPERATION_PROCESS;
            }
            let mut nop_callback =
                |_data: &mut interface::PredictionModeContextMap<interface::InputReferenceMut>,
                 _cmds: &mut [interface::StaticCommand],
                 _mb: interface::InputPair,
                 _mfv: &mut CombiningAllocator<
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                >| ();

            let result = s.compress_stream(
                op,
                &mut available_in,
                input,
                &mut next_in_offset,
                &mut available_out,
                output,
                &mut next_out_offset,
                &mut total_out,
                &mut nop_callback,
            );
            if !result {
                return (result, next_out_offset);
            }
            if s.is_finished() {
                break;
            }
        }
    }

    (true, next_out_offset)
}

#[cfg(target_pointer_width = "32")]
static lock32: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

/// 32bit systems do not have sufficient memory to compress multiple items
/// at the same time with the current limits and defaults. So we instead spin
/// until a process has completed compression. We cannot use proper locks
/// in nostd, so we fall back to this simple spin lock.
#[cfg(target_pointer_width = "32")]
fn lock_if_32bit() {
    use core::sync::atomic::Ordering;
    loop {
        let cur = lock32.fetch_add(1, Ordering::SeqCst);
        if cur == 0 {
            return;
        }
        lock32.fetch_sub(1, Ordering::SeqCst);
    }
}
#[cfg(target_pointer_width = "32")]
fn unlock_if_32bit() {
    use core::sync::atomic::Ordering;
    lock32.fetch_sub(1, Ordering::SeqCst);
}
#[cfg(not(target_pointer_width = "32"))]
fn lock_if_32bit() {}
#[cfg(not(target_pointer_width = "32"))]
fn unlock_if_32bit() {}

pub(crate) fn oneshot_decompress(
    compressed: &[u8],
    output: &mut [u8],
) -> (BrotliResult, usize, usize) {
    let mut available_in: usize = compressed.len();
    let mut available_out: usize = output.len();
    let mut stack_u8_buffer = define_allocator_memory_pool!(128, u8, [0; 100 * 1024], stack);
    let mut stack_u32_buffer = define_allocator_memory_pool!(128, u32, [0; 36 * 1024], stack);
    let mut stack_hc_buffer = define_allocator_memory_pool!(
        128,
        HuffmanCode,
        [HuffmanCode::default(); 116 * 1024],
        stack
    );

    let stack_u8_allocator = MemPool::<u8>::new_allocator(&mut stack_u8_buffer, bzero);
    let stack_u32_allocator = MemPool::<u32>::new_allocator(&mut stack_u32_buffer, bzero);
    let stack_hc_allocator = MemPool::<HuffmanCode>::new_allocator(&mut stack_hc_buffer, bzero);
    let mut input_offset: usize = 0;
    let mut output_offset: usize = 0;
    let mut written: usize = 0;
    let mut brotli_state =
        BrotliState::new(stack_u8_allocator, stack_u32_allocator, stack_hc_allocator);
    let result = BrotliDecompressStream(
        &mut available_in,
        &mut input_offset,
        compressed,
        &mut available_out,
        &mut output_offset,
        output,
        &mut written,
        &mut brotli_state,
    );
    (result, input_offset, output_offset)
}

fn oneshot(
    input: &[u8],
    compressed: &mut [u8],
    output: &mut [u8],
    q: u32,
    lg: u32,
    magic: bool,
    in_buffer_size: usize,
    out_buffer_size: usize,
) -> (BrotliResult, usize, usize) {
    lock_if_32bit();
    let (success, mut available_in) = oneshot_compress(
        input,
        compressed,
        q,
        lg,
        magic,
        in_buffer_size,
        out_buffer_size,
    );
    if !success {
        //return (BrotliResult::ResultFailure, 0, 0);
        available_in = compressed.len();
    }
    let ret = oneshot_decompress(&mut compressed[..available_in], output);
    unlock_if_32bit();
    ret
}

#[test]
fn test_roundtrip_10x10y() {
    const BUFFER_SIZE: usize = 128;
    let mut compressed: [u8; 13] = [0; 13];
    let mut output = [0u8; BUFFER_SIZE];
    let mut input = [
        'x' as u8, 'x' as u8, 'x' as u8, 'x' as u8, 'x' as u8, 'x' as u8, 'x' as u8, 'x' as u8,
        'x' as u8, 'x' as u8, 'y' as u8, 'y' as u8, 'y' as u8, 'y' as u8, 'y' as u8, 'y' as u8,
        'y' as u8, 'y' as u8, 'y' as u8, 'y' as u8,
    ];
    let (result, compressed_offset, output_offset) = oneshot(
        &mut input[..],
        &mut compressed,
        &mut output[..],
        9,
        10,
        false,
        1,
        1,
    );
    match result {
        BrotliResult::ResultSuccess => {}
        _ => assert!(false),
    }
    let mut i: usize = 0;
    while i < 10 {
        assert_eq!(output[i], 'x' as u8);
        assert_eq!(output[i + 10], 'y' as u8);
        i += 1;
    }
    assert_eq!(output_offset, 20);
    assert_eq!(compressed_offset, compressed.len());
}

macro_rules! test_roundtrip_file {
    ($filedata : expr, $bufsize: expr, $quality: expr, $lgwin: expr, $magic: expr, $in_buf:expr, $out_buf:expr) => {{
        let stack_u8_buffer = unsafe {
            alloc::define_allocator_memory_pool!(4096, u8, [0; 18 * 1024 * 1024], calloc)
        };
        let mut stack_u8_allocator =
            CallocatedFreelist4096::<u8>::new_allocator(stack_u8_buffer.data, bzero);

        let mut compressed = stack_u8_allocator.alloc_cell($bufsize);
        let inp = $filedata;
        let mut output = stack_u8_allocator.alloc_cell(inp.len() + 16);
        let (result, compressed_offset, output_offset) = oneshot(
            &inp[..],
            compressed.slice_mut(),
            output.slice_mut(),
            $quality,
            $lgwin,
            $magic,
            $in_buf,
            $out_buf,
        );
        match result {
            BrotliResult::ResultSuccess => {}
            _ => assert!(false),
        }
        for i in 0..inp.len() {
            if inp[i] != output[i] {
                assert_eq!((i, inp[i]), (i, output[i]));
            }
            assert_eq!(inp[i], output[i]);
        }
        assert!(compressed_offset <= compressed.slice().len());
        assert_eq!(output_offset, inp.len());
        stack_u8_allocator.free_cell(output);
        stack_u8_allocator.free_cell(compressed);
    }};
}

#[test]
fn test_roundtrip_64x() {
    test_roundtrip_file!(include_bytes!("../../testdata/64x"), 72, 9, 10, false, 3, 2);
}
#[test]
fn test_roundtrip_ukkonooa() {
    test_roundtrip_file!(
        include_bytes!("../../testdata/ukkonooa"),
        82,
        9,
        10,
        true,
        3,
        2
    );
}
#[test]
fn test_roundtrip_backward65536() {
    test_roundtrip_file!(
        include_bytes!("../../testdata/backward65536"),
        72000,
        9,
        10,
        false,
        3,
        2
    );
}
#[test]
fn test_roundtrip_aaabaaaa() {
    test_roundtrip_file!(
        include_bytes!("../../testdata/aaabaaaa"),
        72000,
        9,
        10,
        true,
        3,
        2
    );
}
#[test]
fn test_roundtrip_monkey() {
    test_roundtrip_file!(
        include_bytes!("../../testdata/monkey"),
        72000,
        9,
        10,
        false,
        16,
        15
    );
}
#[test]
fn test_roundtrip_quickfox_repeated() {
    test_roundtrip_file!(
        include_bytes!("../../testdata/quickfox_repeated"),
        16384,
        9,
        10,
        true,
        257,
        255
    );
}

#[test]
fn test_roundtrip_asyoulik() {
    test_roundtrip_file!(
        include_bytes!("../../testdata/asyoulik.txt"),
        64384,
        9,
        15,
        false,
        513,
        511
    );
}

#[test]
fn test_roundtrip_asyoulik9_5() {
    test_roundtrip_file!(
        include_bytes!("../../testdata/asyoulik.txt"),
        62384,
        10,
        15,
        true,
        513,
        511
    );
}

#[test]
fn test_roundtrip_compressed() {
    test_roundtrip_file!(
        include_bytes!("../../testdata/compressed_file"),
        50400,
        9,
        10,
        false,
        1025,
        1024
    );
}

#[test]
fn test_roundtrip_compressed_repeated() {
    test_roundtrip_file!(
        include_bytes!("../../testdata/compressed_repeated"),
        120000,
        9,
        16,
        false,
        2049,
        2047
    );
}

#[test]
fn test_roundtrip_first_58_bytes_alice() {
    test_roundtrip_file!(
        &include_bytes!("../../testdata/alice29.txt")[..58],
        50400,
        2,
        10,
        true,
        1,
        2
    );
}
#[test]
fn test_roundtrip_first_2_bytes_alice() {
    test_roundtrip_file!(
        &include_bytes!("../../testdata/alice29.txt")[..2],
        50400,
        2,
        10,
        true,
        1,
        2
    );
}

#[test]
fn test_roundtrip_quickfox() {
    test_roundtrip_file!(
        include_bytes!("../../testdata/quickfox"),
        256,
        9,
        10,
        false,
        1,
        2
    );
}

#[test]
fn test_roundtrip_x() {
    const BUFFER_SIZE: usize = 16384;
    let mut compressed: [u8; 6] = [0x0b, 0x00, 0x80, 0x58, 0x03, 0];
    let mut output = [0u8; BUFFER_SIZE];
    let mut input = ['X' as u8];
    let (result, compressed_offset, output_offset) = oneshot(
        &mut input[..],
        &mut compressed[..],
        &mut output[..],
        9,
        10,
        false,
        1,
        2,
    );
    match result {
        BrotliResult::ResultSuccess => {}
        _ => assert!(false),
    }
    assert_eq!(output[0], 'X' as u8);
    assert_eq!(output_offset, 1);
    assert_eq!(compressed_offset, compressed.len());
}

#[test]
fn test_roundtrip_empty() {
    let mut compressed: [u8; 2] = [0x06, 0];
    let mut output = [0u8; 1];
    let (result, compressed_offset, output_offset) = oneshot(
        &mut [],
        &mut compressed[..],
        &mut output[..],
        9,
        10,
        false,
        2,
        3,
    );
    match result {
        BrotliResult::ResultSuccess => {}
        _ => assert!(false),
    }
    assert_eq!(output_offset, 0);
    assert_eq!(compressed_offset, compressed.len());
}

// At quality 1 the two-pass compressor's hash table is the smallest power of two at or
// above the input size, so a 20,000-byte input gets a 2^15 table. Google's encoder uses
// 4-byte matches for tables of up to 2^15 entries, `(B <= 15) ? 4 : 6`.
#[test]
fn test_quality_1_matches_c_with_2_15_hash_table() {
    let mut input = [0u8; 20_000];
    for (i, b) in input.iter_mut().enumerate() {
        *b = ((i % 13) + (i >> 11)) as u8;
    }
    // BrotliEncoderCompress(1, 22, BROTLI_MODE_GENERIC, ...) in Google's brotli 1.1.0 and 1.2.0
    let expected: [u8; 119] = [
        0x8b, 0x0f, 0x27, 0x00, 0x80, 0xaa, 0xaa, 0xaa, 0xea, 0x5f, 0xb7, 0x4d, 0x58, 0x56, 0x85,
        0xff, 0xdc, 0x6d, 0xad, 0x3a, 0xd5, 0x63, 0xbe, 0x6a, 0xfb, 0x78, 0x18, 0x1e, 0x61, 0x8c,
        0x81, 0xd3, 0x83, 0x39, 0x19, 0x25, 0x10, 0xca, 0xdd, 0xd5, 0x7f, 0xca, 0xa5, 0xb6, 0x3e,
        0xe6, 0xda, 0xe7, 0x3e, 0xc1, 0xea, 0xac, 0xec, 0x17, 0xe1, 0x40, 0x5d, 0x3c, 0x05, 0x40,
        0x12, 0x30, 0x46, 0x17, 0x11, 0xf7, 0x62, 0x1c, 0xa9, 0x8b, 0xb3, 0xfd, 0x2f, 0x02, 0x0b,
        0x03, 0xd3, 0x45, 0x81, 0x45, 0x4f, 0x00, 0x7b, 0x64, 0x3f, 0xaa, 0x8b, 0x38, 0xfd, 0x5f,
        0xd9, 0x2f, 0xc3, 0x41, 0xba, 0xb8, 0x6a, 0xfe, 0xff, 0xd7, 0xf6, 0x20, 0xc0, 0xea, 0xe2,
        0x14, 0x00, 0xe9, 0x81, 0x61, 0xba, 0xc8, 0xf8, 0x5b, 0x80, 0x29, 0x17, 0x2c, 0xc7,
    ];
    let mut compressed = [0u8; 1024];
    let mut output = [0u8; 20_000];
    let (result, compressed_len, output_len) = oneshot(
        &input[..],
        &mut compressed[..],
        &mut output[..],
        1,
        22,
        false,
        input.len(),
        1024,
    );
    match result {
        BrotliResult::ResultSuccess => {}
        _ => assert!(false),
    }
    assert_eq!(output_len, input.len());
    assert!(output[..] == input[..]);
    assert_eq!(&compressed[..compressed_len], &expected[..]);
}

#[cfg(feature = "std")]
#[test]
fn test_compress_into_short_buffer() {
    use std::io::{Cursor, ErrorKind, Write};

    // this plaintext should compress to 11 bytes
    let plaintext = [0u8; 2048];

    // but we only provide space for 10
    let mut output_buffer = [0u8; 10];
    let mut output_cursor = Cursor::new(&mut output_buffer[..]);

    let mut w = crate::CompressorWriter::new(&mut output_cursor, 4096, 4, 22);
    assert_eq!(w.write(&plaintext).unwrap(), 2048);
    assert_eq!(w.flush().unwrap_err().kind(), ErrorKind::WriteZero);
    w.into_inner();

    println!("{output_buffer:?}");
}
/*


#[cfg(feature="std")]
struct Buffer {
  data: Vec<u8>,
  read_offset: usize,
}
#[cfg(feature="std")]
impl Buffer {
  pub fn new(buf: &[u8]) -> Buffer {
    let mut ret = Buffer {
      data: Vec::<u8>::new(),
      read_offset: 0,
    };
    ret.data.extend(buf);
    return ret;
  }
}
#[cfg(feature="std")]
impl io::Read for Buffer {
  fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> {
    let bytes_to_read = min(buf.len(), self.data.len() - self.read_offset);
    if bytes_to_read > 0 {
      buf[0..bytes_to_read]
        .clone_from_slice(&self.data[self.read_offset..self.read_offset + bytes_to_read]);
    }
    self.read_offset += bytes_to_read;
    return Ok(bytes_to_read);
  }
}
#[cfg(feature="std")]
impl io::Write for Buffer {
  fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> {
    self.data.extend(buf);
    return Ok(buf.len());
  }
  fn flush(self: &mut Self) -> io::Result<()> {
    return Ok(());
  }
}


*/
