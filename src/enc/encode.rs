#![allow(dead_code)]
use super::hash_to_binary_tree::{InitializeH10, ZopfliNode};
use super::constants::{BROTLI_WINDOW_GAP, BROTLI_CONTEXT_LUT, BROTLI_CONTEXT,
                       BROTLI_NUM_HISTOGRAM_DISTANCE_SYMBOLS, BROTLI_MAX_NPOSTFIX, BROTLI_MAX_NDIRECT};
use super::backward_references::{BrotliCreateBackwardReferences, Struct1, UnionHasher,
                                 BrotliEncoderParams, BrotliEncoderMode, BrotliHasherParams, H2Sub,
                                 H3Sub, H4Sub, H5Sub, H6Sub, H54Sub, AdvHasher, BasicHasher, H9,
                                 H9_BUCKET_BITS, H9_BLOCK_SIZE, H9_BLOCK_BITS, H9_NUM_LAST_DISTANCES_TO_CHECK,
                                 AnyHasher, HowPrepared, StoreLookaheadThenStore};

use super::vectorization::Mem256f;
use super::interface;
use super::interface::StaticCommand;
use super::bit_cost::{BitsEntropy, ShannonEntropy};
#[allow(unused_imports)]
use super::block_split::BlockSplit;
#[allow(unused_imports)]
use super::brotli_bit_stream::{BrotliBuildAndStoreHuffmanTreeFast, BrotliStoreHuffmanTree,
                               BrotliStoreMetaBlock, BrotliStoreMetaBlockFast,
                               BrotliStoreMetaBlockTrivial, BrotliStoreUncompressedMetaBlock,
                               MetaBlockSplit, RecoderState};
                               
use enc::input_pair::InputReferenceMut;
use super::command::{Command, GetLengthCode, BrotliDistanceParams};
use super::compress_fragment::BrotliCompressFragmentFast;
use super::compress_fragment_two_pass::{BrotliCompressFragmentTwoPass, BrotliWriteBits};
#[allow(unused_imports)]
use super::entropy_encode::{BrotliConvertBitDepthsToSymbols, BrotliCreateHuffmanTree, HuffmanTree};
use super::cluster::{HistogramPair};
use super::metablock::{BrotliBuildMetaBlock, BrotliBuildMetaBlockGreedy, BrotliOptimizeHistograms, BrotliInitDistanceParams};
use super::static_dict::{BrotliGetDictionary, kNumDistanceCacheEntries};
use super::histogram::{ContextType, HistogramLiteral, HistogramCommand, HistogramDistance, CostAccessors};
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::utf8_util::BrotliIsMostlyUTF8;
use super::util::{brotli_min_size_t, Log2FloorNonZero};
use super::pdf::PDF;
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



static kCompressFragmentTwoPassBlockSize: usize = (1i32 << 17i32) as (usize);

static kMinUTF8Ratio: super::util::floatX = 0.75 as super::util::floatX;

#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum BrotliEncoderParameter {
  BROTLI_PARAM_MODE = 0,
  BROTLI_PARAM_QUALITY = 1,
  BROTLI_PARAM_LGWIN = 2,
  BROTLI_PARAM_LGBLOCK = 3,
  BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING = 4,
    BROTLI_PARAM_SIZE_HINT = 5,
    BROTLI_PARAM_LARGE_WINDOW = 6,
  BROTLI_PARAM_Q9_5 = 150,
  BROTLI_METABLOCK_CALLBACK = 151,
  BROTLI_PARAM_STRIDE_DETECTION_QUALITY = 152,
  BROTLI_PARAM_HIGH_ENTROPY_DETECTION_QUALITY = 153,
  BROTLI_PARAM_LITERAL_BYTE_SCORE = 154,
  BROTLI_PARAM_CDF_ADAPTATION_DETECTION = 155,
  BROTLI_PARAM_PRIOR_BITMASK_DETECTION = 156,
  BROTLI_PARAM_SPEED = 157,
  BROTLI_PARAM_SPEED_MAX = 158,
  BROTLI_PARAM_CM_SPEED = 159,
  BROTLI_PARAM_CM_SPEED_MAX = 160,
  BROTLI_PARAM_SPEED_LOW = 161,
  BROTLI_PARAM_SPEED_LOW_MAX = 162,
  BROTLI_PARAM_CM_SPEED_LOW = 164,
  BROTLI_PARAM_CM_SPEED_LOW_MAX = 165,
  BROTLI_PARAM_AVOID_DISTANCE_PREFIX_SEARCH = 166,
}

pub struct RingBuffer<AllocU8: alloc::Allocator<u8>> {
  pub size_: u32,
  pub mask_: u32,
  pub tail_size_: u32,
  pub total_size_: u32,
  pub cur_size_: u32,
  pub pos_: u32,
  pub data_mo: AllocU8::AllocatedMemory,
  pub buffer_index: usize,
}


#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum BrotliEncoderStreamState {
  BROTLI_STREAM_PROCESSING = 0,
  BROTLI_STREAM_FLUSH_REQUESTED = 1,
  BROTLI_STREAM_FINISHED = 2,
  BROTLI_STREAM_METADATA_HEAD = 3,
  BROTLI_STREAM_METADATA_BODY = 4,
}

enum NextOut {
    DynamicStorage(u32),
    TinyBuf(u32),
    None,
}
fn GetNextOutInternal<'a>(
    next_out :&NextOut,
    storage : &'a mut [u8],
    tiny_buf : &'a mut [u8;16],
) -> &'a mut[u8]{
    match next_out {
        &NextOut::DynamicStorage(offset) =>
            return &mut storage[offset as usize..],
        &NextOut::TinyBuf(offset) =>
            return &mut tiny_buf[offset as usize..],
        &NextOut::None => panic!("Next out: Null ptr deref"),
    }
}
macro_rules! GetNextOut {
    ($s : expr) => {
        GetNextOutInternal(&$s.next_out_,
                           $s.storage_.slice_mut(),
                           &mut $s.tiny_buf_)
    };
}
fn NextOutIncrement(next_out :&NextOut, inc : i32) -> NextOut{
    match next_out {
        &NextOut::DynamicStorage(offset) =>
            return NextOut::DynamicStorage((offset as i32 + inc) as u32),
        &NextOut::TinyBuf(offset) =>
            return NextOut::TinyBuf((offset as i32 + inc) as u32),
        &NextOut::None => panic!("Next out: Null ptr deref"),
    }
}
fn IsNextOutNull(next_out :&NextOut) -> bool {
    match next_out {
        &NextOut::DynamicStorage(_) =>
            false,
        &NextOut::TinyBuf(_) =>
            false,
        &NextOut::None => true,
    }
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
  pub input_pos_: u64,
  pub ringbuffer_: RingBuffer<AllocU8>,
  pub cmd_alloc_size_: usize,
  pub commands_: AllocCommand::AllocatedMemory, // not sure about this one
  pub num_commands_: usize,
  pub num_literals_: usize,
  pub last_insert_len_: usize,
  pub last_flush_pos_: u64,
  pub last_processed_pos_: u64,
  pub dist_cache_: [i32; 16],
  pub saved_dist_cache_: [i32; kNumDistanceCacheEntries],
  pub last_bytes_: u16,
  pub last_bytes_bits_: u8,
  pub prev_byte_: u8,
  pub prev_byte2_: u8,
  pub storage_size_: usize,
  pub storage_: AllocU8::AllocatedMemory,
  pub small_table_: [i32; 1024],
  pub large_table_: AllocI32::AllocatedMemory,
//  pub large_table_size_: usize, // <-- get this by doing large_table_.len()
  pub cmd_depths_: [u8; 128],
  pub cmd_bits_: [u16; 128],
  pub cmd_code_: [u8; 512],
  pub cmd_code_numbits_: usize,
  pub command_buf_: AllocU32::AllocatedMemory,
  pub literal_buf_: AllocU8::AllocatedMemory,
  next_out_: NextOut,
  pub available_out_: usize,
  pub total_out_: usize,
  pub tiny_buf_: [u8; 16],
  pub remaining_metadata_bytes_: u32,
  pub stream_state_: BrotliEncoderStreamState,
  pub is_last_block_emitted_: i32,
  pub is_initialized_: i32,
  pub literal_scratch_space: <HistogramLiteral as CostAccessors>::i32vec,
  pub command_scratch_space: <HistogramCommand as CostAccessors>::i32vec,
  pub distance_scratch_space: <HistogramDistance as CostAccessors>::i32vec,
  pub recoder_state: RecoderState,
}



