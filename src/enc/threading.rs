use alloc::{Allocator, SliceWrapper, SliceWrapperMut};
use core::marker::PhantomData;
use core::mem;
use core::any;
#[cfg(not(feature="no-stdlib"))]
use std;
use super::BrotliAlloc;
use super::encode::{
  BrotliEncoderOperation,
  BrotliEncoderCreateInstance,
  BrotliEncoderSetCustomDictionary,
  BrotliEncoderDestroyInstance,
  BrotliEncoderMaxCompressedSize,
  BrotliEncoderCompressStream,
};
use concat::{
  BroCatli,
  BroCatliResult,
};
use core::ops::Range;
use super::backward_references::BrotliEncoderParams;
pub type PoisonedThreadError = ();

#[cfg(not(feature="no-stdlib"))]
pub type LowLevelThreadError = std::boxed::Box<any::Any + Send + 'static>;
#[cfg(feature="no-stdlib")]
pub type LowLevelThreadError = ();


pub trait AnyBoxConstructor {
    fn new(data:LowLevelThreadError) -> Self;
}

pub trait Joinable<T:Send+'static, U:Send+'static>:Sized {
  fn join(self) -> Result<T, U>;
}
#[derive(Debug)]
pub enum BrotliEncoderThreadError {
    InsufficientOutputSpace,
    ConcatenationDidNotProcessFullFile,
    ConcatenationError(BroCatliResult),
    ConcatenationFinalizationError(BroCatliResult),
    OtherThreadPanic,
    ThreadExecError(LowLevelThreadError),
}

impl AnyBoxConstructor for BrotliEncoderThreadError {
    fn new(data:LowLevelThreadError) -> Self {
        BrotliEncoderThreadError::ThreadExecError(data)
    }
}


pub struct CompressedFileChunk<Alloc:BrotliAlloc+Send+'static> where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
    data_backing:<Alloc as Allocator<u8>>::AllocatedMemory,
    data_size: usize,
}
pub struct CompressionThreadResult<Alloc:BrotliAlloc+Send+'static> where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  compressed: Result<CompressedFileChunk<Alloc>, BrotliEncoderThreadError>,
  alloc: Alloc,
}
pub enum InternalSendAlloc<T:Send+'static, Alloc:BrotliAlloc+Send+'static, Join: Joinable<T, BrotliEncoderThreadError>>
  where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  A(Alloc),
  Join(Join),
  SpawningOrJoining(PhantomData<T>),
}
pub struct SendAlloc<T:Send+'static,
                     Alloc:BrotliAlloc +Send+'static,
                     Join:Joinable<T, BrotliEncoderThreadError>>(pub InternalSendAlloc<T, Alloc, Join>)//FIXME pub
  where <Alloc as Allocator<u8>>::AllocatedMemory: Send;

impl<T:Send+'static, Alloc:BrotliAlloc+Send+'static,Join:Joinable<T, BrotliEncoderThreadError>> SendAlloc<T, Alloc, Join>
  where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  pub fn new(alloc: Alloc) -> Self {
    SendAlloc::<T, Alloc, Join>(InternalSendAlloc::A(alloc))
  }
  pub fn unwrap_or(self, other: Alloc) -> Alloc {
    match self.0 {
      InternalSendAlloc::A(alloc) => {
        alloc
      },
      InternalSendAlloc::SpawningOrJoining(_) | InternalSendAlloc::Join(_) => {
        other
      },
    }
  }
  pub fn unwrap(self) -> Alloc {
    match self.0 {
      InternalSendAlloc::A(alloc) => {
        alloc
      },
      InternalSendAlloc::SpawningOrJoining(_) | InternalSendAlloc::Join(_) => panic!("Item permanently borrowed/leaked"),
    }
  }
  pub fn replace_with_default(&mut self) -> Alloc {
    match mem::replace(&mut self.0, InternalSendAlloc::SpawningOrJoining(PhantomData::default())) {
      InternalSendAlloc::A(alloc) => {
        alloc
      },
      InternalSendAlloc::SpawningOrJoining(_) | InternalSendAlloc::Join(_) => panic!("Item permanently borrowed/leaked"),
    }
  }
}

pub enum InternalOwned<T> { // FIXME pub
  Item(T),
  Borrowed,
}

pub struct Owned<T>(pub InternalOwned<T>); // FIXME pub
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
  pub fn view(&self) -> &T {
    if let InternalOwned::Item(ref x) = self.0 {
      x
    } else {
      panic!("Item permanently borrowed")
    }
  }
}



pub trait OwnedRetriever<U:Send+'static> {
  fn view<T, F:FnOnce(&U)-> T>(&self, f:F) -> Result<T, PoisonedThreadError>;
  fn unwrap(self) -> Result<U, PoisonedThreadError>;
}

