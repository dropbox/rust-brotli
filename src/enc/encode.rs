use super::backward_references::{BrotliCreateBackwardReferences, Struct1, UnionHasher,
                                 BrotliEncoderParams, BrotliEncoderMode, BrotliHasherParams, H2Sub,
                                 H3Sub, H4Sub, H5Sub, H6Sub, H54Sub, AdvHasher, BasicHasher,
                                 AnyHasher, HowPrepared, StoreLookaheadThenStore};

use super::bit_cost::{BitsEntropy, ShannonEntropy};
use super::block_split::BlockSplit;
use super::brotli_bit_stream::{BrotliBuildAndStoreHuffmanTreeFast, BrotliStoreHuffmanTree,
                               BrotliStoreMetaBlock, BrotliStoreMetaBlockFast,
                               BrotliStoreMetaBlockTrivial, BrotliStoreUncompressedMetaBlock};
use super::command::{Command, GetLengthCode, CommandCopyLen, CommandRestoreDistanceCode, RecomputeDistancePrefixes};
use super::compress_fragment::BrotliCompressFragmentFast;
use super::compress_fragment_two_pass::{BrotliCompressFragmentTwoPass, BrotliWriteBits};
use super::entropy_encode::{BrotliConvertBitDepthsToSymbols, BrotliCreateHuffmanTree, HuffmanTree,
                            NewHuffmanTree};

use super::metablock::{BrotliBuildMetaBlock, BrotliBuildMetaBlockGreedy, BrotliOptimizeHistograms};
use super::static_dict::{BROTLI_UNALIGNED_LOAD32, BROTLI_UNALIGNED_LOAD64, BROTLI_UNALIGNED_STORE64,
                         FindMatchLengthWithLimit, BrotliGetDictionary};
use super::histogram::{ContextType};
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::utf8_util::BrotliIsMostlyUTF8;
use super::util::{brotli_min_size_t, Log2FloorNonZero};
use core;

//fn BrotliCreateHqZopfliBackwardReferences(m: &mut [MemoryManager],
//                                          dictionary: &[BrotliDictionary],
//                                          num_bytes: usize,
//                                          position: usize,
//                                          ringbuffer: &[u8],
//                                          ringbuffer_mask: usize,
//                                          params: &[BrotliEncoderParams],
//                                          hasher: &mut [u8],
//                                          dist_cache: &mut [i32],
//                                          last_insert_len: &mut [usize],
//                                          commands: &mut [Command],
//                                          num_commands: &mut [usize],
//                                          num_literals: &mut [usize]);
//fn BrotliCreateZopfliBackwardReferences(m: &mut [MemoryManager],
//                                       dictionary: &[BrotliDictionary],
//                                      num_bytes: usize,
//                                        position: usize,
//                                        ringbuffer: &[u8],
//                                        ringbuffer_mask: usize,
//                                        params: &[BrotliEncoderParams],
//                                        hasher: &mut [u8],
//                                        dist_cache: &mut [i32],
//                                        last_insert_len: &mut [usize],
//                                        commands: &mut [Command],
//                                        num_commands: &mut [usize],
//                                        num_literals: &mut [usize]);
//fn BrotliInitBlockSplit(xself: &mut BlockSplit);
//fn BrotliInitMemoryManager(m: &mut [MemoryManager],
//                           alloc_func: fn(&mut [::std::os::raw::c_void], usize)
//                                          -> *mut ::std::os::raw::c_void,
//                           free_func: fn(*mut ::std::os::raw::c_void,
//                                         *mut ::std::os::raw::c_void),
//                           opaque: *mut ::std::os::raw::c_void);
//fn BrotliInitZopfliNodes(array: &mut [ZopfliNode], length: usize);
//fn BrotliWipeOutMemoryManager(m: &mut [MemoryManager]);


static kBrotliMinWindowBits: i32 = 10i32;

static kBrotliMaxWindowBits: i32 = 24i32;

static kInvalidMatch: u32 = 0xfffffffu32;

static kCutoffTransformsCount: u32 = 10u32;

static kCutoffTransforms: usize = 0x71b520ausize << 32i32 | 0xda2d3200u32 as (usize);

static kHashMul32: u32 = 0x1e35a7bdu32;

static kHashMul64: usize = 0x1e35a7bdusize << 32i32 | 0x1e35a7bdusize;

static kHashMul64Long: usize = 0x1fe35a7bu32 as (usize) << 32i32 | 0xd3579bd3u32 as (usize);


static kCompressFragmentTwoPassBlockSize: usize = (1i32 << 17i32) as (usize);

static kMinUTF8Ratio: f64 = 0.75f64;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum BrotliEncoderParameter {
  BROTLI_PARAM_MODE = 0,
  BROTLI_PARAM_QUALITY = 1,
  BROTLI_PARAM_LGWIN = 2,
  BROTLI_PARAM_LGBLOCK = 3,
  BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING = 4,
  BROTLI_PARAM_SIZE_HINT = 5,
}

/*
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum BrotliEncoderMode {
  BROTLI_MODE_GENERIC = 0,
  BROTLI_MODE_TEXT = 1,
  BROTLI_MODE_FONT = 2,
}
*/




pub struct RingBuffer<AllocU8: alloc::Allocator<u8>> {
  pub size_: u32,
  pub mask_: u32,
  pub tail_size_: u32,
  pub total_size_: u32,
  pub cur_size_: u32,
  pub pos_: u32,
  pub data_: AllocU8::AllocatedMemory,
  pub buffer_index: usize,
}


#[derive(PartialEq, Eq, Copy, Clone)]
pub enum BrotliEncoderStreamState {
  BROTLI_STREAM_PROCESSING = 0,
  BROTLI_STREAM_FLUSH_REQUESTED = 1,
  BROTLI_STREAM_FINISHED = 2,
  BROTLI_STREAM_METADATA_HEAD = 3,
  BROTLI_STREAM_METADATA_BODY = 4,
}


pub struct BrotliEncoderStateStruct<AllocU8: alloc::Allocator<u8>,
                                    AllocU16: alloc::Allocator<u16>,
                                    AllocU32: alloc::Allocator<u32>,
                                    AllocI32: alloc::Allocator<i32>,
                                    AllocCommand: alloc::Allocator<Command>>
{
  pub params: BrotliEncoderParams,
  pub m8: AllocU8,
  pub m16: AllocU16,
  pub mi32: AllocI32,
  pub m32: AllocU32,
  pub mc: AllocCommand,
  pub hasher_: UnionHasher<AllocU16, AllocU32>,
  pub input_pos_: usize,
  pub ringbuffer_: RingBuffer<AllocU8>,
  pub cmd_alloc_size_: usize,
  pub commands_: AllocCommand::AllocatedMemory, // not sure about this one
  pub num_commands_: usize,
  pub num_literals_: usize,
  pub last_insert_len_: usize,
  pub last_flush_pos_: usize,
  pub last_processed_pos_: usize,
  pub dist_cache_: [i32; 16],
  pub saved_dist_cache_: [i32; 4],
  pub last_byte_: u8,
  pub last_byte_bits_: u8,
  pub prev_byte_: u8,
  pub prev_byte2_: u8,
  pub storage_size_: usize,
  pub storage_: AllocU8::AllocatedMemory,
  pub small_table_: [i32; 1024],
  pub large_table_: AllocI32::AllocatedMemory,
  pub large_table_size_: usize,
  pub cmd_depths_: [u8; 128],
  pub cmd_bits_: [u16; 128],
  pub cmd_code_: [u8; 512],
  pub cmd_code_numbits_: usize,
  pub command_buf_: AllocU32::AllocatedMemory,
  pub literal_buf_: AllocU8::AllocatedMemory,
  pub next_out_: AllocU8::AllocatedMemory, // not sure about this one: may be a pointer to l
  pub available_out_: usize,
  pub total_out_: usize,
  pub tiny_buf_: [u8; 16],
  pub remaining_metadata_bytes_: u32,
  pub stream_state_: BrotliEncoderStreamState,
  pub is_last_block_emitted_: i32,
  pub is_initialized_: i32,
}



