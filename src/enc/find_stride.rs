use super::interface;
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use core::mem;
use core::ops::{Index, IndexMut, Range};
use super::input_pair::InputPair;
use super::bit_cost::BitsEntropy;
use super::util::{floatX, FastLog2};
use super::command::{Command, GetCopyLengthCode, GetInsertLengthCode, CommandDistanceIndexAndOffset};

// the cost of storing a particular population of data including the approx
// cost of a huffman table to describe the frequencies of each symbol
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

// this holds a population of data assuming 1 byte of prior for that data
// bucket_populations is therefore a 65536-long dynamically allocated buffer
struct EntropyBucketPopulation<AllocU32: alloc::Allocator<u32> > {
    pub bucket_populations: AllocU32::AllocatedMemory,
    pub cached_bit_entropy: floatX,
}
impl<AllocU32:alloc::Allocator<u32>> EntropyBucketPopulation<AllocU32> {
   fn clone_from(&mut self, other: &EntropyBucketPopulation<AllocU32>) {
        self.bucket_populations.slice_mut().clone_from_slice(other.bucket_populations.slice());
   }
   fn add_assign(&mut self, other: &EntropyBucketPopulation<AllocU32>) {
       assert_eq!(self.bucket_populations.slice().len(), other.bucket_populations.slice().len());
       for (item, other_item) in self.bucket_populations.slice_mut().iter_mut().zip(other.bucket_populations.slice().iter()) {
           *item += *other_item;
       }
       self.cached_bit_entropy = HuffmanCost(self.bucket_populations.slice());
   }
   // clear the allocated memory and reset literal population to zero
   fn bzero(&mut self) {
      self.cached_bit_entropy = 0.0;
      for bp in self.bucket_populations.slice_mut().iter_mut() {
         *bp = 0;
      }
   }
   // setup population to the sum of an array of populations where the stride of that row matches. Additionally allow another optional
   fn initiate_from(&mut self, rows: [&[Self];2], rows_stride:[&[u8];2], stride: u8, do_clear: bool) {
      self.cached_bit_entropy = 0.0;
      let mut found_any = false;
      for (sub_row, sub_stride) in rows.iter().zip(rows_stride.iter()) {
          for (item, istride) in sub_row.iter().zip(sub_stride.iter()) {
              if *istride != stride {
                 continue; // if we chain, then optional was already filtered by stride
              }
              if do_clear && !found_any {
                  self.bucket_populations.slice_mut().clone_from_slice(item.bucket_populations.slice());
                  found_any = true;
              } else{
                  for (dst, src) in self.bucket_populations.slice_mut().iter_mut().zip(item.bucket_populations.slice().iter()) {
                      *dst += *src;
                  }
              }
          }
      }
      if do_clear && !found_any {
          self.bzero();
      } else {
          self.cached_bit_entropy = HuffmanCost(self.bucket_populations.slice());
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
    pub fn free(&mut self, m32: &mut AllocU32) {
        for item in self.pop.iter_mut() {
            m32.free_cell(mem::replace(&mut item.bucket_populations,
                                       AllocU32::AllocatedMemory::default()));
        }
    }
    pub fn new(m32: &mut AllocU32) -> Self {
        let size = 256 * 256;
        EntropyPyramid::<AllocU32> {
           pop: [
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
           ],         
           stride:[1;NUM_NODES],
        }
    }
    pub fn populate_entry(&mut self, input:InputPair, scratch: &mut EntropyTally<AllocU32>, index: u32, mirror_range: Option<Range<usize>>, prev_range: Option<Range<usize>>) {
        let mut initial_entropies = [0.0 as floatX; NUM_STRIDES];
        {
            let pop_ranges = [match mirror_range{
                                 None => &[],
                                 Some(ref ir) => &self.pop[ir.clone()],
                              },
                              match prev_range {
                                 None => &[],
                                 Some(ref pr) => &self.pop[pr.clone()],
                              }];
            let stride_ranges = [match mirror_range{
                                 None => &[],
                                 Some(ref ir) => &self.stride[ir.clone()],
                              },
                              match prev_range {
                                 None => &[],
                                 Some(ref pr) => &self.stride[pr.clone()],
                              }];
            for stride in 0..NUM_STRIDES {
                     scratch.pop[stride].initiate_from(pop_ranges, stride_ranges,
                                                       stride as u8, true);
                     initial_entropies[stride] = scratch.pop[stride].cached_bit_entropy;
            }
        }
        scratch.observe_input_stream(input.0, input.1);
        let mut best_entropy_index = 0;
        let mut min_entropy_value = scratch.pop[0].cached_bit_entropy - initial_entropies[0];
        for stride in 1..NUM_STRIDES {
           let entropy_value = scratch.pop[stride].cached_bit_entropy - initial_entropies[stride];
           if entropy_value < min_entropy_value {
                best_entropy_index = stride;
                min_entropy_value = entropy_value;
           }
        }
        self.pop[index as usize].clone_from(&scratch.pop[best_entropy_index]);
        self.stride[index as usize] = best_entropy_index as u8;
    }
    fn populate(&mut self, input0:&[u8], input1:&[u8], scratch: &mut EntropyTally<AllocU32>) {
        let input = InputPair(input0, input1);
        self.populate_entry(input, scratch, 0, None, None); // BASE

        // LEVEL 1
        self.populate_entry(input.split_at(input.len() >> 1).0, scratch, 1, Some(0..1), None);
        self.populate_entry(input.split_at(input.len() >> 1).1, scratch, 2, None, Some(1..2)); // should we use the range from 0..1??

        // LEVEL 2
        self.populate_entry(input.split_at(input.len() >> 2).0, scratch, 3, Some(1..3), None);
        self.populate_entry(input.split_at(input.len() >> 1).0.split_at(input.len() >>2).1, scratch, 4, Some(2..3), Some(3..4));
        self.populate_entry(input.split_at(input.len() >> 1).1.split_at(input.len() >>2).0, scratch, 5, Some(3..5), None);
        self.populate_entry(input.split_at(input.len() >> 1).1.split_at(input.len() >>2).1, scratch, 6, Some(3..6), None);
        assert_eq!(NUM_LEVELS, 3); // we hard coded the 3 levels for now... we can add more later or make this into some kind of recursion

    }
}

impl<AllocU32:alloc::Allocator<u32> > EntropyTally<AllocU32> {
    pub fn new_pair(m32: &mut AllocU32) -> (EntropyTally<AllocU32>, EntropyTally<AllocU32>) {
        let size = 256 * 256;
        (EntropyTally::<AllocU32> {
            pop:[
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
            ]},
            EntropyTally::<AllocU32> {
            pop:[
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                EntropyBucketPopulation::<AllocU32>{
                    cached_bit_entropy:0.0,
                    bucket_populations:m32.alloc_cell(size),
                },
                ]
            })
    }
    fn observe_input_stream(&mut self, input0:&[u8], input1:&[u8]) {
        let mut priors = [0u8;NUM_STRIDES];
        for (index, val) in input0.iter().chain(input1.iter()).enumerate() {
            for stride in 0..NUM_STRIDES {
                if stride > index {
                    self.pop[stride].bucket_populations.slice_mut()[priors[stride] as usize * 256 + (*val as usize)] += 1;
                }
            }
            {
                let mut tmp = [0u8;NUM_STRIDES - 1];
                tmp.clone_from_slice(&priors[..(NUM_STRIDES - 1)]);
                priors[1..].clone_from_slice(&tmp[..]);
                priors[0] = *val;
            }
        }
        for stride in 0..NUM_STRIDES {
            self.pop[stride].cached_bit_entropy = HuffmanCost(self.pop[stride].bucket_populations.slice());
        }
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
