#![allow(unknown_lints)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#[cfg(not(feature="no-stdlib"))]
use std::io::Write;
use super::util::floatX;
use super::input_pair::{InputPair, InputReference};
use super::block_split::BlockSplit;
use enc::backward_references::BrotliEncoderParams;

use super::super::dictionary::{kBrotliDictionary, kBrotliDictionarySizeBitsByLength,
                               kBrotliDictionaryOffsetsByLength};
use super::super::transform::{TransformDictionaryWord};
use super::static_dict::kNumDistanceCacheEntries;
use super::command::{Command, GetCopyLengthCode, GetInsertLengthCode, CommandDistanceIndexAndOffset};
use super::constants::{BROTLI_NUM_BLOCK_LEN_SYMBOLS, kZeroRepsBits, kZeroRepsDepth,
                       kNonZeroRepsBits, kNonZeroRepsDepth, kCodeLengthBits, kCodeLengthDepth,
                       kStaticCommandCodeDepth, kStaticCommandCodeBits, kStaticDistanceCodeDepth,
                       kStaticDistanceCodeBits, kSigned3BitContextLookup, kUTF8ContextLookup,
                       kInsBase, kInsExtra, kCopyBase, kCopyExtra};
use super::entropy_encode::{HuffmanTree, BrotliWriteHuffmanTree, BrotliCreateHuffmanTree,
                            BrotliConvertBitDepthsToSymbols, NewHuffmanTree, InitHuffmanTree,
                            SortHuffmanTreeItems, HuffmanComparator, BrotliSetDepth};
use super::histogram::{HistogramAddItem, HistogramLiteral, HistogramCommand, HistogramDistance,
                       ContextType};
use super::super::alloc;
use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::super::core;
use super::find_stride;

use super::interface;
use super::interface::CommandProcessor;
use super::context_map_entropy::{ContextMapEntropy, SpeedAndMax};
pub struct PrefixCodeRange {
  pub offset: u32,
  pub nbits: u32,
}

fn window_size_from_lgwin(lgwin: i32) -> usize{
    (1usize << lgwin) - 16usize
}

fn context_type_str(context_type:ContextType) -> &'static str {
   match context_type {
         ContextType::CONTEXT_LSB6 => "lsb6",
         ContextType::CONTEXT_MSB6 => "msb6",
         ContextType::CONTEXT_UTF8 => "utf8",
         ContextType::CONTEXT_SIGNED => "sign",
   }
}

fn prediction_mode_str(prediction_mode_nibble:interface::LiteralPredictionModeNibble) -> &'static str {
   match prediction_mode_nibble.prediction_mode() {
         interface::LITERAL_PREDICTION_MODE_SIGN => "sign",
         interface::LITERAL_PREDICTION_MODE_LSB6 => "lsb6",
         interface::LITERAL_PREDICTION_MODE_MSB6 => "msb6",
         interface::LITERAL_PREDICTION_MODE_UTF8 => "utf8",
         _ => "unknown",
   }
}


const COMMAND_BUFFER_SIZE: usize = 8192;


struct CommandQueue<'a, AllocU16:alloc::Allocator<u16>, AllocU32:alloc::Allocator<u32>, AllocF:alloc::Allocator<floatX>> {
    mb: InputPair<'a>,
    mb_byte_offset: usize,
    queue: [interface::Command<InputReference<'a> >;COMMAND_BUFFER_SIZE],
    loc: usize,
    last_btypel_index: Option<usize>,
    entropy_tally_scratch: find_stride::EntropyTally<AllocU32>,
    entropy_pyramid: find_stride::EntropyPyramid<AllocU32>,
    context_map_entropy: ContextMapEntropy<'a, AllocU16, AllocU32, AllocF>,
    stride_detection_quality: u8,
    high_entropy_detection_quality: u8,
    block_type_literal: u8,
}

impl<'a,
     AllocU16:alloc::Allocator<u16>,
     AllocU32:alloc::Allocator<u32>,
     AllocF:alloc::Allocator<floatX> > CommandQueue<'a, AllocU16, AllocU32, AllocF> {
    fn new(_m32: &mut AllocU32,
           mb: InputPair<'a>,
           stride_detection_quality: u8,
           high_entropy_detection_quality: u8,
           context_map_entropy: ContextMapEntropy<'a, AllocU16, AllocU32, AllocF>,
           entropy_tally_scratch: find_stride::EntropyTally<AllocU32>,
           entropy_pyramid: find_stride::EntropyPyramid<AllocU32>,
           ) -> CommandQueue <'a, AllocU16, AllocU32, AllocF> {
        CommandQueue {
            mb:mb,
            mb_byte_offset:0,
            queue:[interface::Command::<InputReference<'a>>::default();COMMAND_BUFFER_SIZE],
            loc:0,
            entropy_tally_scratch: entropy_tally_scratch,
            entropy_pyramid: entropy_pyramid,
            last_btypel_index: None,
            stride_detection_quality: stride_detection_quality,
            high_entropy_detection_quality: high_entropy_detection_quality,
            context_map_entropy: context_map_entropy,
            block_type_literal: 0,
        }
    }
    fn full(&self) -> bool {
        self.loc == self.queue.len()
    }
    fn size(&self) -> usize {
        self.loc
    }
    fn clear(&mut self) {
        self.loc = 0;
        self.block_type_literal = 0;
    }
    fn content(&mut self) -> &[interface::Command<InputReference>] {
        self.queue.split_at(self.loc).0
    }
    fn flush<Cb>(&mut self, callback: &mut Cb) where Cb:FnMut(&[interface::Command<InputReference>]) {
       let mut local_byte_offset = self.mb_byte_offset;
       let cur_stride = self.entropy_tally_scratch.pick_best_stride(self.queue.split_at(self.loc).0,
                                                                    self.mb.0,
                                                                    self.mb.1,
                                                                    &mut self.mb_byte_offset,
                                                                    &self.entropy_pyramid,
                                                                    self.stride_detection_quality);
       if self.high_entropy_detection_quality > 1 {
           for command in self.queue.split_at_mut(self.loc).0.iter_mut() {
               match *command {
                   interface::Command::BlockSwitchCommand(_) |
                   interface::Command::BlockSwitchDistance(_) |
                   interface::Command::PredictionMode(_) => {},
                   interface::Command::BlockSwitchLiteral(bs) => {
                      self.block_type_literal = bs.block_type();
                   },
                   interface::Command::Copy(ref copy) => {
                       local_byte_offset += copy.num_bytes as usize;
                   },
                   interface::Command::Dict(ref dict) => {
                       local_byte_offset += dict.final_size as usize;
                   },
                   interface::Command::Literal(ref mut lit) => {
                       let priors = self.entropy_tally_scratch.get_previous_bytes(
                           self.mb.0,
                           self.mb.1,
                           local_byte_offset);
                       let mut rev_priors = priors;
                       rev_priors.reverse();
                       //print!("Stride {} prev {:?} byte offset {} {:?}\n", cur_stride, rev_priors, local_byte_offset, lit.data.slice());
                       self.context_map_entropy.track_cdf_speed(lit.data.slice(), priors[0], priors[1], self.block_type_literal);
                       local_byte_offset += lit.data.slice().len();
                   }
               }
           }
       }
       match self.last_btypel_index.clone() {
           None => {},
           Some(literal_block_type_offset) => {
               match &mut self.queue[literal_block_type_offset] {
                   &mut interface::Command::BlockSwitchLiteral(ref mut cmd) => cmd.1 = cur_stride,
                   _ => panic!("Logic Error: literal block type index must point to literal block type"),
               }
           },
       }
       self.last_btypel_index = None;
       callback(self.queue.split_at(self.loc).0);
       self.clear();
    }
    fn free<Cb>(&mut self, m16: &mut AllocU16, m32: &mut AllocU32, mf64: &mut AllocF,
                callback: &mut Cb) where Cb:FnMut(&[interface::Command<InputReference>]) {
       self.flush(callback);
       self.entropy_tally_scratch.free(m32);
       self.entropy_pyramid.free(m32);
       self.context_map_entropy.free(m16, m32, mf64);

    }
}
impl<'a, AllocU16: alloc::Allocator<u16>,
     AllocU32: alloc::Allocator<u32>,
     AllocF: alloc::Allocator<floatX>> interface::CommandProcessor<'a> for CommandQueue<'a, AllocU16, AllocU32, AllocF> {
    fn push<Cb> (&mut self, val: interface::Command<InputReference<'a> >, callback :&mut Cb)
     where Cb: FnMut(&[interface::Command<InputReference>]) {
        self.queue[self.loc] = val;
        self.loc += 1;
        if self.full() {
            self.flush(callback);
        }
    }
    fn push_block_switch_literal<Cb>(&mut self, block_type: u8, callback: &mut Cb) where Cb:FnMut(&[interface::Command<InputReference>]) {
        self.flush(callback);
        self.last_btypel_index = Some(self.size());
        self.push(interface::Command::BlockSwitchLiteral(
            interface::LiteralBlockSwitch::new(block_type, 0)), callback)
    }
}


#[cfg(not(feature="no-stdlib"))]
fn warn_on_missing_free() {
    let _err = ::std::io::stderr().write(b"Need to free entropy_tally_scratch before dropping CommandQueue\n");
}
#[cfg(feature="no-stdlib")]
fn warn_on_missing_free() {
     // no way to warn in this case
}
impl<'a,
     AllocU16: alloc::Allocator<u16>,
     AllocU32: alloc::Allocator<u32>,
     AllocF: alloc::Allocator<floatX>> Drop for CommandQueue<'a, AllocU16, AllocU32, AllocF> {
  fn drop(&mut self) {
      if !self.entropy_tally_scratch.is_free() {
          warn_on_missing_free();
      }
  }
}
/*
#[cfg(not(feature="billing"))]
fn best_speed_log(_name:&str,
                  _data:&[[SpeedAndMax;2];256],
                  _cost:&[[floatX;2];256]) {}
#[cfg(feature="billing")]*/
fn best_speed_log(name:&str,
                  data:&[[SpeedAndMax;2];256],
                  cost:&[[floatX;2];256]) {
    let mut total_cost:floatX = 0.0;
    for (index, _spd) in data.iter().enumerate() {
        for high in 0..2 {
           total_cost += cost[index][high];
        }
    }
    println!("{} cost: {}", name, total_cost);
    for (index, spd) in data.iter().enumerate() {
        for high in 0..2 {
            println!("({}, {}) Speed [ inc: {}, max: {}, algo: 0 ] cost: {}",
                     index,
                     high != 0,
                     spd[high].0,
                     spd[high].1,
                     cost[index][high]);
        }
    }
}

