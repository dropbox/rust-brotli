#![cfg(test)]
extern crate core;
use super::HeapAllocator;
#[allow(unused_imports)]
use super::alloc_no_stdlib::{Allocator, SliceWrapper, SliceWrapperMut};
use super::brotli::BrotliResult;
use super::brotli::BrotliState;
#[cfg(not(feature="no-stdlib"))]
use super::brotli::{CompressorReader, CompressorWriter};
#[cfg(not(feature="no-stdlib"))]
use super::brotli::{Decompressor, DecompressorWriter};
use super::brotli::HuffmanCode;
use core::cmp;
use std::io;
#[cfg(not(feature="no-stdlib"))]
use std::io::{Read, Write};
use std::time::Duration;
#[cfg(not(feature="disable-timer"))]
use std::time::SystemTime;
use brotli::BrotliDecompressStream;

struct Buffer {
  data: Vec<u8>,
  read_offset: usize,
}

struct UnlimitedBuffer {
  data: Vec<u8>,
  read_offset: usize,
}


#[cfg(feature="disable-timer")]
fn now() -> Duration {
  return Duration::new(0, 0);
}
#[cfg(not(feature="disable-timer"))]
fn now() -> SystemTime {
  return SystemTime::now();
}

#[cfg(not(feature="disable-timer"))]
fn elapsed(start: SystemTime) -> (Duration, bool) {
  match start.elapsed() {
    Ok(delta) => return (delta, false),
    _ => return (Duration::new(0, 0), true),
  }
}

#[cfg(feature="disable-timer")]
fn elapsed(_start: Duration) -> (Duration, bool) {
  return (Duration::new(0, 0), true);
}


fn _write_all<OutputType>(w: &mut OutputType, buf: &[u8]) -> Result<(), io::Error>
  where OutputType: io::Write
{
  let mut total_written: usize = 0;
  while total_written < buf.len() {
    match w.write(&buf[total_written..]) {
      Err(e) => {
        match e.kind() {
          io::ErrorKind::Interrupted => continue,
          _ => return Err(e),
        }
      }
      Ok(cur_written) => {
        if cur_written == 0 {
          return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Write EOF"));
        }
        total_written += cur_written;
      }
    }
  }
  Ok(())
}


#[cfg(feature="benchmark")]
const NUM_BENCHMARK_ITERATIONS: usize = 1000;
#[cfg(not(feature="benchmark"))]
const NUM_BENCHMARK_ITERATIONS: usize = 2;

// option_env!("BENCHMARK_MODE").is_some()

