#![cfg(not(feature="no-stdlib"))]
use core::mem;
use std;
use core::marker::PhantomData;
use std::thread::{
    JoinHandle,
};
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
  BrotliEncoderThreadError,
  AnyBoxConstructor,
  PoisonedThreadError,
};

// in-place thread create

use std::sync::RwLock;


pub struct MultiThreadedJoinable<T:Send+'static, U:Send+'static>(JoinHandle<T>, PhantomData<U>);

impl<T:Send+'static, U:Send+'static+AnyBoxConstructor> Joinable<T, U> for MultiThreadedJoinable<T, U> {
  fn join(self) -> Result<T, U> {
      match self.0.join() {
          Ok(t) => Ok(t),
          Err(e) => Err(<U as AnyBoxConstructor>::new(e)),
      }
  }
}
pub struct MultiThreadedOwnedRetriever<U:Send+'static>(RwLock<U>);

impl<U:Send+'static> OwnedRetriever<U> for MultiThreadedOwnedRetriever<U> {
  fn view<T, F:FnOnce(&U)->T>(&self, mut f:F) -> Result<T, PoisonedThreadError> {
      match self.0.read() {
          Ok(u) => Ok(f(&*u)),
          Err(_) => Err(PoisonedThreadError::default()),
      }
  }
  fn unwrap(self) -> Result<U, PoisonedThreadError> {
      match self.0.into_inner() {
          Ok(u) => Ok(u),
          Err(_) => Err(PoisonedThreadError::default()),
      }
  }
}


#[derive(Default)]
pub struct MultiThreadedSpawner{}


fn spawn_work<T:Send+'static, F: Fn(usize, usize, &U, Alloc) -> T+Send+'static, Alloc:BrotliAlloc+Send+'static, U:Send+'static+Sync>(index: usize, num_threads: usize, locked_input:std::sync::Arc<RwLock<U>>, alloc:Alloc, f:F) -> std::thread::JoinHandle<T>
where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static {
  std::thread::spawn(move || {
      let t:T = locked_input.view(move |guard:&U|->T {
          f(index, num_threads, guard, alloc)
      }).unwrap();
      t
  })
}

impl<T:Send+'static, Alloc:BrotliAlloc+Send+'static, U:Send+'static+Sync> BatchSpawnable<T, Alloc, U> for MultiThreadedSpawner
where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static {
  type JoinHandle = MultiThreadedJoinable<T, BrotliEncoderThreadError>;
  type FinalJoinHandle = std::sync::Arc<RwLock<U>>;
    fn batch_spawn<F: Fn(usize, usize, &U, Alloc) -> T+Send+'static+Copy>(
    &mut self,
    input: &mut Owned<U>,
    alloc_per_thread:&mut [SendAlloc<T, Alloc, Self::JoinHandle>],
    f: F,
    ) -> Self::FinalJoinHandle {
      let num_threads = alloc_per_thread.len();
      let locked_input = std::sync::Arc::<RwLock<U>>::new(RwLock::new(mem::replace(input, Owned(InternalOwned::Borrowed)).unwrap()));
      for (index, work) in alloc_per_thread.iter_mut().enumerate() {
        let alloc = work.replace_with_default();
        let ret = spawn_work(index, num_threads, locked_input.clone(), alloc, f);
        *work = SendAlloc(InternalSendAlloc::Join(MultiThreadedJoinable(ret, PhantomData::default())));
      }
      locked_input
    }
}
impl<T:Send+'static,
     Alloc:BrotliAlloc+Send+'static,
     U:Send+'static+Sync>
  BatchSpawnableLite<T, Alloc, U> for MultiThreadedSpawner
  where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static {
  type JoinHandle = <MultiThreadedSpawner as BatchSpawnable<T, Alloc, U>>::JoinHandle;
  type FinalJoinHandle = <MultiThreadedSpawner as BatchSpawnable<T, Alloc, U>>::FinalJoinHandle;
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
                                   <MultiThreadedSpawner as BatchSpawnable<CompressionThreadResult<Alloc>,Alloc, SliceW>>::JoinHandle>],
) -> Result<usize, BrotliEncoderThreadError> where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  CompressMulti(params, owned_input, output, alloc_per_thread, &mut MultiThreadedSpawner::default())
}

