
#![cfg_attr(feature="benchmark", feature(test))]

mod integration_tests;
mod tests;
mod util;

extern crate brotli;
extern crate brotli_decompressor;
extern crate core;
#[allow(unused_imports)]
#[macro_use]
extern crate alloc_no_stdlib;
use brotli::CustomRead;
use core::ops;
use brotli::enc::cluster::HistogramPair;
use brotli::enc::ZopfliNode;
use brotli::enc::StaticCommand;
use brotli::enc::backward_references::BrotliEncoderMode;
use brotli::enc::command::Command;
use brotli::enc::entropy_encode::HuffmanTree;
use brotli::enc::histogram::{ContextType, HistogramLiteral, HistogramCommand, HistogramDistance};


pub struct Rebox<T> {
  b: Box<[T]>,
}

impl<T> core::default::Default for Rebox<T> {
  fn default() -> Self {
    let v: Vec<T> = Vec::new();
    let b = v.into_boxed_slice();
    Rebox::<T> { b: b }
  }
}

impl<T> ops::Index<usize> for Rebox<T> {
  type Output = T;
  fn index(&self, index: usize) -> &T {
    &(*self.b)[index]
  }
}

impl<T> ops::IndexMut<usize> for Rebox<T> {
  fn index_mut(&mut self, index: usize) -> &mut T {
    &mut (*self.b)[index]
  }
}

impl<T> alloc_no_stdlib::SliceWrapper<T> for Rebox<T> {
  fn slice(&self) -> &[T] {
    &*self.b
  }
}

impl<T> alloc_no_stdlib::SliceWrapperMut<T> for Rebox<T> {
  fn slice_mut(&mut self) -> &mut [T] {
    &mut *self.b
  }
}

pub struct HeapAllocator<T: core::clone::Clone> {
  pub default_value: T,
}

//#[cfg(not(feature="unsafe"))]
impl<T: core::clone::Clone> alloc_no_stdlib::Allocator<T> for HeapAllocator<T> {
  type AllocatedMemory = Rebox<T>;
  fn alloc_cell(self: &mut HeapAllocator<T>, len: usize) -> Rebox<T> {
    let v: Vec<T> = vec![self.default_value.clone();len];
    let b = v.into_boxed_slice();
    Rebox::<T> { b: b }
  }
  fn free_cell(self: &mut HeapAllocator<T>, _data: Rebox<T>) {}
}
/* FAILS test: compressor must fail to initialize data first
#[cfg(feature="unsafe")]
impl<T: core::clone::Clone> alloc_no_stdlib::Allocator<T> for HeapAllocator<T> {
  type AllocatedMemory = Rebox<T>;
  fn alloc_cell(self: &mut HeapAllocator<T>, len: usize) -> Rebox<T> {
    let mut v: Vec<T> = Vec::with_capacity(len);
    unsafe {
      v.set_len(len);
    }
    let b = v.into_boxed_slice();
    Rebox::<T> { b: b }
  }
  fn free_cell(self: &mut HeapAllocator<T>, _data: Rebox<T>) {}
}
*/

#[allow(unused_imports)]
use alloc_no_stdlib::{SliceWrapper, SliceWrapperMut, StackAllocator, AllocatedStackMemory,
                      Allocator, bzero};
use brotli_decompressor::HuffmanCode;

use std::env;

use std::fs::File;

use std::io::{self, Error, ErrorKind, Read, Write, Seek, SeekFrom};

macro_rules! println_stderr(
    ($($val:tt)*) => { {
        writeln!(&mut ::std::io::stderr(), $($val)*).unwrap();
    } }
);

use std::path::Path;


// declare_stack_allocator_struct!(MemPool, 4096, global);



