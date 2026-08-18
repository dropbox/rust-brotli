use crate::alloc::{Allocator, SliceWrapper, SliceWrapperMut};
use core::marker::PhantomData;
use core::ops::Range;
use core::{any, mem};
#[cfg(feature = "std")]
use std;

use super::BrotliAlloc;
use super::backward_references::{AnyHasher, BrotliEncoderParams, CloneWithAlloc, UnionHasher};
use super::encode::{
    BrotliEncoderDestroyInstance, BrotliEncoderMaxCompressedSize, BrotliEncoderOperation,
    SanitizeParams, hasher_setup,
};
use crate::concat::{BroCatli, BroCatliResult};
use crate::enc::combined_alloc::{alloc_default, allocate};
use crate::enc::encode::BrotliEncoderStateStruct;

pub type PoisonedThreadError = ();

#[cfg(feature = "std")]
pub type LowLevelThreadError = std::boxed::Box<dyn any::Any + Send + 'static>;
#[cfg(not(feature = "std"))]
pub type LowLevelThreadError = ();

pub trait AnyBoxConstructor {
    fn new(data: LowLevelThreadError) -> Self;
}

pub trait Joinable<T: Send + 'static, U: Send + 'static>: Sized {
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
    fn new(data: LowLevelThreadError) -> Self {
        BrotliEncoderThreadError::ThreadExecError(data)
    }
}

fn set_pending_error(
    pending_error: &mut Option<BrotliEncoderThreadError>,
    error: BrotliEncoderThreadError,
) {
    if pending_error.is_none() {
        *pending_error = Some(error);
    }
}

pub struct CompressedFileChunk<Alloc: BrotliAlloc + Send + 'static>
where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send,
{
    data_backing: <Alloc as Allocator<u8>>::AllocatedMemory,
    data_size: usize,
}
pub struct CompressionThreadResult<Alloc: BrotliAlloc + Send + 'static>
where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send,
{
    compressed: Result<CompressedFileChunk<Alloc>, BrotliEncoderThreadError>,
    alloc: Alloc,
}
pub enum InternalSendAlloc<
    ReturnVal: Send + 'static,
    ExtraInput: Send + 'static,
    Alloc: BrotliAlloc + Send + 'static,
    Join: Joinable<ReturnVal, BrotliEncoderThreadError>,
> where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send,
{
    A(Alloc, ExtraInput),
    Join(Join),
    SpawningOrJoining(PhantomData<ReturnVal>),
}
impl<
    ReturnVal: Send + 'static,
    ExtraInput: Send + 'static,
    Alloc: BrotliAlloc + Send + 'static,
    Join: Joinable<ReturnVal, BrotliEncoderThreadError>,
> InternalSendAlloc<ReturnVal, ExtraInput, Alloc, Join>
where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send,
{
    fn unwrap_input(&mut self) -> (&mut Alloc, &mut ExtraInput) {
        match *self {
            InternalSendAlloc::A(ref mut alloc, ref mut extra) => (alloc, extra),
            _ => panic!("Bad state for allocator"),
        }
    }
}

pub struct SendAlloc<
    ReturnValue: Send + 'static,
    ExtraInput: Send + 'static,
    Alloc: BrotliAlloc + Send + 'static,
    Join: Joinable<ReturnValue, BrotliEncoderThreadError>,
>(pub InternalSendAlloc<ReturnValue, ExtraInput, Alloc, Join>)
//FIXME pub
where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send;

impl<
    ReturnValue: Send + 'static,
    ExtraInput: Send + 'static,
    Alloc: BrotliAlloc + Send + 'static,
    Join: Joinable<ReturnValue, BrotliEncoderThreadError>,
> SendAlloc<ReturnValue, ExtraInput, Alloc, Join>
where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send,
{
    pub fn new(alloc: Alloc, extra_input: ExtraInput) -> Self {
        SendAlloc::<ReturnValue, ExtraInput, Alloc, Join>(InternalSendAlloc::A(alloc, extra_input))
    }
    pub fn unwrap_or(self, other: Alloc, other_extra: ExtraInput) -> (Alloc, ExtraInput) {
        match self.0 {
            InternalSendAlloc::A(alloc, extra_input) => (alloc, extra_input),
            InternalSendAlloc::SpawningOrJoining(_) | InternalSendAlloc::Join(_) => {
                (other, other_extra)
            }
        }
    }
    fn unwrap_view_mut(&mut self) -> (&mut Alloc, &mut ExtraInput) {
        match self.0 {
            InternalSendAlloc::A(ref mut alloc, ref mut extra_input) => (alloc, extra_input),
            InternalSendAlloc::SpawningOrJoining(_) | InternalSendAlloc::Join(_) => {
                panic!("Item permanently borrowed/leaked")
            }
        }
    }
    pub fn unwrap(self) -> (Alloc, ExtraInput) {
        match self.0 {
            InternalSendAlloc::A(alloc, extra_input) => (alloc, extra_input),
            InternalSendAlloc::SpawningOrJoining(_) | InternalSendAlloc::Join(_) => {
                panic!("Item permanently borrowed/leaked")
            }
        }
    }
    pub fn replace_with_default(&mut self) -> (Alloc, ExtraInput) {
        match mem::replace(
            &mut self.0,
            InternalSendAlloc::SpawningOrJoining(PhantomData),
        ) {
            InternalSendAlloc::A(alloc, extra_input) => (alloc, extra_input),
            InternalSendAlloc::SpawningOrJoining(_) | InternalSendAlloc::Join(_) => {
                panic!("Item permanently borrowed/leaked")
            }
        }
    }
}

