use alloc::{Allocator, SliceWrapper, SliceWrapperMut};
use core::marker::PhantomData;
use core::mem;
use super::BrotliAlloc;
use super::encode::{
  BrotliEncoderOperation,
  BrotliEncoderCreateInstance,
  BrotliEncoderSetCustomDictionary,
  BrotliEncoderDestroyInstance,
  BrotliEncoderMaxCompressedSize,
  BrotliEncoderCompressStream,
};
use core::ops::Range;
use super::backward_references::BrotliEncoderParams;
pub trait Joinable<T:Send+'static, U:Send+'static>:Sized {
  fn join(self) -> Result<T, U>;
}


pub struct CompressionThreadResult<Alloc:BrotliAlloc+Send+'static> where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  compressed: <Alloc as Allocator<u8>>::AllocatedMemory,
  compressed_size: usize,
  alloc: Alloc,
}
enum InternalSendAlloc<T:Send+'static, Alloc:BrotliAlloc+Send+'static, Join: Joinable<T, Alloc>>
  where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
  A(Alloc),
  Join(Join),
  SpawningOrJoining(PhantomData<T>),
}
pub struct SendAlloc<T:Send+'static,
                     Alloc:BrotliAlloc +Send+'static,
                     Join:Joinable<T, Alloc>>(InternalSendAlloc<T, Alloc, Join>)
  where <Alloc as Allocator<u8>>::AllocatedMemory: Send;

impl<T:Send+'static, Alloc:BrotliAlloc+Send+'static,Join:Joinable<T, Alloc>> SendAlloc<T, Alloc, Join>
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

pub trait OwnedRetriever<U:Send+'static> {
  fn view(&self) -> &U;
  fn unwrap(self) -> U;
}

pub trait BatchSpawnable<T:Send+'static,
                         Alloc:BrotliAlloc+Send+'static,
                         U:Send+'static>
  where <Alloc as Allocator<u8>>::AllocatedMemory:Send+'static
{
  type JoinHandle: Joinable<T, Alloc>;
  type FinalJoinHandle: OwnedRetriever<U>;
  // this function takes in an input slice
  // a SendAlloc per thread and converts them all into JoinHandle
  // the input is borrowed until the joins complete
  // owned is set to borrowed
  // the final join handle is a r/w lock which will return the SliceW to the owner
  // the FinalJoinHandle is only to be called when each individual JoinHandle has been examined
  // the function is called with the thread_index, the num_threads, a reference to the slice under a read lock,
  // and an allocator from the alloc_per_thread
  fn batch_spawn<F: Fn(usize, usize, &U, Alloc) -> T>(
    &mut self,
    input: &mut Owned<U>,
    alloc_per_thread:&mut [SendAlloc<T, Alloc, Self::JoinHandle>],
    f: F,
  ) -> Self::FinalJoinHandle;
}



pub fn CompressMultiSlice<Alloc:BrotliAlloc+Send+'static,
                          Spawner:BatchSpawnable<CompressionThreadResult<Alloc>,
                                                 Alloc,
                                                 (<Alloc as Allocator<u8>>::AllocatedMemory, BrotliEncoderParams)>> (
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
  assert_eq!(range.start, 0);
  let mut out_offset = 0usize;
  let mut compression_result = Err(());
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
      compression_result = Err(()); // mark no space??
      break
    }
  }
  BrotliEncoderDestroyInstance(&mut state);
  
  CompressionThreadResult::<Alloc>{
    compressed:mem,
    compressed_size:out_offset,
    alloc:state.m8,
  }  
}

pub fn CompressMulti<Alloc:BrotliAlloc+Send+'static,
                     SliceW: SliceWrapper<u8>+Send+'static,
                     Spawner:BatchSpawnable<CompressionThreadResult<Alloc>,
                                            Alloc,
                                            (SliceW, BrotliEncoderParams)>> (
  params:&BrotliEncoderParams,
  owned_input: &mut Owned<SliceW>,
  output: &mut [u8],
  alloc_per_thread:&mut [SendAlloc<CompressionThreadResult<Alloc>, Alloc, Spawner::JoinHandle>],
  mut thread_spawner: Spawner,
) -> Result<usize, ()> where <Alloc as Allocator<u8>>::AllocatedMemory: Send {
    let num_threads = alloc_per_thread.len();
    let (mut alloc, alloc_rest) = alloc_per_thread.split_at_mut(1);
    let actually_owned_mem = mem::replace(owned_input, Owned(InternalOwned::Borrowed));
    let mut owned_input_pair = Owned::new((actually_owned_mem.unwrap(), params.clone()));
    let mut retrieve_owned_input = thread_spawner.batch_spawn(&mut owned_input_pair, alloc_rest, compress_part);
    let mut compression_result = Err(());
    let mut available_out = output.len();
    {
        let input_and_params = retrieve_owned_input.view();
        let mut state = BrotliEncoderCreateInstance(match mem::replace(&mut alloc[0].0,
                                                                       InternalSendAlloc::SpawningOrJoining(PhantomData::default())) {
            InternalSendAlloc::A(a) => a,
            _ => panic!("all public interfaces which create SendAlloc can only specify subtype A")
        });
        state.params = params.clone();
        state.params.appendable = true; // make sure we are at least appendable, so that future items can be catted in
        let mut range = get_range(0, num_threads, input_and_params.0.len());
        assert_eq!(range.start, 0);
        let mut out_offset = 0usize;
        loop {
            let mut next_in_offset = 0usize;
            let mut available_in = range.end - range.start;
            let result = BrotliEncoderCompressStream(&mut state,
                                                 BrotliEncoderOperation::BROTLI_OPERATION_FINISH,
                                                 &mut available_in,
                                                 &input_and_params.0.slice()[range.clone()],
                                                 &mut next_in_offset,  
                                                 &mut available_out,
                                                 output,
                                                 &mut out_offset,
                                                 &mut None,
                                                 &mut |_a,_b,_c,_d|());
            let new_range = range.start + next_in_offset..range.end;
            range = new_range;
            if result != 0 {
                compression_result = Ok(out_offset);
                break;
            } else if available_out == 0 {
                compression_result = Err(()); // mark no space??
                break
            }
        }
        BrotliEncoderDestroyInstance(&mut state);
        alloc[0].0 = InternalSendAlloc::A(state.m8);
    }
    for thread in alloc_rest.iter_mut() {
        let alloc;
        match mem::replace(&mut thread.0, InternalSendAlloc::SpawningOrJoining(PhantomData::default())) {
            InternalSendAlloc::A(_) | InternalSendAlloc::SpawningOrJoining(_) => panic!("Thread not properly spawned"),
            InternalSendAlloc::Join(join) => match join.join() {
                Ok(result) => {
                    alloc = result.alloc;
                },
                Err(allocator) => {
                    alloc = allocator;
                    compression_result = Err(());
                },
            },
        }
        thread.0 = InternalSendAlloc::A(alloc);
    }
    *owned_input = Owned::new(retrieve_owned_input.unwrap().0); // return the input to its rightful owner before returning
    compression_result
}

