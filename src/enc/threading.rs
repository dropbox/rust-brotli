use alloc::{Allocator, SliceWrapper};
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
enum InternalSendAlloc<Alloc:BrotliAlloc+Send+'static, Join: Joinable<CompressionThreadResult<Alloc>>>
  where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  A(Alloc),
  Join(Join),
  Spawning,
}
pub struct SendAlloc<Alloc:BrotliAlloc +Send+'static,
                     Join:Joinable<CompressionThreadResult<Alloc>>>(InternalSendAlloc<Alloc, Join>)
  where <Alloc as Allocator<u8>>::AllocatedMemory: Send;

impl<Alloc:BrotliAlloc+Send+'static,Join:Joinable<CompressionThreadResult<Alloc>>> SendAlloc<Alloc, Join>
  where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  pub fn new(alloc: Alloc) -> Self {
    SendAlloc::<Alloc, Join>(InternalSendAlloc::A(alloc))
  }
  pub fn unwrap_or(self, other: Alloc) -> Alloc {
    match self.0 {
      InternalSendAlloc::A(alloc) => {
        alloc
      },
      InternalSendAlloc::Spawning | InternalSendAlloc::Join(_) => {
        other
      },
    }
  }
  pub fn unwrap(self) -> Alloc {
    match self.0 {
      InternalSendAlloc::A(alloc) => {
        alloc
      },
      InternalSendAlloc::Join(_) | InternalSendAlloc::Spawning => panic!("Item permanently borrowed/leaked"),
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
  fn unwrap(self) -> SliceW;
}

pub trait BatchSpawnable<Alloc:BrotliAlloc+Send+'static, SliceW:SliceWrapper<u8>+Send+'static>
  where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static
{
  type JoinHandle: Joinable<CompressionThreadResult<Alloc>>;
  type FinalJoinHandle: OwnedRetriever<SliceW>;
  // this function takes in a SendAlloc per thread and converts them all into JoinHandle
  // the input is borrowed until the joins complete
  // owned is set to borrowed
  // the final join handle is a r/w lock which will return the SliceW to the owner
  // the FinalJoinHandle is only to be called when each individual JoinHandle has been examined
  fn batch_spawn<F: FnOnce() -> CompressionThreadResult<Alloc> + Send + 'static>(
    input: &mut Owned<SliceW>,
    alloc_per_thread:&mut [SendAlloc<Alloc, Self::JoinHandle>],
    f: F,
  ) -> Self::FinalJoinHandle;
}



pub fn CompressMulti<Alloc:BrotliAlloc+Send+'static, SliceW: SliceWrapper<u8>+Send+'static, Spawner:BatchSpawnable<Alloc, SliceW>> (
  params:&BrotliEncoderParams,
  input: &mut Owned<SliceW>,
  output: &mut [u8],
  alloc_per_thread:&mut [SendAlloc<Alloc, Spawner::JoinHandle>],
  thread_spawner: Spawner,
) -> Result<usize, ()> where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  
  Err(())
}

