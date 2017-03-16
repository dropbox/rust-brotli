use super::super::alloc;
use super::super::alloc::SliceWrapper;
use super::super::alloc::SliceWrapperMut;
use super::super::core;
use super::constants::{BROTLI_NUM_BLOCK_LEN_SYMBOLS,
                       kZeroRepsBits,kZeroRepsDepth,
                       kNonZeroRepsBits,kNonZeroRepsDepth,
                       kCodeLengthBits,kCodeLengthDepth,
};
use super::entropy_encode::{HuffmanTree,
                            BrotliWriteHuffmanTree, BrotliCreateHuffmanTree,
                            BrotliConvertBitDepthsToSymbols,
                            NewHuffmanTree, InitHuffmanTree,
                            SortHuffmanTreeItems, SortHuffmanTree, BrotliSetDepth,
};
pub struct PrefixCodeRange {
    pub offset : u32,
    pub nbits : u32,
}


static mut kBlockLengthPrefixCode
    : [PrefixCodeRange; BROTLI_NUM_BLOCK_LEN_SYMBOLS]
    = [   PrefixCodeRange {
              offset: 1u32,
              nbits: 2u32
          },
          PrefixCodeRange { offset: 5u32, nbits: 2u32 },
          PrefixCodeRange { offset: 9u32, nbits: 2u32 },
          PrefixCodeRange { offset: 13u32, nbits: 2u32 },
          PrefixCodeRange { offset: 17u32, nbits: 3u32 },
          PrefixCodeRange { offset: 25u32, nbits: 3u32 },
          PrefixCodeRange { offset: 33u32, nbits: 3u32 },
          PrefixCodeRange { offset: 41u32, nbits: 3u32 },
          PrefixCodeRange { offset: 49u32, nbits: 4u32 },
          PrefixCodeRange { offset: 65u32, nbits: 4u32 },
          PrefixCodeRange { offset: 81u32, nbits: 4u32 },
          PrefixCodeRange { offset: 97u32, nbits: 4u32 },
          PrefixCodeRange { offset: 113u32, nbits: 5u32 },
          PrefixCodeRange { offset: 145u32, nbits: 5u32 },
          PrefixCodeRange { offset: 177u32, nbits: 5u32 },
          PrefixCodeRange { offset: 209u32, nbits: 5u32 },
          PrefixCodeRange { offset: 241u32, nbits: 6u32 },
          PrefixCodeRange { offset: 305u32, nbits: 6u32 },
          PrefixCodeRange { offset: 369u32, nbits: 7u32 },
          PrefixCodeRange { offset: 497u32, nbits: 8u32 },
          PrefixCodeRange { offset: 753u32, nbits: 9u32 },
          PrefixCodeRange {
              offset: 1265u32,
              nbits: 10u32
          },
          PrefixCodeRange {
              offset: 2289u32,
              nbits: 11u32
          },
          PrefixCodeRange {
              offset: 4337u32,
              nbits: 12u32
          },
          PrefixCodeRange {
              offset: 8433u32,
              nbits: 13u32
          },
          PrefixCodeRange {
              offset: 16625u32,
              nbits: 24u32
          }
      ];

fn BrotliWriteBits(
    n_bits : u8,
    bits : u64,
    mut pos : &mut usize,
    mut array : &mut[u8]
) {
    assert!((bits >> n_bits as usize) == 0);
    assert!(n_bits <= 56);
    let ptr_offset : usize = ((*pos >> 3) as u32) as usize;
    let mut v = array[ptr_offset] as u64;
    v |= bits << ((*pos) as u64 & 7);
    array[ptr_offset + 7] = (v >> 56) as u8 ;
    array[ptr_offset + 6] = ((v >> 48) & 0xff) as u8;
    array[ptr_offset + 5] = ((v >> 40) & 0xff) as u8;
    array[ptr_offset + 4] = ((v >> 24) & 0xff) as u8;
    array[ptr_offset + 3] = ((v >> 16) & 0xff) as u8;
    array[ptr_offset + 2] = ((v >> 8) & 0xff) as u8;
    array[ptr_offset + 1] = ((v >> 4) & 0xff) as u8;
    array[ptr_offset] = (v & 0xff) as u8;
    *pos += n_bits as usize
}

