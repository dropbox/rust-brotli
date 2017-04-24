#[cfg(not(feature="no-stdlib"))]
use std::io::{self, Error, ErrorKind, Write};
#[cfg(not(feature="no-stdlib"))]
pub use alloc::HeapAlloc;
#[cfg(all(feature="unsafe",not(feature="no-stdlib")))]
pub use alloc::HeapAllocUninitialized;
pub use huffman::{HuffmanCode, HuffmanTreeGroup};
pub use state::BrotliState;
// use io_wrappers::write_all;
pub use io_wrappers::{CustomWrite};
#[cfg(not(feature="no-stdlib"))]
pub use io_wrappers::{IntoIoWriter, IoWriterWrapper};
pub use super::decode::{BrotliDecompressStream, BrotliResult};
pub use alloc::{AllocatedStackMemory, Allocator, SliceWrapper, SliceWrapperMut, StackAllocator};
use super::enc::writer::write_all;
#[cfg(not(feature="no-stdlib"))]
pub struct DecompressorWriterCustomAlloc<W: Write,
     BufferType : SliceWrapperMut<u8>,
     AllocU8 : Allocator<u8>,
     AllocU32 : Allocator<u32>,
     AllocHC : Allocator<HuffmanCode> >(DecompressorWriterCustomIo<io::Error,
                                                             IntoIoWriter<W>,
                                                             BufferType,
                                                             AllocU8, AllocU32, AllocHC>);


#[cfg(not(feature="no-stdlib"))]
impl<W: Write,
     BufferType : SliceWrapperMut<u8>,
     AllocU8,
     AllocU32,
     AllocHC> DecompressorWriterCustomAlloc<W, BufferType, AllocU8, AllocU32, AllocHC>
 where AllocU8 : Allocator<u8>, AllocU32 : Allocator<u32>, AllocHC : Allocator<HuffmanCode>
    {

    pub fn new(w: W, buffer : BufferType,
               alloc_u8 : AllocU8, alloc_u32 : AllocU32, alloc_hc : AllocHC) -> Self {
        DecompressorWriterCustomAlloc::<W, BufferType, AllocU8, AllocU32, AllocHC>(
          DecompressorWriterCustomIo::<Error,
                                 IntoIoWriter<W>,
                                 BufferType,
                                 AllocU8, AllocU32, AllocHC>::new(IntoIoWriter::<W>(w),
                                                                  buffer,
                                                                  alloc_u8, alloc_u32, alloc_hc,
                                                                  Error::new(ErrorKind::InvalidData,
                                                                             "Invalid Data")))
    }
}
#[cfg(not(feature="no-stdlib"))]
impl<W: Write,
     BufferType : SliceWrapperMut<u8>,
     AllocU8 : Allocator<u8>,
     AllocU32 : Allocator<u32>,
     AllocHC : Allocator<HuffmanCode> > Write for DecompressorWriterCustomAlloc<W,
                                                                         BufferType,
                                                                         AllocU8,
                                                                         AllocU32,
                                                                         AllocHC> {
  	fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
       self.0.write(buf)
    }
  	fn flush(&mut self) -> Result<(), Error> {
       self.0.flush()
    }
}


#[cfg(not(any(feature="unsafe", feature="no-stdlib")))]
pub struct DecompressorWriter<W: Write>(DecompressorWriterCustomAlloc<W,
                                                         <HeapAlloc<u8>
                                                          as Allocator<u8>>::AllocatedMemory,
                                                         HeapAlloc<u8>,
                                                         HeapAlloc<u32>,
                                                         HeapAlloc<HuffmanCode> >);


#[cfg(not(any(feature="unsafe", feature="no-stdlib")))]
impl<W: Write> DecompressorWriter<W> {
  pub fn new(w: W, buffer_size: usize) -> Self {
    let mut alloc_u8 = HeapAlloc::<u8> { default_value: 0 };
    let buffer = alloc_u8.alloc_cell(buffer_size);
    let alloc_u32 = HeapAlloc::<u32> { default_value: 0 };
    let alloc_hc = HeapAlloc::<HuffmanCode> { default_value: HuffmanCode::default() };
    DecompressorWriter::<W>(DecompressorWriterCustomAlloc::<W,
                                                <HeapAlloc<u8>
                                                 as Allocator<u8>>::AllocatedMemory,
                                                HeapAlloc<u8>,
                                                HeapAlloc<u32>,
                                                HeapAlloc<HuffmanCode> >::new(w,
                                                                              buffer,
                                                                              alloc_u8,
                                                                              alloc_u32,
                                                                              alloc_hc))
  }
}


#[cfg(all(feature="unsafe", not(feature="no-stdlib")))]
pub struct DecompressorWriter<W: Write>(DecompressorWriterCustomAlloc<W,
                                                         <HeapAllocUninitialized<u8>
                                                          as Allocator<u8>>::AllocatedMemory,
                                                         HeapAllocUninitialized<u8>,
                                                         HeapAllocUninitialized<u32>,
                                                         HeapAllocUninitialized<HuffmanCode> >);


