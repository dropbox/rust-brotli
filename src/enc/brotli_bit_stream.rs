use super::constants::BROTLI_NUM_BLOCK_LEN_SYMBOLS;
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

pub struct HuffmanTree {
    pub total_count_ : u32,
    pub index_left_ : i16,
    pub index_right_or_value_ : i16,
}

fn BrotliWriteBits(
    mut n_bits : u8,
    mut bits : u64,
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
    mut code_length_bitdepth : &[u8],
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
            let mut l
                : usize
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
    mut huffman_tree : &[u8],
    mut huffman_tree_extra_bits : &[u8],
    mut code_length_bitdepth : &[u8],
    mut code_length_bitdepth_symbols : &[u16],
    mut storage_ix : &mut usize,
    mut storage : &mut [u8]
) {
    let mut i : usize;
    i = 0usize;
    'loop1: loop {
        if i < huffman_tree_size {
            let mut ix : usize = huffman_tree[i as (usize)] as (usize);
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
