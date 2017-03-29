use super::constants::{BROTLI_NUM_BLOCK_LEN_SYMBOLS, kZeroRepsBits, kZeroRepsDepth,
                       kNonZeroRepsBits, kNonZeroRepsDepth, kCodeLengthBits, kCodeLengthDepth};
use super::entropy_encode::{HuffmanTree, BrotliWriteHuffmanTree, BrotliCreateHuffmanTree,
                            BrotliConvertBitDepthsToSymbols, NewHuffmanTree, InitHuffmanTree,
                            SortHuffmanTreeItems, SortHuffmanTree, BrotliSetDepth};
use super::super::alloc;
use super::super::alloc::SliceWrapper;
use super::super::alloc::SliceWrapperMut;
use super::super::core;
pub struct PrefixCodeRange {
  pub offset: u32,
  pub nbits: u32,
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

fn BrotliWriteBits(n_bits: u8, bits: u64, mut pos: &mut usize, mut array: &mut [u8]) {
  assert!((bits >> n_bits as usize) == 0);
  assert!(n_bits <= 56);
  let ptr_offset: usize = ((*pos >> 3) as u32) as usize;
  let mut v = array[ptr_offset] as u64;
  v |= bits << ((*pos) as u64 & 7);
  array[ptr_offset + 7] = (v >> 56) as u8;
  array[ptr_offset + 6] = ((v >> 48) & 0xff) as u8;
  array[ptr_offset + 5] = ((v >> 40) & 0xff) as u8;
  array[ptr_offset + 4] = ((v >> 24) & 0xff) as u8;
  array[ptr_offset + 3] = ((v >> 16) & 0xff) as u8;
  array[ptr_offset + 2] = ((v >> 8) & 0xff) as u8;
  array[ptr_offset + 1] = ((v >> 4) & 0xff) as u8;
  array[ptr_offset] = (v & 0xff) as u8;
  *pos += n_bits as usize
}

fn BrotliWriteBitsPrepareStorage(pos: usize, mut array: &mut [u8]) {
  assert_eq!(pos & 7, 0);
  array[pos >> 3] = 0;
}

fn BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(num_codes: i32,
                                                code_length_bitdepth: &[u8],
                                                mut storage_ix: &mut usize,
                                                mut storage: &mut [u8]) {
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
        if code_length_bitdepth[(kStorageOrder[codes_to_store.wrapping_sub(1) as usize] as
            (usize))] as (i32) != 0i32 {
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
        let mut l: usize = code_length_bitdepth[(kStorageOrder[i as usize] as (usize))] as (usize);
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
                                   mut storage_ix: &mut usize,
                                   mut storage: &mut [u8]) {
  let mut i: usize;
  i = 0usize;
  while i < huffman_tree_size {
    {
      let mut ix: usize = huffman_tree[(i as (usize))] as (usize);
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
                              mut tree: &mut [HuffmanTree],
                              mut storage_ix: &mut usize,
                              mut storage: &mut [u8]) {
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
  let mut code_length_bitdepth_symbols: [u16; 18] = [0;18];
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

fn StoreStaticCodeLengthCode(mut storage_ix: &mut usize, mut storage: &mut [u8]) {
  BrotliWriteBits(40,
                  0xffu32 as (u64) << 32i32 | 0x55555554u32 as (u64),
                  storage_ix,
                  storage);
}

pub fn BrotliBuildAndStoreHuffmanTreeFast<AllocHT: alloc::Allocator<HuffmanTree>>(
    mut m : AllocHT,
    histogram : &[u32],
    histogram_total : usize,
    max_bits : usize,
    mut depth : &mut [u8],
    mut bits : &mut [u16],
    mut storage_ix : &mut usize,
    mut storage : &mut [u8]
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
          let mut sentinel: HuffmanTree;
          let mut i: i32 = 0i32;
          let mut j: i32 = n + 1i32;
          let mut k: i32;
          SortHuffmanTreeItems(tree.slice_mut(), n as (usize), SortHuffmanTree{});
          sentinel = NewHuffmanTree(!(0u32), -1i16, -1i16);
          tree.slice_mut()[(node_index.wrapping_add(1u32) as (usize))] = sentinel.clone();
          tree.slice_mut()[(node_index as (usize))] = sentinel.clone();
          node_index = node_index.wrapping_add(2u32);
          k = n - 1i32;
          while k > 0i32 {
            {
              let mut left: i32;
              let mut right: i32;
              if (tree.slice()[(i as (usize))]).total_count_ <= (tree.slice()[(j as (usize))]).total_count_ {
                left = i;
                i = i + 1;
              } else {
                left = j;
                j = j + 1;
              }
              if (tree.slice()[(i as (usize))]).total_count_ <= (tree.slice()[(j as (usize))]).total_count_ {
                right = i;
                i = i + 1;
              } else {
                right = j;
                j = j + 1;
              }
              let sum_total =
                (tree.slice()[(left as (usize))])
                  .total_count_
                  .wrapping_add((tree.slice()[(right as (usize))]).total_count_);
              (tree.slice_mut()[(node_index.wrapping_sub(1u32) as (usize))]).total_count_ = sum_total;
              (tree.slice_mut()[(node_index.wrapping_sub(1u32) as (usize))]).index_left_ = left as (i16);
              (tree.slice_mut()[(node_index.wrapping_sub(1u32) as (usize))]).index_right_or_value_ = right as
                                                                                         (i16);
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
            if depth[(symbols[j as usize] as (usize))] as (i32) < depth[(symbols[i as usize] as (usize)) as usize] as (i32) {
              let mut brotli_swap_tmp: u64 = symbols[j as usize];
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


pub enum ContextType {
  CONTEXT_LSB6 = 0,
  CONTEXT_MSB6 = 1,
  CONTEXT_UTF8 = 2,
  CONTEXT_SIGNED = 3,
}

#[derive(Clone, Copy)]
pub struct Command {
  pub insert_len_: u32,
  pub copy_len_: u32,
  pub dist_extra_: u32,
  pub cmd_prefix_: u16,
  pub dist_prefix_: u16,
}

#[derive(Clone, Copy)]
pub struct BlockSplit {
  pub num_types: usize,
  pub num_blocks: usize,
  pub types: *mut u8,
  pub lengths: *mut u32,
  pub types_alloc_size: usize,
  pub lengths_alloc_size: usize,
}


pub struct HistogramLiteral {
  pub data_: [u32; 256],
  pub total_count_: usize,
  pub bit_cost_: f64,
}
impl SliceWrapper<u32> for HistogramLiteral {
  fn slice(&self) -> &[u32] {
    return &self.data_[..];
  }
}
impl SliceWrapperMut<u32> for HistogramLiteral {
  fn slice_mut(&mut self) -> &mut [u32] {
    return &mut self.data_[..];
  }
}
pub struct HistogramCommand {
  pub data_: [u32; 704],
  pub total_count_: usize,
  pub bit_cost_: f64,
}

impl SliceWrapper<u32> for HistogramCommand {
  fn slice(&self) -> &[u32] {
    return &self.data_[..];
  }
}
impl SliceWrapperMut<u32> for HistogramCommand {
  fn slice_mut(&mut self) -> &mut [u32] {
    return &mut self.data_[..];
  }
}
pub struct HistogramDistance {
  pub data_: [u32; 520],
  pub total_count_: usize,
  pub bit_cost_: f64,
}
impl SliceWrapper<u32> for HistogramDistance {
  fn slice(&self) -> &[u32] {
    return &self.data_[..];
  }
}
impl SliceWrapperMut<u32> for HistogramDistance {
  fn slice_mut(&mut self) -> &mut [u32] {
    return &mut self.data_[..];
  }
}

#[derive(Clone, Copy)]
pub struct MetaBlockSplit {
  pub literal_split: BlockSplit,
  pub command_split: BlockSplit,
  pub distance_split: BlockSplit,
  pub literal_context_map: *mut u32,
  pub literal_context_map_size: usize,
  pub distance_context_map: *mut u32,
  pub distance_context_map_size: usize,
  pub literal_histograms: *mut HistogramLiteral,
  pub literal_histograms_size: usize,
  pub command_histograms: *mut HistogramCommand,
  pub command_histograms_size: usize,
  pub distance_histograms: *mut HistogramDistance,
  pub distance_histograms_size: usize,
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

pub struct BlockEncoder<AllocU8: alloc::Allocator<u8>,
                        AllocU16: alloc::Allocator<u16>,
                        AllocU32: alloc::Allocator<u32>>
{
  /*    pub alloc_u8 : AllocU8,
    pub alloc_u16 : AllocU16,
    pub alloc_u32 : AllocU32,
    pub alloc_ht : AllocHT,*/
  pub alphabet_size_: usize,
  pub num_block_types_: usize,
  pub block_types_: AllocU8::AllocatedMemory,
  pub block_lengths_: AllocU32::AllocatedMemory,
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

fn BrotliEncodeMlen(mut length: u32,
                    mut bits: &mut u64,
                    mut numbits: &mut u32,
                    mut nibblesbits: &mut u32) {
  let mut lg: u32 = (if length == 1u32 {
                       1u32
                     } else {
                       Log2FloorNonZero(length.wrapping_sub(1u32) as (u32) as (u64))
                         .wrapping_add(1u32)
                     }) as (u32);
  let mut mnibbles: u32 = (if lg < 16u32 {
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


fn StoreCompressedMetaBlockHeader(mut is_final_block: i32,
                                  mut length: usize,
                                  mut storage_ix: &mut usize,
                                  mut storage: &mut [u8]) {
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

fn NewBlockEncoder<AllocU8: alloc::Allocator<u8>,
                   AllocU16: alloc::Allocator<u16>,
                   AllocU32: alloc::Allocator<u32>>
  (mut alphabet_size: usize,
   mut num_block_types: usize,
   mut block_types: AllocU8::AllocatedMemory,
   mut block_lengths: AllocU32::AllocatedMemory,
   num_blocks: usize)
   -> BlockEncoder<AllocU8, AllocU16, AllocU32> {
  let block_len: usize;
  if num_blocks != 0 && block_lengths.slice().len() != 0 {
    block_len = block_lengths.slice()[0] as usize;
  } else {
    block_len = 0;
  }
  return BlockEncoder::<AllocU8, AllocU16, AllocU32> {
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







extern "C" fn NextBlockTypeCode(mut calculator: &mut BlockTypeCodeCalculator,
                                mut type_: u8)
                                -> usize {
  let mut type_code: usize = (if type_ as (usize) ==
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

fn BlockLengthPrefixCode(mut len: u32) -> u32 {
  let mut code: u32 = (if len >= 177u32 {
                         if len >= 753u32 { 20i32 } else { 14i32 }
                       } else if len >= 41u32 {
    7i32
  } else {
    0i32
  }) as (u32);
  'loop1: loop {
    if code < (26i32 - 1i32) as (u32) &&
       (len >= kBlockLengthPrefixCode[code.wrapping_add(1u32) as (usize)].offset) {
      code = code.wrapping_add(1 as (u32));
      continue 'loop1;
    } else {
      break 'loop1;
    }
  }
  code
}

fn StoreVarLenUint8(mut n: u64, mut storage_ix: &mut usize, mut storage: &mut [u8]) {
  if n == 0 {
    BrotliWriteBits(1, 0, storage_ix, storage);
  } else {
    let mut nbits: u8 = Log2FloorNonZero(n) as (u8);
    BrotliWriteBits(1, 1, storage_ix, storage);
    BrotliWriteBits(3, nbits as u64, storage_ix, storage);
    BrotliWriteBits(nbits, n.wrapping_sub(1u64 << nbits), storage_ix, storage);
  }
}


fn StoreSimpleHuffmanTree(mut depths: &[u8],
                          mut symbols: &mut [usize],
                          mut num_symbols: usize,
                          mut max_bits_usize: usize,
                          mut storage_ix: &mut usize,
                          mut storage: &mut [u8]) {
  let max_bits: u8 = max_bits_usize as u8;
  BrotliWriteBits(2, 1, storage_ix, storage);
  BrotliWriteBits(2,
                  num_symbols.wrapping_sub(1usize) as u64,
                  storage_ix,
                  storage);
  let mut i: usize;
  i = 0usize;
  'loop1: loop {
    if i < num_symbols {
      let mut j: usize;
      j = i.wrapping_add(1usize);
      'loop9: loop {
        if j < num_symbols {
          if depths[symbols[j as (usize)] as (usize)] as (i32) <
             depths[symbols[i as (usize)] as (usize)] as (i32) {
            let mut __brotli_swap_tmp: usize = symbols[j as (usize)];
            symbols[j as (usize)] = symbols[i as (usize)];
            symbols[i as (usize)] = __brotli_swap_tmp;
          }
          j = j.wrapping_add(1 as (usize));
          continue 'loop9;
        } else {
          break 'loop9;
        }
      }
      i = i.wrapping_add(1 as (usize));
      continue 'loop1;
    } else {
      break 'loop1;
    }
  }
  if num_symbols == 2usize {
    BrotliWriteBits(max_bits,
                    symbols[0i32 as (usize)] as u64,
                    storage_ix,
                    storage);
    BrotliWriteBits(max_bits,
                    symbols[1i32 as (usize)] as u64,
                    storage_ix,
                    storage);
  } else if num_symbols == 3usize {
    BrotliWriteBits(max_bits,
                    symbols[0i32 as (usize)] as u64,
                    storage_ix,
                    storage);
    BrotliWriteBits(max_bits,
                    symbols[1i32 as (usize)] as u64,
                    storage_ix,
                    storage);
    BrotliWriteBits(max_bits,
                    symbols[2i32 as (usize)] as u64,
                    storage_ix,
                    storage);
  } else {
    BrotliWriteBits(max_bits,
                    symbols[0i32 as (usize)] as u64,
                    storage_ix,
                    storage);
    BrotliWriteBits(max_bits,
                    symbols[1i32 as (usize)] as u64,
                    storage_ix,
                    storage);
    BrotliWriteBits(max_bits,
                    symbols[2i32 as (usize)] as u64,
                    storage_ix,
                    storage);
    BrotliWriteBits(max_bits,
                    symbols[3i32 as (usize)] as u64,
                    storage_ix,
                    storage);
    BrotliWriteBits(1,
                    if depths[symbols[0i32 as (usize)] as (usize)] as (i32) == 1i32 {
                      1
                    } else {
                      0
                    } as (u64),
                    storage_ix,
                    storage);
  }
}

fn BuildAndStoreHuffmanTree(mut histogram: &[u32],
                            length: usize,
                            mut tree: &mut [HuffmanTree],
                            mut depth: &mut [u8],
                            mut bits: &mut [u16],
                            mut storage_ix: &mut usize,
                            mut storage: &mut [u8]) {
  let mut count: usize = 0usize;
  let mut s4: [usize; 4] = [0; 4];
  let mut i: usize = 0;
  let mut max_bits: usize = 0usize;
  'loop1: loop {
    if i < length {
      if histogram[i as (usize)] != 0 {
        if count < 4usize {
          s4[count] = i;
        } else if count > 4usize {
          break 'loop1;
        }
        count = count.wrapping_add(1 as (usize));
      }
      i = i.wrapping_add(1);
      continue 'loop1;
    } else {
      break 'loop1;
    }
  }
  let mut max_bits_counter: usize = length.wrapping_sub(1usize);
  'loop6: loop {
    if max_bits_counter != 0 {
      max_bits_counter = max_bits_counter >> 1i32;
      max_bits = max_bits.wrapping_add(1 as (usize));
      continue 'loop6;
    } else {
      break 'loop6;
    }
  }
  if count <= 1usize {
    BrotliWriteBits(4, 1, storage_ix, storage);
    BrotliWriteBits(max_bits as u8, s4[0] as u64, storage_ix, storage);
    depth[s4[0usize] as (usize)] = 0i32 as (u8);
    bits[s4[0usize] as (usize)] = 0i32 as (u16);
  } else {
    for depth_elem in depth[..length].iter_mut() {
      *depth_elem = 0;
    }
    BrotliCreateHuffmanTree(histogram, length, 15i32, tree, depth);
    BrotliConvertBitDepthsToSymbols(depth, length, bits);
    if count <= 4usize {
      StoreSimpleHuffmanTree(depth, &mut s4[..], count, max_bits, storage_ix, storage);
    } else {
      BrotliStoreHuffmanTree(depth, length, tree, storage_ix, storage);
    }
  }
}