pub enum InternalOwned<T> {
    // FIXME pub
    Item(T),
    Borrowed,
}

pub struct Owned<T>(pub InternalOwned<T>); // FIXME pub
impl<T> Owned<T> {
    pub fn new(data: T) -> Self {
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

pub trait OwnedRetriever<U: Send + 'static> {
    fn view<T, F: FnOnce(&U) -> T>(&self, f: F) -> Result<T, PoisonedThreadError>;
    fn unwrap(self) -> Result<U, PoisonedThreadError>;
}

#[cfg(feature = "std")]
impl<U: Send + 'static> OwnedRetriever<U> for std::sync::Arc<std::sync::RwLock<U>> {
    fn view<T, F: FnOnce(&U) -> T>(&self, f: F) -> Result<T, PoisonedThreadError> {
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

pub trait BatchSpawnable<
    ReturnValue: Send + 'static,
    ExtraInput: Send + 'static,
    Alloc: BrotliAlloc + Send + 'static,
    U: Send + 'static + Sync,
> where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send + 'static,
{
    type JoinHandle: Joinable<ReturnValue, BrotliEncoderThreadError>;
    type FinalJoinHandle: OwnedRetriever<U>;
    // this function takes in an input slice
    // a SendAlloc per thread and converts them all into JoinHandle
    // the input is borrowed until the joins complete
    // owned is set to borrowed
    // the final join handle is a r/w lock which will return the SliceW to the owner
    // the FinalJoinHandle is only to be called when each individual JoinHandle has been examined
    // the function is called with the thread_index, the num_threads, a reference to the slice under a read lock,
    // and an allocator from the alloc_per_thread
    fn make_spawner(&mut self, input: &mut Owned<U>) -> Self::FinalJoinHandle;
    fn spawn<F: Fn(ExtraInput, usize, usize, &U, Alloc) -> ReturnValue + Send + 'static + Copy>(
        &mut self,
        handle: &mut Self::FinalJoinHandle,
        alloc: &mut SendAlloc<ReturnValue, ExtraInput, Alloc, Self::JoinHandle>,
        index: usize,
        num_threads: usize,
        f: F,
    );
}

pub trait BatchSpawnableLite<
    ReturnValue: Send + 'static,
    ExtraInput: Send + 'static,
    Alloc: BrotliAlloc + Send + 'static,
    U: Send + 'static + Sync,
> where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send + 'static,
{
    type JoinHandle: Joinable<ReturnValue, BrotliEncoderThreadError>;
    type FinalJoinHandle: OwnedRetriever<U>;
    fn make_spawner(&mut self, input: &mut Owned<U>) -> Self::FinalJoinHandle;
    fn spawn(
        &mut self,
        handle: &mut Self::FinalJoinHandle,
        alloc_per_thread: &mut SendAlloc<ReturnValue, ExtraInput, Alloc, Self::JoinHandle>,
        index: usize,
        num_threads: usize,
        f: fn(ExtraInput, usize, usize, &U, Alloc) -> ReturnValue,
    );
}
/*
impl<ReturnValue:Send+'static,
     ExtraInput:Send+'static,
     Alloc:BrotliAlloc+Send+'static,
     U:Send+'static+Sync>
     BatchSpawnableLite<T, Alloc, U> for BatchSpawnable<T, Alloc, U> {
  type JoinHandle = <Self as BatchSpawnable<T, Alloc, U>>::JoinHandle;
  type FinalJoinHandle = <Self as BatchSpawnable<T, Alloc, U>>::FinalJoinHandle;
  fn batch_spawn(
    &mut self,
    input: &mut Owned<U>,
    alloc_per_thread:&mut [SendAlloc<ReturnValue, ExtraInput, Alloc, Self::JoinHandle>],
    f: fn(usize, usize, &U, Alloc) -> T,
  ) -> Self::FinalJoinHandle {
   <Self as BatchSpawnable<ReturnValue, ExtraInput,  Alloc, U>>::batch_spawn(self, input, alloc_per_thread, f)
  }
}*/

pub fn CompressMultiSlice<
    Alloc: BrotliAlloc + Send + 'static,
    Spawner: BatchSpawnableLite<
            CompressionThreadResult<Alloc>,
            UnionHasher<Alloc>,
            Alloc,
            (
                <Alloc as Allocator<u8>>::AllocatedMemory,
                BrotliEncoderParams,
            ),
        >,
>(
    params: &BrotliEncoderParams,
    input_slice: &[u8],
    output: &mut [u8],
    alloc_per_thread: &mut [SendAlloc<
        CompressionThreadResult<Alloc>,
        UnionHasher<Alloc>,
        Alloc,
        Spawner::JoinHandle,
    >],
    thread_spawner: &mut Spawner,
) -> Result<usize, BrotliEncoderThreadError>
where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send + Sync,
    <Alloc as Allocator<u16>>::AllocatedMemory: Send + Sync,
    <Alloc as Allocator<u32>>::AllocatedMemory: Send + Sync,
{
    let input = if let InternalSendAlloc::A(ref mut alloc, ref _extra) = alloc_per_thread[0].0 {
        let mut input = allocate::<u8, _>(alloc, input_slice.len());
        input.slice_mut().copy_from_slice(input_slice);
        input
    } else {
        alloc_default::<u8, Alloc>()
    };
    let mut owned_input = Owned::new(input);
    let ret = CompressMulti(
        params,
        &mut owned_input,
        output,
        alloc_per_thread,
        thread_spawner,
    );
    if let InternalSendAlloc::A(ref mut alloc, ref _extra) = alloc_per_thread[0].0 {
        <Alloc as Allocator<u8>>::free_cell(alloc, owned_input.unwrap());
    }
    ret
}

fn get_range(thread_index: usize, num_threads: usize, file_size: usize) -> Range<usize> {
    ((thread_index * file_size) / num_threads)..(((thread_index + 1) * file_size) / num_threads)
}

fn compress_part<Alloc: BrotliAlloc + Send + 'static, SliceW: SliceWrapper<u8>>(
    hasher: UnionHasher<Alloc>,
    thread_index: usize,
    num_threads: usize,
    input_and_params: &(SliceW, BrotliEncoderParams),
    alloc: Alloc,
) -> CompressionThreadResult<Alloc>
where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send + 'static,
{
    compress_part_slice(
        hasher,
        thread_index,
        num_threads,
        input_and_params.0.slice(),
        &input_and_params.1,
        alloc,
    )
}

