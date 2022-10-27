#![allow(dead_code)]
use super::super::alloc;
use super::super::alloc::Allocator;
use super::super::alloc::SliceWrapper;
use core;
pub struct BlockSplit<Alloc: alloc::Allocator<u8> + alloc::Allocator<u32>> {
  pub num_types: usize,
  pub num_blocks: usize,
  pub types: <Alloc as Allocator<u8>>::AllocatedMemory,
  pub lengths: <Alloc as Allocator<u32>>::AllocatedMemory,
}

impl<Alloc: alloc::Allocator<u8> + alloc::Allocator<u32>> BlockSplit<Alloc> {
  pub fn new() -> BlockSplit<Alloc> {
    BlockSplit {
      num_types: 0,
      num_blocks: 0,
      types: <Alloc as Allocator<u8>>::AllocatedMemory::default(),
      lengths: <Alloc as Allocator<u32>>::AllocatedMemory::default(),
    }
  }
  pub fn destroy(&mut self, m: &mut Alloc) {
    <Alloc as Allocator<u8>>::free_cell(m, core::mem::replace(&mut self.types, <Alloc as Allocator<u8>>::AllocatedMemory::default()));
    <Alloc as Allocator<u32>>::free_cell(m, core::mem::replace(&mut self.lengths, <Alloc as Allocator<u32>>::AllocatedMemory::default()));
    self.num_blocks = 0;
    self.num_types = 0;
  }
  pub fn types_alloc_size(&self) -> usize {
    self.types.slice().len()
  }
  pub fn lengths_alloc_size(&self) -> usize {
    self.lengths.slice().len()
  }
}
