
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};

use core::ops::{Index,IndexMut};
use super::command::{Command, GetCopyLengthCode, GetInsertLengthCode, CommandDistanceIndexAndOffset};
struct EntropyBucketPopulation<AllocU32: alloc::Allocator<u32> > {
    pub bucket_populations: AllocU32::AllocatedMemory,
}


const NUM_STRIDES:usize = 8;
#[derive(Copy,Clone)]
pub struct BucketPopIndex {
    pub val: u8,
    pub six_bits: u8,
    pub stride: u8,
}

impl <AllocU32: alloc::Allocator<u32> > Index<BucketPopIndex> for EntropyBucketPopulation<AllocU32> {
    type Output = u32;
    fn index<'a>(&'a self, index: BucketPopIndex) -> &'a u32 {
        &self.bucket_populations.slice()[index.val as usize + index.six_bits as usize * 256 + index.stride as usize * 256 * 64]
    }
}
impl <AllocU32: alloc::Allocator<u32> > IndexMut<BucketPopIndex> for EntropyBucketPopulation<AllocU32> {
    fn index_mut<'a>(&'a mut self, index: BucketPopIndex) -> &'a mut u32 {
        &mut self.bucket_populations.slice_mut()[index.val as usize + index.six_bits as usize * 256 + index.stride as usize * 256 * 64]
    }
}

pub struct EntropyTally<AllocU32: alloc::Allocator<u32> > {
    pop:[EntropyBucketPopulation<AllocU32>;NUM_STRIDES],
}

impl<AllocU32:alloc::Allocator<u32> > EntropyTally<AllocU32> {
    pub fn new_pair(m32: &mut AllocU32) -> (EntropyTally<AllocU32>, EntropyTally<AllocU32>) {
        let size = 256 * 256 * 64;
        (EntropyTally::<AllocU32> {
            pop:[
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
            ]},
            EntropyTally::<AllocU32> {
            pop:[
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    bucket_populations:m32.alloc_cell(size),
                },
                ]
            })
    }
    pub fn pick_best_stride(&mut self, commands: &[Command], btype_bytes_left:u32,
                            scratch: &mut EntropyTally<AllocU32>) -> u8 {
        for cmd in commands.iter() {
            
        }
        0
    }
}