fn compress_part_slice<Alloc: BrotliAlloc + Send + 'static>(
    hasher: UnionHasher<Alloc>,
    thread_index: usize,
    num_threads: usize,
    input: &[u8],
    params: &BrotliEncoderParams,
    mut alloc: Alloc,
) -> CompressionThreadResult<Alloc>
where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send + 'static,
{
    let mut range = get_range(thread_index, num_threads, input.len());
    let mut mem = allocate::<u8, _>(
        &mut alloc,
        BrotliEncoderMaxCompressedSize(range.end - range.start),
    );
    let mut state = BrotliEncoderStateStruct::new(alloc);
    state.params = params.clone();
    if thread_index != 0 {
        state.params.catable = true; // make sure we can concatenate this to the other work results
        state.params.magic_number = false; // no reason to pepper this around
    }
    state.params.appendable = true; // make sure we are at least appendable, so that future items can be catted in
    if thread_index != 0 {
        state.set_custom_dictionary_with_optional_precomputed_hasher(
            range.start,
            &input[..range.start],
            hasher,
            true,
        );
    }
    let mut out_offset = 0usize;
    let compression_result;
    let mut available_out = mem.len();
    loop {
        let mut next_in_offset = 0usize;
        let mut available_in = range.end - range.start;
        let result = state.compress_stream(
            BrotliEncoderOperation::BROTLI_OPERATION_FINISH,
            &mut available_in,
            &input[range.clone()],
            &mut next_in_offset,
            &mut available_out,
            mem.slice_mut(),
            &mut out_offset,
            &mut None,
            &mut |_a, _b, _c, _d| (),
        );
        let new_range = range.start + next_in_offset..range.end;
        range = new_range;
        if result {
            compression_result = Ok(out_offset);
            break;
        } else if available_out == 0 {
            compression_result = Err(BrotliEncoderThreadError::InsufficientOutputSpace); // mark no space??
            break;
        }
    }
    BrotliEncoderDestroyInstance(&mut state);
    match compression_result {
        Ok(size) => CompressionThreadResult::<Alloc> {
            compressed: Ok(CompressedFileChunk {
                data_backing: mem,
                data_size: size,
            }),
            alloc: state.m8,
        },
        Err(e) => {
            <Alloc as Allocator<u8>>::free_cell(&mut state.m8, mem);
            CompressionThreadResult::<Alloc> {
                compressed: Err(e),
                alloc: state.m8,
            }
        }
    }
}

