use core::mem;
#[cfg(not(feature="no-stdlib"))]
use std;
use alloc::{SliceWrapper, Allocator};
use enc::BrotliAlloc;
use enc::BrotliEncoderParams;
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
  ReadGuard,
};





pub struct SingleThreadedJoinable<T:Send+'static, U:Send+'static> {
  result:Result<T, U>,
}
impl<T:Send+'static, U:Send+'static> Joinable<T, U> for SingleThreadedJoinable<T, U> {
  fn join(self) -> Result<T, U> {
    self.result
  }
}
#[cfg(not(feature="no-stdlib"))]
pub struct SingleThreadedOwnedRetriever<U:Send+'static>(std::sync::RwLock<U>);
#[cfg(not(feature="no-stdlib"))]
impl<U:Send+'static> OwnedRetriever<U> for SingleThreadedOwnedRetriever<U> {
    fn view<'a>(&'a self) -> Result<ReadGuard<'a, U>, PoisonedThreadError> {
        Ok(ReadGuard(self.0.read().unwrap()))
    }
  fn unwrap(self) -> Result<U,PoisonedThreadError> {Ok(self.0.into_inner().unwrap())}
}
#[cfg(not(feature="no-stdlib"))]
impl<U:Send+'static> SingleThreadedOwnedRetriever<U> {
    fn new(u:U) -> Self {
        SingleThreadedOwnedRetriever(std::sync::RwLock::new(u))
    }
}

#[cfg(feature="no-stdlib")]
pub struct SingleThreadedOwnedRetriever<U:Send+'static>(U);
#[cfg(feature="no-stdlib")]
impl<U:Send+'static> SingleThreadedOwnedRetriever<U> {
    fn new(u:U) -> Self {
        SingleThreadedOwnedRetriever(u)
    }
}
#[cfg(feature="no-stdlib")]
impl<U:Send+'static> OwnedRetriever<U> for SingleThreadedOwnedRetriever<U> {
  fn view<'a>(&'a self) -> Result<ReadGuard<'a, U>, PoisonedThreadError> {Ok(ReadGuard(&self.0))}
  fn unwrap(self) -> Result<U,PoisonedThreadError> {Ok(self.0)}
}

#[derive(Default)]
pub struct SingleThreadedSpawner{}

impl<T:Send+'static, Alloc:BrotliAlloc+Send+'static, U:Send+'static+Sync> BatchSpawnable<T, Alloc, U> for SingleThreadedSpawner
where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static {
  type JoinHandle = SingleThreadedJoinable<T, BrotliEncoderThreadError>;
  type FinalJoinHandle = SingleThreadedOwnedRetriever<U>;
    fn batch_spawn<F: Fn(usize, usize, &U, Alloc) -> T+Send+'static+Copy>(
    &mut self,
    input: &mut Owned<U>,
    alloc_per_thread:&mut [SendAlloc<T, Alloc, Self::JoinHandle>],
    f: F,
    ) -> Self::FinalJoinHandle {
      let num_threads = alloc_per_thread.len();
      for (index, work) in alloc_per_thread.iter_mut().enumerate() {
        let alloc = work.replace_with_default();
        let ret = f(index, num_threads, input.view(), alloc);
        *work = SendAlloc(InternalSendAlloc::Join(SingleThreadedJoinable{result:Ok(ret)}));
      }
      SingleThreadedOwnedRetriever::<U>::new(mem::replace(input, Owned(InternalOwned::Borrowed)).unwrap())
    }
}

impl<T:Send+'static,
     Alloc:BrotliAlloc+Send+'static,
     U:Send+'static+Sync>
  BatchSpawnableLite<T, Alloc, U> for SingleThreadedSpawner
  where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static {
  type JoinHandle = <SingleThreadedSpawner as BatchSpawnable<T, Alloc, U>>::JoinHandle;
  type FinalJoinHandle = <SingleThreadedSpawner as BatchSpawnable<T, Alloc, U>>::FinalJoinHandle;
  fn batch_spawn(
    &mut self,
    input: &mut Owned<U>,
    alloc_per_thread:&mut [SendAlloc<T, Alloc, Self::JoinHandle>],
    f: fn(usize, usize, &U, Alloc) -> T,
  ) -> Self::FinalJoinHandle {
   <Self as BatchSpawnable<T, Alloc, U>>::batch_spawn(self, input, alloc_per_thread, f)
  }
}


pub fn compress_multi<Alloc:BrotliAlloc+Send+'static,
                      SliceW: SliceWrapper<u8>+Send+'static+Sync> (
  params:&BrotliEncoderParams,
  owned_input: &mut Owned<SliceW>,
  output: &mut [u8],
  alloc_per_thread:&mut [SendAlloc<CompressionThreadResult<Alloc>,
                                   Alloc,
                                   <SingleThreadedSpawner as BatchSpawnable<CompressionThreadResult<Alloc>,Alloc, SliceW>>::JoinHandle>],
) -> Result<usize, BrotliEncoderThreadError> where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  CompressMulti(params, owned_input, output, alloc_per_thread, &mut SingleThreadedSpawner::default())
}
                      
