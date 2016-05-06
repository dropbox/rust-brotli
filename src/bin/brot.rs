extern crate core;

extern crate brotli;
use brotli::Decompressor;

use std::io;
use std::io::{Read, Write};
macro_rules! stderr {
    ($($arg:tt)*) => (
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr (file handle closed?): {}", x),
        }
    )
}


pub fn decompress_from_to<R:Read, W:Write>(mut r : R, mut w : W) -> io::Result<usize> {
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

fn main() {
    let r = Decompressor::new(io::stdin());
    let w = io::stdout();
    match decompress_from_to(r, w) {
        Ok(_) => return,
        Err(e) => {
            stderr!("Output Error {}\n", e);
            std::process::exit(1);
        }
    }
}
