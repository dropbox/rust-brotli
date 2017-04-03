use core;
use super::super::alloc;
pub struct BlockSplit<AllocU8:alloc::Allocator<u8>,
                      AllocU32:alloc::Allocator<u32>>{
  pub num_types: usize,
  pub num_blocks: usize,
  pub types: AllocU8::AllocatedMemory,
  pub lengths: AllocU32::AllocatedMemory,
}

impl<AllocU8:alloc::Allocator<u8>,
                      AllocU32:alloc::Allocator<u32>> BlockSplit<AllocU8, AllocU32> {
    pub fn new() -> BlockSplit<AllocU8, AllocU32> {
       BlockSplit {
          num_types: 0, num_blocks:0, types:AllocU8::AllocatedMemory::default(), lengths:AllocU32::AllocatedMemory::default(),
       }
    }
    pub fn destroy(&mut self, m8: &mut AllocU8, m32: &mut AllocU32) {
        m8.free_cell(core::mem::replace(&mut self.types, AllocU8::AllocatedMemory::default()));
        m32.free_cell(core::mem::replace(&mut self.lengths, AllocU32::AllocatedMemory::default()));
        self.num_blocks = 0;
        self.num_types = 0;
    }
}
