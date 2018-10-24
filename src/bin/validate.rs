use super::BrotliEncoderParams;
use std::io::{self, Error, ErrorKind, Read, Write};
use super::{Rebox, HeapAllocator, IoWriterWrapper};
use brotli::{DecompressorWriterCustomIo, CustomWrite};
use alloc_no_stdlib::{Allocator, SliceWrapper};



struct Tee<OutputA:Write, OutputB:Write>(OutputA, OutputB);
impl <OutputA:Write, OutputB:Write> Write for Tee<OutputA, OutputB> {
    fn write(&mut self, data:&[u8]) -> Result<usize, io::Error> {
        match self.0.write(data) {
            Err(err) => return Err(err),
            Ok(size) => match self.1.write_all(&data[..size]) {
                Ok(_) => Ok(size),
                Err(err) => Err(err),
            }
        }
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        match self.0.flush() {
            Err(err) => return Err(err),
            Ok(_) => {
                loop {
                    match self.1.flush() {
                        Err(e) => match e.kind() {
                            ErrorKind::Interrupted => continue,
                            _ => return Err(e),
                        }
                        Ok(e) => return Ok(e),
                    }
                }
            },
        }
    }
}

struct DecompressAndValidate<'a, OutputType:Write+'a>(
    DecompressorWriterCustomIo<io::Error,
                               IoWriterWrapper<'a, OutputType>,
                               Rebox<u8>, // buffer type
                               HeapAllocator,
                               HeapAllocator,
                               HeapAllocator>
);

impl<'a, OutputType:Write> Write for DecompressAndValidate<'a, OutputType> {
    fn write(&mut self, data:&[u8]) -> Result<usize, io::Error> {
        self.0.write(data)
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        self.0.flush()
    }
}

fn make_sha_writer() -> io::Sink {
    io::sink()
}

fn make_sha_reader<InputType:Read>(r:&mut InputType) -> &mut InputType {
    r
}

fn sha_ok<InputType:Read>(_writer: io::Sink, _reader: &mut InputType) -> bool {
    false
}

struct ShaReader<InputType:Read> {
    reader:InputType,
}



pub fn compress_validate<InputType:Read, OutputType: Write>(r: &mut InputType,
                                                            w: &mut OutputType,
                                                            buffer_size: usize,
                                                            params: &BrotliEncoderParams,
                                                            custom_dictionary:Rebox<u8>,
                                                            num_threads: usize) -> Result<(), io::Error> {
    let mut m8 = HeapAllocator::default();
    let buffer = <HeapAllocator as Allocator<u8>>::alloc_cell(&mut m8, buffer_size);
    // FIXME: could reuse the dictionary to seed the compressor, but that violates the abstraction right now
    // also dictionaries are not very popular since they are mostly an internal concept, given their deprecation in
    // the standard brotli spec
    let mut dict = Vec::<u8>::new();

    dict.extend_from_slice(custom_dictionary.slice());
    let mut sha_writer = make_sha_writer();
    let mut sha_reader = make_sha_reader(r);
    let ret;
    {
        let validate_writer = DecompressAndValidate(DecompressorWriterCustomIo::new_with_custom_dictionary(
            IoWriterWrapper(&mut sha_writer),
            buffer,
            m8,
            HeapAllocator::default(),
            HeapAllocator::default(),
            custom_dictionary,
            Error::new(ErrorKind::InvalidData,
                       "Invalid Data")));
        let mut overarching_writer = Tee(validate_writer, w);
        ret = super::compress(sha_reader, &mut overarching_writer, buffer_size, params, &dict[..], num_threads);
    }
    match ret {
        Ok(_ret) => {
            if sha_ok(sha_writer, sha_reader) {
                return Ok(());
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "Validation failed"));
            }
        },
        Err(e) => Err(e),
    }
}
