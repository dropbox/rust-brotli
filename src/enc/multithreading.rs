#![cfg(not(feature="no-stdlib"))]
use core::mem;
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
  Joinable,
  Owned,
  OwnedRetriever,
  CompressionThreadResult,
  InternalOwned,
  BrotliEncoderThreadError,
    AnyBoxConstructor,
    PoisonedThreadError,
};

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
/*
pub struct MultiThreadedOwnedRetriever<U:Send+'static>(RwLock<U>);

impl<U:Send+'static+Default> OwnedRetriever<U> for MultiThreadedOwnedRetriever<U> {
  fn view(&self) -> Result<&U, PoisonedThreadError> {
      match self.0.read() {
          Ok(u) => Ok(u),
          Err(_) => PoisonedThreadError::default(),
      }
  }
  fn unwrap(self) -> Result<U, PoisonedThreadError> {
      match self.0.into_inner() {
          Ok(u) => Ok(u),
          Err(_) => PoisonedThreadError::default(),
      }
  }
}


#[derive(Default)]
pub struct SingleThreadedSpawner{}

impl<T:Send+'static, Alloc:BrotliAlloc+Send+'static, U:Send+'static> BatchSpawnable<T, Alloc, U> for SingleThreadedSpawner
where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static {
  type JoinHandle = SingleThreadedJoinable<T, Alloc>;
  type FinalJoinHandle = SingleThreadedOwnedRetriever<U>;
    fn batch_spawn<F: Fn(usize, usize, &U, Alloc) -> T>(
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
      SingleThreadedOwnedRetriever::<U>(mem::replace(input, Owned(InternalOwned::Borrowed)).unwrap())
    }
}


pub fn compress_multi<Alloc:BrotliAlloc+Send+'static,
                      SliceW: SliceWrapper<u8>+Send+'static> (
  params:&BrotliEncoderParams,
  owned_input: &mut Owned<SliceW>,
  output: &mut [u8],
  alloc_per_thread:&mut [SendAlloc<CompressionThreadResult<Alloc>,
                                   Alloc,
                                   <SingleThreadedSpawner as BatchSpawnable<CompressionThreadResult<Alloc>,Alloc, Alloc>>::JoinHandle>],
) -> Result<usize, BrotliEncoderThreadError> where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  CompressMulti(params, owned_input, output, alloc_per_thread, SingleThreadedSpawner::default())
}
*/                      