fn BrotliWriteBitsPrepareStorage(
     pos : usize, mut array : &mut[u8]) {
     assert_eq!(pos & 7, 0);
    array[pos >> 3] = 0;
}

fn BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(
    num_codes : i32,
    code_length_bitdepth : &[u8],
    mut storage_ix : &mut usize,
    mut storage : &mut [u8]
) {
    static kStorageOrder
        : [u8; 18]
        = [   1i32 as (u8),
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
              15i32 as (u8)
          ];
    static kHuffmanBitLengthHuffmanCodeSymbols
        : [u8; 6]
        = [   0i32 as (u8),
              7i32 as (u8),
              3i32 as (u8),
              2i32 as (u8),
              1i32 as (u8),
              15i32 as (u8)
          ];
    static kHuffmanBitLengthHuffmanCodeBitLengths
        : [u8; 6]
        = [   2i32 as (u8),
              4i32 as (u8),
              3i32 as (u8),
              2i32 as (u8),
              2i32 as (u8),
              4i32 as (u8)
          ];
    let mut skip_some : u64 = 0u64;
    let mut codes_to_store : u64 = 18;
    if num_codes > 1i32 {
        'loop1: loop {
            if codes_to_store > 0 {
                if code_length_bitdepth[
                        kStorageOrder[
                            codes_to_store.wrapping_sub(1) as usize
                        ] as (usize)
                    ] as (i32) != 0i32 {
                    break 'loop1;
                } else {
                    codes_to_store = codes_to_store.wrapping_sub(1);
                    continue 'loop1;
                }
            } else {
                break 'loop1;
            }
        }
    }
    if code_length_bitdepth[
            kStorageOrder[0usize] as (usize)
        ] as (i32) == 0i32 && (code_length_bitdepth[
                                    kStorageOrder[1usize] as (usize)
                                ] as (i32) == 0i32) {
        skip_some = 2u64;
        if code_length_bitdepth[
                kStorageOrder[2usize] as (usize)
            ] as (i32) == 0i32 {
            skip_some = 3u64;
        }
    }
    BrotliWriteBits(2,skip_some,storage_ix,storage);
    let mut i : u64;
    i = skip_some;
    'loop8: loop {
        if i < codes_to_store {
            let l : usize
                = code_length_bitdepth[
                       kStorageOrder[i as usize] as (usize)
                   ] as (usize);
            BrotliWriteBits(
                kHuffmanBitLengthHuffmanCodeBitLengths[l] as u8,
                kHuffmanBitLengthHuffmanCodeSymbols[l] as u64,
                storage_ix,
                storage
            );
            i = i.wrapping_add(1);
            continue 'loop8;
        } else {
            break 'loop8;
        }
    }
}

