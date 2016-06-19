mod integration_tests;
extern crate brotli_no_stdlib as brotli;
extern crate core;
#[macro_use]
extern crate alloc_no_stdlib;
use core::ops;

pub struct Rebox<T> {
   b : Box<[T]>,
}

impl<T> core::default::Default for Rebox<T> {
    fn default() -> Self {
       let v : Vec<T> = Vec::new();
       let b = v.into_boxed_slice();
       return Rebox::<T>{b : b};
    }
}

impl<T> ops::Index<usize> for Rebox<T>{
    type Output = T;
    fn index(&self, index : usize) -> &T {
        return &(*self.b)[index]
    }
}

impl<T> ops::IndexMut<usize> for Rebox<T>{
    fn index_mut(&mut self, index : usize) -> &mut T {
        return &mut (*self.b)[index]
    }
}

impl<T> alloc_no_stdlib::SliceWrapper<T> for Rebox<T> {
    fn slice(&self) -> & [T] {
       return &*self.b
    }
}

impl<T> alloc_no_stdlib::SliceWrapperMut<T> for Rebox<T> {
    fn slice_mut(&mut self) -> &mut [T] {
       return &mut*self.b
    }
}

pub struct HeapAllocator<T : core::clone::Clone>{
   pub default_value : T,
}

#[cfg(not(feature="unsafe"))]
impl<T : core::clone::Clone> alloc_no_stdlib::Allocator<T> for HeapAllocator<T> {
   type AllocatedMemory = Rebox<T>;
   fn alloc_cell(self : &mut HeapAllocator<T>, len : usize) -> Rebox<T> {
       let v : Vec<T> = vec![self.default_value.clone();len];
       let b = v.into_boxed_slice();
       return Rebox::<T>{b : b};
   }
   fn free_cell(self : &mut HeapAllocator<T>, _data : Rebox<T>) {

   }
}

#[cfg(feature="unsafe")]
impl<T : core::clone::Clone> alloc_no_stdlib::Allocator<T> for HeapAllocator<T> {
   type AllocatedMemory = Rebox<T>;
   fn alloc_cell(self : &mut HeapAllocator<T>, len : usize) -> Rebox<T> {
       let mut v : Vec<T> = Vec::with_capacity(len);
       unsafe{v.set_len(len);}
       let b = v.into_boxed_slice();
       return Rebox::<T>{b : b};
   }
   fn free_cell(self : &mut HeapAllocator<T>, _data : Rebox<T>) {

   }
}



use alloc_no_stdlib::{Allocator, SliceWrapperMut, SliceWrapper,
            /*StackAllocator, AllocatedStackMemory, uninitialized*/};

//use alloc::{SliceWrapper,SliceWrapperMut, StackAllocator, AllocatedStackMemory, Allocator};
use brotli::{BrotliDecompressStream, BrotliState, BrotliResult, HuffmanCode};

use std::io::{self, Read, Write, ErrorKind, Error};
use std::time::Duration;
use std::env;

use std::fs::File;

use std::path::Path;

#[cfg(not(feature="disable-timer"))]
use std::time::SystemTime;

#[cfg(feature="disable-timer")]
fn now() -> Duration {
    return Duration::new(0, 0);
}
#[cfg(not(feature="disable-timer"))]
fn now() -> SystemTime {
    return SystemTime::now();
}

#[cfg(not(feature="disable-timer"))]
fn elapsed(start : SystemTime) -> (Duration, bool) {
    match start.elapsed() {
        Ok(delta) => return (delta, false),
        _ => return (Duration::new(0, 0), true),
    }
}

#[cfg(feature="disable-timer")]
fn elapsed(_start : Duration) -> (Duration, bool) {
    return (Duration::new(0, 0), true);
}

//declare_stack_allocator_struct!(MemPool, 4096, global);


fn _write_all<OutputType> (w : &mut OutputType, buf : &[u8]) -> Result<(), io::Error>
where OutputType: Write {
    let mut total_written : usize = 0;
    while total_written < buf.len() {
        match w.write(&buf[total_written..]) {
            Err(e) => {
                match e.kind() {
                    ErrorKind::Interrupted => continue,
                    _ => return Err(e),
                }
            },
            Ok(cur_written) => {
                if cur_written == 0 {
                     return Err(Error::new(ErrorKind::UnexpectedEof, "Write EOF"));
                }
                total_written += cur_written;
            }
        }
    }
    Ok(())
}

