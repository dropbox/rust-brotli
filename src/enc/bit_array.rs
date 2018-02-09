use core::mem;
use core::marker::PhantomData;
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};

pub trait BitArrayTrait {
    fn first_bit_set(&self, start: usize, size:usize) -> usize;
    fn first_bit_unset(&self, start: usize, size:usize) -> usize;
    fn len(&self) -> usize;
}

pub trait BitArrayMutTrait : BitArrayTrait {
    fn copy_from_byte_slice(&mut self, start: usize, size: usize, data: &[u8]);
}
#[derive(Default)]
pub struct AlwaysZero{
    pub size:usize
}
impl BitArrayTrait for AlwaysZero {
    fn first_bit_set(&self, _start: usize, size:usize) -> usize{
        size
    }
    fn first_bit_unset(&self, start: usize, _size:usize) -> usize{
        start
    }
    fn len(&self) -> usize{
        self.size
    }
}
pub struct BitArrayViewMut<'a> {
    bitvec: &'a mut [u8],
}
impl<'a> BitArrayViewMut<'a> {
    pub fn view(&'a self, start: usize, end: usize) -> BitArrayView<'a> {
        BitArrayView::<'a>{
            bitvec:&self.bitvec[start..end],
        }
    }
    pub fn full_view(&'a self) -> BitArrayView<'a> {
        BitArrayView::<'a>{
            bitvec:self.bitvec,
        }
    }
}

impl<'a> BitArrayTrait for BitArrayViewMut<'a> {
    fn first_bit_set(&self, start: usize, size:usize) -> usize{
        self.full_view().first_bit_set(start, size)
    }
    fn first_bit_unset(&self, start: usize, size:usize) -> usize{
        self.full_view().first_bit_unset(start, size)
    }
    fn len(&self) -> usize{
        self.bitvec.len()
    }
}

impl<'a> BitArrayMutTrait for BitArrayViewMut<'a> {
    fn copy_from_byte_slice(&mut self, start: usize, size: usize, data: &[u8]) {
        let dst = self.bitvec.split_at_mut(start).1.split_at_mut(size).0;
        if data.len() >= size {
            dst.clone_from_slice(data.split_at(size).0);
        } else {
            for item in dst.iter_mut() {
                *item = 0;
            }
            assert_eq!(data.len(), 0);
        }
    }
}

pub struct BitArrayView<'a> {
    bitvec: &'a [u8],
}
impl<'a> BitArrayView<'a> {
    pub fn new(data:&'a [u8]) -> Self {
        BitArrayView {
            bitvec:data,
        }
    }
}
impl<'a> BitArrayTrait for BitArrayView<'a> {
    fn first_bit_set(&self, start: usize, size:usize) -> usize{
        for (index, item) in self.bitvec.split_at(start).1.split_at(size).0.iter().enumerate() {
            if *item != 0 {
                return index;
            }
        }
        size
    }
    fn first_bit_unset(&self, start: usize, size:usize) -> usize{
        for (index, item) in self.bitvec.split_at(start).1.split_at(size).0.iter().enumerate() {
            if *item == 0 {
                return index;
            }
        }
        size
    }
    fn len(&self) -> usize{
        self.bitvec.len()
    }
}
pub struct BitArray<AllocU8: alloc::Allocator<u8>,
                    AllocU32:alloc::Allocator<u32> > {
    bitvec: AllocU8::AllocatedMemory,
    _m32: PhantomData<AllocU32>,
}
impl<AllocU8: alloc::Allocator<u8>,
     AllocU32: alloc::Allocator<u32>> Default for BitArray<AllocU8, AllocU32> {
    fn default() -> Self {
        BitArray::<AllocU8, AllocU32>{
            bitvec: AllocU8::AllocatedMemory::default(),
            _m32: PhantomData::<AllocU32>::default()
        }        
    }
}
impl<AllocU8: alloc::Allocator<u8>,
     AllocU32: alloc::Allocator<u32>> BitArray<AllocU8, AllocU32> {
    pub fn new(m8: &mut AllocU8,
           _m32: &mut AllocU32, size:usize) -> Self{
        BitArray::<AllocU8, AllocU32>{
            bitvec: m8.alloc_cell(size),
            _m32: PhantomData::<AllocU32>::default()
        }
    }
    pub fn view_mut(&mut self, start: usize, end: usize) -> BitArrayViewMut {
        BitArrayViewMut{bitvec:&mut self.bitvec.slice_mut()[start..end]}
    }
    pub fn view(&self, start: usize, end: usize) -> BitArrayView {
        BitArrayView{bitvec:&self.bitvec.slice()[start..end]}
    }
    pub fn free(&mut self, m8: &mut AllocU8, _m32: &mut AllocU32) {
        m8.free_cell(mem::replace(&mut self.bitvec, AllocU8::AllocatedMemory::default()));
    }
    pub fn allocate_valid_range(&self, m8: &mut AllocU8, size: usize) -> AllocU8::AllocatedMemory {
        m8.alloc_cell(size)
    }
    pub fn resize(&mut self, m8: &mut AllocU8, m32: &mut AllocU32, new_size:usize) {
        let mut new_self = Self::new(m8, m32, new_size);
        let old_size = self.bitvec.slice().len();
        if old_size != 0 {
            new_self.bitvec.slice_mut().split_at_mut(old_size).0.clone_from_slice(self.bitvec.slice());
        }
        mem::replace(self, new_self).free(m8, m32);
    }
}
 