pub fn decompress_internal<InputType, OutputType>(r: &mut InputType,
                                                  mut w: &mut OutputType,
                                                  input_buffer_limit: usize,
                                                  output_buffer_limit: usize,
                                                  benchmark_mode: bool)
                                                  -> Result<(), io::Error>
  where InputType: io::Read,
        OutputType: io::Write
{
  let mut total = Duration::new(0, 0);
  let range: usize;
  let mut timing_error: bool = false;
  if benchmark_mode {
    range = NUM_BENCHMARK_ITERATIONS;
  } else {
    range = 1;
  }
  for _i in 0..range {
    let mut brotli_state =
      BrotliState::new(HeapAllocator::<u8> { default_value: 0 },
                       HeapAllocator::<u32> { default_value: 0 },
                       HeapAllocator::<HuffmanCode> { default_value: HuffmanCode::default() });
    let mut input = brotli_state.alloc_u8.alloc_cell(input_buffer_limit);
    let mut output = brotli_state.alloc_u8.alloc_cell(output_buffer_limit);
    let mut available_out: usize = output.slice().len();

    // let amount = try!(r.read(&mut buf));
    let mut available_in: usize = 0;
    let mut input_offset: usize = 0;
    let mut output_offset: usize = 0;
    let mut result: BrotliResult = BrotliResult::NeedsMoreInput;
    loop {
      match result {
        BrotliResult::NeedsMoreInput => {
          input_offset = 0;
          match r.read(input.slice_mut()) {
            Err(e) => {
              match e.kind() {
                io::ErrorKind::Interrupted => continue,
                _ => return Err(e),
              }
            }
            Ok(size) => {
              if size == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Read EOF"));
              }
              available_in = size;
            }
          }
        }
        BrotliResult::NeedsMoreOutput => {
          try!(_write_all(&mut w, &output.slice()[..output_offset]));
          output_offset = 0;
        }
        BrotliResult::ResultSuccess => break,
        BrotliResult::ResultFailure => panic!("FAILURE"),
      }
      let mut written: usize = 0;
      let start = now();
      result = BrotliDecompressStream(&mut available_in,
                                      &mut input_offset,
                                      &input.slice(),
                                      &mut available_out,
                                      &mut output_offset,
                                      &mut output.slice_mut(),
                                      &mut written,
                                      &mut brotli_state);

      let (delta, err) = elapsed(start);
      if err {
        timing_error = true;
      }
      total = total + delta;
      if output_offset != 0 {
        try!(_write_all(&mut w, &output.slice()[..output_offset]));
        output_offset = 0;
        available_out = output.slice().len()
      }
    }
    brotli_state.BrotliStateCleanup();
  }
  if timing_error {
    let _r = super::writeln0(&mut io::stderr(), "Timing error");
  } else {
    let _r = super::writeln_time(&mut io::stderr(),
                                 "Iterations; Time",
                                 range as u64,
                                 total.as_secs(),
                                 total.subsec_nanos());
  }
  Ok(())
}

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
impl UnlimitedBuffer {
  pub fn new(buf: &[u8]) -> Self {
    let mut ret = UnlimitedBuffer {
      data: Vec::<u8>::new(),
      read_offset: 0,
    };
    ret.data.extend(buf);
    return ret;
  }
}
impl io::Read for Buffer {
  fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> {
    if self.read_offset == self.data.len() {
      self.read_offset = 0;
    }
    let bytes_to_read = cmp::min(buf.len(), self.data.len() - self.read_offset);
    if bytes_to_read > 0 {
      buf[0..bytes_to_read].clone_from_slice(&self.data[self.read_offset..
                                              self.read_offset + bytes_to_read]);
    }
    self.read_offset += bytes_to_read;
    return Ok(bytes_to_read);
  }
}
impl io::Write for Buffer {
  fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> {
    if self.read_offset == self.data.len() {
      return Ok(buf.len());
    }
    self.data.extend(buf);
    return Ok(buf.len());
  }
  fn flush(self: &mut Self) -> io::Result<()> {
    return Ok(());
  }
}
impl io::Read for UnlimitedBuffer {
  fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> {
    let bytes_to_read = cmp::min(buf.len(), self.data.len() - self.read_offset);
    if bytes_to_read > 0 {
      buf[0..bytes_to_read].clone_from_slice(&self.data[self.read_offset..
                                              self.read_offset + bytes_to_read]);
    }
    self.read_offset += bytes_to_read;
    return Ok(bytes_to_read);
  }
}

impl io::Write for UnlimitedBuffer {
  fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> {
    self.data.extend(buf);
    return Ok(buf.len());
  }
  fn flush(self: &mut Self) -> io::Result<()> {
    return Ok(());
  }
}

#[allow(non_snake_case)]
#[test]
fn test_roundtrip_64x() {
  let X = 'X' as u8;
  let in_buf: [u8; 64] = [X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
                          X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
                          X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X];

  let mut input = UnlimitedBuffer::new(&in_buf);
  let mut compressed = UnlimitedBuffer::new(&[]);
  let mut output = UnlimitedBuffer::new(&[]);
  let q: u32 = 9;
  let lgwin: u32 = 16;
  match super::compress(&mut input, &mut compressed, 65536, q, lgwin) {
    Ok(_) => {}
    Err(e) => panic!("Error {:?}", e),
  }
  let mut compressed_in = UnlimitedBuffer::new(&compressed.data[..]);
  match super::decompress(&mut compressed_in, &mut output, 65536) {
    Ok(_) => {}
    Err(e) => panic!("Error {:?}", e),
  }
  for i in 0..input.data.len() {
    assert_eq!(output.data[i], input.data[i]);
  }
  assert_eq!(output.data.len(), input.data.len());
  assert_eq!(input.read_offset, in_buf.len());
}