pub fn CompressMulti<
    Alloc: BrotliAlloc + Send + 'static,
    SliceW: SliceWrapper<u8> + Send + 'static + Sync,
    Spawner: BatchSpawnableLite<
            CompressionThreadResult<Alloc>,
            UnionHasher<Alloc>,
            Alloc,
            (SliceW, BrotliEncoderParams),
        >,
>(
    params: &BrotliEncoderParams,
    owned_input: &mut Owned<SliceW>,
    output: &mut [u8],
    alloc_per_thread: &mut [SendAlloc<
        CompressionThreadResult<Alloc>,
        UnionHasher<Alloc>,
        Alloc,
        Spawner::JoinHandle,
    >],
    thread_spawner: &mut Spawner,
) -> Result<usize, BrotliEncoderThreadError>
where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send,
    <Alloc as Allocator<u16>>::AllocatedMemory: Send,
    <Alloc as Allocator<u32>>::AllocatedMemory: Send,
{
    let num_threads = alloc_per_thread.len();
    let actually_owned_mem = mem::replace(owned_input, Owned(InternalOwned::Borrowed));
    let mut owned_input_pair = Owned::new((actually_owned_mem.unwrap(), params.clone()));
    // start thread spawner
    let mut spawner_and_input = thread_spawner.make_spawner(&mut owned_input_pair);
    if num_threads > 1 {
        // spawn first thread without "custom dictionary" while we compute the custom dictionary for other work items
        thread_spawner.spawn(
            &mut spawner_and_input,
            &mut alloc_per_thread[0],
            0,
            num_threads,
            compress_part,
        );
    }
    // populate all hashers at once, cloning them one by one
    let mut compression_last_thread_result;
    if num_threads > 1 && params.favor_cpu_efficiency {
        let mut local_params = params.clone();
        SanitizeParams(&mut local_params);
        let mut hasher = UnionHasher::Uninit;
        hasher_setup(
            alloc_per_thread[num_threads - 1].0.unwrap_input().0,
            &mut hasher,
            &mut local_params,
            None, // No unwrappable custom dict used here.
            &[],
            0,
            0,
            false,
        );
        let mut setup_error = false;
        for thread_index in 1..num_threads {
            let res = spawner_and_input.view(|input_and_params: &(SliceW, BrotliEncoderParams)| {
                let range = get_range(thread_index - 1, num_threads, input_and_params.0.len());
                let overlap = hasher.StoreLookahead().wrapping_sub(1);
                if range.end - range.start > overlap {
                    hasher.BulkStoreRange(
                        input_and_params.0.slice(),
                        usize::MAX,
                        if range.start > overlap {
                            range.start - overlap
                        } else {
                            0
                        },
                        range.end - overlap,
                    );
                }
            });
            if let Err(_e) = res {
                setup_error = true;
                break;
            }
            if thread_index + 1 != num_threads {
                {
                    let (alloc, out_hasher) = alloc_per_thread[thread_index].unwrap_view_mut();
                    *out_hasher = hasher.clone_with_alloc(alloc);
                }
                thread_spawner.spawn(
                    &mut spawner_and_input,
                    &mut alloc_per_thread[thread_index],
                    thread_index,
                    num_threads,
                    compress_part,
                );
            }
        }
        if setup_error {
            let mut setup_result = Err(BrotliEncoderThreadError::OtherThreadPanic);
            for thread in alloc_per_thread.iter_mut() {
                match mem::replace(
                    &mut thread.0,
                    InternalSendAlloc::SpawningOrJoining(PhantomData),
                ) {
                    InternalSendAlloc::Join(join) => match join.join() {
                        Ok(mut thread_result) => {
                            if let Ok(compressed_out) = thread_result.compressed {
                                <Alloc as Allocator<u8>>::free_cell(
                                    &mut thread_result.alloc,
                                    compressed_out.data_backing,
                                );
                            }
                            thread.0 =
                                InternalSendAlloc::A(thread_result.alloc, UnionHasher::Uninit);
                        }
                        Err(join_error) => setup_result = Err(join_error),
                    },
                    other => thread.0 = other,
                }
            }
            if let Ok(retrieved_owned_input) = spawner_and_input.unwrap() {
                *owned_input = Owned::new(retrieved_owned_input.0);
            }
            return setup_result;
        }
        let (alloc, _extra) = alloc_per_thread[num_threads - 1].replace_with_default();
        compression_last_thread_result = spawner_and_input.view(move |input_and_params:&(SliceW, BrotliEncoderParams)| -> CompressionThreadResult<Alloc> {
        compress_part(hasher,
                      num_threads - 1,
                      num_threads,
                      input_and_params,
                      alloc,
        )
      });
    } else {
        if num_threads > 1 {
            for thread_index in 1..num_threads - 1 {
                thread_spawner.spawn(
                    &mut spawner_and_input,
                    &mut alloc_per_thread[thread_index],
                    thread_index,
                    num_threads,
                    compress_part,
                );
            }
        }
        let (alloc, _extra) = alloc_per_thread[num_threads - 1].replace_with_default();
        compression_last_thread_result = spawner_and_input.view(move |input_and_params:&(SliceW, BrotliEncoderParams)| -> CompressionThreadResult<Alloc> {
        compress_part(UnionHasher::Uninit,
                      num_threads - 1,
                      num_threads,
                      input_and_params,
                      alloc,
        )
      });
    }
    let mut compression_result = Ok(0usize);
    let mut pending_error = None;
    let mut out_file_size = 0usize;
    let mut bro_cat_li = BroCatli::new();
    for (index, thread) in alloc_per_thread.iter_mut().enumerate() {
        let cur_result = if index + 1 == num_threads {
            match mem::replace(&mut compression_last_thread_result, Err(())) {
                Ok(result) => Some(result),
                Err(_err) => {
                    set_pending_error(
                        &mut pending_error,
                        BrotliEncoderThreadError::OtherThreadPanic,
                    );
                    None
                }
            }
        } else {
            match mem::replace(
                &mut thread.0,
                InternalSendAlloc::SpawningOrJoining(PhantomData),
            ) {
                InternalSendAlloc::A(_, _) | InternalSendAlloc::SpawningOrJoining(_) => {
                    panic!("Thread not properly spawned")
                }
                InternalSendAlloc::Join(join) => match join.join() {
                    Ok(result) => Some(result),
                    Err(err) => {
                        set_pending_error(&mut pending_error, err);
                        None
                    }
                },
            }
        };
        if let Some(mut cur_result) = cur_result {
            match cur_result.compressed {
                Ok(compressed_out) => {
                    if pending_error.is_none() {
                        bro_cat_li.new_brotli_file();
                        let mut in_offset = 0usize;
                        let cat_result = bro_cat_li.stream(
                            &compressed_out.data_backing.slice()[..compressed_out.data_size],
                            &mut in_offset,
                            output,
                            &mut out_file_size,
                        );
                        match cat_result {
                            BroCatliResult::Success | BroCatliResult::NeedsMoreInput => {
                                compression_result = Ok(out_file_size);
                            }
                            BroCatliResult::NeedsMoreOutput => {
                                set_pending_error(
                                    &mut pending_error,
                                    BrotliEncoderThreadError::InsufficientOutputSpace,
                                );
                                // not enough space
                            }
                            err => {
                                set_pending_error(
                                    &mut pending_error,
                                    BrotliEncoderThreadError::ConcatenationError(err),
                                );
                                // misc error
                            }
                        }
                    }
                    <Alloc as Allocator<u8>>::free_cell(
                        &mut cur_result.alloc,
                        compressed_out.data_backing,
                    );
                }
                Err(e) => {
                    set_pending_error(&mut pending_error, e);
                }
            }
            thread.0 = InternalSendAlloc::A(cur_result.alloc, UnionHasher::Uninit);
        }
    }
    if let Some(error) = pending_error {
        compression_result = Err(error);
    }
    if compression_result.is_ok() {
        match bro_cat_li.finish(output, &mut out_file_size) {
            BroCatliResult::Success => compression_result = Ok(out_file_size),
            err => {
                compression_result = Err(BrotliEncoderThreadError::ConcatenationFinalizationError(
                    err,
                ))
            }
        }
    }
    match spawner_and_input.unwrap() {
        Ok(retrieved_owned_input) => {
            *owned_input = Owned::new(retrieved_owned_input.0); // return the input to its rightful owner before returning
        }
        _ => {
            if compression_result.is_ok() {
                compression_result = Err(BrotliEncoderThreadError::OtherThreadPanic);
            }
        }
    }
    compression_result
}

