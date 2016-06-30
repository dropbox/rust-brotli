#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]


use alloc;
use core;
use bit_reader::{BrotliBitReader, BrotliGetAvailableBits, BrotliInitBitReader};
use huffman::{BROTLI_HUFFMAN_MAX_CODE_LENGTH, BROTLI_HUFFMAN_MAX_CODE_LENGTHS_SIZE,
              BROTLI_HUFFMAN_MAX_TABLE_SIZE, HuffmanCode, HuffmanTreeGroup};

#[allow(dead_code)]
pub enum WhichTreeGroup {
  LITERAL,
  INSERT_COPY,
  DISTANCE,
}

pub enum BrotliRunningState {
  BROTLI_STATE_UNINITED,
  BROTLI_STATE_METABLOCK_BEGIN,
  BROTLI_STATE_METABLOCK_HEADER,
  BROTLI_STATE_METABLOCK_HEADER_2,
  BROTLI_STATE_CONTEXT_MODES,
  BROTLI_STATE_COMMAND_BEGIN,
  BROTLI_STATE_COMMAND_INNER,
  BROTLI_STATE_COMMAND_POST_DECODE_LITERALS,
  BROTLI_STATE_COMMAND_POST_WRAP_COPY,
  BROTLI_STATE_UNCOMPRESSED,
  BROTLI_STATE_METADATA,
  BROTLI_STATE_COMMAND_INNER_WRITE,
  BROTLI_STATE_METABLOCK_DONE,
  BROTLI_STATE_COMMAND_POST_WRITE_1,
  BROTLI_STATE_COMMAND_POST_WRITE_2,
  BROTLI_STATE_HUFFMAN_CODE_0,
  BROTLI_STATE_HUFFMAN_CODE_1,
  BROTLI_STATE_HUFFMAN_CODE_2,
  BROTLI_STATE_HUFFMAN_CODE_3,
  BROTLI_STATE_CONTEXT_MAP_1,
  BROTLI_STATE_CONTEXT_MAP_2,
  BROTLI_STATE_TREE_GROUP,
  BROTLI_STATE_DONE,
}

pub enum BrotliRunningMetablockHeaderState {
  BROTLI_STATE_METABLOCK_HEADER_NONE,
  BROTLI_STATE_METABLOCK_HEADER_EMPTY,
  BROTLI_STATE_METABLOCK_HEADER_NIBBLES,
  BROTLI_STATE_METABLOCK_HEADER_SIZE,
  BROTLI_STATE_METABLOCK_HEADER_UNCOMPRESSED,
  BROTLI_STATE_METABLOCK_HEADER_RESERVED,
  BROTLI_STATE_METABLOCK_HEADER_BYTES,
  BROTLI_STATE_METABLOCK_HEADER_METADATA,
}
pub enum BrotliRunningUncompressedState {
  BROTLI_STATE_UNCOMPRESSED_NONE,
  BROTLI_STATE_UNCOMPRESSED_WRITE,
}

pub enum BrotliRunningTreeGroupState {
  BROTLI_STATE_TREE_GROUP_NONE,
  BROTLI_STATE_TREE_GROUP_LOOP,
}

pub enum BrotliRunningContextMapState {
  BROTLI_STATE_CONTEXT_MAP_NONE,
  BROTLI_STATE_CONTEXT_MAP_READ_PREFIX,
  BROTLI_STATE_CONTEXT_MAP_HUFFMAN,
  BROTLI_STATE_CONTEXT_MAP_DECODE,
  BROTLI_STATE_CONTEXT_MAP_TRANSFORM,
}

pub enum BrotliRunningHuffmanState {
  BROTLI_STATE_HUFFMAN_NONE,
  BROTLI_STATE_HUFFMAN_SIMPLE_SIZE,
  BROTLI_STATE_HUFFMAN_SIMPLE_READ,
  BROTLI_STATE_HUFFMAN_SIMPLE_BUILD,
  BROTLI_STATE_HUFFMAN_COMPLEX,
  BROTLI_STATE_HUFFMAN_LENGTH_SYMBOLS,
}