struct IoWriterWrapper<'a, OutputType: Write + 'a>(&'a mut OutputType);


struct IoReaderWrapper<'a, OutputType: Read + 'a>(&'a mut OutputType);

impl<'a, OutputType: Write> brotli::CustomWrite<io::Error> for IoWriterWrapper<'a, OutputType> {
  fn flush(self: &mut Self) -> Result<(), io::Error> {
    loop {
      match self.0.flush() {
        Err(e) => {
          match e.kind() {
            ErrorKind::Interrupted => continue,
            _ => return Err(e),
          }
        }
        Ok(_) => return Ok(()),
      }
    }
  }
  fn write(self: &mut Self, buf: &[u8]) -> Result<usize, io::Error> {
    loop {
      match self.0.write(buf) {
        Err(e) => {
          match e.kind() {
            ErrorKind::Interrupted => continue,
            _ => return Err(e),
          }
        }
        Ok(cur_written) => return Ok(cur_written),
      }
    }
  }
}


impl<'a, InputType: Read> brotli::CustomRead<io::Error> for IoReaderWrapper<'a, InputType> {
  fn read(self: &mut Self, buf: &mut [u8]) -> Result<usize, io::Error> {
    loop {
      match self.0.read(buf) {
        Err(e) => {
          match e.kind() {
            ErrorKind::Interrupted => continue,
            _ => return Err(e),
          }
        }
        Ok(cur_read) => return Ok(cur_read),
      }
    }
  }
}

struct IntoIoReader<OutputType: Read>(OutputType);

impl<InputType: Read> brotli::CustomRead<io::Error> for IntoIoReader<InputType> {
  fn read(self: &mut Self, buf: &mut [u8]) -> Result<usize, io::Error> {
    loop {
      match self.0.read(buf) {
        Err(e) => {
          match e.kind() {
            ErrorKind::Interrupted => continue,
            _ => return Err(e),
          }
        }
        Ok(cur_read) => return Ok(cur_read),
      }
    }
  }
}
#[cfg(not(feature="seccomp"))]
pub fn decompress<InputType, OutputType>(r: &mut InputType,
                                         w: &mut OutputType,
                                         buffer_size: usize)
                                         -> Result<(), io::Error>
  where InputType: Read,
        OutputType: Write
{
  let mut alloc_u8 = HeapAllocator::<u8> { default_value: 0 };
  let mut input_buffer = alloc_u8.alloc_cell(buffer_size);
  let mut output_buffer = alloc_u8.alloc_cell(buffer_size);
  brotli::BrotliDecompressCustomIo(&mut IoReaderWrapper::<InputType>(r),
                                   &mut IoWriterWrapper::<OutputType>(w),
                                   input_buffer.slice_mut(),
                                   output_buffer.slice_mut(),
                                   alloc_u8,
                                   HeapAllocator::<u32> { default_value: 0 },
                                   HeapAllocator::<HuffmanCode> {
                                     default_value: HuffmanCode::default(),
                                   },
                                   Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"))
}
#[cfg(feature="seccomp")]
extern "C" {
  fn calloc(n_elem: usize, el_size: usize) -> *mut u8;
  fn free(ptr: *mut u8);
  fn syscall(value: i32) -> i32;
  fn prctl(operation: i32, flags: u32) -> i32;
}
#[cfg(feature="seccomp")]
const PR_SET_SECCOMP: i32 = 22;
#[cfg(feature="seccomp")]
const SECCOMP_MODE_STRICT: u32 = 1;

#[cfg(feature="seccomp")]
declare_stack_allocator_struct!(CallocAllocatedFreelist, 8192, calloc);

#[cfg(feature="seccomp")]
pub fn decompress<InputType, OutputType>(r: &mut InputType,
                                         mut w: &mut OutputType,
                                         buffer_size: usize)
                                         -> Result<(), io::Error>
  where InputType: Read,
        OutputType: Write
{

  let mut u8_buffer =
    unsafe { define_allocator_memory_pool!(4, u8, [0; 1024 * 1024 * 200], calloc) };
  let mut u32_buffer = unsafe { define_allocator_memory_pool!(4, u32, [0; 16384], calloc) };
  let mut hc_buffer =
    unsafe { define_allocator_memory_pool!(4, HuffmanCode, [0; 1024 * 1024 * 16], calloc) };
  let mut alloc_u8 = CallocAllocatedFreelist::<u8>::new_allocator(u8_buffer.data, bzero);
  let alloc_u32 = CallocAllocatedFreelist::<u32>::new_allocator(u32_buffer.data, bzero);
  let alloc_hc = CallocAllocatedFreelist::<HuffmanCode>::new_allocator(hc_buffer.data, bzero);
  let ret = unsafe { prctl(PR_SET_SECCOMP, SECCOMP_MODE_STRICT) };
  if ret != 0 {
    panic!("Unable to activate seccomp");
  }
  match brotli::BrotliDecompressCustomIo(&mut IoReaderWrapper::<InputType>(r),
                                         &mut IoWriterWrapper::<OutputType>(w),
                                         &mut alloc_u8.alloc_cell(buffer_size).slice_mut(),
                                         &mut alloc_u8.alloc_cell(buffer_size).slice_mut(),
                                         alloc_u8,
                                         alloc_u32,
                                         alloc_hc,
                                         Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF")) {
    Err(e) => Err(e),
    Ok(()) => {
        unsafe{syscall(60);};
        unreachable!()
      }
  }
}

pub fn compress<InputType, OutputType>(r: &mut InputType,
                                       w: &mut OutputType,
                                       buffer_size: usize,
                                       params:&brotli::enc::BrotliEncoderParams) -> Result<usize, io::Error>
    where InputType: Read,
          OutputType: Write {
    let mut alloc_u8 = HeapAllocator::<u8> { default_value: 0 };
    let mut input_buffer = alloc_u8.alloc_cell(buffer_size);
    let mut output_buffer = alloc_u8.alloc_cell(buffer_size);
    let mut log = |pm:&mut brotli::interface::PredictionModeContextMap<brotli::InputReferenceMut>,
                   data:&mut [brotli::interface::Command<brotli::SliceOffset>],
                   mb:brotli::InputPair| {
        let tmp = brotli::interface::Command::PredictionMode(
            brotli::interface::PredictionModeContextMap::<brotli::InputReference>{
                literal_context_map:brotli::InputReference::from(&pm.literal_context_map),
                predmode_speed_and_distance_context_map:brotli::InputReference::from(&pm.predmode_speed_and_distance_context_map),
            });
        util::write_one(&tmp);
        for cmd in data.iter() {
            util::write_one(&brotli::thaw_pair(cmd, &mb));
        }
    };
    if params.log_meta_block {
        println_stderr!("window {} 0 0 0", params.lgwin);
    }
    brotli::BrotliCompressCustomIo(&mut IoReaderWrapper::<InputType>(r),
                                   &mut IoWriterWrapper::<OutputType>(w),
                                   &mut input_buffer.slice_mut(),
                                   &mut output_buffer.slice_mut(),
                                   params,
                                   alloc_u8,
                                   HeapAllocator::<u16>{default_value:0},
                                   HeapAllocator::<i32>{default_value:0},
                                   HeapAllocator::<u32>{default_value:0},
                                   HeapAllocator::<u64>{default_value:0},
                                   HeapAllocator::<Command>{default_value:Command::default()},
                                   HeapAllocator::<brotli::enc::floatX>{default_value:0.0 as brotli::enc::floatX},
                                   HeapAllocator::<brotli::enc::Mem256f>{default_value:brotli::enc::Mem256f::default()},
                                   HeapAllocator::<brotli::enc::PDF>{default_value:brotli::enc::PDF::default()},
                                   HeapAllocator::<StaticCommand>{default_value:StaticCommand::default()},
                                   HeapAllocator::<HistogramLiteral>{
                                       default_value:HistogramLiteral::default(),
                                   },
                                   HeapAllocator::<HistogramCommand>{
                                       default_value:HistogramCommand::default(),
                                   },
                                   HeapAllocator::<HistogramDistance>{
                                       default_value:HistogramDistance::default(),
                                   },
                                   HeapAllocator::<HistogramPair>{
                                       default_value:HistogramPair::default(),
                                   },
                                   HeapAllocator::<ContextType>{
                                       default_value:ContextType::default(),
                                   },
                                   HeapAllocator::<HuffmanTree>{
                                       default_value:HuffmanTree::default(),
                                   },
                                   HeapAllocator::<ZopfliNode>{
                                       default_value:ZopfliNode::default(),
                                   },
                                   &mut log,
                                   Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"))
}

// This decompressor is defined unconditionally on whether no-stdlib is defined
// so we can exercise the code in any case
pub struct BrotliDecompressor<R: Read>(brotli::DecompressorCustomIo<io::Error,
                                                                    IntoIoReader<R>,
                                                                    Rebox<u8>,
                                                                    HeapAllocator<u8>,
                                                                    HeapAllocator<u32>,
                                                                    HeapAllocator<HuffmanCode>>);



impl<R: Read> BrotliDecompressor<R> {
  pub fn new(r: R, buffer_size: usize) -> Self {
    let mut alloc_u8 = HeapAllocator::<u8> { default_value: 0 };
    let buffer = alloc_u8.alloc_cell(buffer_size);
    let alloc_u32 = HeapAllocator::<u32> { default_value: 0 };
    let alloc_hc = HeapAllocator::<HuffmanCode> { default_value: HuffmanCode::default() };
    BrotliDecompressor::<R>(
          brotli::DecompressorCustomIo::<Error,
                                 IntoIoReader<R>,
                                 Rebox<u8>,
                                 HeapAllocator<u8>, HeapAllocator<u32>, HeapAllocator<HuffmanCode> >
                                 ::new(IntoIoReader::<R>(r),
                                                         buffer,
                                                         alloc_u8, alloc_u32, alloc_hc,
                                                         io::Error::new(ErrorKind::InvalidData,
                                                                        "Invalid Data")))
  }
}

impl<R: Read> Read for BrotliDecompressor<R> {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
    self.0.read(buf)
  }
}

#[cfg(test)]
fn writeln0<OutputType: Write>(strm: &mut OutputType,
                               data: &str)
                               -> core::result::Result<(), io::Error> {
  writeln!(strm, "{:}", data)
}
#[cfg(test)]
fn writeln_time<OutputType: Write>(strm: &mut OutputType,
                                   data: &str,
                                   v0: u64,
                                   v1: u64,
                                   v2: u32)
                                   -> core::result::Result<(), io::Error> {
  writeln!(strm, "{:} {:} {:}.{:09}", v0, data, v1, v2)
}

fn main() {
  let mut do_compress = false;
  let mut params = brotli::enc::BrotliEncoderInitParams();
  params.quality = 11; // default
  let mut filenames = [std::string::String::new(), std::string::String::new()];
  let mut num_benchmarks = 1;
  if env::args_os().len() > 1 {
    let mut first = true;
    for argument in env::args() {
      if first {
        first = false;
        continue;
      }
      if argument == "--dump-dictionary" {
        util::print_dictionary(util::permute_dictionary());
        return
      }
      if argument == "-utf8" {
          params.mode = BrotliEncoderMode::BROTLI_FORCE_UTF8_PRIOR;
          continue;
      }
      if argument == "-msb" {
          params.mode = BrotliEncoderMode::BROTLI_FORCE_MSB_PRIOR;
          continue;
      }
      if argument == "-lsb" {
          params.mode = BrotliEncoderMode::BROTLI_FORCE_LSB_PRIOR;
          continue;
      }
      if argument == "-signed" {
          params.mode = BrotliEncoderMode::BROTLI_FORCE_SIGNED_PRIOR;
          continue;
      }
      if argument == "-i" {
        // display the intermediate representation of metablocks
        params.log_meta_block = true;
        continue;
      }
      if argument == "-0" || argument == "-q0" {
        params.quality = 0;
        continue;
      }
      if argument == "-1" || argument == "-q1" {
        params.quality = 1;
        continue;
      }
      if argument == "-2" || argument == "-q2" {
        params.quality = 2;
        continue;
      }
      if argument == "-3" || argument == "-q3" {
        params.quality = 3;
        continue;
      }
      if argument == "-4" || argument == "-q4" {
        params.quality = 4;
        continue;
      }
      if argument == "-5" || argument == "-q5" {
        params.quality = 5;
        continue;
      }
      if argument == "-6" || argument == "-q6" {
        params.quality = 6;
        continue;
      }
      if argument == "-7" || argument == "-q7" {
        params.quality = 7;
        continue;
      }
      if argument == "-8" || argument == "-q8" {
        params.quality = 8;
        continue;
      }
      if argument == "-9" || argument == "-q9" {
        params.quality = 9;
        continue;
      }
      if argument == "-9.5" || argument == "-q9.5" {
        params.quality = 10;
        params.q9_5 = true;
        continue;
      }
      if argument == "-9.5x" || argument == "-q9.5x" {
        params.quality = 11;
        params.q9_5 = true;
        continue;
      }
      if argument == "-10" || argument == "-q10" {
        params.quality = 10;
        continue;
      }
      if argument == "-11" || argument == "-q11" {
        params.quality = 11;
        continue;
      }
      if argument == "-q9.5y" {
          params.quality = 12;
          params.q9_5 = true;
        continue;
      }
      if argument.starts_with("-l") {
        params.lgblock = argument.trim_matches('-').trim_matches('l').parse::<i32>().unwrap();
        continue;
      }
      if argument.starts_with("-bytescore=") {
        params.hasher.literal_byte_score = argument.trim_matches('-').trim_matches('b').trim_matches('y').trim_matches('t').trim_matches('e').trim_matches('s').trim_matches('c').trim_matches('o').trim_matches('r').trim_matches('e').trim_matches('=').parse::<i32>().unwrap();
        continue;
      }
      if argument.starts_with("-w") {
          params.lgwin = argument.trim_matches('-').trim_matches('w').parse::<i32>().unwrap();
          continue;
      }
      if argument.starts_with("-l") {
          params.lgblock = argument.trim_matches('-').trim_matches('l').parse::<i32>().unwrap();
          continue;
      }
      if argument.starts_with("-findprior") {
          params.prior_bitmask_detection = 1;
          continue;
      }
      if argument.starts_with("-findspeed=") {
          params.cdf_adaptation_detection = argument.trim_matches('-').trim_matches('f').trim_matches('i').trim_matches('n').trim_matches('d').trim_matches('r').trim_matches('a').trim_matches('n').trim_matches('d').trim_matches('o').trim_matches('m').trim_matches('=').parse::<u32>().unwrap() as u8;
          continue;
      } else if argument == "-findspeed" {
          params.cdf_adaptation_detection = 1;
          continue;
      }
      if argument == "-basicstride" {
          params.stride_detection_quality = 1;
          continue;
      } else if argument == "-advstride" {
          params.stride_detection_quality = 3;
          continue;
      } else {
          if argument == "-stride" {
              params.stride_detection_quality = 2;
              continue;
          } else {
              if argument.starts_with("-s") && !argument.starts_with("-speed=") {
                  params.size_hint = argument.trim_matches('-').trim_matches('s').parse::<usize>().unwrap();
                  continue;
              }
          }
      }
      if argument.starts_with("-speed=") {
          let comma_string = argument.trim_matches('-').trim_matches('s').trim_matches('p').trim_matches('e').trim_matches('e').trim_matches('d').trim_matches('=');
          let split = comma_string.split(",");
          for (index, s) in split.enumerate() {
              let data = s.parse::<u16>().unwrap();
              if data > 16384 {
                  println_stderr!("Speed must be <= 16384, not {}", data);
              }
              if index == 0 {
                  for item in params.literal_adaptation.iter_mut() {
                      item.0 = data;
                  }
              } else if index == 1 {
                  for item in params.literal_adaptation.iter_mut() {
                      item.1 = data;
                  }
              } else {
                  if (index & 1) == 0 {
                      params.literal_adaptation[index / 2].0 = data;
                  }else {
                      params.literal_adaptation[index / 2].1 = data;
                  }
              }
          }
          continue;
      }
      if argument == "-avoiddistanceprefixsearch" {
          params.avoid_distance_prefix_search = true;
      }
      if argument.starts_with("-b") {
          num_benchmarks = argument.trim_matches('-').trim_matches('b').parse::<usize>().unwrap();
          continue;
      }
      if argument == "-c" {
        do_compress = true;
        continue;
      }
      if argument == "-h" || argument == "-help" || argument == "--help" {
        println_stderr!("Decompression:\nbrotli [input_file] [output_file]\nCompression:brotli -c -q9.5 -w22 [input_file] [output_file]\nQuality may be one of -q9.5 -q9.5x -q9.5y or -q[0-11] for standard brotli settings.\nOptional size hint -s<size> to direct better compression\n\nThe -i parameter produces a cross human readdable IR representation of the file.\nThis can be ingested by other compressors.\nIR-specific options include:\n-findprior\n-speed=<inc,max,inc,max,inc,max,inc,max>");
        return;
      }
      if filenames[0] == "" {
         filenames[0] = argument.clone();
         continue;
      }
      if filenames[1] == "" {
         filenames[1] = argument.clone();
         continue;
      }
      panic!("Unknown Argument {:}", argument);
   }
   if filenames[0] != "" {
      let mut input = match File::open(&Path::new(&filenames[0])) {
        Err(why) => panic!("couldn't open {:}\n{:}", filenames[0], why),
        Ok(file) => file,
      };
      if filenames[1] != "" {
        let mut output = match File::create(&Path::new(&filenames[1])) {
          Err(why) => panic!("couldn't open file for writing: {:}\n{:}", filenames[1], why),
          Ok(file) => file,
        };
        for i in 0..num_benchmarks {
          if do_compress {
            match compress(&mut input, &mut output, 65536, &params) {
                Ok(_) => {}
                Err(e) => panic!("Error {:?}", e),
            }
          } else {
            match decompress(&mut input, &mut output, 65536) {
              Ok(_) => {}
              Err(e) => panic!("Error: {:} during brotli decompress\nTo compress with Brotli, specify the -c flag.", e),
            }
          }
          if i + 1 != num_benchmarks {
              input.seek(SeekFrom::Start(0)).unwrap();
              output.seek(SeekFrom::Start(0)).unwrap();
          }
        }
        drop(output);
      } else {
        assert_eq!(num_benchmarks, 1);
        if do_compress {
          match compress(&mut input, &mut io::stdout(), 65536, &params) {
            Ok(_) => {}
            Err(e) => panic!("Error {:?}", e),
          }
        } else {
          match decompress(&mut input, &mut io::stdout(), 65536) {
            Ok(_) => {}
            Err(e) => panic!("Error: {:} during brotli decompress\nTo compress with Brotli, specify the -c flag.", e),
          }
        }
      }
      drop(input);
   } else {
      assert_eq!(num_benchmarks, 1);
      if do_compress {
        match compress(&mut io::stdin(), &mut io::stdout(), 65536, &params) {
          Ok(_) => return,
          Err(e) => panic!("Error {:?}", e),
        }
      } else {
        match decompress(&mut io::stdin(), &mut io::stdout(), 65536) {
          Ok(_) => return,
          Err(e) => panic!("Error: {:} during brotli decompress\nTo compress with Brotli, specify the -c flag.", e),
        }
      }
    }
  } else {
    assert_eq!(num_benchmarks, 1);
    match decompress(&mut io::stdin(), &mut io::stdout(), 65536) {
      Ok(_) => return,
      Err(e) => panic!("Error: {:} during brotli decompress\nTo compress with Brotli, specify the -c flag.", e),
    }
  }
}
