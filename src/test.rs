#![cfg(test)]
extern crate core;
use core::cmp;

use std::io;

struct Buffer {
    data : Vec<u8>,
    read_offset : usize,
}
impl Buffer {
    pub fn new(buf : &[u8]) -> Buffer {
        let mut ret = Buffer{
            data : Vec::<u8>::new(),
            read_offset : 0,
        };
        ret.data.extend(buf);
        return ret;
    }
}
impl io::Read for Buffer {
    fn read(self : &mut Self, buf : &mut [u8]) -> io::Result<usize> {
        let bytes_to_read = cmp::min(buf.len(), self.data.len() - self.read_offset);
        if bytes_to_read > 0 {
            buf[0..bytes_to_read].clone_from_slice(&self.data[self.read_offset ..
                                                                     self.read_offset + bytes_to_read]);
        }
        self.read_offset += bytes_to_read;
        return Ok(bytes_to_read);
    }
}
impl io::Write for Buffer {
    fn write(self : &mut Self, buf : &[u8]) -> io::Result<usize> {
        self.data.extend(buf);
        return Ok(buf.len());
    }
    fn flush(self : &mut Self) -> io::Result<()> {
        return Ok(());
    }
}


#[test]
fn test_10x_10y() {
    let in_buf : [u8;12] = [0x1b, 0x13, 0x00, 0x00, 0xa4, 0xb0, 0xb2, 0xea, 0x81, 0x47, 0x02, 0x8a];

    let mut output = Buffer::new(&[]);
    let mut input = super::Decompressor::new(Buffer::new(&in_buf));
    match super::copy_from_to(&mut input, &mut output) {
        Ok(_) => {},
        Err(e) => panic!("Error {:?}", e),
    }
    let mut i : usize = 0;
    while i < 10 {
      assert_eq!(output.data[i], 'X' as u8);
      assert_eq!(output.data[i + 10], 'Y' as u8);
      i += 1;
    }
    assert_eq!(output.data.len(), 20);
}


#[test]
fn test_alice() {
    let in_buf = include_bytes!("../testdata/alice29.txt.compressed");

    let mut output = Buffer::new(&[]);
    let mut input = super::Decompressor::new(Buffer::new(in_buf));
    match super::copy_from_to(&mut input, &mut output) {
        Ok(_) => {},
        Err(e) => panic!("Error {:?}", e),
    }
    let mut i : usize = 0;
    let truth = include_bytes!("../testdata/alice29.txt");
    while i < truth.len() {
      assert_eq!(output.data[i], truth[i]);
      i += 1;
    }
    assert_eq!(truth.len(), output.data.len());
}