pub fn BrotliEncoderSetParameter<AllocU8: alloc::Allocator<u8>,
                                 AllocU16: alloc::Allocator<u16>,
                                 AllocU32: alloc::Allocator<u32>,
                                 AllocI32: alloc::Allocator<i32>,
                                 AllocCommand: alloc::Allocator<Command>>
  (mut state: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
   mut p: BrotliEncoderParameter,
   mut value: u32)
   -> i32 {
  if (*state).is_initialized_ != 0 {
    return 0i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_MODE as (i32) {
    (*state).params.mode = match value {
      0 => BrotliEncoderMode::BROTLI_MODE_GENERIC,
      1 => BrotliEncoderMode::BROTLI_MODE_TEXT,
      2 => BrotliEncoderMode::BROTLI_MODE_FONT,
      _ => BrotliEncoderMode::BROTLI_MODE_GENERIC,
    };
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_QUALITY as (i32) {
    (*state).params.quality = value as (i32);
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_LGWIN as (i32) {
    (*state).params.lgwin = value as (i32);
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_LGBLOCK as (i32) {
    (*state).params.lgblock = value as (i32);
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING as (i32) {
    if value != 0u32 && (value != 1u32) {
      return 0i32;
    }
    (*state).params.disable_literal_context_modeling = if !!!(value == 0) { 1i32 } else { 0i32 };
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_SIZE_HINT as (i32) {
    (*state).params.size_hint = value as (usize);
    return 1i32;
  }
  0i32
}
fn BrotliEncoderInitParams() -> BrotliEncoderParams {
  return BrotliEncoderParams {
           mode: BrotliEncoderMode::BROTLI_MODE_GENERIC,
           quality: 9,
           lgwin: 22i32,
           lgblock: 0i32,
           size_hint: 0usize,
           disable_literal_context_modeling: 0i32,
           hasher: BrotliHasherParams {
             type_: 6,
             block_bits: 9 - 1,
             bucket_bits: 15,
             hash_len: 5,
             num_last_distances_to_check: 16,
           },
         };
}


fn RingBufferInit<AllocU8: alloc::Allocator<u8>>() -> RingBuffer<AllocU8> {
  return RingBuffer {
           size_: 0,
           mask_: 0, // 0xff??
           tail_size_: 0,
           total_size_: 0,

           cur_size_: 0,
           pos_: 0,
           data_: AllocU8::AllocatedMemory::default(),
           buffer_index: 0usize,
         };
}

pub fn BrotliEncoderCreateInstance<AllocU8: alloc::Allocator<u8>,
                                   AllocU16: alloc::Allocator<u16>,
                                   AllocU32: alloc::Allocator<u32>,
                                   AllocI32: alloc::Allocator<i32>,
                                   AllocCommand: alloc::Allocator<Command>>
  (m8: AllocU8,
   m16: AllocU16,
   mi32: AllocI32,
   m32: AllocU32,
   mc: AllocCommand)
   -> BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand> {
  let cache: [i32; 16] = [4, 11, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  BrotliEncoderStateStruct::<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand> {
    params: BrotliEncoderInitParams(),
    input_pos_: 0usize,
    num_commands_: 0usize,
    num_literals_: 0usize,
    last_insert_len_: 0usize,
    last_flush_pos_: 0usize,
    last_processed_pos_: 0usize,
    prev_byte_: 0i32 as (u8),
    prev_byte2_: 0i32 as (u8),
    storage_size_: 0usize,
    storage_: AllocU8::AllocatedMemory::default(),
    hasher_: UnionHasher::<AllocU16, AllocU32>::default(),
    large_table_: AllocI32::AllocatedMemory::default(),
    large_table_size_: 0usize,
    cmd_code_numbits_: 0usize,
    command_buf_: AllocU32::AllocatedMemory::default(),
    literal_buf_: AllocU8::AllocatedMemory::default(),
    next_out_: AllocU8::AllocatedMemory::default(), //FIXME this should be a pointer
    available_out_: 0usize,
    total_out_: 0usize,
    stream_state_: BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING,
    is_last_block_emitted_: 0i32,
    is_initialized_: 0i32,
    ringbuffer_: RingBufferInit(),
    commands_: AllocCommand::AllocatedMemory::default(),
    cmd_alloc_size_: 0usize,
    dist_cache_: cache,
    saved_dist_cache_: [cache[0], cache[1], cache[2], cache[3]],
    cmd_bits_: [0; 128],
    cmd_depths_: [0; 128],
    last_byte_: 0,
    last_byte_bits_: 0,
    cmd_code_: [0; 512],
    m8: m8,
    m16: m16,
    mi32:mi32,
    m32: m32,
    mc: mc,
    remaining_metadata_bytes_: 0,
    small_table_: [0; 1024],
    tiny_buf_: [0; 16],
  }
}

/* no new news in here: it's all in the former init function
pub fn BrotliEncoderCreateInstance(mut alloc_func: fn(&mut [::std::os::raw::c_void], usize)
                                                      -> *mut ::std::os::raw::c_void,
                                   mut free_func: fn(*mut ::std::os::raw::c_void,
                                                     *mut ::std::os::raw::c_void),
                                   mut opaque: *mut ::std::os::raw::c_void)
                                   -> *mut BrotliEncoderStateStruct {
  let mut state: *mut BrotliEncoderStateStruct = 0i32;
  if alloc_func == 0 && (free_func == 0) {
    state = malloc(::std::mem::size_of::<BrotliEncoderStateStruct>());
  } else if alloc_func != 0 && (free_func != 0) {
    state = alloc_func(opaque, ::std::mem::size_of::<BrotliEncoderStateStruct>());
  }
  if state == 0i32 {
    return 0i32;
  }
  BrotliInitMemoryManager(&mut (*state).memory_manager_, alloc_func, free_func, opaque);
  BrotliEncoderInitState(state);
  state
}*/

fn RingBufferFree<AllocU8: alloc::Allocator<u8>>(mut m: &mut AllocU8,
                                                 mut rb: &mut RingBuffer<AllocU8>) {
  m.free_cell(core::mem::replace(&mut rb.data_, AllocU8::AllocatedMemory::default()));
}

fn DestroyHasher<AllocU16:alloc::Allocator<u16>, AllocU32:alloc::Allocator<u32>>(
mut m16: &mut AllocU16, mut m32:&mut AllocU32, mut handle: &mut UnionHasher<AllocU16, AllocU32>){
  match handle {
    &mut UnionHasher::H5(ref mut hasher) => {
      m16.free_cell(core::mem::replace(&mut hasher.num, AllocU16::AllocatedMemory::default()));
      m32.free_cell(core::mem::replace(&mut hasher.buckets, AllocU32::AllocatedMemory::default()));
    }
    &mut UnionHasher::H6(ref mut hasher) => {
      m16.free_cell(core::mem::replace(&mut hasher.num, AllocU16::AllocatedMemory::default()));
      m32.free_cell(core::mem::replace(&mut hasher.buckets, AllocU32::AllocatedMemory::default()));
    }
    _ => {}
  }
  *handle = UnionHasher::<AllocU16, AllocU32>::default();
}


fn BrotliEncoderCleanupState<AllocU8: alloc::Allocator<u8>,
                             AllocU16: alloc::Allocator<u16>,
                             AllocU32: alloc::Allocator<u32>,
                             AllocI32: alloc::Allocator<i32>,
                             AllocCommand: alloc::Allocator<Command>>
  (mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) {
  {
    s.m8.free_cell(core::mem::replace(&mut (*s).storage_, AllocU8::AllocatedMemory::default()));
  }
  {
    s.mc.free_cell(core::mem::replace(&mut (*s).commands_,
                                      AllocCommand::AllocatedMemory::default()));
  }
  RingBufferFree(&mut s.m8, &mut (*s).ringbuffer_);
  DestroyHasher(&mut s.m16, &mut s.m32, &mut (*s).hasher_);
  {
    s.mi32.free_cell(core::mem::replace(&mut (*s).large_table_,
                                       AllocI32::AllocatedMemory::default()));
  }
  {
    s.m32.free_cell(core::mem::replace(&mut (*s).command_buf_,
                                       AllocU32::AllocatedMemory::default()));
  }
  {
    s.m8.free_cell(core::mem::replace(&mut (*s).literal_buf_, AllocU8::AllocatedMemory::default()));
  }
}

pub fn BrotliEncoderDestroyInstance<AllocU8: alloc::Allocator<u8>,
                                    AllocU16: alloc::Allocator<u16>,
                                    AllocU32: alloc::Allocator<u32>,
                                    AllocI32: alloc::Allocator<i32>,
                                    AllocCommand: alloc::Allocator<Command>>
  (mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) {
  BrotliEncoderCleanupState(s);
}

fn brotli_min_int(a: i32, b: i32) -> i32 {
  if a < b { a } else { b }
}

fn brotli_max_int(a: i32, b: i32) -> i32 {
  if a > b { a } else { b }
}

fn SanitizeParams(mut params: &mut BrotliEncoderParams) {
  (*params).quality = brotli_min_int(11i32, brotli_max_int(0i32, (*params).quality));
  if (*params).lgwin < 10i32 {
    (*params).lgwin = 10i32;
  } else if (*params).lgwin > 24i32 {
    (*params).lgwin = 24i32;
  }
}

fn ComputeLgBlock(mut params: &BrotliEncoderParams) -> i32 {
  let mut lgblock: i32 = (*params).lgblock;
  if (*params).quality == 0i32 || (*params).quality == 1i32 {
    lgblock = (*params).lgwin;
  } else if (*params).quality < 4i32 {
    lgblock = 14i32;
  } else if lgblock == 0i32 {
    lgblock = 16i32;
    if (*params).quality >= 9i32 && ((*params).lgwin > lgblock) {
      lgblock = brotli_min_int(18i32, (*params).lgwin);
    }
  } else {
    lgblock = brotli_min_int(24i32, brotli_max_int(16i32, lgblock));
  }
  lgblock
}

fn ComputeRbBits(mut params: &BrotliEncoderParams) -> i32 {
  1i32 + brotli_max_int((*params).lgwin, (*params).lgblock)
}

fn RingBufferSetup<AllocU8: alloc::Allocator<u8>>(mut params: &BrotliEncoderParams,
                                                  mut rb: &mut RingBuffer<AllocU8>) {
  let mut window_bits: i32 = ComputeRbBits(params);
  let mut tail_bits: i32 = (*params).lgblock;
  *(&mut (*rb).size_) = 1u32 << window_bits;
  *(&mut (*rb).mask_) = (1u32 << window_bits).wrapping_sub(1u32);
  *(&mut (*rb).tail_size_) = 1u32 << tail_bits;
  *(&mut (*rb).total_size_) = (*rb).size_.wrapping_add((*rb).tail_size_);
}

fn EncodeWindowBits(mut lgwin: i32, mut last_byte: &mut u8, mut last_byte_bits: &mut u8) {
  if lgwin == 16i32 {
    *last_byte = 0i32 as (u8);
    *last_byte_bits = 1i32 as (u8);
  } else if lgwin == 17i32 {
    *last_byte = 1i32 as (u8);
    *last_byte_bits = 7i32 as (u8);
  } else if lgwin > 17i32 {
    *last_byte = (lgwin - 17i32 << 1i32 | 1i32) as (u8);
    *last_byte_bits = 4i32 as (u8);
  } else {
    *last_byte = (lgwin - 8i32 << 4i32 | 1i32) as (u8);
    *last_byte_bits = 7i32 as (u8);
  }
}

fn InitCommandPrefixCodes(mut cmd_depths: &mut [u8],
                          mut cmd_bits: &mut [u16],
                          mut cmd_code: &mut [u8],
                          mut cmd_code_numbits: &mut usize) {
  static kDefaultCommandDepths: [u8; 128] = [0i32 as (u8),
                                             4i32 as (u8),
                                             4i32 as (u8),
                                             5i32 as (u8),
                                             6i32 as (u8),
                                             6i32 as (u8),
                                             7i32 as (u8),
                                             7i32 as (u8),
                                             7i32 as (u8),
                                             7i32 as (u8),
                                             7i32 as (u8),
                                             8i32 as (u8),
                                             8i32 as (u8),
                                             8i32 as (u8),
                                             8i32 as (u8),
                                             8i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             4i32 as (u8),
                                             4i32 as (u8),
                                             4i32 as (u8),
                                             4i32 as (u8),
                                             4i32 as (u8),
                                             5i32 as (u8),
                                             5i32 as (u8),
                                             6i32 as (u8),
                                             6i32 as (u8),
                                             6i32 as (u8),
                                             6i32 as (u8),
                                             7i32 as (u8),
                                             7i32 as (u8),
                                             7i32 as (u8),
                                             7i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             0i32 as (u8),
                                             4i32 as (u8),
                                             4i32 as (u8),
                                             5i32 as (u8),
                                             5i32 as (u8),
                                             5i32 as (u8),
                                             6i32 as (u8),
                                             6i32 as (u8),
                                             7i32 as (u8),
                                             8i32 as (u8),
                                             8i32 as (u8),
                                             9i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             10i32 as (u8),
                                             5i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             6i32 as (u8),
                                             6i32 as (u8),
                                             6i32 as (u8),
                                             6i32 as (u8),
                                             6i32 as (u8),
                                             6i32 as (u8),
                                             5i32 as (u8),
                                             5i32 as (u8),
                                             5i32 as (u8),
                                             5i32 as (u8),
                                             5i32 as (u8),
                                             5i32 as (u8),
                                             4i32 as (u8),
                                             4i32 as (u8),
                                             4i32 as (u8),
                                             4i32 as (u8),
                                             4i32 as (u8),
                                             4i32 as (u8),
                                             4i32 as (u8),
                                             5i32 as (u8),
                                             5i32 as (u8),
                                             5i32 as (u8),
                                             5i32 as (u8),
                                             5i32 as (u8),
                                             5i32 as (u8),
                                             6i32 as (u8),
                                             6i32 as (u8),
                                             7i32 as (u8),
                                             7i32 as (u8),
                                             7i32 as (u8),
                                             8i32 as (u8),
                                             10i32 as (u8),
                                             12i32 as (u8),
                                             12i32 as (u8),
                                             12i32 as (u8),
                                             12i32 as (u8),
                                             12i32 as (u8),
                                             12i32 as (u8),
                                             12i32 as (u8),
                                             12i32 as (u8),
                                             12i32 as (u8),
                                             12i32 as (u8),
                                             12i32 as (u8),
                                             12i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8),
                                             0i32 as (u8)];
  static kDefaultCommandBits: [u16; 128] = [0i32 as (u16),
                                            0i32 as (u16),
                                            8i32 as (u16),
                                            9i32 as (u16),
                                            3i32 as (u16),
                                            35i32 as (u16),
                                            7i32 as (u16),
                                            71i32 as (u16),
                                            39i32 as (u16),
                                            103i32 as (u16),
                                            23i32 as (u16),
                                            47i32 as (u16),
                                            175i32 as (u16),
                                            111i32 as (u16),
                                            239i32 as (u16),
                                            31i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            4i32 as (u16),
                                            12i32 as (u16),
                                            2i32 as (u16),
                                            10i32 as (u16),
                                            6i32 as (u16),
                                            13i32 as (u16),
                                            29i32 as (u16),
                                            11i32 as (u16),
                                            43i32 as (u16),
                                            27i32 as (u16),
                                            59i32 as (u16),
                                            87i32 as (u16),
                                            55i32 as (u16),
                                            15i32 as (u16),
                                            79i32 as (u16),
                                            319i32 as (u16),
                                            831i32 as (u16),
                                            191i32 as (u16),
                                            703i32 as (u16),
                                            447i32 as (u16),
                                            959i32 as (u16),
                                            0i32 as (u16),
                                            14i32 as (u16),
                                            1i32 as (u16),
                                            25i32 as (u16),
                                            5i32 as (u16),
                                            21i32 as (u16),
                                            19i32 as (u16),
                                            51i32 as (u16),
                                            119i32 as (u16),
                                            159i32 as (u16),
                                            95i32 as (u16),
                                            223i32 as (u16),
                                            479i32 as (u16),
                                            991i32 as (u16),
                                            63i32 as (u16),
                                            575i32 as (u16),
                                            127i32 as (u16),
                                            639i32 as (u16),
                                            383i32 as (u16),
                                            895i32 as (u16),
                                            255i32 as (u16),
                                            767i32 as (u16),
                                            511i32 as (u16),
                                            1023i32 as (u16),
                                            14i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            27i32 as (u16),
                                            59i32 as (u16),
                                            7i32 as (u16),
                                            39i32 as (u16),
                                            23i32 as (u16),
                                            55i32 as (u16),
                                            30i32 as (u16),
                                            1i32 as (u16),
                                            17i32 as (u16),
                                            9i32 as (u16),
                                            25i32 as (u16),
                                            5i32 as (u16),
                                            0i32 as (u16),
                                            8i32 as (u16),
                                            4i32 as (u16),
                                            12i32 as (u16),
                                            2i32 as (u16),
                                            10i32 as (u16),
                                            6i32 as (u16),
                                            21i32 as (u16),
                                            13i32 as (u16),
                                            29i32 as (u16),
                                            3i32 as (u16),
                                            19i32 as (u16),
                                            11i32 as (u16),
                                            15i32 as (u16),
                                            47i32 as (u16),
                                            31i32 as (u16),
                                            95i32 as (u16),
                                            63i32 as (u16),
                                            127i32 as (u16),
                                            255i32 as (u16),
                                            767i32 as (u16),
                                            2815i32 as (u16),
                                            1791i32 as (u16),
                                            3839i32 as (u16),
                                            511i32 as (u16),
                                            2559i32 as (u16),
                                            1535i32 as (u16),
                                            3583i32 as (u16),
                                            1023i32 as (u16),
                                            3071i32 as (u16),
                                            2047i32 as (u16),
                                            4095i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16),
                                            0i32 as (u16)];
  static kDefaultCommandCode: [u8; 57] = [0xffi32 as (u8),
                                          0x77i32 as (u8),
                                          0xd5i32 as (u8),
                                          0xbfi32 as (u8),
                                          0xe7i32 as (u8),
                                          0xdei32 as (u8),
                                          0xeai32 as (u8),
                                          0x9ei32 as (u8),
                                          0x51i32 as (u8),
                                          0x5di32 as (u8),
                                          0xdei32 as (u8),
                                          0xc6i32 as (u8),
                                          0x70i32 as (u8),
                                          0x57i32 as (u8),
                                          0xbci32 as (u8),
                                          0x58i32 as (u8),
                                          0x58i32 as (u8),
                                          0x58i32 as (u8),
                                          0xd8i32 as (u8),
                                          0xd8i32 as (u8),
                                          0x58i32 as (u8),
                                          0xd5i32 as (u8),
                                          0xcbi32 as (u8),
                                          0x8ci32 as (u8),
                                          0xeai32 as (u8),
                                          0xe0i32 as (u8),
                                          0xc3i32 as (u8),
                                          0x87i32 as (u8),
                                          0x1fi32 as (u8),
                                          0x83i32 as (u8),
                                          0xc1i32 as (u8),
                                          0x60i32 as (u8),
                                          0x1ci32 as (u8),
                                          0x67i32 as (u8),
                                          0xb2i32 as (u8),
                                          0xaai32 as (u8),
                                          0x6i32 as (u8),
                                          0x83i32 as (u8),
                                          0xc1i32 as (u8),
                                          0x60i32 as (u8),
                                          0x30i32 as (u8),
                                          0x18i32 as (u8),
                                          0xcci32 as (u8),
                                          0xa1i32 as (u8),
                                          0xcei32 as (u8),
                                          0x88i32 as (u8),
                                          0x54i32 as (u8),
                                          0x94i32 as (u8),
                                          0x46i32 as (u8),
                                          0xe1i32 as (u8),
                                          0xb0i32 as (u8),
                                          0xd0i32 as (u8),
                                          0x4ei32 as (u8),
                                          0xb2i32 as (u8),
                                          0xf7i32 as (u8),
                                          0x4i32 as (u8),
                                          0x0i32 as (u8)];
  static kDefaultCommandCodeNumBits: usize = 448usize;
  cmd_depths[..].clone_from_slice(&kDefaultCommandDepths[..]);
  cmd_bits[..].clone_from_slice(&kDefaultCommandBits[..]);
  cmd_code[..].clone_from_slice(&kDefaultCommandCode[..]);
  *cmd_code_numbits = kDefaultCommandCodeNumBits;
}

fn EnsureInitialized<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>
  (mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>)
   -> i32 {
  if (*s).is_initialized_ != 0 {
    return 1i32;
  }
  SanitizeParams(&mut (*s).params);
  (*s).params.lgblock = ComputeLgBlock(&mut (*s).params);
  (*s).remaining_metadata_bytes_ = !(0u32);
  RingBufferSetup(&mut (*s).params, &mut (*s).ringbuffer_);
  {
    let mut lgwin: i32 = (*s).params.lgwin;
    if (*s).params.quality == 0i32 || (*s).params.quality == 1i32 {
      lgwin = brotli_max_int(lgwin, 18i32);
    }
    EncodeWindowBits(lgwin, &mut (*s).last_byte_, &mut (*s).last_byte_bits_);
  }
  if (*s).params.quality == 0i32 {
    InitCommandPrefixCodes(&mut (*s).cmd_depths_[..],
                           &mut (*s).cmd_bits_[..],
                           &mut (*s).cmd_code_[..],
                           &mut (*s).cmd_code_numbits_);
  }
  (*s).is_initialized_ = 1i32;
  1i32
}

fn RingBufferInitBuffer<AllocU8: alloc::Allocator<u8>>(mut m: &mut AllocU8,
                                                       buflen: u32,
                                                       mut rb: &mut RingBuffer<AllocU8>) {
  static kSlackForEightByteHashingEverywhere: usize = 7usize;
  let mut new_data =
    m.alloc_cell(((2u32).wrapping_add(buflen) as (usize))
                   .wrapping_add(kSlackForEightByteHashingEverywhere));
  let mut i: usize;
  if !(*rb).data_.slice().len() != 0 {
    let lim: usize = ((2u32).wrapping_add((*rb).cur_size_) as (usize))
      .wrapping_add(kSlackForEightByteHashingEverywhere);
    new_data.slice_mut()[..lim].clone_from_slice(&(*rb).data_.slice()[..lim]);
    m.free_cell(core::mem::replace(&mut (*rb).data_, AllocU8::AllocatedMemory::default()));
  }
  core::mem::replace(&mut (*rb).data_, new_data);
  (*rb).cur_size_ = buflen;
  (*rb).buffer_index = 2usize;
  (*rb).data_.slice_mut()[((*rb).buffer_index.wrapping_sub(2usize))] = 0;
  (*rb).data_.slice_mut()[((*rb).buffer_index.wrapping_sub(1usize))] = 0;
  i = 0usize;
  while i < kSlackForEightByteHashingEverywhere {
    {
      (*rb).data_.slice_mut()[((*rb)
         .buffer_index
         .wrapping_add((*rb).cur_size_ as (usize))
         .wrapping_add(i) as (usize))] = 0;
    }
    i = i.wrapping_add(1 as (usize));
  }
}


fn RingBufferWriteTail<AllocU8: alloc::Allocator<u8>>(bytes: &[u8],
                                                      mut n: usize,
                                                      mut rb: &mut RingBuffer<AllocU8>) {
  let masked_pos: usize = ((*rb).pos_ & (*rb).mask_) as (usize);
  if masked_pos < (*rb).tail_size_ as (usize) {
    let p: usize = ((*rb).size_ as (usize)).wrapping_add(masked_pos);
    let begin = ((*rb).buffer_index.wrapping_add(p) as (usize));
    let lim = brotli_min_size_t(n, ((*rb).tail_size_ as (usize)).wrapping_sub(masked_pos));
    (*rb).data_.slice_mut()[begin..(begin + lim)].clone_from_slice(&bytes[..lim]);
  }
}

fn RingBufferWrite<AllocU8: alloc::Allocator<u8>>(mut m: &mut AllocU8,
                                                  mut bytes: &[u8],
                                                  mut n: usize,
                                                  mut rb: &mut RingBuffer<AllocU8>) {
  if (*rb).pos_ == 0u32 && (n < (*rb).tail_size_ as (usize)) {
    (*rb).pos_ = n as (u32);
    RingBufferInitBuffer(m, (*rb).pos_, rb);
    (*rb).data_.slice_mut()[((*rb).buffer_index as (usize))..(((*rb).buffer_index as (usize)) + n)]
      .clone_from_slice(&bytes[..n]);
    return;
  }
  if (*rb).cur_size_ < (*rb).total_size_ {
    RingBufferInitBuffer(m, (*rb).total_size_, rb);
    if !(0i32 == 0) {
      return;
    }
    (*rb).data_.slice_mut()[((*rb)
       .buffer_index
       .wrapping_add((*rb).size_ as (usize))
       .wrapping_sub(2usize) as (usize))] = 0i32 as (u8);
    (*rb).data_.slice_mut()[((*rb)
       .buffer_index
       .wrapping_add((*rb).size_ as (usize))
       .wrapping_sub(1usize) as (usize))] = 0i32 as (u8);
  }
  {
    let masked_pos: usize = ((*rb).pos_ & (*rb).mask_) as (usize);
    RingBufferWriteTail(bytes, n, rb);
    if masked_pos.wrapping_add(n) <= (*rb).size_ as (usize) {
      // a single write fits
      let start = ((*rb).buffer_index.wrapping_add(masked_pos) as (usize));
      (*rb).data_.slice_mut()[start..(start + n)].clone_from_slice(&bytes[..n]);
    } else {
      {
        let start = ((*rb).buffer_index.wrapping_add(masked_pos) as (usize));
        let mid = brotli_min_size_t(n, ((*rb).total_size_ as (usize)).wrapping_sub(masked_pos));
        (*rb).data_.slice_mut()[start..(start + mid)].clone_from_slice(&bytes[..mid]);
      }
      let xstart = ((*rb).buffer_index.wrapping_add(0usize) as (usize));
      let size = n.wrapping_sub(((*rb).size_ as (usize)).wrapping_sub(masked_pos));
      let bytes_start = (((*rb).size_ as (usize)).wrapping_sub(masked_pos) as (usize));
      (*rb).data_.slice_mut()[xstart..(xstart + size)].clone_from_slice(&bytes[bytes_start..
                                                                         (bytes_start +
                                                                          size)]);
    }
  }
  let data_2 = (*rb).data_.slice()[((*rb)
     .buffer_index
     .wrapping_add((*rb).size_ as (usize))
     .wrapping_sub(2usize) as (usize))];
  (*rb).data_.slice_mut()[((*rb).buffer_index.wrapping_sub(2usize) as (usize))] = data_2;
  let data_1 = (*rb).data_.slice()[((*rb)
     .buffer_index
     .wrapping_add((*rb).size_ as (usize))
     .wrapping_sub(1usize) as (usize))];
  (*rb).data_.slice_mut()[((*rb).buffer_index.wrapping_sub(1usize) as (usize))] = data_1;
  (*rb).pos_ = (*rb).pos_.wrapping_add(n as (u32));
  if (*rb).pos_ > 1u32 << 30i32 {
    (*rb).pos_ = (*rb).pos_ & (1u32 << 30i32).wrapping_sub(1u32) | 1u32 << 30i32;
  }
}

fn CopyInputToRingBuffer<AllocU8: alloc::Allocator<u8>,
                         AllocU16: alloc::Allocator<u16>,
                         AllocU32: alloc::Allocator<u32>,
                         AllocI32: alloc::Allocator<i32>,
                         AllocCommand: alloc::Allocator<Command>>
  (mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
   input_size: usize,
   mut input_buffer: &[u8]) {
  if EnsureInitialized(s) == 0 {
    return;
  }
  RingBufferWrite(&mut s.m8, input_buffer, input_size, &mut s.ringbuffer_);
  if !(0i32 == 0) {
    return;
  }
  (*s).input_pos_ = (*s).input_pos_.wrapping_add(input_size);
  if (s.ringbuffer_).pos_ <= (s.ringbuffer_).mask_ {
    let start = ((s.ringbuffer_).buffer_index.wrapping_add((s.ringbuffer_).pos_ as (usize)) as
                 (usize));
    for item in (s.ringbuffer_).data_.slice_mut()[start..(start + 7)].iter_mut() {
      *item = 0;
    }
  }
}


fn ChooseHasher(mut params: &mut BrotliEncoderParams) {
  let mut hparams = &mut params.hasher;
  if (*params).quality > 9i32 {
    (*hparams).type_ = 10i32;
  } else if (*params).quality == 4i32 && ((*params).size_hint >= (1i32 << 20i32) as (usize)) {
    (*hparams).type_ = 54i32;
  } else if (*params).quality < 5i32 {
    (*hparams).type_ = (*params).quality;
  } else if (*params).lgwin <= 16i32 {
    (*hparams).type_ = if (*params).quality < 7i32 {
      40i32
    } else if (*params).quality < 9i32 {
      41i32
    } else {
      42i32
    };
  } else if (*params).size_hint >= (1i32 << 20i32) as (usize) && ((*params).lgwin >= 19i32) {
    (*hparams).type_ = 6i32;
    (*hparams).block_bits = (*params).quality - 1i32;
    (*hparams).bucket_bits = 15i32;
    (*hparams).hash_len = 5i32;
    (*hparams).num_last_distances_to_check = if (*params).quality < 7i32 {
      4i32
    } else if (*params).quality < 9i32 {
      10i32
    } else {
      16i32
    };
  } else {
    (*hparams).type_ = 5i32;
    (*hparams).block_bits = (*params).quality - 1i32;
    (*hparams).bucket_bits = if (*params).quality < 7i32 {
      14i32
    } else {
      15i32
    };
    (*hparams).num_last_distances_to_check = if (*params).quality < 7i32 {
      4i32
    } else if (*params).quality < 9i32 {
      10i32
    } else {
      16i32
    };
  }
}

macro_rules! InitializeHX{
    ($name:ident,$subexpr:expr, $subtype:ty) => {
        fn $name(params : &BrotliEncoderParams) -> BasicHasher<$subtype> {
            BasicHasher {
                GetHasherCommon:Struct1{
                    params:params.hasher,
                    is_prepared_:0,
                    dict_num_lookups:0,
                    dict_num_matches:0,
                },
                buckets_:$subexpr,
            }
        }
    };
}
#[cfg(feature="unsafe")]
fn if_unsafe_bzero<T: core::convert::From<u8>>(t: &mut [T]) {
  for item in t.iter_mut() {
    *item = T::from(0u8);
  }
}

#[cfg(not(feature="unsafe"))]
fn if_unsafe_bzero<T>(t: &mut [T]) {}

InitializeHX!(InitializeH2, H2Sub { buckets_: [0; 65537] }, H2Sub);
InitializeHX!(InitializeH3, H3Sub { buckets_: [0; 65538] }, H3Sub);
InitializeHX!(InitializeH4, H4Sub { buckets_: [0; 131076] }, H4Sub);
InitializeHX!(InitializeH54, H54Sub { buckets_: [0; 1048580] }, H54Sub);
fn InitializeH5<AllocU16: alloc::Allocator<u16>, AllocU32: alloc::Allocator<u32>>
  (mut m16: &mut AllocU16,
   mut m32: &mut AllocU32,
   mut params: &BrotliEncoderParams)
   -> AdvHasher<H5Sub, AllocU16, AllocU32> {
  let block_size = 1u64 << params.hasher.block_bits;
  let bucket_size = 1u64 << params.hasher.bucket_bits;
  let mut buckets = m32.alloc_cell((bucket_size * block_size) as usize);
  let mut num = m16.alloc_cell(bucket_size as usize);
  if_unsafe_bzero(buckets.slice_mut());
  if_unsafe_bzero(num.slice_mut());
  AdvHasher {
    buckets: buckets,
    num: num,
    GetHasherCommon: Struct1 {
      params: params.hasher,
      is_prepared_: 0,
      dict_num_lookups: 0,
      dict_num_matches: 0,
    },
    specialization: H5Sub {},
    hash_shift_: 32i32 - params.hasher.bucket_bits,
    bucket_size_: bucket_size,
    block_size_: block_size,
    block_mask_: block_size.wrapping_sub(1u64) as (u32),
  }
}
fn InitializeH6<AllocU16: alloc::Allocator<u16>, AllocU32: alloc::Allocator<u32>>
  (mut m16: &mut AllocU16,
   mut m32: &mut AllocU32,
   mut params: &BrotliEncoderParams)
   -> AdvHasher<H6Sub, AllocU16, AllocU32> {
  let block_size = 1u64 << params.hasher.block_bits;
  let bucket_size = 1u64 << params.hasher.bucket_bits;
  let mut buckets = m32.alloc_cell((bucket_size * block_size) as usize);
  let mut num = m16.alloc_cell(bucket_size as usize);
  if_unsafe_bzero(buckets.slice_mut());
  if_unsafe_bzero(num.slice_mut());
  AdvHasher {
    buckets: buckets,
    num: num,
    GetHasherCommon: Struct1 {
      params: params.hasher,
      is_prepared_: 0,
      dict_num_lookups: 0,
      dict_num_matches: 0,
    },
    hash_shift_: 64i32 - params.hasher.bucket_bits,
    specialization: H6Sub {
      hash_mask: 0xffffffffffffffffu64 >> 64i32 - 8i32 * params.hasher.hash_len,
    },
    bucket_size_: 1u64 << params.hasher.bucket_bits,
    block_size_: block_size,
    block_mask_: block_size.wrapping_sub(1u64) as (u32),
  }
}

fn BrotliMakeHasher<AllocU16: alloc::Allocator<u16>, AllocU32: alloc::Allocator<u32>>
  (mut m16: &mut AllocU16,
   mut m32: &mut AllocU32,
   params: &BrotliEncoderParams)
   -> UnionHasher<AllocU16, AllocU32> {
  let hasher_type: i32 = params.hasher.type_;
  if hasher_type == 2i32 {
    return UnionHasher::H2(InitializeH2(params));
  }
  if hasher_type == 3i32 {
    return UnionHasher::H3(InitializeH3(params));
  }
  if hasher_type == 4i32 {
    return UnionHasher::H4(InitializeH4(params));
  }
  if hasher_type == 5i32 {
    return UnionHasher::H5(InitializeH5(m16, m32, params));
  }
  if hasher_type == 6i32 {
    return UnionHasher::H6(InitializeH6(m16, m32, params));
  }
  /*
    if hasher_type == 40i32 {
      return InitializeH40(params);
    }
    if hasher_type == 41i32 {
      return InitializeH41(params);
    }
    if hasher_type == 42i32 {
      return InitializeH42(params);
    }
*/
  if hasher_type == 54i32 {
    return UnionHasher::H54(InitializeH54(params));
  }
  /*
    if hasher_type == 10i32 {
      return InitializeH10(params);
    }*/
  return UnionHasher::Uninit;
}
fn HasherReset<AllocU16:alloc::Allocator<u16>,
AllocU32:alloc::Allocator<u32>>(mut t: &mut UnionHasher<AllocU16, AllocU32>){
  match t {
    &mut UnionHasher::Uninit => {}
    _ => (t.GetHasherCommon()).is_prepared_ = 0i32,
  };
}
fn GetHasherCommon<AllocU16: alloc::Allocator<u16>, AllocU32: alloc::Allocator<u32>>
  (mut t: &mut UnionHasher<AllocU16, AllocU32>)
   -> &mut Struct1 {
  t.GetHasherCommon()
}

fn HasherSetup<AllocU16:alloc::Allocator<u16>,
               AllocU32:alloc::Allocator<u32>>(mut m16: &mut AllocU16,
               mut m32: &mut AllocU32,
               mut handle: &mut UnionHasher<AllocU16, AllocU32>,
               mut params: &mut BrotliEncoderParams,
               mut data: &[u8],
               mut position: usize,
               mut input_size: usize,
mut is_last: i32){
  let mut one_shot: i32 = (position == 0usize && (is_last != 0)) as (i32);
  let is_uninit = match (handle) {
    &mut UnionHasher::Uninit => true,
    _ => false,
  };
  if is_uninit {
    let mut alloc_size: usize;
    ChooseHasher(&mut (*params));
    //alloc_size = HasherSize(params, one_shot, input_size);
    //xself = BrotliAllocate(m, alloc_size.wrapping_mul(::std::mem::size_of::<u8>()))
    *handle = BrotliMakeHasher(m16, m32, params);
    handle.GetHasherCommon().params = (*params).hasher;
    HasherReset(handle); // this sets everything to zero, unlike in C
  } else {
    match handle.Prepare(one_shot != 0, input_size, data) {
      HowPrepared::ALREADY_PREPARED => {}
      HowPrepared::NEWLY_PREPARED => {
        if position == 0usize {
          let mut common = handle.GetHasherCommon();
          (*common).dict_num_lookups = 0usize;
          (*common).dict_num_matches = 0usize;
        }
      }
    }
  }
}

fn HasherPrependCustomDictionary<AllocU16: alloc::Allocator<u16>, AllocU32: alloc::Allocator<u32>>
  (mut m16: &mut AllocU16,
   mut m32: &mut AllocU32,
   mut handle: &mut UnionHasher<AllocU16, AllocU32>,
   mut params: &mut BrotliEncoderParams,
   size: usize,
   mut dict: &[u8]) {
  let mut overlap: usize;
  let mut i: usize;
  HasherSetup(m16, m32, handle, params, dict, 0usize, size, 0i32);
  match handle {
    &mut UnionHasher::H2(ref mut hasher) => StoreLookaheadThenStore(hasher, size, dict),
    &mut UnionHasher::H3(ref mut hasher) => StoreLookaheadThenStore(hasher, size, dict),
    &mut UnionHasher::H4(ref mut hasher) => StoreLookaheadThenStore(hasher, size, dict),
    &mut UnionHasher::H5(ref mut hasher) => StoreLookaheadThenStore(hasher, size, dict),
    &mut UnionHasher::H6(ref mut hasher) => StoreLookaheadThenStore(hasher, size, dict),
    &mut UnionHasher::H54(ref mut hasher) => StoreLookaheadThenStore(hasher, size, dict),
    &mut UnionHasher::Uninit => panic!("Uninitialized"),
  }
}

pub fn BrotliEncoderSetCustomDictionary<AllocU8: alloc::Allocator<u8>,
                                        AllocU16: alloc::Allocator<u16>,
                                        AllocU32: alloc::Allocator<u32>,
                                        AllocI32: alloc::Allocator<i32>,
                                        AllocCommand: alloc::Allocator<Command>>
  (mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
   mut size: usize,
   mut dict: &[u8]) {
  let mut max_dict_size: usize = (1usize << (*s).params.lgwin).wrapping_sub(16usize);
  let mut dict_size: usize = size;
  if EnsureInitialized(s) == 0 {
    return;
  }
  if dict_size == 0usize || (*s).params.quality == 0i32 || (*s).params.quality == 1i32 {
    return;
  }
  if size > max_dict_size {
    dict = &dict[(size.wrapping_sub(max_dict_size) as (usize))..];
    dict_size = max_dict_size;
  }
  CopyInputToRingBuffer(s, dict_size, dict);
  (*s).last_flush_pos_ = dict_size;
  (*s).last_processed_pos_ = dict_size;
  if dict_size > 0usize {
    (*s).prev_byte_ = dict[(dict_size.wrapping_sub(1usize) as (usize))];
  }
  if dict_size > 1usize {
    (*s).prev_byte2_ = dict[(dict_size.wrapping_sub(2usize) as (usize))];
  }
  let mut m16 = &mut s.m16;
  let mut m32 = &mut s.m32;
  HasherPrependCustomDictionary(m16,
                                m32,
                                &mut (*s).hasher_,
                                &mut (*s).params,
                                dict_size,
                                dict);
}


pub fn BrotliEncoderMaxCompressedSize(mut input_size: usize) -> usize {
  let mut num_large_blocks: usize = input_size >> 24i32;
  let mut tail: usize = input_size.wrapping_sub(num_large_blocks << 24i32);
  let mut tail_overhead: usize = (if tail > (1i32 << 20i32) as (usize) {
                                    4i32
                                  } else {
                                    3i32
                                  }) as (usize);
  let mut overhead: usize = (2usize)
    .wrapping_add((4usize).wrapping_mul(num_large_blocks))
    .wrapping_add(tail_overhead)
    .wrapping_add(1usize);
  let mut result: usize = input_size.wrapping_add(overhead);
  if input_size == 0usize {
    return 1usize;
  }
  if result < input_size { 0usize } else { result }
}

fn InitOrStitchToPreviousBlock<AllocU16: alloc::Allocator<u16>, AllocU32: alloc::Allocator<u32>>
  (mut m16: &mut AllocU16,
   mut m32: &mut AllocU32,
   mut handle: &mut UnionHasher<AllocU16, AllocU32>,
   data: &[u8],
   mask: usize,
   params: &mut BrotliEncoderParams,
   mut position: usize,
   mut input_size: usize,
   mut is_last: i32) {
  HasherSetup(m16,
              m32,
              handle,
              params,
              data,
              position,
              input_size,
              is_last);
  handle.StitchToPreviousBlock(input_size, position, data, mask);
}


pub struct Struct49 {
  pub cost: f32,
  pub next: u32,
  pub shortcut: u32,
}



pub struct ZopfliNode {
  pub length: u32,
  pub distance: u32,
  pub insert_length: u32,
  pub u: Struct49,
}


fn InitInsertCommand(mut xself: &mut Command, mut insertlen: usize) {
  (*xself).insert_len_ = insertlen as (u32);
  (*xself).copy_len_ = (4i32 << 24i32) as (u32);
  (*xself).dist_extra_ = 0u32;
  (*xself).dist_prefix_ = 16i32 as (u16);
  GetLengthCode(insertlen, 4usize, 0i32, &mut (*xself).cmd_prefix_);
}

fn ShouldCompress(mut data: &[u8],
                  mask: usize,
                  last_flush_pos: usize,
                  bytes: usize,
                  num_literals: usize,
                  num_commands: usize)
                  -> i32 {
  if num_commands < (bytes >> 8i32).wrapping_add(2usize) {
    if num_literals as (f64) > 0.99f64 * bytes as (f64) {
      let mut literal_histo: [u32; 256] =
        [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
         0u32];
      static kSampleRate: u32 = 13u32;
      static kMinEntropy: f64 = 7.92f64;
      let bit_cost_threshold: f64 = bytes as (f64) * kMinEntropy / kSampleRate as (f64);
      let mut t: usize = bytes.wrapping_add(kSampleRate as (usize))
        .wrapping_sub(1usize)
        .wrapping_div(kSampleRate as (usize));
      let mut pos: u32 = last_flush_pos as (u32);
      let mut i: usize;
      i = 0usize;
      while i < t {
        {
          {
            let _rhs = 1;
            let _lhs = &mut literal_histo[data[((pos as (usize) & mask) as (usize))] as (usize)];
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
          }
          pos = pos.wrapping_add(kSampleRate);
        }
        i = i.wrapping_add(1 as (usize));
      }
      if BitsEntropy(&literal_histo[..], 256usize) > bit_cost_threshold {
        return 0i32;
      }
    }
  }
  1i32
}
/*
fn BrotliCompressBufferQuality10(mut lgwin: i32,
                                 mut input_size: usize,
                                 mut input_buffer: &[u8],
                                 mut encoded_size: &mut [usize],
                                 mut encoded_buffer: &mut [u8])
                                 -> i32 {
  let mut memory_manager: MemoryManager;
  let mut m: *mut MemoryManager = &mut memory_manager;
  let mask: usize = !(0usize) >> 1i32;
  let max_backward_limit: usize = (1usize << lgwin).wrapping_sub(16usize);
  let mut dist_cache: [i32; 4] = [4i32, 11i32, 15i32, 16i32];
  let mut saved_dist_cache: [i32; 4] = [4i32, 11i32, 15i32, 16i32];
  let mut ok: i32 = 1i32;
  let max_out_size: usize = *encoded_size;
  let mut total_out_size: usize = 0usize;
  let mut last_byte: u8;
  let mut last_byte_bits: u8;
  let mut hasher: *mut u8 = 0i32;
  let hasher_eff_size: usize = brotli_min_size_t(input_size,
                                                 max_backward_limit.wrapping_add(16usize));
  let mut params: BrotliEncoderParams;
  let mut dictionary: *const BrotliDictionary = BrotliGetDictionary();
  let lgmetablock: i32 = brotli_min_int(24i32, lgwin + 1i32);
  let mut max_block_size: usize;
  let max_metablock_size: usize = 1usize << lgmetablock;
  let max_literals_per_metablock: usize = max_metablock_size.wrapping_div(8usize);
  let max_commands_per_metablock: usize = max_metablock_size.wrapping_div(8usize);
  let mut metablock_start: usize = 0usize;
  let mut prev_byte: u8 = 0i32 as (u8);
  let mut prev_byte2: u8 = 0i32 as (u8);
  BrotliEncoderInitParams(&mut params);
  params.quality = 10i32;
  params.lgwin = lgwin;
  SanitizeParams(&mut params);
  params.lgblock = ComputeLgBlock(&mut params);
  max_block_size = 1usize << params.lgblock;
  BrotliInitMemoryManager(m,
                          0i32 as
                          (fn(*mut ::std::os::raw::c_void, usize) -> *mut ::std::os::raw::c_void),
                          0i32 as (fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void)),
                          0i32);
  0i32;
  EncodeWindowBits(lgwin, &mut last_byte, &mut last_byte_bits);
  InitOrStitchToPreviousBlock(m,
                              &mut hasher,
                              input_buffer,
                              mask,
                              &mut params,
                              0usize,
                              hasher_eff_size,
                              1i32);
  if !(0i32 == 0) {
    BrotliWipeOutMemoryManager(m);
    return 0i32;
  }
  while ok != 0 && (metablock_start < input_size) {
    let metablock_end: usize = brotli_min_size_t(input_size,
                                                 metablock_start.wrapping_add(max_metablock_size));
    let expected_num_commands: usize =
      metablock_end.wrapping_sub(metablock_start).wrapping_div(12usize).wrapping_add(16usize);
    let mut commands: *mut Command = 0i32;
    let mut num_commands: usize = 0usize;
    let mut last_insert_len: usize = 0usize;
    let mut num_literals: usize = 0usize;
    let mut metablock_size: usize = 0usize;
    let mut cmd_alloc_size: usize = 0usize;
    let mut is_last: i32;
    let mut storage: *mut u8;
    let mut storage_ix: usize;
    let mut block_start: usize;
    block_start = metablock_start;
    while block_start < metablock_end {
      let mut block_size: usize = brotli_min_size_t(metablock_end.wrapping_sub(block_start),
                                                    max_block_size);
      let mut nodes: *mut ZopfliNode = if block_size.wrapping_add(1usize) != 0 {
        BrotliAllocate(m,
                       block_size.wrapping_add(1usize)
                         .wrapping_mul(::std::mem::size_of::<ZopfliNode>()))
      } else {
        0i32
      };
      let mut path_size: usize;
      let mut new_cmd_alloc_size: usize;
      if !(0i32 == 0) {
        BrotliWipeOutMemoryManager(m);
        return 0i32;
      }
      BrotliInitZopfliNodes(nodes, block_size.wrapping_add(1usize));
      StitchToPreviousBlockH10(hasher, block_size, block_start, input_buffer, mask);
      path_size = BrotliZopfliComputeShortestPath(m,
                                                  dictionary,
                                                  block_size,
                                                  block_start,
                                                  input_buffer,
                                                  mask,
                                                  &mut params,
                                                  max_backward_limit,
                                                  dist_cache.as_mut_ptr(),
                                                  hasher,
                                                  nodes);
      if !(0i32 == 0) {
        BrotliWipeOutMemoryManager(m);
        return 0i32;
      }
      new_cmd_alloc_size = brotli_max_size_t(expected_num_commands,
                                             num_commands.wrapping_add(path_size)
                                               .wrapping_add(1usize));
      if cmd_alloc_size != new_cmd_alloc_size {
        let mut new_commands: *mut Command = if new_cmd_alloc_size != 0 {
          BrotliAllocate(m,
                         new_cmd_alloc_size.wrapping_mul(::std::mem::size_of::<Command>()))
        } else {
          0i32
        };
        if !(0i32 == 0) {
          BrotliWipeOutMemoryManager(m);
          return 0i32;
        }
        cmd_alloc_size = new_cmd_alloc_size;
        if !commands.is_null() {
          memcpy(new_commands,
                 commands,
                 ::std::mem::size_of::<Command>().wrapping_mul(num_commands));
          {
            BrotliFree(m, commands);
            commands = 0i32;
          }
        }
        commands = new_commands;
      }
      BrotliZopfliCreateCommands(block_size,
                                 block_start,
                                 max_backward_limit,
                                 &mut nodes[(0usize)],
                                 dist_cache.as_mut_ptr(),
                                 &mut last_insert_len,
                                 &mut commands[(num_commands as (usize))],
                                 &mut num_literals);
      num_commands = num_commands.wrapping_add(path_size);
      block_start = block_start.wrapping_add(block_size);
      metablock_size = metablock_size.wrapping_add(block_size);
      {
        BrotliFree(m, nodes);
        nodes = 0i32;
      }
      if num_literals > max_literals_per_metablock || num_commands > max_commands_per_metablock {
        {
          break;
        }
      }
    }
    if last_insert_len > 0usize {
      InitInsertCommand(&mut commands[({
                                let _old = num_commands;
                                num_commands = num_commands.wrapping_add(1 as (usize));
                                _old
                              } as (usize))],
                        last_insert_len);
      num_literals = num_literals.wrapping_add(last_insert_len);
    }
    is_last = if !!(metablock_start.wrapping_add(metablock_size) == input_size) {
      1i32
    } else {
      0i32
    };
    storage = 0i32;
    storage_ix = last_byte_bits as (usize);
    if metablock_size == 0usize {
      storage = if 16i32 != 0 {
        BrotliAllocate(m, (16usize).wrapping_mul(::std::mem::size_of::<u8>()))
      } else {
        0i32
      };
      if !(0i32 == 0) {
        BrotliWipeOutMemoryManager(m);
        return 0i32;
      }
      storage[(0usize)] = last_byte;
      BrotliWriteBits(2usize, 3usize, &mut storage_ix, storage);
      storage_ix = storage_ix.wrapping_add(7u32 as (usize)) & !7u32 as (usize);
    } else if ShouldCompress(input_buffer,
                             mask,
                             metablock_start,
                             metablock_size,
                             num_literals,
                             num_commands) == 0 {
      memcpy(dist_cache.as_mut_ptr(),
             saved_dist_cache.as_mut_ptr(),
             (4usize).wrapping_mul(::std::mem::size_of::<i32>()));
      storage = if metablock_size.wrapping_add(16usize) != 0 {
        BrotliAllocate(m,
                       metablock_size.wrapping_add(16usize)
                         .wrapping_mul(::std::mem::size_of::<u8>()))
      } else {
        0i32
      };
      if !(0i32 == 0) {
        BrotliWipeOutMemoryManager(m);
        return 0i32;
      }
      storage[(0usize)] = last_byte;
      BrotliStoreUncompressedMetaBlock(is_last,
                                       input_buffer,
                                       metablock_start,
                                       mask,
                                       metablock_size,
                                       &mut storage_ix,
                                       storage);
    } else {
      let mut num_direct_distance_codes: u32 = 0u32;
      let mut distance_postfix_bits: u32 = 0u32;
      let mut literal_context_mode: ContextType = ContextType::CONTEXT_UTF8;
      let mut mb: MetaBlockSplit;
      InitMetaBlockSplit(&mut mb);
      if BrotliIsMostlyUTF8(input_buffer,
                            metablock_start,
                            mask,
                            metablock_size,
                            kMinUTF8Ratio) == 0 {
        literal_context_mode = ContextType::CONTEXT_SIGNED;
      }
      BrotliBuildMetaBlock(m,
                           input_buffer,
                           metablock_start,
                           mask,
                           &mut params,
                           prev_byte,
                           prev_byte2,
                           commands,
                           num_commands,
                           literal_context_mode,
                           &mut mb);
      if !(0i32 == 0) {
        BrotliWipeOutMemoryManager(m);
        return 0i32;
      }
      BrotliOptimizeHistograms(num_direct_distance_codes as (usize),
                               distance_postfix_bits as (usize),
                               &mut mb);
      storage = if (2usize).wrapping_mul(metablock_size).wrapping_add(502usize) != 0 {
        BrotliAllocate(m,
                       (2usize)
                         .wrapping_mul(metablock_size)
                         .wrapping_add(502usize)
                         .wrapping_mul(::std::mem::size_of::<u8>()))
      } else {
        0i32
      };
      if !(0i32 == 0) {
        BrotliWipeOutMemoryManager(m);
        return 0i32;
      }
      storage[(0usize)] = last_byte;
      BrotliStoreMetaBlock(m,
                           input_buffer,
                           metablock_start,
                           metablock_size,
                           mask,
                           prev_byte,
                           prev_byte2,
                           is_last,
                           num_direct_distance_codes,
                           distance_postfix_bits,
                           literal_context_mode,
                           commands,
                           num_commands,
                           &mut mb,
                           &mut storage_ix,
                           storage);
      if !(0i32 == 0) {
        BrotliWipeOutMemoryManager(m);
        return 0i32;
      }
      if metablock_size.wrapping_add(4usize) < storage_ix >> 3i32 {
        memcpy(dist_cache.as_mut_ptr(),
               saved_dist_cache.as_mut_ptr(),
               (4usize).wrapping_mul(::std::mem::size_of::<i32>()));
        storage[(0usize)] = last_byte;
        storage_ix = last_byte_bits as (usize);
        BrotliStoreUncompressedMetaBlock(is_last,
                                         input_buffer,
                                         metablock_start,
                                         mask,
                                         metablock_size,
                                         &mut storage_ix,
                                         storage);
      }
      DestroyMetaBlockSplit(m, &mut mb);
    }
    last_byte = storage[((storage_ix >> 3i32) as (usize))];
    last_byte_bits = (storage_ix & 7u32 as (usize)) as (u8);
    metablock_start = metablock_start.wrapping_add(metablock_size);
    prev_byte = input_buffer[(metablock_start.wrapping_sub(1usize) as (usize))];
    prev_byte2 = input_buffer[(metablock_start.wrapping_sub(2usize) as (usize))];
    memcpy(saved_dist_cache.as_mut_ptr(),
           dist_cache.as_mut_ptr(),
           (4usize).wrapping_mul(::std::mem::size_of::<i32>()));
    {
      let out_size: usize = storage_ix >> 3i32;
      total_out_size = total_out_size.wrapping_add(out_size);
      if total_out_size <= max_out_size {
        memcpy(encoded_buffer, storage, out_size);
        encoded_buffer = encoded_buffer[(out_size as (usize))..];
      } else {
        ok = 0i32;
      }
    }
    {
      BrotliFree(m, storage);
      storage = 0i32;
    }
    {
      BrotliFree(m, commands);
      commands = 0i32;
    }
  }
  *encoded_size = total_out_size;
  DestroyHasher(m, &mut hasher);
  return ok;
  BrotliWipeOutMemoryManager(m);
  0i32
}
*/

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum BrotliEncoderOperation {
  BROTLI_OPERATION_PROCESS = 0,
  BROTLI_OPERATION_FLUSH = 1,
  BROTLI_OPERATION_FINISH = 2,
  BROTLI_OPERATION_EMIT_METADATA = 3,
}

fn MakeUncompressedStream(mut input: &[u8], mut input_size: usize, mut output: &mut [u8]) -> usize {
  let mut size: usize = input_size;
  let mut result: usize = 0usize;
  let mut offset: usize = 0usize;
  if input_size == 0usize {
    output[(0usize)] = 6i32 as (u8);
    return 1usize;
  }
  output[({
     let _old = result;
     result = result.wrapping_add(1 as (usize));
     _old
   } as (usize))] = 0x21i32 as (u8);
  output[({
     let _old = result;
     result = result.wrapping_add(1 as (usize));
     _old
   } as (usize))] = 0x3i32 as (u8);
  while size > 0usize {
    let mut nibbles: u32 = 0u32;
    let mut chunk_size: u32;
    let mut bits: u32;
    chunk_size = if size > (1u32 << 24i32) as (usize) {
      1u32 << 24i32
    } else {
      size as (u32)
    };
    if chunk_size > 1u32 << 16i32 {
      nibbles = if chunk_size > 1u32 << 20i32 {
        2i32
      } else {
        1i32
      } as (u32);
    }
    bits = nibbles << 1i32 | chunk_size.wrapping_sub(1u32) << 3i32 |
           1u32 << (19u32).wrapping_add((4u32).wrapping_mul(nibbles));
    output[({
       let _old = result;
       result = result.wrapping_add(1 as (usize));
       _old
     } as (usize))] = bits as (u8);
    output[({
       let _old = result;
       result = result.wrapping_add(1 as (usize));
       _old
     } as (usize))] = (bits >> 8i32) as (u8);
    output[({
       let _old = result;
       result = result.wrapping_add(1 as (usize));
       _old
     } as (usize))] = (bits >> 16i32) as (u8);
    if nibbles == 2u32 {
      output[({
         let _old = result;
         result = result.wrapping_add(1 as (usize));
         _old
       } as (usize))] = (bits >> 24i32) as (u8);
    }
    output[(result as usize)..(result + chunk_size as usize)].clone_from_slice(
           &input[offset .. (offset + chunk_size as usize)]);
    result = result.wrapping_add(chunk_size as (usize));
    offset = offset.wrapping_add(chunk_size as (usize));
    size = size.wrapping_sub(chunk_size as (usize));
  }
  output[({
     let _old = result;
     result = result.wrapping_add(1 as (usize));
     _old
   } as (usize))] = 3i32 as (u8);
  result
}
/* FIXME: add again later
pub fn BrotliEncoderCompress(mut quality: i32,
                             mut lgwin: i32,
                             mut mode: BrotliEncoderMode,
                             mut input_size: usize,
                             mut input_buffer: &[u8],
                             mut encoded_size: &mut [usize],
                             mut encoded_buffer: &mut [u8])
                             -> i32 {
  let mut s: *mut BrotliEncoderStateStruct;
  let mut out_size: usize = *encoded_size;
  let mut input_start: *const u8 = input_buffer;
  let mut output_start: *mut u8 = encoded_buffer;
  let mut max_out_size: usize = BrotliEncoderMaxCompressedSize(input_size);
  if out_size == 0usize {
    return 0i32;
  }
  if input_size == 0usize {
    *encoded_size = 1usize;
    *encoded_buffer = 6i32 as (u8);
    return 1i32;
  }
  let mut is_fallback: i32 = 0i32;
  if quality == 10i32 {
    let lg_win: i32 = brotli_min_int(24i32, brotli_max_int(16i32, lgwin));
      panic!("Unimplemented: need to set 9.5 here");
    //let mut ok: i32 = BrotliCompressBufferQuality10(lg_win,
    //                                                input_size,
    //                                                input_buffer,
    //                                                encoded_size,
    //                                                encoded_buffer);
    //if ok == 0 || max_out_size != 0 && (*encoded_size > max_out_size) {
    //  is_fallback = 1i32;
    //} else {
    //  return 1i32;
    //}
  }
  if is_fallback == 0 {
    s = BrotliEncoderCreateInstance(0i32 as
                                    (fn(*mut ::std::os::raw::c_void, usize)
                                        -> *mut ::std::os::raw::c_void),
                                    0i32 as
                                    (fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void)),
                                    0i32);
    if s.is_null() {
      return 0i32;
    } else {
      let mut available_in: usize = input_size;
      let mut next_in: *const u8 = input_buffer;
      let mut available_out: usize = *encoded_size;
      let mut next_out: *mut u8 = encoded_buffer;
      let mut total_out: usize = 0usize;
      let mut result: i32 = 0i32;
      BrotliEncoderSetParameter(s,
                                BrotliEncoderParameter::BROTLI_PARAM_QUALITY,
                                quality as (u32));
      BrotliEncoderSetParameter(s,
                                BrotliEncoderParameter::BROTLI_PARAM_LGWIN,
                                lgwin as (u32));
      BrotliEncoderSetParameter(s, BrotliEncoderParameter::BROTLI_PARAM_MODE, mode as (u32));
      BrotliEncoderSetParameter(s,
                                BrotliEncoderParameter::BROTLI_PARAM_SIZE_HINT,
                                input_size as (u32));
      result = BrotliEncoderCompressStream(s,
                                           BrotliEncoderOperation::BROTLI_OPERATION_FINISH,
                                           &mut available_in,
                                           &mut next_in,
                                           &mut available_out,
                                           &mut next_out,
                                           &mut total_out);
      if BrotliEncoderIsFinished(s) == 0 {
        result = 0i32;
      }
      *encoded_size = total_out;
      BrotliEncoderDestroyInstance(s);
      if result == 0 || max_out_size != 0 && (*encoded_size > max_out_size) {
        is_fallback = 1i32;
      } else {
        return 1i32;
      }
    }
  }
  *encoded_size = 0usize;
  if max_out_size == 0 {
    return 0i32;
  }
  if out_size >= max_out_size {
    *encoded_size = MakeUncompressedStream(input_start, input_size, output_start);
    return 1i32;
  }
  0i32
}

fn InjectBytePaddingBlock<AllocU8: alloc::Allocator<u8>,
                        AllocU16: alloc::Allocator<u16>,
                        AllocU32: alloc::Allocator<u32>,
                        AllocI32: alloc::Allocator<i32>,
                        AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8,
                                                                                                      AllocU16,
                                                                                                      AllocU32,
AllocI32, 
                                                                                                      AllocCommand>) {
  let mut seal: u32 = (*s).last_byte_ as (u32);
  let mut seal_bits: usize = (*s).last_byte_bits_ as (usize);
  let mut destination: *mut u8;
  (*s).last_byte_ = 0i32 as (u8);
  (*s).last_byte_bits_ = 0i32 as (u8);
  seal = seal | 0x6u32 << seal_bits;
  seal_bits = seal_bits.wrapping_add(6usize);
  if !(*s).next_out_.is_null() {
    destination = (*s).next_out_[((*s).available_out_ as (usize))..];
  } else {
    destination = (*s).tiny_buf_.u8.as_mut_ptr();
    (*s).next_out_ = destination;
  }
  destination[(0usize)] = seal as (u8);
  if seal_bits > 8usize {
    destination[(1usize)] = (seal >> 8i32) as (u8);
  }
  (*s).available_out_ = (*s).available_out_.wrapping_add(seal_bits.wrapping_add(7usize) >> 3i32);
}
fn InjectFlushOrPushOutput<AllocU8: alloc::Allocator<u8>,
                           AllocU16: alloc::Allocator<u16>,
                           AllocU32: alloc::Allocator<u32>,
                           AllocI32: alloc::Allocator<i32>,
                           AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8,
                                                                                                         AllocU16,
                                                                                                         AllocU32,
AllocI32, 
                                                                                                         AllocCommand>,
                           mut available_out: &mut [usize],
                           mut next_out: &mut [*mut u8],
                           mut total_out: &mut usize)
                           -> i32 {
  if (*s).stream_state_ as (i32) ==
     BrotliEncoderStreamState::BROTLI_STREAM_FLUSH_REQUESTED as (i32) &&
     ((*s).last_byte_bits_ as (i32) != 0i32) {
    InjectBytePaddingBlock(s);
    return 1i32;
  }
  if (*s).available_out_ != 0usize && (*available_out != 0usize) {
    let mut copy_output_size: usize = brotli_min_size_t((*s).available_out_, *available_out);
    memcpy(*next_out, (*s).next_out_, copy_output_size);
    *next_out = (*next_out).offset(copy_output_size as (isize));
    *available_out = (*available_out).wrapping_sub(copy_output_size);
    (*s).next_out_ = (*s).next_out_[(copy_output_size as (usize))..];
    (*s).available_out_ = (*s).available_out_.wrapping_sub(copy_output_size);
    (*s).total_out_ = (*s).total_out_.wrapping_add(copy_output_size);
    if !total_out.is_null() {
      *total_out = (*s).total_out_;
    }
    return 1i32;
  }
  0i32
}
*/

fn UnprocessedInputSize<AllocU8: alloc::Allocator<u8>,
                        AllocU16: alloc::Allocator<u16>,
                        AllocU32: alloc::Allocator<u32>,
                        AllocI32: alloc::Allocator<i32>,
                        AllocCommand: alloc::Allocator<Command>>(
                            mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) -> usize {
  (*s).input_pos_.wrapping_sub((*s).last_processed_pos_)
}

fn UpdateSizeHint<AllocU8: alloc::Allocator<u8>,
                  AllocU16: alloc::Allocator<u16>,
                  AllocU32: alloc::Allocator<u32>,
                  AllocI32: alloc::Allocator<i32>,
                  AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
                        mut available_in: usize) {
  if (*s).params.size_hint == 0usize {
    let mut delta: usize = UnprocessedInputSize(s);
    let mut tail: usize = available_in;
    let mut limit: u32 = 1u32 << 30i32;
    let mut total: u32;
    if delta >= limit as (usize) || tail >= limit as (usize) ||
       delta.wrapping_add(tail) >= limit as (usize) {
      total = limit;
    } else {
      total = delta.wrapping_add(tail) as (u32);
    }
    (*s).params.size_hint = total as (usize);
  }
}


fn WrapPosition(mut position: usize) -> u32 {
  let mut result: u32 = position as (u32);
  let mut gb: usize = position >> 30i32;
  if gb > 2usize {
    result = result & (1u32 << 30i32).wrapping_sub(1u32) |
             ((gb.wrapping_sub(1usize) & 1usize) as (u32)).wrapping_add(1u32) << 30i32;
  }
  result
}

fn InputBlockSize<AllocU8: alloc::Allocator<u8>,
                  AllocU16: alloc::Allocator<u16>,
                  AllocU32: alloc::Allocator<u32>,
                  AllocI32: alloc::Allocator<i32>,
                  AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8,
                                                                                                 AllocU16,
                                                                                                AllocU32,
                                                                                                AllocI32, 
                                                                                                 AllocCommand>) -> usize {
  if EnsureInitialized(s) == 0 {
    return 0usize;
  }
  1usize << (*s).params.lgblock
}

fn GetBrotliStorage<AllocU8: alloc::Allocator<u8>,
                    AllocU16: alloc::Allocator<u16>,
                    AllocU32: alloc::Allocator<u32>,
                    AllocI32: alloc::Allocator<i32>,
                    AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8,
                                                                                                  AllocU16,
                                                                                                  AllocU32,
                                                                                                  AllocI32, 
                                                                                                  AllocCommand>,
                                                                    mut size: usize) {
  if (*s).storage_size_ < size {
    (*s).m8.free_cell(core::mem::replace(&mut (*s).storage_, AllocU8::AllocatedMemory::default()));
    (*s).storage_ = (*s).m8.alloc_cell(size);
    (*s).storage_size_ = size;
  }
}

fn MaxHashTableSize(quality: i32) -> usize {
  (if quality == 0i32 {
     1i32 << 15i32
   } else {
     1i32 << 17i32
   }) as (usize)
}

fn HashTableSize(mut max_table_size: usize, mut input_size: usize) -> usize {
  let mut htsize: usize = 256usize;
  while htsize < max_table_size && (htsize < input_size) {
    htsize = htsize << 1i32;
  }
  htsize
}


fn GetHashTable<'a, AllocU8: alloc::Allocator<u8>,
                 AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &'a mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
                mut quality: i32,
                mut input_size: usize,
                mut table_size: &mut usize)
                -> &'a mut [i32] {
  let max_table_size: usize = MaxHashTableSize(quality);
  let mut htsize: usize = HashTableSize(max_table_size, input_size);
  let mut table: &mut [i32];
  if quality == 0i32 {
    if htsize & 0xaaaaausize == 0usize {
      htsize = htsize << 1i32;
    }
  }
  if htsize <= (*s).small_table_.len() {
    table = &mut (*s).small_table_[..];
  } else {
    if htsize > (*s).large_table_.slice().len() {
      (*s).large_table_size_ = htsize;
      {
          (*s).mi32.free_cell(core::mem::replace(&mut (*s).large_table_,
                                                 AllocI32::AllocatedMemory::default()));
      }
      (*s).large_table_ = (*s).mi32.alloc_cell(htsize);
    }
    table = (*s).large_table_.slice_mut();
  }
  *table_size = htsize;
  for item in table[..htsize].iter_mut() {
      *item = 0;
  }
  table // FIXME: probably need a macro to do this without borrowing the whole EncoderStateStruct
}
fn UpdateLastProcessedPos<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) -> i32 {
  let mut wrapped_last_processed_pos: u32 = WrapPosition((*s).last_processed_pos_);
  let mut wrapped_input_pos: u32 = WrapPosition((*s).input_pos_);
  (*s).last_processed_pos_ = (*s).input_pos_;
  if !!(wrapped_input_pos < wrapped_last_processed_pos) {
    1i32
  } else {
    0i32
  }
}