pub enum BrotliRunningDecodeUint8State {
  BROTLI_STATE_DECODE_UINT8_NONE,
  BROTLI_STATE_DECODE_UINT8_SHORT,
  BROTLI_STATE_DECODE_UINT8_LONG,
}

pub enum BrotliRunningReadBlockLengthState {
  BROTLI_STATE_READ_BLOCK_LENGTH_NONE,
  BROTLI_STATE_READ_BLOCK_LENGTH_SUFFIX,
}

pub const kLiteralContextBits: usize = 6;

pub struct BlockTypeAndLengthState<AllocHC: alloc::Allocator<HuffmanCode>> {
  pub substate_read_block_length: BrotliRunningReadBlockLengthState,
  pub num_block_types: [u32; 3],
  pub block_length_index: u32,
  pub block_length: [u32; 3],
  pub block_type_trees: AllocHC::AllocatedMemory,
  pub block_len_trees: AllocHC::AllocatedMemory,
  pub block_type_rb: [u32; 6],
}

pub struct BrotliState<AllocU8: alloc::Allocator<u8>,
                       AllocU32: alloc::Allocator<u32>,
                       AllocHC: alloc::Allocator<HuffmanCode>>
{
  pub state: BrotliRunningState,

  // This counter is reused for several disjoint loops.
  pub loop_counter: i32,
  pub br: BrotliBitReader,
  pub alloc_u8: AllocU8,
  pub alloc_u32: AllocU32,
  pub alloc_hc: AllocHC,
  // void* memory_manager_opaque,
  pub buffer: [u8; 8],
  pub buffer_length: u32,
  pub pos: i32,
  pub max_backward_distance: i32,
  pub max_backward_distance_minus_custom_dict_size: i32,
  pub max_distance: i32,
  pub ringbuffer_size: i32,
  pub ringbuffer_mask: i32,
  pub dist_rb_idx: i32,
  pub dist_rb: [i32; 4],
  pub ringbuffer: AllocU8::AllocatedMemory,
  // pub ringbuffer_end : usize,
  pub htree_command_index: u16,
  pub context_lookup1: &'static [u8],
  pub context_lookup2: &'static [u8],
  pub context_map_slice_index: usize,
  pub dist_context_map_slice_index: usize,

  pub sub_loop_counter: u32,

  // This ring buffer holds a few past copy distances that will be used by */
  // some special distance codes.
  pub literal_hgroup: HuffmanTreeGroup<AllocU32, AllocHC>,
  pub insert_copy_hgroup: HuffmanTreeGroup<AllocU32, AllocHC>,
  pub distance_hgroup: HuffmanTreeGroup<AllocU32, AllocHC>,
  // This is true if the literal context map histogram type always matches the
  // block type. It is then not needed to keep the context (faster decoding).
  pub trivial_literal_context: i32,
  pub distance_context: i32,
  pub meta_block_remaining_len: i32,
  pub block_type_length_state: BlockTypeAndLengthState<AllocHC>,
  pub distance_postfix_bits: u32,
  pub num_direct_distance_codes: u32,
  pub distance_postfix_mask: i32,
  pub num_dist_htrees: u32,
  pub dist_context_map: AllocU8::AllocatedMemory,
  // NOT NEEDED? the index below seems to supersede it pub literal_htree : AllocHC::AllocatedMemory,
  pub literal_htree_index: u8,
  pub dist_htree_index: u8,
  pub repeat_code_len: u32,
  pub prev_code_len: u32,

  pub copy_length: i32,
  pub distance_code: i32,

  // For partial write operations
  pub rb_roundtrips: usize, // How many times we went around the ringbuffer
  pub partial_pos_out: usize, // How much output to the user in total (<= rb)

  // For ReadHuffmanCode
  pub symbol: u32,
  pub repeat: u32,
  pub space: u32,

  pub table: [HuffmanCode; 32],
  // List of of symbol chains.
  pub symbol_lists_index: usize, // AllocU16::AllocatedMemory,
  // Storage from symbol_lists.
  pub symbols_lists_array: [u16; BROTLI_HUFFMAN_MAX_CODE_LENGTH + 1 +
                                 BROTLI_HUFFMAN_MAX_CODE_LENGTHS_SIZE],
  // Tails of symbol chains.
  pub next_symbol: [i32; 32],
  pub code_length_code_lengths: [u8; 18],
  // Population counts for the code lengths
  pub code_length_histo: [u16; 16],

  // For HuffmanTreeGroupDecode
  pub htree_index: i32,
  pub htree_next_offset: u32,

  // For DecodeContextMap
  pub context_index: u32,
  pub max_run_length_prefix: u32,
  pub code: u32,
  // always pre-allocated on state creation
  pub context_map_table: AllocHC::AllocatedMemory,

  // For InverseMoveToFrontTransform
  pub mtf_upper_bound: u32,
  pub mtf: [u8; 256],

  // For custom dictionaries
  pub custom_dict: AllocU8::AllocatedMemory,
  pub custom_dict_size: i32,
  // less used attributes are in the end of this struct */
  // States inside function calls
  pub substate_metablock_header: BrotliRunningMetablockHeaderState,
  pub substate_tree_group: BrotliRunningTreeGroupState,
  pub substate_context_map: BrotliRunningContextMapState,
  pub substate_uncompressed: BrotliRunningUncompressedState,
  pub substate_huffman: BrotliRunningHuffmanState,
  pub substate_decode_uint8: BrotliRunningDecodeUint8State,

  pub is_last_metablock: u8,
  pub is_uncompressed: u8,
  pub is_metadata: u8,
  pub size_nibbles: u8,
  pub window_bits: u32,

  pub num_literal_htrees: u32,
  pub context_map: AllocU8::AllocatedMemory,
  pub context_modes: AllocU8::AllocatedMemory,
}