#[cfg(not(feature="no-stdlib"))]
impl<U:Send+'static> OwnedRetriever<U> for std::sync::Arc<std::sync::RwLock<U>> {
  fn view<T, F:FnOnce(&U)-> T>(&self, f:F) -> Result<T, PoisonedThreadError> {
      match self.read() {
          Ok(ref u) => Ok(f(u)),
          Err(_) => Err(PoisonedThreadError::default()),
      }
  }
  fn unwrap(self) -> Result<U, PoisonedThreadError> {
    match std::sync::Arc::try_unwrap(self) {
      Ok(rwlock) => match rwlock.into_inner() {
        Ok(u) => Ok(u),
        Err(_) => Err(PoisonedThreadError::default()),
      },
      Err(_) => Err(PoisonedThreadError::default()),
    }
  }
}



pub trait BatchSpawnable<T:Send+'static,
                         Alloc:BrotliAlloc+Send+'static,
                         U:Send+'static+Sync>
  where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static
{
  type JoinHandle: Joinable<T, BrotliEncoderThreadError>;
  type FinalJoinHandle: OwnedRetriever<U>;
  // this function takes in an input slice
  // a SendAlloc per thread and converts them all into JoinHandle
  // the input is borrowed until the joins complete
  // owned is set to borrowed
  // the final join handle is a r/w lock which will return the SliceW to the owner
  // the FinalJoinHandle is only to be called when each individual JoinHandle has been examined
  // the function is called with the thread_index, the num_threads, a reference to the slice under a read lock,
  // and an allocator from the alloc_per_thread
  fn batch_spawn<F: Fn(usize, usize, &U, Alloc) -> T+Send+'static+Copy>(
    &mut self,
    input: &mut Owned<U>,
    alloc_per_thread:&mut [SendAlloc<T, Alloc, Self::JoinHandle>],
    f: F,
  ) -> Self::FinalJoinHandle;
}

pub trait BatchSpawnableLite<T:Send+'static,
                         Alloc:BrotliAlloc+Send+'static,
                         U:Send+'static+Sync>
  where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static
{
  type JoinHandle: Joinable<T, BrotliEncoderThreadError>;
  type FinalJoinHandle: OwnedRetriever<U>;
  fn batch_spawn(
    &mut self,
    input: &mut Owned<U>,
    alloc_per_thread:&mut [SendAlloc<T, Alloc, Self::JoinHandle>],
    f: fn(usize, usize, &U, Alloc) -> T,
  ) -> Self::FinalJoinHandle;
}
/*
impl<T:Send+'static,
     Alloc:BrotliAlloc+Send+'static,
     U:Send+'static+Sync>
     BatchSpawnableLite<T, Alloc, U> for BatchSpawnable<T, Alloc, U> {
  type JoinHandle = <Self as BatchSpawnable<T, Alloc, U>>::JoinHandle;
  type FinalJoinHandle = <Self as BatchSpawnable<T, Alloc, U>>::FinalJoinHandle;
  fn batch_spawn(
    &mut self,
    input: &mut Owned<U>,
    alloc_per_thread:&mut [SendAlloc<T, Alloc, Self::JoinHandle>],
    f: fn(usize, usize, &U, Alloc) -> T,
  ) -> Self::FinalJoinHandle {
   <Self as BatchSpawnable<T, Alloc, U>>::batch_spawn(self, input, alloc_per_thread, f)
  }
}*/

pub fn CompressMultiSlice<Alloc:BrotliAlloc+Send+'static,
                          Spawner:BatchSpawnableLite<CompressionThreadResult<Alloc>,
                                                 Alloc,
                                                 (<Alloc as Allocator<u8>>::AllocatedMemory, BrotliEncoderParams)>> (
  params:&BrotliEncoderParams,
  input_slice: &[u8],
  output: &mut [u8],
  alloc_per_thread:&mut [SendAlloc<CompressionThreadResult<Alloc>, Alloc, Spawner::JoinHandle>],
  thread_spawner: &mut Spawner,
) -> Result<usize, BrotliEncoderThreadError> where <Alloc as Allocator<u8>>::AllocatedMemory: Send+Sync {
  let input = if let InternalSendAlloc::A(ref mut alloc) = alloc_per_thread[0].0 {
    let mut input = <Alloc as Allocator<u8>>::alloc_cell(alloc, input_slice.len());
    input.slice_mut().clone_from_slice(input_slice);
    input
  } else {
    <Alloc as Allocator<u8>>::AllocatedMemory::default()
  };
  let mut owned_input = Owned::new(input);
  let ret = CompressMulti(params, &mut owned_input, output, alloc_per_thread, thread_spawner);
  if let InternalSendAlloc::A(ref mut alloc) = alloc_per_thread[0].0 {
    <Alloc as Allocator<u8>>::free_cell(alloc, owned_input.unwrap());
  }
  ret
}

