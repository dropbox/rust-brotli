mod integration_tests;
mod tests;
extern crate brotli_no_stdlib as brotli;
extern crate core;
#[macro_use]
extern crate alloc_no_stdlib;
use core::ops;
use brotli::CustomRead;
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


#[allow(unused_imports)]
use alloc_no_stdlib::{Allocator, SliceWrapperMut, SliceWrapper};

//use alloc::{SliceWrapper,SliceWrapperMut, StackAllocator, AllocatedStackMemory, Allocator};
use brotli::{HuffmanCode};

use std::io::{self, Read, Write, ErrorKind, Error};

use std::env;

use std::fs::File;

use std::path::Path;


//declare_stack_allocator_struct!(MemPool, 4096, global);



struct IoWriterWrapper<'a, OutputType: Write + 'a>(&'a mut OutputType);


struct IoReaderWrapper<'a, OutputType: Read + 'a>(&'a mut OutputType);

impl<'a, OutputType: Write> brotli::CustomWrite<io::Error> for IoWriterWrapper<'a, OutputType> {
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

pub fn decompress<InputType, OutputType> (r : &mut InputType, mut w : &mut OutputType, buffer_size : usize) -> Result<(), io::Error>
where InputType: Read, OutputType: Write {
    let mut alloc_u8 = HeapAllocator::<u8>{default_value:0};
    let mut input_buffer = alloc_u8.alloc_cell(buffer_size);
    let mut output_buffer = alloc_u8.alloc_cell(buffer_size);
    return brotli::BrotliDecompressCustomIo(&mut IoReaderWrapper::<InputType>(r),
                                    &mut IoWriterWrapper::<OutputType>(w),
                                    input_buffer.slice_mut(), output_buffer.slice_mut(),
                                    alloc_u8,
                                    HeapAllocator::<u32>{default_value:0},
                                    HeapAllocator::<HuffmanCode>{default_value:HuffmanCode::default()},
                                    Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"));
}






// This decompressor is defined unconditionally on whether no-stdlib is defined
// so we can exercise the code in any case
pub struct BrotliDecompressor<R: Read>(brotli::DecompressorCustomIo<io::Error,
                                                            IntoIoReader<R>,
                                                            Rebox<u8>,
                                                            HeapAllocator<u8>, HeapAllocator<u32>, HeapAllocator<HuffmanCode> >);



impl<R: Read> BrotliDecompressor<R>
    {

    pub fn new(r: R, buffer_size : usize) -> Self {
        let mut alloc_u8 = HeapAllocator::<u8>{default_value : 0};
        let buffer = alloc_u8.alloc_cell(buffer_size);
        let alloc_u32 = HeapAllocator::<u32>{default_value:0};
        let alloc_hc = HeapAllocator::<HuffmanCode>{default_value:HuffmanCode::default()};
        return BrotliDecompressor::<R>(
          brotli::DecompressorCustomIo::<Error,
                                 IntoIoReader<R>,
                                 Rebox<u8>,
                                 HeapAllocator<u8>, HeapAllocator<u32>, HeapAllocator<HuffmanCode> >
                                 ::new(IntoIoReader::<R>(r),
                                                         buffer,
                                                         alloc_u8, alloc_u32, alloc_hc,
                                                         io::Error::new(ErrorKind::InvalidData,
                                                                        "Invalid Data")));
    }
}

impl<R: Read> Read for BrotliDecompressor<R> {
  	fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, Error> {
       return self.0.read(buf);
    }
}

#[cfg(test)]
fn writeln0<OutputType : Write> (strm : &mut OutputType, data: &str) -> core::result::Result<(), io::Error> {
   writeln!(strm, "{:}", data)
}
#[cfg(test)]
fn writeln_time<OutputType : Write> (strm : &mut OutputType, data: &str, v0 : u64, v1 : u32) -> core::result::Result<(), io::Error> {
   writeln!(strm, "{:} {:}.{:09}", data, v0, v1)
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
            match decompress(&mut input, &mut output, 65536) {
                Ok(_) => {},
                Err(e) => panic!("Error {:?}", e),
            }
            drop(output);
            drop(input);
        }
    } else {
        match decompress(&mut io::stdin(), &mut io::stdout(), 65536) {
            Ok(_) => return,
            Err(e) => panic!("Error {:?}", e),
        }
    }
}
