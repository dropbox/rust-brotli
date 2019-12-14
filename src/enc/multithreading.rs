#![cfg(feature="std")]
use core::mem;
use std;
use core::marker::PhantomData;
use std::thread::{
    JoinHandle,
};
use alloc::{SliceWrapper, Allocator};
use enc::BrotliAlloc;
use enc::BrotliEncoderParams;
use enc::backward_references::UnionHasher;
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
  fn view<T, F:FnOnce(&U)->T>(&self, f:F) -> Result<T, PoisonedThreadError> {
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


fn spawn_work<ReturnValue:Send+'static,
              ExtraInput:Send+'static,
              F: Fn(ExtraInput, usize, usize, &U, Alloc) -> ReturnValue+Send+'static,
              Alloc:BrotliAlloc+Send+'static,
              U:Send+'static+Sync>(
  extra_input: ExtraInput,
  index: usize, num_threads:
  usize, locked_input:std::sync::Arc<RwLock<U>>,
  alloc:Alloc, f:F,
) -> std::thread::JoinHandle<ReturnValue> where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static {
  std::thread::spawn(move || {
      let t:ReturnValue = locked_input.view(move |guard:&U|->ReturnValue {
          f(extra_input, index, num_threads, guard, alloc)
      }).unwrap();
      t
  })
}

impl<ReturnValue:Send+'static, ExtraInput:Send+'static, Alloc:BrotliAlloc+Send+'static, U:Send+'static+Sync> BatchSpawnable<ReturnValue, ExtraInput, Alloc, U> for MultiThreadedSpawner
where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static {
  type JoinHandle = MultiThreadedJoinable<ReturnValue, BrotliEncoderThreadError>;
  type FinalJoinHandle = std::sync::Arc<RwLock<U>>;
    fn make_spawner(
    &mut self,
    input: &mut Owned<U>,
    ) -> Self::FinalJoinHandle {
      std::sync::Arc::<RwLock<U>>::new(RwLock::new(mem::replace(input, Owned(InternalOwned::Borrowed)).unwrap()))
    }
    fn spawn<F: Fn(ExtraInput, usize, usize, &U, Alloc) -> ReturnValue+Send+'static+Copy>(
    &mut self,
    input: &mut Self::FinalJoinHandle,
    work:&mut SendAlloc<ReturnValue, ExtraInput, Alloc, Self::JoinHandle>,
    index: usize,
    num_threads: usize,
    f: F,
    ) {
     let (alloc, extra_input) = work.replace_with_default();
     let ret = spawn_work(extra_input, index, num_threads, input.clone(), alloc, f);
     *work = SendAlloc(InternalSendAlloc::Join(MultiThreadedJoinable(ret, PhantomData::default())));
    }
}
impl<ReturnValue:Send+'static, ExtraInput:Send+'static,
     Alloc:BrotliAlloc+Send+'static,
     U:Send+'static+Sync>
  BatchSpawnableLite<ReturnValue, ExtraInput, Alloc, U> for MultiThreadedSpawner
  where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static,
        <Alloc as Allocator<u16>>::AllocatedMemory: Send+Sync,
        <Alloc as Allocator<u32>>::AllocatedMemory: Send+Sync {
  type JoinHandle = <MultiThreadedSpawner as BatchSpawnable<ReturnValue, ExtraInput, Alloc, U>>::JoinHandle;
  type FinalJoinHandle = <MultiThreadedSpawner as BatchSpawnable<ReturnValue, ExtraInput, Alloc, U>>::FinalJoinHandle;
  fn make_spawner(
    &mut self,
    input: &mut Owned<U>,
  ) -> Self::FinalJoinHandle {
     <Self as BatchSpawnable<ReturnValue, ExtraInput, Alloc, U>>::make_spawner(self, input)
  }
  fn spawn(
    &mut self,
    handle:&mut Self::FinalJoinHandle,
    alloc_per_thread:&mut SendAlloc<ReturnValue, ExtraInput, Alloc, Self::JoinHandle>,
    index: usize,
    num_threads: usize,
    f: fn(ExtraInput, usize, usize, &U, Alloc) -> ReturnValue,
  ) {
   <Self as BatchSpawnable<ReturnValue, ExtraInput, Alloc, U>>::spawn(self, handle, alloc_per_thread, index, num_threads, f)
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
                                   <MultiThreadedSpawner as BatchSpawnable<CompressionThreadResult<Alloc>,UnionHasher<Alloc>, Alloc, SliceW>>::JoinHandle>],
) -> Result<usize, BrotliEncoderThreadError>
  where <Alloc as Allocator<u8>>::AllocatedMemory: Send,
        <Alloc as Allocator<u16>>::AllocatedMemory: Send+Sync,
        <Alloc as Allocator<u32>>::AllocatedMemory: Send+Sync {
  CompressMulti(params, owned_input, output, alloc_per_thread, &mut MultiThreadedSpawner::default())
}