/// Handle to a live thread scope: what [`CompressMultiScoped`] uses to hand a
/// chunk of work to another thread.
///
/// `'env` is the lifetime of the data a task may borrow. An implementation must
/// guarantee that every task it accepts has run to completion (or been dropped)
/// by the time the [`ThreadScope::scope`] call that vended this spawner
/// returns; that is what makes borrowing `'env` data from the tasks sound.
#[cfg(feature = "std")]
pub trait ScopedSpawner<'env> {
    /// Starts `task` on some other thread. Must not block waiting for it.
    fn spawn<Task: FnOnce() + Send + 'env>(&self, task: Task);
}

/// The work [`ThreadScope::scope`] runs inside the scope it opens.
///
/// This is a trait rather than a closure because [`run`](ScopeBody::run) has to
/// be generic over the spawner: a scope implementation only learns its own
/// spawner type once it is inside e.g. `std::thread::scope`, which quantifies
/// over a lifetime that cannot be named from the outside.
///
/// `Self` and [`Output`](ScopeBody::Output) are `Send` so that implementations
/// of [`ThreadScope`] are free to use a scope API that runs the body somewhere
/// other than the calling thread, such as `rayon::scope`.
#[cfg(feature = "std")]
pub trait ScopeBody<'env>: Send {
    type Output: Send;
    /// Called exactly once, with a spawner for the freshly opened scope.
    fn run<Spawner: ScopedSpawner<'env>>(self, spawner: &Spawner) -> Self::Output;
}

