#[cfg(not(feature="no-stdlib"))]
use std::io::{self, Error, ErrorKind, Read};
#[cfg(not(feature="no-stdlib"))]
pub use alloc::HeapAlloc;
#[cfg(all(feature="unsafe",not(feature="no-stdlib")))]
pub use alloc::HeapAllocUninitialized;
pub use huffman::{HuffmanCode, HuffmanTreeGroup};
pub use state::BrotliState;
// use io_wrappers::write_all;
pub use io_wrappers::{CustomRead, CustomWrite};
#[cfg(not(feature="no-stdlib"))]
pub use io_wrappers::{IntoIoReader, IoReaderWrapper, IoWriterWrapper};
pub use super::decode::{BrotliDecompressStream, BrotliResult};
pub use alloc::{AllocatedStackMemory, Allocator, SliceWrapper, SliceWrapperMut, StackAllocator};

#[cfg(not(feature="no-stdlib"))]
pub struct DecompressorCustomAlloc<R: Read,
     BufferType : SliceWrapperMut<u8>,
     AllocU8 : Allocator<u8>,
     AllocU32 : Allocator<u32>,
     AllocHC : Allocator<HuffmanCode> >(DecompressorCustomIo<io::Error,
                                                             IntoIoReader<R>,
                                                             BufferType,
                                                             AllocU8, AllocU32, AllocHC>);


#[cfg(not(feature="no-stdlib"))]
impl<R: Read,
     BufferType : SliceWrapperMut<u8>,
     AllocU8,
     AllocU32,
     AllocHC> DecompressorCustomAlloc<R, BufferType, AllocU8, AllocU32, AllocHC>
 where AllocU8 : Allocator<u8>, AllocU32 : Allocator<u32>, AllocHC : Allocator<HuffmanCode>
    {

    pub fn new(r: R, buffer : BufferType,
               alloc_u8 : AllocU8, alloc_u32 : AllocU32, alloc_hc : AllocHC) -> Self {
        DecompressorCustomAlloc::<R, BufferType, AllocU8, AllocU32, AllocHC>(
          DecompressorCustomIo::<Error,
                                 IntoIoReader<R>,
                                 BufferType,
                                 AllocU8, AllocU32, AllocHC>::new(IntoIoReader::<R>(r),
                                                                  buffer,
                                                                  alloc_u8, alloc_u32, alloc_hc,
                                                                  Error::new(ErrorKind::InvalidData,
                                                                             "Invalid Data")))
    }
}
#[cfg(not(feature="no-stdlib"))]
impl<R: Read,
     BufferType : SliceWrapperMut<u8>,
     AllocU8 : Allocator<u8>,
     AllocU32 : Allocator<u32>,
     AllocHC : Allocator<HuffmanCode> > Read for DecompressorCustomAlloc<R,
                                                                         BufferType,
                                                                         AllocU8,
                                                                         AllocU32,
                                                                         AllocHC> {
  	fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, Error> {
       self.0.read(buf)
    }
}


#[cfg(not(any(feature="unsafe", feature="no-stdlib")))]
pub struct Decompressor<R: Read>(DecompressorCustomAlloc<R,
                                                         <HeapAlloc<u8>
                                                          as Allocator<u8>>::AllocatedMemory,
                                                         HeapAlloc<u8>,
                                                         HeapAlloc<u32>,
                                                         HeapAlloc<HuffmanCode> >);


#[cfg(not(any(feature="unsafe", feature="no-stdlib")))]
impl<R: Read> Decompressor<R> {
  pub fn new(r: R, buffer_size: usize) -> Self {
    let mut alloc_u8 = HeapAlloc::<u8> { default_value: 0 };
    let buffer = alloc_u8.alloc_cell(buffer_size);
    let alloc_u32 = HeapAlloc::<u32> { default_value: 0 };
    let alloc_hc = HeapAlloc::<HuffmanCode> { default_value: HuffmanCode::default() };
    Decompressor::<R>(DecompressorCustomAlloc::<R,
                                                <HeapAlloc<u8>
                                                 as Allocator<u8>>::AllocatedMemory,
                                                HeapAlloc<u8>,
                                                HeapAlloc<u32>,
                                                HeapAlloc<HuffmanCode> >::new(r,
                                                                              buffer,
                                                                              alloc_u8,
                                                                              alloc_u32,
                                                                              alloc_hc))
  }
}


#[cfg(all(feature="unsafe", not(feature="no-stdlib")))]
pub struct Decompressor<R: Read>(DecompressorCustomAlloc<R,
                                                         <HeapAllocUninitialized<u8>
                                                          as Allocator<u8>>::AllocatedMemory,
                                                         HeapAllocUninitialized<u8>,
                                                         HeapAllocUninitialized<u32>,
                                                         HeapAllocUninitialized<HuffmanCode> >);


