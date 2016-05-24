extern crate core;

extern crate brotli;
use brotli::{Decompressor, copy_from_to};

use std::io;
use std::io::Write;
macro_rules! stderr {
    ($($arg:tt)*) => (
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr (file handle closed?): {}", x),
        }
    )
}


fn main() {
    let r = Decompressor::new(io::stdin());
    let w = io::stdout();
    match copy_from_to(r, w) {
        Ok(_) => return,
        Err(e) => {
            stderr!("Output Error {}\n", e);
            std::process::exit(1);
        }
    }
}