fn process_command_queue<'a, Cb:FnMut(&[interface::Command<InputReference>]), CmdProcessor: interface::CommandProcessor<'a> > (
    command_queue: &mut CmdProcessor,
    input: InputPair<'a>,
    commands: &[Command],
    n_postfix: u32,
    n_direct: u32,
    dist_cache: &[i32;kNumDistanceCacheEntries],
    mut recoder_state :RecoderState,
    block_type: &MetaBlockSplitRefs,
    params: &BrotliEncoderParams,
    context_type:Option<ContextType>,
    callback: &mut Cb,
) -> RecoderState {
    let mut input_iter = input.clone();
    let mut local_dist_cache = [0i32;kNumDistanceCacheEntries];
    local_dist_cache.clone_from_slice(&dist_cache[..]);
    let mut btypel_counter = 0usize;
    let mut btypec_counter = 0usize;
    let mut btyped_counter = 0usize;
    let mut btypel_sub = if block_type.btypel.num_types == 1 { 1u32<<31 } else {block_type.btypel.lengths[0]};
    let mut btypec_sub = if block_type.btypec.num_types == 1 { 1u32<<31 } else {block_type.btypec.lengths[0]};
    let mut btyped_sub = if block_type.btyped.num_types == 1 { 1u32<<31 } else {block_type.btyped.lengths[0]};
    {
        command_queue.push_block_switch_literal(0, callback);
    }
    let mut mb_len = input.len();
    for cmd in commands.iter() {
        let (inserts, interim) = input_iter.split_at(core::cmp::min(cmd.insert_len_ as usize,
                                                                     mb_len));
        recoder_state.num_bytes_encoded += inserts.len();
        let _copy_cursor = input.len() - interim.len();
        // let distance_context = CommandDistanceContext(cmd);
        let copylen_code: u32 = CommandCopyLenCode(cmd);

        let (prev_dist_index, dist_offset) = CommandDistanceIndexAndOffset(cmd, n_postfix, n_direct);
        let final_distance: usize;
        if prev_dist_index == 0 {
            final_distance = dist_offset as usize;
        } else {
            final_distance = (local_dist_cache[prev_dist_index - 1] as isize + dist_offset) as usize;
        }
        let copy_len = copylen_code as usize;
        let actual_copy_len : usize;
        let max_distance = core::cmp::min(recoder_state.num_bytes_encoded, window_size_from_lgwin(params.lgwin));
        assert!(inserts.len() <= mb_len);
        {
            btypec_sub -= 1;
            if btypec_sub == 0 {
                btypec_counter += 1;
                if block_type.btypec.types.len() > btypec_counter {
                    btypec_sub = block_type.btypec.lengths[btypec_counter];
                    command_queue.push(interface::Command::BlockSwitchCommand(
                        interface::BlockSwitch(block_type.btypec.types[btypec_counter])),
                                       callback);
                } else {
                    btypec_sub = 1u32 << 31;
                }
            }
        }
        if inserts.len() != 0 {
            let mut tmp_inserts = inserts;
            while tmp_inserts.len() > btypel_sub as usize {
                // we have to divide some:
                let (in_a, in_b) = tmp_inserts.split_at(btypel_sub as usize);
                if in_a.len() != 0 {
                    if let Some(_) = context_type {
                        command_queue.push_literals(&in_a, callback);
                    } else if params.high_entropy_detection_quality == 0 {
                        command_queue.push_literals(&in_a, callback);
                    } else {
                        command_queue.push_rand_literals(&in_a, callback);
                    }
                }
                mb_len -= in_a.len();
                tmp_inserts = in_b;
                btypel_counter += 1;
                if block_type.btypel.types.len() > btypel_counter {
                    btypel_sub = block_type.btypel.lengths[btypel_counter];
                    command_queue.push_block_switch_literal(block_type.btypel.types[btypel_counter], callback);
                } else {
                    btypel_sub = 1u32<<31;
                }
            }
            if let Some(_) = context_type {
                command_queue.push_literals(&tmp_inserts, callback);
            } else if params.high_entropy_detection_quality == 0 {
                command_queue.push_literals(&tmp_inserts, callback);
            }else {
                command_queue.push_rand_literals(&tmp_inserts, callback);
            }
            if tmp_inserts.len() != 0 {
                mb_len -= tmp_inserts.len();
                btypel_sub -= tmp_inserts.len() as u32;
            }
        }
        if copy_len != 0 && cmd.cmd_prefix_ >= 128 {
            btyped_sub -= 1;
            if btyped_sub == 0 {
                btyped_counter += 1;
                if block_type.btyped.types.len() > btyped_counter {
                    btyped_sub = block_type.btyped.lengths[btyped_counter];
                    command_queue.push(interface::Command::BlockSwitchDistance(
                        interface::BlockSwitch(block_type.btyped.types[btyped_counter])), callback);
                } else {
                    btyped_sub = 1u32 << 31;
                }
            }
        }
        if final_distance > max_distance { // is dictionary
            assert!(copy_len >= 4);
            assert!(copy_len < 25);
            let dictionary_offset = final_distance - max_distance - 1;
            let ndbits = kBrotliDictionarySizeBitsByLength[copy_len] as usize;
            let action = dictionary_offset >> ndbits;
            let word_sub_index = dictionary_offset & ((1 << ndbits) - 1);
            let word_index = word_sub_index * copy_len + kBrotliDictionaryOffsetsByLength[copy_len] as usize;
            let raw_word = &kBrotliDictionary[word_index..word_index + copy_len];
            let mut transformed_word = [0u8; 38];
            actual_copy_len = TransformDictionaryWord(&mut transformed_word[..],
                                                      raw_word,
                                                      copy_len as i32,
                                                      action as i32) as usize;
            if actual_copy_len <= mb_len {
                command_queue.push(interface::Command::Dict(
                    interface::DictCommand{
                        word_size: copy_len as u8,
                        transform: action as u8,
                        final_size: actual_copy_len as u8,
                        empty: 0,
                        word_id: word_sub_index as u32,
                    }), callback);
                mb_len -= actual_copy_len;
                assert_eq!(InputPair(transformed_word.split_at(actual_copy_len).0, &[]),
                           interim.split_at(actual_copy_len).0);
            } else if mb_len != 0 {
                // truncated dictionary word: represent it as literals instead
                // won't be random noise since it fits in the dictionary, so we won't check for rand
                command_queue.push_literals(&interim.split_at(mb_len).0, callback);
                mb_len = 0;
                assert_eq!(InputPair(transformed_word.split_at(mb_len).0, &[]),
                           interim.split_at(mb_len).0);
            }
        } else {
            actual_copy_len = core::cmp::min(mb_len, copy_len);
            if actual_copy_len != 0 {
                command_queue.push(interface::Command::Copy(
                    interface::CopyCommand{
                        distance: final_distance as u32,
                        num_bytes: actual_copy_len as u32,
                    }), callback);
            }
            mb_len -= actual_copy_len;
            if prev_dist_index != 1 || dist_offset != 0 { // update distance cache unless it's the "0 distance symbol"
               let mut tmp_dist_cache = [0i32;kNumDistanceCacheEntries - 1];
               tmp_dist_cache.clone_from_slice(&local_dist_cache[..kNumDistanceCacheEntries - 1]);
               local_dist_cache[1..].clone_from_slice(&tmp_dist_cache[..]);
               local_dist_cache[0] = final_distance as i32;

            }
        }
        let (copied, remainder) = interim.split_at(actual_copy_len);
        recoder_state.num_bytes_encoded += copied.len();
        input_iter = remainder;
    }
    recoder_state
}

fn LogMetaBlock<'a,
                AllocU16:alloc::Allocator<u16>,
                AllocU32:alloc::Allocator<u32>,
                AllocF:alloc::Allocator<floatX>,
                Cb>(m16: &mut AllocU16,
                    m32: &mut AllocU32,
                    mf: &mut AllocF,
                    commands: &[Command], input0: &'a[u8],input1: &'a[u8],
                    n_postfix: u32,
                    n_direct: u32,
                    dist_cache: &[i32;kNumDistanceCacheEntries],
                    recoder_state :&mut RecoderState,
                    block_type: MetaBlockSplitRefs,
                    params: &BrotliEncoderParams,
                    context_type:Option<ContextType>,
                    callback: &mut Cb) where Cb:FnMut(&[interface::Command<InputReference>]){
    let mut local_literal_context_map = [0u8; 256 * 64];
    let mut local_distance_context_map = [0u8; 256 * 64];
    let mut local_context_speeds = [0u8; 1024 * 3];
    assert_eq!(*block_type.btypel.types.iter().max().unwrap_or(&0) as u32 + 1,
               block_type.btypel.num_types);
    assert_eq!(*block_type.btypec.types.iter().max().unwrap_or(&0) as u32 + 1,
               block_type.btypec.num_types);
    assert_eq!(*block_type.btyped.types.iter().max().unwrap_or(&0) as u32 + 1,
               block_type.btyped.num_types);
    if block_type.literal_context_map.len() <= 256 * 64 {
        for (index, item) in block_type.literal_context_map.iter().enumerate() {
            local_literal_context_map[index] = *item as u8;
        }
    }
    if block_type.distance_context_map.len() <= 256 * 64 {
        for (index, item) in block_type.distance_context_map.iter().enumerate() {
            local_distance_context_map[index] = *item as u8;
        }
    }
    let mut prediction_mode = interface::PredictionModeContextMap::<InputReference>{
        literal_prediction_mode: interface::LiteralPredictionModeNibble(context_type.unwrap_or(ContextType::CONTEXT_LSB6) as u8),
        literal_context_map:InputReference(&local_literal_context_map.split_at(block_type.literal_context_map.len()).0),
        distance_context_map:InputReference(&local_distance_context_map.split_at(block_type.distance_context_map.len()).0),
        context_speeds:InputReference(&[]),
    };
    let mut entropy_tally_scratch = if params.stride_detection_quality == 0 {
        find_stride::EntropyTally::<AllocU32>::disabled_placeholder(m32)
    } else {
        find_stride::EntropyTally::<AllocU32>::new(m32, None)
    };
    let mut entropy_pyramid = if params.stride_detection_quality == 0{
        find_stride::EntropyPyramid::<AllocU32>::disabled_placeholder(m32)
    } else {
        find_stride::EntropyPyramid::<AllocU32>::new(m32)
    };
    if params.stride_detection_quality > 0 {
        entropy_pyramid.populate(input0, input1, &mut entropy_tally_scratch);
    }
    let mut context_map_entropy = ContextMapEntropy::<AllocU16, AllocU32, AllocF>::new(m16, m32, mf, InputPair(input0, input1),
                                                                                       entropy_pyramid.stride_last_level_range(),
                                                                                       prediction_mode);
    let input = InputPair(input0, input1);
    process_command_queue(&mut context_map_entropy,
                         input,
                         commands,
                         n_postfix,
                         n_direct,
                         dist_cache,
                         *recoder_state,
                         &block_type,
                         params,
                         context_type,
                          &mut |_x|());
     let cm_speed = context_map_entropy.best_speeds(true, false);
     let stride_speed = context_map_entropy.best_speeds(false, false);
     let combined_speed = context_map_entropy.best_speeds(false, true);
     let acost = context_map_entropy.best_speeds_costs(true, false);
     let bcost = context_map_entropy.best_speeds_costs(false, false);
     let ccost = context_map_entropy.best_speeds_costs(false, true);
    if params.high_entropy_detection_quality != 0 {
        {
            let stride_speed_start = prediction_mode.stride_context_speed_offset();
            let stride_max_start = prediction_mode.stride_context_speed_max_offset();
            for index in 0..512 {
                local_context_speeds[index + stride_speed_start] = prediction_mode.u16_to_f8(stride_speed[index >> 1][index & 1].0);
                local_context_speeds[index + stride_max_start] = prediction_mode.u16_to_f8(stride_speed[index >> 1][index & 1].1);
            }
        }
        {
            let cm_speed_start = prediction_mode.context_map_speed_offset();
            let cm_max_start = prediction_mode.context_map_speed_max_offset();
            for index in 0..512 {
                local_context_speeds[index + cm_speed_start] = prediction_mode.u16_to_f8(cm_speed[index >> 1][index & 1].0);
                local_context_speeds[index + cm_max_start] = prediction_mode.u16_to_f8(cm_speed[index >> 1][index & 1].1);
            }
        }
        {
            let combined_speed_start = prediction_mode.combined_stride_context_speed_offset();
            let combined_max_start = prediction_mode.combined_stride_context_speed_max_offset();
            for index in 0..512 {
                local_context_speeds[index + combined_speed_start] = prediction_mode.u16_to_f8(combined_speed[index >> 1][index & 1].0);
                local_context_speeds[index + combined_max_start] =  prediction_mode.u16_to_f8(combined_speed[index >> 1][index & 1].1);
            }
        }
        prediction_mode.context_speeds = InputReference(&local_context_speeds[..])
     }
     best_speed_log("CM", &cm_speed, &acost);
     best_speed_log("Stride", &stride_speed, &bcost);
     best_speed_log("StrideCombined", &combined_speed, &ccost);
     let mut command_queue = CommandQueue::new(m32, InputPair(input0, input1),
                                              params.stride_detection_quality,
                                               params.high_entropy_detection_quality,
                                               context_map_entropy,
                                               entropy_tally_scratch,
                                               entropy_pyramid);

    command_queue.push(interface::Command::PredictionMode(
        prediction_mode.clone()), callback);
     
    *recoder_state = process_command_queue(&mut command_queue,
                                           input,
                                           commands,
                                           n_postfix,
                                           n_direct,
                                           dist_cache,
                                           *recoder_state,
                                           &block_type,
                                           params,
                                           context_type,
                                           callback);
    command_queue.free(m16, m32, mf, callback);
//   ::std::io::stderr().write(input0).unwrap();
//   ::std::io::stderr().write(input1).unwrap();
}

static kBlockLengthPrefixCode: [PrefixCodeRange; BROTLI_NUM_BLOCK_LEN_SYMBOLS] =
  [PrefixCodeRange {
     offset: 1u32,
     nbits: 2u32,
   },
   PrefixCodeRange {
     offset: 5u32,
     nbits: 2u32,
   },
   PrefixCodeRange {
     offset: 9u32,
     nbits: 2u32,
   },
   PrefixCodeRange {
     offset: 13u32,
     nbits: 2u32,
   },
   PrefixCodeRange {
     offset: 17u32,
     nbits: 3u32,
   },
   PrefixCodeRange {
     offset: 25u32,
     nbits: 3u32,
   },
   PrefixCodeRange {
     offset: 33u32,
     nbits: 3u32,
   },
   PrefixCodeRange {
     offset: 41u32,
     nbits: 3u32,
   },
   PrefixCodeRange {
     offset: 49u32,
     nbits: 4u32,
   },
   PrefixCodeRange {
     offset: 65u32,
     nbits: 4u32,
   },
   PrefixCodeRange {
     offset: 81u32,
     nbits: 4u32,
   },
   PrefixCodeRange {
     offset: 97u32,
     nbits: 4u32,
   },
   PrefixCodeRange {
     offset: 113u32,
     nbits: 5u32,
   },
   PrefixCodeRange {
     offset: 145u32,
     nbits: 5u32,
   },
   PrefixCodeRange {
     offset: 177u32,
     nbits: 5u32,
   },
   PrefixCodeRange {
     offset: 209u32,
     nbits: 5u32,
   },
   PrefixCodeRange {
     offset: 241u32,
     nbits: 6u32,
   },
   PrefixCodeRange {
     offset: 305u32,
     nbits: 6u32,
   },
   PrefixCodeRange {
     offset: 369u32,
     nbits: 7u32,
   },
   PrefixCodeRange {
     offset: 497u32,
     nbits: 8u32,
   },
   PrefixCodeRange {
     offset: 753u32,
     nbits: 9u32,
   },
   PrefixCodeRange {
     offset: 1265u32,
     nbits: 10u32,
   },
   PrefixCodeRange {
     offset: 2289u32,
     nbits: 11u32,
   },
   PrefixCodeRange {
     offset: 4337u32,
     nbits: 12u32,
   },
   PrefixCodeRange {
     offset: 8433u32,
     nbits: 13u32,
   },
   PrefixCodeRange {
     offset: 16625u32,
     nbits: 24u32,
   }];

