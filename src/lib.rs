#[macro_use]
extern crate alloc_no_stdlib as alloc;
extern crate brotli_no_stdlib;
extern crate core;
use std::io;
mod heap_alloc;
mod test;
use heap_alloc::{HeapAllocator};
use brotli_no_stdlib::{HuffmanCode, BrotliState, BrotliDecompressStream, BrotliResult};


pub struct Decompressor<R: io::Read> {
    input_buffer : [u8;65536],
    total_out : usize,
    input_offset : usize,
    input_len : usize,
    input_eof : bool,
    input: R,
    state : BrotliState<HeapAllocator<u8>, HeapAllocator<u32>, HeapAllocator<HuffmanCode> >,
}
macro_rules! stderr {
    ($($arg:tt)*) => (
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr (file handle closed?): {}", x),
        }
    )
}

impl<R: io::Read> Decompressor<R> {

    pub fn new(r: R) -> Decompressor<R> {
        let ret = Decompressor{
            input_buffer : [0; 65536],
            total_out : 0,
            input_offset : 0,
            input_len : 0,
            input_eof : false,
            input: r,
            state : BrotliState::new(HeapAllocator::<u8>{default_value : 0u8},
                                     HeapAllocator::<u32>{default_value : 0u32},
                                     HeapAllocator::<HuffmanCode>{
                                         default_value : HuffmanCode::default()}),
        };
        return ret;
    }

    pub fn copy_to_front(&mut self) {
        if self.input_offset == self.input_buffer.len() {
            self.input_offset = 0;
            self.input_len = 0;
        } else if self.input_offset + 256 > self.input_buffer.len() {
            let (mut first, second) = self.input_buffer[..].split_at_mut(self.input_offset);
            let avail_in = self.input_len - self.input_offset;
            first[0..avail_in].clone_from_slice(&second[0..avail_in]);
            self.input_offset = 0;
        }
    }
}
impl<'a, R: io::Read> io::Read for Decompressor<R> {
	fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
            let mut output_offset : usize = 0;
            let mut avail_out = buf.len() - output_offset;
            let mut avail_in = self.input_len - self.input_offset;
            let mut needs_input = false;
            while avail_out == buf.len() && (needs_input == false || self.input_eof == false) {
                    if !self.input_eof {
                        match self.input.read(&mut self.input_buffer[self.input_len..]) {
                            Err(e) => match e.kind() {
                                io::ErrorKind::Interrupted => continue,
                                _ => self.input_eof = true,
                            },
                            Ok(size) => if size == 0 {
                                //self.input_eof=true;
                            }else {
                                needs_input = false;
                                self.input_len += size;
                                avail_in = self.input_len - self.input_offset;
                            },
                        }
                    }
                    match BrotliDecompressStream(&mut avail_in,
                                                  &mut self.input_offset,
                                                  &self.input_buffer[..],
                                                  &mut avail_out,
                                                  &mut output_offset,
                                                  buf,
                                                  &mut self.total_out,
                                                  &mut self.state) {
                        BrotliResult::NeedsMoreInput => {needs_input = true; self.copy_to_front();},
                        BrotliResult::NeedsMoreOutput => {},
                        BrotliResult::ResultSuccess => break,
                        BrotliResult::ResultFailure => return Err(io::Error::new(io::ErrorKind::InvalidData,
                                                      "Invalid Data")),
                  }
            }
            return Ok(output_offset);
        }
}

pub fn copy_from_to<R:io::Read, W:io::Write>(mut r : R, mut w : W) -> io::Result<usize> {
    let mut buffer : [u8;65536] = [0; 65536];
    let mut out_size : usize = 0;
    loop {
        match r.read(&mut buffer[..]) {
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::Interrupted => continue,
                     _ => {},
                }
                return Err(e)
            },
            Ok(size) => if size == 0 {
               break;
            } else {
                match w.write_all(&buffer[..size]) {
                    Err(e) => {
                        match e.kind() {
                            io::ErrorKind::Interrupted => continue,
                            _ => {},
                        }
                        return Err(e)
                    }
                    Ok(_) => out_size += size,
                }
            },
        }
    }
    return Ok(out_size);
}

