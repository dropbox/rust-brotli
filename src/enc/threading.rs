use alloc::{Allocator, SliceWrapper, SliceWrapperMut};
use core::marker::PhantomData;
use super::BrotliAlloc;
use super::backward_references::BrotliEncoderParams;
pub trait Joinable<T:Send+'static>:Sized {
  fn join(self) -> Result<T, ()>;
}


pub struct CompressionThreadResult<Alloc:BrotliAlloc+Send+'static> where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  compressed: <Alloc as Allocator<u8>>::AllocatedMemory,
  compressed_size: usize,
  alloc: Alloc,
}
enum InternalSendAlloc<T:Send+'static, Alloc:BrotliAlloc+Send+'static, Join: Joinable<T>>
  where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  A(Alloc),
  Join(Join),
  Spawning(PhantomData<T>),
}
pub struct SendAlloc<T:Send+'static,
                     Alloc:BrotliAlloc +Send+'static,
                     Join:Joinable<T>>(InternalSendAlloc<T, Alloc, Join>)
  where <Alloc as Allocator<u8>>::AllocatedMemory: Send;

impl<T:Send+'static, Alloc:BrotliAlloc+Send+'static,Join:Joinable<T>> SendAlloc<T, Alloc, Join>
  where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  pub fn new(alloc: Alloc) -> Self {
    SendAlloc::<T, Alloc, Join>(InternalSendAlloc::A(alloc))
  }
  pub fn unwrap_or(self, other: Alloc) -> Alloc {
    match self.0 {
      InternalSendAlloc::A(alloc) => {
        alloc
      },
      InternalSendAlloc::Spawning(_) | InternalSendAlloc::Join(_) => {
        other
      },
    }
  }
  pub fn unwrap(self) -> Alloc {
    match self.0 {
      InternalSendAlloc::A(alloc) => {
        alloc
      },
      InternalSendAlloc::Join(_) | InternalSendAlloc::Spawning(_) => panic!("Item permanently borrowed/leaked"),
    }
  }
}
  

enum InternalOwned<T> {
  Item(T),
  Borrowed,
}

pub struct Owned<T>(InternalOwned<T>);
impl<T> Owned<T> {
  pub fn new(data:T) -> Self {
    Owned::<T>(InternalOwned::Item(data))
  }
  pub fn unwrap_or(self, other: T) -> T {
    if let InternalOwned::Item(x) = self.0 {
      x
    } else {
      other
    }
  }
  pub fn unwrap(self) -> T {
    if let InternalOwned::Item(x) = self.0 {
      x
    } else {
      panic!("Item permanently borrowed")
    }
  }
}

pub trait OwnedRetriever<SliceW: SliceWrapper<u8>+Send+'static> {
  fn view(&self) -> &[u8];
  fn unwrap(self) -> SliceW;
}

pub trait BatchSpawnable<T:Send+'static, Alloc:BrotliAlloc+Send+'static, SliceW:SliceWrapper<u8>+Send+'static>
  where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static
{
  type JoinHandle: Joinable<T>;
  type FinalJoinHandle: OwnedRetriever<SliceW>;
  // this function takes in a SendAlloc per thread and converts them all into JoinHandle
  // the input is borrowed until the joins complete
  // owned is set to borrowed
  // the final join handle is a r/w lock which will return the SliceW to the owner
  // the FinalJoinHandle is only to be called when each individual JoinHandle has been examined
  fn batch_spawn<F: FnOnce(usize, &SliceW, Alloc) -> T>(
    &mut self,
    input: &mut Owned<SliceW>,
    alloc_per_thread:&mut [SendAlloc<T, Alloc, Self::JoinHandle>],
    f: F,
  ) -> Self::FinalJoinHandle;
}



pub fn CompressMultiSlice<Alloc:BrotliAlloc+Send+'static,
                          Spawner:BatchSpawnable<CompressionThreadResult<Alloc>,
                                                 Alloc,
                                                 <Alloc as Allocator<u8>>::AllocatedMemory>> (
  params:&BrotliEncoderParams,
  input_slice: &[u8],
  output: &mut [u8],
  alloc_per_thread:&mut [SendAlloc<CompressionThreadResult<Alloc>, Alloc, Spawner::JoinHandle>],
  mut thread_spawner: Spawner,
) -> Result<usize, ()> where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  let input = if let InternalSendAlloc::A(ref mut alloc) = alloc_per_thread[0].0 {
    let mut input = <Alloc as Allocator<u8>>::alloc_cell(alloc, input_slice.len());
    input.slice_mut().clone_from_slice(input_slice);
    input
  } else {
    <Alloc as Allocator<u8>>::AllocatedMemory::default()
  };
  let mut owned_input = Owned::new(input);
  let ret = CompressMulti(params, &mut owned_input, output, alloc_per_thread,thread_spawner);
  if let InternalSendAlloc::A(ref mut alloc) = alloc_per_thread[0].0 {
    <Alloc as Allocator<u8>>::free_cell(alloc, owned_input.unwrap());
  }
  ret
}


pub fn CompressMulti<Alloc:BrotliAlloc+Send+'static,
                     SliceW: SliceWrapper<u8>+Send+'static,
                     Spawner:BatchSpawnable<CompressionThreadResult<Alloc>,
                                            Alloc,
                                            SliceW>> (
  params:&BrotliEncoderParams,
  owned_input: &mut Owned<SliceW>,
  output: &mut [u8],
  alloc_per_thread:&mut [SendAlloc<CompressionThreadResult<Alloc>, Alloc, Spawner::JoinHandle>],
  mut thread_spawner: Spawner,
) -> Result<usize, ()> where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  let (mut alloc, alloc_rest) = alloc_per_thread.split_at_mut(1);
  let mut retrieve_owned_input = thread_spawner.batch_spawn(owned_input, alloc_rest, |_index,_input,mut alloc|{
    let mem = <Alloc as Allocator<u8>>::alloc_cell(&mut alloc, 0);
    CompressionThreadResult::<Alloc>{
      compressed:mem,
      compressed_size:0,
      alloc:alloc,
    }
  });
  {
    let input = retrieve_owned_input.view();
    
  }
  Err(())
}