fn BrotliWriteBits(n_bits: u8, bits: u64, pos: &mut usize, array: &mut [u8]) {
  assert!((bits >> n_bits as usize) == 0);
  assert!(n_bits <= 56);
  let ptr_offset: usize = ((*pos >> 3) as u32) as usize;
  let mut v = array[ptr_offset] as u64;
  v |= bits << ((*pos) as u64 & 7);
  array[ptr_offset + 7] = (v >> 56) as u8;
  array[ptr_offset + 6] = ((v >> 48) & 0xff) as u8;
  array[ptr_offset + 5] = ((v >> 40) & 0xff) as u8;
  array[ptr_offset + 4] = ((v >> 32) & 0xff) as u8;
  array[ptr_offset + 3] = ((v >> 24) & 0xff) as u8;
  array[ptr_offset + 2] = ((v >> 16) & 0xff) as u8;
  array[ptr_offset + 1] = ((v >> 8) & 0xff) as u8;
  array[ptr_offset] = (v & 0xff) as u8;
  *pos += n_bits as usize
}

fn BrotliWriteBitsPrepareStorage(pos: usize, array: &mut [u8]) {
  assert_eq!(pos & 7, 0);
  array[pos >> 3] = 0;
}

fn BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(num_codes: i32,
                                                code_length_bitdepth: &[u8],
                                                storage_ix: &mut usize,
                                                storage: &mut [u8]) {
  static kStorageOrder: [u8; 18] = [1i32 as (u8),
                                    2i32 as (u8),
                                    3i32 as (u8),
                                    4i32 as (u8),
                                    0i32 as (u8),
                                    5i32 as (u8),
                                    17i32 as (u8),
                                    6i32 as (u8),
                                    16i32 as (u8),
                                    7i32 as (u8),
                                    8i32 as (u8),
                                    9i32 as (u8),
                                    10i32 as (u8),
                                    11i32 as (u8),
                                    12i32 as (u8),
                                    13i32 as (u8),
                                    14i32 as (u8),
                                    15i32 as (u8)];
  static kHuffmanBitLengthHuffmanCodeSymbols: [u8; 6] =
    [0i32 as (u8), 7i32 as (u8), 3i32 as (u8), 2i32 as (u8), 1i32 as (u8), 15i32 as (u8)];
  static kHuffmanBitLengthHuffmanCodeBitLengths: [u8; 6] =
    [2i32 as (u8), 4i32 as (u8), 3i32 as (u8), 2i32 as (u8), 2i32 as (u8), 4i32 as (u8)];
  let mut skip_some: u64 = 0u64;
  let mut codes_to_store: u64 = 18;
  if num_codes > 1i32 {
    'break5: while codes_to_store > 0 {
      {
        if code_length_bitdepth[(kStorageOrder[codes_to_store.wrapping_sub(1) as
            usize] as (usize))] as (i32) != 0i32 {
          {
            break 'break5;
          }
        }
      }
      codes_to_store = codes_to_store.wrapping_sub(1);
    }
  }
  if code_length_bitdepth[(kStorageOrder[0usize] as (usize))] as (i32) == 0i32 &&
     (code_length_bitdepth[(kStorageOrder[1usize] as (usize))] as (i32) == 0i32) {
    skip_some = 2;
    if code_length_bitdepth[(kStorageOrder[2usize] as (usize))] as (i32) == 0i32 {
      skip_some = 3;
    }
  }
  BrotliWriteBits(2, skip_some, storage_ix, storage);
  {
    let mut i: u64;
    i = skip_some;
    while i < codes_to_store {
      {
        let l: usize = code_length_bitdepth[(kStorageOrder[i as usize] as (usize))] as (usize);
        BrotliWriteBits(kHuffmanBitLengthHuffmanCodeBitLengths[l] as (u8),
                        kHuffmanBitLengthHuffmanCodeSymbols[l] as u64,
                        storage_ix,
                        storage);
      }
      i = i.wrapping_add(1);
    }
  }

}

