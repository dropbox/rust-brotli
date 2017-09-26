use super::interface;
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use core::mem;
use core::ops::{Index,IndexMut};
use super::bit_cost::BitsEntropy;
use super::command::{Command, GetCopyLengthCode, GetInsertLengthCode, CommandDistanceIndexAndOffset};
struct EntropyBucketPopulation<AllocU32: alloc::Allocator<u32> > {
    pub bucket_populations: AllocU32::AllocatedMemory,
}
impl<AllocU32:alloc::Allocator<u32>> EntropyBucketPopulation<AllocU32> {
   fn add_assign(&mut self, other: &EntropyBucketPopulation<AllocU32>) {
       assert_eq!(self.bucket_populations.slice().len(), other.bucket_populations.slice().len());
       for (item, other_item) in self.bucket_populations.slice_mut().iter_mut().zip(other.bucket_populations.slice().iter()) {
           *item += *other_item;
       }
   }
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
        let size = 256 * 256;
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
    fn identify_best_population_from_scratch(&self, scratch:&mut EntropyTally<AllocU32>) -> u8 {
        let mut entropies = [0.0f64;8];
        for index in 0..scratch.pop.len() {
           scratch.pop[index].add_assign(&self.pop[index]);
           entropies[index] = BitsEntropy(scratch.pop[index].bucket_populations.slice(), scratch.pop[index].bucket_populations.slice().len()) as f64;
        }
        let mut best = entropies[0];
        let mut best_index = 0usize;
        for index in 1..entropies.len() {
            if entropies[index] < best {
                 best_index=index;
                 best = entropies[index];
            }
        }
        best_index as u8
    }
    fn get_previous_bytes(&self, input0:&[u8], input1:&[u8], bytes_processed: usize) -> [u8; 8] {
        let mut retval = [0u8; 8];
        for index in 0..8 {
            let bp_offset = index + 1;
            if bp_offset <= bytes_processed {
                 let offset = bytes_processed - bp_offset;
                 if offset >= input0.len() {
		    retval[index] = input1[offset - input0.len()];
                 } else {
                    retval[index] = input0[offset];
                 }
            }
	}
	retval
    }
    pub fn pick_best_stride<InputReference:SliceWrapper<u8>>(&mut self, commands: &[interface::Command<InputReference>], scratch: &mut EntropyTally<AllocU32>, input0: &[u8], input1: &[u8], bytes_processed: &mut usize) -> u8 {
        for cmd in commands.iter() {
            match cmd {
                &interface::Command::Copy(ref copy) => {
		    *bytes_processed += copy.num_bytes as usize;
                },
                &interface::Command::Dict(ref dict) => {
		    *bytes_processed += dict.final_size as usize;
                },
                &interface::Command::Literal(ref lit) => {
                    let mut priors = self.get_previous_bytes(input0, input1, *bytes_processed);
                    for val in lit.data.slice().iter() {
                         for (index, prior) in priors.iter().enumerate() {
                             scratch.pop[index].bucket_populations.slice_mut()[256 * (*prior as usize) + *val as usize] += 1;
                             // increment the population value of this literal
                             // for the respective prior for the stride index
                         }
                         { //reset prior values for the next item
                             let mut tmp = [0u8;7];
                             tmp.clone_from_slice(&priors[..7]);
                             priors[1..].clone_from_slice(&tmp[..]);
                             priors[0] = *val;
                         }
                    }
                    *bytes_processed += lit.data.slice().len();
                },
                _ => {},
            }
        }
        let retval = self.identify_best_population_from_scratch(scratch);
        self.pop[retval as usize].bucket_populations.slice_mut().clone_from_slice(scratch.pop[retval as usize].bucket_populations.slice());
        retval
    }
    pub fn free(&mut self, m32: &mut AllocU32) {
        for item in self.pop.iter_mut() {
            m32.free_cell(mem::replace(&mut item.bucket_populations, AllocU32::AllocatedMemory::default()))
        }
    }
    pub fn is_free(&mut self) -> bool {
        self.pop[0].bucket_populations.slice().len() == 0
    }
}