fn roundtrip_helper(in_buf: &[u8], q: u32, lgwin: u32) {
  let mut input = UnlimitedBuffer::new(&in_buf);
  let mut compressed = UnlimitedBuffer::new(&[]);
  let mut output = UnlimitedBuffer::new(&[]);
  match super::compress(&mut input, &mut compressed, 65536, q, lgwin) {
    Ok(_) => {}
    Err(e) => panic!("Error {:?}", e),
  }
  let mut compressed_in = UnlimitedBuffer::new(&compressed.data[..]);
  match super::decompress(&mut compressed_in, &mut output, 65536) {
    Ok(_) => {}
    Err(e) => panic!("Error {:?}", e),
  }
  for i in 0..input.data.len() {
    assert_eq!(output.data[i], input.data[i]);
  }
  assert_eq!(output.data.len(), input.data.len());
  assert_eq!(input.read_offset, in_buf.len());
}

#[test]
fn test_roundtrip_quickfox_repeated() {
  roundtrip_helper(include_bytes!("testdata/quickfox_repeated"), 9, 22);
}

#[test]
fn test_roundtrip_alice29() {
  roundtrip_helper(include_bytes!("testdata/alice29.txt"), 9, 22);
}

#[test]
fn test_roundtrip_as_you_lik() {
  roundtrip_helper(include_bytes!("testdata/asyoulik.txt"), 9, 20);
}

#[cfg(not(feature="no-stdlib"))]
fn reader_helper(mut in_buf: &[u8], q: u32, lgwin: u32) {
  let original_buf = in_buf;
  let mut cmp = [0u8; 259];
  let mut input = UnlimitedBuffer::new(&in_buf);
  {
  let renc = CompressorReader::new(&mut input, 255, q, lgwin);
  let mut rdec = Decompressor::new(renc, 257);
  loop {
    match rdec.read(&mut cmp[..]) {
      Ok(size) => {
        if size == 0 {
          break;
        }
        assert_eq!(cmp[..size], in_buf[..size]);
        in_buf = &in_buf[size..];
      }
      Err(e) => panic!("Error {:?}", e),
    }
  }
  }
  in_buf = original_buf;
  input = UnlimitedBuffer::new(&in_buf);
  let mut r2enc = CompressorReader::new(&mut input, 255, q, lgwin);
  let mut compressed_size = 0usize;
  loop {
    match r2enc.read(&mut cmp[..]) {
      Ok(size) => {
        if size == 0 {
          break;
        }
        compressed_size += size;
      }
      Err(e) => panic!("Error {:?}", e),
    }
  }
  let pct_ratio = 90usize;
  assert!(compressed_size < original_buf.len() * pct_ratio / 100);
}
#[cfg(not(feature="no-stdlib"))]
#[test]
fn test_reader_as_you_lik() {
  reader_helper(include_bytes!("testdata/asyoulik.txt"), 9, 20);
}
#[cfg(not(feature="no-stdlib"))]
#[test]
fn test_reader_quickfox_repeated() {
  reader_helper(include_bytes!("testdata/quickfox_repeated"), 9, 20);
}
#[cfg(not(feature="no-stdlib"))]
#[test]
fn test_reader_random_then_unicode() {
  reader_helper(include_bytes!("testdata/random_then_unicode"), 9, 20);
}

#[cfg(not(feature="no-stdlib"))]
#[test]
fn test_reader_alice() {
  reader_helper(include_bytes!("testdata/alice29.txt"), 9, 22);
}