#[cfg(all(feature="unsafe", not(feature="no-stdlib")))]
impl<W: Write> DecompressorWriter<W> {
  pub fn new(w: W, buffer_size: usize) -> Self {
    let mut alloc_u8 = unsafe { HeapAllocUninitialized::<u8>::new() };
    let buffer = alloc_u8.alloc_cell(buffer_size);
    let alloc_u32 = unsafe { HeapAllocUninitialized::<u32>::new() };
    let alloc_hc = unsafe { HeapAllocUninitialized::<HuffmanCode>::new() };
    DecompressorWriter::<W>(DecompressorWriterCustomAlloc::<W,
                                                <HeapAllocUninitialized<u8>
                                                 as Allocator<u8>>::AllocatedMemory,
                                                HeapAllocUninitialized<u8>,
                                                HeapAllocUninitialized<u32>,
                                                HeapAllocUninitialized<HuffmanCode> >
      ::new(w, buffer, alloc_u8, alloc_u32, alloc_hc))
  }
}


#[cfg(not(feature="no-stdlib"))]
impl<W: Write> Write for DecompressorWriter<W> {
  	fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
       self.0.write(buf)
    }
  	fn flush(&mut self) -> Result<(), Error> {
       self.0.flush()
    }
}

pub struct DecompressorWriterCustomIo<ErrType,
                                W: CustomWrite<ErrType>,
                                BufferType: SliceWrapperMut<u8>,
                                AllocU8: Allocator<u8>,
                                AllocU32: Allocator<u32>,
                                AllocHC: Allocator<HuffmanCode>>
{
  output_buffer: BufferType,
  total_out: usize,
  output: W,
  error_if_invalid_data: Option<ErrType>,
  state: BrotliState<AllocU8, AllocU32, AllocHC>,
}

impl<ErrType,
     W: CustomWrite<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     AllocU8,
     AllocU32,
     AllocHC> DecompressorWriterCustomIo<ErrType, W, BufferType, AllocU8, AllocU32, AllocHC>
 where AllocU8 : Allocator<u8>, AllocU32 : Allocator<u32>, AllocHC : Allocator<HuffmanCode>
{

    pub fn new(w: W, buffer : BufferType,
               alloc_u8 : AllocU8, alloc_u32 : AllocU32, alloc_hc : AllocHC,
               invalid_data_error_type : ErrType) -> Self {
        DecompressorWriterCustomIo::<ErrType, W, BufferType, AllocU8, AllocU32, AllocHC>{
            output_buffer : buffer,
            total_out : 0,
            output: w,
            state : BrotliState::new(alloc_u8,
                                     alloc_u32,
                                     alloc_hc),
            error_if_invalid_data : Some(invalid_data_error_type),
        }
    }
    fn close(&mut self) -> Result<(), ErrType>{
        loop {
            let mut avail_in : usize = 0;
            let mut input_offset : usize = 0;
            let mut avail_out : usize = self.output_buffer.slice_mut().len();
            let mut output_offset : usize = 0;
            let ret = BrotliDecompressStream(
                &mut avail_in,
                &mut input_offset,
                &[],
                &mut avail_out,
                &mut output_offset,
                self.output_buffer.slice_mut(),                
                &mut self.total_out,
                &mut self.state);
          match write_all(&mut self.output, &self.output_buffer.slice_mut()[..output_offset]) {
            Ok(_) => {},
            Err(e) => return Err(e),
           }
           match ret {
           BrotliResult::NeedsMoreInput => return Err(self.error_if_invalid_data.take().unwrap()),
           BrotliResult::NeedsMoreOutput => {},
           BrotliResult::ResultSuccess => return Ok(()),
           BrotliResult::ResultFailure => return Err(self.error_if_invalid_data.take().unwrap()),
           }
        }
    }
}
impl<ErrType,
     W: CustomWrite<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     AllocU8 : Allocator<u8>,
     AllocU32 : Allocator<u32>,
     AllocHC : Allocator<HuffmanCode> > Drop for DecompressorWriterCustomIo<ErrType,
                                                                                     W,
                                                                                     BufferType,
                                                                                     AllocU8,
                                                                                     AllocU32,
                                                                                     AllocHC> {
  fn drop(&mut self) {
    match self.close() {
          Ok(_) => {},
          Err(_) => {},
    }
  }
}
impl<ErrType,
     W: CustomWrite<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     AllocU8 : Allocator<u8>,
     AllocU32 : Allocator<u32>,
     AllocHC : Allocator<HuffmanCode> > CustomWrite<ErrType> for DecompressorWriterCustomIo<ErrType,
                                                                                     W,
                                                                                     BufferType,
                                                                                     AllocU8,
                                                                                     AllocU32,
                                                                                     AllocHC> {
	fn write(&mut self, buf: &[u8]) -> Result<usize, ErrType > {
        let mut avail_in = buf.len();
        let mut input_offset : usize = 0;
        loop {
            let mut output_offset = 0;
            let mut avail_out = self.output_buffer.slice_mut().len();
            let op_result = BrotliDecompressStream(&mut avail_in,
                                     &mut input_offset,
                                     &buf[..],
                                     &mut avail_out,
                                     &mut output_offset,
                                     self.output_buffer.slice_mut(),
                                     &mut self.total_out,
                                     &mut self.state);
         match write_all(&mut self.output, &self.output_buffer.slice_mut()[..output_offset]) {
          Ok(_) => {},
          Err(e) => return Err(e),
         }
         match op_result {
          BrotliResult::NeedsMoreInput => assert_eq!(avail_in, 0),
          BrotliResult::NeedsMoreOutput => continue,
          BrotliResult::ResultSuccess => return Ok((buf.len())),
          BrotliResult::ResultFailure => return Err(self.error_if_invalid_data.take().unwrap()),
        }
        if avail_in == 0 {
           break
        }
      }
      Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), ErrType> {
       self.output.flush()
    }
}

