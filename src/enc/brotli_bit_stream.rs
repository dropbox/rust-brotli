use super::constants::{BROTLI_NUM_BLOCK_LEN_SYMBOLS, kZeroRepsBits, kZeroRepsDepth,
                       kNonZeroRepsBits, kNonZeroRepsDepth, kCodeLengthBits, kCodeLengthDepth};
use super::entropy_encode::{HuffmanTree, BrotliWriteHuffmanTree, BrotliCreateHuffmanTree,
                            BrotliConvertBitDepthsToSymbols, NewHuffmanTree, InitHuffmanTree,
                            SortHuffmanTreeItems, SortHuffmanTree, BrotliSetDepth};
use super::histogram::{HistogramClear, HistogramAddItem,HistogramLiteral,HistogramCommand,HistogramDistance};
use super::super::alloc;
use super::super::alloc::{SliceWrapper,SliceWrapperMut};
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
    mut m : &mut AllocHT,
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

#[derive(Clone, Copy)]
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

pub struct BlockSplit<AllocU8: alloc::Allocator<u8>,
                        AllocU32: alloc::Allocator<u32> > {
  pub num_types: usize,
  pub num_blocks: usize,
  pub types: AllocU8::AllocatedMemory,
  pub lengths: AllocU32::AllocatedMemory,
}
/*

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
*/
pub struct MetaBlockSplit<AllocU8: alloc::Allocator<u8>,
                        AllocU32: alloc::Allocator<u32>,
                        AllocHL: alloc::Allocator<HistogramLiteral>,
                        AllocHC: alloc::Allocator<HistogramCommand>,
                        AllocHD: alloc::Allocator<HistogramDistance>>{
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

pub struct BlockEncoder<'a,
                        AllocU8: alloc::Allocator<u8>,
                        AllocU16: alloc::Allocator<u16>>
{
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

fn NewBlockEncoder<'a, AllocU8: alloc::Allocator<u8>,
                   AllocU16: alloc::Allocator<u16>>
  (mut alphabet_size: usize,
   mut num_block_types: usize,
   mut block_types: &'a [u8],
   mut block_lengths: &'a [u32],
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
  while code < (26i32 - 1i32) as (u32) &&
        (len >= kBlockLengthPrefixCode[code.wrapping_add(1u32) as (usize)].offset) {
    code = code.wrapping_add(1 as (u32));
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
                          mut max_bits: usize,
                          mut storage_ix: &mut usize,
                          mut storage: &mut [u8]) {
  BrotliWriteBits(2, 1, storage_ix, storage);
  BrotliWriteBits(2,
                  num_symbols.wrapping_sub(1) as u64,
                  storage_ix,
                  storage);
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
    BrotliWriteBits(max_bits as u8, symbols[(0usize)] as u64, storage_ix, storage);
    BrotliWriteBits(max_bits as u8, symbols[(1usize)] as u64, storage_ix, storage);
  } else if num_symbols == 3usize {
    BrotliWriteBits(max_bits as u8, symbols[(0usize)] as u64, storage_ix, storage);
    BrotliWriteBits(max_bits as u8, symbols[(1usize)] as u64, storage_ix, storage);
    BrotliWriteBits(max_bits as u8, symbols[(2usize)] as u64, storage_ix, storage);
  } else {
    BrotliWriteBits(max_bits as u8, symbols[(0usize)] as u64, storage_ix, storage);
    BrotliWriteBits(max_bits as u8, symbols[(1usize)] as u64, storage_ix, storage);
    BrotliWriteBits(max_bits as u8, symbols[(2usize)] as u64, storage_ix, storage);
    BrotliWriteBits(max_bits as u8, symbols[(3usize)] as u64, storage_ix, storage);
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

fn BuildAndStoreHuffmanTree(mut histogram: &[u32],
                            length: usize,
                            mut tree: &mut [HuffmanTree],
                            mut depth: &mut [u8],
                            mut bits: &mut [u16],
                            mut storage_ix: &mut usize,
                            mut storage: &mut [u8]) {
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

fn GetBlockLengthPrefixCode(mut len: u32,
                            mut code: &mut usize,
                            mut n_extra: &mut u32,
                            mut extra: &mut u32) {
  *code = BlockLengthPrefixCode(len) as (usize);
  *n_extra = kBlockLengthPrefixCode[*code].nbits;
  *extra = len.wrapping_sub(kBlockLengthPrefixCode[*code].offset);
}

fn StoreBlockSwitch(mut code: &mut BlockSplitCode,
                    block_len: u32,
                    block_type: u8,
                    mut is_first_block: i32,
                    mut storage_ix: &mut usize,
                    mut storage: &mut [u8]) {
  let mut typecode: usize = NextBlockTypeCode(&mut (*code).type_code_calculator, block_type);
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
  BrotliWriteBits(len_nextra as (u8),
                  len_extra as (u64),
                  storage_ix,
                  storage);
}

fn BuildAndStoreBlockSplitCode(mut types: &[u8],
                               mut lengths: &[u32],
                               num_blocks: usize,
                               num_types: usize,
                               mut tree: &mut [HuffmanTree],
                               mut code: &mut BlockSplitCode,
                               mut storage_ix: &mut usize,
                               mut storage: &mut [u8]) {
  let mut type_histo: [u32; 258] = [0;258];
  let mut length_histo: [u32; 26] = [0;26];
  let mut i: usize;
  let mut type_code_calculator = NewBlockTypeCodeCalculator();
  i = 0usize;
  while i < num_blocks {
    {
      let mut type_code: usize = NextBlockTypeCode(&mut type_code_calculator,
                                                   types[(i as (usize))]);
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

fn BuildAndStoreBlockSwitchEntropyCodes<'a, AllocU8: alloc::Allocator<u8>,
                        AllocU16: alloc::Allocator<u16>>(mut xself: &mut BlockEncoder<'a, AllocU8, AllocU16>,
                                        mut tree: &mut [HuffmanTree],
                                        mut storage_ix: &mut usize,
                                        mut storage: &mut [u8]) {
  BuildAndStoreBlockSplitCode((*xself).block_types_,
                              (*xself).block_lengths_,
                              (*xself).num_blocks_,
                              (*xself).num_block_types_,
                              tree,
                              &mut (*xself).block_split_code_,
                              storage_ix,
                              storage);
}

fn StoreTrivialContextMap(mut num_types: usize,
                          mut context_bits: usize,
                          mut tree: &mut [HuffmanTree],
                          mut storage_ix: &mut usize,
                          mut storage: &mut [u8]) {
  StoreVarLenUint8(num_types.wrapping_sub(1usize) as u64, storage_ix, storage);
  if num_types > 1usize {
    let mut repeat_code: usize = context_bits.wrapping_sub(1u32 as (usize));
    let mut repeat_bits: usize = (1u32 << repeat_code).wrapping_sub(1u32) as (usize);
    let mut alphabet_size: usize = num_types.wrapping_add(repeat_code);
    let mut histogram: [u32; 272] = [0;272];
    let mut depths: [u8; 272] = [0;272];
    let mut bits: [u16; 272] = [0;272];
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
        let mut code: usize = if i == 0usize {
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

fn IndexOf(mut v: &[u8], mut v_size: usize, mut value: u8) -> usize {
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

fn MoveToFront(mut v: &mut [u8], mut index: usize) {
  let mut value: u8 = v[(index as (usize))];
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

fn MoveToFrontTransform(mut v_in: &[u32], v_size: usize, mut v_out: &mut [u32]) {
  let mut i: usize;
  let mut mtf: [u8; 256] = [0;256];
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
    let mut mtf_size: usize = max_value.wrapping_add(1u32) as (usize);
    i = 0usize;
    while i < v_size {
      {
        let mut index: usize = IndexOf(&mtf[..], mtf_size, v_in[(i as (usize))] as (u8));
        0i32;
        v_out[(i as (usize))] = index as (u32);
        MoveToFront(&mut mtf[..], index);
      }
      i = i.wrapping_add(1 as (usize));
    }
  }
}

fn brotli_max_uint32_t(mut a: u32, mut b: u32) -> u32 {
  if a > b { a } else { b }
}

fn brotli_min_uint32_t(mut a: u32, mut b: u32) -> u32 {
  if a < b { a } else { b }
}

fn RunLengthCodeZeros(in_size: usize,
                      mut v: &mut [u32],
                      mut out_size: &mut usize,
                      mut max_run_length_prefix: &mut u32) {
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
          let mut run_length_prefix: u32 = Log2FloorNonZero(reps as (u64));
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

fn EncodeContextMap<AllocU32: alloc::Allocator<u32>>(mut m: &mut AllocU32,
                    mut context_map: &[u32],
                    mut context_map_size: usize,
                    mut num_clusters: usize,
                    mut tree: &mut [HuffmanTree],
                    mut storage_ix: &mut usize,
                    mut storage: &mut [u8]) {
  let mut i: usize;
  let mut rle_symbols: AllocU32::AllocatedMemory;
  let mut max_run_length_prefix: u32 = 6u32;
  let mut num_rle_symbols: usize = 0usize;
  static kSymbolMask: u32 = (1u32 << 9i32) - 1;
  let mut depths: [u8; 272] = [0;272];
  let mut bits: [u16; 272] = [0;272];
  StoreVarLenUint8(num_clusters.wrapping_sub(1usize) as u64, storage_ix, storage);
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
  let mut histogram: [u32; 272] = [0;272];
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
    let mut use_rle: i32 = if !!(max_run_length_prefix > 0u32) {
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
                              HistogramType:SliceWrapper<u32> >(mut m8: &mut AllocU8,
                                   mut m16: &mut AllocU16,
                                   mut xself: &mut BlockEncoder<AllocU8, AllocU16>,
                                    mut histograms: &[HistogramType],
                                    histograms_size: usize,
                                    mut tree: &mut [HuffmanTree],
                                    mut storage_ix: &mut usize,
                                    mut storage: &mut [u8]) {
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
        let mut ix: usize = i.wrapping_mul(alphabet_size);
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
                             (mut xself: &mut BlockEncoder<AllocU8, AllocU16>,
               mut symbol: usize,
               mut storage_ix: &mut usize,
               mut storage: &mut [u8]) {
  if (*xself).block_len_ == 0usize {
    let mut block_ix: usize = {
      (*xself).block_ix_ = (*xself).block_ix_.wrapping_add(1 as (usize));
      (*xself).block_ix_
    };
    let mut block_len: u32 = (*xself).block_lengths_[(block_ix as (usize))];
    let mut block_type: u8 = (*xself).block_types_[(block_ix as (usize))];
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
    let mut ix: usize = (*xself).entropy_ix_.wrapping_add(symbol);
    BrotliWriteBits((*xself).depths_.slice()[(ix as (usize))] as (u8),
                    (*xself).bits_.slice()[(ix as (usize))] as (u64),
                    storage_ix,
                    storage);
  }
}

fn CommandCopyLenCode(mut xself: &Command) -> u32 {
  (*xself).copy_len_ & 0xffffffu32 ^ (*xself).copy_len_ >> 24i32
}
fn GetInsertLengthCode(mut insertlen: usize) -> u16 {
  if insertlen < 6usize {
    insertlen as (u16)
  } else if insertlen < 130usize {
    let mut nbits: u32 = Log2FloorNonZero(insertlen.wrapping_sub(2usize) as u64).wrapping_sub(1u32);
    ((nbits << 1i32) as (usize))
      .wrapping_add(insertlen.wrapping_sub(2usize) >> nbits)
      .wrapping_add(2usize) as (u16)
  } else if insertlen < 2114usize {
    Log2FloorNonZero(insertlen.wrapping_sub(66usize) as u64).wrapping_add(10u32) as (u16)
  } else if insertlen < 6210usize {
    21u32 as (u16)
  } else if insertlen < 22594usize {
    22u32 as (u16)
  } else {
    23u32 as (u16)
  }
}

fn GetCopyLengthCode(mut copylen: usize) -> u16 {
  if copylen < 10usize {
    copylen.wrapping_sub(2usize) as (u16)
  } else if copylen < 134usize {
    let mut nbits: u32 = Log2FloorNonZero(copylen.wrapping_sub(6usize) as u64).wrapping_sub(1u32);
    ((nbits << 1i32) as (usize))
      .wrapping_add(copylen.wrapping_sub(6usize) >> nbits)
      .wrapping_add(4usize) as (u16)
  } else if copylen < 2118usize {
    Log2FloorNonZero(copylen.wrapping_sub(70usize) as u64).wrapping_add(12u32) as (u16)
  } else {
    23u32 as (u16)
  }
}
static kInsBase: [u32; 24] = [0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 8u32, 10u32, 14u32,
                                  18u32, 26u32, 34u32, 50u32, 66u32, 98u32, 130u32, 194u32,
                                  322u32, 578u32, 1090u32, 2114u32, 6210u32, 22594u32];

static kInsExtra: [u32; 24] = [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 1u32, 2u32, 2u32,
                                   3u32, 3u32, 4u32, 4u32, 5u32, 5u32, 6u32, 7u32, 8u32, 9u32,
                                   10u32, 12u32, 14u32, 24u32];

static kCopyBase: [u32; 24] = [2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 12u32,
                                   14u32, 18u32, 22u32, 30u32, 38u32, 54u32, 70u32, 102u32,
                                   134u32, 198u32, 326u32, 582u32, 1094u32, 2118u32];

static kCopyExtra: [u32; 24] = [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 1u32,
                                    2u32, 2u32, 3u32, 3u32, 4u32, 4u32, 5u32, 5u32, 6u32, 7u32,
                                    8u32, 9u32, 10u32, 24u32];

fn GetInsertExtra(mut inscode: u16) -> u32 {
  kInsExtra[inscode as (usize)]
}

fn GetInsertBase(mut inscode: u16) -> u32 {
  kInsBase[inscode as (usize)]
}

fn GetCopyBase(mut copycode: u16) -> u32 {
  kCopyBase[copycode as (usize)]
}

fn GetCopyExtra(mut copycode: u16) -> u32 {
  kCopyExtra[copycode as (usize)]
}

fn StoreCommandExtra(mut cmd: &Command, mut storage_ix: &mut usize, mut storage: &mut [u8]) {
  let mut copylen_code: u32 = CommandCopyLenCode(cmd);
  let mut inscode: u16 = GetInsertLengthCode((*cmd).insert_len_ as (usize));
  let mut copycode: u16 = GetCopyLengthCode(copylen_code as (usize));
  let mut insnumextra: u32 = GetInsertExtra(inscode);
  let mut insextraval: u64 = (*cmd).insert_len_.wrapping_sub(GetInsertBase(inscode)) as (u64);
  let mut copyextraval: u64 = copylen_code.wrapping_sub(GetCopyBase(copycode)) as (u64);
  let mut bits: u64 = copyextraval << insnumextra | insextraval;
  BrotliWriteBits(insnumextra.wrapping_add(GetCopyExtra(copycode)) as (u8),
                  bits,
                  storage_ix,
                  storage);
}

fn Context(mut p1: u8, mut p2: u8, mut mode: ContextType) -> u8 {
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
  0i32 as (u8)
}
static kUTF8ContextLookup: [u8; 512] = [0i32 as (u8),
                                            0i32 as (u8),
                                            0i32 as (u8),
                                            0i32 as (u8),
                                            0i32 as (u8),
                                            0i32 as (u8),
                                            0i32 as (u8),
                                            0i32 as (u8),
                                            0i32 as (u8),
                                            4i32 as (u8),
                                            4i32 as (u8),
                                            0i32 as (u8),
                                            0i32 as (u8),
                                            4i32 as (u8),
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
                                            0i32 as (u8),
                                            0i32 as (u8),
                                            8i32 as (u8),
                                            12i32 as (u8),
                                            16i32 as (u8),
                                            12i32 as (u8),
                                            12i32 as (u8),
                                            20i32 as (u8),
                                            12i32 as (u8),
                                            16i32 as (u8),
                                            24i32 as (u8),
                                            28i32 as (u8),
                                            12i32 as (u8),
                                            12i32 as (u8),
                                            32i32 as (u8),
                                            12i32 as (u8),
                                            36i32 as (u8),
                                            12i32 as (u8),
                                            44i32 as (u8),
                                            44i32 as (u8),
                                            44i32 as (u8),
                                            44i32 as (u8),
                                            44i32 as (u8),
                                            44i32 as (u8),
                                            44i32 as (u8),
                                            44i32 as (u8),
                                            44i32 as (u8),
                                            44i32 as (u8),
                                            32i32 as (u8),
                                            32i32 as (u8),
                                            24i32 as (u8),
                                            40i32 as (u8),
                                            28i32 as (u8),
                                            12i32 as (u8),
                                            12i32 as (u8),
                                            48i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            48i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            48i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            48i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            48i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            52i32 as (u8),
                                            24i32 as (u8),
                                            12i32 as (u8),
                                            28i32 as (u8),
                                            12i32 as (u8),
                                            12i32 as (u8),
                                            12i32 as (u8),
                                            56i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            56i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            56i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            56i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            56i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            60i32 as (u8),
                                            24i32 as (u8),
                                            12i32 as (u8),
                                            28i32 as (u8),
                                            12i32 as (u8),
                                            0i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
                                            2i32 as (u8),
                                            3i32 as (u8),
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
                                            0i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            3i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
                                            1i32 as (u8),
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
                                            0i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8),
                                            2i32 as (u8)];

static kSigned3BitContextLookup: [u8; 256] = [0i32 as (u8),
                                                  1i32 as (u8),
                                                  1i32 as (u8),
                                                  1i32 as (u8),
                                                  1i32 as (u8),
                                                  1i32 as (u8),
                                                  1i32 as (u8),
                                                  1i32 as (u8),
                                                  1i32 as (u8),
                                                  1i32 as (u8),
                                                  1i32 as (u8),
                                                  1i32 as (u8),
                                                  1i32 as (u8),
                                                  1i32 as (u8),
                                                  1i32 as (u8),
                                                  1i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  2i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  3i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
                                                  4i32 as (u8),
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
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  5i32 as (u8),
                                                  6i32 as (u8),
                                                  6i32 as (u8),
                                                  6i32 as (u8),
                                                  6i32 as (u8),
                                                  6i32 as (u8),
                                                  6i32 as (u8),
                                                  6i32 as (u8),
                                                  6i32 as (u8),
                                                  6i32 as (u8),
                                                  6i32 as (u8),
                                                  6i32 as (u8),
                                                  6i32 as (u8),
                                                  6i32 as (u8),
                                                  6i32 as (u8),
                                                  6i32 as (u8),
                                                  7i32 as (u8)];

fn StoreSymbolWithContext<AllocU8: alloc::Allocator<u8>,
                             AllocU16: alloc::Allocator<u16>>(mut xself: &mut BlockEncoder<AllocU8, AllocU16>,
                          mut symbol: usize,
                          mut context: usize,
                          mut context_map: &[u32],
                          mut storage_ix: &mut usize,
                          mut storage: &mut [u8],
                          context_bits: usize) {
  if (*xself).block_len_ == 0usize {
    let mut block_ix: usize = {
      (*xself).block_ix_ = (*xself).block_ix_.wrapping_add(1 as (usize));
      (*xself).block_ix_
    };
    let mut block_len: u32 = (*xself).block_lengths_[(block_ix as (usize))];
    let mut block_type: u8 = (*xself).block_types_[(block_ix as (usize))];
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
    let mut histo_ix: usize = context_map[((*xself).entropy_ix_.wrapping_add(context) as
     (usize))] as (usize);
    let mut ix: usize = histo_ix.wrapping_mul((*xself).alphabet_size_).wrapping_add(symbol);
    BrotliWriteBits((*xself).depths_.slice()[(ix as (usize))] as (u8),
                    (*xself).bits_.slice()[(ix as (usize))] as (u64),
                    storage_ix,
                    storage);
  }
}

fn CommandCopyLen(mut xself: &Command) -> u32 {
  (*xself).copy_len_ & 0xffffffu32
}

fn CommandDistanceContext(mut xself: &Command) -> u32 {
  let mut r: u32 = ((*xself).cmd_prefix_ as (i32) >> 6i32) as (u32);
  let mut c: u32 = ((*xself).cmd_prefix_ as (i32) & 7i32) as (u32);
  if (r == 0u32 || r == 2u32 || r == 4u32 || r == 7u32) && (c <= 2u32) {
    return c;
  }
  3u32
}

fn CleanupBlockEncoder<AllocU8: alloc::Allocator<u8>,
                        AllocU16: alloc::Allocator<u16>>(m8: &mut AllocU8,
                                  m16 : &mut AllocU16, mut xself: &mut BlockEncoder<AllocU8, AllocU16>) {
    m8.free_cell(core::mem::replace(&mut (*xself).depths_, AllocU8::AllocatedMemory::default()));
    m16.free_cell(core::mem::replace(&mut (*xself).bits_, AllocU16::AllocatedMemory::default()));
}

fn JumpToByteBoundary(mut storage_ix: &mut usize, mut storage: &mut [u8]) {
  *storage_ix = (*storage_ix).wrapping_add(7u32 as (usize)) & !7u32 as (usize);
  storage[((*storage_ix >> 3i32) as (usize))] = 0i32 as (u8);
}


pub fn BrotliStoreMetaBlock<AllocU8: alloc::Allocator<u8>,
                        AllocU16: alloc::Allocator<u16>,
                        AllocU32: alloc::Allocator<u32>,
                        AllocHT: alloc::Allocator<HuffmanTree>,
                        AllocHL: alloc::Allocator<HistogramLiteral>,
                        AllocHC: alloc::Allocator<HistogramCommand>,
                        AllocHD: alloc::Allocator<HistogramDistance> >(mut m8 : &mut AllocU8,
                        mut m16 : &mut AllocU16,
                        mut m32 : &mut AllocU32,
                            mut mht: &mut AllocHT,
                            mut input: &[u8],
                            mut start_pos: usize,
                            mut length: usize,
                            mut mask: usize,
                            mut prev_byte: u8,
                            mut prev_byte2: u8,
                            mut is_last: i32,
                            mut num_direct_distance_codes: u32,
                            mut distance_postfix_bits: u32,
                            mut literal_context_mode: ContextType,
                            mut commands: &[Command],
                            mut n_commands: usize,
                            mut mb: &mut MetaBlockSplit<AllocU8, AllocU32, AllocHL, AllocHC, AllocHD>,
                            mut storage_ix: &mut usize,
                            mut storage: &mut [u8]) {
  let mut pos: usize = start_pos;
  let mut i: usize;
  let mut num_distance_codes: usize = (16u32)
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
  literal_enc = NewBlockEncoder::<AllocU8, AllocU16>(
                   256usize,
                   (*mb).literal_split.num_types,
                   (*mb).literal_split.types.slice(),
                   (*mb).literal_split.lengths.slice(),
                   (*mb).literal_split.num_blocks);
  command_enc = NewBlockEncoder::<AllocU8, AllocU16>(
                   704usize,
                   (*mb).command_split.num_types,
                                      (*mb).command_split.types.slice(),
                                                         (*mb).command_split.lengths.slice(),
                   (*mb).command_split.num_blocks);
  distance_enc = NewBlockEncoder::<AllocU8, AllocU16>(
                   num_distance_codes,
                   (*mb).distance_split.num_types,
                   (*mb).distance_split.types.slice(),
                   (*mb).distance_split.lengths.slice(),
                   (*mb).distance_split.num_blocks);
  BuildAndStoreBlockSwitchEntropyCodes(&mut literal_enc, tree.slice_mut(), storage_ix, storage);
  BuildAndStoreBlockSwitchEntropyCodes(&mut command_enc, tree.slice_mut(), storage_ix, storage);
  BuildAndStoreBlockSwitchEntropyCodes(&mut distance_enc, tree.slice_mut(), storage_ix, storage);
  BrotliWriteBits(2,
                  distance_postfix_bits as (u64),
                  storage_ix,
                  storage);
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
  BuildAndStoreEntropyCodes(m8, m16,
                                   &mut literal_enc,
                                   (*mb).literal_histograms.slice(),
                                   (*mb).literal_histograms_size,
                                   tree.slice_mut(),
                                   storage_ix,
                                   storage);
  BuildAndStoreEntropyCodes(m8, m16,
                                   &mut command_enc,
                                   (*mb).command_histograms.slice(),
                                   (*mb).command_histograms_size,
                                   tree.slice_mut(),
                                   storage_ix,
                                   storage);
  BuildAndStoreEntropyCodes(m8, m16,
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
      let cmd: Command = commands[(i as (usize))];
      let mut cmd_code: usize = cmd.cmd_prefix_ as (usize);
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
            let mut context: usize = Context(prev_byte, prev_byte2, literal_context_mode) as
                                     (usize);
            let mut literal: u8 = input[((pos & mask) as (usize))];
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
          let mut dist_code: usize = cmd.dist_prefix_ as (usize);
          let mut distnumextra: u32 = cmd.dist_extra_ >> 24i32;
          let mut distextra: usize = (cmd.dist_extra_ & 0xffffffu32) as (usize);
          if (*mb).distance_context_map_size == 0usize {
            StoreSymbol(&mut distance_enc, dist_code, storage_ix, storage);
          } else {
            let mut context: usize = CommandDistanceContext(&cmd) as (usize);
            StoreSymbolWithContext(&mut distance_enc,
                                   dist_code,
                                   context,
                                   (*mb).distance_context_map.slice(),
                                   storage_ix,
                                   storage,
                                   2usize);
          }
          BrotliWriteBits(distnumextra as (u8), distextra as u64, storage_ix, storage);
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

fn BuildHistograms(mut input: &[u8],
                   mut start_pos: usize,
                   mut mask: usize,
                   mut commands: &[Command],
                   mut n_commands: usize,
                   mut lit_histo: &mut HistogramLiteral,
                   mut cmd_histo: &mut HistogramCommand,
                   mut dist_histo: &mut HistogramDistance) {
  let mut pos: usize = start_pos;
  let mut i: usize;
  i = 0usize;
  while i < n_commands {
    {
      let cmd: Command = commands[(i as (usize))];
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
fn StoreDataWithHuffmanCodes(mut input: &[u8],
                             mut start_pos: usize,
                             mut mask: usize,
                             mut commands: &[Command],
                             mut n_commands: usize,
                             mut lit_depth: &[u8],
                             mut lit_bits: &[u16],
                             mut cmd_depth: &[u8],
                             mut cmd_bits: &[u16],
                             mut dist_depth: &[u8],
                             mut dist_bits: &[u16],
                             mut storage_ix: &mut usize,
                             mut storage: &mut [u8]) {
  let mut pos: usize = start_pos;
  let mut i: usize;
  i = 0usize;
  while i < n_commands {
    {
      let cmd: Command = commands[(i as (usize))];
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
        let distextra: u32 = cmd.dist_extra_ & 0xffffffu32;
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

pub fn BrotliStoreMetaBlockTrivial(mut input: &[u8],
                                   mut start_pos: usize,
                                   mut length: usize,
                                   mut mask: usize,
                                   mut is_last: i32,
                                   mut commands: &[Command],
                                   mut n_commands: usize,
                                   mut storage_ix: &mut usize,
                                   mut storage: &mut [u8]) {
  let mut lit_histo: HistogramLiteral = HistogramLiteral::default();
  let mut cmd_histo: HistogramCommand = HistogramCommand::default();
  let mut dist_histo: HistogramDistance = HistogramDistance::default();
  let mut lit_depth: [u8; 256] = [0;256]; // FIXME these zero-initializations are costly
  let mut lit_bits: [u16; 256] = [0;256];
  let mut cmd_depth: [u8; 704] = [0; 704];
  let mut cmd_bits: [u16; 704] = [0;704];
  let mut dist_depth: [u8; 64] = [0;64];
  let mut dist_bits: [u16; 64] = [0;64];
  const MAX_HUFFMAN_TREE_SIZE :usize = (2i32 * 704i32 + 1i32) as usize;
  let mut tree :[HuffmanTree; MAX_HUFFMAN_TREE_SIZE] = [HuffmanTree{total_count_:0,index_left_:0,index_right_or_value_:0}; MAX_HUFFMAN_TREE_SIZE];
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

fn StoreStaticCommandHuffmanTree(mut storage_ix: &mut usize, mut storage: &mut [u8]) {
  BrotliWriteBits(56,
                  0x926244u32 as (u64) << 32i32 | 0x16307003,
                  storage_ix,
                  storage);
  BrotliWriteBits(3, 0x0u64, storage_ix, storage);
}

fn StoreStaticDistanceHuffmanTree(mut storage_ix: &mut usize, mut storage: &mut [u8]) {
  BrotliWriteBits(28, 0x369dc03u64, storage_ix, storage);
}


static kStaticCommandCodeDepth: [u8; 704] = [9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 9i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8),
                                                 11i32 as (u8)];

static kStaticDistanceCodeDepth: [u8; 64] = [6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8),
                                                 6i32 as (u8)];

static kStaticDistanceCodeBits: [u16; 64] = [0i32 as (u16),
                                                 32i32 as (u16),
                                                 16i32 as (u16),
                                                 48i32 as (u16),
                                                 8i32 as (u16),
                                                 40i32 as (u16),
                                                 24i32 as (u16),
                                                 56i32 as (u16),
                                                 4i32 as (u16),
                                                 36i32 as (u16),
                                                 20i32 as (u16),
                                                 52i32 as (u16),
                                                 12i32 as (u16),
                                                 44i32 as (u16),
                                                 28i32 as (u16),
                                                 60i32 as (u16),
                                                 2i32 as (u16),
                                                 34i32 as (u16),
                                                 18i32 as (u16),
                                                 50i32 as (u16),
                                                 10i32 as (u16),
                                                 42i32 as (u16),
                                                 26i32 as (u16),
                                                 58i32 as (u16),
                                                 6i32 as (u16),
                                                 38i32 as (u16),
                                                 22i32 as (u16),
                                                 54i32 as (u16),
                                                 14i32 as (u16),
                                                 46i32 as (u16),
                                                 30i32 as (u16),
                                                 62i32 as (u16),
                                                 1i32 as (u16),
                                                 33i32 as (u16),
                                                 17i32 as (u16),
                                                 49i32 as (u16),
                                                 9i32 as (u16),
                                                 41i32 as (u16),
                                                 25i32 as (u16),
                                                 57i32 as (u16),
                                                 5i32 as (u16),
                                                 37i32 as (u16),
                                                 21i32 as (u16),
                                                 53i32 as (u16),
                                                 13i32 as (u16),
                                                 45i32 as (u16),
                                                 29i32 as (u16),
                                                 61i32 as (u16),
                                                 3i32 as (u16),
                                                 35i32 as (u16),
                                                 19i32 as (u16),
                                                 51i32 as (u16),
                                                 11i32 as (u16),
                                                 43i32 as (u16),
                                                 27i32 as (u16),
                                                 59i32 as (u16),
                                                 7i32 as (u16),
                                                 39i32 as (u16),
                                                 23i32 as (u16),
                                                 55i32 as (u16),
                                                 15i32 as (u16),
                                                 47i32 as (u16),
                                                 31i32 as (u16),
                                                 63i32 as (u16)];
static kStaticCommandCodeBits: [u16; 704] = [0i32 as (u16),
                                                 256i32 as (u16),
                                                 128i32 as (u16),
                                                 384i32 as (u16),
                                                 64i32 as (u16),
                                                 320i32 as (u16),
                                                 192i32 as (u16),
                                                 448i32 as (u16),
                                                 32i32 as (u16),
                                                 288i32 as (u16),
                                                 160i32 as (u16),
                                                 416i32 as (u16),
                                                 96i32 as (u16),
                                                 352i32 as (u16),
                                                 224i32 as (u16),
                                                 480i32 as (u16),
                                                 16i32 as (u16),
                                                 272i32 as (u16),
                                                 144i32 as (u16),
                                                 400i32 as (u16),
                                                 80i32 as (u16),
                                                 336i32 as (u16),
                                                 208i32 as (u16),
                                                 464i32 as (u16),
                                                 48i32 as (u16),
                                                 304i32 as (u16),
                                                 176i32 as (u16),
                                                 432i32 as (u16),
                                                 112i32 as (u16),
                                                 368i32 as (u16),
                                                 240i32 as (u16),
                                                 496i32 as (u16),
                                                 8i32 as (u16),
                                                 264i32 as (u16),
                                                 136i32 as (u16),
                                                 392i32 as (u16),
                                                 72i32 as (u16),
                                                 328i32 as (u16),
                                                 200i32 as (u16),
                                                 456i32 as (u16),
                                                 40i32 as (u16),
                                                 296i32 as (u16),
                                                 168i32 as (u16),
                                                 424i32 as (u16),
                                                 104i32 as (u16),
                                                 360i32 as (u16),
                                                 232i32 as (u16),
                                                 488i32 as (u16),
                                                 24i32 as (u16),
                                                 280i32 as (u16),
                                                 152i32 as (u16),
                                                 408i32 as (u16),
                                                 88i32 as (u16),
                                                 344i32 as (u16),
                                                 216i32 as (u16),
                                                 472i32 as (u16),
                                                 56i32 as (u16),
                                                 312i32 as (u16),
                                                 184i32 as (u16),
                                                 440i32 as (u16),
                                                 120i32 as (u16),
                                                 376i32 as (u16),
                                                 248i32 as (u16),
                                                 504i32 as (u16),
                                                 4i32 as (u16),
                                                 260i32 as (u16),
                                                 132i32 as (u16),
                                                 388i32 as (u16),
                                                 68i32 as (u16),
                                                 324i32 as (u16),
                                                 196i32 as (u16),
                                                 452i32 as (u16),
                                                 36i32 as (u16),
                                                 292i32 as (u16),
                                                 164i32 as (u16),
                                                 420i32 as (u16),
                                                 100i32 as (u16),
                                                 356i32 as (u16),
                                                 228i32 as (u16),
                                                 484i32 as (u16),
                                                 20i32 as (u16),
                                                 276i32 as (u16),
                                                 148i32 as (u16),
                                                 404i32 as (u16),
                                                 84i32 as (u16),
                                                 340i32 as (u16),
                                                 212i32 as (u16),
                                                 468i32 as (u16),
                                                 52i32 as (u16),
                                                 308i32 as (u16),
                                                 180i32 as (u16),
                                                 436i32 as (u16),
                                                 116i32 as (u16),
                                                 372i32 as (u16),
                                                 244i32 as (u16),
                                                 500i32 as (u16),
                                                 12i32 as (u16),
                                                 268i32 as (u16),
                                                 140i32 as (u16),
                                                 396i32 as (u16),
                                                 76i32 as (u16),
                                                 332i32 as (u16),
                                                 204i32 as (u16),
                                                 460i32 as (u16),
                                                 44i32 as (u16),
                                                 300i32 as (u16),
                                                 172i32 as (u16),
                                                 428i32 as (u16),
                                                 108i32 as (u16),
                                                 364i32 as (u16),
                                                 236i32 as (u16),
                                                 492i32 as (u16),
                                                 28i32 as (u16),
                                                 284i32 as (u16),
                                                 156i32 as (u16),
                                                 412i32 as (u16),
                                                 92i32 as (u16),
                                                 348i32 as (u16),
                                                 220i32 as (u16),
                                                 476i32 as (u16),
                                                 60i32 as (u16),
                                                 316i32 as (u16),
                                                 188i32 as (u16),
                                                 444i32 as (u16),
                                                 124i32 as (u16),
                                                 380i32 as (u16),
                                                 252i32 as (u16),
                                                 508i32 as (u16),
                                                 2i32 as (u16),
                                                 258i32 as (u16),
                                                 130i32 as (u16),
                                                 386i32 as (u16),
                                                 66i32 as (u16),
                                                 322i32 as (u16),
                                                 194i32 as (u16),
                                                 450i32 as (u16),
                                                 34i32 as (u16),
                                                 290i32 as (u16),
                                                 162i32 as (u16),
                                                 418i32 as (u16),
                                                 98i32 as (u16),
                                                 354i32 as (u16),
                                                 226i32 as (u16),
                                                 482i32 as (u16),
                                                 18i32 as (u16),
                                                 274i32 as (u16),
                                                 146i32 as (u16),
                                                 402i32 as (u16),
                                                 82i32 as (u16),
                                                 338i32 as (u16),
                                                 210i32 as (u16),
                                                 466i32 as (u16),
                                                 50i32 as (u16),
                                                 306i32 as (u16),
                                                 178i32 as (u16),
                                                 434i32 as (u16),
                                                 114i32 as (u16),
                                                 370i32 as (u16),
                                                 242i32 as (u16),
                                                 498i32 as (u16),
                                                 10i32 as (u16),
                                                 266i32 as (u16),
                                                 138i32 as (u16),
                                                 394i32 as (u16),
                                                 74i32 as (u16),
                                                 330i32 as (u16),
                                                 202i32 as (u16),
                                                 458i32 as (u16),
                                                 42i32 as (u16),
                                                 298i32 as (u16),
                                                 170i32 as (u16),
                                                 426i32 as (u16),
                                                 106i32 as (u16),
                                                 362i32 as (u16),
                                                 234i32 as (u16),
                                                 490i32 as (u16),
                                                 26i32 as (u16),
                                                 282i32 as (u16),
                                                 154i32 as (u16),
                                                 410i32 as (u16),
                                                 90i32 as (u16),
                                                 346i32 as (u16),
                                                 218i32 as (u16),
                                                 474i32 as (u16),
                                                 58i32 as (u16),
                                                 314i32 as (u16),
                                                 186i32 as (u16),
                                                 442i32 as (u16),
                                                 122i32 as (u16),
                                                 378i32 as (u16),
                                                 250i32 as (u16),
                                                 506i32 as (u16),
                                                 6i32 as (u16),
                                                 262i32 as (u16),
                                                 134i32 as (u16),
                                                 390i32 as (u16),
                                                 70i32 as (u16),
                                                 326i32 as (u16),
                                                 198i32 as (u16),
                                                 454i32 as (u16),
                                                 38i32 as (u16),
                                                 294i32 as (u16),
                                                 166i32 as (u16),
                                                 422i32 as (u16),
                                                 102i32 as (u16),
                                                 358i32 as (u16),
                                                 230i32 as (u16),
                                                 486i32 as (u16),
                                                 22i32 as (u16),
                                                 278i32 as (u16),
                                                 150i32 as (u16),
                                                 406i32 as (u16),
                                                 86i32 as (u16),
                                                 342i32 as (u16),
                                                 214i32 as (u16),
                                                 470i32 as (u16),
                                                 54i32 as (u16),
                                                 310i32 as (u16),
                                                 182i32 as (u16),
                                                 438i32 as (u16),
                                                 118i32 as (u16),
                                                 374i32 as (u16),
                                                 246i32 as (u16),
                                                 502i32 as (u16),
                                                 14i32 as (u16),
                                                 270i32 as (u16),
                                                 142i32 as (u16),
                                                 398i32 as (u16),
                                                 78i32 as (u16),
                                                 334i32 as (u16),
                                                 206i32 as (u16),
                                                 462i32 as (u16),
                                                 46i32 as (u16),
                                                 302i32 as (u16),
                                                 174i32 as (u16),
                                                 430i32 as (u16),
                                                 110i32 as (u16),
                                                 366i32 as (u16),
                                                 238i32 as (u16),
                                                 494i32 as (u16),
                                                 30i32 as (u16),
                                                 286i32 as (u16),
                                                 158i32 as (u16),
                                                 414i32 as (u16),
                                                 94i32 as (u16),
                                                 350i32 as (u16),
                                                 222i32 as (u16),
                                                 478i32 as (u16),
                                                 62i32 as (u16),
                                                 318i32 as (u16),
                                                 190i32 as (u16),
                                                 446i32 as (u16),
                                                 126i32 as (u16),
                                                 382i32 as (u16),
                                                 254i32 as (u16),
                                                 510i32 as (u16),
                                                 1i32 as (u16),
                                                 257i32 as (u16),
                                                 129i32 as (u16),
                                                 385i32 as (u16),
                                                 65i32 as (u16),
                                                 321i32 as (u16),
                                                 193i32 as (u16),
                                                 449i32 as (u16),
                                                 33i32 as (u16),
                                                 289i32 as (u16),
                                                 161i32 as (u16),
                                                 417i32 as (u16),
                                                 97i32 as (u16),
                                                 353i32 as (u16),
                                                 225i32 as (u16),
                                                 481i32 as (u16),
                                                 17i32 as (u16),
                                                 273i32 as (u16),
                                                 145i32 as (u16),
                                                 401i32 as (u16),
                                                 81i32 as (u16),
                                                 337i32 as (u16),
                                                 209i32 as (u16),
                                                 465i32 as (u16),
                                                 49i32 as (u16),
                                                 305i32 as (u16),
                                                 177i32 as (u16),
                                                 433i32 as (u16),
                                                 113i32 as (u16),
                                                 369i32 as (u16),
                                                 241i32 as (u16),
                                                 497i32 as (u16),
                                                 9i32 as (u16),
                                                 265i32 as (u16),
                                                 137i32 as (u16),
                                                 393i32 as (u16),
                                                 73i32 as (u16),
                                                 329i32 as (u16),
                                                 201i32 as (u16),
                                                 457i32 as (u16),
                                                 41i32 as (u16),
                                                 297i32 as (u16),
                                                 169i32 as (u16),
                                                 425i32 as (u16),
                                                 105i32 as (u16),
                                                 361i32 as (u16),
                                                 233i32 as (u16),
                                                 489i32 as (u16),
                                                 25i32 as (u16),
                                                 281i32 as (u16),
                                                 153i32 as (u16),
                                                 409i32 as (u16),
                                                 89i32 as (u16),
                                                 345i32 as (u16),
                                                 217i32 as (u16),
                                                 473i32 as (u16),
                                                 57i32 as (u16),
                                                 313i32 as (u16),
                                                 185i32 as (u16),
                                                 441i32 as (u16),
                                                 121i32 as (u16),
                                                 377i32 as (u16),
                                                 249i32 as (u16),
                                                 505i32 as (u16),
                                                 5i32 as (u16),
                                                 261i32 as (u16),
                                                 133i32 as (u16),
                                                 389i32 as (u16),
                                                 69i32 as (u16),
                                                 325i32 as (u16),
                                                 197i32 as (u16),
                                                 453i32 as (u16),
                                                 37i32 as (u16),
                                                 293i32 as (u16),
                                                 165i32 as (u16),
                                                 421i32 as (u16),
                                                 101i32 as (u16),
                                                 357i32 as (u16),
                                                 229i32 as (u16),
                                                 485i32 as (u16),
                                                 21i32 as (u16),
                                                 277i32 as (u16),
                                                 149i32 as (u16),
                                                 405i32 as (u16),
                                                 85i32 as (u16),
                                                 341i32 as (u16),
                                                 213i32 as (u16),
                                                 469i32 as (u16),
                                                 53i32 as (u16),
                                                 309i32 as (u16),
                                                 181i32 as (u16),
                                                 437i32 as (u16),
                                                 117i32 as (u16),
                                                 373i32 as (u16),
                                                 245i32 as (u16),
                                                 501i32 as (u16),
                                                 13i32 as (u16),
                                                 269i32 as (u16),
                                                 141i32 as (u16),
                                                 397i32 as (u16),
                                                 77i32 as (u16),
                                                 333i32 as (u16),
                                                 205i32 as (u16),
                                                 461i32 as (u16),
                                                 45i32 as (u16),
                                                 301i32 as (u16),
                                                 173i32 as (u16),
                                                 429i32 as (u16),
                                                 109i32 as (u16),
                                                 365i32 as (u16),
                                                 237i32 as (u16),
                                                 493i32 as (u16),
                                                 29i32 as (u16),
                                                 285i32 as (u16),
                                                 157i32 as (u16),
                                                 413i32 as (u16),
                                                 93i32 as (u16),
                                                 349i32 as (u16),
                                                 221i32 as (u16),
                                                 477i32 as (u16),
                                                 61i32 as (u16),
                                                 317i32 as (u16),
                                                 189i32 as (u16),
                                                 445i32 as (u16),
                                                 125i32 as (u16),
                                                 381i32 as (u16),
                                                 253i32 as (u16),
                                                 509i32 as (u16),
                                                 3i32 as (u16),
                                                 259i32 as (u16),
                                                 131i32 as (u16),
                                                 387i32 as (u16),
                                                 67i32 as (u16),
                                                 323i32 as (u16),
                                                 195i32 as (u16),
                                                 451i32 as (u16),
                                                 35i32 as (u16),
                                                 291i32 as (u16),
                                                 163i32 as (u16),
                                                 419i32 as (u16),
                                                 99i32 as (u16),
                                                 355i32 as (u16),
                                                 227i32 as (u16),
                                                 483i32 as (u16),
                                                 19i32 as (u16),
                                                 275i32 as (u16),
                                                 147i32 as (u16),
                                                 403i32 as (u16),
                                                 83i32 as (u16),
                                                 339i32 as (u16),
                                                 211i32 as (u16),
                                                 467i32 as (u16),
                                                 51i32 as (u16),
                                                 307i32 as (u16),
                                                 179i32 as (u16),
                                                 435i32 as (u16),
                                                 115i32 as (u16),
                                                 371i32 as (u16),
                                                 243i32 as (u16),
                                                 499i32 as (u16),
                                                 11i32 as (u16),
                                                 267i32 as (u16),
                                                 139i32 as (u16),
                                                 395i32 as (u16),
                                                 75i32 as (u16),
                                                 331i32 as (u16),
                                                 203i32 as (u16),
                                                 459i32 as (u16),
                                                 43i32 as (u16),
                                                 299i32 as (u16),
                                                 171i32 as (u16),
                                                 427i32 as (u16),
                                                 107i32 as (u16),
                                                 363i32 as (u16),
                                                 235i32 as (u16),
                                                 491i32 as (u16),
                                                 27i32 as (u16),
                                                 283i32 as (u16),
                                                 155i32 as (u16),
                                                 411i32 as (u16),
                                                 91i32 as (u16),
                                                 347i32 as (u16),
                                                 219i32 as (u16),
                                                 475i32 as (u16),
                                                 59i32 as (u16),
                                                 315i32 as (u16),
                                                 187i32 as (u16),
                                                 443i32 as (u16),
                                                 123i32 as (u16),
                                                 379i32 as (u16),
                                                 251i32 as (u16),
                                                 507i32 as (u16),
                                                 7i32 as (u16),
                                                 1031i32 as (u16),
                                                 519i32 as (u16),
                                                 1543i32 as (u16),
                                                 263i32 as (u16),
                                                 1287i32 as (u16),
                                                 775i32 as (u16),
                                                 1799i32 as (u16),
                                                 135i32 as (u16),
                                                 1159i32 as (u16),
                                                 647i32 as (u16),
                                                 1671i32 as (u16),
                                                 391i32 as (u16),
                                                 1415i32 as (u16),
                                                 903i32 as (u16),
                                                 1927i32 as (u16),
                                                 71i32 as (u16),
                                                 1095i32 as (u16),
                                                 583i32 as (u16),
                                                 1607i32 as (u16),
                                                 327i32 as (u16),
                                                 1351i32 as (u16),
                                                 839i32 as (u16),
                                                 1863i32 as (u16),
                                                 199i32 as (u16),
                                                 1223i32 as (u16),
                                                 711i32 as (u16),
                                                 1735i32 as (u16),
                                                 455i32 as (u16),
                                                 1479i32 as (u16),
                                                 967i32 as (u16),
                                                 1991i32 as (u16),
                                                 39i32 as (u16),
                                                 1063i32 as (u16),
                                                 551i32 as (u16),
                                                 1575i32 as (u16),
                                                 295i32 as (u16),
                                                 1319i32 as (u16),
                                                 807i32 as (u16),
                                                 1831i32 as (u16),
                                                 167i32 as (u16),
                                                 1191i32 as (u16),
                                                 679i32 as (u16),
                                                 1703i32 as (u16),
                                                 423i32 as (u16),
                                                 1447i32 as (u16),
                                                 935i32 as (u16),
                                                 1959i32 as (u16),
                                                 103i32 as (u16),
                                                 1127i32 as (u16),
                                                 615i32 as (u16),
                                                 1639i32 as (u16),
                                                 359i32 as (u16),
                                                 1383i32 as (u16),
                                                 871i32 as (u16),
                                                 1895i32 as (u16),
                                                 231i32 as (u16),
                                                 1255i32 as (u16),
                                                 743i32 as (u16),
                                                 1767i32 as (u16),
                                                 487i32 as (u16),
                                                 1511i32 as (u16),
                                                 999i32 as (u16),
                                                 2023i32 as (u16),
                                                 23i32 as (u16),
                                                 1047i32 as (u16),
                                                 535i32 as (u16),
                                                 1559i32 as (u16),
                                                 279i32 as (u16),
                                                 1303i32 as (u16),
                                                 791i32 as (u16),
                                                 1815i32 as (u16),
                                                 151i32 as (u16),
                                                 1175i32 as (u16),
                                                 663i32 as (u16),
                                                 1687i32 as (u16),
                                                 407i32 as (u16),
                                                 1431i32 as (u16),
                                                 919i32 as (u16),
                                                 1943i32 as (u16),
                                                 87i32 as (u16),
                                                 1111i32 as (u16),
                                                 599i32 as (u16),
                                                 1623i32 as (u16),
                                                 343i32 as (u16),
                                                 1367i32 as (u16),
                                                 855i32 as (u16),
                                                 1879i32 as (u16),
                                                 215i32 as (u16),
                                                 1239i32 as (u16),
                                                 727i32 as (u16),
                                                 1751i32 as (u16),
                                                 471i32 as (u16),
                                                 1495i32 as (u16),
                                                 983i32 as (u16),
                                                 2007i32 as (u16),
                                                 55i32 as (u16),
                                                 1079i32 as (u16),
                                                 567i32 as (u16),
                                                 1591i32 as (u16),
                                                 311i32 as (u16),
                                                 1335i32 as (u16),
                                                 823i32 as (u16),
                                                 1847i32 as (u16),
                                                 183i32 as (u16),
                                                 1207i32 as (u16),
                                                 695i32 as (u16),
                                                 1719i32 as (u16),
                                                 439i32 as (u16),
                                                 1463i32 as (u16),
                                                 951i32 as (u16),
                                                 1975i32 as (u16),
                                                 119i32 as (u16),
                                                 1143i32 as (u16),
                                                 631i32 as (u16),
                                                 1655i32 as (u16),
                                                 375i32 as (u16),
                                                 1399i32 as (u16),
                                                 887i32 as (u16),
                                                 1911i32 as (u16),
                                                 247i32 as (u16),
                                                 1271i32 as (u16),
                                                 759i32 as (u16),
                                                 1783i32 as (u16),
                                                 503i32 as (u16),
                                                 1527i32 as (u16),
                                                 1015i32 as (u16),
                                                 2039i32 as (u16),
                                                 15i32 as (u16),
                                                 1039i32 as (u16),
                                                 527i32 as (u16),
                                                 1551i32 as (u16),
                                                 271i32 as (u16),
                                                 1295i32 as (u16),
                                                 783i32 as (u16),
                                                 1807i32 as (u16),
                                                 143i32 as (u16),
                                                 1167i32 as (u16),
                                                 655i32 as (u16),
                                                 1679i32 as (u16),
                                                 399i32 as (u16),
                                                 1423i32 as (u16),
                                                 911i32 as (u16),
                                                 1935i32 as (u16),
                                                 79i32 as (u16),
                                                 1103i32 as (u16),
                                                 591i32 as (u16),
                                                 1615i32 as (u16),
                                                 335i32 as (u16),
                                                 1359i32 as (u16),
                                                 847i32 as (u16),
                                                 1871i32 as (u16),
                                                 207i32 as (u16),
                                                 1231i32 as (u16),
                                                 719i32 as (u16),
                                                 1743i32 as (u16),
                                                 463i32 as (u16),
                                                 1487i32 as (u16),
                                                 975i32 as (u16),
                                                 1999i32 as (u16),
                                                 47i32 as (u16),
                                                 1071i32 as (u16),
                                                 559i32 as (u16),
                                                 1583i32 as (u16),
                                                 303i32 as (u16),
                                                 1327i32 as (u16),
                                                 815i32 as (u16),
                                                 1839i32 as (u16),
                                                 175i32 as (u16),
                                                 1199i32 as (u16),
                                                 687i32 as (u16),
                                                 1711i32 as (u16),
                                                 431i32 as (u16),
                                                 1455i32 as (u16),
                                                 943i32 as (u16),
                                                 1967i32 as (u16),
                                                 111i32 as (u16),
                                                 1135i32 as (u16),
                                                 623i32 as (u16),
                                                 1647i32 as (u16),
                                                 367i32 as (u16),
                                                 1391i32 as (u16),
                                                 879i32 as (u16),
                                                 1903i32 as (u16),
                                                 239i32 as (u16),
                                                 1263i32 as (u16),
                                                 751i32 as (u16),
                                                 1775i32 as (u16),
                                                 495i32 as (u16),
                                                 1519i32 as (u16),
                                                 1007i32 as (u16),
                                                 2031i32 as (u16),
                                                 31i32 as (u16),
                                                 1055i32 as (u16),
                                                 543i32 as (u16),
                                                 1567i32 as (u16),
                                                 287i32 as (u16),
                                                 1311i32 as (u16),
                                                 799i32 as (u16),
                                                 1823i32 as (u16),
                                                 159i32 as (u16),
                                                 1183i32 as (u16),
                                                 671i32 as (u16),
                                                 1695i32 as (u16),
                                                 415i32 as (u16),
                                                 1439i32 as (u16),
                                                 927i32 as (u16),
                                                 1951i32 as (u16),
                                                 95i32 as (u16),
                                                 1119i32 as (u16),
                                                 607i32 as (u16),
                                                 1631i32 as (u16),
                                                 351i32 as (u16),
                                                 1375i32 as (u16),
                                                 863i32 as (u16),
                                                 1887i32 as (u16),
                                                 223i32 as (u16),
                                                 1247i32 as (u16),
                                                 735i32 as (u16),
                                                 1759i32 as (u16),
                                                 479i32 as (u16),
                                                 1503i32 as (u16),
                                                 991i32 as (u16),
                                                 2015i32 as (u16),
                                                 63i32 as (u16),
                                                 1087i32 as (u16),
                                                 575i32 as (u16),
                                                 1599i32 as (u16),
                                                 319i32 as (u16),
                                                 1343i32 as (u16),
                                                 831i32 as (u16),
                                                 1855i32 as (u16),
                                                 191i32 as (u16),
                                                 1215i32 as (u16),
                                                 703i32 as (u16),
                                                 1727i32 as (u16),
                                                 447i32 as (u16),
                                                 1471i32 as (u16),
                                                 959i32 as (u16),
                                                 1983i32 as (u16),
                                                 127i32 as (u16),
                                                 1151i32 as (u16),
                                                 639i32 as (u16),
                                                 1663i32 as (u16),
                                                 383i32 as (u16),
                                                 1407i32 as (u16),
                                                 895i32 as (u16),
                                                 1919i32 as (u16),
                                                 255i32 as (u16),
                                                 1279i32 as (u16),
                                                 767i32 as (u16),
                                                 1791i32 as (u16),
                                                 511i32 as (u16),
                                                 1535i32 as (u16),
                                                 1023i32 as (u16),
                                                 2047i32 as (u16)];

pub fn BrotliStoreMetaBlockFast<AllocHT: alloc::Allocator<HuffmanTree>>(mut m : &mut AllocHT,
                                mut input: &[u8],
                                mut start_pos: usize,
                                mut length: usize,
                                mut mask: usize,
                                mut is_last: i32,
                                mut commands: &[Command],
                                mut n_commands: usize,
                                mut storage_ix: &mut usize,
                                mut storage: &mut [u8]) {
  StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
  BrotliWriteBits(13, 0, storage_ix, storage);
  if n_commands <= 128usize {
    let mut histogram: [u32; 256] = [0;256];
    let mut pos: usize = start_pos;
    let mut num_literals: usize = 0usize;
    let mut i: usize;
    let mut lit_depth: [u8; 256]=[0;256];
    let mut lit_bits: [u16; 256]=[0;256];
    i = 0usize;
    while i < n_commands {
      {
        let cmd: Command = commands[(i as (usize))];
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
    let mut lit_depth: [u8; 256] = [0;256];
    let mut lit_bits: [u16; 256] = [0;256];
    let mut cmd_depth: [u8; 704] = [0;704];
    let mut cmd_bits: [u16; 704] = [0;704];
    let mut dist_depth: [u8; 64] = [0;64];
    let mut dist_bits: [u16; 64] = [0;64];
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
fn BrotliStoreUncompressedMetaBlockHeader(mut length: usize,
                                          mut storage_ix: &mut usize,
                                          mut storage: &mut [u8]) {
  let mut lenbits: u64 = 0;
  let mut nlenbits: u32 = 0;
  let mut nibblesbits: u32 = 0;
  BrotliWriteBits(1, 0, storage_ix, storage);
  BrotliEncodeMlen(length as u32, &mut lenbits, &mut nlenbits, &mut nibblesbits);
  BrotliWriteBits(2, nibblesbits as u64, storage_ix, storage);
  BrotliWriteBits(nlenbits as u8, lenbits as u64, storage_ix, storage);
  BrotliWriteBits(1, 1, storage_ix, storage);
}


pub fn BrotliStoreUncompressedMetaBlock(mut is_final_block: i32,
                                        mut input: &[u8],
                                        mut position: usize,
                                        mut mask: usize,
                                        mut len: usize,
                                        mut storage_ix: &mut usize,
                                        mut storage: &mut [u8]) {
  let mut masked_pos: usize = position & mask;
  BrotliStoreUncompressedMetaBlockHeader(len, storage_ix, storage);
  JumpToByteBoundary(storage_ix, storage);
  if masked_pos.wrapping_add(len) > mask.wrapping_add(1usize) {
    let mut len1: usize = mask.wrapping_add(1usize).wrapping_sub(masked_pos);
    let dst_start = ((*storage_ix >> 3i32) as (usize));
    storage[dst_start..len1].clone_from_slice(&input[masked_pos..(masked_pos + len1)]);
    *storage_ix = (*storage_ix).wrapping_add(len1 << 3i32);
    len = len.wrapping_sub(len1);
    masked_pos = 0usize;
  }
  let dst_start = (*storage_ix >> 3i32) as (usize);
  storage[dst_start..dst_start + len].clone_from_slice(&input[masked_pos..masked_pos + len]);
  *storage_ix = (*storage_ix).wrapping_add(len << 3i32);
  BrotliWriteBitsPrepareStorage(*storage_ix, storage);
  if is_final_block != 0 {
    BrotliWriteBits(1u8, 1u64, storage_ix, storage);
    BrotliWriteBits(1u8, 1u64, storage_ix, storage);
    JumpToByteBoundary(storage_ix, storage);
  }
}


pub fn BrotliStoreSyncMetaBlock(mut storage_ix: &mut usize, mut storage: &mut [u8]) {
  BrotliWriteBits(6, 6, storage_ix, storage);
  JumpToByteBoundary(storage_ix, storage);
}
