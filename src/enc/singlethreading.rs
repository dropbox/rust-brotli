use core::mem;
#[cfg(feature="std")]
use std;
use alloc::{SliceWrapper, Allocator};
use enc::BrotliAlloc;
use enc::BrotliEncoderParams;
use core::marker::PhantomData;
use super::backward_references::{UnionHasher};
use enc::threading::{
  CompressMulti,
  SendAlloc,
  InternalSendAlloc,
  BatchSpawnable,
  BatchSpawnableLite,
  Joinable,
  Owned,
  OwnedRetriever,
  CompressionThreadResult,
  InternalOwned,
  PoisonedThreadError,
  BrotliEncoderThreadError,
};





pub struct SingleThreadedJoinable<T:Send+'static, U:Send+'static> {
  result:Result<T, U>,
}
impl<T:Send+'static, U:Send+'static> Joinable<T, U> for SingleThreadedJoinable<T, U> {
  fn join(self) -> Result<T, U> {
    self.result
  }
}
#[cfg(feature="std")]
pub struct SingleThreadedOwnedRetriever<U:Send+'static>(std::sync::RwLock<U>);
#[cfg(feature="std")]
impl<U:Send+'static> OwnedRetriever<U> for SingleThreadedOwnedRetriever<U> {
  fn view<T, F:FnOnce(&U)-> T>(&self, f:F) -> Result<T, PoisonedThreadError> {
    Ok(f(&*self.0.read().unwrap()))
  }
  fn unwrap(self) -> Result<U,PoisonedThreadError> {Ok(self.0.into_inner().unwrap())}
}
#[cfg(feature="std")]
impl<U:Send+'static> SingleThreadedOwnedRetriever<U> {
    fn new(u:U) -> Self {
        SingleThreadedOwnedRetriever(std::sync::RwLock::new(u))
    }
}

#[cfg(not(feature="std"))]
pub struct SingleThreadedOwnedRetriever<U:Send+'static>(U);
#[cfg(not(feature="std"))]
impl<U:Send+'static> SingleThreadedOwnedRetriever<U> {
    fn new(u:U) -> Self {
        SingleThreadedOwnedRetriever(u)
    }
}
#[cfg(not(feature="std"))]
impl<U:Send+'static> OwnedRetriever<U> for SingleThreadedOwnedRetriever<U> {
  fn view<T, F:FnOnce(&U)->T>(&self, f:F) -> Result<T, PoisonedThreadError> {
    Ok(f(&self.0))
  }
  fn unwrap(self) -> Result<U,PoisonedThreadError> {Ok(self.0)}
}

#[derive(Default)]
pub struct SingleThreadedSpawner{}

impl<ReturnValue:Send+'static, ExtraInput:Send+'static, Alloc:BrotliAlloc+Send+'static, U:Send+'static+Sync>
  BatchSpawnable<ReturnValue, ExtraInput, Alloc, U> for SingleThreadedSpawner
where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static {
  type JoinHandle = SingleThreadedJoinable<ReturnValue, BrotliEncoderThreadError>;
  type FinalJoinHandle = SingleThreadedOwnedRetriever<U>;
    fn batch_spawn<F: Fn(ExtraInput, usize, usize, &U, Alloc) -> ReturnValue+Send+'static+Copy>(
    &mut self,
    input: &mut Owned<U>,
    alloc_per_thread:&mut [SendAlloc<ReturnValue, ExtraInput, Alloc, Self::JoinHandle>],
    f: F,
    ) -> Self::FinalJoinHandle {
      let num_threads = alloc_per_thread.len();
      for (index, work) in alloc_per_thread.iter_mut().enumerate() {
        let (alloc, extra_input) = work.replace_with_default();
        let ret = f(extra_input, index, num_threads, input.view(), alloc);
        *work = SendAlloc(InternalSendAlloc::Join(SingleThreadedJoinable{result:Ok(ret)}));
      }
      SingleThreadedOwnedRetriever::<U>::new(mem::replace(input, Owned(InternalOwned::Borrowed)).unwrap())
    }
}