/// Bridges [`CompressMultiScoped`] to a scoped-thread API such as
/// `std::thread::scope` or `rayon::scope`, so that neither has to be a
/// dependency of this crate.
///
/// [`StdThreadScope`] implements this over `std::thread::scope`. A rayon-backed
/// implementation is a handful of lines in the calling crate, and either of
/// rayon's two scope entry points will do — [`ScopeBody`]'s `Send` bounds are
/// there so that `rayon::scope`, which requires both its body and its return
/// value to be `Send`, is usable as well:
///
/// ```ignore
/// use simd_brotli::enc::threading::{ScopeBody, ScopedSpawner, ThreadScope};
///
/// struct RayonSpawner<'a, 'scope>(&'a rayon::Scope<'scope>);
///
/// impl<'a, 'scope, 'env: 'scope> ScopedSpawner<'env> for RayonSpawner<'a, 'scope> {
///     fn spawn<Task: FnOnce() + Send + 'env>(&self, task: Task) {
///         self.0.spawn(move |_| task());
///     }
/// }
///
/// /// Runs the body on the calling thread; chunks go to the pool.
/// pub struct RayonThreadScope;
///
/// impl ThreadScope for RayonThreadScope {
///     fn scope<'env, Body: ScopeBody<'env>>(&self, body: Body) -> Body::Output {
///         rayon::in_place_scope(|scope| body.run(&RayonSpawner(scope)))
///     }
/// }
///
/// /// Runs the body on the pool as well.
/// pub struct RayonPoolScope;
///
/// impl ThreadScope for RayonPoolScope {
///     fn scope<'env, Body: ScopeBody<'env>>(&self, body: Body) -> Body::Output {
///         rayon::scope(|scope| body.run(&RayonSpawner(scope)))
///     }
/// }
/// ```
///
/// Both produce identical output. Prefer `in_place_scope` when the calling
/// thread is yours to use: the body compresses the last chunk itself, so
/// running it in place keeps that work on this thread rather than handing it to
/// a pool worker while this thread blocks on it. Reach for `scope` when the
/// caller should not be doing encode work at all — inside an existing rayon
/// task, say, or when the calling thread has to stay responsive.
#[cfg(feature = "std")]
pub trait ThreadScope {
    /// Opens a scope, runs `body` in it, and returns `body`'s output only once
    /// every task `body` spawned has finished.
    fn scope<'env, Body: ScopeBody<'env>>(&self, body: Body) -> Body::Output;
}

/// [`ThreadScope`] backed by `std::thread::scope`: one OS thread per chunk, no
/// pool.
#[cfg(feature = "std")]
#[derive(Default, Copy, Clone)]
pub struct StdThreadScope;

#[cfg(feature = "std")]
struct StdScopeSpawner<'scope, 'env: 'scope>(&'scope std::thread::Scope<'scope, 'env>);

#[cfg(feature = "std")]
impl<'scope, 'env: 'scope> ScopedSpawner<'env> for StdScopeSpawner<'scope, 'env> {
    fn spawn<Task: FnOnce() + Send + 'env>(&self, task: Task) {
        // The handle is dropped, not joined: `std::thread::scope` joins every
        // outstanding thread when it returns.
        self.0.spawn(task);
    }
}

#[cfg(feature = "std")]
impl ThreadScope for StdThreadScope {
    fn scope<'env, Body: ScopeBody<'env>>(&self, body: Body) -> Body::Output {
        std::thread::scope(|scope| body.run(&StdScopeSpawner(scope)))
    }
}