fn BrotliStoreHuffmanTreeToBitMask(huffman_tree_size: usize,
                                   huffman_tree: &[u8],
                                   huffman_tree_extra_bits: &[u8],
                                   code_length_bitdepth: &[u8],
                                   code_length_bitdepth_symbols: &[u16],
                                   storage_ix: &mut usize,
                                   storage: &mut [u8]) {
  let mut i: usize;
  i = 0usize;
  while i < huffman_tree_size {
    {
      let ix: usize = huffman_tree[(i as (usize))] as (usize);
      BrotliWriteBits(code_length_bitdepth[(ix as (usize))] as (u8),
                      code_length_bitdepth_symbols[(ix as (usize))] as (u64),
                      storage_ix,
                      storage);
      if ix == 16usize {
        BrotliWriteBits(2,
                        huffman_tree_extra_bits[(i as (usize))] as (u64),
                        storage_ix,
                        storage);
      } else if ix == 17usize {
        BrotliWriteBits(3,
                        huffman_tree_extra_bits[(i as (usize))] as (u64),
                        storage_ix,
                        storage);
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
}

pub fn BrotliStoreHuffmanTree(depths: &[u8],
                              num: usize,
                              tree: &mut [HuffmanTree],
                              storage_ix: &mut usize,
                              storage: &mut [u8]) {
  let mut huffman_tree: [u8; 704] = [0; 704];
  let mut huffman_tree_extra_bits: [u8; 704] = [0; 704];
  let mut huffman_tree_size: usize = 0usize;
  let mut code_length_bitdepth: [u8; 18] = [0i32 as (u8),
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
                                            0i32 as (u8),
                                            0i32 as (u8)];
  let mut code_length_bitdepth_symbols: [u16; 18] = [0; 18];
  let mut huffman_tree_histogram: [u32; 18] = [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                                               0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                                               0u32, 0u32];
  let mut i: usize;
  let mut num_codes: i32 = 0i32;
  let mut code: usize = 0usize;
  0i32;
  BrotliWriteHuffmanTree(depths,
                         num,
                         &mut huffman_tree_size,
                         &mut huffman_tree[..],
                         &mut huffman_tree_extra_bits[..]);
  i = 0usize;
  while i < huffman_tree_size {
    {
      let _rhs = 1;
      let _lhs = &mut huffman_tree_histogram[huffman_tree[i] as (usize)];
      *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    i = i.wrapping_add(1 as (usize));
  }
  i = 0usize;
  'break3: while i < 18usize {
    {
      if huffman_tree_histogram[i] != 0 {
        if num_codes == 0i32 {
          code = i;
          num_codes = 1i32;
        } else if num_codes == 1i32 {
          num_codes = 2i32;
          {
            {
              break 'break3;
            }
          }
        }
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  BrotliCreateHuffmanTree(&mut huffman_tree_histogram,
                          18usize,
                          5i32,
                          tree,
                          &mut code_length_bitdepth);
  BrotliConvertBitDepthsToSymbols(&mut code_length_bitdepth,
                                  18usize,
                                  &mut code_length_bitdepth_symbols);
  BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(num_codes,
                                               &code_length_bitdepth,
                                               storage_ix,
                                               storage);
  if num_codes == 1i32 {
    code_length_bitdepth[code] = 0i32 as (u8);
  }
  BrotliStoreHuffmanTreeToBitMask(huffman_tree_size,
                                  &huffman_tree,
                                  &huffman_tree_extra_bits,
                                  &code_length_bitdepth,
                                  &code_length_bitdepth_symbols,
                                  storage_ix,
                                  storage);
}

fn StoreStaticCodeLengthCode(storage_ix: &mut usize, storage: &mut [u8]) {
  BrotliWriteBits(40,
                  0xffu32 as (u64) << 32i32 | 0x55555554u32 as (u64),
                  storage_ix,
                  storage);
}

pub struct SimpleSortHuffmanTree {}

impl HuffmanComparator for SimpleSortHuffmanTree {
  fn Cmp(self: &Self, v0: &HuffmanTree, v1: &HuffmanTree) -> bool {
    return (*v0).total_count_ < (*v1).total_count_;
  }
}

pub fn BrotliBuildAndStoreHuffmanTreeFast<AllocHT: alloc::Allocator<HuffmanTree>>(
    m : &mut AllocHT,
    histogram : &[u32],
    histogram_total : usize,
    max_bits : usize,
    depth : &mut [u8],
    bits : &mut [u16],
    storage_ix : &mut usize,
    storage : &mut [u8]
){
  let mut count: u64 = 0;
  let mut symbols: [u64; 4] = [0; 4];
  let mut length: u64 = 0;
  let mut total: usize = histogram_total;
  while total != 0usize {
    if histogram[(length as (usize))] != 0 {
      if count < 4 {
        symbols[count as usize] = length;
      }
      count = count.wrapping_add(1);
      total = total.wrapping_sub(histogram[(length as (usize))] as (usize));
    }
    length = length.wrapping_add(1);
  }
  if count <= 1 {
    BrotliWriteBits(4, 1, storage_ix, storage);
    BrotliWriteBits(max_bits as u8, symbols[0usize], storage_ix, storage);
    depth[symbols[0usize] as (usize)] = 0i32 as (u8);
    bits[symbols[0usize] as (usize)] = 0i32 as (u16);
    return ;
  }
  for depth_elem in depth[..(length as usize)].iter_mut() {
    *depth_elem = 0; // memset
  }
  {
    let max_tree_size: u64 = (2u64).wrapping_mul(length).wrapping_add(1);
    let mut tree = if max_tree_size != 0 {
      m.alloc_cell(max_tree_size as usize)
    } else {
      AllocHT::AllocatedMemory::default() // null
    };
    let mut count_limit: u32;
    if !(0i32 == 0) {
      return;
    }
    count_limit = 1u32;
    'break11: loop {
      {
        let mut node_index: u32 = 0u32;
        let mut l: u64;
        l = length;
        while l != 0 {
          l = l.wrapping_sub(1);
          if histogram[(l as (usize))] != 0 {
            if histogram[(l as (usize))] >= count_limit {
              InitHuffmanTree(&mut tree.slice_mut()[(node_index as (usize))],
                              histogram[(l as (usize))],
                              -1i32 as (i16),
                              l as (i16));
            } else {
              InitHuffmanTree(&mut tree.slice_mut()[(node_index as (usize))],
                              count_limit,
                              -1i32 as (i16),
                              l as (i16));
            }
            node_index = node_index.wrapping_add(1 as (u32));
          }
        }
        {
          let n: i32 = node_index as (i32);
          let sentinel: HuffmanTree;
          let mut i: i32 = 0i32;
          let mut j: i32 = n + 1i32;
          let mut k: i32;
          SortHuffmanTreeItems(tree.slice_mut(), n as (usize), SimpleSortHuffmanTree {});
          sentinel = NewHuffmanTree(!(0u32), -1i16, -1i16);
          tree.slice_mut()[(node_index.wrapping_add(1u32) as (usize))] = sentinel.clone();
          tree.slice_mut()[(node_index as (usize))] = sentinel.clone();
          node_index = node_index.wrapping_add(2u32);
          k = n - 1i32;
          while k > 0i32 {
            {
              let left: i32;
              let right: i32;
              if (tree.slice()[(i as (usize))]).total_count_ <=
                 (tree.slice()[(j as (usize))]).total_count_ {
                left = i;
                i = i + 1;
              } else {
                left = j;
                j = j + 1;
              }
              if (tree.slice()[(i as (usize))]).total_count_ <=
                 (tree.slice()[(j as (usize))]).total_count_ {
                right = i;
                i = i + 1;
              } else {
                right = j;
                j = j + 1;
              }
              let sum_total = (tree.slice()[(left as (usize))])
                .total_count_
                .wrapping_add((tree.slice()[(right as (usize))]).total_count_);
              let tree_ind = (node_index.wrapping_sub(1u32) as (usize));
              (tree.slice_mut()[tree_ind]).total_count_ = sum_total;
              (tree.slice_mut()[tree_ind]).index_left_ = left as (i16);
              (tree.slice_mut()[tree_ind]).index_right_or_value_ = right as (i16);
              tree.slice_mut()[(node_index as (usize))] = sentinel.clone();
              node_index = node_index.wrapping_add(1u32);
            }
            k = k - 1;
          }
          if BrotliSetDepth(2i32 * n - 1i32, tree.slice_mut(), depth, 14i32) {
            {
              break 'break11;
            }
          }
        }
      }
      count_limit = count_limit.wrapping_mul(2u32);
    }
    {
      m.free_cell(core::mem::replace(&mut tree, AllocHT::AllocatedMemory::default()));
    }
  }
  BrotliConvertBitDepthsToSymbols(depth, length as usize, bits);
  if count <= 4 {
    let mut i: u64;
    BrotliWriteBits(2, 1, storage_ix, storage);
    BrotliWriteBits(2, count.wrapping_sub(1) as u64, storage_ix, storage);
    i = 0;
    while i < count {
      {
        let mut j: u64;
        j = i.wrapping_add(1);
        while j < count {
          {
            if depth[(symbols[j as usize] as (usize))] as (i32) <
               depth[(symbols[i as usize] as (usize)) as usize] as (i32) {
              let brotli_swap_tmp: u64 = symbols[j as usize];
              symbols[j as usize] = symbols[i as usize];
              symbols[i as usize] = brotli_swap_tmp;
            }
          }
          j = j.wrapping_add(1);
        }
      }
      i = i.wrapping_add(1);
    }
    if count == 2 {
      BrotliWriteBits(max_bits as u8, symbols[0usize], storage_ix, storage);
      BrotliWriteBits(max_bits as u8, symbols[1usize], storage_ix, storage);
    } else if count == 3 {
      BrotliWriteBits(max_bits as u8, symbols[0usize], storage_ix, storage);
      BrotliWriteBits(max_bits as u8, symbols[1usize], storage_ix, storage);
      BrotliWriteBits(max_bits as u8, symbols[2usize], storage_ix, storage);
    } else {
      BrotliWriteBits(max_bits as u8, symbols[0usize], storage_ix, storage);
      BrotliWriteBits(max_bits as u8, symbols[1usize], storage_ix, storage);
      BrotliWriteBits(max_bits as u8, symbols[2usize], storage_ix, storage);
      BrotliWriteBits(max_bits as u8, symbols[3usize], storage_ix, storage);
      BrotliWriteBits(1,
                      if depth[(symbols[0usize] as (usize))] as (i32) == 1i32 {
                        1i32
                      } else {
                        0i32
                      } as (u64),
                      storage_ix,
                      storage);
    }
  } else {
    let mut previous_value: u8 = 8i32 as (u8);
    let mut i: u64;
    StoreStaticCodeLengthCode(storage_ix, storage);
    i = 0;
    while i < length {
      let value: u8 = depth[(i as (usize))];
      let mut reps: u64 = 1;
      let mut k: u64;
      k = i.wrapping_add(1);
      while k < length && (depth[(k as (usize))] as (i32) == value as (i32)) {
        {
          reps = reps.wrapping_add(1);
        }
        k = k.wrapping_add(1);
      }
      i = i.wrapping_add(reps);
      if value as (i32) == 0i32 {
        BrotliWriteBits(kZeroRepsDepth[reps as usize] as (u8),
                        kZeroRepsBits[reps as usize] as u64,
                        storage_ix,
                        storage);
      } else {
        if previous_value as (i32) != value as (i32) {
          BrotliWriteBits(kCodeLengthDepth[value as (usize)] as (u8),
                          kCodeLengthBits[value as (usize)] as (u64),
                          storage_ix,
                          storage);
          reps = reps.wrapping_sub(1);
        }
        if reps < 3 {
          while reps != 0 {
            reps = reps.wrapping_sub(1);
            BrotliWriteBits(kCodeLengthDepth[value as (usize)] as (u8),
                            kCodeLengthBits[value as (usize)] as (u64),
                            storage_ix,
                            storage);
          }
        } else {
          reps = reps.wrapping_sub(3);
          BrotliWriteBits(kNonZeroRepsDepth[reps as usize] as (u8),
                          kNonZeroRepsBits[reps as usize] as u64,
                          storage_ix,
                          storage);
        }
        previous_value = value;
      }
    }
  }
}

pub struct MetaBlockSplit<AllocU8: alloc::Allocator<u8>,
                          AllocU32: alloc::Allocator<u32>,
                          AllocHL: alloc::Allocator<HistogramLiteral>,
                          AllocHC: alloc::Allocator<HistogramCommand>,
                          AllocHD: alloc::Allocator<HistogramDistance>>
{
  pub literal_split: BlockSplit<AllocU8, AllocU32>,
  pub command_split: BlockSplit<AllocU8, AllocU32>,
  pub distance_split: BlockSplit<AllocU8, AllocU32>,
  pub literal_context_map: AllocU32::AllocatedMemory,
  pub literal_context_map_size: usize,
  pub distance_context_map: AllocU32::AllocatedMemory,
  pub distance_context_map_size: usize,
  pub literal_histograms: AllocHL::AllocatedMemory,
  pub literal_histograms_size: usize,
  pub command_histograms: AllocHC::AllocatedMemory,
  pub command_histograms_size: usize,
  pub distance_histograms: AllocHD::AllocatedMemory,
  pub distance_histograms_size: usize,
}
impl <AllocU8: alloc::Allocator<u8>,
                          AllocU32: alloc::Allocator<u32>,
                          AllocHL: alloc::Allocator<HistogramLiteral>,
                          AllocHC: alloc::Allocator<HistogramCommand>,
      AllocHD: alloc::Allocator<HistogramDistance>>
    MetaBlockSplit <AllocU8, AllocU32, AllocHL, AllocHC, AllocHD> {
    pub fn new() -> Self {
        return MetaBlockSplit {
            literal_split:BlockSplit::<AllocU8, AllocU32>::new(),
            command_split:BlockSplit::<AllocU8, AllocU32>::new(),
            distance_split:BlockSplit::<AllocU8, AllocU32>::new(),
            literal_context_map : AllocU32::AllocatedMemory::default(),
            literal_context_map_size : 0,
            distance_context_map : AllocU32::AllocatedMemory::default(),
            distance_context_map_size : 0,
            literal_histograms : AllocHL::AllocatedMemory::default(),
            literal_histograms_size : 0,
            command_histograms : AllocHC::AllocatedMemory::default(),
            command_histograms_size : 0,
            distance_histograms : AllocHD::AllocatedMemory::default(),
            distance_histograms_size : 0,
        }
    }
    pub fn destroy(&mut self,
                   m8: &mut AllocU8,
                   m32: &mut AllocU32,
                   mhl: &mut AllocHL,
                   mhc: &mut AllocHC,
                   mhd: &mut AllocHD) {
        self.literal_split.destroy(m8,m32);
        self.command_split.destroy(m8,m32);
        self.distance_split.destroy(m8,m32);
        m32.free_cell(core::mem::replace(&mut self.literal_context_map,
                                         AllocU32::AllocatedMemory::default()));
        self.literal_context_map_size = 0;
        m32.free_cell(core::mem::replace(&mut self.distance_context_map,
                                         AllocU32::AllocatedMemory::default()));
        self.distance_context_map_size = 0;
        mhl.free_cell(core::mem::replace(&mut self.literal_histograms,
                                         AllocHL::AllocatedMemory::default()));

        self.literal_histograms_size = 0;
        mhc.free_cell(core::mem::replace(&mut self.command_histograms,
                                         AllocHC::AllocatedMemory::default()));
        self.command_histograms_size = 0;
        mhd.free_cell(core::mem::replace(&mut self.distance_histograms,
                                         AllocHD::AllocatedMemory::default()));
        self.distance_histograms_size = 0;
    }
}
#[derive(Clone, Copy)]
pub struct BlockTypeCodeCalculator {
  pub last_type: usize,
  pub second_last_type: usize,
}

pub struct BlockSplitCode {
  pub type_code_calculator: BlockTypeCodeCalculator,
  pub type_depths: [u8; 258],
  pub type_bits: [u16; 258],
  pub length_depths: [u8; 26],
  pub length_bits: [u16; 26],
}

pub struct BlockEncoder<'a, AllocU8: alloc::Allocator<u8>, AllocU16: alloc::Allocator<u16>> {
  /*    pub alloc_u8 : AllocU8,
    pub alloc_u16 : AllocU16,
    pub alloc_u32 : AllocU32,
    pub alloc_ht : AllocHT,*/
  pub alphabet_size_: usize,
  pub num_block_types_: usize,
  pub block_types_: &'a [u8],
  pub block_lengths_: &'a [u32],
  pub num_blocks_: usize,
  pub block_split_code_: BlockSplitCode,
  pub block_ix_: usize,
  pub block_len_: usize,
  pub entropy_ix_: usize,
  pub depths_: AllocU8::AllocatedMemory,
  pub bits_: AllocU16::AllocatedMemory,
}

fn Log2FloorNonZero(mut n: u64) -> u32 {
  let mut result: u32 = 0u32;
  'loop1: loop {
    if {
         n = n >> 1i32;
         n
       } != 0 {
      result = result.wrapping_add(1 as (u32));
      continue 'loop1;
    } else {
      break 'loop1;
    }
  }
  result
}

fn BrotliEncodeMlen(length: u32,
                    bits: &mut u64,
                    numbits: &mut u32,
                    nibblesbits: &mut u32) {
  let lg: u32 = (if length == 1u32 {
                   1u32
                 } else {
                   Log2FloorNonZero(length.wrapping_sub(1u32) as (u32) as (u64)).wrapping_add(1u32)
                 }) as (u32);
  let mnibbles: u32 = (if lg < 16u32 {
       16u32
     } else {
       lg.wrapping_add(3u32)
     })
    .wrapping_div(4u32);
  assert!(length > 0);
  assert!(length <= (1 << 24));
  assert!(lg <= 24);
  *nibblesbits = mnibbles.wrapping_sub(4u32);
  *numbits = mnibbles.wrapping_mul(4u32);
  *bits = length.wrapping_sub(1u32) as u64;
}


fn StoreCompressedMetaBlockHeader(is_final_block: i32,
                                  length: usize,
                                  storage_ix: &mut usize,
                                  storage: &mut [u8]) {
  let mut lenbits: u64 = 0;
  let mut nlenbits: u32 = 0;
  let mut nibblesbits: u32 = 0;
  BrotliWriteBits(1, is_final_block as (u64), storage_ix, storage);
  if is_final_block != 0 {
    BrotliWriteBits(1, 0, storage_ix, storage);
  }
  BrotliEncodeMlen(length as u32, &mut lenbits, &mut nlenbits, &mut nibblesbits);
  BrotliWriteBits(2, nibblesbits as u64, storage_ix, storage);
  BrotliWriteBits(nlenbits as u8, lenbits, storage_ix, storage);
  if is_final_block == 0 {
    BrotliWriteBits(1, 0, storage_ix, storage);
  }
}

fn NewBlockTypeCodeCalculator() -> BlockTypeCodeCalculator {
  return BlockTypeCodeCalculator {
           last_type: 1,
           second_last_type: 0,
         };
}

fn NewBlockEncoder<'a, AllocU8: alloc::Allocator<u8>, AllocU16: alloc::Allocator<u16>>
  (alphabet_size: usize,
   num_block_types: usize,
   block_types: &'a [u8],
   block_lengths: &'a [u32],
   num_blocks: usize)
   -> BlockEncoder<'a, AllocU8, AllocU16> {
  let block_len: usize;
  if num_blocks != 0 && block_lengths.len() != 0 {
    block_len = block_lengths[0] as usize;
  } else {
    block_len = 0;
  }
  return BlockEncoder::<AllocU8, AllocU16> {
           alphabet_size_: alphabet_size,
           num_block_types_: num_block_types,
           block_types_: block_types,
           block_lengths_: block_lengths,
           num_blocks_: num_blocks,
           block_split_code_: BlockSplitCode {
             type_code_calculator: NewBlockTypeCodeCalculator(),
             type_depths: [0; 258],
             type_bits: [0; 258],
             length_depths: [0; 26],
             length_bits: [0; 26],
           },
           block_ix_: 0,
           block_len_: block_len,
           entropy_ix_: 0,
           depths_: AllocU8::AllocatedMemory::default(),
           bits_: AllocU16::AllocatedMemory::default(),
         };
}





fn NextBlockTypeCode(calculator: &mut BlockTypeCodeCalculator, type_: u8) -> usize {
  let type_code: usize = (if type_ as (usize) ==
                             (*calculator).last_type.wrapping_add(1usize) {
                            1u32
                          } else if type_ as (usize) == (*calculator).second_last_type {
    0u32
  } else {
    (type_ as (u32)).wrapping_add(2u32)
  }) as (usize);
  (*calculator).second_last_type = (*calculator).last_type;
  (*calculator).last_type = type_ as (usize);
  type_code
}

fn BlockLengthPrefixCode(len: u32) -> u32 {
  let mut code: u32 = (if len >= 177u32 {
                         if len >= 753u32 { 20i32 } else { 14i32 }
                       } else if len >= 41u32 {
    7i32
  } else {
    0i32
  }) as (u32);
  while code < (26i32 - 1i32) as (u32) &&
        (len >= kBlockLengthPrefixCode[code.wrapping_add(1u32) as (usize)].offset) {
    code = code.wrapping_add(1 as (u32));
  }
  code
}

fn StoreVarLenUint8(n: u64, storage_ix: &mut usize, storage: &mut [u8]) {
  if n == 0 {
    BrotliWriteBits(1, 0, storage_ix, storage);
  } else {
    let nbits: u8 = Log2FloorNonZero(n) as (u8);
    BrotliWriteBits(1, 1, storage_ix, storage);
    BrotliWriteBits(3, nbits as u64, storage_ix, storage);
    BrotliWriteBits(nbits, n.wrapping_sub(1u64 << nbits), storage_ix, storage);
  }
}


fn StoreSimpleHuffmanTree(depths: &[u8],
                          symbols: &mut [usize],
                          num_symbols: usize,
                          max_bits: usize,
                          storage_ix: &mut usize,
                          storage: &mut [u8]) {
  BrotliWriteBits(2, 1, storage_ix, storage);
  BrotliWriteBits(2, num_symbols.wrapping_sub(1) as u64, storage_ix, storage);
  {
    let mut i: usize;
    i = 0usize;
    while i < num_symbols {
      {
        let mut j: usize;
        j = i.wrapping_add(1usize);
        while j < num_symbols {
          {
            if depths[(symbols[(j as (usize))] as (usize))] as (i32) <
               depths[(symbols[(i as (usize))] as (usize))] as (i32) {
              let mut __brotli_swap_tmp: usize = symbols[(j as (usize))];
              symbols[(j as (usize))] = symbols[(i as (usize))];
              symbols[(i as (usize))] = __brotli_swap_tmp;
            }
          }
          j = j.wrapping_add(1 as (usize));
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
  }
  if num_symbols == 2usize {
    BrotliWriteBits(max_bits as u8,
                    symbols[(0usize)] as u64,
                    storage_ix,
                    storage);
    BrotliWriteBits(max_bits as u8,
                    symbols[(1usize)] as u64,
                    storage_ix,
                    storage);
  } else if num_symbols == 3usize {
    BrotliWriteBits(max_bits as u8,
                    symbols[(0usize)] as u64,
                    storage_ix,
                    storage);
    BrotliWriteBits(max_bits as u8,
                    symbols[(1usize)] as u64,
                    storage_ix,
                    storage);
    BrotliWriteBits(max_bits as u8,
                    symbols[(2usize)] as u64,
                    storage_ix,
                    storage);
  } else {
    BrotliWriteBits(max_bits as u8,
                    symbols[(0usize)] as u64,
                    storage_ix,
                    storage);
    BrotliWriteBits(max_bits as u8,
                    symbols[(1usize)] as u64,
                    storage_ix,
                    storage);
    BrotliWriteBits(max_bits as u8,
                    symbols[(2usize)] as u64,
                    storage_ix,
                    storage);
    BrotliWriteBits(max_bits as u8,
                    symbols[(3usize)] as u64,
                    storage_ix,
                    storage);
    BrotliWriteBits(1,
                    if depths[(symbols[(0usize)] as (usize))] as (i32) == 1i32 {
                      1i32
                    } else {
                      0i32
                    } as (u64),
                    storage_ix,
                    storage);
  }
}

fn BuildAndStoreHuffmanTree(histogram: &[u32],
                            length: usize,
                            tree: &mut [HuffmanTree],
                            depth: &mut [u8],
                            bits: &mut [u16],
                            storage_ix: &mut usize,
                            storage: &mut [u8]) {
  let mut count: usize = 0usize;
  let mut s4: [usize; 4] = [0usize, 0usize, 0usize, 0usize];
  let mut i: usize;
  let mut max_bits: usize = 0usize;
  i = 0usize;
  'break31: while i < length {
    {
      if histogram[(i as (usize))] != 0 {
        if count < 4usize {
          s4[count] = i;
        } else if count > 4usize {
          {
            break 'break31;
          }
        }
        count = count.wrapping_add(1 as (usize));
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  {
    let mut max_bits_counter: usize = length.wrapping_sub(1usize);
    while max_bits_counter != 0 {
      max_bits_counter = max_bits_counter >> 1i32;
      max_bits = max_bits.wrapping_add(1 as (usize));
    }
  }
  if count <= 1usize {
    BrotliWriteBits(4, 1, storage_ix, storage);
    BrotliWriteBits(max_bits as u8, s4[0usize] as u64, storage_ix, storage);
    depth[(s4[0usize] as (usize))] = 0i32 as (u8);
    bits[(s4[0usize] as (usize))] = 0i32 as (u16);
    return;
  }

  for depth_elem in depth[..length].iter_mut() {
    *depth_elem = 0; // memset
  }
  BrotliCreateHuffmanTree(histogram, length, 15i32, tree, depth);
  BrotliConvertBitDepthsToSymbols(depth, length, bits);
  if count <= 4usize {
    StoreSimpleHuffmanTree(depth, &mut s4[..], count, max_bits, storage_ix, storage);
  } else {
    BrotliStoreHuffmanTree(depth, length, tree, storage_ix, storage);
  }
}

fn GetBlockLengthPrefixCode(len: u32,
                            code: &mut usize,
                            n_extra: &mut u32,
                            extra: &mut u32) {
  *code = BlockLengthPrefixCode(len) as (usize);
  *n_extra = kBlockLengthPrefixCode[*code].nbits;
  *extra = len.wrapping_sub(kBlockLengthPrefixCode[*code].offset);
}

fn StoreBlockSwitch(code: &mut BlockSplitCode,
                    block_len: u32,
                    block_type: u8,
                    is_first_block: i32,
                    storage_ix: &mut usize,
                    storage: &mut [u8]) {
  let typecode: usize = NextBlockTypeCode(&mut (*code).type_code_calculator, block_type);
  let mut lencode: usize = 0;
  let mut len_nextra: u32 = 0;
  let mut len_extra: u32 = 0;
  if is_first_block == 0 {
    BrotliWriteBits((*code).type_depths[typecode] as (u8),
                    (*code).type_bits[typecode] as (u64),
                    storage_ix,
                    storage);
  }
  GetBlockLengthPrefixCode(block_len, &mut lencode, &mut len_nextra, &mut len_extra);
  BrotliWriteBits((*code).length_depths[lencode] as (u8),
                  (*code).length_bits[lencode] as (u64),
                  storage_ix,
                  storage);
  BrotliWriteBits(len_nextra as (u8), len_extra as (u64), storage_ix, storage);
}

fn BuildAndStoreBlockSplitCode(types: &[u8],
                               lengths: &[u32],
                               num_blocks: usize,
                               num_types: usize,
                               tree: &mut [HuffmanTree],
                               code: &mut BlockSplitCode,
                               storage_ix: &mut usize,
                               storage: &mut [u8]) {
  let mut type_histo: [u32; 258] = [0; 258];
  let mut length_histo: [u32; 26] = [0; 26];
  let mut i: usize;
  let mut type_code_calculator = NewBlockTypeCodeCalculator();
  i = 0usize;
  while i < num_blocks {
    {
      let type_code: usize = NextBlockTypeCode(&mut type_code_calculator, types[(i as (usize))]);
      if i != 0usize {
        let _rhs = 1;
        let _lhs = &mut type_histo[type_code];
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
      }
      {
        let _rhs = 1;
        let _lhs = &mut length_histo[BlockLengthPrefixCode(lengths[(i as (usize))]) as (usize)];
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  StoreVarLenUint8(num_types.wrapping_sub(1) as u64, storage_ix, storage);
  if num_types > 1usize {
    BuildAndStoreHuffmanTree(&mut type_histo[0usize..],
                             num_types.wrapping_add(2usize),
                             tree,
                             &mut (*code).type_depths[0usize..],
                             &mut (*code).type_bits[0usize..],
                             storage_ix,
                             storage);
    BuildAndStoreHuffmanTree(&mut length_histo[0usize..],
                             26usize,
                             tree,
                             &mut (*code).length_depths[0usize..],
                             &mut (*code).length_bits[0usize..],
                             storage_ix,
                             storage);
    StoreBlockSwitch(code,
                     lengths[(0usize)],
                     types[(0usize)],
                     1i32,
                     storage_ix,
                     storage);
  }
}

fn BuildAndStoreBlockSwitchEntropyCodes<'a,
                                        AllocU8: alloc::Allocator<u8>,
                                        AllocU16: alloc::Allocator<u16>>
  (xself: &mut BlockEncoder<'a, AllocU8, AllocU16>,
   tree: &mut [HuffmanTree],
   storage_ix: &mut usize,
   storage: &mut [u8]) {
  BuildAndStoreBlockSplitCode((*xself).block_types_,
                              (*xself).block_lengths_,
                              (*xself).num_blocks_,
                              (*xself).num_block_types_,
                              tree,
                              &mut (*xself).block_split_code_,
                              storage_ix,
                              storage);
}

fn StoreTrivialContextMap(num_types: usize,
                          context_bits: usize,
                          tree: &mut [HuffmanTree],
                          storage_ix: &mut usize,
                          storage: &mut [u8]) {
  StoreVarLenUint8(num_types.wrapping_sub(1usize) as u64, storage_ix, storage);
  if num_types > 1usize {
    let repeat_code: usize = context_bits.wrapping_sub(1u32 as (usize));
    let repeat_bits: usize = (1u32 << repeat_code).wrapping_sub(1u32) as (usize);
    let alphabet_size: usize = num_types.wrapping_add(repeat_code);
    let mut histogram: [u32; 272] = [0; 272];
    let mut depths: [u8; 272] = [0; 272];
    let mut bits: [u16; 272] = [0; 272];
    let mut i: usize;
    BrotliWriteBits(1u8, 1u64, storage_ix, storage);
    BrotliWriteBits(4u8,
                    repeat_code.wrapping_sub(1usize) as u64,
                    storage_ix,
                    storage);
    histogram[repeat_code] = num_types as (u32);
    histogram[0usize] = 1u32;
    i = context_bits;
    while i < alphabet_size {
      {
        histogram[i] = 1u32;
      }
      i = i.wrapping_add(1 as (usize));
    }
    BuildAndStoreHuffmanTree(&mut histogram[..],
                             alphabet_size,
                             tree,
                             &mut depths[..],
                             &mut bits[..],
                             storage_ix,
                             storage);
    i = 0usize;
    while i < num_types {
      {
        let code: usize = if i == 0usize {
          0usize
        } else {
          i.wrapping_add(context_bits).wrapping_sub(1usize)
        };
        BrotliWriteBits(depths[code] as (u8),
                        bits[code] as (u64),
                        storage_ix,
                        storage);
        BrotliWriteBits(depths[repeat_code] as (u8),
                        bits[repeat_code] as (u64),
                        storage_ix,
                        storage);
        BrotliWriteBits(repeat_code as u8, repeat_bits as u64, storage_ix, storage);
      }
      i = i.wrapping_add(1 as (usize));
    }
    BrotliWriteBits(1, 1, storage_ix, storage);
  }
}

fn IndexOf(v: &[u8], v_size: usize, value: u8) -> usize {
  let mut i: usize = 0usize;
  while i < v_size {
    {
      if v[(i as (usize))] as (i32) == value as (i32) {
        return i;
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  i
}

fn MoveToFront(v: &mut [u8], index: usize) {
  let value: u8 = v[(index as (usize))];
  let mut i: usize;
  i = index;
  while i != 0usize {
    {
      v[(i as (usize))] = v[(i.wrapping_sub(1usize) as (usize))];
    }
    i = i.wrapping_sub(1 as (usize));
  }
  v[(0usize)] = value;
}

fn MoveToFrontTransform(v_in: &[u32], v_size: usize, v_out: &mut [u32]) {
  let mut i: usize;
  let mut mtf: [u8; 256] = [0; 256];
  let mut max_value: u32;
  if v_size == 0usize {
    return;
  }
  max_value = v_in[(0usize)];
  i = 1usize;
  while i < v_size {
    {
      if v_in[(i as (usize))] > max_value {
        max_value = v_in[(i as (usize))];
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  0i32;
  i = 0usize;
  while i <= max_value as (usize) {
    {
      mtf[i] = i as (u8);
    }
    i = i.wrapping_add(1 as (usize));
  }
  {
    let mtf_size: usize = max_value.wrapping_add(1u32) as (usize);
    i = 0usize;
    while i < v_size {
      {
        let index: usize = IndexOf(&mtf[..], mtf_size, v_in[(i as (usize))] as (u8));
        0i32;
        v_out[(i as (usize))] = index as (u32);
        MoveToFront(&mut mtf[..], index);
      }
      i = i.wrapping_add(1 as (usize));
    }
  }
}

fn brotli_max_uint32_t(a: u32, b: u32) -> u32 {
  if a > b { a } else { b }
}

fn brotli_min_uint32_t(a: u32, b: u32) -> u32 {
  if a < b { a } else { b }
}

fn RunLengthCodeZeros(in_size: usize,
                      v: &mut [u32],
                      out_size: &mut usize,
                      max_run_length_prefix: &mut u32) {
  let mut max_reps: u32 = 0u32;
  let mut i: usize;
  let mut max_prefix: u32;
  i = 0usize;
  while i < in_size {
    let mut reps: u32 = 0u32;
    while i < in_size && (v[(i as (usize))] != 0u32) {
      i = i.wrapping_add(1 as (usize));
    }
    while i < in_size && (v[(i as (usize))] == 0u32) {
      {
        reps = reps.wrapping_add(1 as (u32));
      }
      i = i.wrapping_add(1 as (usize));
    }
    max_reps = brotli_max_uint32_t(reps, max_reps);
  }
  max_prefix = if max_reps > 0u32 {
    Log2FloorNonZero(max_reps as (u64))
  } else {
    0u32
  };
  max_prefix = brotli_min_uint32_t(max_prefix, *max_run_length_prefix);
  *max_run_length_prefix = max_prefix;
  *out_size = 0usize;
  i = 0usize;
  while i < in_size {
    0i32;
    if v[(i as (usize))] != 0u32 {
      v[(*out_size as (usize))] = (v[(i as (usize))]).wrapping_add(*max_run_length_prefix);
      i = i.wrapping_add(1 as (usize));
      *out_size = (*out_size).wrapping_add(1 as (usize));
    } else {
      let mut reps: u32 = 1u32;
      let mut k: usize;
      k = i.wrapping_add(1usize);
      while k < in_size && (v[(k as (usize))] == 0u32) {
        {
          reps = reps.wrapping_add(1 as (u32));
        }
        k = k.wrapping_add(1 as (usize));
      }
      i = i.wrapping_add(reps as (usize));
      while reps != 0u32 {
        if reps < 2u32 << max_prefix {
          let run_length_prefix: u32 = Log2FloorNonZero(reps as (u64));
          let extra_bits: u32 = reps.wrapping_sub(1u32 << run_length_prefix);
          v[(*out_size as (usize))] = run_length_prefix.wrapping_add(extra_bits << 9i32);
          *out_size = (*out_size).wrapping_add(1 as (usize));
          {
            {
              break;
            }
          }
        } else {
          let extra_bits: u32 = (1u32 << max_prefix).wrapping_sub(1u32);
          v[(*out_size as (usize))] = max_prefix.wrapping_add(extra_bits << 9i32);
          reps = reps.wrapping_sub((2u32 << max_prefix).wrapping_sub(1u32));
          *out_size = (*out_size).wrapping_add(1 as (usize));
        }
      }
    }
  }
}

fn EncodeContextMap<AllocU32: alloc::Allocator<u32>>(m: &mut AllocU32,
                                                     context_map: &[u32],
                                                     context_map_size: usize,
                                                     num_clusters: usize,
                                                     tree: &mut [HuffmanTree],
                                                     storage_ix: &mut usize,
                                                     storage: &mut [u8]) {
  let mut i: usize;
  let mut rle_symbols: AllocU32::AllocatedMemory;
  let mut max_run_length_prefix: u32 = 6u32;
  let mut num_rle_symbols: usize = 0usize;
  static kSymbolMask: u32 = (1u32 << 9i32) - 1;
  let mut depths: [u8; 272] = [0; 272];
  let mut bits: [u16; 272] = [0; 272];
  StoreVarLenUint8(num_clusters.wrapping_sub(1usize) as u64,
                   storage_ix,
                   storage);
  if num_clusters == 1usize {
    return;
  }
  rle_symbols = if context_map_size != 0 {
    m.alloc_cell(context_map_size)
  } else {
    AllocU32::AllocatedMemory::default()
  };  
  MoveToFrontTransform(context_map, context_map_size, rle_symbols.slice_mut());
  RunLengthCodeZeros(context_map_size,
                     rle_symbols.slice_mut(),
                     &mut num_rle_symbols,
                     &mut max_run_length_prefix);
  let mut histogram: [u32; 272] = [0; 272];
  i = 0usize;
  while i < num_rle_symbols {
    {
      let _rhs = 1;
      let _lhs = &mut histogram[(rle_symbols.slice()[(i as (usize))] & kSymbolMask) as (usize)];
      *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    i = i.wrapping_add(1 as (usize));
  }
  {
    let use_rle: i32 = if !!(max_run_length_prefix > 0u32) {
      1i32
    } else {
      0i32
    };
    BrotliWriteBits(1, use_rle as (u64), storage_ix, storage);
    if use_rle != 0 {
      BrotliWriteBits(4,
                      max_run_length_prefix.wrapping_sub(1u32) as (u64),
                      storage_ix,
                      storage);
    }
  }
  BuildAndStoreHuffmanTree(&mut histogram[..],
                           num_clusters.wrapping_add(max_run_length_prefix as (usize)),
                           tree,
                           &mut depths[..],
                           &mut bits[..],
                           storage_ix,
                           storage);
  i = 0usize;
  while i < num_rle_symbols {
    {
      let rle_symbol: u32 = rle_symbols.slice()[(i as (usize))] & kSymbolMask;
      let extra_bits_val: u32 = rle_symbols.slice()[(i as (usize))] >> 9i32;
      BrotliWriteBits(depths[rle_symbol as (usize)] as (u8),
                      bits[rle_symbol as (usize)] as (u64),
                      storage_ix,
                      storage);
      if rle_symbol > 0u32 && (rle_symbol <= max_run_length_prefix) {
        BrotliWriteBits(rle_symbol as (u8),
                        extra_bits_val as (u64),
                        storage_ix,
                        storage);
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  BrotliWriteBits(1, 1, storage_ix, storage);
  m.free_cell(rle_symbols);
}

fn BuildAndStoreEntropyCodes<AllocU8: alloc::Allocator<u8>,
                             AllocU16: alloc::Allocator<u16>,
                             HistogramType: SliceWrapper<u32>>
  (m8: &mut AllocU8,
   m16: &mut AllocU16,
   xself: &mut BlockEncoder<AllocU8, AllocU16>,
   histograms: &[HistogramType],
   histograms_size: usize,
   tree: &mut [HuffmanTree],
   storage_ix: &mut usize,
   storage: &mut [u8]) {
  let alphabet_size: usize = (*xself).alphabet_size_;
  let table_size: usize = histograms_size.wrapping_mul(alphabet_size);
  (*xself).depths_ = if table_size != 0 {
    m8.alloc_cell(table_size)
  } else {
    AllocU8::AllocatedMemory::default()
  };
  (*xself).bits_ = if table_size != 0 {
    m16.alloc_cell(table_size)
  } else {
    AllocU16::AllocatedMemory::default()
  };
  {
    let mut i: usize;
    i = 0usize;
    while i < histograms_size {
      {
        let ix: usize = i.wrapping_mul(alphabet_size);
        BuildAndStoreHuffmanTree(&(histograms[(i as (usize))]).slice()[0..],
                                 alphabet_size,
                                 tree,
                                 &mut (*xself).depths_.slice_mut()[(ix as (usize))..],
                                 &mut (*xself).bits_.slice_mut()[(ix as (usize))..],
                                 storage_ix,
                                 storage);
      }
      i = i.wrapping_add(1 as (usize));
    }
  }
}

fn StoreSymbol<AllocU8: alloc::Allocator<u8>,
               AllocU16: alloc::Allocator<u16>>
               (xself: &mut BlockEncoder<AllocU8, AllocU16>,
               symbol: usize,
               storage_ix: &mut usize,
               storage: &mut [u8]){
  if (*xself).block_len_ == 0usize {
    let block_ix: usize = {
      (*xself).block_ix_ = (*xself).block_ix_.wrapping_add(1 as (usize));
      (*xself).block_ix_
    };
    let block_len: u32 = (*xself).block_lengths_[(block_ix as (usize))];
    let block_type: u8 = (*xself).block_types_[(block_ix as (usize))];
    (*xself).block_len_ = block_len as (usize);
    (*xself).entropy_ix_ = (block_type as (usize)).wrapping_mul((*xself).alphabet_size_);
    StoreBlockSwitch(&mut (*xself).block_split_code_,
                     block_len,
                     block_type,
                     0i32,
                     storage_ix,
                     storage);
  }
  (*xself).block_len_ = (*xself).block_len_.wrapping_sub(1 as (usize));
  {
    let ix: usize = (*xself).entropy_ix_.wrapping_add(symbol);
    BrotliWriteBits((*xself).depths_.slice()[(ix as (usize))] as (u8),
                    (*xself).bits_.slice()[(ix as (usize))] as (u64),
                    storage_ix,
                    storage);
  }
}

fn CommandCopyLenCode(xself: &Command) -> u32 {
  (*xself).copy_len_ & 0xffffffu32 ^ (*xself).copy_len_ >> 24i32
}
fn GetInsertExtra(inscode: u16) -> u32 {
  kInsExtra[inscode as (usize)]
}

fn GetInsertBase(inscode: u16) -> u32 {
  kInsBase[inscode as (usize)]
}

fn GetCopyBase(copycode: u16) -> u32 {
  kCopyBase[copycode as (usize)]
}

fn GetCopyExtra(copycode: u16) -> u32 {
  kCopyExtra[copycode as (usize)]
}

fn StoreCommandExtra(cmd: &Command, storage_ix: &mut usize, storage: &mut [u8]) {
  let copylen_code: u32 = CommandCopyLenCode(cmd);
  let inscode: u16 = GetInsertLengthCode((*cmd).insert_len_ as (usize));
  let copycode: u16 = GetCopyLengthCode(copylen_code as (usize));
  let insnumextra: u32 = GetInsertExtra(inscode);
  let insextraval: u64 = (*cmd).insert_len_.wrapping_sub(GetInsertBase(inscode)) as (u64);
  let copyextraval: u64 = copylen_code.wrapping_sub(GetCopyBase(copycode)) as (u64);
  let bits: u64 = copyextraval << insnumextra | insextraval;
  BrotliWriteBits(insnumextra.wrapping_add(GetCopyExtra(copycode)) as (u8),
                  bits,
                  storage_ix,
                  storage);
}

fn Context(p1: u8, p2: u8, mode: ContextType) -> u8 {
  match mode {
    ContextType::CONTEXT_LSB6 => {
      return (p1 as (i32) & 0x3fi32) as (u8);
    }
    ContextType::CONTEXT_MSB6 => {
      return (p1 as (i32) >> 2i32) as (u8);
    }
    ContextType::CONTEXT_UTF8 => {
      return (kUTF8ContextLookup[p1 as (usize)] as (i32) |
              kUTF8ContextLookup[(p2 as (i32) + 256i32) as (usize)] as (i32)) as (u8);
    }
    ContextType::CONTEXT_SIGNED => {
      return ((kSigned3BitContextLookup[p1 as (usize)] as (i32) << 3i32) +
              kSigned3BitContextLookup[p2 as (usize)] as (i32)) as (u8);
    }
  }
  //  0i32 as (u8)
}

fn StoreSymbolWithContext<AllocU8: alloc::Allocator<u8>,
                          AllocU16: alloc::Allocator<u16>>(xself: &mut BlockEncoder<AllocU8,
                                                                                        AllocU16>,
                                                           symbol: usize,
                                                           context: usize,
                                                           context_map: &[u32],
                                                           storage_ix: &mut usize,
                                                           storage: &mut [u8],
context_bits: usize){
  if (*xself).block_len_ == 0usize {
    let block_ix: usize = {
      (*xself).block_ix_ = (*xself).block_ix_.wrapping_add(1 as (usize));
      (*xself).block_ix_
    };
    let block_len: u32 = (*xself).block_lengths_[(block_ix as (usize))];
    let block_type: u8 = (*xself).block_types_[(block_ix as (usize))];
    (*xself).block_len_ = block_len as (usize);
    (*xself).entropy_ix_ = block_type as (usize) << context_bits;
    StoreBlockSwitch(&mut (*xself).block_split_code_,
                     block_len,
                     block_type,
                     0i32,
                     storage_ix,
                     storage);
  }
  (*xself).block_len_ = (*xself).block_len_.wrapping_sub(1 as (usize));
  {
    let histo_ix: usize = context_map[((*xself).entropy_ix_.wrapping_add(context) as (usize))] as
                          (usize);
    let ix: usize = histo_ix.wrapping_mul((*xself).alphabet_size_).wrapping_add(symbol);
    BrotliWriteBits((*xself).depths_.slice()[(ix as (usize))] as (u8),
                    (*xself).bits_.slice()[(ix as (usize))] as (u64),
                    storage_ix,
                    storage);
  }
}

fn CommandCopyLen(xself: &Command) -> u32 {
  (*xself).copy_len_ & 0xffffffu32
}

fn CommandDistanceContext(xself: &Command) -> u32 {
  let r: u32 = ((*xself).cmd_prefix_ as (i32) >> 6i32) as (u32);
  let c: u32 = ((*xself).cmd_prefix_ as (i32) & 7i32) as (u32);
  if (r == 0u32 || r == 2u32 || r == 4u32 || r == 7u32) && (c <= 2u32) {
    return c;
  }
  3u32
}

fn CleanupBlockEncoder<AllocU8: alloc::Allocator<u8>,
                        AllocU16: alloc::Allocator<u16>>(m8: &mut AllocU8,
m16 : &mut AllocU16, xself: &mut BlockEncoder<AllocU8, AllocU16>){
  m8.free_cell(core::mem::replace(&mut (*xself).depths_, AllocU8::AllocatedMemory::default()));
  m16.free_cell(core::mem::replace(&mut (*xself).bits_, AllocU16::AllocatedMemory::default()));
}

fn JumpToByteBoundary(storage_ix: &mut usize, storage: &mut [u8]) {
  *storage_ix = (*storage_ix).wrapping_add(7u32 as (usize)) & !7u32 as (usize);
  storage[((*storage_ix >> 3i32) as (usize))] = 0i32 as (u8);
}


pub fn BrotliStoreMetaBlock<'a,
                            AllocU8: alloc::Allocator<u8>,
                            AllocU16: alloc::Allocator<u16>,
                            AllocU32: alloc::Allocator<u32>,
                            AllocF: alloc::Allocator<floatX>,
                            AllocHT: alloc::Allocator<HuffmanTree>,
                            AllocHL: alloc::Allocator<HistogramLiteral>,
                            AllocHC: alloc::Allocator<HistogramCommand>,
                            AllocHD: alloc::Allocator<HistogramDistance>,
                            Cb>
  (m8: &mut AllocU8,
   m16: &mut AllocU16,
   m32: &mut AllocU32,
   mf: &mut AllocF,
   mht: &mut AllocHT,
   input: &'a[u8],
   start_pos: usize,
   length: usize,
   mask: usize,
   params: &BrotliEncoderParams,
   mut prev_byte: u8,
   mut prev_byte2: u8,
   is_last: i32,
   num_direct_distance_codes: u32,
   distance_postfix_bits: u32,
   literal_context_mode: ContextType,
   distance_cache: &[i32; kNumDistanceCacheEntries],
   commands: &[Command],
   n_commands: usize,
   mb: &mut MetaBlockSplit<AllocU8, AllocU32, AllocHL, AllocHC, AllocHD>,
   recoder_state: &mut RecoderState,
   storage_ix: &mut usize,
   storage: &mut [u8],
  callback: &mut Cb) where Cb: FnMut(&[interface::Command<InputReference>]) {
  let (input0,input1) = InputPairFromMaskedInput(input, start_pos, length, mask);
  if params.log_meta_block {
      LogMetaBlock(m16, m32, mf, commands.split_at(n_commands).0, input0, input1,
                   distance_postfix_bits, num_direct_distance_codes, distance_cache,
                   recoder_state,
                   block_split_reference(mb),
                   params,
                   Some(literal_context_mode),
                   callback);
  }
  let mut pos: usize = start_pos;
  let mut i: usize;
  let num_distance_codes: usize = (16u32)
    .wrapping_add(num_direct_distance_codes)
    .wrapping_add(48u32 << distance_postfix_bits) as (usize);
  let mut tree: AllocHT::AllocatedMemory;
  let mut literal_enc: BlockEncoder<AllocU8, AllocU16>;
  let mut command_enc: BlockEncoder<AllocU8, AllocU16>;
  let mut distance_enc: BlockEncoder<AllocU8, AllocU16>;
  StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
  tree = if 2i32 * 704i32 + 1i32 != 0 {
    mht.alloc_cell((2i32 * 704i32 + 1i32) as (usize))
  } else {
    AllocHT::AllocatedMemory::default()
  };
  literal_enc = NewBlockEncoder::<AllocU8, AllocU16>(256usize,
                                                     (*mb).literal_split.num_types,
                                                     (*mb).literal_split.types.slice(),
                                                     (*mb).literal_split.lengths.slice(),
                                                     (*mb).literal_split.num_blocks);
  command_enc = NewBlockEncoder::<AllocU8, AllocU16>(704usize,
                                                     (*mb).command_split.num_types,
                                                     (*mb).command_split.types.slice(),
                                                     (*mb).command_split.lengths.slice(),
                                                     (*mb).command_split.num_blocks);
  distance_enc = NewBlockEncoder::<AllocU8, AllocU16>(num_distance_codes,
                                                      (*mb).distance_split.num_types,
                                                      (*mb).distance_split.types.slice(),
                                                      (*mb).distance_split.lengths.slice(),
                                                      (*mb).distance_split.num_blocks);
  BuildAndStoreBlockSwitchEntropyCodes(&mut literal_enc, tree.slice_mut(), storage_ix, storage);
  BuildAndStoreBlockSwitchEntropyCodes(&mut command_enc, tree.slice_mut(), storage_ix, storage);
  BuildAndStoreBlockSwitchEntropyCodes(&mut distance_enc, tree.slice_mut(), storage_ix, storage);
  BrotliWriteBits(2, distance_postfix_bits as (u64), storage_ix, storage);
  BrotliWriteBits(4,
                  (num_direct_distance_codes >> distance_postfix_bits) as (u64),
                  storage_ix,
                  storage);
  i = 0usize;
  while i < (*mb).literal_split.num_types {
    {
      BrotliWriteBits(2, literal_context_mode as (u64), storage_ix, storage);
    }
    i = i.wrapping_add(1 as (usize));
  }
  if (*mb).literal_context_map_size == 0usize {
    StoreTrivialContextMap((*mb).literal_histograms_size,
                           6,
                           tree.slice_mut(),
                           storage_ix,
                           storage);
  } else {
    EncodeContextMap(m32,
                     (*mb).literal_context_map.slice(),
                     (*mb).literal_context_map_size,
                     (*mb).literal_histograms_size,
                     tree.slice_mut(),
                     storage_ix,
                     storage);
  }
  if (*mb).distance_context_map_size == 0usize {
    StoreTrivialContextMap((*mb).distance_histograms_size,
                           2usize,
                           tree.slice_mut(),
                           storage_ix,
                           storage);
  } else {
    EncodeContextMap(m32,
                     (*mb).distance_context_map.slice(),
                     (*mb).distance_context_map_size,
                     (*mb).distance_histograms_size,
                     tree.slice_mut(),
                     storage_ix,
                     storage);
  }
  BuildAndStoreEntropyCodes(m8,
                            m16,
                            &mut literal_enc,
                            (*mb).literal_histograms.slice(),
                            (*mb).literal_histograms_size,
                            tree.slice_mut(),
                            storage_ix,
                            storage);
  BuildAndStoreEntropyCodes(m8,
                            m16,
                            &mut command_enc,
                            (*mb).command_histograms.slice(),
                            (*mb).command_histograms_size,
                            tree.slice_mut(),
                            storage_ix,
                            storage);
  BuildAndStoreEntropyCodes(m8,
                            m16,
                            &mut distance_enc,
                            (*mb).distance_histograms.slice(),
                            (*mb).distance_histograms_size,
                            tree.slice_mut(),
                            storage_ix,
                            storage);
  {
    mht.free_cell(core::mem::replace(&mut tree, AllocHT::AllocatedMemory::default()));
  }
  i = 0usize;
  while i < n_commands {
    {
      let cmd: Command = commands[(i as (usize))].clone();
      let cmd_code: usize = cmd.cmd_prefix_ as (usize);
      StoreSymbol(&mut command_enc, cmd_code, storage_ix, storage);
      StoreCommandExtra(&cmd, storage_ix, storage);
      if (*mb).literal_context_map_size == 0usize {
        let mut j: usize;
        j = cmd.insert_len_ as (usize);
        while j != 0usize {
          {
            StoreSymbol(&mut literal_enc,
                        input[((pos & mask) as (usize))] as (usize),
                        storage_ix,
                        storage);
            pos = pos.wrapping_add(1 as (usize));
          }
          j = j.wrapping_sub(1 as (usize));
        }
      } else {
        let mut j: usize;
        j = cmd.insert_len_ as (usize);
        while j != 0usize {
          {
            let context: usize = Context(prev_byte, prev_byte2, literal_context_mode) as (usize);
            let literal: u8 = input[((pos & mask) as (usize))];
            StoreSymbolWithContext(&mut literal_enc,
                                   literal as (usize),
                                   context,
                                   (*mb).literal_context_map.slice(),
                                   storage_ix,
                                   storage,
                                   6usize);
            prev_byte2 = prev_byte;
            prev_byte = literal;
            pos = pos.wrapping_add(1 as (usize));
          }
          j = j.wrapping_sub(1 as (usize));
        }
      }
      pos = pos.wrapping_add(CommandCopyLen(&cmd) as (usize));
      if CommandCopyLen(&cmd) != 0 {
        prev_byte2 = input[((pos.wrapping_sub(2usize) & mask) as (usize))];
        prev_byte = input[((pos.wrapping_sub(1usize) & mask) as (usize))];
        if cmd.cmd_prefix_ as (i32) >= 128i32 {
          let dist_code: usize = cmd.dist_prefix_ as (usize);
          let distnumextra: u32 = cmd.dist_extra_ >> 24i32;
          let distextra: u64 = (cmd.dist_extra_ & 0xffffffu32) as (u64);
          if (*mb).distance_context_map_size == 0usize {
            StoreSymbol(&mut distance_enc, dist_code, storage_ix, storage);
          } else {
            let context: usize = CommandDistanceContext(&cmd) as (usize);
            StoreSymbolWithContext(&mut distance_enc,
                                   dist_code,
                                   context,
                                   (*mb).distance_context_map.slice(),
                                   storage_ix,
                                   storage,
                                   2usize);
          }
          BrotliWriteBits(distnumextra as (u8), distextra, storage_ix, storage);
        }
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  CleanupBlockEncoder(m8, m16, &mut distance_enc);
  CleanupBlockEncoder(m8, m16, &mut command_enc);
  CleanupBlockEncoder(m8, m16, &mut literal_enc);
  if is_last != 0 {
    JumpToByteBoundary(storage_ix, storage);
  }
}

fn BuildHistograms(input: &[u8],
                   start_pos: usize,
                   mask: usize,
                   commands: &[Command],
                   n_commands: usize,
                   lit_histo: &mut HistogramLiteral,
                   cmd_histo: &mut HistogramCommand,
                   dist_histo: &mut HistogramDistance) {
  let mut pos: usize = start_pos;
  let mut i: usize;
  i = 0usize;
  while i < n_commands {
    {
      let cmd: Command = commands[(i as (usize))].clone();
      let mut j: usize;
      HistogramAddItem(cmd_histo, cmd.cmd_prefix_ as (usize));
      j = cmd.insert_len_ as (usize);
      while j != 0usize {
        {
          HistogramAddItem(lit_histo, input[((pos & mask) as (usize))] as (usize));
          pos = pos.wrapping_add(1 as (usize));
        }
        j = j.wrapping_sub(1 as (usize));
      }
      pos = pos.wrapping_add(CommandCopyLen(&cmd) as (usize));
      if CommandCopyLen(&cmd) != 0 && (cmd.cmd_prefix_ as (i32) >= 128i32) {
        HistogramAddItem(dist_histo, cmd.dist_prefix_ as (usize));
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
}
fn StoreDataWithHuffmanCodes(input: &[u8],
                             start_pos: usize,
                             mask: usize,
                             commands: &[Command],
                             n_commands: usize,
                             lit_depth: &[u8],
                             lit_bits: &[u16],
                             cmd_depth: &[u8],
                             cmd_bits: &[u16],
                             dist_depth: &[u8],
                             dist_bits: &[u16],
                             storage_ix: &mut usize,
                             storage: &mut [u8]) {
  let mut pos: usize = start_pos;
  let mut i: usize;
  i = 0usize;
  while i < n_commands {
    {
      let cmd: Command = commands[(i as (usize))].clone();
      let cmd_code: usize = cmd.cmd_prefix_ as (usize);
      let mut j: usize;
      BrotliWriteBits(cmd_depth[(cmd_code as (usize))] as (u8),
                      cmd_bits[(cmd_code as (usize))] as (u64),
                      storage_ix,
                      storage);
      StoreCommandExtra(&cmd, storage_ix, storage);
      j = cmd.insert_len_ as (usize);
      while j != 0usize {
        {
          let literal: u8 = input[((pos & mask) as (usize))];
          BrotliWriteBits(lit_depth[(literal as (usize))] as (u8),
                          lit_bits[(literal as (usize))] as (u64),
                          storage_ix,
                          storage);
          pos = pos.wrapping_add(1 as (usize));
        }
        j = j.wrapping_sub(1 as (usize));
      }
      pos = pos.wrapping_add(CommandCopyLen(&cmd) as (usize));
      if CommandCopyLen(&cmd) != 0 && (cmd.cmd_prefix_ as (i32) >= 128i32) {
        let dist_code: usize = cmd.dist_prefix_ as (usize);
        let distnumextra: u32 = cmd.dist_extra_ >> 24i32;
        let distextra: u32 = cmd.dist_extra_ & 0xffffff;
        BrotliWriteBits(dist_depth[(dist_code as (usize))] as (u8),
                        dist_bits[(dist_code as (usize))] as (u64),
                        storage_ix,
                        storage);
        BrotliWriteBits(distnumextra as (u8),
                        distextra as (u64),
                        storage_ix,
                        storage);
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
}

fn nop<'a>(_data:&[interface::Command<InputReference>]){
}
pub fn BrotliStoreMetaBlockTrivial<'a,
                                   AllocU16:alloc::Allocator<u16>,
                                   AllocU32:alloc::Allocator<u32>,
                                   AllocF:alloc::Allocator<floatX>,
                                   Cb>
    (m16:&mut AllocU16,
     m32:&mut AllocU32,
     mf:&mut AllocF,
     input: &'a [u8],
     start_pos: usize,
     length: usize,
     mask: usize,
     params: &BrotliEncoderParams,
     is_last: i32,
     distance_cache: &[i32; kNumDistanceCacheEntries],
     commands: &[Command],
     n_commands: usize,
     recoder_state: &mut RecoderState,
     storage_ix: &mut usize,
     storage: &mut [u8],
    f:&mut Cb) where Cb: FnMut(&[interface::Command<InputReference>]) {
  let (input0,input1) = InputPairFromMaskedInput(input, start_pos, length, mask);
  if params.log_meta_block {
      LogMetaBlock(m16,
                   m32,
                   mf,
                   commands.split_at(n_commands).0,
                   input0,
                   input1,
                   0,
                   0,
                   distance_cache,
                   recoder_state,
                   block_split_nop(),
                   params,
                   Some(ContextType::CONTEXT_LSB6),
                   f);
  }
  let mut lit_histo: HistogramLiteral = HistogramLiteral::default();
  let mut cmd_histo: HistogramCommand = HistogramCommand::default();
  let mut dist_histo: HistogramDistance = HistogramDistance::default();
  let mut lit_depth: [u8; 256] = [0; 256]; // FIXME these zero-initializations are costly
  let mut lit_bits: [u16; 256] = [0; 256];
  let mut cmd_depth: [u8; 704] = [0; 704];
  let mut cmd_bits: [u16; 704] = [0; 704];
  let mut dist_depth: [u8; 64] = [0; 64];
  let mut dist_bits: [u16; 64] = [0; 64];
  const MAX_HUFFMAN_TREE_SIZE: usize = (2i32 * 704i32 + 1i32) as usize;
  let mut tree: [HuffmanTree; MAX_HUFFMAN_TREE_SIZE] = [HuffmanTree {
    total_count_: 0,
    index_left_: 0,
    index_right_or_value_: 0,
  }; MAX_HUFFMAN_TREE_SIZE];
  StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
  BuildHistograms(input,
                  start_pos,
                  mask,
                  commands,
                  n_commands,
                  &mut lit_histo,
                  &mut cmd_histo,
                  &mut dist_histo);
  BrotliWriteBits(13, 0, storage_ix, storage);
  BuildAndStoreHuffmanTree(lit_histo.slice_mut(),
                           256,
                           &mut tree[..],
                           &mut lit_depth[..],
                           &mut lit_bits[..],
                           storage_ix,
                           storage);
  BuildAndStoreHuffmanTree(cmd_histo.slice_mut(),
                           704usize,
                           &mut tree[..],
                           &mut cmd_depth[..],
                           &mut cmd_bits[..],
                           storage_ix,
                           storage);
  BuildAndStoreHuffmanTree(dist_histo.slice_mut(),
                           64usize,
                           &mut tree[..],
                           &mut dist_depth[..],
                           &mut dist_bits[..],
                           storage_ix,
                           storage);
  StoreDataWithHuffmanCodes(input,
                            start_pos,
                            mask,
                            commands,
                            n_commands,
                            &mut lit_depth[..],
                            &mut lit_bits[..],
                            &mut cmd_depth[..],
                            &mut cmd_bits[..],
                            &mut dist_depth[..],
                            &mut dist_bits[..],
                            storage_ix,
                            storage);
  if is_last != 0 {
    JumpToByteBoundary(storage_ix, storage);
  }
}

fn StoreStaticCommandHuffmanTree(storage_ix: &mut usize, storage: &mut [u8]) {
  BrotliWriteBits(56,
                  0x926244u32 as (u64) << 32i32 | 0x16307003,
                  storage_ix,
                  storage);
  BrotliWriteBits(3, 0x0u64, storage_ix, storage);
}

fn StoreStaticDistanceHuffmanTree(storage_ix: &mut usize, storage: &mut [u8]) {
  BrotliWriteBits(28, 0x369dc03u64, storage_ix, storage);
}


struct BlockSplitRef<'a> {
    types: &'a [u8],
    lengths:&'a [u32],
    num_types: u32,
}
impl<'a> Default for BlockSplitRef<'a> {
    fn default() -> Self {
        BlockSplitRef {
            types:&[],
            lengths:&[],
            num_types:1,
        }
    }
}


#[derive(Default)]
struct MetaBlockSplitRefs<'a> {
    btypel : BlockSplitRef<'a>,
    literal_context_map:&'a [u32],
    btypec : BlockSplitRef<'a>,
    btyped : BlockSplitRef<'a>,
    distance_context_map:&'a [u32],
}

fn block_split_nop() -> MetaBlockSplitRefs<'static> {
    return MetaBlockSplitRefs::default()

}

fn block_split_reference<'a,
                         AllocU8: alloc::Allocator<u8>,
                         AllocU32: alloc::Allocator<u32>,
                         AllocHL: alloc::Allocator<HistogramLiteral>,
                         AllocHC: alloc::Allocator<HistogramCommand>,
                         AllocHD: alloc::Allocator<HistogramDistance>>
    (mb:&'a MetaBlockSplit<AllocU8, AllocU32, AllocHL, AllocHC, AllocHD>) -> MetaBlockSplitRefs<'a> {
        return MetaBlockSplitRefs::<'a> {
            btypel:BlockSplitRef {
                types: mb.literal_split.types.slice().split_at(mb.literal_split.num_blocks).0,
                lengths:mb.literal_split.lengths.slice().split_at(mb.literal_split.num_blocks).0,
                num_types:mb.literal_split.num_types as u32,
            },
            literal_context_map: mb.literal_context_map.slice().split_at(mb.literal_context_map_size).0,
            btypec:BlockSplitRef {
                types: mb.command_split.types.slice().split_at(mb.command_split.num_blocks).0,
                lengths:mb.command_split.lengths.slice().split_at(mb.command_split.num_blocks).0,
                num_types:mb.command_split.num_types as u32,
            },
            btyped:BlockSplitRef {
                types: mb.distance_split.types.slice().split_at(mb.distance_split.num_blocks).0,
                lengths:mb.distance_split.lengths.slice().split_at(mb.distance_split.num_blocks).0,
                num_types:mb.distance_split.num_types as u32,
            },
            distance_context_map: mb.distance_context_map.slice().split_at(mb.distance_context_map_size).0,
        }
}
     
#[derive(Clone, Copy)]
pub struct RecoderState {
    pub num_bytes_encoded : usize,
}

impl RecoderState {
    pub fn new() -> Self {
        RecoderState{
            num_bytes_encoded:0,
        }
    }
}


pub fn BrotliStoreMetaBlockFast<Cb,
                                AllocU16:alloc::Allocator<u16>,
                                AllocU32:alloc::Allocator<u32>,
                                AllocF:alloc::Allocator<floatX>,
                                AllocHT: alloc::Allocator<HuffmanTree>>(m : &mut AllocHT,
                                                                        m16: &mut AllocU16,
                                                                        m32: &mut AllocU32,
                                                                        mf: &mut AllocF,
                                input: &[u8],
                                start_pos: usize,
                                length: usize,
                                mask: usize,
                                params: &BrotliEncoderParams,
                                is_last: i32,
                                dist_cache: &[i32; kNumDistanceCacheEntries],
                                commands: &[Command],
                                n_commands: usize,
                                recoder_state: &mut RecoderState,
                                storage_ix: &mut usize,
                                storage: &mut [u8],
                                cb: &mut Cb) where Cb: FnMut(&[interface::Command<InputReference>]) {
  let (input0,input1) = InputPairFromMaskedInput(input, start_pos, length, mask);
  if params.log_meta_block {
      LogMetaBlock(m16,
                   m32,
                   mf,
                   commands.split_at(n_commands).0, input0, input1, 0, 0, dist_cache, recoder_state,
                   block_split_nop(),
                   params,
                   Some(ContextType::CONTEXT_LSB6),
                   cb);
  }
  StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
  BrotliWriteBits(13, 0, storage_ix, storage);
  if n_commands <= 128usize {
    let mut histogram: [u32; 256] = [0; 256];
    let mut pos: usize = start_pos;
    let mut num_literals: usize = 0usize;
    let mut i: usize;
    let mut lit_depth: [u8; 256] = [0; 256];
    let mut lit_bits: [u16; 256] = [0; 256];
    i = 0usize;
    while i < n_commands {
      {
        let cmd: Command = commands[(i as (usize))].clone();
        let mut j: usize;
        j = cmd.insert_len_ as (usize);
        while j != 0usize {
          {
            {
              let _rhs = 1;
              let _lhs = &mut histogram[input[((pos & mask) as (usize))] as (usize)];
              *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            pos = pos.wrapping_add(1 as (usize));
          }
          j = j.wrapping_sub(1 as (usize));
        }
        num_literals = num_literals.wrapping_add(cmd.insert_len_ as (usize));
        pos = pos.wrapping_add(CommandCopyLen(&cmd) as (usize));
      }
      i = i.wrapping_add(1 as (usize));
    }
    BrotliBuildAndStoreHuffmanTreeFast(m,
                                       &mut histogram[..],
                                       num_literals,
                                       8usize,
                                       &mut lit_depth[..],
                                       &mut lit_bits[..],
                                       storage_ix,
                                       storage);
    StoreStaticCommandHuffmanTree(storage_ix, storage);
    StoreStaticDistanceHuffmanTree(storage_ix, storage);
    StoreDataWithHuffmanCodes(input,
                              start_pos,
                              mask,
                              commands,
                              n_commands,
                              &mut lit_depth[..],
                              &mut lit_bits[..],
                              &kStaticCommandCodeDepth[..],
                              &kStaticCommandCodeBits[..],
                              &kStaticDistanceCodeDepth[..],
                              &kStaticDistanceCodeBits[..],
                              storage_ix,
                              storage);
  } else {
    let mut lit_histo: HistogramLiteral = HistogramLiteral::default();
    let mut cmd_histo: HistogramCommand = HistogramCommand::default();
    let mut dist_histo: HistogramDistance = HistogramDistance::default();
    let mut lit_depth: [u8; 256] = [0; 256];
    let mut lit_bits: [u16; 256] = [0; 256];
    let mut cmd_depth: [u8; 704] = [0; 704];
    let mut cmd_bits: [u16; 704] = [0; 704];
    let mut dist_depth: [u8; 64] = [0; 64];
    let mut dist_bits: [u16; 64] = [0; 64];
    BuildHistograms(input,
                    start_pos,
                    mask,
                    commands,
                    n_commands,
                    &mut lit_histo,
                    &mut cmd_histo,
                    &mut dist_histo);
    BrotliBuildAndStoreHuffmanTreeFast(m,
                                       lit_histo.slice(),
                                       lit_histo.total_count_,
                                       8usize,
                                       &mut lit_depth[..],
                                       &mut lit_bits[..],
                                       storage_ix,
                                       storage);
    BrotliBuildAndStoreHuffmanTreeFast(m,
                                       cmd_histo.slice(),
                                       cmd_histo.total_count_,
                                       10usize,
                                       &mut cmd_depth[..],
                                       &mut cmd_bits[..],
                                       storage_ix,
                                       storage);
    BrotliBuildAndStoreHuffmanTreeFast(m,
                                       dist_histo.slice(),
                                       dist_histo.total_count_,
                                       6usize,
                                       &mut dist_depth[..],
                                       &mut dist_bits[..],
                                       storage_ix,
                                       storage);
    StoreDataWithHuffmanCodes(input,
                              start_pos,
                              mask,
                              commands,
                              n_commands,
                              &mut lit_depth[..],
                              &mut lit_bits[..],
                              &mut cmd_depth[..],
                              &mut cmd_bits[..],
                              &mut dist_depth[..],
                              &mut dist_bits[..],
                              storage_ix,
                              storage);
  }
  if is_last != 0 {
    JumpToByteBoundary(storage_ix, storage);
  }
}
fn BrotliStoreUncompressedMetaBlockHeader(length: usize,
                                          storage_ix: &mut usize,
                                          storage: &mut [u8]) {
  let mut lenbits: u64 = 0;
  let mut nlenbits: u32 = 0;
  let mut nibblesbits: u32 = 0;
  BrotliWriteBits(1, 0, storage_ix, storage);
  BrotliEncodeMlen(length as u32, &mut lenbits, &mut nlenbits, &mut nibblesbits);
  BrotliWriteBits(2, nibblesbits as u64, storage_ix, storage);
  BrotliWriteBits(nlenbits as u8, lenbits as u64, storage_ix, storage);
  BrotliWriteBits(1, 1, storage_ix, storage);
}

fn InputPairFromMaskedInput<'a>(input:&'a [u8], position: usize, len: usize, mask:usize) -> (&'a [u8], &'a [u8]) {
  let masked_pos: usize = position & mask;
  if masked_pos.wrapping_add(len) > mask.wrapping_add(1usize) {
    let len1: usize = mask.wrapping_add(1usize).wrapping_sub(masked_pos);
    return (&input[masked_pos..(masked_pos + len1)],
            &input[0..len.wrapping_sub(len1)]);
  }
  return (&input[masked_pos..masked_pos + len], &[]);

}
pub fn BrotliStoreUncompressedMetaBlock<Cb, AllocU16:alloc::Allocator<u16>, AllocU32:alloc::Allocator<u32>, AllocF:alloc::Allocator<floatX>>
    (m16:&mut AllocU16,
     m32:&mut AllocU32,
     mf:&mut AllocF,
     is_final_block: i32,
     input: &[u8],
     position: usize,
     mask: usize,
     params: &BrotliEncoderParams,
     len: usize,
     recoder_state: &mut RecoderState,
     storage_ix: &mut usize,
     storage: &mut [u8],
     suppress_meta_block_logging: bool,
     cb: &mut Cb) where Cb: FnMut(&[interface::Command<InputReference>]){
  let (input0,input1) = InputPairFromMaskedInput(input, position, len, mask);
  BrotliStoreUncompressedMetaBlockHeader(len, storage_ix, storage);
  JumpToByteBoundary(storage_ix, storage);
  let dst_start0 = ((*storage_ix >> 3i32) as (usize));
  storage[dst_start0..(dst_start0 + input0.len())].clone_from_slice(input0);
  *storage_ix = (*storage_ix).wrapping_add(input0.len() << 3i32);
  let dst_start1 = ((*storage_ix >> 3i32) as (usize));
  storage[dst_start1..(dst_start1 + input1.len())].clone_from_slice(input1);
  *storage_ix = (*storage_ix).wrapping_add(input1.len() << 3i32);
  BrotliWriteBitsPrepareStorage(*storage_ix, storage);
  if params.log_meta_block && !suppress_meta_block_logging {
    let cmds = [Command{insert_len_:len as u32,
                        copy_len_:0,
                        dist_extra_:0,
                        cmd_prefix_:0,
                        dist_prefix_:0
    }];

    LogMetaBlock(m16, m32, mf, &cmds, input0, input1, 0, 0, &[0i32, 0i32, 0i32, 0i32], recoder_state,
                 block_split_nop(),
                 params,
                 None,
                 cb);
  }
  if is_final_block != 0 {
    BrotliWriteBits(1u8, 1u64, storage_ix, storage);
    BrotliWriteBits(1u8, 1u64, storage_ix, storage);
    JumpToByteBoundary(storage_ix, storage);
  }
}


pub fn BrotliStoreSyncMetaBlock(storage_ix: &mut usize, storage: &mut [u8]) {
  BrotliWriteBits(6, 6, storage_ix, storage);
  JumpToByteBoundary(storage_ix, storage);
}