impl <'brotli_state,
      AllocU8 : alloc::Allocator<u8>,
      AllocU32 : alloc::Allocator<u32>,
      AllocHC : alloc::Allocator<HuffmanCode> > BrotliState<AllocU8, AllocU32, AllocHC> {
    pub fn new(alloc_u8 : AllocU8,
           alloc_u32 : AllocU32,
           alloc_hc : AllocHC) -> Self{
        let MB_HEADER_NONE = BrotliRunningMetablockHeaderState::BROTLI_STATE_METABLOCK_HEADER_NONE;
        let READ_BLOCK_LENGTH_NONE =
          BrotliRunningReadBlockLengthState::BROTLI_STATE_READ_BLOCK_LENGTH_NONE;
        let mut retval = BrotliState::<AllocU8, AllocU32, AllocHC>{
            state : BrotliRunningState::BROTLI_STATE_UNINITED,
            loop_counter : 0,
            br : BrotliBitReader::default(),
            alloc_u8 : alloc_u8,
            alloc_u32 : alloc_u32,
            alloc_hc : alloc_hc,
            buffer : [0u8; 8],
            buffer_length : 0,
            pos : 0,
            max_backward_distance : 0,
            max_backward_distance_minus_custom_dict_size : 0,
            max_distance : 0,
            ringbuffer_size : 0,
            ringbuffer_mask: 0,
            dist_rb_idx : 0,
            dist_rb : [16, 15, 11, 4],
            ringbuffer : AllocU8::AllocatedMemory::default(),
            htree_command_index : 0,
            context_lookup1 : &[],
            context_lookup2 : &[],
            context_map_slice_index : 0,
            dist_context_map_slice_index : 0,
            sub_loop_counter : 0,

            literal_hgroup : HuffmanTreeGroup::<AllocU32, AllocHC>::default(),
            insert_copy_hgroup : HuffmanTreeGroup::<AllocU32, AllocHC>::default(),
            distance_hgroup : HuffmanTreeGroup::<AllocU32, AllocHC>::default(),
            trivial_literal_context : 0,
            distance_context : 0,
            meta_block_remaining_len : 0,
            block_type_length_state : BlockTypeAndLengthState::<AllocHC> {
              block_length_index : 0,
              block_length : [0; 3],
              num_block_types : [0;3],
              block_type_rb: [0;6],
              substate_read_block_length : READ_BLOCK_LENGTH_NONE,
              block_type_trees : AllocHC::AllocatedMemory::default(),
              block_len_trees : AllocHC::AllocatedMemory::default(),
            },
            distance_postfix_bits : 0,
            num_direct_distance_codes : 0,
            distance_postfix_mask : 0,
            num_dist_htrees : 0,
            dist_context_map : AllocU8::AllocatedMemory::default(),
            //// not needed literal_htree : AllocHC::AllocatedMemory::default(),
            literal_htree_index : 0,
            dist_htree_index : 0,
            repeat_code_len : 0,
            prev_code_len : 0,
            copy_length : 0,
            distance_code : 0,
            rb_roundtrips : 0,  /* How many times we went around the ringbuffer */
            partial_pos_out : 0,  /* How much output to the user in total (<= rb) */
            symbol : 0,
            repeat : 0,
            space : 0,
            table : [HuffmanCode::default(); 32],
            //symbol_lists: AllocU16::AllocatedMemory::default(),
            symbol_lists_index : BROTLI_HUFFMAN_MAX_CODE_LENGTH + 1,
            symbols_lists_array : [0;BROTLI_HUFFMAN_MAX_CODE_LENGTH + 1 +
                              BROTLI_HUFFMAN_MAX_CODE_LENGTHS_SIZE],
            next_symbol : [0; 32],
            code_length_code_lengths : [0; 18],
            code_length_histo : [0; 16],
            htree_index : 0,
            htree_next_offset : 0,

            /* For DecodeContextMap */
           context_index : 0,
           max_run_length_prefix : 0,
           code : 0,
           context_map_table : AllocHC::AllocatedMemory::default(),

           /* For InverseMoveToFrontTransform */
           mtf_upper_bound : 255,
           mtf : [0; 256],

           /* For custom dictionaries */
           custom_dict : AllocU8::AllocatedMemory::default(),
           custom_dict_size : 0,

           /* less used attributes are in the end of this struct */
           /* States inside function calls */
           substate_metablock_header : MB_HEADER_NONE,
           substate_tree_group : BrotliRunningTreeGroupState::BROTLI_STATE_TREE_GROUP_NONE,
           substate_context_map : BrotliRunningContextMapState::BROTLI_STATE_CONTEXT_MAP_NONE,
           substate_uncompressed : BrotliRunningUncompressedState::BROTLI_STATE_UNCOMPRESSED_NONE,
           substate_huffman : BrotliRunningHuffmanState::BROTLI_STATE_HUFFMAN_NONE,
           substate_decode_uint8 : BrotliRunningDecodeUint8State::BROTLI_STATE_DECODE_UINT8_NONE,

           is_last_metablock : 0,
           is_uncompressed : 0,
           is_metadata : 0,
           size_nibbles : 0,
           window_bits : 0,

           num_literal_htrees : 0,
           context_map : AllocU8::AllocatedMemory::default(),
           context_modes : AllocU8::AllocatedMemory::default(),
        };
        retval.context_map_table = retval.alloc_hc.alloc_cell(
          BROTLI_HUFFMAN_MAX_TABLE_SIZE as usize);
        BrotliInitBitReader(&mut retval.br);
        retval
    }
    pub fn BrotliStateMetablockBegin(self : &mut Self) {
        self.meta_block_remaining_len = 0;
        self.block_type_length_state.block_length[0] = 1u32 << 28;
        self.block_type_length_state.block_length[1] = 1u32 << 28;
        self.block_type_length_state.block_length[2] = 1u32 << 28;
        self.block_type_length_state.num_block_types[0] = 1;
        self.block_type_length_state.num_block_types[1] = 1;
        self.block_type_length_state.num_block_types[2] = 1;
        self.block_type_length_state.block_type_rb[0] = 1;
        self.block_type_length_state.block_type_rb[1] = 0;
        self.block_type_length_state.block_type_rb[2] = 1;
        self.block_type_length_state.block_type_rb[3] = 0;
        self.block_type_length_state.block_type_rb[4] = 1;
        self.block_type_length_state.block_type_rb[5] = 0;
        self.alloc_u8.free_cell(core::mem::replace(&mut self.context_map,
                                             AllocU8::AllocatedMemory::default()));
        self.alloc_u8.free_cell(core::mem::replace(&mut self.context_modes,
                                             AllocU8::AllocatedMemory::default()));
        self.alloc_u8.free_cell(core::mem::replace(&mut self.dist_context_map,
                                             AllocU8::AllocatedMemory::default()));
        self.context_map_slice_index = 0;
        self.literal_htree_index = 0;
        self.dist_context_map_slice_index = 0;
        self.dist_htree_index = 0;
        self.context_lookup1 = &[];
        self.context_lookup2 = &[];
        self.literal_hgroup.reset(&mut self.alloc_u32, &mut self.alloc_hc);
        self.insert_copy_hgroup.reset(&mut self.alloc_u32, &mut self.alloc_hc);
        self.distance_hgroup.reset(&mut self.alloc_u32, &mut self.alloc_hc);
    }
    pub fn BrotliStateCleanupAfterMetablock(self : &mut Self) {
        self.alloc_u8.free_cell(core::mem::replace(&mut self.context_map,
                                             AllocU8::AllocatedMemory::default()));
        self.alloc_u8.free_cell(core::mem::replace(&mut self.context_modes,
                                             AllocU8::AllocatedMemory::default()));
        self.alloc_u8.free_cell(core::mem::replace(&mut self.dist_context_map,
                                             AllocU8::AllocatedMemory::default()));


        self.literal_hgroup.reset(&mut self.alloc_u32, &mut self.alloc_hc);
        self.insert_copy_hgroup.reset(&mut self.alloc_u32, &mut self.alloc_hc);
        self.distance_hgroup.reset(&mut self.alloc_u32, &mut self.alloc_hc);
    }

   pub fn BrotliStateCleanup(self : &mut Self) {
      self.BrotliStateCleanupAfterMetablock();
      self.alloc_u8.free_cell(core::mem::replace(&mut self.ringbuffer,
                              AllocU8::AllocatedMemory::default()));
      self.alloc_hc.free_cell(core::mem::replace(&mut self.block_type_length_state.block_type_trees,
                              AllocHC::AllocatedMemory::default()));
      self.alloc_hc.free_cell(core::mem::replace(&mut self.block_type_length_state.block_len_trees,
                              AllocHC::AllocatedMemory::default()));
      self.alloc_hc.free_cell(core::mem::replace(&mut self.context_map_table,
                              AllocHC::AllocatedMemory::default()));

      //FIXME??  BROTLI_FREE(s, s->legacy_input_buffer);
      //FIXME??  BROTLI_FREE(s, s->legacy_output_buffer);
    }

    pub fn BrotliStateIsStreamStart(self : &Self) -> bool {
        match self.state {
            BrotliRunningState::BROTLI_STATE_UNINITED =>
                BrotliGetAvailableBits(&self.br) == 0,
            _ => false,
        }
    }

    pub fn BrotliStateIsStreamEnd(self : &Self) -> bool {
        match self.state {
            BrotliRunningState::BROTLI_STATE_DONE => true,
            _ => false
        }
    }
    pub fn BrotliHuffmanTreeGroupInit(self :&mut Self, group : WhichTreeGroup,
                                      alphabet_size : u16, ntrees : u16) {
        match group {
            WhichTreeGroup::LITERAL => self.literal_hgroup.init(&mut self.alloc_u32,
                                                                &mut self.alloc_hc,
                                                                alphabet_size, ntrees),
            WhichTreeGroup::INSERT_COPY => self.insert_copy_hgroup.init(&mut self.alloc_u32,
                                                                        &mut self.alloc_hc,
                                                                        alphabet_size, ntrees),
            WhichTreeGroup::DISTANCE => self.distance_hgroup.init(&mut self.alloc_u32,
                                                                  &mut self.alloc_hc,
                                                                  alphabet_size, ntrees),
        }
    }
    pub fn BrotliHuffmanTreeGroupRelease(self :&mut Self, group : WhichTreeGroup) {
        match group {
            WhichTreeGroup::LITERAL => self.literal_hgroup.reset(&mut self.alloc_u32,
                                                                 &mut self.alloc_hc),
            WhichTreeGroup::INSERT_COPY => self.insert_copy_hgroup.reset(&mut self.alloc_u32,
                                                                         &mut self.alloc_hc),
            WhichTreeGroup::DISTANCE => self.distance_hgroup.reset(&mut self.alloc_u32,
                                                                   &mut self.alloc_hc),
        }
    }
}
