use super::super::alloc;
pub struct BlockSplit<AllocU8:alloc::Allocator<u8>,
                      AllocU32:alloc::Allocator<u32>>{
  pub num_types: usize,
  pub num_blocks: usize,
  pub types: AllocU8::AllocatedMemory,
  pub lengths: AllocU32::AllocatedMemory,
}