fn BrotliStoreHuffmanTreeToBitMask(
    huffman_tree_size : usize,
    huffman_tree : &[u8],
    huffman_tree_extra_bits : &[u8],
    code_length_bitdepth : &[u8],
    code_length_bitdepth_symbols : &[u16],
    mut storage_ix : &mut usize,
    mut storage : &mut [u8]
) {
    let mut i : usize;
    i = 0usize;
    'loop1: loop {
        if i < huffman_tree_size {
            let ix : usize = huffman_tree[i as (usize)] as (usize);
            BrotliWriteBits(
                code_length_bitdepth[ix as (usize)],
                code_length_bitdepth_symbols[ix as (usize)] as (u64),
                storage_ix,
                storage
            );
            if ix == 17usize {
                BrotliWriteBits(
                    3,
                    huffman_tree_extra_bits[i as (usize)] as (u64),
                    storage_ix,
                    storage
                );
            } else if ix == 16usize {
                BrotliWriteBits(
                    2,
                    huffman_tree_extra_bits[i as (usize)] as (u64),
                    storage_ix,
                    storage
                );
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

pub fn BrotliStoreHuffmanTree(
    depths : &[u8],
    num : usize,
    mut tree : &mut [HuffmanTree],
    mut storage_ix : &mut usize,
    mut storage : &mut [u8]
) {
    let mut huffman_tree : [u8; 704] = [0; 704];
    let mut huffman_tree_extra_bits : [u8; 704] = [0; 704]; // ugh how to avoid
    let mut huffman_tree_size : usize = 0usize;
    let mut code_length_bitdepth
        : [u8; 18] = [0; 18];
    let mut code_length_bitdepth_symbols : [u16; 18] = [0; 18];
    let mut huffman_tree_histogram
        : [u32; 18] = [0;18];
    let mut i : usize;
    let mut num_codes : i32 = 0i32;
    let mut code : usize = 0usize;
    assert!(num <= 704usize);
    BrotliWriteHuffmanTree(
        depths,
        num,
        &mut huffman_tree_size,
        &mut huffman_tree[..],
        &mut huffman_tree_extra_bits[..]
    );
    i = 0usize;
    'loop1: loop {
        if i < huffman_tree_size {
            {
                let _rhs = 1;
                let _lhs = &mut huffman_tree_histogram[huffman_tree[i] as (usize)];
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    i = 0usize;
    'loop3: loop {
        if i < 18usize {
            if huffman_tree_histogram[i] != 0 {
                if num_codes == 0i32 {
                    code = i;
                    num_codes = 1i32;
                } else if num_codes == 1i32 {
                    num_codes = 2i32;
                }
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop3;
        } else {
            break 'loop3;
        }
    }
    BrotliCreateHuffmanTree(
        &mut huffman_tree_histogram,
        18usize,
        5i32,
        tree,
        &mut code_length_bitdepth
    );
    BrotliConvertBitDepthsToSymbols(
        &mut code_length_bitdepth,
        18usize,
        &mut code_length_bitdepth_symbols
    );
    BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(
        num_codes,
        &code_length_bitdepth,
        storage_ix,
        storage
    );
    if num_codes == 1i32 {
        code_length_bitdepth[code] = 0i32 as (u8);
    }
    BrotliStoreHuffmanTreeToBitMask(
        huffman_tree_size,
        &huffman_tree,
        &huffman_tree_extra_bits,
        &code_length_bitdepth,
        &code_length_bitdepth_symbols,
        storage_ix,
        storage
    );
}

fn StoreStaticCodeLengthCode(
    mut storage_ix : &mut usize, mut storage : &mut [u8]
) {
    BrotliWriteBits(
        40,
        0xffu32 as (u64) << 32i32 | 0x55555554u32 as (u64),
        storage_ix,
        storage
    );
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
) {
    let mut count : usize = 0usize;
    let mut symbols
        : [usize; 4]
        = [   0usize,
              0usize,
              0usize,
              0usize
          ];
    let mut length : usize = 0usize;
    let mut total : usize = histogram_total;
    'loop1: loop {
        if total != 0usize {
            if histogram[length as (usize)] != 0 {
                if count < 4usize {
                    symbols[count] = length;
                }
                count = count.wrapping_add(1 as (usize));
                total = total.wrapping_sub(
                            histogram[length as (usize)] as (usize)
                        );
            }
            length = length.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    if count <= 1usize {
        BrotliWriteBits(
            4,
            1,
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits as u8,
            symbols[0usize] as u64,
            storage_ix,
            storage
        );
        depth[symbols[0usize] as (usize)] = 0i32 as (u8);
        bits[symbols[0usize] as (usize)] = 0i32 as (u16);
    } else {
       for depth_elem in depth[..length].iter_mut() {
                        *depth_elem = 0; // memset
       }
        let max_tree_size
            : usize
            = (2usize).wrapping_mul(length).wrapping_add(
                  1usize
              );
        let mut tree
            = if max_tree_size != 0 {
                  m.alloc_cell(max_tree_size)
              } else {
                  AllocHT::AllocatedMemory::default() // null
              };
        let mut count_limit : u32;
        if !(0i32 == 0) {
        } else {
            count_limit = 1u32;
            'loop5: loop {
                let mut node_index : usize = 0;
                let mut l : usize;
                l = length;
                'loop6: loop {
                    if l != 0usize {
                        l = l.wrapping_sub(1 as (usize));
                        if histogram[l as (usize)] != 0 {
                            if histogram[l as (usize)] >= count_limit {
                                InitHuffmanTree(
                                    &mut tree.slice_mut()[node_index],
                                    histogram[l as (usize)],
                                    -1i32 as (i16),
                                    l as (i16)
                                );
                            } else {
                                InitHuffmanTree(&mut tree.slice_mut()[node_index],count_limit,-1i32 as (i16),l as (i16));
                            }
                            node_index += 1;
                            continue 'loop6;
                        } else {
                            continue 'loop6;
                        }
                    } else {
                        break 'loop6;
                    }
                }
                let n
                    : i32
                    = node_index as i32;

                let mut i : i32 = 0i32; // Points to the next leaf node
                let mut j : i32 = n + 1i32; // points to the next non-leaf node
                let mut k : i32;
                
                SortHuffmanTreeItems(&mut tree.slice_mut(),n as (usize),SortHuffmanTree{});
                // The nodes are:
                // [0, n): the sorted leaf nodes that we start with.
                // [n]: we add a sentinel here.
                // [n + 1, 2n): new parent nodes are added here, starting from
                //              (n+1). These are naturally in ascending order.
                // [2n]: we add a sentinel at the end as well.
                // There will be (2n+1) elements at the end.
                let mut sentinel : HuffmanTree = NewHuffmanTree(!(0u32), -1i16, -1i16);
                {
                    tree.slice_mut()[node_index + 1 as usize] = sentinel.clone();
                    tree.slice_mut()[node_index as usize] = sentinel.clone();
                    node_index += 2;
                }
                k = n - 1i32;
                'loop8: loop {
                    if k > 0i32 {
                        let left : i32;
                        let right : i32;
                        if (tree.slice()[i as (usize)]).total_count_ <= (tree.slice()[
                                                                              j as (usize)
                                                                          ]).total_count_ {
                            left = i;
                            i = i + 1;
                        } else {
                            left = j;
                            j = j + 1;
                        }
                        if (tree.slice()[i as (usize)]).total_count_ <= (tree.slice()[
                                                                              j as (usize)
                                                                          ]).total_count_ {
                            right = i;
                            i = i + 1;
                        } else {
                            right = j;
                            j = j + 1;
                        }
                        (tree.slice_mut()[node_index - 1]).total_count_ = (tree.slice()[
                                                                              left as (usize)
                                                                          ]).total_count_.wrapping_add(
                                                                            (tree.slice()[
                                                                                  right as (usize)
                                                                              ]).total_count_
                                                                        );
                        (tree.slice_mut()[node_index - 1]).index_left_ = left as (i16);
                        (tree.slice_mut()[node_index - 1]).index_right_or_value_ = right as (i16);
                        tree.slice_mut()[node_index] = sentinel.clone();
                        node_index += 1;
                        k = k - 1;
                        continue 'loop8;
                    } else {
                        break 'loop8;
                    }
                }
                if BrotliSetDepth(2i32 * n - 1i32,tree.slice_mut(),depth,14i32) {
                    break 'loop5;
                } else {
                    count_limit = count_limit.wrapping_mul(2u32);
                    continue 'loop5;
                }
            }
            m.free_cell(core::mem::replace(&mut tree, AllocHT::AllocatedMemory::default()));
            BrotliConvertBitDepthsToSymbols(depth,length,bits);
            if count <= 4usize {
                let mut i : usize;
                BrotliWriteBits(
                    2,
                    1,
                    storage_ix,
                    storage
                );
                BrotliWriteBits(
                    2,
                    count.wrapping_sub(1usize) as u64,
                    storage_ix,
                    storage
                );
                i = 0usize;
                'loop28: loop {
                    if i < count {
                        let mut j : usize;
                        j = i.wrapping_add(1usize);
                        'loop36: loop {
                            if j < count {
                                if depth[symbols[j] as (usize)] as (i32) < depth[
                                                                                        symbols[
                                                                                            i
                                                                                        ] as (usize)
                                                                                    ] as (i32) {
                                    let mut __brotli_swap_tmp : usize = symbols[j];
                                    symbols[j] = symbols[i];
                                    symbols[i] = __brotli_swap_tmp;
                                }
                                j = j.wrapping_add(1 as (usize));
                                continue 'loop36;
                            } else {
                                break 'loop36;
                            }
                        }
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop28;
                    } else {
                        break 'loop28;
                    }
                }
                if count == 2usize {
                    BrotliWriteBits(
                        max_bits as u8,
                        symbols[0usize] as u64,
                        storage_ix,
                        storage
                    );
                    BrotliWriteBits(
                        max_bits as u8,
                        symbols[1usize] as u64,
                        storage_ix,
                        storage
                    );
                } else if count == 3usize {
                    BrotliWriteBits(
                        max_bits as u8,
                        symbols[0usize] as u64,
                        storage_ix,
                        storage
                    );
                    BrotliWriteBits(
                        max_bits as u8,
                        symbols[1usize] as u64, 
                        storage_ix,
                        storage
                    );
                    BrotliWriteBits(
                        max_bits as u8,
                        symbols[2usize] as u64,
                        storage_ix,
                        storage
                    );
                } else {
                    BrotliWriteBits(
                        max_bits as u8,
                        symbols[0usize] as u64,
                        storage_ix,
                        storage
                    );
                    BrotliWriteBits(
                        max_bits as u8,
                        symbols[1usize] as u64,
                        storage_ix,
                        storage
                    );
                    BrotliWriteBits(
                        max_bits as u8,
                        symbols[2usize] as u64,
                        storage_ix,
                        storage
                    );
                    BrotliWriteBits(
                        max_bits as u8,
                        symbols[3usize] as u64,
                        storage_ix,
                        storage
                    );
                    BrotliWriteBits(
                        1,
                        if depth[
                                symbols[0usize] as (usize)
                            ] as (i32) == 1i32 {
                            1i32
                        } else {
                            0i32
                        } as (u64),
                        storage_ix,
                        storage
                    );
                }
            } else {
                let mut previous_value : u8 = 8i32 as (u8);
                let mut i : usize;
                StoreStaticCodeLengthCode(storage_ix,storage);
                i = 0usize;
                'loop13: loop {
                    if i < length {
                        let value : u8 = depth[i as (usize)];
                        let mut reps : usize = 1usize;
                        let mut k : usize;
                        k = i.wrapping_add(1usize);
                        'loop15: loop {
                            if k < length && (depth[
                                                   k as (usize)
                                               ] as (i32) == value as (i32)) {
                                reps = reps.wrapping_add(1 as (usize));
                                k = k.wrapping_add(1 as (usize));
                                continue 'loop15;
                            } else {
                                break 'loop15;
                            }
                        }
                        i = i.wrapping_add(reps);
                        if value as (i32) == 0i32 {
                            BrotliWriteBits(
                                kZeroRepsDepth[reps] as (u8),
                                kZeroRepsBits[reps] as u64,
                                storage_ix,
                                storage
                            );
                            continue 'loop13;
                        } else {
                            if previous_value as (i32) != value as (i32) {
                                BrotliWriteBits(
                                    kCodeLengthDepth[value as (usize)] as (u8),
                                    kCodeLengthBits[value as (usize)] as (u64),
                                    storage_ix,
                                    storage
                                );
                                reps = reps.wrapping_sub(1 as (usize));
                            }
                            if reps < 3usize {
                                'loop21: loop {
                                    if reps != 0usize {
                                        reps = reps.wrapping_sub(1 as (usize));
                                        BrotliWriteBits(
                                            kCodeLengthDepth[value as (usize)] as (u8),
                                            kCodeLengthBits[value as (usize)] as (u64),
                                            storage_ix,
                                            storage
                                        );
                                        continue 'loop21;
                                    } else {
                                        break 'loop21;
                                    }
                                }
                            } else {
                                reps = reps.wrapping_sub(3usize);
                                BrotliWriteBits(
                                    kNonZeroRepsDepth[reps] as (u8),
                                    kNonZeroRepsBits[reps] as u64,
                                    storage_ix,
                                    storage
                                );
                            }
                            previous_value = value;
                            continue 'loop13;
                        }
                    } else {
                        break 'loop13;
                    }
                }
            }
        }
    }
}