#[cfg(all(feature="unsafe", not(feature="no-stdlib")))]
impl<R: Read> Decompressor<R> {
  pub fn new(r: R, buffer_size: usize) -> Self {
    let mut alloc_u8 = unsafe { HeapAllocUninitialized::<u8>::new() };
    let buffer = alloc_u8.alloc_cell(buffer_size);
    let alloc_u32 = unsafe { HeapAllocUninitialized::<u32>::new() };
    let alloc_hc = unsafe { HeapAllocUninitialized::<HuffmanCode>::new() };
    Decompressor::<R>(DecompressorCustomAlloc::<R,
                                                <HeapAllocUninitialized<u8>
                                                 as Allocator<u8>>::AllocatedMemory,
                                                HeapAllocUninitialized<u8>,
                                                HeapAllocUninitialized<u32>,
                                                HeapAllocUninitialized<HuffmanCode> >
      ::new(r, buffer, alloc_u8, alloc_u32, alloc_hc))
  }
}


#[cfg(not(feature="no-stdlib"))]
impl<R: Read> Read for Decompressor<R> {
  fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, Error> {
    self.0.read(buf)
  }
}

pub struct DecompressorCustomIo<ErrType,
                                R: CustomRead<ErrType>,
                                BufferType: SliceWrapperMut<u8>,
                                AllocU8: Allocator<u8>,
                                AllocU32: Allocator<u32>,
                                AllocHC: Allocator<HuffmanCode>>
{
  input_buffer: BufferType,
  total_out: usize,
  input_offset: usize,
  input_len: usize,
  input_eof: bool,
  input: R,
  error_if_invalid_data: Option<ErrType>,
  read_error: Option<ErrType>,
  state: BrotliState<AllocU8, AllocU32, AllocHC>,
}

impl<ErrType,
     R: CustomRead<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     AllocU8,
     AllocU32,
     AllocHC> DecompressorCustomIo<ErrType, R, BufferType, AllocU8, AllocU32, AllocHC>
 where AllocU8 : Allocator<u8>, AllocU32 : Allocator<u32>, AllocHC : Allocator<HuffmanCode>
{

    pub fn new(r: R, buffer : BufferType,
               alloc_u8 : AllocU8, alloc_u32 : AllocU32, alloc_hc : AllocHC,
               invalid_data_error_type : ErrType) -> Self {
        DecompressorCustomIo::<ErrType, R, BufferType, AllocU8, AllocU32, AllocHC>{
            input_buffer : buffer,
            total_out : 0,
            input_offset : 0,
            input_len : 0,
            input_eof : false,
            input: r,
            state : BrotliState::new(alloc_u8,
                                     alloc_u32,
                                     alloc_hc),
            error_if_invalid_data : Some(invalid_data_error_type),
            read_error : None,
        }
    }

    pub fn copy_to_front(&mut self) {
        let avail_in = self.input_len - self.input_offset;
        if self.input_offset == self.input_buffer.slice_mut().len() {
            self.input_offset = 0;
            self.input_len = 0;
        } else if self.input_offset + 256 > self.input_buffer.slice_mut().len() && avail_in < self.input_offset {
            let (mut first, second) = self.input_buffer.slice_mut().split_at_mut(self.input_offset);
            self.input_len -= self.input_offset;
            first[0..avail_in].clone_from_slice(&second[0..avail_in]);
            self.input_offset = 0;
        }
    }
}
impl<ErrType,
     R: CustomRead<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     AllocU8 : Allocator<u8>,
     AllocU32 : Allocator<u32>,
     AllocHC : Allocator<HuffmanCode> > CustomRead<ErrType> for DecompressorCustomIo<ErrType,
                                                                                     R,
                                                                                     BufferType,
                                                                                     AllocU8,
                                                                                     AllocU32,
                                                                                     AllocHC> {
	fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, ErrType > {
      let mut output_offset : usize = 0;
      let mut avail_out = buf.len() - output_offset;
      let mut avail_in = self.input_len - self.input_offset;
      let mut needs_input = false;
      while avail_out == buf.len() && (!needs_input || !self.input_eof) {
        if self.input_len < self.input_buffer.slice_mut().len() && !self.input_eof {
          match self.input.read(&mut self.input_buffer.slice_mut()[self.input_len..]) {
            Err(e) => {
              self.read_error = Some(e);
              self.input_eof = true;
            },
            Ok(size) => if size == 0 {
              self.input_eof = true;
            }else {
              needs_input = false;
              self.input_len += size;
              avail_in = self.input_len - self.input_offset;
            },
          }
        }
        match BrotliDecompressStream(&mut avail_in,
                                     &mut self.input_offset,
                                     &self.input_buffer.slice_mut()[..],
                                     &mut avail_out,
                                     &mut output_offset,
                                     buf,
                                     &mut self.total_out,
                                     &mut self.state) {
          BrotliResult::NeedsMoreInput => {
            match self.read_error.take() {
              Some(err) => return Err(err),
              None => {
                needs_input = true;
                self.copy_to_front();
              },
            }
          },
          BrotliResult::NeedsMoreOutput => {},
          BrotliResult::ResultSuccess => break,
          BrotliResult::ResultFailure => return Err(self.error_if_invalid_data.take().unwrap()),
        }
      }
      Ok(output_offset)
    }
}

