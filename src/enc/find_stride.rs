use super::interface;
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use core::mem;
use core::ops::{Index,IndexMut};
use super::bit_cost::BitsEntropy;
use super::util::{floatX, FastLog2};
use super::command::{Command, GetCopyLengthCode, GetInsertLengthCode, CommandDistanceIndexAndOffset};

fn ApproxCost(population: &[u32]) -> floatX{
    let mut retval: floatX= 0.0 as floatX;
    for pop in population.iter() {
        retval += *pop as floatX * (*pop as floatX);
    }
    return 1.0 as floatX/retval;
}
fn HuffmanCost(population: &[u32]) -> floatX{
    assert_eq!(population.len(), 256 * 256);
    let mut cost : floatX = 0.0 as floatX;
    let mut sum : floatX = 0.0 as floatX;
    let mut buckets : floatX = 0.0 as floatX;
    for pop in population.iter() {
       if *pop == 0 {
           continue;
       }
       cost -= *pop as floatX * FastLog2(*pop as u64);
       sum += *pop as floatX;
       buckets += 1.0 as floatX;
    }
    return 12.0 as floatX * buckets +  cost + sum * FastLog2(sum as u64);
}

struct EntropyBucketPopulation<AllocU32: alloc::Allocator<u32> > {
    pub bucket_populations: AllocU32::AllocatedMemory,
    pub cached_bit_entropy: f64,
}
impl<AllocU32:alloc::Allocator<u32>> EntropyBucketPopulation<AllocU32> {
   fn add_assign(&mut self, other: &EntropyBucketPopulation<AllocU32>) {
       assert_eq!(self.bucket_populations.slice().len(), other.bucket_populations.slice().len());
       for (item, other_item) in self.bucket_populations.slice_mut().iter_mut().zip(other.bucket_populations.slice().iter()) {
           *item += *other_item;
       }
       self.cached_bit_entropy = HuffmanCost(self.bucket_populations.slice()) as f64;
   }
   fn bzero(&mut self) {
      self.cached_bit_entropy = 0.0;
      for bp in self.bucket_populations.slice_mut().iter_mut() {
         *bp = 0;
      }
   }
   fn initiate_from(&mut self, row: &[Self], row_stride:&[u8], prev_item: Option<&Self>, stride: u8) {
      self.cached_bit_entropy = 0.0;
      let mut found_any = false;
      for (index, item) in row.iter().enumerate() {
          if row_stride[index] != stride {
             continue;
          }
          if !found_any {
              self.bucket_populations.slice_mut().clone_from_slice(item.bucket_populations.slice());
              found_any = true;
          } else{
              for (dst, src) in self.bucket_populations.slice_mut().iter_mut().zip(item.bucket_populations.slice().iter()) {
                  *dst += *src;
              }
          }
      }
      match prev_item {
          None => {}, 
          Some(other) => {
              if !found_any {
                  self.bucket_populations.slice_mut().clone_from_slice(other.bucket_populations.slice());
                  found_any = true;
              } else{
                  for (dst, src) in self.bucket_populations.slice_mut().iter_mut().zip(other.bucket_populations.slice()) {
                     *dst += *src;
                  }
              }
          }
      }
     if !found_any {
         self.bzero();
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
const NUM_LEVELS: usize = 3;
const NUM_NODES: usize = (1<<(1 + NUM_LEVELS)) - 1;
pub struct EntropyPyramid<AllocU32: alloc::Allocator<u32> > {
    pop: [EntropyBucketPopulation<AllocU32>;NUM_NODES],
    stride: [u8;NUM_NODES],
}

impl<AllocU32:alloc::Allocator<u32>> EntropyPyramid<AllocU32> {
    pub fn new(m32: &mut AllocU32) -> Self {
        let size = 256 * 256;
        EntropyPyramid::<AllocU32> {
           pop: [
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
           ],         
           stride:[1;NUM_NODES],
        }
    }
    pub fn populate_entry(&mut self, input0:&[u8], input1:&[u8], scratch: &mut EntropyTally<AllocU32>, index: u32, mirror_index_start: Option<u32>, mirror_index_end: Option<u32>, alt_prev_index: Option<u32>) {
        let (alt_tally, alt_stride) = match alt_prev_index {
           None => (None, None),
           Some(alt_index) => (Some(&self.pop[alt_index as usize]), Some(self.stride[alt_index as usize])),
        };
        for stride in 0..NUM_STRIDES {
            match mirror_index_start {
                None => {
                   scratch.pop[stride].bzero();
                },
                Some(mirror_index) => {
                 scratch.pop[stride].initiate_from(&self.pop[mirror_index as usize ..
                                                   mirror_index_end.unwrap() as usize],
                                                   &self.stride[mirror_index as usize ..mirror_index_end.unwrap() as usize],
                                                   if alt_stride == Some(stride as u8) {alt_tally} else {None},
                                                   stride as u8);
                },
            }
        }
    }
}

impl<AllocU32:alloc::Allocator<u32> > EntropyTally<AllocU32> {
    pub fn new_pair(m32: &mut AllocU32) -> (EntropyTally<AllocU32>, EntropyTally<AllocU32>) {
        let size = 256 * 256;
        (EntropyTally::<AllocU32> {
            pop:[
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
            ]},
            EntropyTally::<AllocU32> {
            pop:[
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0f64,
                    bucket_populations:m32.alloc_cell(size),
                },
                ]
            })
    }
    fn identify_best_population_from_scratch(&self, scratch:&mut EntropyTally<AllocU32>) -> u8 {
        for index in 0..scratch.pop.len() {
           scratch.pop[index].add_assign(&self.pop[index]);
	   
        }
        let mut best_index = scratch.pop.len() - 1;
        let mut best = scratch.pop[best_index].cached_bit_entropy - self.pop[best_index].cached_bit_entropy;
        for index in 0..(scratch.pop.len() - 1){
            if scratch.pop[index].cached_bit_entropy - self.pop[index].cached_bit_entropy < best {
                 best_index=index;
                 best = scratch.pop[index].cached_bit_entropy - self.pop[index].cached_bit_entropy;
            }
        }
        best_index as u8
    }
    fn get_previous_bytes(&self, input0:&[u8], input1:&[u8], bytes_processed: usize) -> [u8; NUM_STRIDES] {
        let mut retval = [0u8; NUM_STRIDES];
        for index in 0..NUM_STRIDES {
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
                    for (lindex, val) in lit.data.slice().iter().enumerate() {
			 if lindex == NUM_STRIDES  {
			    let vpriors = self.get_previous_bytes(input0, input1, NUM_STRIDES+*bytes_processed);
			    assert_eq!(vpriors, priors);
			 }
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
	self.pop[retval as usize].cached_bit_entropy = scratch.pop[retval as usize].cached_bit_entropy;
        retval + 1
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