#[cfg(not(feature="no-stdlib"))]
fn writer_helper(mut in_buf: &[u8], buf_size: usize, q: u32, lgwin: u32) {
  let original_buf = in_buf;
  let mut output = UnlimitedBuffer::new(&[]);
  {
  {let wdec = DecompressorWriter::new(&mut output, 257);
  {let mut wenc = CompressorWriter::new(wdec, 255, q, lgwin);
  while in_buf.len() > 0 {
    match wenc.write(&in_buf[..cmp::min(in_buf.len(), buf_size)]) {
      Ok(size) => {
        if size == 0 {
          break;
        }
        in_buf = &in_buf[size..];
      }
      Err(e) => panic!("Error {:?}", e),
    }
  }
  }
  }
  assert_eq!(output.data.len(), original_buf.len());
  for i in 0..cmp::min(original_buf.len(), output.data.len()) {
    assert_eq!(output.data[i], original_buf[i]);
  }
  in_buf = original_buf;
  let mut compressed = UnlimitedBuffer::new(&[]);
  {
  let mut wenc = CompressorWriter::new(&mut compressed, 255, q, lgwin);
  while in_buf.len() > 0 {
    match wenc.write(&in_buf[..cmp::min(in_buf.len(), buf_size)]) {
      Ok(size) => {
        if size == 0 {
          break;
        }
        in_buf = &in_buf[size..];
      }
      Err(e) => panic!("Error {:?}", e),
    }
  }
  }
  let pct_ratio = 90usize;
  assert!(compressed.data.len() < original_buf.len() * pct_ratio / 100);
  }
}
#[cfg(not(feature="no-stdlib"))]
#[test]
fn test_writer_as_you_lik() {
  writer_helper(include_bytes!("testdata/asyoulik.txt"), 17, 9, 20);
}
#[cfg(not(feature="no-stdlib"))]
#[test]
fn test_writer_64x() {
  writer_helper(include_bytes!("testdata/64x"), 17, 9, 20);
}
#[cfg(not(feature="no-stdlib"))]
#[test]
fn test_writer_quickfox_repeated() {
  writer_helper(include_bytes!("testdata/quickfox_repeated"), 251, 9, 20);
}
#[cfg(not(feature="no-stdlib"))]
#[test]
fn test_writer_random_then_unicode() {
  writer_helper(include_bytes!("testdata/random_then_unicode"), 277, 9, 20);
}

#[cfg(not(feature="no-stdlib"))]
#[test]
fn test_writer_alice() {
  writer_helper(include_bytes!("testdata/alice29.txt"), 299, 9, 22);
}


#[test]
fn test_10x_10y() {
  let in_buf: [u8; 12] = [0x1b, 0x13, 0x00, 0x00, 0xa4, 0xb0, 0xb2, 0xea, 0x81, 0x47, 0x02, 0x8a];
  let mut input = Buffer::new(&in_buf);
  let mut output = Buffer::new(&[]);
  output.read_offset = 20;
  match super::decompress(&mut input, &mut output, 65536) {
    Ok(_) => {}
    Err(e) => panic!("Error {:?}", e),
  }
  let mut i: usize = 0;
  while i < 10 {
    assert_eq!(output.data[i], 'X' as u8);
    assert_eq!(output.data[i + 10], 'Y' as u8);
    i += 1;
  }
  assert_eq!(output.data.len(), 20);
  assert_eq!(input.read_offset, in_buf.len());
}

#[test]
fn test_10x_10y_one_out_byte() {
  let in_buf: [u8; 12] = [0x1b, 0x13, 0x00, 0x00, 0xa4, 0xb0, 0xb2, 0xea, 0x81, 0x47, 0x02, 0x8a];
  let mut input = Buffer::new(&in_buf);
  let mut output = Buffer::new(&[]);
  output.read_offset = 20;
  match decompress_internal(&mut input, &mut output, 12, 1, false) {
    Ok(_) => {}
    Err(e) => panic!("Error {:?}", e),
  }
  let mut i: usize = 0;
  while i < 10 {
    assert_eq!(output.data[i], 'X' as u8);
    assert_eq!(output.data[i + 10], 'Y' as u8);
    i += 1;
  }
  assert_eq!(output.data.len(), 20);
  assert_eq!(input.read_offset, in_buf.len());
}

#[test]
fn test_10x_10y_byte_by_byte() {
  let in_buf: [u8; 12] = [0x1b, 0x13, 0x00, 0x00, 0xa4, 0xb0, 0xb2, 0xea, 0x81, 0x47, 0x02, 0x8a];
  let mut input = Buffer::new(&in_buf);
  let mut output = Buffer::new(&[]);
  output.read_offset = 20;
  match decompress_internal(&mut input, &mut output, 1, 1, false) {
    Ok(_) => {}
    Err(e) => panic!("Error {:?}", e),
  }
  let mut i: usize = 0;
  while i < 10 {
    assert_eq!(output.data[i], 'X' as u8);
    assert_eq!(output.data[i + 10], 'Y' as u8);
    i += 1;
  }
  assert_eq!(output.data.len(), 20);
  assert_eq!(input.read_offset, in_buf.len());
}


