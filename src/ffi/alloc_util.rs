use alloc::Allocator;
use ::enc::BrotliAlloc;
use brotli_decompressor::ffi::alloc_util::SubclassableAllocator;








pub struct BrotliSubclassableAllocator(SubclassableAllocator);

impl BrotliSubclassableAllocator {
  pub fn new(s:SubclassableAllocator) -> BrotliSubclassableAllocator {
    BrotliSubclassableAllocator(s)
  }
}
impl<T:Clone+Default> Allocator<T> for BrotliSubclassableAllocator {
  type AllocatedMemory = <SubclassableAllocator as Allocator<T>>::AllocatedMemory;
  fn alloc_cell(&mut self, s:usize) -> Self::AllocatedMemory {
    self.0.alloc_cell(s)
  }
  fn free_cell(&mut self, data:Self::AllocatedMemory) {
    self.0.free_cell(data)
  }
}


impl BrotliAlloc for BrotliSubclassableAllocator {
}


