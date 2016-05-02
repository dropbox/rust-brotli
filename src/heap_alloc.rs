use alloc;
use core::ops;
use core;

pub struct Mem<T> {
   b : Box<[T]>,
}

impl<T> core::default::Default for Mem<T> {
    fn default() -> Self {
       let v : Vec<T> = Vec::new();
       let b = v.into_boxed_slice();
       return Mem::<T>{b : b};
    }
}

impl<T> ops::Index<usize> for Mem<T>{
    type Output = T;
    fn index(&self, index : usize) -> &T {
        return &(*self.b)[index]
    }
}

impl<T> ops::IndexMut<usize> for Mem<T>{
    fn index_mut(&mut self, index : usize) -> &mut T {
        return &mut (*self.b)[index]
    }
}

impl<T> alloc::SliceWrapper<T> for Mem<T> {
    fn slice(&self) -> & [T] {
       return &*self.b
    }
}

impl<T> alloc::SliceWrapperMut<T> for Mem<T> {
    fn slice_mut(&mut self) -> &mut [T] {
       return &mut*self.b
    }
}

pub struct HeapAllocator<T : core::clone::Clone>{
   pub default_value : T,
}

impl<T : core::clone::Clone> alloc::Allocator<T> for HeapAllocator<T> {
   type AllocatedMemory = Mem<T>;
   fn alloc_cell(self : &mut HeapAllocator<T>, len : usize) -> Mem<T> {

       let v : Vec<T> = vec![self.default_value.clone();len];
       let b = v.into_boxed_slice();
       return Mem::<T>{b : b};
   }
   fn free_cell(self : &mut HeapAllocator<T>, _data : Mem<T>) {

   }
}