/// Compresses `input` in `alloc_per_thread.len()` catable chunks inside a
/// caller-provided thread scope, concatenating them into `output` and returning
/// the compressed size.
///
/// This is [`CompressMulti`] without the ownership dance: because the scope
/// guarantees the workers finish before it returns, the input is borrowed
/// rather than moved behind an `Arc<RwLock<..>>`, so the workers read it with no
/// locking and no refcount traffic. Each worker writes its result straight into
/// its own slot instead of being joined one at a time, and — as in
/// [`CompressMulti`] — the last chunk is compressed on the calling thread while
/// the others run.
///
/// Every slot of `alloc_per_thread` must be `Some` on entry; on return each
/// slot holds its allocator back, except for a slot whose worker panicked.
///
/// ```
/// use simd_brotli::enc::BrotliEncoderParams;
/// use simd_brotli::enc::threading::{CompressMultiScoped, StdThreadScope};
/// use alloc_stdlib::StandardAlloc;
///
/// let input = vec![42u8; 1 << 16];
/// let mut output = vec![0u8; 1 << 16];
/// let mut alloc_per_thread = [Some(StandardAlloc::default()); 4];
/// let size = CompressMultiScoped(
///     &BrotliEncoderParams::default(),
///     &input[..],
///     &mut output[..],
///     &mut alloc_per_thread[..],
///     &StdThreadScope,
/// ).unwrap();
/// assert!(size < input.len());
/// ```
#[cfg(feature = "std")]
pub fn CompressMultiScoped<Alloc: BrotliAlloc + Send + 'static, Scope: ThreadScope>(
    params: &BrotliEncoderParams,
    input: &[u8],
    output: &mut [u8],
    alloc_per_thread: &mut [Option<Alloc>],
    thread_scope: &Scope,
) -> Result<usize, BrotliEncoderThreadError>
where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send,
    <Alloc as Allocator<u16>>::AllocatedMemory: Send,
    <Alloc as Allocator<u32>>::AllocatedMemory: Send,
{
    let num_threads = alloc_per_thread.len();
    assert!(
        num_threads != 0,
        "CompressMultiScoped needs at least one allocator"
    );
    let mut results = std::vec::Vec::<Option<CompressionThreadResult<Alloc>>>::new();
    results.resize_with(num_threads, || None);
    {
        let (last_alloc, head_allocs) = alloc_per_thread.split_last_mut().unwrap();
        let (last_result, head_results) = results.split_last_mut().unwrap();
        thread_scope.scope(CompressChunks {
            params,
            input,
            // `IterMut::next` hands out slots borrowed for the whole scope
            // rather than for the duration of one call, which is what lets each
            // spawned task keep its own slot.
            alloc_slots: head_allocs.iter_mut(),
            result_slots: head_results.iter_mut(),
            last_alloc,
            last_result,
        });
    }
    // Every worker has finished by now, so the results can be concatenated in
    // order without any further synchronization.
    let mut compression_result = Ok(0usize);
    let mut pending_error = None;
    let mut out_file_size = 0usize;
    let mut bro_cat_li = BroCatli::new();
    for (alloc_slot, result) in alloc_per_thread.iter_mut().zip(results) {
        let mut cur_result = match result {
            Some(cur_result) => cur_result,
            // The worker never ran to completion: its slot, and its allocator,
            // went down with it.
            None => {
                set_pending_error(
                    &mut pending_error,
                    BrotliEncoderThreadError::OtherThreadPanic,
                );
                continue;
            }
        };
        match cur_result.compressed {
            Ok(compressed_out) => {
                if pending_error.is_none() {
                    bro_cat_li.new_brotli_file();
                    let mut in_offset = 0usize;
                    let cat_result = bro_cat_li.stream(
                        &compressed_out.data_backing.slice()[..compressed_out.data_size],
                        &mut in_offset,
                        output,
                        &mut out_file_size,
                    );
                    match cat_result {
                        BroCatliResult::Success | BroCatliResult::NeedsMoreInput => {
                            compression_result = Ok(out_file_size);
                        }
                        BroCatliResult::NeedsMoreOutput => {
                            set_pending_error(
                                &mut pending_error,
                                BrotliEncoderThreadError::InsufficientOutputSpace,
                            );
                        }
                        err => {
                            set_pending_error(
                                &mut pending_error,
                                BrotliEncoderThreadError::ConcatenationError(err),
                            );
                        }
                    }
                }
                <Alloc as Allocator<u8>>::free_cell(
                    &mut cur_result.alloc,
                    compressed_out.data_backing,
                );
            }
            Err(e) => {
                set_pending_error(&mut pending_error, e);
            }
        }
        *alloc_slot = Some(cur_result.alloc);
    }
    if let Some(error) = pending_error {
        compression_result = Err(error);
    }
    if compression_result.is_ok() {
        match bro_cat_li.finish(output, &mut out_file_size) {
            BroCatliResult::Success => compression_result = Ok(out_file_size),
            err => {
                compression_result = Err(BrotliEncoderThreadError::ConcatenationFinalizationError(
                    err,
                ))
            }
        }
    }
    compression_result
}