fn MaxMetablockSize(mut params: &BrotliEncoderParams) -> usize {
  let mut bits: i32 = brotli_min_int(ComputeRbBits(params), 24i32);
  1usize << bits
}



fn ChooseContextMap(mut quality: i32,
                    mut bigram_histo: &mut [u32],
                    mut num_literal_contexts: &mut usize,
                    mut literal_context_map: &mut &[u32]) {
  static kStaticContextMapContinuation: [u32; 64] =
    [1u32, 1u32, 2u32, 2u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 0u32];
  static kStaticContextMapSimpleUTF8: [u32; 64] =
    [0u32, 0u32, 1u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 0u32];
  let mut monogram_histo: [u32; 3] = [0u32, 0u32, 0u32];
  let mut two_prefix_histo: [u32; 6] = [0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
  let mut total: usize;
  let mut i: usize;
  let mut dummy: usize = 0;
  let mut entropy: [f64; 4] = [0.0f64;4];
  i = 0usize;
  while i < 9usize {
    {
      {
        let _rhs = bigram_histo[(i as (usize))];
        let _lhs = &mut monogram_histo[i.wrapping_rem(3usize)];
        *_lhs = (*_lhs).wrapping_add(_rhs);
      }
      {
        let _rhs = bigram_histo[(i as (usize))];
        let _lhs = &mut two_prefix_histo[i.wrapping_rem(6usize)];
        *_lhs = (*_lhs).wrapping_add(_rhs);
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  entropy[1usize] = ShannonEntropy(&monogram_histo[..], 3usize, &mut dummy);
  entropy[2usize] = ShannonEntropy(&two_prefix_histo[..], 3usize, &mut dummy) +
                    ShannonEntropy(&two_prefix_histo[3i32 as (usize)..],
                                   3usize,
                                   &mut dummy);
  entropy[3usize] = 0i32 as (f64);
  i = 0usize;
  while i < 3usize {
    {
      let _rhs = ShannonEntropy(&bigram_histo[((3usize).wrapping_mul(i) as (usize))..],
                                3usize,
                                &mut dummy);
      let _lhs = &mut entropy[3usize];
      *_lhs = *_lhs + _rhs;
    }
    i = i.wrapping_add(1 as (usize));
  }
  total = monogram_histo[0usize]
    .wrapping_add(monogram_histo[1usize])
    .wrapping_add(monogram_histo[2usize]) as (usize);
  0i32;
  entropy[0usize] = 1.0f64 / total as (f64);
  {
    let _rhs = entropy[0usize];
    let _lhs = &mut entropy[1usize];
    *_lhs = *_lhs * _rhs;
  }
  {
    let _rhs = entropy[0usize];
    let _lhs = &mut entropy[2usize];
    *_lhs = *_lhs * _rhs;
  }
  {
    let _rhs = entropy[0usize];
    let _lhs = &mut entropy[3usize];
    *_lhs = *_lhs * _rhs;
  }
  if quality < 7i32 {
    entropy[3usize] = entropy[1usize] * 10i32 as (f64);
  }
  if entropy[1usize] - entropy[2usize] < 0.2f64 && (entropy[1usize] - entropy[3usize] < 0.2f64) {
    *num_literal_contexts = 1usize;
  } else if entropy[2usize] - entropy[3usize] < 0.02f64 {
    *num_literal_contexts = 2usize;
    *literal_context_map = &kStaticContextMapSimpleUTF8[..];
  } else {
    *num_literal_contexts = 3usize;
    *literal_context_map = &kStaticContextMapContinuation[..];
  }
}


fn DecideOverLiteralContextModeling(mut input: &[u8],
                                    mut start_pos: usize,
                                    mut length: usize,
                                    mut mask: usize,
                                    mut quality: i32,
                                    mut literal_context_mode: &mut ContextType,
                                    mut num_literal_contexts: &mut usize,
                                    mut literal_context_map: &mut &[u32]) {
  if quality < 5i32 || length < 64usize {
  } else {
    let end_pos: usize = start_pos.wrapping_add(length);
    let mut bigram_prefix_histo: [u32; 9] = [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
    while start_pos.wrapping_add(64usize) <= end_pos {
      {
        static lut: [i32; 4] = [0i32, 0i32, 1i32, 2i32];
        let stride_end_pos: usize = start_pos.wrapping_add(64usize);
        let mut prev: i32 = lut[(input[((start_pos & mask) as (usize))] as (i32) >> 6i32) as
        (usize)] * 3i32;
        let mut pos: usize;
        pos = start_pos.wrapping_add(1usize);
        while pos < stride_end_pos {
          {
            let literal: u8 = input[((pos & mask) as (usize))];
            {
              let _rhs = 1;
              let cur_ind = (prev + lut[(literal as (i32) >> 6i32) as (usize)]);
              let _lhs = &mut bigram_prefix_histo[cur_ind as
                              (usize)];
              *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            prev = lut[(literal as (i32) >> 6i32) as (usize)] * 3i32;
          }
          pos = pos.wrapping_add(1 as (usize));
        }
      }
      start_pos = start_pos.wrapping_add(4096usize);
    }
    *literal_context_mode = ContextType::CONTEXT_UTF8;
    ChooseContextMap(quality,
                     &mut bigram_prefix_histo[..],
                     num_literal_contexts,
                     literal_context_map);
  }
}
/*
fn WriteMetaBlockInternal(mut m: &mut [MemoryManager],
                          mut data: &[u8],
                          mask: usize,
                          last_flush_pos: usize,
                          bytes: usize,
                          is_last: i32,
                          mut params: &[BrotliEncoderParams],
                          prev_byte: u8,
                          prev_byte2: u8,
                          num_literals: usize,
                          num_commands: usize,
                          mut commands: &mut [Command],
                          mut saved_dist_cache: &[i32],
                          mut dist_cache: &mut [i32],
                          mut storage_ix: &mut [usize],
                          mut storage: &mut [u8]) {
  let wrapped_last_flush_pos: u32 = WrapPosition(last_flush_pos);
  let mut last_byte: u8;
  let mut last_byte_bits: u8;
  let mut num_direct_distance_codes: u32 = 0u32;
  let mut distance_postfix_bits: u32 = 0u32;
  if bytes == 0usize {
    BrotliWriteBits(2usize, 3usize, storage_ix, storage);
    *storage_ix = (*storage_ix).wrapping_add(7u32 as (usize)) & !7u32 as (usize);
    return;
  }
  if ShouldCompress(data,
                    mask,
                    last_flush_pos,
                    bytes,
                    num_literals,
                    num_commands) == 0 {
    memcpy(dist_cache,
           saved_dist_cache,
           (4usize).wrapping_mul(::std::mem::size_of::<i32>()));
    BrotliStoreUncompressedMetaBlock(is_last,
                                     data,
                                     wrapped_last_flush_pos as (usize),
                                     mask,
                                     bytes,
                                     storage_ix,
                                     storage);
    return;
  }
  last_byte = storage[(0usize)];
  last_byte_bits = (*storage_ix & 0xffusize) as (u8);
  if (*params).quality >= 10i32 &&
     ((*params).mode as (i32) == BrotliEncoderMode::BROTLI_MODE_FONT as (i32)) {
    num_direct_distance_codes = 12u32;
    distance_postfix_bits = 1u32;
    RecomputeDistancePrefixes(commands,
                              num_commands,
                              num_direct_distance_codes,
                              distance_postfix_bits);
  }
  if (*params).quality <= 2i32 {
    BrotliStoreMetaBlockFast(m,
                             data,
                             wrapped_last_flush_pos as (usize),
                             bytes,
                             mask,
                             is_last,
                             commands,
                             num_commands,
                             storage_ix,
                             storage);
    if !(0i32 == 0) {
      return;
    }
  } else if (*params).quality < 4i32 {
    BrotliStoreMetaBlockTrivial(m,
                                data,
                                wrapped_last_flush_pos as (usize),
                                bytes,
                                mask,
                                is_last,
                                commands,
                                num_commands,
                                storage_ix,
                                storage);
    if !(0i32 == 0) {
      return;
    }
  } else {
    let mut literal_context_mode: ContextType = ContextType::CONTEXT_UTF8;
    let mut mb: MetaBlockSplit;
    InitMetaBlockSplit(&mut mb);
    if (*params).quality < 10i32 {
      let mut num_literal_contexts: usize = 1usize;
      let mut literal_context_map: *const u32 = 0i32;
      if (*params).disable_literal_context_modeling == 0 {
        DecideOverLiteralContextModeling(data,
                                         wrapped_last_flush_pos as (usize),
                                         bytes,
                                         mask,
                                         (*params).quality,
                                         &mut literal_context_mode,
                                         &mut num_literal_contexts,
                                         &mut literal_context_map);
      }
      BrotliBuildMetaBlockGreedy(m,
                                 data,
                                 wrapped_last_flush_pos as (usize),
                                 mask,
                                 prev_byte,
                                 prev_byte2,
                                 literal_context_mode,
                                 num_literal_contexts,
                                 literal_context_map,
                                 commands,
                                 num_commands,
                                 &mut mb);
      if !(0i32 == 0) {
        return;
      }
    } else {
      if BrotliIsMostlyUTF8(data,
                            wrapped_last_flush_pos as (usize),
                            mask,
                            bytes,
                            kMinUTF8Ratio) == 0 {
        literal_context_mode = ContextType::CONTEXT_SIGNED;
      }
      BrotliBuildMetaBlock(m,
                           data,
                           wrapped_last_flush_pos as (usize),
                           mask,
                           params,
                           prev_byte,
                           prev_byte2,
                           commands,
                           num_commands,
                           literal_context_mode,
                           &mut mb);
      if !(0i32 == 0) {
        return;
      }
    }
    if (*params).quality >= 4i32 {
      BrotliOptimizeHistograms(num_direct_distance_codes as (usize),
                               distance_postfix_bits as (usize),
                               &mut mb);
    }
    BrotliStoreMetaBlock(m,
                         data,
                         wrapped_last_flush_pos as (usize),
                         bytes,
                         mask,
                         prev_byte,
                         prev_byte2,
                         is_last,
                         num_direct_distance_codes,
                         distance_postfix_bits,
                         literal_context_mode,
                         commands,
                         num_commands,
                         &mut mb,
                         storage_ix,
                         storage);
    if !(0i32 == 0) {
      return;
    }
    DestroyMetaBlockSplit(m, &mut mb);
  }
  if bytes.wrapping_add(4usize) < *storage_ix >> 3i32 {
    memcpy(dist_cache,
           saved_dist_cache,
           (4usize).wrapping_mul(::std::mem::size_of::<i32>()));
    storage[(0usize)] = last_byte;
    *storage_ix = last_byte_bits as (usize);
    BrotliStoreUncompressedMetaBlock(is_last,
                                     data,
                                     wrapped_last_flush_pos as (usize),
                                     mask,
                                     bytes,
                                     storage_ix,
                                     storage);
  }
}

fn EncodeData<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
              is_last: i32,
              force_flush: i32,
              mut out_size: &mut [usize],
              mut output: &mut [*mut u8])
              -> i32 {
  let delta: usize = UnprocessedInputSize(s);
  let bytes: u32 = delta as (u32);
  let wrapped_last_processed_pos: u32 = WrapPosition((*s).last_processed_pos_);
  let mut data: *mut u8;
  let mut mask: u32;
  let mut m: *mut MemoryManager = &mut (*s).memory_manager_;
  let mut dictionary: *const BrotliDictionary = BrotliGetDictionary();
  if EnsureInitialized(s) == 0 {
    return 0i32;
  }
  data = &mut *(*s).ringbuffer_.data_[((*s).ringbuffer_.buffer_index as (usize))..];
  mask = (*s).ringbuffer_.mask_;
  if (*s).is_last_block_emitted_ != 0 {
    return 0i32;
  }
  if is_last != 0 {
    (*s).is_last_block_emitted_ = 1i32;
  }
  if delta > InputBlockSize(s) {
    return 0i32;
  }
  if (*s).params.quality == 1i32 && (*s).command_buf_.is_null() {
    (*s).command_buf_ = if kCompressFragmentTwoPassBlockSize != 0 {
      BrotliAllocate(m,
                     kCompressFragmentTwoPassBlockSize.wrapping_mul(::std::mem::size_of::<u32>()))
    } else {
      0i32
    };
    (*s).literal_buf_ = if kCompressFragmentTwoPassBlockSize != 0 {
      BrotliAllocate(m,
                     kCompressFragmentTwoPassBlockSize.wrapping_mul(::std::mem::size_of::<u8>()))
    } else {
      0i32
    };
    if !(0i32 == 0) {
      return 0i32;
    }
  }
  if (*s).params.quality == 0i32 || (*s).params.quality == 1i32 {
    let mut storage: *mut u8;
    let mut storage_ix: usize = (*s).last_byte_bits_ as (usize);
    let mut table_size: usize;
    let mut table: *mut i32;
    if delta == 0usize && (is_last == 0) {
      *out_size = 0usize;
      return 1i32;
    }
    storage = GetBrotliStorage(s,
                               (2u32).wrapping_mul(bytes).wrapping_add(502u32) as (usize));
    if !(0i32 == 0) {
      return 0i32;
    }
    storage[(0usize)] = (*s).last_byte_;
    table = GetHashTable(s, (*s).params.quality, bytes as (usize), &mut table_size);
    if !(0i32 == 0) {
      return 0i32;
    }
    if (*s).params.quality == 0i32 {
      BrotliCompressFragmentFast(m,
                                 &mut data[((wrapped_last_processed_pos & mask) as (usize))],
                                 bytes as (usize),
                                 is_last,
                                 table,
                                 table_size,
                                 (*s).cmd_depths_.as_mut_ptr(),
                                 (*s).cmd_bits_.as_mut_ptr(),
                                 &mut (*s).cmd_code_numbits_,
                                 (*s).cmd_code_.as_mut_ptr(),
                                 &mut storage_ix,
                                 storage);
      if !(0i32 == 0) {
        return 0i32;
      }
    } else {
      BrotliCompressFragmentTwoPass(m,
                                    &mut data[((wrapped_last_processed_pos & mask) as (usize))],
                                    bytes as (usize),
                                    is_last,
                                    (*s).command_buf_,
                                    (*s).literal_buf_,
                                    table,
                                    table_size,
                                    &mut storage_ix,
                                    storage);
      if !(0i32 == 0) {
        return 0i32;
      }
    }
    (*s).last_byte_ = storage[((storage_ix >> 3i32) as (usize))];
    (*s).last_byte_bits_ = (storage_ix & 7u32 as (usize)) as (u8);
    UpdateLastProcessedPos(s);
    *output = &mut storage[(0usize)];
    *out_size = storage_ix >> 3i32;
    return 1i32;
  }
  {
    let mut newsize: usize =
      (*s).num_commands_.wrapping_add(bytes.wrapping_div(2u32) as (usize)).wrapping_add(1usize);
    if newsize > (*s).cmd_alloc_size_ {
      let mut new_commands: *mut Command;
      newsize = newsize.wrapping_add(bytes.wrapping_div(4u32).wrapping_add(16u32) as (usize));
      (*s).cmd_alloc_size_ = newsize;
      new_commands = if newsize != 0 {
        BrotliAllocate(m, newsize.wrapping_mul(::std::mem::size_of::<Command>()))
      } else {
        0i32
      };
      if !(0i32 == 0) {
        return 0i32;
      }
      if !(*s).commands_.is_null() {
        memcpy(new_commands,
               (*s).commands_,
               ::std::mem::size_of::<Command>().wrapping_mul((*s).num_commands_));
        {
          BrotliFree(m, (*s).commands_);
          (*s).commands_ = 0i32;
        }
      }
      (*s).commands_ = new_commands;
    }
  }
  InitOrStitchToPreviousBlock(m,
                              &mut (*s).hasher_,
                              data,
                              mask as (usize),
                              &mut (*s).params,
                              wrapped_last_processed_pos as (usize),
                              bytes as (usize),
                              is_last);
  if !(0i32 == 0) {
    return 0i32;
  }
  if (*s).params.quality == 10i32 {
    0i32;
    BrotliCreateZopfliBackwardReferences(m,
                                         dictionary,
                                         bytes as (usize),
                                         wrapped_last_processed_pos as (usize),
                                         data,
                                         mask as (usize),
                                         &mut (*s).params,
                                         (*s).hasher_,
                                         (*s).dist_cache_.as_mut_ptr(),
                                         &mut (*s).last_insert_len_,
                                         &mut *(*s).commands_[((*s).num_commands_ as (usize))..],
                                         &mut (*s).num_commands_,
                                         &mut (*s).num_literals_);
    if !(0i32 == 0) {
      return 0i32;
    }
  } else if (*s).params.quality == 11i32 {
    0i32;
    BrotliCreateHqZopfliBackwardReferences(m,
                                           dictionary,
                                           bytes as (usize),
                                           wrapped_last_processed_pos as (usize),
                                           data,
                                           mask as (usize),
                                           &mut (*s).params,
                                           (*s).hasher_,
                                           (*s).dist_cache_.as_mut_ptr(),
                                           &mut (*s).last_insert_len_,
                                           &mut *(*s).commands_[((*s).num_commands_ as (usize))..],
                                           &mut (*s).num_commands_,
                                           &mut (*s).num_literals_);
    if !(0i32 == 0) {
      return 0i32;
    }
  } else {
    BrotliCreateBackwardReferences(dictionary,
                                   bytes as (usize),
                                   wrapped_last_processed_pos as (usize),
                                   data,
                                   mask as (usize),
                                   &mut (*s).params,
                                   (*s).hasher_,
                                   (*s).dist_cache_.as_mut_ptr(),
                                   &mut (*s).last_insert_len_,
                                   &mut *(*s).commands_[((*s).num_commands_ as (usize))..],
                                   &mut (*s).num_commands_,
                                   &mut (*s).num_literals_);
  }
  {
    let max_length: usize = MaxMetablockSize(&mut (*s).params);
    let max_literals: usize = max_length.wrapping_div(8usize);
    let max_commands: usize = max_length.wrapping_div(8usize);
    let processed_bytes: usize = (*s).input_pos_.wrapping_sub((*s).last_flush_pos_);
    let next_input_fits_metablock: i32 = if !!(processed_bytes.wrapping_add(InputBlockSize(s)) <=
                                               max_length) {
      1i32
    } else {
      0i32
    };
    let should_flush: i32 = if !!((*s).params.quality < 4i32 &&
                                  ((*s).num_literals_.wrapping_add((*s).num_commands_) >=
                                   0x2fffusize)) {
      1i32
    } else {
      0i32
    };
    if is_last == 0 && (force_flush == 0) && (should_flush == 0) &&
       (next_input_fits_metablock != 0) && ((*s).num_literals_ < max_literals) &&
       ((*s).num_commands_ < max_commands) {
      if UpdateLastProcessedPos(s) != 0 {
        HasherReset((*s).hasher_);
      }
      *out_size = 0usize;
      return 1i32;
    }
  }
  if (*s).last_insert_len_ > 0usize {
    InitInsertCommand(&mut *(*s).commands_[({
                               let _old = (*s).num_commands_;
                               (*s).num_commands_ = (*s).num_commands_.wrapping_add(1 as (usize));
                               _old
                             } as (usize))..],
                      (*s).last_insert_len_);
    (*s).num_literals_ = (*s).num_literals_.wrapping_add((*s).last_insert_len_);
    (*s).last_insert_len_ = 0usize;
  }
  if is_last == 0 && ((*s).input_pos_ == (*s).last_flush_pos_) {
    *out_size = 0usize;
    return 1i32;
  }
  0i32;
  0i32;
  0i32;
  {
    let metablock_size: u32 = (*s).input_pos_.wrapping_sub((*s).last_flush_pos_) as (u32);
    let mut storage: *mut u8 =
      GetBrotliStorage(s,
                       (2u32).wrapping_mul(metablock_size).wrapping_add(502u32) as (usize));
    let mut storage_ix: usize = (*s).last_byte_bits_ as (usize);
    if !(0i32 == 0) {
      return 0i32;
    }
    storage[(0usize)] = (*s).last_byte_;
    WriteMetaBlockInternal(m,
                           data,
                           mask as (usize),
                           (*s).last_flush_pos_,
                           metablock_size as (usize),
                           is_last,
                           &mut (*s).params,
                           (*s).prev_byte_,
                           (*s).prev_byte2_,
                           (*s).num_literals_,
                           (*s).num_commands_,
                           (*s).commands_,
                           (*s).saved_dist_cache_.as_mut_ptr(),
                           (*s).dist_cache_.as_mut_ptr(),
                           &mut storage_ix,
                           storage);
    if !(0i32 == 0) {
      return 0i32;
    }
    (*s).last_byte_ = storage[((storage_ix >> 3i32) as (usize))];
    (*s).last_byte_bits_ = (storage_ix & 7u32 as (usize)) as (u8);
    (*s).last_flush_pos_ = (*s).input_pos_;
    if UpdateLastProcessedPos(s) != 0 {
      HasherReset((*s).hasher_);
    }
    if (*s).last_flush_pos_ > 0usize {
      (*s).prev_byte_ = data[((((*s).last_flush_pos_ as (u32)).wrapping_sub(1u32) & mask) as
       (usize))];
    }
    if (*s).last_flush_pos_ > 1usize {
      (*s).prev_byte2_ = data[(((*s).last_flush_pos_.wrapping_sub(2usize) as (u32) & mask) as
       (usize))];
    }
    (*s).num_commands_ = 0usize;
    (*s).num_literals_ = 0usize;
    memcpy((*s).saved_dist_cache_.as_mut_ptr(),
           (*s).dist_cache_.as_mut_ptr(),
           ::std::mem::size_of::<[i32; 4]>());
    *output = &mut storage[(0usize)];
    *out_size = storage_ix >> 3i32;
    1i32
  }
}

fn WriteMetadataHeader<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
                       block_size: usize,
                       mut header: &mut [u8])
                       -> usize {
  let mut storage_ix: usize;
  storage_ix = (*s).last_byte_bits_ as (usize);
  header[(0usize)] = (*s).last_byte_;
  (*s).last_byte_ = 0i32 as (u8);
  (*s).last_byte_bits_ = 0i32 as (u8);
  BrotliWriteBits(1usize, 0usize, &mut storage_ix, header);
  BrotliWriteBits(2usize, 3usize, &mut storage_ix, header);
  BrotliWriteBits(1usize, 0usize, &mut storage_ix, header);
  if block_size == 0usize {
    BrotliWriteBits(2usize, 0usize, &mut storage_ix, header);
  } else {
    let mut nbits: u32 = if block_size == 1usize {
      0u32
    } else {
      Log2FloorNonZero((block_size as (u32)).wrapping_sub(1u32) as (usize)).wrapping_add(1u32)
    };
    let mut nbytes: u32 = nbits.wrapping_add(7u32).wrapping_div(8u32);
    BrotliWriteBits(2usize, nbytes as (usize), &mut storage_ix, header);
    BrotliWriteBits((8u32).wrapping_mul(nbytes) as (usize),
                    block_size.wrapping_sub(1usize),
                    &mut storage_ix,
                    header);
  }
  storage_ix.wrapping_add(7u32 as (usize)) >> 3i32
}

fn brotli_min_uint32_t(mut a: u32, mut b: u32) -> u32 {
  if a < b { a } else { b }
}

fn ProcessMetadata<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
                   mut available_in: &mut [usize],
                   mut next_in: &mut [&[u8]],
                   mut available_out: &mut [usize],
                   mut next_out: &mut [*mut u8],
                   mut total_out: &mut usize)
                   -> i32 {
  if *available_in > (1u32 << 24i32) as (usize) {
    return 0i32;
  }
  if (*s).stream_state_ as (i32) == BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING as (i32) {
    (*s).remaining_metadata_bytes_ = *available_in as (u32);
    (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_METADATA_HEAD;
  }
  if (*s).stream_state_ as (i32) !=
     BrotliEncoderStreamState::BROTLI_STREAM_METADATA_HEAD as (i32) &&
     ((*s).stream_state_ as (i32) !=
      BrotliEncoderStreamState::BROTLI_STREAM_METADATA_BODY as (i32)) {
    return 0i32;
  }
  while 1i32 != 0 {
    if InjectFlushOrPushOutput(s, available_out, next_out, total_out) != 0 {
      {
        continue;
      }
    }
    if (*s).available_out_ != 0usize {
      {
        break;
      }
    }
    if (*s).input_pos_ != (*s).last_flush_pos_ {
      let mut result: i32 =
        EncodeData(s, 0i32, 1i32, &mut (*s).available_out_, &mut (*s).next_out_);
      if result == 0 {
        return 0i32;
      }
      {
        {
          continue;
        }
      }
    }
    if (*s).stream_state_ as (i32) ==
       BrotliEncoderStreamState::BROTLI_STREAM_METADATA_HEAD as (i32) {
      (*s).next_out_ = (*s).tiny_buf_.u8.as_mut_ptr();
      (*s).available_out_ =
        WriteMetadataHeader(s, (*s).remaining_metadata_bytes_ as (usize), (*s).next_out_);
      (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_METADATA_BODY;
      {
        {
          continue;
        }
      }
    } else {
      if (*s).remaining_metadata_bytes_ == 0u32 {
        (*s).remaining_metadata_bytes_ = !(0u32);
        (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING;
        {
          {
            break;
          }
        }
      }
      if *available_out != 0 {
        let mut copy: u32 = brotli_min_size_t((*s).remaining_metadata_bytes_ as (usize),
                                              *available_out) as (u32);
        memcpy(*next_out, *next_in, copy as (usize));
        *next_in = (*next_in).offset(copy as (isize));
        *available_in = (*available_in).wrapping_sub(copy as (usize));
        (*s).remaining_metadata_bytes_ = (*s).remaining_metadata_bytes_.wrapping_sub(copy);
        *next_out = (*next_out).offset(copy as (isize));
        *available_out = (*available_out).wrapping_sub(copy as (usize));
      } else {
        let mut copy: u32 = brotli_min_uint32_t((*s).remaining_metadata_bytes_, 16u32);
        (*s).next_out_ = (*s).tiny_buf_.u8.as_mut_ptr();
        memcpy((*s).next_out_, *next_in, copy as (usize));
        *next_in = (*next_in).offset(copy as (isize));
        *available_in = (*available_in).wrapping_sub(copy as (usize));
        (*s).remaining_metadata_bytes_ = (*s).remaining_metadata_bytes_.wrapping_sub(copy);
        (*s).available_out_ = copy as (usize);
      }
      {
        {
          continue;
        }
      }
    }
  }
  1i32
}

fn CheckFlushComplete<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) {
  if (*s).stream_state_ as (i32) ==
     BrotliEncoderStreamState::BROTLI_STREAM_FLUSH_REQUESTED as (i32) &&
     ((*s).available_out_ == 0usize) {
    (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING;
    (*s).next_out_ = 0i32;
  }
}

fn BrotliEncoderCompressStreamFast<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
                                   mut op: BrotliEncoderOperation,
                                   mut available_in: &mut [usize],
                                   mut next_in: &mut [&[u8]],
                                   mut available_out: &mut [usize],
                                   mut next_out: &mut [*mut u8],
                                   mut total_out: &mut usize)
                                   -> i32 {
  let block_size_limit: usize = 1usize << (*s).params.lgwin;
  let buf_size: usize = brotli_min_size_t(kCompressFragmentTwoPassBlockSize,
                                          brotli_min_size_t(*available_in, block_size_limit));
  let mut tmp_command_buf: *mut u32 = 0i32;
  let mut command_buf: *mut u32 = 0i32;
  let mut tmp_literal_buf: *mut u8 = 0i32;
  let mut literal_buf: *mut u8 = 0i32;
  let mut m: *mut MemoryManager = &mut (*s).memory_manager_;
  if (*s).params.quality != 0i32 && ((*s).params.quality != 1i32) {
    return 0i32;
  }
  if (*s).params.quality == 1i32 {
    if (*s).command_buf_.is_null() && (buf_size == kCompressFragmentTwoPassBlockSize) {
      (*s).command_buf_ = if kCompressFragmentTwoPassBlockSize != 0 {
        BrotliAllocate(m,
                       kCompressFragmentTwoPassBlockSize.wrapping_mul(::std::mem::size_of::<u32>()))
      } else {
        0i32
      };
      (*s).literal_buf_ = if kCompressFragmentTwoPassBlockSize != 0 {
        BrotliAllocate(m,
                       kCompressFragmentTwoPassBlockSize.wrapping_mul(::std::mem::size_of::<u8>()))
      } else {
        0i32
      };
      if !(0i32 == 0) {
        return 0i32;
      }
    }
    if !(*s).command_buf_.is_null() {
      command_buf = (*s).command_buf_;
      literal_buf = (*s).literal_buf_;
    } else {
      tmp_command_buf = if buf_size != 0 {
        BrotliAllocate(m, buf_size.wrapping_mul(::std::mem::size_of::<u32>()))
      } else {
        0i32
      };
      tmp_literal_buf = if buf_size != 0 {
        BrotliAllocate(m, buf_size.wrapping_mul(::std::mem::size_of::<u8>()))
      } else {
        0i32
      };
      if !(0i32 == 0) {
        return 0i32;
      }
      command_buf = tmp_command_buf;
      literal_buf = tmp_literal_buf;
    }
  }
  while 1i32 != 0 {
    if InjectFlushOrPushOutput(s, available_out, next_out, total_out) != 0 {
      {
        continue;
      }
    }
    if (*s).available_out_ == 0usize &&
       ((*s).stream_state_ as (i32) ==
        BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING as (i32)) &&
       (*available_in != 0usize ||
        op as (i32) != BrotliEncoderOperation::BROTLI_OPERATION_PROCESS as (i32)) {
      let mut block_size: usize = brotli_min_size_t(block_size_limit, *available_in);
      let mut is_last: i32 = (*available_in == block_size &&
                              (op as (i32) ==
                               BrotliEncoderOperation::BROTLI_OPERATION_FINISH as (i32))) as
                             (i32);
      let mut force_flush: i32 =
        (*available_in == block_size &&
         (op as (i32) == BrotliEncoderOperation::BROTLI_OPERATION_FLUSH as (i32))) as (i32);
      let mut max_out_size: usize = (2usize).wrapping_mul(block_size).wrapping_add(502usize);
      let mut inplace: i32 = 1i32;
      let mut storage: *mut u8 = 0i32;
      let mut storage_ix: usize = (*s).last_byte_bits_ as (usize);
      let mut table_size: usize;
      let mut table: *mut i32;
      if force_flush != 0 && (block_size == 0usize) {
        (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_FLUSH_REQUESTED;
        {
          {
            continue;
          }
        }
      }
      if max_out_size <= *available_out {
        storage = *next_out;
      } else {
        inplace = 0i32;
        storage = GetBrotliStorage(s, max_out_size);
        if !(0i32 == 0) {
          return 0i32;
        }
      }
      storage[(0usize)] = (*s).last_byte_;
      table = GetHashTable(s, (*s).params.quality, block_size, &mut table_size);
      if !(0i32 == 0) {
        return 0i32;
      }
      if (*s).params.quality == 0i32 {
        BrotliCompressFragmentFast(m,
                                   *next_in,
                                   block_size,
                                   is_last,
                                   table,
                                   table_size,
                                   (*s).cmd_depths_.as_mut_ptr(),
                                   (*s).cmd_bits_.as_mut_ptr(),
                                   &mut (*s).cmd_code_numbits_,
                                   (*s).cmd_code_.as_mut_ptr(),
                                   &mut storage_ix,
                                   storage);
        if !(0i32 == 0) {
          return 0i32;
        }
      } else {
        BrotliCompressFragmentTwoPass(m,
                                      *next_in,
                                      block_size,
                                      is_last,
                                      command_buf,
                                      literal_buf,
                                      table,
                                      table_size,
                                      &mut storage_ix,
                                      storage);
        if !(0i32 == 0) {
          return 0i32;
        }
      }
      *next_in = (*next_in).offset(block_size as (isize));
      *available_in = (*available_in).wrapping_sub(block_size);
      if inplace != 0 {
        let mut out_bytes: usize = storage_ix >> 3i32;
        0i32;
        0i32;
        *next_out = (*next_out).offset(out_bytes as (isize));
        *available_out = (*available_out).wrapping_sub(out_bytes);
        (*s).total_out_ = (*s).total_out_.wrapping_add(out_bytes);
        if !total_out.is_null() {
          *total_out = (*s).total_out_;
        }
      } else {
        let mut out_bytes: usize = storage_ix >> 3i32;
        (*s).next_out_ = storage;
        (*s).available_out_ = out_bytes;
      }
      (*s).last_byte_ = storage[((storage_ix >> 3i32) as (usize))];
      (*s).last_byte_bits_ = (storage_ix & 7u32 as (usize)) as (u8);
      if force_flush != 0 {
        (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_FLUSH_REQUESTED;
      }
      if is_last != 0 {
        (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_FINISHED;
      }
      {
        {
          continue;
        }
      }
    }
    {
      {
        break;
      }
    }
  }
  {
    BrotliFree(m, tmp_command_buf);
    tmp_command_buf = 0i32;
  }
  {
    BrotliFree(m, tmp_literal_buf);
    tmp_literal_buf = 0i32;
  }
  CheckFlushComplete(s);
  1i32
}

fn RemainingInputBlockSize<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) -> usize {
  let delta: usize = UnprocessedInputSize(s);
  let mut block_size: usize = InputBlockSize(s);
  if delta >= block_size {
    return 0usize;
  }
  block_size.wrapping_sub(delta)
}


pub fn BrotliEncoderCompressStream<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
                                   mut op: BrotliEncoderOperation,
                                   mut available_in: &mut [usize],
                                   mut next_in: &mut [&[u8]],
                                   mut available_out: &mut [usize],
                                   mut next_out: &mut [*mut u8],
                                   mut total_out: &mut usize)
                                   -> i32 {
  if EnsureInitialized(s) == 0 {
    return 0i32;
  }
  if (*s).remaining_metadata_bytes_ != !(0u32) {
    if *available_in != (*s).remaining_metadata_bytes_ as (usize) {
      return 0i32;
    }
    if op as (i32) != BrotliEncoderOperation::BROTLI_OPERATION_EMIT_METADATA as (i32) {
      return 0i32;
    }
  }
  if op as (i32) == BrotliEncoderOperation::BROTLI_OPERATION_EMIT_METADATA as (i32) {
    UpdateSizeHint(s, 0usize);
    return ProcessMetadata(s, available_in, next_in, available_out, next_out, total_out);
  }
  if (*s).stream_state_ as (i32) ==
     BrotliEncoderStreamState::BROTLI_STREAM_METADATA_HEAD as (i32) ||
     (*s).stream_state_ as (i32) == BrotliEncoderStreamState::BROTLI_STREAM_METADATA_BODY as (i32) {
    return 0i32;
  }
  if (*s).stream_state_ as (i32) !=
     BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING as (i32) && (*available_in != 0usize) {
    return 0i32;
  }
  if (*s).params.quality == 0i32 || (*s).params.quality == 1i32 {
    return BrotliEncoderCompressStreamFast(s,
                                           op,
                                           available_in,
                                           next_in,
                                           available_out,
                                           next_out,
                                           total_out);
  }
  while 1i32 != 0 {
    let mut remaining_block_size: usize = RemainingInputBlockSize(s);
    if remaining_block_size != 0usize && (*available_in != 0usize) {
      let mut copy_input_size: usize = brotli_min_size_t(remaining_block_size, *available_in);
      CopyInputToRingBuffer(s, copy_input_size, *next_in);
      *next_in = (*next_in).offset(copy_input_size as (isize));
      *available_in = (*available_in).wrapping_sub(copy_input_size);
      {
        {
          continue;
        }
      }
    }
    if InjectFlushOrPushOutput(s, available_out, next_out, total_out) != 0 {
      {
        continue;
      }
    }
    if (*s).available_out_ == 0usize &&
       ((*s).stream_state_ as (i32) ==
        BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING as (i32)) {
      if remaining_block_size == 0usize ||
         op as (i32) != BrotliEncoderOperation::BROTLI_OPERATION_PROCESS as (i32) {
        let mut is_last: i32 = if !!(*available_in == 0usize &&
                                     (op as (i32) ==
                                      BrotliEncoderOperation::BROTLI_OPERATION_FINISH as (i32))) {
          1i32
        } else {
          0i32
        };
        let mut force_flush: i32 =
          if !!(*available_in == 0usize &&
                (op as (i32) == BrotliEncoderOperation::BROTLI_OPERATION_FLUSH as (i32))) {
            1i32
          } else {
            0i32
          };
        let mut result: i32;
        UpdateSizeHint(s, *available_in);
        result = EncodeData(s,
                            is_last,
                            force_flush,
                            &mut (*s).available_out_,
                            &mut (*s).next_out_);
        if result == 0 {
          return 0i32;
        }
        if force_flush != 0 {
          (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_FLUSH_REQUESTED;
        }
        if is_last != 0 {
          (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_FINISHED;
        }
        {
          {
            continue;
          }
        }
      }
    }
    {
      {
        break;
      }
    }
  }
  CheckFlushComplete(s);
  1i32
}


pub fn BrotliEncoderIsFinished<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) -> i32 {
  if !!((*s).stream_state_ as (i32) == BrotliEncoderStreamState::BROTLI_STREAM_FINISHED as (i32) &&
        (BrotliEncoderHasMoreOutput(s) == 0)) {
    1i32
  } else {
    0i32
  }
}


pub fn BrotliEncoderHasMoreOutput<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) -> i32 {
  if !!((*s).available_out_ != 0usize) {
    1i32
  } else {
    0i32
  }
}


pub fn BrotliEncoderTakeOutput<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
                               mut size: &mut [usize])
                               -> *const u8 {
  let mut consumed_size: usize = (*s).available_out_;
  let mut result: *mut u8 = (*s).next_out_;
  if *size != 0 {
    consumed_size = brotli_min_size_t(*size, (*s).available_out_);
  }
  if consumed_size != 0 {
    (*s).next_out_ = (*s).next_out_[(consumed_size as (usize))..];
    (*s).available_out_ = (*s).available_out_.wrapping_sub(consumed_size);
    (*s).total_out_ = (*s).total_out_.wrapping_add(consumed_size);
    CheckFlushComplete(s);
    *size = consumed_size;
  } else {
    *size = 0usize;
    result = 0i32;
  }
  result
}


pub fn BrotliEncoderVersion() -> u32 {
  0x1000000u32
}


pub fn BrotliEncoderInputBlockSize<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) -> usize {
  InputBlockSize(s)
}


pub fn BrotliEncoderCopyInputToRingBuffer<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
                                          input_size: usize,
                                          mut input_buffer: &[u8]) {
  CopyInputToRingBuffer(s, input_size, input_buffer);
}


pub fn BrotliEncoderWriteData<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(mut s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
                              is_last: i32,
                              force_flush: i32,
                              mut out_size: &mut [usize],
                              mut output: &mut [*mut u8])
                              -> i32 {
  EncodeData(s, is_last, force_flush, out_size, output)
}
*/