impl<ReturnValue:Send+'static,
     ExtraInput:Send+'static,
     Alloc:BrotliAlloc+Send+'static,
     U:Send+'static+Sync>
  BatchSpawnableLite<ReturnValue, ExtraInput, Alloc, U> for SingleThreadedSpawner
  where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static {
  type JoinHandle = <SingleThreadedSpawner as BatchSpawnable<ReturnValue, ExtraInput, Alloc, U>>::JoinHandle;
  type FinalJoinHandle = <SingleThreadedSpawner as BatchSpawnable<ReturnValue, ExtraInput, Alloc, U>>::FinalJoinHandle;
  fn batch_spawn(
    &mut self,
    input: &mut Owned<U>,
    alloc_per_thread:&mut [SendAlloc<ReturnValue, ExtraInput, Alloc, Self::JoinHandle>],
    f: fn(ExtraInput, usize, usize, &U, Alloc) -> ReturnValue,
  ) -> Self::FinalJoinHandle {
   <Self as BatchSpawnable<ReturnValue, ExtraInput, Alloc, U>>::batch_spawn(self, input, alloc_per_thread, f)
  }
}


pub fn compress_multi<Alloc:BrotliAlloc+Send+'static,
                      SliceW: SliceWrapper<u8>+Send+'static+Sync> (
  params:&BrotliEncoderParams,
  owned_input: &mut Owned<SliceW>,
  output: &mut [u8],
  alloc_per_thread:&mut [SendAlloc<CompressionThreadResult<Alloc>,
                                   UnionHasher<Alloc>,
                                   Alloc,
                                   <SingleThreadedSpawner as BatchSpawnable<CompressionThreadResult<Alloc>,UnionHasher<Alloc>, Alloc, SliceW>>::JoinHandle>],
) -> Result<usize, BrotliEncoderThreadError> where <Alloc as Allocator<u8>>::AllocatedMemory: Send, <Alloc as Allocator<u16>>::AllocatedMemory: Send+Sync, <Alloc as Allocator<u32>>::AllocatedMemory: Send+Sync {
  CompressMulti(params, owned_input, output, alloc_per_thread, &mut SingleThreadedSpawner::default())
}

pub struct WorkerPool<A,B,C> {
  a: PhantomData<A>,
  b: PhantomData<B>,
  c: PhantomData<C>,
}
pub fn new_work_pool<A,B, C>(_num_threads: usize) -> WorkerPool<A, B, C>{
  WorkerPool::<A,B,C>{
    a:PhantomData::default(),
    b:PhantomData::default(),
    c:PhantomData::default(),
  }
}

pub fn compress_worker_pool<Alloc:BrotliAlloc+Send+'static,
                      SliceW: SliceWrapper<u8>+Send+'static+Sync> (
  params:&BrotliEncoderParams,
  owned_input: &mut Owned<SliceW>,
  output: &mut [u8],
  alloc_per_thread:&mut [SendAlloc<CompressionThreadResult<Alloc>,
                                   UnionHasher<Alloc>,
                                   Alloc,
                                   <SingleThreadedSpawner as BatchSpawnable<CompressionThreadResult<Alloc>,
                                                                            UnionHasher<Alloc>,
                                                                            Alloc,
                                                                            SliceW>>::JoinHandle>],
  _worker_pool:&mut WorkerPool<CompressionThreadResult<Alloc>, Alloc, (SliceW, BrotliEncoderParams)>,
) -> Result<usize, BrotliEncoderThreadError> where <Alloc as Allocator<u8>>::AllocatedMemory: Send, <Alloc as Allocator<u16>>::AllocatedMemory: Send+Sync, <Alloc as Allocator<u32>>::AllocatedMemory: Send+Sync {
  compress_multi(params, owned_input, output, alloc_per_thread)
}