//trace_macros!(true);
//declare_stack_allocator_struct!(GlobalAllocatedFreelist, 4096, global);
//define_allocator_memory_pool!(global_u8_buffer, 4096, u8, [0; 1024 * 1024 * 100], global);
//define_allocator_memory_pool!(global_u32_buffer, 4096, u32, [0; 1024 * 1024 * 100], global);
//define_allocator_memory_pool!(global_hc_buffer, 4096, ::brotli::HuffmanCode, [::brotli::HuffmanCode{value : 0, bits :0}; 1024 * 1024 * 100], global);
  
/*  
extern {
  fn calloc(n_elem : usize, el_size : usize) -> *mut u8;
  fn malloc(len : usize) -> *mut u8;
  fn free(item : *mut u8);
}
*/
pub fn decompress<InputType, OutputType> (r : &mut InputType, mut w : &mut OutputType) -> Result<(), io::Error>
where InputType: Read, OutputType: Write {
    return decompress_internal(r, w, 4096 * 1024, 4096 * 1024);
}
pub fn decompress_internal<InputType, OutputType> (r : &mut InputType, mut w : &mut OutputType, input_buffer_limit : usize, output_buffer_limit : usize) -> Result<(), io::Error>
where InputType: Read, OutputType: Write {
  let mut total = Duration::new(0, 0);
  let range : usize;
  let mut timing_error : bool = false;
  if option_env!("BENCHMARK_MODE").is_some() {
    range = 1000;
  } else {
    range = 1;
  }
  for _i in 0..range {
    //let calloc_u8_allocator = MemPool::<u8>::new_allocator(&mut calloc_u8_buffer.data, bzero);
    //let calloc_u32_allocator = MemPool::<u32>::new_allocator(&mut calloc_u32_buffer.data, bzero);
    //let calloc_hc_allocator = MemPool::<HuffmanCode>::new_allocator(&mut calloc_hc_buffer.data, bzero);
    //test(calloc_u8_allocator);
    let mut brotli_state = BrotliState::new(HeapAllocator::<u8>{default_value:0},HeapAllocator::<u32>{default_value:0},HeapAllocator::<HuffmanCode>{default_value:HuffmanCode::default()});
    let mut input = brotli_state.alloc_u8.alloc_cell(input_buffer_limit);
    let mut output = brotli_state.alloc_u8.alloc_cell(output_buffer_limit);
    let mut available_out : usize = output.slice().len();

    //let amount = try!(r.read(&mut buf));
    let mut available_in : usize = 0;
    let mut input_offset : usize = 0;
    let mut output_offset : usize = 0;
    let mut result : BrotliResult = BrotliResult::NeedsMoreInput;
    loop {
      match result {
          BrotliResult::NeedsMoreInput => {
              input_offset = 0;
              match r.read(input.slice_mut()) {
                  Err(e) => {
                      match e.kind() {
                          ErrorKind::Interrupted => continue,
                          _ => return Err(e),
                      }
                  },
                  Ok(size) => {
                      if size == 0 {
                          return Err(Error::new(ErrorKind::UnexpectedEof, "Read EOF"));
                      }
                      available_in = size;
                  },
              }
          },
          BrotliResult::NeedsMoreOutput => {
              try!(_write_all(&mut w, &output.slice()[..output_offset]));
              output_offset = 0;
          },
          BrotliResult::ResultSuccess => break,
          BrotliResult::ResultFailure => panic!("FAILURE"),
      }
      let mut written :usize = 0;
      let start = now();
      result = BrotliDecompressStream(&mut available_in, &mut input_offset, &input.slice(),
                                      &mut available_out, &mut output_offset, &mut output.slice_mut(),
                                      &mut written, &mut brotli_state);

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
      let _r = writeln!(&mut std::io::stderr(), "Timing error\n");
  } else {
      let _r = writeln!(&mut std::io::stderr(), "Time {:}.{:09}\n",
                        total.as_secs(),
                        total.subsec_nanos());
  }
  Ok(())
}

fn main() {
    if env::args_os().len() > 1 {
        let mut first = true;
        for argument in env::args() {
            if first {
               first = false;
               continue;
            }
            let mut input = match File::open(&Path::new(&argument)) {
                Err(why) => panic!("couldn't open {}: {:?}", argument,
                                                       why),
                Ok(file) => file,
            };
            let oa = argument + ".original";
            let mut output = match File::create(&Path::new(&oa), ) {
                Err(why) => panic!("couldn't open file for writing: {:} {:?}", oa, why),
                Ok(file) => file,
            };
            match decompress(&mut input, &mut output) {
                Ok(_) => {},
                Err(e) => panic!("Error {:?}", e),
            }
            drop(output);
            drop(input);
        }
    } else {
        match decompress(&mut io::stdin(), &mut io::stdout()) {
            Ok(_) => return,
            Err(e) => panic!("Error {:?}", e),
        }
    }
}