pub fn BrotliEncoderSetParameter<AllocU8: alloc::Allocator<u8>,
                                 AllocU16: alloc::Allocator<u16>,
                                 AllocU32: alloc::Allocator<u32>,
                                 AllocI32: alloc::Allocator<i32>,
                                 AllocCommand: alloc::Allocator<Command>>
  (state: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
   p: BrotliEncoderParameter,
   value: u32)
   -> i32 {
  if (*state).is_initialized_ != 0 {
    return 0i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_MODE as (i32) {
    (*state).params.mode = match value {
      0 => BrotliEncoderMode::BROTLI_MODE_GENERIC,
      1 => BrotliEncoderMode::BROTLI_MODE_TEXT,
      2 => BrotliEncoderMode::BROTLI_MODE_FONT,
      3 => BrotliEncoderMode::BROTLI_FORCE_LSB_PRIOR,
      4 => BrotliEncoderMode::BROTLI_FORCE_MSB_PRIOR,
      5 => BrotliEncoderMode::BROTLI_FORCE_UTF8_PRIOR,
      6 => BrotliEncoderMode::BROTLI_FORCE_SIGNED_PRIOR,
      _ => BrotliEncoderMode::BROTLI_MODE_GENERIC,
    };
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_QUALITY as (i32) {
    (*state).params.quality = value as (i32);
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_STRIDE_DETECTION_QUALITY as (i32) {
    (*state).params.stride_detection_quality = value as (u8);
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_HIGH_ENTROPY_DETECTION_QUALITY as (i32) {
    (*state).params.high_entropy_detection_quality = value as (u8);
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_CDF_ADAPTATION_DETECTION as (i32) {
    (*state).params.cdf_adaptation_detection = value as (u8);
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_Q9_5 as (i32) {
    (*state).params.q9_5 = (value != 0);
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_PRIOR_BITMASK_DETECTION as (i32) {
    (*state).params.prior_bitmask_detection = value as u8;
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_SPEED as (i32) {
    (*state).params.literal_adaptation[1].0 = value as u16;
    if (*state).params.literal_adaptation[0] == (0,0) {
        (*state).params.literal_adaptation[0].0 = value as u16;
    }
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_SPEED_MAX as (i32) {
    (*state).params.literal_adaptation[1].1 = value as u16;
    if (*state).params.literal_adaptation[0].1 == 0 {
        (*state).params.literal_adaptation[0].1 = value as u16;
    }
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_CM_SPEED as (i32) {
    (*state).params.literal_adaptation[3].0 = value as u16;
    if (*state).params.literal_adaptation[2] == (0,0) {
        (*state).params.literal_adaptation[2].0 = value as u16;
    }
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_CM_SPEED_MAX as (i32) {
    (*state).params.literal_adaptation[3].1 = value as u16;
    if (*state).params.literal_adaptation[2].1 == 0 {
        (*state).params.literal_adaptation[2].1 = value as u16;
    }
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_SPEED_LOW as (i32) {
    (*state).params.literal_adaptation[0].0 = value as u16;
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_SPEED_LOW_MAX as (i32) {
    (*state).params.literal_adaptation[0].1 = value as u16;
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_CM_SPEED_LOW as (i32) {
    (*state).params.literal_adaptation[2].0 = value as u16;
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_CM_SPEED_LOW_MAX as (i32) {
    (*state).params.literal_adaptation[2].1 = value as u16;
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_LITERAL_BYTE_SCORE as (i32) {
    (*state).params.hasher.literal_byte_score = value as i32;
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_METABLOCK_CALLBACK as (i32) {
    (*state).params.log_meta_block = if value != 0 {true} else {false};
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
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_LARGE_WINDOW as (i32) {
    (*state).params.large_window = value != 0;
    return 1i32;
  }
  if p as (i32) == BrotliEncoderParameter::BROTLI_PARAM_AVOID_DISTANCE_PREFIX_SEARCH as (i32) {
    (*state).params.avoid_distance_prefix_search = value != 0;
    return 1i32;
  }
  0i32
}
/* "Large Window Brotli" */
pub const BROTLI_LARGE_MAX_DISTANCE_BITS: u32 = 62;
pub const BROTLI_LARGE_MIN_WBITS: u32 = 10;
pub const BROTLI_LARGE_MAX_WBITS: u32 = 30;

pub const BROTLI_MAX_DISTANCE_BITS:u32 = 24;
pub const BROTLI_MAX_WINDOW_BITS:usize = BROTLI_MAX_DISTANCE_BITS as usize;
pub const BROTLI_MAX_DISTANCE:usize = 0x3FFFFFC;
pub const BROTLI_MAX_ALLOWED_DISTANCE:usize = 0x7FFFFFC;
pub const BROTLI_NUM_DISTANCE_SHORT_CODES:u32 = 16;
pub fn BROTLI_DISTANCE_ALPHABET_SIZE(NPOSTFIX: u32, NDIRECT:u32, MAXNBITS: u32) -> u32 {
    BROTLI_NUM_DISTANCE_SHORT_CODES + (NDIRECT) +
        ((MAXNBITS) << ((NPOSTFIX) + 1))
}

//#define BROTLI_NUM_DISTANCE_SYMBOLS \
//    BROTLI_DISTANCE_ALPHABET_SIZE(  \
//        BROTLI_MAX_NDIRECT, BROTLI_MAX_NPOSTFIX, BROTLI_LARGE_MAX_DISTANCE_BITS)

pub const BROTLI_NUM_DISTANCE_SYMBOLS:usize = 1128;

pub fn BrotliEncoderInitParams() -> BrotliEncoderParams {
    return BrotliEncoderParams {
           dist: BrotliDistanceParams {
               distance_postfix_bits:0,
               num_direct_distance_codes:0,
               alphabet_size: BROTLI_DISTANCE_ALPHABET_SIZE(0, 0, BROTLI_MAX_DISTANCE_BITS),
               max_distance: BROTLI_MAX_DISTANCE,
           },
           mode: BrotliEncoderMode::BROTLI_MODE_GENERIC,
           log_meta_block: false,
           large_window:false,
           avoid_distance_prefix_search:false,
           quality: 11,
           q9_5: false,
           lgwin: 22i32,
           lgblock: 0i32,
           size_hint: 0usize,
           disable_literal_context_modeling: 0i32,
           stride_detection_quality: 0,
           high_entropy_detection_quality: 0,
           cdf_adaptation_detection: 0,
           prior_bitmask_detection: 0,
           literal_adaptation: [(0,0);4],
           hasher: BrotliHasherParams {
             type_: 6,
             block_bits: 9 - 1,
             bucket_bits: 15,
             hash_len: 5,
             num_last_distances_to_check: 16,
             literal_byte_score: 0,
           },
         };
}

fn ExtendLastCommand<AllocU8:alloc::Allocator<u8>,
                     AllocU16:alloc::Allocator<u16>,
                     AllocU32:alloc::Allocator<u32>,
                     AllocI32:alloc::Allocator<i32>,
                     AllocCommand:alloc::Allocator<Command>>(
    s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
    bytes: &mut u32,
    wrapped_last_processed_pos: &mut u32
) {
    let last_command = &mut s.commands_.slice_mut()[s.num_commands_ - 1];
   
    let mask = s.ringbuffer_.mask_;
    let max_backward_distance:u64 = (1u64 << s.params.lgwin) - BROTLI_WINDOW_GAP as u64;
    let last_copy_len = u64::from(last_command.copy_len_) & 0x1ffffff;
    let last_processed_pos:u64 = s.last_processed_pos_ - last_copy_len;
    let max_distance:u64 = if last_processed_pos < max_backward_distance {
        last_processed_pos
    } else {
        max_backward_distance
    };
    let cmd_dist:u64 = s.dist_cache_[0] as u64;
    let distance_code:u32 = super::command::CommandRestoreDistanceCode(last_command, &s.params.dist);
    if (distance_code < BROTLI_NUM_DISTANCE_SHORT_CODES ||
        distance_code as u64 - (BROTLI_NUM_DISTANCE_SHORT_CODES - 1) as u64 == cmd_dist) {
        if (cmd_dist <= max_distance) {
            while (*bytes != 0 &&
                   s.ringbuffer_.data_mo.slice()[s.ringbuffer_.buffer_index + (*wrapped_last_processed_pos as usize & mask as usize)] ==
                   s.ringbuffer_.data_mo.slice()[s.ringbuffer_.buffer_index + (((*wrapped_last_processed_pos as usize).wrapping_sub(cmd_dist as usize)) & mask as usize)]) {
                last_command.copy_len_+=1;
                (*bytes)-=1;
                (*wrapped_last_processed_pos)+=1;
            }
        }
        /* The copy length is at most the metablock size, and thus expressible. */
        GetLengthCode(last_command.insert_len_ as usize,
                      ((last_command.copy_len_ & 0x1FFFFFF) as i32 +
                       (last_command.copy_len_ >> 25) as i32) as usize,
                      ((last_command.dist_prefix_ & 0x3FF) == 0) as i32,
                      &mut last_command.cmd_prefix_);
    }
}


fn RingBufferInit<AllocU8: alloc::Allocator<u8>>() -> RingBuffer<AllocU8> {
  return RingBuffer {
           size_: 0,
           mask_: 0, // 0xff??
           tail_size_: 0,
           total_size_: 0,

           cur_size_: 0,
           pos_: 0,
           data_mo: AllocU8::AllocatedMemory::default(),
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
    input_pos_: 0,
    num_commands_: 0usize,
    num_literals_: 0usize,
    last_insert_len_: 0usize,
    last_flush_pos_: 0,
    last_processed_pos_: 0,
    prev_byte_: 0i32 as (u8),
    prev_byte2_: 0i32 as (u8),
    storage_size_: 0usize,
    storage_: AllocU8::AllocatedMemory::default(),
    hasher_: UnionHasher::<AllocU16, AllocU32>::default(),
    large_table_: AllocI32::AllocatedMemory::default(),
//    large_table_size_: 0usize,
    cmd_code_numbits_: 0usize,
    command_buf_: AllocU32::AllocatedMemory::default(),
    literal_buf_: AllocU8::AllocatedMemory::default(),
    next_out_: NextOut::None,
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
    last_bytes_: 0,
    last_bytes_bits_: 0,
    cmd_code_: [0; 512],
    m8: m8,
    m16: m16,
    mi32:mi32,
    m32: m32,
    mc: mc,
    remaining_metadata_bytes_: 0,
    small_table_: [0; 1024],
    tiny_buf_: [0; 16],
    literal_scratch_space: HistogramLiteral::make_nnz_storage(),
    command_scratch_space: HistogramCommand::make_nnz_storage(),
    distance_scratch_space: HistogramDistance::make_nnz_storage(),
    recoder_state: RecoderState::new(),
  }
}

fn RingBufferFree<AllocU8: alloc::Allocator<u8>>(m: &mut AllocU8,
                                                 rb: &mut RingBuffer<AllocU8>) {
  m.free_cell(core::mem::replace(&mut rb.data_mo, AllocU8::AllocatedMemory::default()));
}
fn DestroyHasher<AllocU16:alloc::Allocator<u16>, AllocU32:alloc::Allocator<u32>>(
m16: &mut AllocU16, m32:&mut AllocU32, handle: &mut UnionHasher<AllocU16, AllocU32>){
  handle.free(m16, m32);
}
/*
fn DestroyHasher<AllocU16:alloc::Allocator<u16>, AllocU32:alloc::Allocator<u32>>(
m16: &mut AllocU16, m32:&mut AllocU32, handle: &mut UnionHasher<AllocU16, AllocU32>){
  match handle {
    &mut UnionHasher::H2(ref mut hasher) => {
        m32.free_cell(core::mem::replace(&mut hasher.buckets_.buckets_, AllocU32::AllocatedMemory::default()));
    }
    &mut UnionHasher::H3(ref mut hasher) => {
        m32.free_cell(core::mem::replace(&mut hasher.buckets_.buckets_, AllocU32::AllocatedMemory::default()));
    }
    &mut UnionHasher::H4(ref mut hasher) => {
        m32.free_cell(core::mem::replace(&mut hasher.buckets_.buckets_, AllocU32::AllocatedMemory::default()));
    }
    &mut UnionHasher::H54(ref mut hasher) => {
        m32.free_cell(core::mem::replace(&mut hasher.buckets_.buckets_, AllocU32::AllocatedMemory::default()));
    }
    &mut UnionHasher::H5(ref mut hasher) => {
      m16.free_cell(core::mem::replace(&mut hasher.num, AllocU16::AllocatedMemory::default()));
      m32.free_cell(core::mem::replace(&mut hasher.buckets, AllocU32::AllocatedMemory::default()));
    }
    &mut UnionHasher::H6(ref mut hasher) => {
      m16.free_cell(core::mem::replace(&mut hasher.num, AllocU16::AllocatedMemory::default()));
      m32.free_cell(core::mem::replace(&mut hasher.buckets, AllocU32::AllocatedMemory::default()));
    }
    &mut UnionHasher::H9(ref mut hasher) => {
      m16.free_cell(core::mem::replace(&mut hasher.num_, AllocU16::AllocatedMemory::default()));
      m32.free_cell(core::mem::replace(&mut hasher.buckets_, AllocU32::AllocatedMemory::default()));
    }
    _ => {}
  }
  *handle = UnionHasher::<AllocU16, AllocU32>::default();
}
*/

fn BrotliEncoderCleanupState<AllocU8: alloc::Allocator<u8>,
                             AllocU16: alloc::Allocator<u16>,
                             AllocU32: alloc::Allocator<u32>,
                             AllocI32: alloc::Allocator<i32>,
                             AllocCommand: alloc::Allocator<Command>>
  (s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) {
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
  (s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) {
  BrotliEncoderCleanupState(s);
}

fn brotli_min_int(a: i32, b: i32) -> i32 {
  if a < b { a } else { b }
}

fn brotli_max_int(a: i32, b: i32) -> i32 {
  if a > b { a } else { b }
}

fn SanitizeParams(params: &mut BrotliEncoderParams) {
  (*params).quality = brotli_min_int(11i32, brotli_max_int(0i32, (*params).quality));
  if (*params).lgwin < 10i32 {
    (*params).lgwin = 10i32;
  } else if (*params).lgwin > 24i32 {
    (*params).lgwin = 24i32;
  }
}

fn ComputeLgBlock(params: &BrotliEncoderParams) -> i32 {
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

fn ComputeRbBits(params: &BrotliEncoderParams) -> i32 {
  1i32 + brotli_max_int((*params).lgwin, (*params).lgblock)
}

fn RingBufferSetup<AllocU8: alloc::Allocator<u8>>(params: &BrotliEncoderParams,
                                                  rb: &mut RingBuffer<AllocU8>) {
  let window_bits: i32 = ComputeRbBits(params);
  let tail_bits: i32 = (*params).lgblock;
  *(&mut (*rb).size_) = 1u32 << window_bits;
  *(&mut (*rb).mask_) = (1u32 << window_bits).wrapping_sub(1u32);
  *(&mut (*rb).tail_size_) = 1u32 << tail_bits;
  *(&mut (*rb).total_size_) = (*rb).size_.wrapping_add((*rb).tail_size_);
}

fn EncodeWindowBits(lgwin: i32, large_window: bool, last_bytes: &mut u16, last_bytes_bits: &mut u8) {
    if large_window {
        *last_bytes = (((lgwin & 0x3F) << 8) | 0x11) as u16;
        *last_bytes_bits = 14;
    } else {
        if lgwin == 16i32 {
            *last_bytes = 0i32 as (u16);
            *last_bytes_bits = 1i32 as (u8);
        } else if lgwin == 17i32 {
            *last_bytes = 1i32 as (u16);
            *last_bytes_bits = 7i32 as (u8);
        } else if lgwin > 17i32 {
            *last_bytes = (lgwin - 17i32 << 1i32 | 1i32) as (u16);
            *last_bytes_bits = 4i32 as (u8);
        } else {
            *last_bytes = (lgwin - 8i32 << 4i32 | 1i32) as (u16);
            *last_bytes_bits = 7i32 as (u8);
        }
    }
}

fn InitCommandPrefixCodes(cmd_depths: &mut [u8],
                          cmd_bits: &mut [u16],
                          cmd_code: &mut [u8],
                          cmd_code_numbits: &mut usize) {
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
  cmd_code[..kDefaultCommandCode.len()].clone_from_slice(&kDefaultCommandCode[..]);
  *cmd_code_numbits = kDefaultCommandCodeNumBits;
}

fn EnsureInitialized<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>
  (s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>)
   -> i32 {
  if (*s).is_initialized_ != 0 {
    return 1i32;
  }
  SanitizeParams(&mut (*s).params);
  (*s).params.lgblock = ComputeLgBlock(&mut (*s).params);
  ChooseDistanceParams(&mut s.params);
  (*s).remaining_metadata_bytes_ = !(0u32);
  RingBufferSetup(&mut (*s).params, &mut (*s).ringbuffer_);
  {
    let mut lgwin: i32 = (*s).params.lgwin;
    if (*s).params.quality == 0i32 || (*s).params.quality == 1i32 {
      lgwin = brotli_max_int(lgwin, 18i32);
    }
    EncodeWindowBits(lgwin, s.params.large_window, &mut (*s).last_bytes_, &mut (*s).last_bytes_bits_);
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

fn RingBufferInitBuffer<AllocU8: alloc::Allocator<u8>>(m: &mut AllocU8,
                                                       buflen: u32,
                                                       rb: &mut RingBuffer<AllocU8>) {
  static kSlackForEightByteHashingEverywhere: usize = 7usize;
  let mut new_data =
    m.alloc_cell(((2u32).wrapping_add(buflen) as (usize))
                   .wrapping_add(kSlackForEightByteHashingEverywhere));
  let mut i: usize;
  if (*rb).data_mo.slice().len() != 0 {
    let lim: usize = ((2u32).wrapping_add((*rb).cur_size_) as (usize))
      .wrapping_add(kSlackForEightByteHashingEverywhere);
    new_data.slice_mut()[..lim].clone_from_slice(&(*rb).data_mo.slice()[..lim]);
    m.free_cell(core::mem::replace(&mut (*rb).data_mo, AllocU8::AllocatedMemory::default()));
  }
  core::mem::replace(&mut (*rb).data_mo, new_data);
  (*rb).cur_size_ = buflen;
  (*rb).buffer_index = 2usize;
  (*rb).data_mo.slice_mut()[((*rb).buffer_index.wrapping_sub(2usize))] = 0;
  (*rb).data_mo.slice_mut()[((*rb).buffer_index.wrapping_sub(1usize))] = 0;
  i = 0usize;
  while i < kSlackForEightByteHashingEverywhere {
    {
      (*rb).data_mo.slice_mut()[((*rb)
         .buffer_index
         .wrapping_add((*rb).cur_size_ as (usize))
         .wrapping_add(i) as (usize))] = 0;
    }
    i = i.wrapping_add(1 as (usize));
  }
}


fn RingBufferWriteTail<AllocU8: alloc::Allocator<u8>>(bytes: &[u8],
                                                      n: usize,
                                                      rb: &mut RingBuffer<AllocU8>) {
  let masked_pos: usize = ((*rb).pos_ & (*rb).mask_) as (usize);
  if masked_pos < (*rb).tail_size_ as (usize) {
    let p: usize = ((*rb).size_ as (usize)).wrapping_add(masked_pos);
    let begin = ((*rb).buffer_index.wrapping_add(p) as (usize));
    let lim = brotli_min_size_t(n, ((*rb).tail_size_ as (usize)).wrapping_sub(masked_pos));
    (*rb).data_mo.slice_mut()[begin..(begin + lim)].clone_from_slice(&bytes[..lim]);
  }
}

fn RingBufferWrite<AllocU8: alloc::Allocator<u8>>(m: &mut AllocU8,
                                                  bytes: &[u8],
                                                  n: usize,
                                                  rb: &mut RingBuffer<AllocU8>) {
  if (*rb).pos_ == 0u32 && (n < (*rb).tail_size_ as (usize)) {
    (*rb).pos_ = n as (u32);
    RingBufferInitBuffer(m, (*rb).pos_, rb);
    (*rb).data_mo.slice_mut()[((*rb).buffer_index as (usize))..(((*rb).buffer_index as (usize)) + n)]
      .clone_from_slice(&bytes[..n]);
    return;
  }
  if (*rb).cur_size_ < (*rb).total_size_ {
    RingBufferInitBuffer(m, (*rb).total_size_, rb);
    if !(0i32 == 0) {
      return;
    }
    (*rb).data_mo.slice_mut()[((*rb)
       .buffer_index
       .wrapping_add((*rb).size_ as (usize))
       .wrapping_sub(2usize) as (usize))] = 0i32 as (u8);
    (*rb).data_mo.slice_mut()[((*rb)
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
      (*rb).data_mo.slice_mut()[start..(start + n)].clone_from_slice(&bytes[..n]);
    } else {
      {
        let start = ((*rb).buffer_index.wrapping_add(masked_pos) as (usize));
        let mid = brotli_min_size_t(n, ((*rb).total_size_ as (usize)).wrapping_sub(masked_pos));
        (*rb).data_mo.slice_mut()[start..(start + mid)].clone_from_slice(&bytes[..mid]);
      }
      let xstart = ((*rb).buffer_index.wrapping_add(0usize) as (usize));
      let size = n.wrapping_sub(((*rb).size_ as (usize)).wrapping_sub(masked_pos));
      let bytes_start = (((*rb).size_ as (usize)).wrapping_sub(masked_pos) as (usize));
      (*rb).data_mo.slice_mut()[xstart..(xstart + size)].clone_from_slice(&bytes[bytes_start..
                                                                         (bytes_start +
                                                                          size)]);
    }
  }
  let data_2 = (*rb).data_mo.slice()[((*rb)
     .buffer_index
     .wrapping_add((*rb).size_ as (usize))
     .wrapping_sub(2usize) as (usize))];
  (*rb).data_mo.slice_mut()[((*rb).buffer_index.wrapping_sub(2usize) as (usize))] = data_2;
  let data_1 = (*rb).data_mo.slice()[((*rb)
     .buffer_index
     .wrapping_add((*rb).size_ as (usize))
     .wrapping_sub(1usize) as (usize))];
  (*rb).data_mo.slice_mut()[((*rb).buffer_index.wrapping_sub(1usize) as (usize))] = data_1;
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
  (s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
   input_size: usize,
   input_buffer: &[u8]) {
  if EnsureInitialized(s) == 0 {
    return;
  }
  RingBufferWrite(&mut s.m8, input_buffer, input_size, &mut s.ringbuffer_);
  if !(0i32 == 0) {
    return;
  }
  (*s).input_pos_ = (*s).input_pos_.wrapping_add(input_size as u64);
  if (s.ringbuffer_).pos_ <= (s.ringbuffer_).mask_ {
    let start = ((s.ringbuffer_).buffer_index.wrapping_add((s.ringbuffer_).pos_ as (usize)) as
                 (usize));
    for item in (s.ringbuffer_).data_mo.slice_mut()[start..(start + 7)].iter_mut() {
      *item = 0;
    }
  }
}


fn ChooseHasher(params: &mut BrotliEncoderParams) {
  let hparams = &mut params.hasher;
  if (*params).quality >= 10 && !params.q9_5{
      (*hparams).type_ = 10;
  } else if (*params).quality == 10 { // we are using quality 10 as a proxy for "9.5"
      (*hparams).type_ = 9;
      (*hparams).num_last_distances_to_check = H9_NUM_LAST_DISTANCES_TO_CHECK as i32;
      (*hparams).block_bits = H9_BLOCK_BITS as i32;
      (*hparams).bucket_bits = H9_BUCKET_BITS as i32;
      (*hparams).hash_len = 4;
  } else if (*params).quality == 4 && ((*params).size_hint >= (1i32 << 20i32) as (usize)) {
    (*hparams).type_ = 54i32;
  } else if (*params).quality < 5 {
    (*hparams).type_ = (*params).quality;
  } else if (*params).lgwin <= 16 {
    (*hparams).type_ = if (*params).quality < 7 {
      40i32
    } else if (*params).quality < 9 {
      41i32
    } else {
      42i32
    };
  } else if (*params).size_hint >= (1i32 << 20i32) as (usize) && ((*params).lgwin >= 19i32) {
    (*hparams).type_ = 6i32;
    (*hparams).block_bits = core::cmp::min((*params).quality - 1, 9);
    (*hparams).bucket_bits = 15i32;
    (*hparams).hash_len = 5i32;
    (*hparams).num_last_distances_to_check = if (*params).quality < 7 {
      4i32
    } else if (*params).quality < 9 {
      10i32
    } else {
      16i32
    };
  } else {
    (*hparams).type_ = 5i32;
    (*hparams).block_bits = core::cmp::min((*params).quality - 1, 9);
    (*hparams).bucket_bits = if (*params).quality < 7 {
      14i32
    } else {
      15i32
    };
    (*hparams).num_last_distances_to_check = if (*params).quality < 7 {
      4i32
    } else if (*params).quality < 9 {
      10i32
    } else {
      16i32
    };
  }
}

fn InitializeH2<AllocU32:alloc::Allocator<u32>>(m32: &mut AllocU32, params : &BrotliEncoderParams) -> BasicHasher<H2Sub<AllocU32>> {
    BasicHasher {
        GetHasherCommon:Struct1{
            params:params.hasher,
            is_prepared_:1,
            dict_num_lookups:0,
            dict_num_matches:0,
        },
        buckets_:H2Sub{buckets_:m32.alloc_cell(65537 + 8)},
        h9_opts: super::backward_references::H9Opts::new(&params.hasher),
    }
}
fn InitializeH3<AllocU32:alloc::Allocator<u32>>(m32: &mut AllocU32, params : &BrotliEncoderParams) -> BasicHasher<H3Sub<AllocU32>> {
    BasicHasher {
        GetHasherCommon:Struct1{
            params:params.hasher,
            is_prepared_:1,
            dict_num_lookups:0,
            dict_num_matches:0,
        },
        buckets_:H3Sub{buckets_:m32.alloc_cell(65538 + 8)},
        h9_opts: super::backward_references::H9Opts::new(&params.hasher),
    }
}
fn InitializeH4<AllocU32:alloc::Allocator<u32>>(m32: &mut AllocU32, params : &BrotliEncoderParams) -> BasicHasher<H4Sub<AllocU32>> {
    BasicHasher {
        GetHasherCommon:Struct1{
            params:params.hasher,
            is_prepared_:1,
            dict_num_lookups:0,
            dict_num_matches:0,
        },
        buckets_:H4Sub{buckets_:m32.alloc_cell(131072 + 8)},
        h9_opts: super::backward_references::H9Opts::new(&params.hasher),
    }
}
fn InitializeH54<AllocU32:alloc::Allocator<u32>>(m32: &mut AllocU32, params : &BrotliEncoderParams) -> BasicHasher<H54Sub<AllocU32>> {
    BasicHasher {
        GetHasherCommon:Struct1{
            params:params.hasher,
            is_prepared_:1,
            dict_num_lookups:0,
            dict_num_matches:0,
        },
        buckets_:H54Sub{buckets_:m32.alloc_cell(1048580 + 8)},
        h9_opts: super::backward_references::H9Opts::new(&params.hasher),
    }
}

fn InitializeH9<AllocU16:alloc::Allocator<u16>,
                AllocU32:alloc::Allocator<u32>>(m16: &mut AllocU16,
                                                m32: &mut AllocU32,
                                                params : &BrotliEncoderParams) -> H9<AllocU16, AllocU32> {
    H9 {
        dict_search_stats_:Struct1{
            params:params.hasher,
            is_prepared_:1,
            dict_num_lookups:0,
            dict_num_matches:0,
        },
        num_:m16.alloc_cell(1<<H9_BUCKET_BITS),
        buckets_:m32.alloc_cell(H9_BLOCK_SIZE<<H9_BUCKET_BITS),
        h9_opts: super::backward_references::H9Opts::new(&params.hasher),
    }
}

fn InitializeH5<AllocU16: alloc::Allocator<u16>, AllocU32: alloc::Allocator<u32>>
  (m16: &mut AllocU16,
   m32: &mut AllocU32,
   params: &BrotliEncoderParams)
   -> AdvHasher<H5Sub, AllocU16, AllocU32> {
  let block_size = 1u64 << params.hasher.block_bits;
  let bucket_size = 1u64 << params.hasher.bucket_bits;
  let buckets = m32.alloc_cell((bucket_size * block_size) as usize);
  let num = m16.alloc_cell(bucket_size as usize);
  AdvHasher {
    buckets: buckets,
    h9_opts: super::backward_references::H9Opts::new(&params.hasher),
    num: num,
    GetHasherCommon: Struct1 {
      params: params.hasher,
      is_prepared_: 1,
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
  (m16: &mut AllocU16,
   m32: &mut AllocU32,
   params: &BrotliEncoderParams)
   -> AdvHasher<H6Sub, AllocU16, AllocU32> {
  let block_size = 1u64 << params.hasher.block_bits;
  let bucket_size = 1u64 << params.hasher.bucket_bits;
  let buckets = m32.alloc_cell((bucket_size * block_size) as usize);
  let num = m16.alloc_cell(bucket_size as usize);
  AdvHasher {
    buckets: buckets,
    num: num,
    h9_opts: super::backward_references::H9Opts::new(&params.hasher),
    GetHasherCommon: Struct1 {
      params: params.hasher,
      is_prepared_: 1,
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
  (m16: &mut AllocU16,
   m32: &mut AllocU32,
   params: &BrotliEncoderParams)
   -> UnionHasher<AllocU16, AllocU32> {
  let hasher_type: i32 = params.hasher.type_;
  if hasher_type == 2i32 {
    return UnionHasher::H2(InitializeH2(m32, params));
  }
  if hasher_type == 3i32 {
    return UnionHasher::H3(InitializeH3(m32, params));
  }
  if hasher_type == 4i32 {
    return UnionHasher::H4(InitializeH4(m32, params));
  }
  if hasher_type == 5i32 {
    return UnionHasher::H5(InitializeH5(m16, m32, params));
  }
  if hasher_type == 6i32 {
    return UnionHasher::H6(InitializeH6(m16, m32, params));
  }
  if hasher_type == 9i32 {
    return UnionHasher::H9(InitializeH9(m16, m32, params));
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
    return UnionHasher::H54(InitializeH54(m32, params));
  }
    if hasher_type == 10i32 {
      return UnionHasher::H10(InitializeH10(m32, false, params, 0));
  }
  // since we don't support all of these, fall back to something sane
  return UnionHasher::H6(InitializeH6(m16, m32, params));
      
//  return UnionHasher::Uninit;
}
fn HasherReset<AllocU16:alloc::Allocator<u16>,
AllocU32:alloc::Allocator<u32>>(t: &mut UnionHasher<AllocU16, AllocU32>){
  match t {
    &mut UnionHasher::Uninit => {}
    _ => (t.GetHasherCommon()).is_prepared_ = 0i32,
  };
}
fn GetHasherCommon<AllocU16: alloc::Allocator<u16>, AllocU32: alloc::Allocator<u32>>
  (t: &mut UnionHasher<AllocU16, AllocU32>)
   -> &mut Struct1 {
  t.GetHasherCommon()
}

fn HasherSetup<AllocU16:alloc::Allocator<u16>,
               AllocU32:alloc::Allocator<u32>>(m16: &mut AllocU16,
                                               m32: &mut AllocU32,
                                               handle: &mut UnionHasher<AllocU16, AllocU32>,
                                               params: &mut BrotliEncoderParams,
                                               data: &[u8],
                                               position: usize,
                                               input_size: usize,
                                               is_last: i32){
  let one_shot: i32 = (position == 0usize && (is_last != 0)) as (i32);
  let is_uninit = match (handle) {
    &mut UnionHasher::Uninit => true,
    _ => false,
  };
  if is_uninit {
    //let alloc_size: usize;
    ChooseHasher(&mut (*params));
    //alloc_size = HasherSize(params, one_shot, input_size);
    //xself = BrotliAllocate(m, alloc_size.wrapping_mul(::std::mem::size_of::<u8>()))
    *handle = BrotliMakeHasher(m16, m32, params);
    handle.GetHasherCommon().params = (*params).hasher;
    HasherReset(handle); // this sets everything to zero, unlike in C
    handle.GetHasherCommon().is_prepared_ = 1;
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
  (m16: &mut AllocU16,
   m32: &mut AllocU32,
   handle: &mut UnionHasher<AllocU16, AllocU32>,
   params: &mut BrotliEncoderParams,
   size: usize,
   dict: &[u8]) {
  HasherSetup(m16, m32, handle, params, dict, 0usize, size, 0i32);
  match handle {
    &mut UnionHasher::H2(ref mut hasher) => StoreLookaheadThenStore(hasher, size, dict),
    &mut UnionHasher::H3(ref mut hasher) => StoreLookaheadThenStore(hasher, size, dict),
    &mut UnionHasher::H4(ref mut hasher) => StoreLookaheadThenStore(hasher, size, dict),
    &mut UnionHasher::H5(ref mut hasher) => StoreLookaheadThenStore(hasher, size, dict),
    &mut UnionHasher::H6(ref mut hasher) => StoreLookaheadThenStore(hasher, size, dict),
    &mut UnionHasher::H9(ref mut hasher) => StoreLookaheadThenStore(hasher, size, dict),
    &mut UnionHasher::H54(ref mut hasher) => StoreLookaheadThenStore(hasher, size, dict),
    &mut UnionHasher::H10(ref mut hasher) => StoreLookaheadThenStore(hasher, size, dict),
    &mut UnionHasher::Uninit => panic!("Uninitialized"),
  }
}

pub fn BrotliEncoderSetCustomDictionary<AllocU8: alloc::Allocator<u8>,
                                        AllocU16: alloc::Allocator<u16>,
                                        AllocU32: alloc::Allocator<u32>,
                                        AllocI32: alloc::Allocator<i32>,
                                        AllocCommand: alloc::Allocator<Command>>
  (s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
   size: usize,
   mut dict: &[u8]) {
  let max_dict_size: usize = (1usize << (*s).params.lgwin).wrapping_sub(16usize);
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
  (*s).last_flush_pos_ = dict_size as u64;
  (*s).last_processed_pos_ = dict_size as u64;
  if dict_size > 0 {
    (*s).prev_byte_ = dict[(dict_size.wrapping_sub(1usize) as (usize))];
  }
  if dict_size > 1usize {
    (*s).prev_byte2_ = dict[(dict_size.wrapping_sub(2usize) as (usize))];
  }
  let m16 = &mut s.m16;
  let m32 = &mut s.m32;
  HasherPrependCustomDictionary(m16,
                                m32,
                                &mut (*s).hasher_,
                                &mut (*s).params,
                                dict_size,
                                dict);
}


pub fn BrotliEncoderMaxCompressedSize(input_size: usize) -> usize {
  let num_large_blocks: usize = input_size >> 14i32;
  let tail: usize = input_size.wrapping_sub(num_large_blocks << 24i32);
  let tail_overhead: usize = (if tail > (1i32 << 20i32) as (usize) {
                                    4i32
                                  } else {
                                    3i32
                                  }) as (usize);
  let overhead: usize = (2usize)
    .wrapping_add((4usize).wrapping_mul(num_large_blocks))
    .wrapping_add(tail_overhead)
    .wrapping_add(1usize);
  let result: usize = input_size.wrapping_add(overhead);
  if input_size == 0usize {
    return 1usize;
  }
  if result < input_size { 0usize } else { result }
}

fn InitOrStitchToPreviousBlock<AllocU16: alloc::Allocator<u16>, AllocU32: alloc::Allocator<u32>>
  (m16: &mut AllocU16,
   m32: &mut AllocU32,
   handle: &mut UnionHasher<AllocU16, AllocU32>,
   data: &[u8],
   mask: usize,
   params: &mut BrotliEncoderParams,
   position: usize,
   input_size: usize,
   is_last: i32) {
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

pub fn InitInsertCommand(xself: &mut Command, insertlen: usize) {
  (*xself).insert_len_ = insertlen as (u32);
  (*xself).copy_len_ = (4i32 << 25i32) as (u32);
  (*xself).dist_extra_ = 0u32;
  (*xself).dist_prefix_ = (1u16 << 10) | BROTLI_NUM_DISTANCE_SHORT_CODES as (u16);
  GetLengthCode(insertlen, 4usize, 0i32, &mut (*xself).cmd_prefix_);
}



fn ShouldCompress(data: &[u8],
                  mask: usize,
                  last_flush_pos: u64,
                  bytes: usize,
                  num_literals: usize,
                  num_commands: usize)
                  -> i32 {
  if num_commands < (bytes >> 8i32).wrapping_add(2usize) {
    if num_literals as (super::util::floatX) > 0.99 as super::util::floatX * bytes as (super::util::floatX) {
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
      static kMinEntropy: super::util::floatX = 7.92 as super::util::floatX;
      let bit_cost_threshold: super::util::floatX = bytes as (super::util::floatX) * kMinEntropy / kSampleRate as (super::util::floatX);
      let t: usize = bytes.wrapping_add(kSampleRate as (usize))
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

/* Chooses the literal context mode for a metablock */
fn ChooseContextMode(params: &BrotliEncoderParams,
    data: &[u8], pos: usize, mask: usize,
    length: usize) -> ContextType{
  /* We only do the computation for the option of something else than
    CONTEXT_UTF8 for the highest qualities */
  match params.mode {
      BrotliEncoderMode::BROTLI_FORCE_LSB_PRIOR => return ContextType::CONTEXT_LSB6,
      BrotliEncoderMode::BROTLI_FORCE_MSB_PRIOR => return ContextType::CONTEXT_MSB6,
      BrotliEncoderMode::BROTLI_FORCE_UTF8_PRIOR => return ContextType::CONTEXT_UTF8,
      BrotliEncoderMode::BROTLI_FORCE_SIGNED_PRIOR => return ContextType::CONTEXT_SIGNED,
      _ => {},
  }
  if (params.quality >= 10 &&
      BrotliIsMostlyUTF8(data, pos, mask, length, kMinUTF8Ratio) == 0) {
    return ContextType::CONTEXT_SIGNED;
  }
  return ContextType::CONTEXT_UTF8;
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
  let saved_dist_cache: [i32; 4] = [4i32, 11i32, 15i32, 16i32];
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
                                       storage,
                                       false);
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
                           lit_scratch_space,
                           cmd_scratch_space,
                           dst_scratch_space,
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
                                         storage,
                                         true);
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

fn MakeUncompressedStream(input: &[u8], input_size: usize, output: &mut [u8]) -> usize {
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
    let chunk_size: u32;
    let bits: u32;
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
pub fn BrotliEncoderCompress<AllocU8: alloc::Allocator<u8>,
                             AllocU16: alloc::Allocator<u16>,
                             AllocU32: alloc::Allocator<u32>,
                             AllocI32: alloc::Allocator<i32>,
                             AllocU64: alloc::Allocator<u64>,
                             AllocF64: alloc::Allocator<super::util::floatX>,
                             AllocFV: alloc::Allocator<Mem256f>,
                             AllocPDF: alloc::Allocator<PDF>,
                             AllocStaticCommand: alloc::Allocator<StaticCommand>,
                             AllocHL: alloc::Allocator<HistogramLiteral>,
                             AllocHC: alloc::Allocator<HistogramCommand>,
                             AllocHD: alloc::Allocator<HistogramDistance>,
                             AllocHP: alloc::Allocator<HistogramPair>,
                             AllocCT: alloc::Allocator<ContextType>,
                             AllocCommand: alloc::Allocator<Command>,
                             AllocHT:alloc::Allocator<HuffmanTree>,
                             AllocZN:alloc::Allocator<ZopfliNode>,
                             MetablockCallback: FnMut(&mut interface::PredictionModeContextMap<InputReferenceMut>,
                                                      &mut [interface::StaticCommand],
                                                      interface::InputPair)>(
    empty_m8: AllocU8,
    empty_m16: AllocU16,
    empty_m32: AllocU32,
    empty_mi32: AllocI32,
    empty_mc: AllocCommand,
    m8: &mut AllocU8,
    m16: &mut AllocU16,
    m32: &mut AllocU32,
    mi32: &mut AllocI32,
    mc: &mut AllocCommand,
    m64: &mut AllocU64,
    mf64: &mut AllocF64,
    mfv: &mut AllocFV,
    mpdf: &mut AllocPDF,
    msc: &mut AllocStaticCommand,
    mhl: &mut AllocHL,
    mhc: &mut AllocHC,
    mhd: &mut AllocHD,
    mhp: &mut AllocHP,
    mct: &mut AllocCT,
    mht: &mut AllocHT,
    mzn: &mut AllocZN,

    quality: i32,
    lgwin: i32,
    mode: BrotliEncoderMode,
    input_size: usize,
    input_buffer: &[u8],
    encoded_size: &mut usize,
    encoded_buffer: &mut [u8],
    metablock_callback: &mut MetablockCallback)
            -> i32 {
  let out_size: usize = *encoded_size;
  let input_start = input_buffer;
  let output_start = encoded_buffer;
  let max_out_size: usize = BrotliEncoderMaxCompressedSize(input_size);
  if out_size == 0usize {
    return 0i32;
  }
  if input_size == 0usize {
    *encoded_size = 1;
    output_start[0] = 6;
    return 1i32;
  }
  let mut is_fallback: i32 = 0i32;
    if quality == 10i32 {
    //let lg_win: i32 = brotli_min_int(BROTLI_LARGE_MAX_WINDOW_BITS, brotli_max_int(16i32, lgwin));
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
    
    let mut s_orig = BrotliEncoderCreateInstance(core::mem::replace(m8, empty_m8),
                                                 core::mem::replace(m16, empty_m16),
                                                 core::mem::replace(mi32, empty_mi32),
                                                 core::mem::replace(m32, empty_m32),
                                                 core::mem::replace(mc, empty_mc));
    let mut result: i32;
    {
      let s = &mut s_orig;
      let mut available_in: usize = input_size;
      let mut next_in_array: &[u8] = input_buffer;
      let mut next_in_offset: usize = 0;  
      let mut available_out: usize = *encoded_size;
      let mut next_out_array: &mut [u8] = output_start;
      let mut next_out_offset: usize = 0;
      let mut total_out = Some(0usize);
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
      if lgwin > BROTLI_MAX_WINDOW_BITS as i32 {
          BrotliEncoderSetParameter(s, BrotliEncoderParameter::BROTLI_PARAM_LARGE_WINDOW, 1);
      }
      result = BrotliEncoderCompressStream(s,
                                             m64,
                                             mf64, mfv, mpdf, msc, mhl, mhc, mhd, mhp, mct, mht, mzn,
                                             BrotliEncoderOperation::BROTLI_OPERATION_FINISH,
                                             &mut available_in,
                                             &mut next_in_array,
                                             &mut next_in_offset,  
                                             &mut available_out,
                                             &mut next_out_array,
                                             &mut next_out_offset,
                                             &mut total_out,
                                             metablock_callback);
      if BrotliEncoderIsFinished(s) == 0 {
        result = 0i32;
      }
       
      *encoded_size = total_out.unwrap();
      BrotliEncoderDestroyInstance(s);
    }
    core::mem::replace(m8, s_orig.m8);
    core::mem::replace(m16, s_orig.m16);
    core::mem::replace(m32, s_orig.m32);
    core::mem::replace(mi32, s_orig.mi32);
    core::mem::replace(mc, s_orig.mc); // unreplace these things so the empty ones are the ones thrown out
    if result == 0 || max_out_size != 0 && (*encoded_size > max_out_size) {
        is_fallback = 1i32;
    } else {
      return 1i32;
    }
  }
  assert!(is_fallback != 0);
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
                        AllocCommand: alloc::Allocator<Command>>(s: &mut BrotliEncoderStateStruct<AllocU8,
                                                                                                  AllocU16,
                                                                                                  AllocU32,
                                                                                                  AllocI32, 
                                                                                                  AllocCommand>) {
  let mut seal: u32 = (*s).last_bytes_ as (u32);
  let mut seal_bits: usize = (*s).last_bytes_bits_ as (usize);
  let destination: &mut [u8];
  (*s).last_bytes_ = 0;
  (*s).last_bytes_bits_ = 0;
  seal = seal | 0x6u32 << seal_bits;
  seal_bits = seal_bits.wrapping_add(6usize);
  if !IsNextOutNull(&(*s).next_out_) {
    destination = &mut GetNextOut!(*s)[((*s).available_out_ as (usize))..];
  } else {
    destination = &mut (*s).tiny_buf_[..];
    (*s).next_out_ = NextOut::TinyBuf(0);
  }
  destination[(0usize)] = seal as (u8);
  if seal_bits > 8usize {
    destination[(1usize)] = (seal >> 8i32) as (u8);
  }
  if seal_bits > 16usize {
    destination[(2usize)] = (seal >> 16i32) as (u8);
  }
  (*s).available_out_ = (*s).available_out_.wrapping_add(seal_bits.wrapping_add(7usize) >> 3i32);
}
fn InjectFlushOrPushOutput<AllocU8: alloc::Allocator<u8>,
                           AllocU16: alloc::Allocator<u16>,
                           AllocU32: alloc::Allocator<u32>,
                           AllocI32: alloc::Allocator<i32>,
                           AllocCommand: alloc::Allocator<Command>>(
    s: &mut BrotliEncoderStateStruct<AllocU8,
                                         AllocU16,
                                         AllocU32,
                                         AllocI32, 
                                         AllocCommand>,
    available_out: &mut usize,
    next_out_array: &mut [u8],
    next_out_offset: &mut usize,
    total_out: &mut Option<usize>)
            -> i32 {
  if (*s).stream_state_ as (i32) ==
     BrotliEncoderStreamState::BROTLI_STREAM_FLUSH_REQUESTED as (i32) &&
     ((*s).last_bytes_bits_ as (i32) != 0i32) {
    InjectBytePaddingBlock(s);
    return 1i32;
  }
  if (*s).available_out_ != 0usize && (*available_out != 0usize) {
    let copy_output_size: usize = brotli_min_size_t((*s).available_out_, *available_out);
    (*next_out_array)[(*next_out_offset)..(*next_out_offset + copy_output_size)].clone_from_slice(&GetNextOut!(s)[..copy_output_size]);
    //memcpy(*next_out, (*s).next_out_, copy_output_size);
    *next_out_offset = (*next_out_offset).wrapping_add(copy_output_size);
    *available_out = (*available_out).wrapping_sub(copy_output_size);
    (*s).next_out_ = NextOutIncrement(&(*s).next_out_, (copy_output_size as (i32)));
    (*s).available_out_ = (*s).available_out_.wrapping_sub(copy_output_size);
    (*s).total_out_ = (*s).total_out_.wrapping_add(copy_output_size);
    if let &mut Some(ref mut total_out_inner) = total_out {
      *total_out_inner = (*s).total_out_;
    }
    return 1i32;
  }
  0i32
}

fn UnprocessedInputSize<AllocU8: alloc::Allocator<u8>,
                        AllocU16: alloc::Allocator<u16>,
                        AllocU32: alloc::Allocator<u32>,
                        AllocI32: alloc::Allocator<i32>,
                        AllocCommand: alloc::Allocator<Command>>(
                            s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) -> u64 {
  (*s).input_pos_.wrapping_sub((*s).last_processed_pos_)
}

fn UpdateSizeHint<AllocU8: alloc::Allocator<u8>,
                  AllocU16: alloc::Allocator<u16>,
                  AllocU32: alloc::Allocator<u32>,
                  AllocI32: alloc::Allocator<i32>,
                  AllocCommand: alloc::Allocator<Command>>(s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
                        available_in: usize) {
  if (*s).params.size_hint == 0usize {
    let delta: u64 = UnprocessedInputSize(s);
    let tail: u64 = available_in as u64;
    let limit: u32 = 1u32 << 30i32;
    let total: u32;
    if delta >= u64::from(limit) || tail >= u64::from(limit) ||
       delta.wrapping_add(tail) >= u64::from(limit) {
      total = limit;
    } else {
      total = delta.wrapping_add(tail) as (u32);
    }
    (*s).params.size_hint = total as (usize);
  }
}


fn WrapPosition(position: u64) -> u32 {
  let mut result: u32 = position as (u32);
  let gb: u64 = position >> 30i32;
  if gb > 2 {
    result = result & (1u32 << 30i32).wrapping_sub(1u32) |
             ((gb.wrapping_sub(1) & 1) as (u32)).wrapping_add(1u32) << 30i32;
  }
  result
}

fn InputBlockSize<AllocU8: alloc::Allocator<u8>,
                  AllocU16: alloc::Allocator<u16>,
                  AllocU32: alloc::Allocator<u32>,
                  AllocI32: alloc::Allocator<i32>,
                  AllocCommand: alloc::Allocator<Command>>(s: &mut BrotliEncoderStateStruct<AllocU8,
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
                    AllocCommand: alloc::Allocator<Command>>(s: &mut BrotliEncoderStateStruct<AllocU8,
                                                                                                  AllocU16,
                                                                                                  AllocU32,
                                                                                                  AllocI32, 
                                                                                                  AllocCommand>,
                                                                    size: usize) {
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

fn HashTableSize(max_table_size: usize, input_size: usize) -> usize {
  let mut htsize: usize = 256usize;
  while htsize < max_table_size && (htsize < input_size) {
    htsize = htsize << 1i32;
  }
  htsize
}

macro_rules! GetHashTable {
    ($s : expr, $quality: expr, $input_size : expr, $table_size : expr) => {
        GetHashTableInternal(&mut $s.mi32, &mut $s.small_table_, &mut $s.large_table_,
                             $quality, $input_size, $table_size)
    };
}
fn GetHashTableInternal<'a, AllocI32: alloc::Allocator<i32>>(mi32: &mut AllocI32,
                                                             small_table_: &'a mut [i32; 1024],
                                                             large_table_: &'a mut AllocI32::AllocatedMemory,
                                                             quality: i32,
                                                             input_size: usize,
                                                             table_size: &mut usize)
                                                                 -> &'a mut [i32] {
  let max_table_size: usize = MaxHashTableSize(quality);
  let mut htsize: usize = HashTableSize(max_table_size, input_size);
  let table: &mut [i32];
  if quality == 0i32 {
    if htsize & 0xaaaaausize == 0usize {
      htsize = htsize << 1i32;
    }
  }
  if htsize <= small_table_.len() {
    table = &mut small_table_[..];
  } else {
    if htsize > large_table_.slice().len() {
      //(*s).large_table_size_ = htsize;
      {
          mi32.free_cell(core::mem::replace(large_table_,
                                            AllocI32::AllocatedMemory::default()));
      }
      *large_table_ = mi32.alloc_cell(htsize);
    }
    table = large_table_.slice_mut();
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
                     AllocCommand: alloc::Allocator<Command>>(s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) -> i32 {
  let wrapped_last_processed_pos: u32 = WrapPosition((*s).last_processed_pos_);
  let wrapped_input_pos: u32 = WrapPosition((*s).input_pos_);
  (*s).last_processed_pos_ = (*s).input_pos_;
  if !!(wrapped_input_pos < wrapped_last_processed_pos) {
    1i32
  } else {
    0i32
  }
}

fn MaxMetablockSize(params: &BrotliEncoderParams) -> usize {
  let bits: i32 = brotli_min_int(ComputeRbBits(params), 24i32);
  1usize << bits
}



fn ChooseContextMap(quality: i32,
                    bigram_histo: &mut [u32],
                    num_literal_contexts: &mut usize,
                    literal_context_map: &mut &[u32]) {
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
  let total: usize;
  let mut i: usize;
  let mut dummy: usize = 0;
  let mut entropy: [super::util::floatX; 4] = [0.0 as super::util::floatX;4];
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
  entropy[3usize] = 0i32 as (super::util::floatX);
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
  entropy[0usize] = 1.0 as super::util::floatX / total as (super::util::floatX);
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
    entropy[3usize] = entropy[1usize] * 10i32 as (super::util::floatX);
  }
  if entropy[1usize] - entropy[2usize] < 0.2 as super::util::floatX && (entropy[1usize] - entropy[3usize] < 0.2 as super::util::floatX) {
    *num_literal_contexts = 1usize;
  } else if entropy[2usize] - entropy[3usize] < 0.02 as super::util::floatX {
    *num_literal_contexts = 2usize;
    *literal_context_map = &kStaticContextMapSimpleUTF8[..];
  } else {
    *num_literal_contexts = 3usize;
    *literal_context_map = &kStaticContextMapContinuation[..];
  }
}

static kStaticContextMapComplexUTF8: [u32; 64] = [
    11, 11, 12, 12, /* 0 special */
    0, 0, 0, 0, /* 4 lf */
    1, 1, 9, 9, /* 8 space */
    2, 2, 2, 2, /* !, first after space/lf and after something else. */
    1, 1, 1, 1, /* " */
    8, 3, 3, 3, /* % */
    1, 1, 1, 1, /* ({[ */
    2, 2, 2, 2, /* }]) */
    8, 4, 4, 4, /* :; */
    8, 7, 4, 4, /* . */
    8, 0, 0, 0, /* > */
    3, 3, 3, 3, /* [0..9] */
    5, 5, 10, 5, /* [A-Z] */
    5, 5, 10, 5,
    6, 6, 6, 6, /* [a-z] */
    6, 6, 6, 6,
  ];
/* Decide if we want to use a more complex static context map containing 13
   context values, based on the entropy reduction of histograms over the
   first 5 bits of literals. */
fn ShouldUseComplexStaticContextMap(input: &[u8],
    mut start_pos: usize, length : usize, mask : usize, quality: i32,
    size_hint: usize,
    num_literal_contexts: &mut usize, literal_context_map: &mut &[u32]) -> bool {
  let _ = quality;
  //BROTLI_UNUSED(quality);
  /* Try the more complex static context map only for long data. */
  if (size_hint < (1 << 20)) {
    return false;
  } else {
    let end_pos = start_pos + length;
    /* To make entropy calculations faster and to fit on the stack, we collect
       histograms over the 5 most significant bits of literals. One histogram
       without context and 13 additional histograms for each context value. */
    let mut combined_histo:[u32; 32] = [0;32];
    let mut context_histo:[[u32;32]; 13] = [[0;32];13];
    let mut total = 0u32;
    let mut entropy = [0.0 as super::util::floatX;3];
    let mut dummy = 0usize;
    let utf8_lut = BROTLI_CONTEXT_LUT(ContextType::CONTEXT_UTF8);
    while start_pos + 64 <= end_pos {
      let stride_end_pos = start_pos + 64;
      let mut prev2 = input[start_pos & mask];
      let mut prev1 = input[(start_pos + 1) & mask];

      /* To make the analysis of the data faster we only examine 64 byte long
         strides at every 4kB intervals. */
      for pos in start_pos + 2..stride_end_pos {
        let literal = input[pos & mask];
        let context = kStaticContextMapComplexUTF8[
            BROTLI_CONTEXT(prev1, prev2, utf8_lut) as usize] as u8;
        total += 1;
        combined_histo[(literal >> 3) as usize] += 1;
        context_histo[context as usize][(literal >> 3) as usize] += 1;
        prev2 = prev1;
        prev1 = literal;
      }
      start_pos += 4096;
    }
    entropy[1] = ShannonEntropy(&combined_histo[..], 32, &mut dummy);
    entropy[2] = 0.0 as super::util::floatX;
    for i in 0..13 {
      assert!(i < 13);
      entropy[2] += ShannonEntropy(&context_histo[i][..], 32, &mut dummy);
    }
    entropy[0] = (1.0 as super::util::floatX) / (total as super::util::floatX);
    entropy[1] *= entropy[0];
    entropy[2] *= entropy[0];
    /* The triggering heuristics below were tuned by compressing the individual
       files of the silesia corpus. If we skip this kind of context modeling
       for not very well compressible input (i.e. entropy using context modeling
       is 60% of maximal entropy) or if expected savings by symbol are less
       than 0.2 bits, then in every case when it triggers, the final compression
       ratio is improved. Note however that this heuristics might be too strict
       for some cases and could be tuned further. */
    if (entropy[2] > 3.0 || entropy[1] - entropy[2] < 0.2) {
      return false;
    } else {
      *num_literal_contexts = 13;
      *literal_context_map = &kStaticContextMapComplexUTF8;
      return true;
    }
  }
}

fn DecideOverLiteralContextModeling(input: &[u8],
                                    mut start_pos: usize,
                                    length: usize,
                                    mask: usize,
                                    quality: i32,
                                    size_hint: usize,
                                    num_literal_contexts: &mut usize,
                                    literal_context_map: &mut &[u32]) {
    
  if quality < 5i32 || length < 64usize {
  } else if ShouldUseComplexStaticContextMap(input, start_pos, length, mask, quality, size_hint,
     num_literal_contexts, literal_context_map) {
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
    ChooseContextMap(quality,
                     &mut bigram_prefix_histo[..],
                     num_literal_contexts,
                     literal_context_map);
  }
}
fn WriteMetaBlockInternal<AllocU8: alloc::Allocator<u8>,
                          AllocU16: alloc::Allocator<u16>,
                          AllocU32: alloc::Allocator<u32>,
                          AllocF64: alloc::Allocator<super::util::floatX>,
                          AllocFV: alloc::Allocator<Mem256f>,
                          AllocPDF: alloc::Allocator<PDF>,
                          AllocStaticCommand: alloc::Allocator<StaticCommand>,
                          AllocHL: alloc::Allocator<HistogramLiteral>,
                          AllocHC: alloc::Allocator<HistogramCommand>,
                          AllocHD: alloc::Allocator<HistogramDistance>,
                          AllocHP: alloc::Allocator<HistogramPair>,
                          AllocCT: alloc::Allocator<ContextType>,
                          AllocHT: alloc::Allocator<HuffmanTree>,
                          Cb>
            (m8: &mut AllocU8,
             m16: &mut AllocU16,
             m32: &mut AllocU32,
             mf64: &mut AllocF64,
             mfv: &mut AllocFV,
             mpdf: &mut AllocPDF,
             mc: &mut AllocStaticCommand,
             mhl: &mut AllocHL,
             mhc: &mut AllocHC,
             mhd: &mut AllocHD,
             mhp: &mut AllocHP,
             mct: &mut AllocCT,
             mht: &mut AllocHT,
             data: &[u8],
             mask: usize,
             last_flush_pos: u64,
             bytes: usize,
             is_last: i32,
             literal_context_mode: ContextType,
             params: &BrotliEncoderParams,
             lit_scratch_space: &mut <HistogramLiteral as CostAccessors>::i32vec,
             cmd_scratch_space: &mut <HistogramCommand as CostAccessors>::i32vec,
             dst_scratch_space: &mut <HistogramDistance as CostAccessors>::i32vec,
             prev_byte: u8,
             prev_byte2: u8,
             num_literals: usize,
             num_commands: usize,
             commands: &mut [Command],
             saved_dist_cache: &[i32;kNumDistanceCacheEntries],
             dist_cache: &mut [i32;16],
             recoder_state: &mut RecoderState,
             storage_ix: &mut usize,
             storage: &mut [u8],
            cb: &mut Cb) where Cb: FnMut(&mut interface::PredictionModeContextMap<InputReferenceMut>,
                                         &mut [interface::StaticCommand],
                                         interface::InputPair) {

  let wrapped_last_flush_pos: u32 = WrapPosition(last_flush_pos);
  let last_bytes: u16;
  let last_bytes_bits: u8;
  let literal_context_lut = BROTLI_CONTEXT_LUT(literal_context_mode);
  let mut block_params = params.clone();
  if bytes == 0usize {
    BrotliWriteBits(2usize, 3, storage_ix, storage);
    *storage_ix = (*storage_ix).wrapping_add(7u32 as (usize)) & !7u32 as (usize);
    return;
  }
  if ShouldCompress(data,
                    mask,
                    last_flush_pos,
                    bytes,
                    num_literals,
                    num_commands) == 0 {
    dist_cache[..4].clone_from_slice(&saved_dist_cache[..4]);
    BrotliStoreUncompressedMetaBlock(m8,
                                     m16,
                                     m32,
                                     mf64,
                                     mfv,
                                     mpdf,
                                     mc,
                                     is_last,
                                     data,
                                     wrapped_last_flush_pos as (usize),
                                     mask,
                                     params,
                                     bytes,
                                     recoder_state,
                                     storage_ix,
                                     storage,
                                     false,
                                     cb);
    return;
  }
  last_bytes = ((storage[1] as u16) << 8) | storage[0] as u16;
  last_bytes_bits = *storage_ix as u8;
  /*if params.dist.num_direct_distance_codes != 0 ||
                    params.dist.distance_postfix_bits != 0 {
    RecomputeDistancePrefixes(commands,
                              num_commands,
                              params.dist.num_direct_distance_codes,
                              params.dist.distance_postfix_bits);
  }*/ // why was this removed??
  if (*params).quality <= 2i32 {
    BrotliStoreMetaBlockFast(mht,
                             m8,
                             m16,
                             m32,
                             mf64,
                             mfv,
                             mpdf,
                             mc,
                             data,
                             wrapped_last_flush_pos as (usize),
                             bytes,
                             mask,
                             is_last,
                             params,
                             saved_dist_cache,
                             commands,
                             num_commands,
                             recoder_state,
                             storage_ix,
                             storage,
                             cb);
    if !(0i32 == 0) {
      return;
    }
  } else if (*params).quality < 4i32 {
    BrotliStoreMetaBlockTrivial(m8, m16, m32, mf64, mfv, mpdf, mc,
                                data,
                                wrapped_last_flush_pos as (usize),
                                bytes,
                                mask,
                                is_last,
                                params,
                                saved_dist_cache,
                                commands,
                                num_commands,
                                recoder_state,
                                storage_ix,
                                storage,
                                cb);
    if !(0i32 == 0) {
      return;
    }
  } else {
    //let mut literal_context_mode: ContextType = ContextType::CONTEXT_UTF8;
    
    let mut mb = MetaBlockSplit::<AllocU8, AllocU32, AllocHL, AllocHC, AllocHD>::new();
    if (*params).quality < 10i32 {
      let mut num_literal_contexts: usize = 1usize;
      let mut literal_context_map: &[u32] = &[];
      if (*params).disable_literal_context_modeling == 0 {
        DecideOverLiteralContextModeling(data,
                                         wrapped_last_flush_pos as (usize),
                                         bytes,
                                         mask,
                                         (*params).quality,
                                         (*params).size_hint,
                                         &mut num_literal_contexts,
                                         &mut literal_context_map);
      }
      BrotliBuildMetaBlockGreedy(m8, m32, mhl, mhc, mhd,
                                 data,
                                 wrapped_last_flush_pos as (usize),
                                 mask,
                                 prev_byte,
                                 prev_byte2,
                                 literal_context_mode,
                                 literal_context_lut,
                                 num_literal_contexts,
                                 literal_context_map,
                                 commands,
                                 num_commands,
                                 &mut mb);
    } else {
      BrotliBuildMetaBlock(m8, m16, m32, mf64, mfv, mpdf, mhl, mhc, mhd, mhp, mct,
                           data,
                           wrapped_last_flush_pos as (usize),
                           mask,
                           &mut block_params,
                           prev_byte,
                           prev_byte2,
                           commands,
                           num_commands,
                           literal_context_mode,
                           lit_scratch_space,
                           cmd_scratch_space,
                           dst_scratch_space,
                           &mut mb);
    }
    if (*params).quality >= 4i32 {
        let mut num_effective_dist_codes = block_params.dist.alphabet_size;
        if num_effective_dist_codes > BROTLI_NUM_HISTOGRAM_DISTANCE_SYMBOLS as u32 {
            num_effective_dist_codes = BROTLI_NUM_HISTOGRAM_DISTANCE_SYMBOLS as u32;
        }
        BrotliOptimizeHistograms(num_effective_dist_codes as usize,
                               &mut mb);
    }
    BrotliStoreMetaBlock(m8, m16, m32, mf64, mfv, mpdf, mc, mht,
                         data,
                         wrapped_last_flush_pos as (usize),
                         bytes,
                         mask,
                         prev_byte,
                         prev_byte2,
                         is_last,
                         &block_params,
                         literal_context_mode,
                         saved_dist_cache,
                         commands,
                         num_commands,
                         &mut mb,
                         recoder_state,
                         storage_ix,
                         storage,
                         cb);
    mb.destroy(m8, m32, mhl, mhc, mhd);
  }
  if bytes.wrapping_add(4usize) < *storage_ix >> 3i32 {
      dist_cache[..4].clone_from_slice(&saved_dist_cache[..4]);
      //memcpy(dist_cache,
      //     saved_dist_cache,
      //     (4usize).wrapping_mul(::std::mem::size_of::<i32>()));
      storage[0] = last_bytes as u8;
      storage[1] = (last_bytes >> 8) as u8;
      *storage_ix = last_bytes_bits as (usize);
      BrotliStoreUncompressedMetaBlock(m8,
                                       m16,
                                       m32,
                                       mf64,
                                       mfv,
                                       mpdf,
                                       mc,
                                       is_last,
                                       data,
                                       wrapped_last_flush_pos as (usize),
                                       mask,
                                       params,
                                       bytes,
                                       recoder_state,
                                       storage_ix,
                                       storage,
                                       true,
                                       cb);
  }
}

fn ChooseDistanceParams(params: &mut BrotliEncoderParams) {
    let mut num_direct_distance_codes = 0u32;
    let mut distance_postfix_bits = 0u32;

    if params.quality >= 4 {
        let ndirect_msb;
        if params.mode == BrotliEncoderMode::BROTLI_MODE_FONT {
            distance_postfix_bits = 1;
            num_direct_distance_codes = 12;
        } else {
            distance_postfix_bits = params.dist.distance_postfix_bits;
            num_direct_distance_codes = params.dist.num_direct_distance_codes;
        }
        ndirect_msb = (num_direct_distance_codes >> distance_postfix_bits) & 0x0f;
        if distance_postfix_bits > BROTLI_MAX_NPOSTFIX as u32 || num_direct_distance_codes > BROTLI_MAX_NDIRECT as u32 || (ndirect_msb << distance_postfix_bits) != num_direct_distance_codes {
            distance_postfix_bits = 0;
            num_direct_distance_codes = 0;
        }
    }
    BrotliInitDistanceParams(params, distance_postfix_bits, num_direct_distance_codes);
    /*(
    if (params.large_window) {
        max_distance = BROTLI_MAX_ALLOWED_DISTANCE;
        if (num_direct_distance_codes != 0 || distance_postfix_bits != 0) {
            max_distance = (3 << 29) - 4;
        }
        alphabet_size = BROTLI_DISTANCE_ALPHABET_SIZE(
            num_direct_distance_codes, distance_postfix_bits,
            BROTLI_LARGE_MAX_DISTANCE_BITS);
    } else {
        alphabet_size = BROTLI_DISTANCE_ALPHABET_SIZE(
            num_direct_distance_codes, distance_postfix_bits,
            BROTLI_MAX_DISTANCE_BITS);

    }

    params.dist.num_direct_distance_codes = num_direct_distance_codes;
    params.dist.distance_postfix_bits = distance_postfix_bits;
    params.dist.alphabet_size = alphabet_size;
    params.dist.max_distance = max_distance;*/
}
        

fn EncodeData<AllocU8: alloc::Allocator<u8>,
              AllocU16: alloc::Allocator<u16>,
              AllocU32: alloc::Allocator<u32>,
              AllocI32: alloc::Allocator<i32>,
              AllocU64: alloc::Allocator<u64>,
              AllocF64: alloc::Allocator<super::util::floatX>,
              AllocFV: alloc::Allocator<Mem256f>,
              AllocPDF: alloc::Allocator<PDF>,
              AllocStaticCommand: alloc::Allocator<StaticCommand>,
              AllocHL: alloc::Allocator<HistogramLiteral>,
              AllocHC: alloc::Allocator<HistogramCommand>,
              AllocHD: alloc::Allocator<HistogramDistance>,
              AllocHP: alloc::Allocator<HistogramPair>,
              AllocCT: alloc::Allocator<ContextType>,
              AllocCommand: alloc::Allocator<Command>,
              AllocHT:alloc::Allocator<HuffmanTree>,
              AllocZN: alloc::Allocator<ZopfliNode>,
              MetablockCallback>(
    s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
    m64: &mut AllocU64,
    mf64: &mut AllocF64,
    mfv: &mut AllocFV,
    mpdf: &mut AllocPDF,
    mc: &mut AllocStaticCommand,
    mhl: &mut AllocHL,
    mhc: &mut AllocHC,
    mhd: &mut AllocHD,
    mhp: &mut AllocHP,
    mct: &mut AllocCT,
    mht: &mut AllocHT,
    mzn: &mut AllocZN,
    is_last: i32,
    force_flush: i32,
    out_size: &mut usize,
    callback: &mut MetablockCallback
//              mut output: &'a mut &'a mut [u8]
) -> i32 where MetablockCallback: FnMut(&mut interface::PredictionModeContextMap<InputReferenceMut>,
                                        &mut [interface::StaticCommand],
                                        interface::InputPair){
  let delta: u64 = UnprocessedInputSize(s);
  let mut bytes: u32 = delta as (u32);
  let mut wrapped_last_processed_pos: u32 = WrapPosition((*s).last_processed_pos_);
  let mask: u32;
  let dictionary = BrotliGetDictionary();
  if EnsureInitialized(s) == 0 {
    return 0i32;
  }
  mask = (*s).ringbuffer_.mask_;
  if (*s).is_last_block_emitted_ != 0 {
    return 0i32;
  }
  if is_last != 0 {
    (*s).is_last_block_emitted_ = 1i32;
  }
  if delta > InputBlockSize(s) as u64 {
    return 0i32;
  }
  if (*s).params.quality == 1i32 && (*s).command_buf_.slice().len() == 0 {
    let new_buf = (*s).m32.alloc_cell(kCompressFragmentTwoPassBlockSize);
    (*s).command_buf_ = new_buf;
    let new_buf8 = (*s).m8.alloc_cell(kCompressFragmentTwoPassBlockSize);
    (*s).literal_buf_ = new_buf8;
  }
  if (*s).params.quality == 0i32 || (*s).params.quality == 1i32 {
    let mut storage_ix: usize = (*s).last_bytes_bits_ as (usize);
    let mut table_size: usize = 0;
    {
    let table: &mut [i32];
    if delta == 0 && (is_last == 0) {
      *out_size = 0usize;
      return 1i32;
    }
    GetBrotliStorage(s,
                     (2u32).wrapping_mul(bytes).wrapping_add(502u32) as (usize));
    let data = &mut (*s).ringbuffer_.data_mo.slice_mut ()[((*s).ringbuffer_.buffer_index as (usize))..];
      
    (*s).storage_.slice_mut()[0] = (*s).last_bytes_ as u8;
    (*s).storage_.slice_mut()[1] = ((*s).last_bytes_ >> 8) as u8;
    table = GetHashTable!(s, (*s).params.quality, bytes as (usize), &mut table_size);
    if (*s).params.quality == 0i32 {
      BrotliCompressFragmentFast(mht,
                                 &mut data[((wrapped_last_processed_pos & mask) as (usize))..],
                                 bytes as (usize),
                                 is_last,
                                 table,
                                 table_size,
                                 &mut (*s).cmd_depths_[..],
                                 &mut (*s).cmd_bits_[..],
                                 &mut (*s).cmd_code_numbits_,
                                 &mut (*s).cmd_code_[..],
                                 &mut storage_ix,
                                 (*s).storage_.slice_mut());
    } else {
      BrotliCompressFragmentTwoPass(mht,
                                    &mut data[((wrapped_last_processed_pos & mask) as (usize))..],
                                    bytes as (usize),
                                    is_last,
                                    (*s).command_buf_.slice_mut(),
                                    (*s).literal_buf_.slice_mut(),
                                    table,
                                    table_size,
                                    &mut storage_ix,
                                    (*s).storage_.slice_mut());
    }
    (*s).last_bytes_ = (*s).storage_.slice()[((storage_ix >> 3i32) as (usize))] as u16 | ((
        (*s).storage_.slice()[((storage_ix >> 3i32) as (usize)) + 1] as u16) << 8);
    (*s).last_bytes_bits_ = (storage_ix & 7u32 as (usize)) as (u8);
    }
    UpdateLastProcessedPos(s);
    // *output = &mut (*s).storage_.slice_mut();
    (*s).next_out_ = NextOut::DynamicStorage(0); // this always returns that
    *out_size = storage_ix >> 3i32;
    return 1i32;
  }
  {
    let mut newsize: usize =
      (*s).num_commands_.wrapping_add(bytes.wrapping_div(2u32) as (usize)).wrapping_add(1usize);
    if newsize > (*s).cmd_alloc_size_ {
      newsize = newsize.wrapping_add(bytes.wrapping_div(4u32).wrapping_add(16u32) as (usize));
      (*s).cmd_alloc_size_ = newsize;
      let mut new_commands = s.mc.alloc_cell(newsize);
      if (*s).commands_.slice().len() != 0 {
        new_commands.slice_mut()[..(*s).num_commands_].clone_from_slice(&(*s).commands_.slice()[..(*s).num_commands_]);
        s.mc.free_cell(core::mem::replace(&mut (*s).commands_, AllocCommand::AllocatedMemory::default()));
      }
      (*s).commands_ = new_commands;
    }
  }
  InitOrStitchToPreviousBlock(&mut (*s).m16, &mut (*s).m32,
                              &mut (*s).hasher_,
                              &mut (*s).ringbuffer_.data_mo.slice_mut()[((*s).ringbuffer_.buffer_index as (usize))..],
                              mask as (usize),
                              &mut (*s).params,
                              wrapped_last_processed_pos as (usize),
                              bytes as (usize),
                              is_last);
  let literal_context_mode = ChooseContextMode(
      &s.params, (*s).ringbuffer_.data_mo.slice(), WrapPosition(s.last_flush_pos_) as usize,
      mask as usize, (s.input_pos_.wrapping_sub(s.last_flush_pos_)) as usize);
  if s.num_commands_ != 0 && s.last_insert_len_ == 0 {
      ExtendLastCommand(s, &mut bytes, &mut wrapped_last_processed_pos);
  }
  if false { // we are remapping 10 as quality=9.5 since Zopfli doesn't seem to offer much benefits here
    panic!(r####"
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
                                         &mut (*s).num_literals_);"####);
  } else if false && (*s).params.quality == 11i32 {
    panic!(r####"BrotliCreateHqZopfliBackwardReferences(m,
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
                                           &mut (*s).num_literals_);"####);
  } else {
    BrotliCreateBackwardReferences(&mut (*s).m32, m64, mf64, mzn, &dictionary,
                                   bytes as (usize),
                                   wrapped_last_processed_pos as (usize),
                                   &mut (*s).ringbuffer_.data_mo.slice_mut()[((*s).ringbuffer_.buffer_index as usize)..],
                                   mask as (usize),
                                   &mut (*s).params,
                                   &mut (*s).hasher_,
                                   &mut (*s).dist_cache_,
                                   &mut (*s).last_insert_len_,
                                   &mut (*s).commands_.slice_mut()[((*s).num_commands_ as (usize))..],
                                   &mut (*s).num_commands_,
                                   &mut (*s).num_literals_);
  }
  {
    let max_length: usize = MaxMetablockSize(&mut (*s).params);
    let max_literals: usize = max_length.wrapping_div(8usize);
    let max_commands: usize = max_length.wrapping_div(8usize);
    let processed_bytes: usize = (*s).input_pos_.wrapping_sub((*s).last_flush_pos_) as usize;
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
        HasherReset(&mut (*s).hasher_);
      }
      *out_size = 0usize;
      return 1i32;
    }
  }
  if (*s).last_insert_len_ > 0usize {
    InitInsertCommand(&mut (*s).commands_.slice_mut()[({
                               let _old = (*s).num_commands_;
                               (*s).num_commands_ = (*s).num_commands_.wrapping_add(1 as (usize));
                               _old
                             } as (usize))],
                      (*s).last_insert_len_);
    (*s).num_literals_ = (*s).num_literals_.wrapping_add((*s).last_insert_len_);
    (*s).last_insert_len_ = 0usize;
  }
  if is_last == 0 && ((*s).input_pos_ == (*s).last_flush_pos_) {
    *out_size = 0usize;
    return 1i32;
  }
  {
    let metablock_size: u32 = (*s).input_pos_.wrapping_sub((*s).last_flush_pos_) as (u32);
    GetBrotliStorage(s,
                     (2u32).wrapping_mul(metablock_size).wrapping_add(503) as (usize));
    let mut storage_ix: usize = (*s).last_bytes_bits_ as (usize);
    (*s).storage_.slice_mut()[(0usize)] = (*s).last_bytes_ as u8;
    (*s).storage_.slice_mut()[(1usize)] = ((*s).last_bytes_ >> 8) as u8;
    WriteMetaBlockInternal(&mut (*s).m8, &mut (*s).m16, &mut (*s).m32, mf64, mfv, mpdf, mc, mhl, mhc, mhd, mhp, mct, mht,
                           &mut (*s).ringbuffer_.data_mo.slice_mut()[((*s).ringbuffer_.buffer_index as usize)..],
                           mask as (usize),
                           (*s).last_flush_pos_,
                           metablock_size as (usize),
                           is_last,
                           literal_context_mode,
                           &mut (*s).params,
                           &mut (*s).literal_scratch_space,
                           &mut (*s).command_scratch_space,
                           &mut (*s).distance_scratch_space,
                           (*s).prev_byte_,
                           (*s).prev_byte2_,
                           (*s).num_literals_,
                           (*s).num_commands_,
                           (*s).commands_.slice_mut(),
                           &mut (*s).saved_dist_cache_,
                           &mut (*s).dist_cache_,
                           &mut (*s).recoder_state,
                           &mut storage_ix,
                           (*s).storage_.slice_mut(),
                           callback);

    (*s).last_bytes_ = (*s).storage_.slice()[((storage_ix >> 3i32) as (usize))] as u16 | (
          ((*s).storage_.slice()[1 + ((storage_ix >> 3i32) as (usize))] as u16)<<8);
    (*s).last_bytes_bits_ = (storage_ix & 7u32 as (usize)) as (u8);
    (*s).last_flush_pos_ = (*s).input_pos_;
    if UpdateLastProcessedPos(s) != 0 {
      HasherReset(&mut (*s).hasher_);
    }
    let data = &(*s).ringbuffer_.data_mo.slice()[(*s).ringbuffer_.buffer_index as usize..];
    if (*s).last_flush_pos_ > 0 {
      (*s).prev_byte_ = data[((((*s).last_flush_pos_ as (u32)).wrapping_sub(1u32) & mask) as
       (usize))];
    }
    if (*s).last_flush_pos_ > 1 {
      (*s).prev_byte2_ = data[(((*s).last_flush_pos_.wrapping_sub(2) as (u32) & mask) as
       (usize))];
    }
    (*s).num_commands_ = 0usize;
    (*s).num_literals_ = 0usize;
    (*s).saved_dist_cache_.clone_from_slice(&(*s).dist_cache_.split_at(4).0);
    // *output = &mut storage[(0usize)];
    (*s).next_out_ = NextOut::DynamicStorage(0); // this always returns that
    *out_size = storage_ix >> 3i32;
    1i32
  }
}

fn WriteMetadataHeader<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>)
                       -> usize {
  let block_size = (*s).remaining_metadata_bytes_ as (usize);
  let header = GetNextOut!(*s);
  let mut storage_ix: usize;
  storage_ix = (*s).last_bytes_bits_ as (usize);
  header[(0usize)] = (*s).last_bytes_ as u8;
  header[(1usize)] = ((*s).last_bytes_ >> 8) as u8;
  (*s).last_bytes_ = 0;
  (*s).last_bytes_bits_ = 0;
  BrotliWriteBits(1usize, 0, &mut storage_ix, header);
  BrotliWriteBits(2usize, 3, &mut storage_ix, header);
  BrotliWriteBits(1usize, 0, &mut storage_ix, header);
  if block_size == 0usize {
    BrotliWriteBits(2usize, 0, &mut storage_ix, header);
  } else {
    let nbits: u32 = if block_size == 1usize {
      0u32
    } else {
      Log2FloorNonZero((block_size as (u32)).wrapping_sub(1u32) as (u64)).wrapping_add(1u32)
    };
    let nbytes: u32 = nbits.wrapping_add(7u32).wrapping_div(8u32);
    BrotliWriteBits(2usize, nbytes as (u64), &mut storage_ix, header);
    BrotliWriteBits((8u32).wrapping_mul(nbytes) as (usize),
                    block_size.wrapping_sub(1usize) as u64,
                    &mut storage_ix,
                    header);
  }
  storage_ix.wrapping_add(7u32 as (usize)) >> 3i32
}

fn brotli_min_uint32_t(a: u32, b: u32) -> u32 {
  if a < b { a } else { b }
}
fn ProcessMetadata<AllocU8: alloc::Allocator<u8>,
              AllocU16: alloc::Allocator<u16>,
              AllocU32: alloc::Allocator<u32>,
                   AllocI32: alloc::Allocator<i32>,
                   AllocU64: alloc::Allocator<u64>,
              AllocF64: alloc::Allocator<super::util::floatX>,
              AllocFV: alloc::Allocator<Mem256f>,
              AllocPDF: alloc::Allocator<PDF>,
              AllocStaticCommand: alloc::Allocator<StaticCommand>,
              AllocHL: alloc::Allocator<HistogramLiteral>,
              AllocHC: alloc::Allocator<HistogramCommand>,
              AllocHD: alloc::Allocator<HistogramDistance>,
              AllocHP: alloc::Allocator<HistogramPair>,
              AllocCT: alloc::Allocator<ContextType>,
              AllocCommand: alloc::Allocator<Command>,
                   AllocHT:alloc::Allocator<HuffmanTree>,
                   AllocZN:alloc::Allocator<ZopfliNode>,
                   MetaBlockCallback:FnMut(&mut interface::PredictionModeContextMap<InputReferenceMut>,
                                           &mut [interface::StaticCommand],
                                           interface::InputPair)>(
    s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
    m64: &mut AllocU64,
    mf64: &mut AllocF64,
    mfv: &mut AllocFV,
    mpdf: &mut AllocPDF,
    mc: &mut AllocStaticCommand,
    mhl: &mut AllocHL,
    mhc: &mut AllocHC,
    mhd: &mut AllocHD,
    mhp: &mut AllocHP,
    mct: &mut AllocCT,
    mht: &mut AllocHT,
    mzn: &mut AllocZN,
    available_in: &mut usize,
    next_in_array: &[u8],
    next_in_offset: &mut usize,
    available_out: &mut usize,
    next_out_array: &mut[u8],
    next_out_offset: &mut usize,
    total_out: &mut Option<usize>,
    metablock_callback: &mut MetaBlockCallback)
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
    if InjectFlushOrPushOutput(s, available_out, next_out_array, next_out_offset, total_out) != 0 {
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
      let mut avail_out : usize = (*s).available_out_;
      let result: i32 =
            EncodeData(s, m64, mf64, mfv, mpdf, mc, mhl, mhc, mhd, mhp, mct, mht, mzn, 0i32, 1i32, &mut avail_out, metablock_callback);
      (*s).available_out_ = avail_out;
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
      (*s).next_out_ = NextOut::TinyBuf(0);
      (*s).available_out_ = WriteMetadataHeader(s);
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
        let copy: u32 = brotli_min_size_t((*s).remaining_metadata_bytes_ as (usize),
                                              *available_out) as (u32);
        next_out_array[*next_out_offset .. (*next_out_offset + copy as usize)].clone_from_slice(
            &next_in_array[*next_in_offset ..(*next_in_offset + copy as usize)]);
        //memcpy(*next_out, *next_in, copy as (usize));
        // *next_in = (*next_in).offset(copy as (isize));
        *next_in_offset += copy as usize;
        *available_in = (*available_in).wrapping_sub(copy as (usize));
        (*s).remaining_metadata_bytes_ = (*s).remaining_metadata_bytes_.wrapping_sub(copy);
        *next_out_offset += copy as usize;
        // *next_out = (*next_out).offset(copy as (isize));
        *available_out = (*available_out).wrapping_sub(copy as (usize));
      } else {
        let copy: u32 = brotli_min_uint32_t((*s).remaining_metadata_bytes_, 16u32);
        (*s).next_out_ = NextOut::TinyBuf(0);
        GetNextOut!(s)[..(copy as usize)].clone_from_slice(
            &next_in_array[*next_in_offset ..(*next_in_offset + copy as usize)]);
        //memcpy((*s).next_out_, *next_in, copy as (usize));
        // *next_in = (*next_in).offset(copy as (isize));
        *next_in_offset += copy as usize;
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
fn CheckFlushCompleteInner(stream_state: &mut BrotliEncoderStreamState,
                           available_out: usize,
                           next_out: &mut NextOut) {

    if *stream_state ==
     BrotliEncoderStreamState::BROTLI_STREAM_FLUSH_REQUESTED &&
     (available_out == 0) {
    *stream_state = BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING;
    *next_out = NextOut::None;
  }
}

fn CheckFlushComplete<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) {
    CheckFlushCompleteInner(&mut (*s).stream_state_,
                            (*s).available_out_,
                            &mut (*s).next_out_);
}


fn BrotliEncoderCompressStreamFast<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                                   AllocCommand: alloc::Allocator<Command>,
                                   AllocHT: alloc::Allocator<HuffmanTree>>(
    s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
    mht: &mut AllocHT,
    op: BrotliEncoderOperation,
    available_in: &mut usize,
    next_in_array: &[u8],
    next_in_offset: &mut usize,
    available_out: &mut usize,
    next_out_array: &mut [u8],
    next_out_offset: &mut usize,
    total_out: &mut Option<usize>)
            -> i32 {
  let block_size_limit: usize = 1usize << (*s).params.lgwin;
  let buf_size: usize = brotli_min_size_t(kCompressFragmentTwoPassBlockSize,
                                          brotli_min_size_t(*available_in, block_size_limit));
  let mut command_buf = AllocU32::AllocatedMemory::default();
  let mut literal_buf = AllocU8::AllocatedMemory::default();
  if (*s).params.quality != 0i32 && ((*s).params.quality != 1i32) {
    return 0i32;
  }
  if (*s).params.quality == 1i32 {
    if (*s).command_buf_.slice().len() == 0 && (buf_size == kCompressFragmentTwoPassBlockSize) {
      (*s).command_buf_ = s.m32.alloc_cell(kCompressFragmentTwoPassBlockSize);
      (*s).literal_buf_ = s.m8.alloc_cell(kCompressFragmentTwoPassBlockSize);
    }
    if (*s).command_buf_.slice().len() != 0 {
      command_buf = core::mem::replace(&mut (*s).command_buf_, AllocU32::AllocatedMemory::default());
      literal_buf = core::mem::replace(&mut (*s).literal_buf_, AllocU8::AllocatedMemory::default());
    } else {
      command_buf = s.m32.alloc_cell(buf_size);
      literal_buf = s.m8.alloc_cell(buf_size);
    }
  }
  while 1i32 != 0 {
    if InjectFlushOrPushOutput(s, available_out, next_out_array, next_out_offset, total_out) != 0 {
      {
        continue;
      }
    }
    if (*s).available_out_ == 0usize &&
       ((*s).stream_state_ as (i32) ==
        BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING as (i32)) &&
       (*available_in != 0usize ||
        op as (i32) != BrotliEncoderOperation::BROTLI_OPERATION_PROCESS as (i32)) {
      let block_size: usize = brotli_min_size_t(block_size_limit, *available_in);
      let is_last: i32 = (*available_in == block_size &&
                              (op as (i32) ==
                               BrotliEncoderOperation::BROTLI_OPERATION_FINISH as (i32))) as
                             (i32);
      let force_flush: i32 =
        (*available_in == block_size &&
         (op as (i32) == BrotliEncoderOperation::BROTLI_OPERATION_FLUSH as (i32))) as (i32);
      let max_out_size: usize = (2usize).wrapping_mul(block_size).wrapping_add(503usize);
      let mut inplace: i32 = 1i32;
      let storage: &mut [u8];
      let mut storage_ix: usize = (*s).last_bytes_bits_ as (usize);
      let mut table_size: usize = 0;
      let table: &mut [i32];
      if force_flush != 0 && (block_size == 0usize) {
        (*s).stream_state_ = BrotliEncoderStreamState::BROTLI_STREAM_FLUSH_REQUESTED;
        {
          {
            continue;
          }
        }
      }
      if max_out_size <= *available_out {
        storage = &mut next_out_array[*next_out_offset..];//GetNextOut!(s);
      } else {
        inplace = 0i32;
        GetBrotliStorage(s, max_out_size);
        storage = (*s).storage_.slice_mut();
      }
      storage[(0usize)] = (*s).last_bytes_ as u8;
      storage[(1usize)] = ((*s).last_bytes_  >> 8) as u8;
      table = GetHashTable!(s, (*s).params.quality, block_size, &mut table_size);
      if (*s).params.quality == 0i32 {
        BrotliCompressFragmentFast(mht,
                                   &(next_in_array)[*next_in_offset..],
                                   block_size,
                                   is_last,
                                   table,
                                   table_size,
                                   &mut (*s).cmd_depths_[..],
                                   &mut (*s).cmd_bits_[..],
                                   &mut (*s).cmd_code_numbits_,
                                   &mut (*s).cmd_code_[..],
                                   &mut storage_ix,
                                   storage);
      } else {
        BrotliCompressFragmentTwoPass(mht,
                                      &(next_in_array)[*next_in_offset..],
                                      block_size,
                                      is_last,
                                      command_buf.slice_mut(),
                                      literal_buf.slice_mut(),
                                      table,
                                      table_size,
                                      &mut storage_ix,
                                      storage);
      }
      *next_in_offset += block_size as usize;
      *available_in = (*available_in).wrapping_sub(block_size);
      if inplace != 0 {
        let out_bytes: usize = storage_ix >> 3i32;
        0i32;
        0i32;
        *next_out_offset += out_bytes as (usize);
        *available_out = (*available_out).wrapping_sub(out_bytes);
        (*s).total_out_ = (*s).total_out_.wrapping_add(out_bytes);
        if let &mut Some(ref mut total_out_inner) = total_out {
          *total_out_inner = (*s).total_out_;
        }
      } else {
        let out_bytes: usize = storage_ix >> 3i32;
        (*s).next_out_ = NextOut::DynamicStorage(0);
        (*s).available_out_ = out_bytes;
      }
      (*s).last_bytes_ = storage[((storage_ix >> 3i32) as (usize))] as u16 | (
          ((storage[1 + ((storage_ix >> 3i32) as (usize))] as u16)<< 8));
      (*s).last_bytes_bits_ = (storage_ix & 7u32 as (usize)) as (u8);
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
  if command_buf.slice().len() == kCompressFragmentTwoPassBlockSize && s.command_buf_.slice().len() == 0 {
      // undo temporary aliasing of command_buf and literal_buf
      (*s).command_buf_ = core::mem::replace(&mut command_buf, AllocU32::AllocatedMemory::default());
      (*s).literal_buf_ = core::mem::replace(&mut literal_buf, AllocU8::AllocatedMemory::default());
  } else {
      s.m32.free_cell(command_buf);
      s.m8.free_cell(literal_buf);
  }
  CheckFlushComplete(s);
  1i32
}
fn RemainingInputBlockSize<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) -> usize {
  let delta: u64 = UnprocessedInputSize(s);
  let block_size: usize = InputBlockSize(s);
  if delta >= block_size as u64 {
    return 0usize;
  }
  (block_size as u64).wrapping_sub(delta) as usize
}

pub fn BrotliEncoderCompressStream<AllocU8: alloc::Allocator<u8>,
                                   AllocU16: alloc::Allocator<u16>,
                                   AllocU32: alloc::Allocator<u32>,
                                   AllocI32: alloc::Allocator<i32>,
                                   AllocU64: alloc::Allocator<u64>,
                                   AllocF64: alloc::Allocator<super::util::floatX>,
                                   AllocFV: alloc::Allocator<Mem256f>,
                                   AllocPDF: alloc::Allocator<PDF>,
                                   AllocStaticCommand: alloc::Allocator<StaticCommand>,
                                   AllocHL: alloc::Allocator<HistogramLiteral>,
                                   AllocHC: alloc::Allocator<HistogramCommand>,
                                   AllocHD: alloc::Allocator<HistogramDistance>,
                                   AllocHP: alloc::Allocator<HistogramPair>,
                                   AllocCT: alloc::Allocator<ContextType>,
                                   AllocCommand: alloc::Allocator<Command>,
                                   AllocHT:alloc::Allocator<HuffmanTree>,
                                   AllocZN: alloc::Allocator<ZopfliNode>,
                                   MetablockCallback:FnMut(&mut interface::PredictionModeContextMap<InputReferenceMut>,
                                                           &mut [interface::StaticCommand],
                                                           interface::InputPair)>(
    s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
    m64: &mut AllocU64,
    mf64: &mut AllocF64,
    mfv: &mut AllocFV,
    mpdf: &mut AllocPDF,
    mc: &mut AllocStaticCommand,
    mhl: &mut AllocHL,
    mhc: &mut AllocHC,
    mhd: &mut AllocHD,
    mhp: &mut AllocHP,
    mct: &mut AllocCT,
    mht: &mut AllocHT,
    mzn: &mut AllocZN,
    op: BrotliEncoderOperation,
    available_in: &mut usize,
    next_in_array: &[u8],
    next_in_offset: &mut usize,
    available_out: &mut usize,
    next_out_array: &mut [u8],
    next_out_offset: &mut usize,
    total_out: &mut Option<usize>,
    metablock_callback: &mut MetablockCallback)
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
    return ProcessMetadata(s, m64, mf64, mfv, mpdf, mc, mhl, mhc, mhd, mhp, mct, mht, mzn, available_in, next_in_array, next_in_offset, available_out, next_out_array, next_out_offset, total_out, metablock_callback);
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
                                           mht,
                                           op,
                                           available_in,
                                           next_in_array,
                                           next_in_offset,
                                           available_out,
                                           next_out_array,
                                           next_out_offset,
                                           total_out);
  }
  while 1i32 != 0 {
    let remaining_block_size: usize = RemainingInputBlockSize(s);
    if remaining_block_size != 0usize && (*available_in != 0usize) {
      let copy_input_size: usize = brotli_min_size_t(remaining_block_size, *available_in);
      CopyInputToRingBuffer(s, copy_input_size, &next_in_array[*next_in_offset..]);
      *next_in_offset += copy_input_size as (usize);
      *available_in = (*available_in).wrapping_sub(copy_input_size);
      {
        {
          continue;
        }
      }
    }
    if InjectFlushOrPushOutput(s, available_out, next_out_array, next_out_offset, total_out) != 0 {
      {
        continue;
      }
    }
    if (*s).available_out_ == 0usize &&
       ((*s).stream_state_ as (i32) ==
        BrotliEncoderStreamState::BROTLI_STREAM_PROCESSING as (i32)) {
      if remaining_block_size == 0usize ||
         op as (i32) != BrotliEncoderOperation::BROTLI_OPERATION_PROCESS as (i32) {
        let is_last: i32 = if !!(*available_in == 0usize &&
                                 (op as (i32) ==
                                  BrotliEncoderOperation::BROTLI_OPERATION_FINISH as (i32))) {
          1i32
        } else {
          0i32
        };
        let force_flush: i32 =
          if !!(*available_in == 0usize &&
                (op as (i32) == BrotliEncoderOperation::BROTLI_OPERATION_FLUSH as (i32))) {
            1i32
          } else {
            0i32
          };
        let result: i32;
        UpdateSizeHint(s, *available_in);
        let mut avail_out = (*s).available_out_;
        result = EncodeData(s,
                            m64, mf64, mfv, mpdf, mc, mhl, mhc, mhd, mhp, mct, mht, mzn,
                            is_last,
                            force_flush,
                            &mut avail_out,
                            metablock_callback);
        (*s).available_out_ = avail_out;
        //this function set next_out to &storage[0]
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
                     AllocCommand: alloc::Allocator<Command>>(s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) -> i32 {
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
                                  AllocCommand: alloc::Allocator<Command>>(
    s: &BrotliEncoderStateStruct<AllocU8,
                                 AllocU16, AllocU32, AllocI32, AllocCommand>) -> i32 {
  if !!((*s).available_out_ != 0usize) {
    1i32
  } else {
    0i32
  }
}


pub fn BrotliEncoderTakeOutput<'a, AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(s: &'a mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
                               size: &mut usize)
                               -> &'a [u8] {
  let mut consumed_size: usize = (*s).available_out_;
  let mut result: &[u8] = GetNextOut!(*s);
  if *size != 0 {
    consumed_size = brotli_min_size_t(*size, (*s).available_out_);
  }
  if consumed_size != 0 {
    (*s).next_out_ = NextOutIncrement(&(*s).next_out_, consumed_size as i32);
    (*s).available_out_ = (*s).available_out_.wrapping_sub(consumed_size);
    (*s).total_out_ = (*s).total_out_.wrapping_add(consumed_size);
    CheckFlushCompleteInner(&mut (*s).stream_state_,
                            (*s).available_out_,
                            &mut (*s).next_out_);
    *size = consumed_size;
  } else {
    *size = 0usize;
    result = &[];
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
                     AllocCommand: alloc::Allocator<Command>>(s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>) -> usize {
  InputBlockSize(s)
}


pub fn BrotliEncoderCopyInputToRingBuffer<AllocU8: alloc::Allocator<u8>,
                     AllocU16: alloc::Allocator<u16>,
                     AllocU32: alloc::Allocator<u32>,
                     AllocI32: alloc::Allocator<i32>,
                     AllocCommand: alloc::Allocator<Command>>(s: &mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
                                          input_size: usize,
                                          input_buffer: &[u8]) {
  CopyInputToRingBuffer(s, input_size, input_buffer);
}


pub fn BrotliEncoderWriteData<'a, AllocU8: alloc::Allocator<u8>,
                              AllocU16: alloc::Allocator<u16>,
                              AllocU32: alloc::Allocator<u32>,
                              AllocI32: alloc::Allocator<i32>,
                              AllocU64: alloc::Allocator<u64>,
                              AllocF64: alloc::Allocator<super::util::floatX>,
                              AllocFV: alloc::Allocator<Mem256f>,
                              AllocPDF: alloc::Allocator<PDF>,
                              AllocStaticCommand: alloc::Allocator<StaticCommand>,
                              AllocHL: alloc::Allocator<HistogramLiteral>,
                              AllocHC: alloc::Allocator<HistogramCommand>,
                              AllocHD: alloc::Allocator<HistogramDistance>,
                              AllocHP: alloc::Allocator<HistogramPair>,
                              AllocCT: alloc::Allocator<ContextType>,
                              AllocCommand: alloc::Allocator<Command>,
                              AllocHT:alloc::Allocator<HuffmanTree>,
                              AllocZN:alloc::Allocator<ZopfliNode>,
                              MetablockCallback:FnMut(&mut interface::PredictionModeContextMap<InputReferenceMut>,
                                                       &mut [interface::StaticCommand],
                                                       interface::InputPair)>(
    s: &'a mut BrotliEncoderStateStruct<AllocU8, AllocU16, AllocU32, AllocI32, AllocCommand>,
    m64: &mut AllocU64,
    mf64: &mut AllocF64,
    mfv: &mut AllocFV,
    mpdf: &mut AllocPDF,
    mc: &mut AllocStaticCommand,
    mhl: &mut AllocHL,
    mhc: &mut AllocHC,
    mhd: &mut AllocHD,
    mhp: &mut AllocHP,
    mct: &mut AllocCT,
    mht: &mut AllocHT,
    mzn: &mut AllocZN,
    is_last: i32,
    force_flush: i32,
    out_size: &mut usize,
    output: &'a mut &'a mut [u8],
    metablock_callback: &mut MetablockCallback)
                              -> i32 {
    let ret = EncodeData(s, m64, mf64, mfv, mpdf, mc, mhl, mhc, mhd, mhp, mct, mht, mzn, is_last, force_flush, out_size, metablock_callback);
    *output = (*s).storage_.slice_mut();
    ret
}