fn assert_decompressed_input_matches_output(input_slice: &[u8],
                                            output_slice: &[u8],
                                            input_buffer_size: usize,
                                            output_buffer_size: usize) {
  let mut input = Buffer::new(input_slice);
  let mut output = Buffer::new(&[]);
  output.read_offset = output_slice.len();
  if input_buffer_size == output_buffer_size {
    match super::decompress(&mut input, &mut output, input_buffer_size) {
      Ok(_) => {}
      Err(e) => panic!("Error {:?}", e),
    }
  } else {
    match decompress_internal(&mut input,
                              &mut output,
                              input_buffer_size,
                              output_buffer_size,
                              false) {
      Ok(_) => {}
      Err(e) => panic!("Error {:?}", e),
    }
  }
  assert_eq!(output.data.len(), output_slice.len());
  assert_eq!(output.data, output_slice)
}

fn benchmark_decompressed_input(input_slice: &[u8],
                                output_slice: &[u8],
                                input_buffer_size: usize,
                                output_buffer_size: usize) {
  let mut input = Buffer::new(input_slice);
  let mut output = Buffer::new(&[]);
  output.read_offset = output_slice.len();
  match decompress_internal(&mut input,
                            &mut output,
                            input_buffer_size,
                            output_buffer_size,
                            true) {
    Ok(_) => {}
    Err(e) => panic!("Error {:?}", e),
  }
  assert_eq!(output.data.len(), output_slice.len());
  assert_eq!(output.data, output_slice)
}

#[test]
fn test_64x() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/64x.compressed"),
                                           include_bytes!("testdata/64x"),
                                           3,
                                           3);
}

#[test]
fn test_as_you_like_it() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/asyoulik.txt.compressed"),
                                           include_bytes!("testdata/asyoulik.txt"),
                                           65536,
                                           65536);
}


#[test]
#[should_panic]
fn test_negative_hypothesis() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/64x"),
                                           include_bytes!("testdata/64x"),
                                           3,
                                           3);
}
static ALICE29_BR: &'static [u8] = include_bytes!("testdata/alice29.txt.compressed");
static ALICE29: &'static [u8] = include_bytes!("testdata/alice29.txt");
#[test]
fn test_alice29() {
  assert_decompressed_input_matches_output(ALICE29_BR, ALICE29, 65536, 65536);
}

#[test]
fn benchmark_alice29() {
  benchmark_decompressed_input(ALICE29_BR, ALICE29, 65536, 65536);
}

#[test]
fn test_alice1() {
  assert_decompressed_input_matches_output(ALICE29_BR, ALICE29, 1, 65536);
}

#[test]
fn test_backward65536() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/backward65536.compressed"),
                                           include_bytes!("testdata/backward65536"),
                                           65536,
                                           65536);
}


#[test]
fn test_compressed_file() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/compressed_file.compressed"),
                                           include_bytes!("testdata/compressed_file"),
                                           65536,
                                           65536);
}

#[test]
fn test_compressed_repeated() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/compressed_repeated.\
                                                           compressed"),
                                           include_bytes!("testdata/compressed_repeated"),
                                           65536,
                                           65536);
}

#[test]
fn test_empty() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty0() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.00"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty1() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.01"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty2() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.02"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty3() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.03"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty4() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.04"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty5() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.05"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty6() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.06"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty7() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.07"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty8() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.08"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty9() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.09"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty10() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.10"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty11() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.11"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty12() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.12"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty13() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.13"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty14() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.14"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty15() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.15"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty16() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.16"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty17() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.17"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}
#[test]
fn test_empty18() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.18"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}

#[test]
fn lcet10() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/lcet10.txt.compressed"),
                                           include_bytes!("testdata/lcet10.txt"),
                                           65536,
                                           65536);
}

