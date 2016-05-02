#[macro_use]
extern crate alloc_no_stdlib as alloc;
extern crate brotli_no_stdlib;
extern crate core;
use std::io;
mod heap_alloc;
use heap_alloc::{HeapAllocator, Mem};
use brotli_no_stdlib::{HuffmanCode, BrotliState};
use alloc::{Allocator, SliceWrapperMut, SliceWrapper, StackAllocator, AllocatedStackMemory};


pub struct Decompressor<'a, R: io::Read> {
    input: R,
    alloc_u8 : HeapAllocator<u8>,
    alloc_u32 : HeapAllocator<u32>,
    alloc_hc : HeapAllocator<HuffmanCode>,
    state : Option<BrotliState<'a, HeapAllocator<u8>, HeapAllocator<u32>, HeapAllocator<HuffmanCode> > >,
}

impl<'a, R: io::Read> Decompressor<'a, R> {
    pub fn new(r: R) -> Decompressor<'a, R> {
        let ret = Decompressor{
            input: r,
            alloc_u8 : HeapAllocator::<u8>{default_value : 0u8},
            alloc_u32 : HeapAllocator::<u32>{default_value : 0u32},
            alloc_hc : HeapAllocator::<HuffmanCode>{default_value : HuffmanCode::default()},
            state : None,
        };
        ret.state = Some(BrotliState::new(ret.alloc_u8, ret.alloc_u32, ret.alloc_hc));
        return ret
    }
}
impl<'a, R: io::Read> io::Read for Decompressor<'a, R> {
	fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Unimplemented"))
        }
}