fn get_range(thread_index: usize, num_threads: usize, file_size: usize) -> Range<usize> {
    ((thread_index * file_size) / num_threads)..(((thread_index + 1) * file_size) / num_threads)
}

fn compress_part<Alloc: BrotliAlloc+Send+'static,
                 SliceW:SliceWrapper<u8>>(
  thread_index: usize,
  num_threads: usize,
  input_and_params:&(SliceW, BrotliEncoderParams),
  mut alloc: Alloc,
) -> CompressionThreadResult<Alloc> where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static {
  let mut range = get_range(thread_index + 1, num_threads + 1, input_and_params.0.len());
  let mut mem = <Alloc as Allocator<u8>>::alloc_cell(&mut alloc,
                                                     BrotliEncoderMaxCompressedSize(range.end - range.start));
  let mut state = BrotliEncoderCreateInstance(alloc);
  state.params = input_and_params.1.clone();
  state.params.catable = true; // make sure we can concatenate this to the other work results
  state.params.appendable = true; // make sure we are at least appendable, so that future items can be catted in
  state.params.magic_number = false; // no reason to pepper this around
  BrotliEncoderSetCustomDictionary(&mut state, range.start, &input_and_params.0.slice()[..range.start]);
  let mut out_offset = 0usize;
  let compression_result;
  let mut available_out = mem.len();
  loop {
    let mut next_in_offset = 0usize;
    let mut available_in = range.end - range.start;
    let result = BrotliEncoderCompressStream(&mut state,
                                             BrotliEncoderOperation::BROTLI_OPERATION_FINISH,
                                             &mut available_in,
                                             &input_and_params.0.slice()[range.clone()],
                                             &mut next_in_offset,  
                                             &mut available_out,
                                             mem.slice_mut(),
                                             &mut out_offset,
                                             &mut None,
                                             &mut |_a,_b,_c,_d|());
    let new_range = range.start + next_in_offset..range.end;
    range = new_range;
    if result != 0 {
      compression_result = Ok(out_offset);
      break;
    } else if available_out == 0 {
      compression_result = Err(BrotliEncoderThreadError::InsufficientOutputSpace); // mark no space??
      break;
    }
  }
  BrotliEncoderDestroyInstance(&mut state);
    match compression_result {
        Ok(size) => {
            CompressionThreadResult::<Alloc>{
                compressed:Ok(CompressedFileChunk{data_backing:mem, data_size:size}),
                alloc:state.m8,
            }
        },
        Err(e) => {
            <Alloc as Allocator<u8>>::free_cell(&mut state.m8, mem);
            CompressionThreadResult::<Alloc>{
                compressed:Err(e),
                alloc:state.m8,
            }
        },
    }
}