/// The work [`CompressMultiScoped`] performs inside the thread scope: it spawns
/// every chunk but the last, then compresses the last one on the calling
/// thread.
///
/// `'env` names the borrows the workers hold: the input, the params, and one
/// disjoint result slot each.
#[cfg(feature = "std")]
struct CompressChunks<'env, Alloc: BrotliAlloc + Send + 'static>
where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send,
{
    params: &'env BrotliEncoderParams,
    input: &'env [u8],
    alloc_slots: core::slice::IterMut<'env, Option<Alloc>>,
    result_slots: core::slice::IterMut<'env, Option<CompressionThreadResult<Alloc>>>,
    last_alloc: &'env mut Option<Alloc>,
    last_result: &'env mut Option<CompressionThreadResult<Alloc>>,
}

#[cfg(feature = "std")]
impl<'env, Alloc: BrotliAlloc + Send + 'static> CompressChunks<'env, Alloc>
where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send,
    <Alloc as Allocator<u16>>::AllocatedMemory: Send,
    <Alloc as Allocator<u32>>::AllocatedMemory: Send,
{
    fn next_alloc(&mut self) -> Alloc {
        self.alloc_slots
            .next()
            .expect("one allocator per chunk")
            .take()
            .expect("allocator slot must be populated")
    }

    fn spawn_chunk<Spawner: ScopedSpawner<'env>>(
        &mut self,
        spawner: &Spawner,
        thread_index: usize,
        num_threads: usize,
        alloc: Alloc,
        hasher: UnionHasher<Alloc>,
    ) {
        let result_slot = self.result_slots.next().expect("one result slot per chunk");
        let (input, params) = (self.input, self.params);
        spawner.spawn(move || {
            *result_slot = Some(compress_part_slice(
                hasher,
                thread_index,
                num_threads,
                input,
                params,
                alloc,
            ));
        });
    }
}

#[cfg(feature = "std")]
impl<'env, Alloc: BrotliAlloc + Send + 'static> ScopeBody<'env> for CompressChunks<'env, Alloc>
where
    <Alloc as Allocator<u8>>::AllocatedMemory: Send,
    <Alloc as Allocator<u16>>::AllocatedMemory: Send,
    <Alloc as Allocator<u32>>::AllocatedMemory: Send,
{
    type Output = ();
    fn run<Spawner: ScopedSpawner<'env>>(mut self, spawner: &Spawner) {
        let num_threads = self.alloc_slots.len() + 1;
        if num_threads > 1 {
            // Spawn the first chunk before anything else: it needs no custom
            // dictionary, so it can run while this thread builds the hasher the
            // later chunks share.
            let alloc = self.next_alloc();
            self.spawn_chunk(spawner, 0, num_threads, alloc, UnionHasher::Uninit);
        }
        let mut last_hasher = UnionHasher::Uninit;
        if num_threads > 1 && self.params.favor_cpu_efficiency {
            let mut local_params = self.params.clone();
            SanitizeParams(&mut local_params);
            let mut hasher = UnionHasher::Uninit;
            hasher_setup(
                self.last_alloc
                    .as_mut()
                    .expect("allocator slot must be populated"),
                &mut hasher,
                &mut local_params,
                None, // No unwrappable custom dict used here.
                &[],
                0,
                0,
                false,
            );
            // Populate the hasher once, cloning it off for each chunk as it
            // becomes ready; the last, clone-free copy stays here for the chunk
            // this thread compresses itself.
            for thread_index in 1..num_threads {
                let range = get_range(thread_index - 1, num_threads, self.input.len());
                let overlap = hasher.StoreLookahead().wrapping_sub(1);
                if range.end - range.start > overlap {
                    hasher.BulkStoreRange(
                        self.input,
                        usize::MAX,
                        range.start.saturating_sub(overlap),
                        range.end - overlap,
                    );
                }
                if thread_index + 1 != num_threads {
                    let mut alloc = self.next_alloc();
                    let thread_hasher = hasher.clone_with_alloc(&mut alloc);
                    self.spawn_chunk(spawner, thread_index, num_threads, alloc, thread_hasher);
                }
            }
            last_hasher = hasher;
        } else {
            for thread_index in 1..num_threads - 1 {
                let alloc = self.next_alloc();
                self.spawn_chunk(
                    spawner,
                    thread_index,
                    num_threads,
                    alloc,
                    UnionHasher::Uninit,
                );
            }
        }
        *self.last_result = Some(compress_part_slice(
            last_hasher,
            num_threads - 1,
            num_threads,
            self.input,
            self.params,
            self.last_alloc
                .take()
                .expect("allocator slot must be populated"),
        ));
    }
}

mod test;