#[test]
fn test_mapsdatazrh() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/mapsdatazrh.compressed"),
                                           include_bytes!("testdata/mapsdatazrh"),
                                           65536,
                                           65536);
}

#[test]
fn test_monkey() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/monkey.compressed"),
                                           include_bytes!("testdata/monkey"),
                                           65536,
                                           65536);
}

#[test]
fn test_monkey1() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/monkey.compressed"),
                                           include_bytes!("testdata/monkey"),
                                           1,
                                           1);
}

#[test]
fn test_monkey3() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/monkey.compressed"),
                                           include_bytes!("testdata/monkey"),
                                           3,
                                           65536);
}

#[test]
fn test_plrabn12() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/plrabn12.txt.compressed"),
                                           include_bytes!("testdata/plrabn12.txt"),
                                           65536,
                                           65536);
}

#[test]
fn test_random_org_10k() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/random_org_10k.bin.\
                                                           compressed"),
                                           include_bytes!("testdata/random_org_10k.bin"),
                                           65536,
                                           65536);
}

#[test]
fn test_ukkonooa() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/ukkonooa.compressed"),
                                           include_bytes!("testdata/ukkonooa"),
                                           65536,
                                           65536);
}

#[test]
fn test_ukkonooa3() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/ukkonooa.compressed"),
                                           include_bytes!("testdata/ukkonooa"),
                                           3,
                                           3);
}

#[test]
fn test_ukkonooa1() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/ukkonooa.compressed"),
                                           include_bytes!("testdata/ukkonooa"),
                                           1,
                                           1);
}

#[test]
fn test_x() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/x.compressed"),
                                           include_bytes!("testdata/x"),
                                           65536,
                                           65536);
}
#[test]
fn test_x_0() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/x.compressed.00"),
                                           include_bytes!("testdata/x"),
                                           65536,
                                           65536);
}
#[test]
fn test_x_1() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/x.compressed.01"),
                                           include_bytes!("testdata/x"),
                                           65536,
                                           65536);
}
#[test]
fn test_x_2() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/x.compressed.02"),
                                           include_bytes!("testdata/x"),
                                           65536,
                                           65536);
}
#[test]
fn test_x_3() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/x.compressed.03"),
                                           include_bytes!("testdata/x"),
                                           65536,
                                           65536);
}

#[test]
fn test_xyzzy() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/xyzzy.compressed"),
                                           include_bytes!("testdata/xyzzy"),
                                           65536,
                                           65536);
}

#[test]
fn test_zeros() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/zeros.compressed"),
                                           include_bytes!("testdata/zeros"),
                                           65536,
                                           65536);
}


#[test]
fn test_metablock_reset() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/metablock_reset.compressed"),
                                           include_bytes!("testdata/metablock_reset"),
                                           65536,
                                           65536);
}

#[test]
fn test_metablock_reset1_65536() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/metablock_reset.compressed"),
                                           include_bytes!("testdata/metablock_reset"),
                                           1,
                                           65536);
}

#[test]
fn test_metablock_reset65536_1() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/metablock_reset.compressed"),
                                           include_bytes!("testdata/metablock_reset"),
                                           65536,
                                           1);
}

#[test]
fn test_metablock_reset1() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/metablock_reset.compressed"),
                                           include_bytes!("testdata/metablock_reset"),
                                           1,
                                           1);
}

#[test]
fn test_metablock_reset3() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/metablock_reset.compressed"),
                                           include_bytes!("testdata/metablock_reset"),
                                           3,
                                           3);
}

#[test]
#[should_panic]
fn test_broken_file() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/borked.compressed"),
                                           include_bytes!("testdata/empty"),
                                           65536,
                                           65536);
}

#[test]
fn test_ends_with_truncated_dictionary() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/ends_with_truncated_dictionary.\
                                                           compressed"),
                                           include_bytes!("testdata/ends_with_truncated_dictionary"),
                                           65536,
                                           65536);
}

#[test]
fn test_random_then_unicode() {
  assert_decompressed_input_matches_output(include_bytes!("testdata/random_then_unicode.\
                                                           compressed"),
                                           include_bytes!("testdata/random_then_unicode"),
                                           65536,
                                           65536);
}