pub fn CompressMulti<Alloc:BrotliAlloc+Send+'static,
                     SliceW: SliceWrapper<u8>+Send+'static+Sync,
                     Spawner:BatchSpawnableLite<CompressionThreadResult<Alloc>,
                                            Alloc,
                                            (SliceW, BrotliEncoderParams)>> (
  params:&BrotliEncoderParams,
  owned_input: &mut Owned<SliceW>,
  output: &mut [u8],
  alloc_per_thread:&mut [SendAlloc<CompressionThreadResult<Alloc>, Alloc, Spawner::JoinHandle>],
  thread_spawner: &mut Spawner,
) -> Result<usize, BrotliEncoderThreadError> where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
    let num_threads = alloc_per_thread.len();
    let (alloc, alloc_rest) = alloc_per_thread.split_at_mut(1);
    let actually_owned_mem = mem::replace(owned_input, Owned(InternalOwned::Borrowed));
    let mut owned_input_pair = Owned::new((actually_owned_mem.unwrap(), params.clone()));
    let retrieve_owned_input = thread_spawner.batch_spawn(&mut owned_input_pair, alloc_rest, compress_part);
    let output_len = output.len();
    let first_thread_output_max_len = if alloc_rest.len() != 0 { output_len / 2 } else {output.len()};
    let mut compression_result;
    let mut available_out = first_thread_output_max_len;
    {
        let first_thread_output = &mut output[output_len - first_thread_output_max_len..];
        compression_result = match retrieve_owned_input.view(move |input_and_params:&(SliceW, BrotliEncoderParams)| -> Result<usize,BrotliEncoderThreadError> {
            let compression_result_inner;
            let mut state = BrotliEncoderCreateInstance(match mem::replace(&mut alloc[0].0,
                                                                           InternalSendAlloc::SpawningOrJoining(PhantomData::default())) {
            InternalSendAlloc::A(a) => a,
                _ => panic!("all public interfaces which create SendAlloc can only specify subtype A")
            });
            state.params = params.clone();
            state.params.appendable = true; // make sure we are at least appendable, so that future items can be catted in
            let mut out_offset = 0usize;

            let mut range = get_range(0, num_threads, (input_and_params.0).len());
            loop {
                assert_eq!(range.start, 0);
                let mut next_in_offset = 0usize;
                let mut available_in = range.end - range.start;
                let result = BrotliEncoderCompressStream(&mut state,
                                                         BrotliEncoderOperation::BROTLI_OPERATION_FINISH,
                                                         &mut available_in,
                                                         &(input_and_params.0).slice()[range.clone()],//fixme *
                                                         &mut next_in_offset,
                                                         &mut available_out,
                                                         first_thread_output,
                                                         &mut out_offset,
                                                         &mut None,
                                                         &mut |_a,_b,_c,_d|());
                let new_range = range.start + next_in_offset..range.end;
                range = new_range;
                if result != 0 {
                    compression_result_inner = Ok(out_offset);
                    break;
                } else if available_out == 0 {
                    compression_result_inner = Err(BrotliEncoderThreadError::InsufficientOutputSpace); // mark no space??
                    break
                }
            }
            BrotliEncoderDestroyInstance(&mut state);
            alloc[0].0 = InternalSendAlloc::A(state.m8);
            compression_result_inner
        }) {
            Ok(res) => {
                res
            },
            Err(_) =>
                Err(BrotliEncoderThreadError::OtherThreadPanic), // lock got poisoned
        }
    };
    let mut bro_cat_li = BroCatli::new();
    if alloc_rest.len() != 0 {
      if let Ok(first_file_size) = compression_result {
        bro_cat_li.new_brotli_file();
        let (cat_output, cat_input) = output.split_at_mut(output_len - first_thread_output_max_len);
        let mut in_offset = 0usize;
        let mut out_offset = 0usize;
        let cat_result = bro_cat_li.stream(&cat_input[..first_file_size],
                                           &mut in_offset,
                                           cat_output,
                                           &mut out_offset);
        match cat_result {
          BroCatliResult::Success | BroCatliResult::NeedsMoreInput  => {
            if in_offset != first_file_size {
              compression_result = Err(BrotliEncoderThreadError::ConcatenationDidNotProcessFullFile); // wasn't able to ingest the full file
            } else {
               compression_result = Ok(out_offset);
            }
          },
          BroCatliResult::NeedsMoreOutput => {
            compression_result = Err(BrotliEncoderThreadError::InsufficientOutputSpace); // not enough space
          },
          err => {
            compression_result = Err(BrotliEncoderThreadError::ConcatenationError(err)); // misc error
          },
        }
      }
    }
    if let Ok(mut out_file_size) = compression_result {
     for thread in alloc_rest.iter_mut() {
       match mem::replace(&mut thread.0, InternalSendAlloc::SpawningOrJoining(PhantomData::default())) {
         InternalSendAlloc::A(_) | InternalSendAlloc::SpawningOrJoining(_) => panic!("Thread not properly spawned"),
         InternalSendAlloc::Join(join) => match join.join() {
           Ok(mut result) => {
             match result.compressed {
               Ok(compressed_out) => {
                 bro_cat_li.new_brotli_file();
                 let mut in_offset = 0usize;
                 let cat_result = bro_cat_li.stream(&compressed_out.data_backing.slice()[..compressed_out.data_size],
                                                    &mut in_offset,
                                                    output,
                                                    &mut out_file_size);
                 match cat_result {
                   BroCatliResult::Success | BroCatliResult::NeedsMoreInput  => {
                     compression_result = Ok(out_file_size);
                   },
                   BroCatliResult::NeedsMoreOutput => {
                     compression_result = Err(BrotliEncoderThreadError::InsufficientOutputSpace); // not enough space
                   },
                   err => {
                     compression_result = Err(BrotliEncoderThreadError::ConcatenationError(err)); // misc error
                   },
                 }
                 <Alloc as Allocator<u8>>::free_cell(&mut result.alloc, compressed_out.data_backing);
               }
               Err(e) => {
                   compression_result = Err(e);
               }
             }
             thread.0 = InternalSendAlloc::A(result.alloc);
           },
           Err(err) => {
             compression_result = Err(err);
           },
         },
       }
     }
     if alloc_rest.len() != 0 {
       match bro_cat_li.finish(output, &mut out_file_size) {
         BroCatliResult::Success => compression_result = Ok(out_file_size),
         err => compression_result = Err(BrotliEncoderThreadError::ConcatenationFinalizationError(err)),
       }
     }
   }

  if let Ok(retrieved_owned_input) = retrieve_owned_input.unwrap() {
      *owned_input = Owned::new(retrieved_owned_input.0); // return the input to its rightful owner before returning
  } else {
    if let Ok(_) = compression_result {
          compression_result = Err(BrotliEncoderThreadError::OtherThreadPanic);
      }
  }
  compression_result
}