impl<AllocU8: alloc::Allocator<u8>,
     AllocU32: alloc::Allocator<u32>> BitArrayTrait for BitArray<AllocU8, AllocU32> {
    fn first_bit_set(&self, start: usize, size:usize) -> usize{
        BitArrayView{bitvec:self.bitvec.slice()}.first_bit_set(start, size)
    }
    fn first_bit_unset(&self, start: usize, size:usize) -> usize{
        BitArrayView{bitvec:self.bitvec.slice()}.first_bit_unset(start, size)
    }
    fn len(&self) -> usize{
        self.bitvec.slice().len()
    }
}


impl<AllocU8: alloc::Allocator<u8>,
     AllocU32: alloc::Allocator<u32>> BitArrayMutTrait for BitArray<AllocU8, AllocU32> {
    fn copy_from_byte_slice(&mut self, start: usize, size: usize, data: &[u8]) {
        let dst = self.bitvec.slice_mut().split_at_mut(start).1.split_at_mut(size).0;
        if data.len() >= size {
            dst.clone_from_slice(data.split_at(size).0);
        } else {
            for item in dst.iter_mut() {
                *item = 0;
            }
            assert_eq!(data.len(), 0);
        }
    }
}
/*
pub struct BitArray<AllocU8: alloc::Allocator<u8>> {
    bitvec: AllocU8::AllocatedMemory,
}
type primitive_type = u32;
const primitive_size:u8 = 32;
const primitive_size_m_1:u8 = 31;
const primitive_size_mask:u32 = 0xffffffff;
impl<AllocU32: alloc::Allocator<u32>> BitArray<AllocU32> {
    fn new(m8: &mut AllocU8, size:usize) -> Self{
        let bit_size = if size & 7 != 0 { 1 + (size >>3) } else { size >> 3};
        BitArray::<AllocU8>{
            bitvec: m8.alloc_cell(bit_size),
        }
    }
    fn first_set(&self, start: usize, stop: usize) -> usize {
        let bit_start_index = start / primitive_size;
        let bit_end_index = (stop + primitive_size_m_1) / primitive_size;
        let mut bit_start_mask: primitive_type = (primitive_size_mask >> (start & primitive_size_m_1)) << (start & primitive_size_m_1);
        let bit_end_mask:primitive_type = ((u64::from(primitive_size_mask) << (primitive_size_m_1 - (stop & primitive_size_m_1))) & primitive_size_mask) as primitive_type;
        if bit_start_index == bit_end_index {
            return false;
        }
        let last = self.bitvec.slice()[bit_end_index - 1];
        if bit_start_index + 1 == bit_stop_index  {
            let mask = last & bit_start_mask & bit_end_mask;
            if mask != 0 {
                return start + mask.count_trailing_zeros()
            }
        }
        for (index, item) in self.bitvec.slice()[bit_start_index..(bit_end_index - 1)].enumerate() {
            if (item & bit_start_mask) != 0 {
                return start + index * 32 + (item & bit_start_mask).count_trailing_zeros()
            }
            bit_start_mask = 0xff;
        }
        if (last & bit_end_mask) ! = 0 {
            ((bit_end_index - 1)<< primitive_size) + (last & bit_end_mask).count_trailing_zeros()
        }
        stop
    }
    fn copy(&mut self, start: usize, data: &[u8]) {
        self.bitvec.split_at(start).1.split_at(data.len()).0.clone_from_slice(data);
    }
    fn set(&mut self, index: usize) {
        self.bitvec.slice_mut()[index >> 3] |= (index & 7);
    }
    fn free(&mut self, m8: &mut AllocU8) {
        m8.free_cell(self.bitvec);
    }
}
*/
