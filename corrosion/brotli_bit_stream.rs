extern {
    fn BrotliAllocate(
        m : *mut MemoryManager, n : usize
    ) -> *mut std::os::raw::c_void;
    fn BrotliConvertBitDepthsToSymbols(
        depth : *const u8, len : usize, bits : *mut u16
    );
    fn BrotliCreateHuffmanTree(
        data : *const u32,
        length : usize,
        tree_limit : i32,
        tree : *mut HuffmanTree,
        depth : *mut u8
    );
    fn BrotliFree(
        m : *mut MemoryManager, p : *mut std::os::raw::c_void
    );
    fn BrotliSetDepth(
        p : i32, pool : *mut HuffmanTree, depth : *mut u8, max_depth : i32
    ) -> i32;
    fn BrotliWriteHuffmanTree(
        depth : *const u8,
        num : usize,
        tree_size : *mut usize,
        tree : *mut u8,
        extra_bits_data : *mut u8
    );
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn memcpy(
        __dest : *mut std::os::raw::c_void,
        __src : *const std::os::raw::c_void,
        __n : usize
    ) -> *mut std::os::raw::c_void;
    fn memset(
        __s : *mut std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut std::os::raw::c_void;
}

static mut kLog2Table
    : *const f32
    = 0.0000000000000000f32 as (*const f32);

static mut kInsBase : *mut u32 = 0i32 as (*mut u32);

static mut kInsExtra : *mut u32 = 0i32 as (*mut u32);

static mut kCopyBase : *mut u32 = 2i32 as (*mut u32);

static mut kCopyExtra : *mut u32 = 0i32 as (*mut u32);

static mut kUTF8ContextLookup : *const u8 = 0i32 as (*const u8);

static mut kSigned3BitContextLookup
    : *const u8
    = 0i32 as (*const u8);

static kBrotliMinWindowBits : i32 = 10i32;

static kBrotliMaxWindowBits : i32 = 24i32;

static mut kCodeLengthDepth : *const u8 = 4i32 as (*const u8);

static mut kStaticCommandCodeDepth
    : *const u8
    = 9i32 as (*const u8);

static mut kStaticDistanceCodeDepth
    : *const u8
    = 6i32 as (*const u8);

static mut kCodeLengthBits : *const u32 = 0i32 as (*const u32);

static mut kZeroRepsBits : *const usize = 0x0i32 as (*const usize);

static mut kZeroRepsDepth : *const u32 = 0i32 as (*const u32);

static mut kNonZeroRepsBits
    : *const usize
    = 0xbi32 as (*const usize);

static mut kNonZeroRepsDepth : *const u32 = 6i32 as (*const u32);

static mut kStaticCommandCodeBits
    : *const u16
    = 0i32 as (*const u16);

static mut kStaticDistanceCodeBits
    : *const u16
    = 0i32 as (*const u16);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PrefixCodeRange {
    pub offset : u32,
    pub nbits : u32,
}

static mut kBlockLengthPrefixCode
    : *const PrefixCodeRange
    = 1i32 as (*const PrefixCodeRange);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HuffmanTree {
    pub total_count_ : u32,
    pub index_left_ : i16,
    pub index_right_or_value_ : i16,
}

unsafe extern fn BROTLI_UNALIGNED_STORE64(
    mut p : *mut std::os::raw::c_void, mut v : usize
) {
    memcpy(
        p,
        &mut v as (*mut usize) as (*const std::os::raw::c_void),
        std::mem::size_of::<usize>()
    );
}

unsafe extern fn BrotliWriteBits(
    mut n_bits : usize,
    mut bits : usize,
    mut pos : *mut usize,
    mut array : *mut u8
) {
    let mut p
        : *mut u8
        = &mut *array.offset((*pos >> 3i32) as (isize)) as (*mut u8);
    let mut v : usize = *p as (usize);
    if bits >> n_bits == 0i32 as (usize) {
        0i32;
    } else {
        __assert_fail(
            b"(bits >> n_bits) == 0\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"BrotliWriteBits\0".as_ptr()
        );
    }
    if n_bits <= 56i32 as (usize) {
        0i32;
    } else {
        __assert_fail(
            b"n_bits <= 56\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"BrotliWriteBits\0".as_ptr()
        );
    }
    v = v | bits << (*pos & 7i32 as (usize));
    BROTLI_UNALIGNED_STORE64(p as (*mut std::os::raw::c_void),v);
    *pos = (*pos).wrapping_add(n_bits);
}

unsafe extern fn BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(
    num_codes : i32,
    mut code_length_bitdepth : *const u8,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    static mut kStorageOrder : *const u8 = 1i32 as (*const u8);
    static mut kHuffmanBitLengthHuffmanCodeSymbols
        : *const u8
        = 0i32 as (*const u8);
    static mut kHuffmanBitLengthHuffmanCodeBitLengths
        : *const u8
        = 2i32 as (*const u8);
    let mut skip_some : usize = 0i32 as (usize);
    let mut codes_to_store : usize = 18i32 as (usize);
    if num_codes > 1i32 {
        'break5: while codes_to_store > 0i32 as (usize) {
            {
                if *code_length_bitdepth.offset(
                        *kStorageOrder.offset(
                             codes_to_store.wrapping_sub(1i32 as (usize)) as (isize)
                         ) as (isize)
                    ) as (i32) != 0i32 {
                    if 1337i32 != 0 {
                        break 'break5;
                    }
                }
            }
            codes_to_store = codes_to_store.wrapping_sub(1 as (usize));
        }
    }
    if *code_length_bitdepth.offset(
            *kStorageOrder.offset(0i32 as (isize)) as (isize)
        ) as (i32) == 0i32 && (*code_length_bitdepth.offset(
                                    *kStorageOrder.offset(1i32 as (isize)) as (isize)
                                ) as (i32) == 0i32) {
        skip_some = 2i32 as (usize);
        if *code_length_bitdepth.offset(
                *kStorageOrder.offset(2i32 as (isize)) as (isize)
            ) as (i32) == 0i32 {
            skip_some = 3i32 as (usize);
        }
    }
    BrotliWriteBits(2i32 as (usize),skip_some,storage_ix,storage);
    {
        let mut i : usize;
        i = skip_some;
        while i < codes_to_store {
            {
                let mut l
                    : usize
                    = *code_length_bitdepth.offset(
                           *kStorageOrder.offset(i as (isize)) as (isize)
                       ) as (usize);
                BrotliWriteBits(
                    *kHuffmanBitLengthHuffmanCodeBitLengths.offset(
                         l as (isize)
                     ) as (usize),
                    *kHuffmanBitLengthHuffmanCodeSymbols.offset(
                         l as (isize)
                     ) as (usize),
                    storage_ix,
                    storage
                );
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
}

unsafe extern fn BrotliStoreHuffmanTreeToBitMask(
    huffman_tree_size : usize,
    mut huffman_tree : *const u8,
    mut huffman_tree_extra_bits : *const u8,
    mut code_length_bitdepth : *const u8,
    mut code_length_bitdepth_symbols : *const u16,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut i : usize;
    i = 0i32 as (usize);
    while i < huffman_tree_size {
        {
            let mut ix : usize = *huffman_tree.offset(i as (isize)) as (usize);
            BrotliWriteBits(
                *code_length_bitdepth.offset(ix as (isize)) as (usize),
                *code_length_bitdepth_symbols.offset(ix as (isize)) as (usize),
                storage_ix,
                storage
            );
            if ix == 16i32 as (usize) {
                BrotliWriteBits(
                    2i32 as (usize),
                    *huffman_tree_extra_bits.offset(i as (isize)) as (usize),
                    storage_ix,
                    storage
                );
            } else if ix == 17i32 as (usize) {
                BrotliWriteBits(
                    3i32 as (usize),
                    *huffman_tree_extra_bits.offset(i as (isize)) as (usize),
                    storage_ix,
                    storage
                );
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
}

#[no_mangle]
pub unsafe extern fn BrotliStoreHuffmanTree(
    mut depths : *const u8,
    mut num : usize,
    mut tree : *mut HuffmanTree,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut huffman_tree : *mut u8;
    let mut huffman_tree_extra_bits : *mut u8;
    let mut huffman_tree_size : usize = 0i32 as (usize);
    let mut code_length_bitdepth : *mut u8 = 0i32 as (*mut u8);
    let mut code_length_bitdepth_symbols : *mut u16;
    let mut huffman_tree_histogram : *mut u32 = 0i32 as (*mut u32);
    let mut i : usize;
    let mut num_codes : i32 = 0i32;
    let mut code : usize = 0i32 as (usize);
    if num <= 704i32 as (usize) {
        0i32;
    } else {
        __assert_fail(
            b"num <= BROTLI_NUM_COMMAND_SYMBOLS\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"BrotliStoreHuffmanTree\0".as_ptr()
        );
    }
    BrotliWriteHuffmanTree(
        depths,
        num,
        &mut huffman_tree_size as (*mut usize),
        huffman_tree,
        huffman_tree_extra_bits
    );
    i = 0i32 as (usize);
    while i < huffman_tree_size {
        {
            let _rhs = 1;
            let _lhs
                = &mut *huffman_tree_histogram.offset(
                            *huffman_tree.offset(i as (isize)) as (isize)
                        );
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        i = i.wrapping_add(1 as (usize));
    }
    i = 0i32 as (usize);
    'break3: while i < 18i32 as (usize) {
        {
            if *huffman_tree_histogram.offset(i as (isize)) != 0 {
                if num_codes == 0i32 {
                    code = i;
                    num_codes = 1i32;
                } else if num_codes == 1i32 {
                    num_codes = 2i32;
                    {
                        if 1337i32 != 0 {
                            break 'break3;
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    BrotliCreateHuffmanTree(
        huffman_tree_histogram as (*const u32),
        18i32 as (usize),
        5i32,
        tree,
        code_length_bitdepth
    );
    BrotliConvertBitDepthsToSymbols(
        code_length_bitdepth as (*const u8),
        18i32 as (usize),
        code_length_bitdepth_symbols
    );
    BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(
        num_codes,
        code_length_bitdepth as (*const u8),
        storage_ix,
        storage
    );
    if num_codes == 1i32 {
        *code_length_bitdepth.offset(code as (isize)) = 0i32 as (u8);
    }
    BrotliStoreHuffmanTreeToBitMask(
        huffman_tree_size,
        huffman_tree as (*const u8),
        huffman_tree_extra_bits as (*const u8),
        code_length_bitdepth as (*const u8),
        code_length_bitdepth_symbols as (*const u16),
        storage_ix,
        storage
    );
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func : unsafe extern fn(*mut std::os::raw::c_void, usize) -> *mut std::os::raw::c_void,
    pub free_func : unsafe extern fn(*mut std::os::raw::c_void, *mut std::os::raw::c_void),
    pub opaque : *mut std::os::raw::c_void,
}

unsafe extern fn InitHuffmanTree(
    mut self : *mut HuffmanTree,
    mut count : u32,
    mut left : i16,
    mut right : i16
) {
    (*self).total_count_ = count;
    (*self).index_left_ = left;
    (*self).index_right_or_value_ = right;
}

unsafe extern fn SortHuffmanTreeItems(
    mut items : *mut HuffmanTree,
    n : usize,
    mut
    comparator
    :
    unsafe extern fn(*const HuffmanTree, *const HuffmanTree) -> i32
) {
    static mut gaps : *const usize = 132i32 as (*const usize);
    if n < 13i32 as (usize) {
        let mut i : usize;
        i = 1i32 as (usize);
        while i < n {
            {
                let mut tmp : HuffmanTree = *items.offset(i as (isize));
                let mut k : usize = i;
                let mut j : usize = i.wrapping_sub(1i32 as (usize));
                while comparator(
                          &mut tmp as (*mut HuffmanTree) as (*const HuffmanTree),
                          &mut *items.offset(
                                    j as (isize)
                                ) as (*mut HuffmanTree) as (*const HuffmanTree)
                      ) != 0 {
                    *items.offset(k as (isize)) = *items.offset(j as (isize));
                    k = j;
                    if {
                           let _old = j;
                           j = j.wrapping_sub(1 as (usize));
                           _old
                       } == 0 {
                        if 1337i32 != 0 {
                            break;
                        }
                    }
                }
                *items.offset(k as (isize)) = tmp;
            }
            i = i.wrapping_add(1 as (usize));
        }
    } else {
        let mut g : i32 = if n < 57i32 as (usize) { 2i32 } else { 0i32 };
        while g < 6i32 {
            {
                let mut gap : usize = *gaps.offset(g as (isize));
                let mut i : usize;
                i = gap;
                while i < n {
                    {
                        let mut j : usize = i;
                        let mut tmp : HuffmanTree = *items.offset(i as (isize));
                        while j >= gap && (comparator(
                                               &mut tmp as (*mut HuffmanTree) as (*const HuffmanTree),
                                               &mut *items.offset(
                                                         j.wrapping_sub(gap) as (isize)
                                                     ) as (*mut HuffmanTree) as (*const HuffmanTree)
                                           ) != 0) {
                            {
                                *items.offset(j as (isize)) = *items.offset(
                                                                   j.wrapping_sub(gap) as (isize)
                                                               );
                            }
                            j = j.wrapping_sub(gap);
                        }
                        *items.offset(j as (isize)) = tmp;
                    }
                    i = i.wrapping_add(1 as (usize));
                }
            }
            g = g + 1;
        }
    }
}

unsafe extern fn SortHuffmanTree(
    mut v0 : *const HuffmanTree, mut v1 : *const HuffmanTree
) -> i32 {
    if !!((*v0).total_count_ < (*v1).total_count_) {
        1i32
    } else {
        0i32
    }
}

unsafe extern fn StoreStaticCodeLengthCode(
    mut storage_ix : *mut usize, mut storage : *mut u8
) {
    BrotliWriteBits(
        40i32 as (usize),
        0xffu32 as (usize) << 32i32 | 0x55555554u32 as (usize),
        storage_ix,
        storage
    );
}

#[no_mangle]
pub unsafe extern fn BrotliBuildAndStoreHuffmanTreeFast(
    mut m : *mut MemoryManager,
    mut histogram : *const u32,
    histogram_total : usize,
    max_bits : usize,
    mut depth : *mut u8,
    mut bits : *mut u16,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut count : usize = 0i32 as (usize);
    let mut symbols : *mut usize = 0i32 as (*mut usize);
    let mut length : usize = 0i32 as (usize);
    let mut total : usize = histogram_total;
    while total != 0i32 as (usize) {
        if *histogram.offset(length as (isize)) != 0 {
            if count < 4i32 as (usize) {
                *symbols.offset(count as (isize)) = length;
            }
            count = count.wrapping_add(1 as (usize));
            total = total.wrapping_sub(
                        *histogram.offset(length as (isize)) as (usize)
                    );
        }
        length = length.wrapping_add(1 as (usize));
    }
    if count <= 1i32 as (usize) {
        BrotliWriteBits(
            4i32 as (usize),
            1i32 as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0i32 as (isize)),
            storage_ix,
            storage
        );
        *depth.offset(
             *symbols.offset(0i32 as (isize)) as (isize)
         ) = 0i32 as (u8);
        *bits.offset(
             *symbols.offset(0i32 as (isize)) as (isize)
         ) = 0i32 as (u16);
        return;
    }
    memset(
        depth as (*mut std::os::raw::c_void),
        0i32,
        length.wrapping_mul(std::mem::size_of::<u8>())
    );
    {
        let max_tree_size
            : usize
            = (2i32 as (usize)).wrapping_mul(length).wrapping_add(
                  1i32 as (usize)
              );
        let mut tree
            : *mut HuffmanTree
            = if max_tree_size != 0 {
                  BrotliAllocate(
                      m,
                      max_tree_size.wrapping_mul(std::mem::size_of::<HuffmanTree>())
                  ) as (*mut HuffmanTree)
              } else {
                  0i32 as (*mut std::os::raw::c_void) as (*mut HuffmanTree)
              };
        let mut count_limit : u32;
        if !(0i32 == 0) {
            return;
        }
        count_limit = 1i32 as (u32);
        'break11: loop {
            {
                let mut node : *mut HuffmanTree = tree;
                let mut l : usize;
                l = length;
                while l != 0i32 as (usize) {
                    l = l.wrapping_sub(1 as (usize));
                    if *histogram.offset(l as (isize)) != 0 {
                        if *histogram.offset(l as (isize)) >= count_limit {
                            InitHuffmanTree(
                                node,
                                *histogram.offset(l as (isize)),
                                -1i32 as (i16),
                                l as (i16)
                            );
                        } else {
                            InitHuffmanTree(node,count_limit,-1i32 as (i16),l as (i16));
                        }
                        node = node.offset(1 as (isize));
                    }
                }
                {
                    let n
                        : i32
                        = ((node as (isize)).wrapping_sub(
                               tree as (isize)
                           ) / std::mem::size_of::<*mut HuffmanTree>() as (isize)) as (i32);
                    let mut sentinel : HuffmanTree;
                    let mut i : i32 = 0i32;
                    let mut j : i32 = n + 1i32;
                    let mut k : i32;
                    SortHuffmanTreeItems(tree,n as (usize),SortHuffmanTree);
                    InitHuffmanTree(
                        &mut sentinel as (*mut HuffmanTree),
                        !(0i32 as (u32)),
                        -1i32 as (i16),
                        -1i32 as (i16)
                    );
                    *{
                         let _old = node;
                         node = node.offset(1 as (isize));
                         _old
                     } = sentinel;
                    *{
                         let _old = node;
                         node = node.offset(1 as (isize));
                         _old
                     } = sentinel;
                    k = n - 1i32;
                    while k > 0i32 {
                        {
                            let mut left : i32;
                            let mut right : i32;
                            if (*tree.offset(i as (isize))).total_count_ <= (*tree.offset(
                                                                                  j as (isize)
                                                                              )).total_count_ {
                                left = i;
                                i = i + 1;
                            } else {
                                left = j;
                                j = j + 1;
                            }
                            if (*tree.offset(i as (isize))).total_count_ <= (*tree.offset(
                                                                                  j as (isize)
                                                                              )).total_count_ {
                                right = i;
                                i = i + 1;
                            } else {
                                right = j;
                                j = j + 1;
                            }
                            (*node.offset(-1i32 as (isize))).total_count_ = (*tree.offset(
                                                                                  left as (isize)
                                                                              )).total_count_.wrapping_add(
                                                                                (*tree.offset(
                                                                                      right as (isize)
                                                                                  )).total_count_
                                                                            );
                            (*node.offset(-1i32 as (isize))).index_left_ = left as (i16);
                            (*node.offset(
                                  -1i32 as (isize)
                              )).index_right_or_value_ = right as (i16);
                            *{
                                 let _old = node;
                                 node = node.offset(1 as (isize));
                                 _old
                             } = sentinel;
                        }
                        k = k - 1;
                    }
                    if BrotliSetDepth(2i32 * n - 1i32,tree,depth,14i32) != 0 {
                        if 1337i32 != 0 {
                            break 'break11;
                        }
                    }
                }
            }
            count_limit = count_limit.wrapping_mul(2i32 as (u32));
        }
        {
            BrotliFree(m,tree as (*mut std::os::raw::c_void));
            tree = 0i32 as (*mut std::os::raw::c_void) as (*mut HuffmanTree);
        }
    }
    BrotliConvertBitDepthsToSymbols(depth as (*const u8),length,bits);
    if count <= 4i32 as (usize) {
        let mut i : usize;
        BrotliWriteBits(
            2i32 as (usize),
            1i32 as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            2i32 as (usize),
            count.wrapping_sub(1i32 as (usize)),
            storage_ix,
            storage
        );
        i = 0i32 as (usize);
        while i < count {
            {
                let mut j : usize;
                j = i.wrapping_add(1i32 as (usize));
                while j < count {
                    {
                        if *depth.offset(
                                *symbols.offset(j as (isize)) as (isize)
                            ) as (i32) < *depth.offset(
                                              *symbols.offset(i as (isize)) as (isize)
                                          ) as (i32) {
                            let mut __brotli_swap_tmp : usize = *symbols.offset(j as (isize));
                            *symbols.offset(j as (isize)) = *symbols.offset(i as (isize));
                            *symbols.offset(i as (isize)) = __brotli_swap_tmp;
                        }
                    }
                    j = j.wrapping_add(1 as (usize));
                }
            }
            i = i.wrapping_add(1 as (usize));
        }
        if count == 2i32 as (usize) {
            BrotliWriteBits(
                max_bits,
                *symbols.offset(0i32 as (isize)),
                storage_ix,
                storage
            );
            BrotliWriteBits(
                max_bits,
                *symbols.offset(1i32 as (isize)),
                storage_ix,
                storage
            );
        } else if count == 3i32 as (usize) {
            BrotliWriteBits(
                max_bits,
                *symbols.offset(0i32 as (isize)),
                storage_ix,
                storage
            );
            BrotliWriteBits(
                max_bits,
                *symbols.offset(1i32 as (isize)),
                storage_ix,
                storage
            );
            BrotliWriteBits(
                max_bits,
                *symbols.offset(2i32 as (isize)),
                storage_ix,
                storage
            );
        } else {
            BrotliWriteBits(
                max_bits,
                *symbols.offset(0i32 as (isize)),
                storage_ix,
                storage
            );
            BrotliWriteBits(
                max_bits,
                *symbols.offset(1i32 as (isize)),
                storage_ix,
                storage
            );
            BrotliWriteBits(
                max_bits,
                *symbols.offset(2i32 as (isize)),
                storage_ix,
                storage
            );
            BrotliWriteBits(
                max_bits,
                *symbols.offset(3i32 as (isize)),
                storage_ix,
                storage
            );
            BrotliWriteBits(
                1i32 as (usize),
                if *depth.offset(
                        *symbols.offset(0i32 as (isize)) as (isize)
                    ) as (i32) == 1i32 {
                    1i32
                } else {
                    0i32
                } as (usize),
                storage_ix,
                storage
            );
        }
    } else {
        let mut previous_value : u8 = 8i32 as (u8);
        let mut i : usize;
        StoreStaticCodeLengthCode(storage_ix,storage);
        i = 0i32 as (usize);
        while i < length {
            let value : u8 = *depth.offset(i as (isize));
            let mut reps : usize = 1i32 as (usize);
            let mut k : usize;
            k = i.wrapping_add(1i32 as (usize));
            while k < length && (*depth.offset(
                                      k as (isize)
                                  ) as (i32) == value as (i32)) {
                {
                    reps = reps.wrapping_add(1 as (usize));
                }
                k = k.wrapping_add(1 as (usize));
            }
            i = i.wrapping_add(reps);
            if value as (i32) == 0i32 {
                BrotliWriteBits(
                    *kZeroRepsDepth.offset(reps as (isize)) as (usize),
                    *kZeroRepsBits.offset(reps as (isize)),
                    storage_ix,
                    storage
                );
            } else {
                if previous_value as (i32) != value as (i32) {
                    BrotliWriteBits(
                        *kCodeLengthDepth.offset(value as (isize)) as (usize),
                        *kCodeLengthBits.offset(value as (isize)) as (usize),
                        storage_ix,
                        storage
                    );
                    reps = reps.wrapping_sub(1 as (usize));
                }
                if reps < 3i32 as (usize) {
                    while reps != 0i32 as (usize) {
                        reps = reps.wrapping_sub(1 as (usize));
                        BrotliWriteBits(
                            *kCodeLengthDepth.offset(value as (isize)) as (usize),
                            *kCodeLengthBits.offset(value as (isize)) as (usize),
                            storage_ix,
                            storage
                        );
                    }
                } else {
                    reps = reps.wrapping_sub(3i32 as (usize));
                    BrotliWriteBits(
                        *kNonZeroRepsDepth.offset(reps as (isize)) as (usize),
                        *kNonZeroRepsBits.offset(reps as (isize)),
                        storage_ix,
                        storage
                    );
                }
                previous_value = value;
            }
        }
    }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum ContextType {
    CONTEXT_LSB6 = 0i32,
    CONTEXT_MSB6 = 1i32,
    CONTEXT_UTF8 = 2i32,
    CONTEXT_SIGNED = 3i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Command {
    pub insert_len_ : u32,
    pub copy_len_ : u32,
    pub dist_extra_ : u32,
    pub cmd_prefix_ : u16,
    pub dist_prefix_ : u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlockSplit {
    pub num_types : usize,
    pub num_blocks : usize,
    pub types : *mut u8,
    pub lengths : *mut u32,
    pub types_alloc_size : usize,
    pub lengths_alloc_size : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramLiteral {
    pub data_ : *mut u32,
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramCommand {
    pub data_ : *mut u32,
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramDistance {
    pub data_ : *mut u32,
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MetaBlockSplit {
    pub literal_split : BlockSplit,
    pub command_split : BlockSplit,
    pub distance_split : BlockSplit,
    pub literal_context_map : *mut u32,
    pub literal_context_map_size : usize,
    pub distance_context_map : *mut u32,
    pub distance_context_map_size : usize,
    pub literal_histograms : *mut HistogramLiteral,
    pub literal_histograms_size : usize,
    pub command_histograms : *mut HistogramCommand,
    pub command_histograms_size : usize,
    pub distance_histograms : *mut HistogramDistance,
    pub distance_histograms_size : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlockTypeCodeCalculator {
    pub last_type : usize,
    pub second_last_type : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlockSplitCode {
    pub type_code_calculator : BlockTypeCodeCalculator,
    pub type_depths : *mut u8,
    pub type_bits : *mut u16,
    pub length_depths : *mut u8,
    pub length_bits : *mut u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlockEncoder {
    pub alphabet_size_ : usize,
    pub num_block_types_ : usize,
    pub block_types_ : *const u8,
    pub block_lengths_ : *const u32,
    pub num_blocks_ : usize,
    pub block_split_code_ : BlockSplitCode,
    pub block_ix_ : usize,
    pub block_len_ : usize,
    pub entropy_ix_ : usize,
    pub depths_ : *mut u8,
    pub bits_ : *mut u16,
}

unsafe extern fn Log2FloorNonZero(mut n : usize) -> u32 {
    let mut result : u32 = 0i32 as (u32);
    while {
              n = n >> 1i32;
              n
          } != 0 {
        result = result.wrapping_add(1 as (u32));
    }
    result
}

unsafe extern fn BrotliEncodeMlen(
    mut length : usize,
    mut bits : *mut usize,
    mut numbits : *mut usize,
    mut nibblesbits : *mut usize
) {
    let mut lg
        : usize
        = (if length == 1i32 as (usize) {
               1i32 as (u32)
           } else {
               Log2FloorNonZero(
                   length.wrapping_sub(1i32 as (usize)) as (u32) as (usize)
               ).wrapping_add(
                   1i32 as (u32)
               )
           }) as (usize);
    let mut mnibbles
        : usize
        = (if lg < 16i32 as (usize) {
               16i32 as (usize)
           } else {
               lg.wrapping_add(3i32 as (usize))
           }).wrapping_div(
              4i32 as (usize)
          );
    if length > 0i32 as (usize) {
        0i32;
    } else {
        __assert_fail(
            b"length > 0\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"BrotliEncodeMlen\0".as_ptr()
        );
    }
    if length <= (1i32 << 24i32) as (usize) {
        0i32;
    } else {
        __assert_fail(
            b"length <= (1 << 24)\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"BrotliEncodeMlen\0".as_ptr()
        );
    }
    if lg <= 24i32 as (usize) {
        0i32;
    } else {
        __assert_fail(
            b"lg <= 24\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"BrotliEncodeMlen\0".as_ptr()
        );
    }
    *nibblesbits = mnibbles.wrapping_sub(4i32 as (usize));
    *numbits = mnibbles.wrapping_mul(4i32 as (usize));
    *bits = length.wrapping_sub(1i32 as (usize));
}

unsafe extern fn StoreCompressedMetaBlockHeader(
    mut is_final_block : i32,
    mut length : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut lenbits : usize;
    let mut nlenbits : usize;
    let mut nibblesbits : usize;
    BrotliWriteBits(
        1i32 as (usize),
        is_final_block as (usize),
        storage_ix,
        storage
    );
    if is_final_block != 0 {
        BrotliWriteBits(
            1i32 as (usize),
            0i32 as (usize),
            storage_ix,
            storage
        );
    }
    BrotliEncodeMlen(
        length,
        &mut lenbits as (*mut usize),
        &mut nlenbits as (*mut usize),
        &mut nibblesbits as (*mut usize)
    );
    BrotliWriteBits(2i32 as (usize),nibblesbits,storage_ix,storage);
    BrotliWriteBits(nlenbits,lenbits,storage_ix,storage);
    if is_final_block == 0 {
        BrotliWriteBits(
            1i32 as (usize),
            0i32 as (usize),
            storage_ix,
            storage
        );
    }
}

unsafe extern fn InitBlockTypeCodeCalculator(
    mut self : *mut BlockTypeCodeCalculator
) {
    (*self).last_type = 1i32 as (usize);
    (*self).second_last_type = 0i32 as (usize);
}

unsafe extern fn InitBlockEncoder(
    mut self : *mut BlockEncoder,
    mut alphabet_size : usize,
    mut num_block_types : usize,
    mut block_types : *const u8,
    mut block_lengths : *const u32,
    num_blocks : usize
) {
    (*self).alphabet_size_ = alphabet_size;
    (*self).num_block_types_ = num_block_types;
    (*self).block_types_ = block_types;
    (*self).block_lengths_ = block_lengths;
    (*self).num_blocks_ = num_blocks;
    InitBlockTypeCodeCalculator(
        &mut (*self).block_split_code_.type_code_calculator as (*mut BlockTypeCodeCalculator)
    );
    (*self).block_ix_ = 0i32 as (usize);
    (*self).block_len_ = if num_blocks == 0i32 as (usize) {
                             0i32 as (u32)
                         } else {
                             *block_lengths.offset(0i32 as (isize))
                         } as (usize);
    (*self).entropy_ix_ = 0i32 as (usize);
    (*self).depths_ = 0i32 as (*mut u8);
    (*self).bits_ = 0i32 as (*mut u16);
}

unsafe extern fn NextBlockTypeCode(
    mut calculator : *mut BlockTypeCodeCalculator, mut type_ : u8
) -> usize {
    let mut type_code
        : usize
        = (if type_ as (usize) == (*calculator).last_type.wrapping_add(
                                      1i32 as (usize)
                                  ) {
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

unsafe extern fn BlockLengthPrefixCode(mut len : u32) -> u32 {
    let mut code
        : u32
        = (if len >= 177i32 as (u32) {
               if len >= 753i32 as (u32) { 20i32 } else { 14i32 }
           } else if len >= 41i32 as (u32) {
               7i32
           } else {
               0i32
           }) as (u32);
    while code < (26i32 - 1i32) as (u32) && (len >= (*kBlockLengthPrefixCode.offset(
                                                          code.wrapping_add(
                                                              1i32 as (u32)
                                                          ) as (isize)
                                                      )).offset) {
        code = code.wrapping_add(1 as (u32));
    }
    code
}

unsafe extern fn StoreVarLenUint8(
    mut n : usize, mut storage_ix : *mut usize, mut storage : *mut u8
) { if n == 0i32 as (usize) {
        BrotliWriteBits(
            1i32 as (usize),
            0i32 as (usize),
            storage_ix,
            storage
        );
    } else {
        let mut nbits : usize = Log2FloorNonZero(n) as (usize);
        BrotliWriteBits(
            1i32 as (usize),
            1i32 as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(3i32 as (usize),nbits,storage_ix,storage);
        BrotliWriteBits(
            nbits,
            n.wrapping_sub(1i32 as (usize) << nbits),
            storage_ix,
            storage
        );
    }
}

unsafe extern fn StoreSimpleHuffmanTree(
    mut depths : *const u8,
    mut symbols : *mut usize,
    mut num_symbols : usize,
    mut max_bits : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliWriteBits(
        2i32 as (usize),
        1i32 as (usize),
        storage_ix,
        storage
    );
    BrotliWriteBits(
        2i32 as (usize),
        num_symbols.wrapping_sub(1i32 as (usize)),
        storage_ix,
        storage
    );
    {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < num_symbols {
            {
                let mut j : usize;
                j = i.wrapping_add(1i32 as (usize));
                while j < num_symbols {
                    {
                        if *depths.offset(
                                *symbols.offset(j as (isize)) as (isize)
                            ) as (i32) < *depths.offset(
                                              *symbols.offset(i as (isize)) as (isize)
                                          ) as (i32) {
                            let mut __brotli_swap_tmp : usize = *symbols.offset(j as (isize));
                            *symbols.offset(j as (isize)) = *symbols.offset(i as (isize));
                            *symbols.offset(i as (isize)) = __brotli_swap_tmp;
                        }
                    }
                    j = j.wrapping_add(1 as (usize));
                }
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    if num_symbols == 2i32 as (usize) {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0i32 as (isize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1i32 as (isize)),
            storage_ix,
            storage
        );
    } else if num_symbols == 3i32 as (usize) {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0i32 as (isize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1i32 as (isize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(2i32 as (isize)),
            storage_ix,
            storage
        );
    } else {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0i32 as (isize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1i32 as (isize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(2i32 as (isize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(3i32 as (isize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            1i32 as (usize),
            if *depths.offset(
                    *symbols.offset(0i32 as (isize)) as (isize)
                ) as (i32) == 1i32 {
                1i32
            } else {
                0i32
            } as (usize),
            storage_ix,
            storage
        );
    }
}

unsafe extern fn BuildAndStoreHuffmanTree(
    mut histogram : *const u32,
    length : usize,
    mut tree : *mut HuffmanTree,
    mut depth : *mut u8,
    mut bits : *mut u16,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut count : usize = 0i32 as (usize);
    let mut s4 : *mut usize = 0i32 as (*mut usize);
    let mut i : usize;
    let mut max_bits : usize = 0i32 as (usize);
    i = 0i32 as (usize);
    'break31: while i < length {
        {
            if *histogram.offset(i as (isize)) != 0 {
                if count < 4i32 as (usize) {
                    *s4.offset(count as (isize)) = i;
                } else if count > 4i32 as (usize) {
                    if 1337i32 != 0 {
                        break 'break31;
                    }
                }
                count = count.wrapping_add(1 as (usize));
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    {
        let mut max_bits_counter
            : usize
            = length.wrapping_sub(1i32 as (usize));
        while max_bits_counter != 0 {
            max_bits_counter = max_bits_counter >> 1i32;
            max_bits = max_bits.wrapping_add(1 as (usize));
        }
    }
    if count <= 1i32 as (usize) {
        BrotliWriteBits(
            4i32 as (usize),
            1i32 as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            *s4.offset(0i32 as (isize)),
            storage_ix,
            storage
        );
        *depth.offset(
             *s4.offset(0i32 as (isize)) as (isize)
         ) = 0i32 as (u8);
        *bits.offset(
             *s4.offset(0i32 as (isize)) as (isize)
         ) = 0i32 as (u16);
        return;
    }
    memset(
        depth as (*mut std::os::raw::c_void),
        0i32,
        length.wrapping_mul(std::mem::size_of::<u8>())
    );
    BrotliCreateHuffmanTree(histogram,length,15i32,tree,depth);
    BrotliConvertBitDepthsToSymbols(depth as (*const u8),length,bits);
    if count <= 4i32 as (usize) {
        StoreSimpleHuffmanTree(
            depth as (*const u8),
            s4,
            count,
            max_bits,
            storage_ix,
            storage
        );
    } else {
        BrotliStoreHuffmanTree(
            depth as (*const u8),
            length,
            tree,
            storage_ix,
            storage
        );
    }
}

unsafe extern fn GetBlockLengthPrefixCode(
    mut len : u32,
    mut code : *mut usize,
    mut n_extra : *mut u32,
    mut extra : *mut u32
) {
    *code = BlockLengthPrefixCode(len) as (usize);
    *n_extra = (*kBlockLengthPrefixCode.offset(
                     *code as (isize)
                 )).nbits;
    *extra = len.wrapping_sub(
                 (*kBlockLengthPrefixCode.offset(*code as (isize))).offset
             );
}

unsafe extern fn StoreBlockSwitch(
    mut code : *mut BlockSplitCode,
    block_len : u32,
    block_type : u8,
    mut is_first_block : i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut typecode
        : usize
        = NextBlockTypeCode(
              &mut (*code).type_code_calculator as (*mut BlockTypeCodeCalculator),
              block_type
          );
    let mut lencode : usize;
    let mut len_nextra : u32;
    let mut len_extra : u32;
    if is_first_block == 0 {
        BrotliWriteBits(
            *(*code).type_depths.offset(typecode as (isize)) as (usize),
            *(*code).type_bits.offset(typecode as (isize)) as (usize),
            storage_ix,
            storage
        );
    }
    GetBlockLengthPrefixCode(
        block_len,
        &mut lencode as (*mut usize),
        &mut len_nextra as (*mut u32),
        &mut len_extra as (*mut u32)
    );
    BrotliWriteBits(
        *(*code).length_depths.offset(lencode as (isize)) as (usize),
        *(*code).length_bits.offset(lencode as (isize)) as (usize),
        storage_ix,
        storage
    );
    BrotliWriteBits(
        len_nextra as (usize),
        len_extra as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BuildAndStoreBlockSplitCode(
    mut types : *const u8,
    mut lengths : *const u32,
    num_blocks : usize,
    num_types : usize,
    mut tree : *mut HuffmanTree,
    mut code : *mut BlockSplitCode,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut type_histo : *mut u32;
    let mut length_histo : *mut u32;
    let mut i : usize;
    let mut type_code_calculator : BlockTypeCodeCalculator;
    memset(
        type_histo as (*mut std::os::raw::c_void),
        0i32,
        num_types.wrapping_add(2i32 as (usize)).wrapping_mul(
            std::mem::size_of::<u32>()
        )
    );
    memset(
        length_histo as (*mut std::os::raw::c_void),
        0i32,
        std::mem::size_of::<*mut u32>()
    );
    InitBlockTypeCodeCalculator(
        &mut type_code_calculator as (*mut BlockTypeCodeCalculator)
    );
    i = 0i32 as (usize);
    while i < num_blocks {
        {
            let mut type_code
                : usize
                = NextBlockTypeCode(
                      &mut type_code_calculator as (*mut BlockTypeCodeCalculator),
                      *types.offset(i as (isize))
                  );
            if i != 0i32 as (usize) {
                let _rhs = 1;
                let _lhs = &mut *type_histo.offset(type_code as (isize));
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            {
                let _rhs = 1;
                let _lhs
                    = &mut *length_histo.offset(
                                BlockLengthPrefixCode(*lengths.offset(i as (isize))) as (isize)
                            );
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    StoreVarLenUint8(
        num_types.wrapping_sub(1i32 as (usize)),
        storage_ix,
        storage
    );
    if num_types > 1i32 as (usize) {
        BuildAndStoreHuffmanTree(
            &mut *type_histo.offset(
                      0i32 as (isize)
                  ) as (*mut u32) as (*const u32),
            num_types.wrapping_add(2i32 as (usize)),
            tree,
            &mut *(*code).type_depths.offset(0i32 as (isize)) as (*mut u8),
            &mut *(*code).type_bits.offset(0i32 as (isize)) as (*mut u16),
            storage_ix,
            storage
        );
        BuildAndStoreHuffmanTree(
            &mut *length_histo.offset(
                      0i32 as (isize)
                  ) as (*mut u32) as (*const u32),
            26i32 as (usize),
            tree,
            &mut *(*code).length_depths.offset(0i32 as (isize)) as (*mut u8),
            &mut *(*code).length_bits.offset(0i32 as (isize)) as (*mut u16),
            storage_ix,
            storage
        );
        StoreBlockSwitch(
            code,
            *lengths.offset(0i32 as (isize)),
            *types.offset(0i32 as (isize)),
            1i32,
            storage_ix,
            storage
        );
    }
}

unsafe extern fn BuildAndStoreBlockSwitchEntropyCodes(
    mut self : *mut BlockEncoder,
    mut tree : *mut HuffmanTree,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BuildAndStoreBlockSplitCode(
        (*self).block_types_,
        (*self).block_lengths_,
        (*self).num_blocks_,
        (*self).num_block_types_,
        tree,
        &mut (*self).block_split_code_ as (*mut BlockSplitCode),
        storage_ix,
        storage
    );
}

unsafe extern fn StoreTrivialContextMap(
    mut num_types : usize,
    mut context_bits : usize,
    mut tree : *mut HuffmanTree,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    StoreVarLenUint8(
        num_types.wrapping_sub(1i32 as (usize)),
        storage_ix,
        storage
    );
    if num_types > 1i32 as (usize) {
        let mut repeat_code
            : usize
            = context_bits.wrapping_sub(1u32 as (usize));
        let mut repeat_bits
            : usize
            = (1u32 << repeat_code).wrapping_sub(1u32) as (usize);
        let mut alphabet_size
            : usize
            = num_types.wrapping_add(repeat_code);
        let mut histogram : *mut u32;
        let mut depths : *mut u8;
        let mut bits : *mut u16;
        let mut i : usize;
        memset(
            histogram as (*mut std::os::raw::c_void),
            0i32,
            alphabet_size.wrapping_mul(std::mem::size_of::<u32>())
        );
        BrotliWriteBits(
            1i32 as (usize),
            1i32 as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            4i32 as (usize),
            repeat_code.wrapping_sub(1i32 as (usize)),
            storage_ix,
            storage
        );
        *histogram.offset(repeat_code as (isize)) = num_types as (u32);
        *histogram.offset(0i32 as (isize)) = 1i32 as (u32);
        i = context_bits;
        while i < alphabet_size {
            {
                *histogram.offset(i as (isize)) = 1i32 as (u32);
            }
            i = i.wrapping_add(1 as (usize));
        }
        BuildAndStoreHuffmanTree(
            histogram as (*const u32),
            alphabet_size,
            tree,
            depths,
            bits,
            storage_ix,
            storage
        );
        i = 0i32 as (usize);
        while i < num_types {
            {
                let mut code
                    : usize
                    = if i == 0i32 as (usize) {
                          0i32 as (usize)
                      } else {
                          i.wrapping_add(context_bits).wrapping_sub(1i32 as (usize))
                      };
                BrotliWriteBits(
                    *depths.offset(code as (isize)) as (usize),
                    *bits.offset(code as (isize)) as (usize),
                    storage_ix,
                    storage
                );
                BrotliWriteBits(
                    *depths.offset(repeat_code as (isize)) as (usize),
                    *bits.offset(repeat_code as (isize)) as (usize),
                    storage_ix,
                    storage
                );
                BrotliWriteBits(repeat_code,repeat_bits,storage_ix,storage);
            }
            i = i.wrapping_add(1 as (usize));
        }
        BrotliWriteBits(
            1i32 as (usize),
            1i32 as (usize),
            storage_ix,
            storage
        );
    }
}

unsafe extern fn IndexOf(
    mut v : *const u8, mut v_size : usize, mut value : u8
) -> usize {
    let mut i : usize = 0i32 as (usize);
    while i < v_size {
        {
            if *v.offset(i as (isize)) as (i32) == value as (i32) {
                return i;
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    i
}

unsafe extern fn MoveToFront(mut v : *mut u8, mut index : usize) {
    let mut value : u8 = *v.offset(index as (isize));
    let mut i : usize;
    i = index;
    while i != 0i32 as (usize) {
        {
            *v.offset(i as (isize)) = *v.offset(
                                           i.wrapping_sub(1i32 as (usize)) as (isize)
                                       );
        }
        i = i.wrapping_sub(1 as (usize));
    }
    *v.offset(0i32 as (isize)) = value;
}

unsafe extern fn MoveToFrontTransform(
    mut v_in : *const u32, v_size : usize, mut v_out : *mut u32
) {
    let mut i : usize;
    let mut mtf : *mut u8;
    let mut max_value : u32;
    if v_size == 0i32 as (usize) {
        return;
    }
    max_value = *v_in.offset(0i32 as (isize));
    i = 1i32 as (usize);
    while i < v_size {
        {
            if *v_in.offset(i as (isize)) > max_value {
                max_value = *v_in.offset(i as (isize));
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    if max_value < 256u32 {
        0i32;
    } else {
        __assert_fail(
            b"max_value < 256u\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"MoveToFrontTransform\0".as_ptr()
        );
    }
    i = 0i32 as (usize);
    while i <= max_value as (usize) {
        {
            *mtf.offset(i as (isize)) = i as (u8);
        }
        i = i.wrapping_add(1 as (usize));
    }
    {
        let mut mtf_size
            : usize
            = max_value.wrapping_add(1i32 as (u32)) as (usize);
        i = 0i32 as (usize);
        while i < v_size {
            {
                let mut index
                    : usize
                    = IndexOf(
                          mtf as (*const u8),
                          mtf_size,
                          *v_in.offset(i as (isize)) as (u8)
                      );
                if index < mtf_size {
                    0i32;
                } else {
                    __assert_fail(
                        b"index < mtf_size\0".as_ptr(),
                        file!().as_ptr(),
                        line!(),
                        b"MoveToFrontTransform\0".as_ptr()
                    );
                }
                *v_out.offset(i as (isize)) = index as (u32);
                MoveToFront(mtf,index);
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
}

unsafe extern fn brotli_max_uint32_t(
    mut a : u32, mut b : u32
) -> u32 {
    if a > b { a } else { b }
}

unsafe extern fn brotli_min_uint32_t(
    mut a : u32, mut b : u32
) -> u32 {
    if a < b { a } else { b }
}

unsafe extern fn RunLengthCodeZeros(
    in_size : usize,
    mut v : *mut u32,
    mut out_size : *mut usize,
    mut max_run_length_prefix : *mut u32
) {
    let mut max_reps : u32 = 0i32 as (u32);
    let mut i : usize;
    let mut max_prefix : u32;
    i = 0i32 as (usize);
    while i < in_size {
        let mut reps : u32 = 0i32 as (u32);
        while i < in_size && (*v.offset(i as (isize)) != 0i32 as (u32)) {
            i = i.wrapping_add(1 as (usize));
        }
        while i < in_size && (*v.offset(i as (isize)) == 0i32 as (u32)) {
            {
                reps = reps.wrapping_add(1 as (u32));
            }
            i = i.wrapping_add(1 as (usize));
        }
        max_reps = brotli_max_uint32_t(reps,max_reps);
    }
    max_prefix = if max_reps > 0i32 as (u32) {
                     Log2FloorNonZero(max_reps as (usize))
                 } else {
                     0i32 as (u32)
                 };
    max_prefix = brotli_min_uint32_t(
                     max_prefix,
                     *max_run_length_prefix
                 );
    *max_run_length_prefix = max_prefix;
    *out_size = 0i32 as (usize);
    i = 0i32 as (usize);
    while i < in_size {
        if *out_size <= i {
            0i32;
        } else {
            __assert_fail(
                b"*out_size <= i\0".as_ptr(),
                file!().as_ptr(),
                line!(),
                b"RunLengthCodeZeros\0".as_ptr()
            );
        }
        if *v.offset(i as (isize)) != 0i32 as (u32) {
            *v.offset(*out_size as (isize)) = (*v.offset(
                                                    i as (isize)
                                                )).wrapping_add(
                                                  *max_run_length_prefix
                                              );
            i = i.wrapping_add(1 as (usize));
            *out_size = (*out_size).wrapping_add(1 as (usize));
        } else {
            let mut reps : u32 = 1i32 as (u32);
            let mut k : usize;
            k = i.wrapping_add(1i32 as (usize));
            while k < in_size && (*v.offset(k as (isize)) == 0i32 as (u32)) {
                {
                    reps = reps.wrapping_add(1 as (u32));
                }
                k = k.wrapping_add(1 as (usize));
            }
            i = i.wrapping_add(reps as (usize));
            while reps != 0i32 as (u32) {
                if reps < 2u32 << max_prefix {
                    let mut run_length_prefix
                        : u32
                        = Log2FloorNonZero(reps as (usize));
                    let extra_bits
                        : u32
                        = reps.wrapping_sub(1u32 << run_length_prefix);
                    *v.offset(*out_size as (isize)) = run_length_prefix.wrapping_add(
                                                          extra_bits << 9i32
                                                      );
                    *out_size = (*out_size).wrapping_add(1 as (usize));
                    {
                        if 1337i32 != 0 {
                            break;
                        }
                    }
                } else {
                    let extra_bits : u32 = (1u32 << max_prefix).wrapping_sub(1u32);
                    *v.offset(*out_size as (isize)) = max_prefix.wrapping_add(
                                                          extra_bits << 9i32
                                                      );
                    reps = reps.wrapping_sub((2u32 << max_prefix).wrapping_sub(1u32));
                    *out_size = (*out_size).wrapping_add(1 as (usize));
                }
            }
        }
    }
}

unsafe extern fn EncodeContextMap(
    mut m : *mut MemoryManager,
    mut context_map : *const u32,
    mut context_map_size : usize,
    mut num_clusters : usize,
    mut tree : *mut HuffmanTree,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut i : usize;
    let mut rle_symbols : *mut u32;
    let mut max_run_length_prefix : u32 = 6i32 as (u32);
    let mut num_rle_symbols : usize = 0i32 as (usize);
    let mut histogram : *mut u32;
    static kSymbolMask : u32 = (1u32 << 9i32).wrapping_sub(1u32);
    let mut depths : *mut u8;
    let mut bits : *mut u16;
    StoreVarLenUint8(
        num_clusters.wrapping_sub(1i32 as (usize)),
        storage_ix,
        storage
    );
    if num_clusters == 1i32 as (usize) {
        return;
    }
    rle_symbols = if context_map_size != 0 {
                      BrotliAllocate(
                          m,
                          context_map_size.wrapping_mul(std::mem::size_of::<u32>())
                      ) as (*mut u32)
                  } else {
                      0i32 as (*mut std::os::raw::c_void) as (*mut u32)
                  };
    if !(0i32 == 0) {
        return;
    }
    MoveToFrontTransform(context_map,context_map_size,rle_symbols);
    RunLengthCodeZeros(
        context_map_size,
        rle_symbols,
        &mut num_rle_symbols as (*mut usize),
        &mut max_run_length_prefix as (*mut u32)
    );
    memset(
        histogram as (*mut std::os::raw::c_void),
        0i32,
        std::mem::size_of::<*mut u32>()
    );
    i = 0i32 as (usize);
    while i < num_rle_symbols {
        {
            let _rhs = 1;
            let _lhs
                = &mut *histogram.offset(
                            (*rle_symbols.offset(i as (isize)) & kSymbolMask) as (isize)
                        );
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        i = i.wrapping_add(1 as (usize));
    }
    {
        let mut use_rle
            : i32
            = if !!(max_run_length_prefix > 0i32 as (u32)) {
                  1i32
              } else {
                  0i32
              };
        BrotliWriteBits(
            1i32 as (usize),
            use_rle as (usize),
            storage_ix,
            storage
        );
        if use_rle != 0 {
            BrotliWriteBits(
                4i32 as (usize),
                max_run_length_prefix.wrapping_sub(1i32 as (u32)) as (usize),
                storage_ix,
                storage
            );
        }
    }
    BuildAndStoreHuffmanTree(
        histogram as (*const u32),
        num_clusters.wrapping_add(max_run_length_prefix as (usize)),
        tree,
        depths,
        bits,
        storage_ix,
        storage
    );
    i = 0i32 as (usize);
    while i < num_rle_symbols {
        {
            let rle_symbol
                : u32
                = *rle_symbols.offset(i as (isize)) & kSymbolMask;
            let extra_bits_val
                : u32
                = *rle_symbols.offset(i as (isize)) >> 9i32;
            BrotliWriteBits(
                *depths.offset(rle_symbol as (isize)) as (usize),
                *bits.offset(rle_symbol as (isize)) as (usize),
                storage_ix,
                storage
            );
            if rle_symbol > 0i32 as (u32) && (rle_symbol <= max_run_length_prefix) {
                BrotliWriteBits(
                    rle_symbol as (usize),
                    extra_bits_val as (usize),
                    storage_ix,
                    storage
                );
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    BrotliWriteBits(
        1i32 as (usize),
        1i32 as (usize),
        storage_ix,
        storage
    );
    {
        BrotliFree(m,rle_symbols as (*mut std::os::raw::c_void));
        rle_symbols = 0i32 as (*mut std::os::raw::c_void) as (*mut u32);
    }
}

unsafe extern fn BuildAndStoreEntropyCodesLiteral(
    mut m : *mut MemoryManager,
    mut self : *mut BlockEncoder,
    mut histograms : *const HistogramLiteral,
    histograms_size : usize,
    mut tree : *mut HuffmanTree,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let alphabet_size : usize = (*self).alphabet_size_;
    let table_size
        : usize
        = histograms_size.wrapping_mul(alphabet_size);
    (*self).depths_ = if table_size != 0 {
                          BrotliAllocate(
                              m,
                              table_size.wrapping_mul(std::mem::size_of::<u8>())
                          ) as (*mut u8)
                      } else {
                          0i32 as (*mut std::os::raw::c_void) as (*mut u8)
                      };
    (*self).bits_ = if table_size != 0 {
                        BrotliAllocate(
                            m,
                            table_size.wrapping_mul(std::mem::size_of::<u16>())
                        ) as (*mut u16)
                    } else {
                        0i32 as (*mut std::os::raw::c_void) as (*mut u16)
                    };
    if !(0i32 == 0) {
        return;
    }
    {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < histograms_size {
            {
                let mut ix : usize = i.wrapping_mul(alphabet_size);
                BuildAndStoreHuffmanTree(
                    &mut *(*histograms.offset(i as (isize))).data_.offset(
                              0i32 as (isize)
                          ) as (*mut u32) as (*const u32),
                    alphabet_size,
                    tree,
                    &mut *(*self).depths_.offset(ix as (isize)) as (*mut u8),
                    &mut *(*self).bits_.offset(ix as (isize)) as (*mut u16),
                    storage_ix,
                    storage
                );
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
}

unsafe extern fn BuildAndStoreEntropyCodesCommand(
    mut m : *mut MemoryManager,
    mut self : *mut BlockEncoder,
    mut histograms : *const HistogramCommand,
    histograms_size : usize,
    mut tree : *mut HuffmanTree,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let alphabet_size : usize = (*self).alphabet_size_;
    let table_size
        : usize
        = histograms_size.wrapping_mul(alphabet_size);
    (*self).depths_ = if table_size != 0 {
                          BrotliAllocate(
                              m,
                              table_size.wrapping_mul(std::mem::size_of::<u8>())
                          ) as (*mut u8)
                      } else {
                          0i32 as (*mut std::os::raw::c_void) as (*mut u8)
                      };
    (*self).bits_ = if table_size != 0 {
                        BrotliAllocate(
                            m,
                            table_size.wrapping_mul(std::mem::size_of::<u16>())
                        ) as (*mut u16)
                    } else {
                        0i32 as (*mut std::os::raw::c_void) as (*mut u16)
                    };
    if !(0i32 == 0) {
        return;
    }
    {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < histograms_size {
            {
                let mut ix : usize = i.wrapping_mul(alphabet_size);
                BuildAndStoreHuffmanTree(
                    &mut *(*histograms.offset(i as (isize))).data_.offset(
                              0i32 as (isize)
                          ) as (*mut u32) as (*const u32),
                    alphabet_size,
                    tree,
                    &mut *(*self).depths_.offset(ix as (isize)) as (*mut u8),
                    &mut *(*self).bits_.offset(ix as (isize)) as (*mut u16),
                    storage_ix,
                    storage
                );
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
}

unsafe extern fn BuildAndStoreEntropyCodesDistance(
    mut m : *mut MemoryManager,
    mut self : *mut BlockEncoder,
    mut histograms : *const HistogramDistance,
    histograms_size : usize,
    mut tree : *mut HuffmanTree,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let alphabet_size : usize = (*self).alphabet_size_;
    let table_size
        : usize
        = histograms_size.wrapping_mul(alphabet_size);
    (*self).depths_ = if table_size != 0 {
                          BrotliAllocate(
                              m,
                              table_size.wrapping_mul(std::mem::size_of::<u8>())
                          ) as (*mut u8)
                      } else {
                          0i32 as (*mut std::os::raw::c_void) as (*mut u8)
                      };
    (*self).bits_ = if table_size != 0 {
                        BrotliAllocate(
                            m,
                            table_size.wrapping_mul(std::mem::size_of::<u16>())
                        ) as (*mut u16)
                    } else {
                        0i32 as (*mut std::os::raw::c_void) as (*mut u16)
                    };
    if !(0i32 == 0) {
        return;
    }
    {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < histograms_size {
            {
                let mut ix : usize = i.wrapping_mul(alphabet_size);
                BuildAndStoreHuffmanTree(
                    &mut *(*histograms.offset(i as (isize))).data_.offset(
                              0i32 as (isize)
                          ) as (*mut u32) as (*const u32),
                    alphabet_size,
                    tree,
                    &mut *(*self).depths_.offset(ix as (isize)) as (*mut u8),
                    &mut *(*self).bits_.offset(ix as (isize)) as (*mut u16),
                    storage_ix,
                    storage
                );
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
}

unsafe extern fn StoreSymbol(
    mut self : *mut BlockEncoder,
    mut symbol : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    if (*self).block_len_ == 0i32 as (usize) {
        let mut block_ix
            : usize
            = {
                  (*self).block_ix_ = (*self).block_ix_.wrapping_add(1 as (usize));
                  (*self).block_ix_
              };
        let mut block_len
            : u32
            = *(*self).block_lengths_.offset(block_ix as (isize));
        let mut block_type
            : u8
            = *(*self).block_types_.offset(block_ix as (isize));
        (*self).block_len_ = block_len as (usize);
        (*self).entropy_ix_ = (block_type as (usize)).wrapping_mul(
                                  (*self).alphabet_size_
                              );
        StoreBlockSwitch(
            &mut (*self).block_split_code_ as (*mut BlockSplitCode),
            block_len,
            block_type,
            0i32,
            storage_ix,
            storage
        );
    }
    (*self).block_len_ = (*self).block_len_.wrapping_sub(1 as (usize));
    {
        let mut ix : usize = (*self).entropy_ix_.wrapping_add(symbol);
        BrotliWriteBits(
            *(*self).depths_.offset(ix as (isize)) as (usize),
            *(*self).bits_.offset(ix as (isize)) as (usize),
            storage_ix,
            storage
        );
    }
}

unsafe extern fn CommandCopyLenCode(
    mut self : *const Command
) -> u32 {
    (*self).copy_len_ & 0xffffffi32 as (u32) ^ (*self).copy_len_ >> 24i32
}

unsafe extern fn GetInsertLengthCode(
    mut insertlen : usize
) -> u16 {
    if insertlen < 6i32 as (usize) {
        insertlen as (u16)
    } else if insertlen < 130i32 as (usize) {
        let mut nbits
            : u32
            = Log2FloorNonZero(
                  insertlen.wrapping_sub(2i32 as (usize))
              ).wrapping_sub(
                  1u32
              );
        ((nbits << 1i32) as (usize)).wrapping_add(
            insertlen.wrapping_sub(2i32 as (usize)) >> nbits
        ).wrapping_add(
            2i32 as (usize)
        ) as (u16)
    } else if insertlen < 2114i32 as (usize) {
        Log2FloorNonZero(
            insertlen.wrapping_sub(66i32 as (usize))
        ).wrapping_add(
            10i32 as (u32)
        ) as (u16)
    } else if insertlen < 6210i32 as (usize) {
        21u32 as (u16)
    } else if insertlen < 22594i32 as (usize) {
        22u32 as (u16)
    } else {
        23u32 as (u16)
    }
}

unsafe extern fn GetCopyLengthCode(mut copylen : usize) -> u16 {
    if copylen < 10i32 as (usize) {
        copylen.wrapping_sub(2i32 as (usize)) as (u16)
    } else if copylen < 134i32 as (usize) {
        let mut nbits
            : u32
            = Log2FloorNonZero(
                  copylen.wrapping_sub(6i32 as (usize))
              ).wrapping_sub(
                  1u32
              );
        ((nbits << 1i32) as (usize)).wrapping_add(
            copylen.wrapping_sub(6i32 as (usize)) >> nbits
        ).wrapping_add(
            4i32 as (usize)
        ) as (u16)
    } else if copylen < 2118i32 as (usize) {
        Log2FloorNonZero(
            copylen.wrapping_sub(70i32 as (usize))
        ).wrapping_add(
            12i32 as (u32)
        ) as (u16)
    } else {
        23u32 as (u16)
    }
}

unsafe extern fn GetInsertExtra(mut inscode : u16) -> u32 {
    *kInsExtra.offset(inscode as (isize))
}

unsafe extern fn GetInsertBase(mut inscode : u16) -> u32 {
    *kInsBase.offset(inscode as (isize))
}

unsafe extern fn GetCopyBase(mut copycode : u16) -> u32 {
    *kCopyBase.offset(copycode as (isize))
}

unsafe extern fn GetCopyExtra(mut copycode : u16) -> u32 {
    *kCopyExtra.offset(copycode as (isize))
}

unsafe extern fn StoreCommandExtra(
    mut cmd : *const Command,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut copylen_code : u32 = CommandCopyLenCode(cmd);
    let mut inscode
        : u16
        = GetInsertLengthCode((*cmd).insert_len_ as (usize));
    let mut copycode
        : u16
        = GetCopyLengthCode(copylen_code as (usize));
    let mut insnumextra : u32 = GetInsertExtra(inscode);
    let mut insextraval
        : usize
        = (*cmd).insert_len_.wrapping_sub(
              GetInsertBase(inscode)
          ) as (usize);
    let mut copyextraval
        : usize
        = copylen_code.wrapping_sub(GetCopyBase(copycode)) as (usize);
    let mut bits : usize = copyextraval << insnumextra | insextraval;
    BrotliWriteBits(
        insnumextra.wrapping_add(GetCopyExtra(copycode)) as (usize),
        bits,
        storage_ix,
        storage
    );
}

unsafe extern fn Context(
    mut p1 : u8, mut p2 : u8, mut mode : ContextType
) -> u8 {
    if mode as (i32) == ContextType::CONTEXT_LSB6 as (i32) {
        return (p1 as (i32) & 0x3fi32) as (u8);
    }
    if mode as (i32) == ContextType::CONTEXT_MSB6 as (i32) {
        return (p1 as (i32) >> 2i32) as (u8);
    }
    if mode as (i32) == ContextType::CONTEXT_UTF8 as (i32) {
        return
            (*kUTF8ContextLookup.offset(
                  p1 as (isize)
              ) as (i32) | *kUTF8ContextLookup.offset(
                                (p2 as (i32) + 256i32) as (isize)
                            ) as (i32)) as (u8);
    }
    if mode as (i32) == ContextType::CONTEXT_SIGNED as (i32) {
        return
            ((*kSigned3BitContextLookup.offset(
                   p1 as (isize)
               ) as (i32) << 3i32) + *kSigned3BitContextLookup.offset(
                                          p2 as (isize)
                                      ) as (i32)) as (u8);
    }
    0i32 as (u8)
}

unsafe extern fn StoreSymbolWithContext(
    mut self : *mut BlockEncoder,
    mut symbol : usize,
    mut context : usize,
    mut context_map : *const u32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8,
    context_bits : usize
) {
    if (*self).block_len_ == 0i32 as (usize) {
        let mut block_ix
            : usize
            = {
                  (*self).block_ix_ = (*self).block_ix_.wrapping_add(1 as (usize));
                  (*self).block_ix_
              };
        let mut block_len
            : u32
            = *(*self).block_lengths_.offset(block_ix as (isize));
        let mut block_type
            : u8
            = *(*self).block_types_.offset(block_ix as (isize));
        (*self).block_len_ = block_len as (usize);
        (*self).entropy_ix_ = block_type as (usize) << context_bits;
        StoreBlockSwitch(
            &mut (*self).block_split_code_ as (*mut BlockSplitCode),
            block_len,
            block_type,
            0i32,
            storage_ix,
            storage
        );
    }
    (*self).block_len_ = (*self).block_len_.wrapping_sub(1 as (usize));
    {
        let mut histo_ix
            : usize
            = *context_map.offset(
                   (*self).entropy_ix_.wrapping_add(context) as (isize)
               ) as (usize);
        let mut ix
            : usize
            = histo_ix.wrapping_mul((*self).alphabet_size_).wrapping_add(
                  symbol
              );
        BrotliWriteBits(
            *(*self).depths_.offset(ix as (isize)) as (usize),
            *(*self).bits_.offset(ix as (isize)) as (usize),
            storage_ix,
            storage
        );
    }
}

unsafe extern fn CommandCopyLen(mut self : *const Command) -> u32 {
    (*self).copy_len_ & 0xffffffi32 as (u32)
}

unsafe extern fn CommandDistanceContext(
    mut self : *const Command
) -> u32 {
    let mut r : u32 = ((*self).cmd_prefix_ as (i32) >> 6i32) as (u32);
    let mut c : u32 = ((*self).cmd_prefix_ as (i32) & 7i32) as (u32);
    if (r == 0i32 as (u32) || r == 2i32 as (u32) || r == 4i32 as (u32) || r == 7i32 as (u32)) && (c <= 2i32 as (u32)) {
        return c;
    }
    3i32 as (u32)
}

unsafe extern fn CleanupBlockEncoder(
    mut m : *mut MemoryManager, mut self : *mut BlockEncoder
) {
    {
        BrotliFree(m,(*self).depths_ as (*mut std::os::raw::c_void));
        (*self).depths_ = 0i32 as (*mut std::os::raw::c_void) as (*mut u8);
    }
    {
        BrotliFree(m,(*self).bits_ as (*mut std::os::raw::c_void));
        (*self).bits_ = 0i32 as (*mut std::os::raw::c_void) as (*mut u16);
    }
}

unsafe extern fn JumpToByteBoundary(
    mut storage_ix : *mut usize, mut storage : *mut u8
) {
    *storage_ix = (*storage_ix).wrapping_add(
                      7u32 as (usize)
                  ) & !7u32 as (usize);
    *storage.offset((*storage_ix >> 3i32) as (isize)) = 0i32 as (u8);
}

#[no_mangle]
pub unsafe extern fn BrotliStoreMetaBlock(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut start_pos : usize,
    mut length : usize,
    mut mask : usize,
    mut prev_byte : u8,
    mut prev_byte2 : u8,
    mut is_last : i32,
    mut num_direct_distance_codes : u32,
    mut distance_postfix_bits : u32,
    mut literal_context_mode : ContextType,
    mut commands : *const Command,
    mut n_commands : usize,
    mut mb : *const MetaBlockSplit,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut pos : usize = start_pos;
    let mut i : usize;
    let mut num_distance_codes
        : usize
        = (16i32 as (u32)).wrapping_add(
              num_direct_distance_codes
          ).wrapping_add(
              48u32 << distance_postfix_bits
          ) as (usize);
    let mut tree : *mut HuffmanTree;
    let mut literal_enc : BlockEncoder;
    let mut command_enc : BlockEncoder;
    let mut distance_enc : BlockEncoder;
    StoreCompressedMetaBlockHeader(is_last,length,storage_ix,storage);
    tree = if 2i32 * 704i32 + 1i32 != 0 {
               BrotliAllocate(
                   m,
                   ((2i32 * 704i32 + 1i32) as (usize)).wrapping_mul(
                       std::mem::size_of::<HuffmanTree>()
                   )
               ) as (*mut HuffmanTree)
           } else {
               0i32 as (*mut std::os::raw::c_void) as (*mut HuffmanTree)
           };
    if !(0i32 == 0) {
        return;
    }
    InitBlockEncoder(
        &mut literal_enc as (*mut BlockEncoder),
        256i32 as (usize),
        (*mb).literal_split.num_types,
        (*mb).literal_split.types as (*const u8),
        (*mb).literal_split.lengths as (*const u32),
        (*mb).literal_split.num_blocks
    );
    InitBlockEncoder(
        &mut command_enc as (*mut BlockEncoder),
        704i32 as (usize),
        (*mb).command_split.num_types,
        (*mb).command_split.types as (*const u8),
        (*mb).command_split.lengths as (*const u32),
        (*mb).command_split.num_blocks
    );
    InitBlockEncoder(
        &mut distance_enc as (*mut BlockEncoder),
        num_distance_codes,
        (*mb).distance_split.num_types,
        (*mb).distance_split.types as (*const u8),
        (*mb).distance_split.lengths as (*const u32),
        (*mb).distance_split.num_blocks
    );
    BuildAndStoreBlockSwitchEntropyCodes(
        &mut literal_enc as (*mut BlockEncoder),
        tree,
        storage_ix,
        storage
    );
    BuildAndStoreBlockSwitchEntropyCodes(
        &mut command_enc as (*mut BlockEncoder),
        tree,
        storage_ix,
        storage
    );
    BuildAndStoreBlockSwitchEntropyCodes(
        &mut distance_enc as (*mut BlockEncoder),
        tree,
        storage_ix,
        storage
    );
    BrotliWriteBits(
        2i32 as (usize),
        distance_postfix_bits as (usize),
        storage_ix,
        storage
    );
    BrotliWriteBits(
        4i32 as (usize),
        (num_direct_distance_codes >> distance_postfix_bits) as (usize),
        storage_ix,
        storage
    );
    i = 0i32 as (usize);
    while i < (*mb).literal_split.num_types {
        {
            BrotliWriteBits(
                2i32 as (usize),
                literal_context_mode as (usize),
                storage_ix,
                storage
            );
        }
        i = i.wrapping_add(1 as (usize));
    }
    if (*mb).literal_context_map_size == 0i32 as (usize) {
        StoreTrivialContextMap(
            (*mb).literal_histograms_size,
            6i32 as (usize),
            tree,
            storage_ix,
            storage
        );
    } else {
        EncodeContextMap(
            m,
            (*mb).literal_context_map as (*const u32),
            (*mb).literal_context_map_size,
            (*mb).literal_histograms_size,
            tree,
            storage_ix,
            storage
        );
        if !(0i32 == 0) {
            return;
        }
    }
    if (*mb).distance_context_map_size == 0i32 as (usize) {
        StoreTrivialContextMap(
            (*mb).distance_histograms_size,
            2i32 as (usize),
            tree,
            storage_ix,
            storage
        );
    } else {
        EncodeContextMap(
            m,
            (*mb).distance_context_map as (*const u32),
            (*mb).distance_context_map_size,
            (*mb).distance_histograms_size,
            tree,
            storage_ix,
            storage
        );
        if !(0i32 == 0) {
            return;
        }
    }
    BuildAndStoreEntropyCodesLiteral(
        m,
        &mut literal_enc as (*mut BlockEncoder),
        (*mb).literal_histograms as (*const HistogramLiteral),
        (*mb).literal_histograms_size,
        tree,
        storage_ix,
        storage
    );
    if !(0i32 == 0) {
        return;
    }
    BuildAndStoreEntropyCodesCommand(
        m,
        &mut command_enc as (*mut BlockEncoder),
        (*mb).command_histograms as (*const HistogramCommand),
        (*mb).command_histograms_size,
        tree,
        storage_ix,
        storage
    );
    if !(0i32 == 0) {
        return;
    }
    BuildAndStoreEntropyCodesDistance(
        m,
        &mut distance_enc as (*mut BlockEncoder),
        (*mb).distance_histograms as (*const HistogramDistance),
        (*mb).distance_histograms_size,
        tree,
        storage_ix,
        storage
    );
    if !(0i32 == 0) {
        return;
    }
    {
        BrotliFree(m,tree as (*mut std::os::raw::c_void));
        tree = 0i32 as (*mut std::os::raw::c_void) as (*mut HuffmanTree);
    }
    i = 0i32 as (usize);
    while i < n_commands {
        {
            let cmd : Command = *commands.offset(i as (isize));
            let mut cmd_code : usize = cmd.cmd_prefix_ as (usize);
            StoreSymbol(
                &mut command_enc as (*mut BlockEncoder),
                cmd_code,
                storage_ix,
                storage
            );
            StoreCommandExtra(&cmd as (*const Command),storage_ix,storage);
            if (*mb).literal_context_map_size == 0i32 as (usize) {
                let mut j : usize;
                j = cmd.insert_len_ as (usize);
                while j != 0i32 as (usize) {
                    {
                        StoreSymbol(
                            &mut literal_enc as (*mut BlockEncoder),
                            *input.offset((pos & mask) as (isize)) as (usize),
                            storage_ix,
                            storage
                        );
                        pos = pos.wrapping_add(1 as (usize));
                    }
                    j = j.wrapping_sub(1 as (usize));
                }
            } else {
                let mut j : usize;
                j = cmd.insert_len_ as (usize);
                while j != 0i32 as (usize) {
                    {
                        let mut context
                            : usize
                            = Context(prev_byte,prev_byte2,literal_context_mode) as (usize);
                        let mut literal : u8 = *input.offset((pos & mask) as (isize));
                        StoreSymbolWithContext(
                            &mut literal_enc as (*mut BlockEncoder),
                            literal as (usize),
                            context,
                            (*mb).literal_context_map as (*const u32),
                            storage_ix,
                            storage,
                            6i32 as (usize)
                        );
                        prev_byte2 = prev_byte;
                        prev_byte = literal;
                        pos = pos.wrapping_add(1 as (usize));
                    }
                    j = j.wrapping_sub(1 as (usize));
                }
            }
            pos = pos.wrapping_add(
                      CommandCopyLen(&cmd as (*const Command)) as (usize)
                  );
            if CommandCopyLen(&cmd as (*const Command)) != 0 {
                prev_byte2 = *input.offset(
                                  (pos.wrapping_sub(2i32 as (usize)) & mask) as (isize)
                              );
                prev_byte = *input.offset(
                                 (pos.wrapping_sub(1i32 as (usize)) & mask) as (isize)
                             );
                if cmd.cmd_prefix_ as (i32) >= 128i32 {
                    let mut dist_code : usize = cmd.dist_prefix_ as (usize);
                    let mut distnumextra : u32 = cmd.dist_extra_ >> 24i32;
                    let mut distextra
                        : usize
                        = (cmd.dist_extra_ & 0xffffffi32 as (u32)) as (usize);
                    if (*mb).distance_context_map_size == 0i32 as (usize) {
                        StoreSymbol(
                            &mut distance_enc as (*mut BlockEncoder),
                            dist_code,
                            storage_ix,
                            storage
                        );
                    } else {
                        let mut context
                            : usize
                            = CommandDistanceContext(&cmd as (*const Command)) as (usize);
                        StoreSymbolWithContext(
                            &mut distance_enc as (*mut BlockEncoder),
                            dist_code,
                            context,
                            (*mb).distance_context_map as (*const u32),
                            storage_ix,
                            storage,
                            2i32 as (usize)
                        );
                    }
                    BrotliWriteBits(
                        distnumextra as (usize),
                        distextra,
                        storage_ix,
                        storage
                    );
                }
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    CleanupBlockEncoder(m,&mut distance_enc as (*mut BlockEncoder));
    CleanupBlockEncoder(m,&mut command_enc as (*mut BlockEncoder));
    CleanupBlockEncoder(m,&mut literal_enc as (*mut BlockEncoder));
    if is_last != 0 {
        JumpToByteBoundary(storage_ix,storage);
    }
}

unsafe extern fn HistogramClearLiteral(
    mut self : *mut HistogramLiteral
) {
    memset(
        (*self).data_ as (*mut std::os::raw::c_void),
        0i32,
        std::mem::size_of::<*mut u32>()
    );
    (*self).total_count_ = 0i32 as (usize);
    (*self).bit_cost_ = 3.402e+38f64;
}

unsafe extern fn HistogramClearCommand(
    mut self : *mut HistogramCommand
) {
    memset(
        (*self).data_ as (*mut std::os::raw::c_void),
        0i32,
        std::mem::size_of::<*mut u32>()
    );
    (*self).total_count_ = 0i32 as (usize);
    (*self).bit_cost_ = 3.402e+38f64;
}

unsafe extern fn HistogramClearDistance(
    mut self : *mut HistogramDistance
) {
    memset(
        (*self).data_ as (*mut std::os::raw::c_void),
        0i32,
        std::mem::size_of::<*mut u32>()
    );
    (*self).total_count_ = 0i32 as (usize);
    (*self).bit_cost_ = 3.402e+38f64;
}

unsafe extern fn HistogramAddCommand(
    mut self : *mut HistogramCommand, mut val : usize
) {
    {
        let _rhs = 1;
        let _lhs = &mut *(*self).data_.offset(val as (isize));
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               1 as (usize)
                           );
}

unsafe extern fn HistogramAddLiteral(
    mut self : *mut HistogramLiteral, mut val : usize
) {
    {
        let _rhs = 1;
        let _lhs = &mut *(*self).data_.offset(val as (isize));
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               1 as (usize)
                           );
}

unsafe extern fn HistogramAddDistance(
    mut self : *mut HistogramDistance, mut val : usize
) {
    {
        let _rhs = 1;
        let _lhs = &mut *(*self).data_.offset(val as (isize));
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               1 as (usize)
                           );
}

unsafe extern fn BuildHistograms(
    mut input : *const u8,
    mut start_pos : usize,
    mut mask : usize,
    mut commands : *const Command,
    mut n_commands : usize,
    mut lit_histo : *mut HistogramLiteral,
    mut cmd_histo : *mut HistogramCommand,
    mut dist_histo : *mut HistogramDistance
) {
    let mut pos : usize = start_pos;
    let mut i : usize;
    i = 0i32 as (usize);
    while i < n_commands {
        {
            let cmd : Command = *commands.offset(i as (isize));
            let mut j : usize;
            HistogramAddCommand(cmd_histo,cmd.cmd_prefix_ as (usize));
            j = cmd.insert_len_ as (usize);
            while j != 0i32 as (usize) {
                {
                    HistogramAddLiteral(
                        lit_histo,
                        *input.offset((pos & mask) as (isize)) as (usize)
                    );
                    pos = pos.wrapping_add(1 as (usize));
                }
                j = j.wrapping_sub(1 as (usize));
            }
            pos = pos.wrapping_add(
                      CommandCopyLen(&cmd as (*const Command)) as (usize)
                  );
            if CommandCopyLen(
                   &cmd as (*const Command)
               ) != 0 && (cmd.cmd_prefix_ as (i32) >= 128i32) {
                HistogramAddDistance(dist_histo,cmd.dist_prefix_ as (usize));
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn StoreDataWithHuffmanCodes(
    mut input : *const u8,
    mut start_pos : usize,
    mut mask : usize,
    mut commands : *const Command,
    mut n_commands : usize,
    mut lit_depth : *const u8,
    mut lit_bits : *const u16,
    mut cmd_depth : *const u8,
    mut cmd_bits : *const u16,
    mut dist_depth : *const u8,
    mut dist_bits : *const u16,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut pos : usize = start_pos;
    let mut i : usize;
    i = 0i32 as (usize);
    while i < n_commands {
        {
            let cmd : Command = *commands.offset(i as (isize));
            let cmd_code : usize = cmd.cmd_prefix_ as (usize);
            let mut j : usize;
            BrotliWriteBits(
                *cmd_depth.offset(cmd_code as (isize)) as (usize),
                *cmd_bits.offset(cmd_code as (isize)) as (usize),
                storage_ix,
                storage
            );
            StoreCommandExtra(&cmd as (*const Command),storage_ix,storage);
            j = cmd.insert_len_ as (usize);
            while j != 0i32 as (usize) {
                {
                    let literal : u8 = *input.offset((pos & mask) as (isize));
                    BrotliWriteBits(
                        *lit_depth.offset(literal as (isize)) as (usize),
                        *lit_bits.offset(literal as (isize)) as (usize),
                        storage_ix,
                        storage
                    );
                    pos = pos.wrapping_add(1 as (usize));
                }
                j = j.wrapping_sub(1 as (usize));
            }
            pos = pos.wrapping_add(
                      CommandCopyLen(&cmd as (*const Command)) as (usize)
                  );
            if CommandCopyLen(
                   &cmd as (*const Command)
               ) != 0 && (cmd.cmd_prefix_ as (i32) >= 128i32) {
                let dist_code : usize = cmd.dist_prefix_ as (usize);
                let distnumextra : u32 = cmd.dist_extra_ >> 24i32;
                let distextra : u32 = cmd.dist_extra_ & 0xffffffi32 as (u32);
                BrotliWriteBits(
                    *dist_depth.offset(dist_code as (isize)) as (usize),
                    *dist_bits.offset(dist_code as (isize)) as (usize),
                    storage_ix,
                    storage
                );
                BrotliWriteBits(
                    distnumextra as (usize),
                    distextra as (usize),
                    storage_ix,
                    storage
                );
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
}

#[no_mangle]
pub unsafe extern fn BrotliStoreMetaBlockTrivial(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut start_pos : usize,
    mut length : usize,
    mut mask : usize,
    mut is_last : i32,
    mut commands : *const Command,
    mut n_commands : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut lit_histo : HistogramLiteral;
    let mut cmd_histo : HistogramCommand;
    let mut dist_histo : HistogramDistance;
    let mut lit_depth : *mut u8;
    let mut lit_bits : *mut u16;
    let mut cmd_depth : *mut u8;
    let mut cmd_bits : *mut u16;
    let mut dist_depth : *mut u8;
    let mut dist_bits : *mut u16;
    let mut tree : *mut HuffmanTree;
    StoreCompressedMetaBlockHeader(is_last,length,storage_ix,storage);
    HistogramClearLiteral(&mut lit_histo as (*mut HistogramLiteral));
    HistogramClearCommand(&mut cmd_histo as (*mut HistogramCommand));
    HistogramClearDistance(
        &mut dist_histo as (*mut HistogramDistance)
    );
    BuildHistograms(
        input,
        start_pos,
        mask,
        commands,
        n_commands,
        &mut lit_histo as (*mut HistogramLiteral),
        &mut cmd_histo as (*mut HistogramCommand),
        &mut dist_histo as (*mut HistogramDistance)
    );
    BrotliWriteBits(
        13i32 as (usize),
        0i32 as (usize),
        storage_ix,
        storage
    );
    tree = if 2i32 * 704i32 + 1i32 != 0 {
               BrotliAllocate(
                   m,
                   ((2i32 * 704i32 + 1i32) as (usize)).wrapping_mul(
                       std::mem::size_of::<HuffmanTree>()
                   )
               ) as (*mut HuffmanTree)
           } else {
               0i32 as (*mut std::os::raw::c_void) as (*mut HuffmanTree)
           };
    if !(0i32 == 0) {
        return;
    }
    BuildAndStoreHuffmanTree(
        lit_histo.data_ as (*const u32),
        256i32 as (usize),
        tree,
        lit_depth,
        lit_bits,
        storage_ix,
        storage
    );
    BuildAndStoreHuffmanTree(
        cmd_histo.data_ as (*const u32),
        704i32 as (usize),
        tree,
        cmd_depth,
        cmd_bits,
        storage_ix,
        storage
    );
    BuildAndStoreHuffmanTree(
        dist_histo.data_ as (*const u32),
        64i32 as (usize),
        tree,
        dist_depth,
        dist_bits,
        storage_ix,
        storage
    );
    {
        BrotliFree(m,tree as (*mut std::os::raw::c_void));
        tree = 0i32 as (*mut std::os::raw::c_void) as (*mut HuffmanTree);
    }
    StoreDataWithHuffmanCodes(
        input,
        start_pos,
        mask,
        commands,
        n_commands,
        lit_depth as (*const u8),
        lit_bits as (*const u16),
        cmd_depth as (*const u8),
        cmd_bits as (*const u16),
        dist_depth as (*const u8),
        dist_bits as (*const u16),
        storage_ix,
        storage
    );
    if is_last != 0 {
        JumpToByteBoundary(storage_ix,storage);
    }
}

unsafe extern fn StoreStaticCommandHuffmanTree(
    mut storage_ix : *mut usize, mut storage : *mut u8
) {
    BrotliWriteBits(
        56i32 as (usize),
        0x926244u32 as (usize) << 32i32 | 0x16307003u32 as (usize),
        storage_ix,
        storage
    );
    BrotliWriteBits(
        3i32 as (usize),
        0x0u32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn StoreStaticDistanceHuffmanTree(
    mut storage_ix : *mut usize, mut storage : *mut u8
) {
    BrotliWriteBits(
        28i32 as (usize),
        0x369dc03u32 as (usize),
        storage_ix,
        storage
    );
}

#[no_mangle]
pub unsafe extern fn BrotliStoreMetaBlockFast(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut start_pos : usize,
    mut length : usize,
    mut mask : usize,
    mut is_last : i32,
    mut commands : *const Command,
    mut n_commands : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    StoreCompressedMetaBlockHeader(is_last,length,storage_ix,storage);
    BrotliWriteBits(
        13i32 as (usize),
        0i32 as (usize),
        storage_ix,
        storage
    );
    if n_commands <= 128i32 as (usize) {
        let mut histogram : *mut u32 = 0i32 as (*mut u32);
        let mut pos : usize = start_pos;
        let mut num_literals : usize = 0i32 as (usize);
        let mut i : usize;
        let mut lit_depth : *mut u8;
        let mut lit_bits : *mut u16;
        i = 0i32 as (usize);
        while i < n_commands {
            {
                let cmd : Command = *commands.offset(i as (isize));
                let mut j : usize;
                j = cmd.insert_len_ as (usize);
                while j != 0i32 as (usize) {
                    {
                        {
                            let _rhs = 1;
                            let _lhs
                                = &mut *histogram.offset(
                                            *input.offset((pos & mask) as (isize)) as (isize)
                                        );
                            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                        }
                        pos = pos.wrapping_add(1 as (usize));
                    }
                    j = j.wrapping_sub(1 as (usize));
                }
                num_literals = num_literals.wrapping_add(
                                   cmd.insert_len_ as (usize)
                               );
                pos = pos.wrapping_add(
                          CommandCopyLen(&cmd as (*const Command)) as (usize)
                      );
            }
            i = i.wrapping_add(1 as (usize));
        }
        BrotliBuildAndStoreHuffmanTreeFast(
            m,
            histogram as (*const u32),
            num_literals,
            8i32 as (usize),
            lit_depth,
            lit_bits,
            storage_ix,
            storage
        );
        if !(0i32 == 0) {
            return;
        }
        StoreStaticCommandHuffmanTree(storage_ix,storage);
        StoreStaticDistanceHuffmanTree(storage_ix,storage);
        StoreDataWithHuffmanCodes(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            lit_depth as (*const u8),
            lit_bits as (*const u16),
            kStaticCommandCodeDepth,
            kStaticCommandCodeBits,
            kStaticDistanceCodeDepth,
            kStaticDistanceCodeBits,
            storage_ix,
            storage
        );
    } else {
        let mut lit_histo : HistogramLiteral;
        let mut cmd_histo : HistogramCommand;
        let mut dist_histo : HistogramDistance;
        let mut lit_depth : *mut u8;
        let mut lit_bits : *mut u16;
        let mut cmd_depth : *mut u8;
        let mut cmd_bits : *mut u16;
        let mut dist_depth : *mut u8;
        let mut dist_bits : *mut u16;
        HistogramClearLiteral(&mut lit_histo as (*mut HistogramLiteral));
        HistogramClearCommand(&mut cmd_histo as (*mut HistogramCommand));
        HistogramClearDistance(
            &mut dist_histo as (*mut HistogramDistance)
        );
        BuildHistograms(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            &mut lit_histo as (*mut HistogramLiteral),
            &mut cmd_histo as (*mut HistogramCommand),
            &mut dist_histo as (*mut HistogramDistance)
        );
        BrotliBuildAndStoreHuffmanTreeFast(
            m,
            lit_histo.data_ as (*const u32),
            lit_histo.total_count_,
            8i32 as (usize),
            lit_depth,
            lit_bits,
            storage_ix,
            storage
        );
        if !(0i32 == 0) {
            return;
        }
        BrotliBuildAndStoreHuffmanTreeFast(
            m,
            cmd_histo.data_ as (*const u32),
            cmd_histo.total_count_,
            10i32 as (usize),
            cmd_depth,
            cmd_bits,
            storage_ix,
            storage
        );
        if !(0i32 == 0) {
            return;
        }
        BrotliBuildAndStoreHuffmanTreeFast(
            m,
            dist_histo.data_ as (*const u32),
            dist_histo.total_count_,
            6i32 as (usize),
            dist_depth,
            dist_bits,
            storage_ix,
            storage
        );
        if !(0i32 == 0) {
            return;
        }
        StoreDataWithHuffmanCodes(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            lit_depth as (*const u8),
            lit_bits as (*const u16),
            cmd_depth as (*const u8),
            cmd_bits as (*const u16),
            dist_depth as (*const u8),
            dist_bits as (*const u16),
            storage_ix,
            storage
        );
    }
    if is_last != 0 {
        JumpToByteBoundary(storage_ix,storage);
    }
}

unsafe extern fn BrotliStoreUncompressedMetaBlockHeader(
    mut length : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut lenbits : usize;
    let mut nlenbits : usize;
    let mut nibblesbits : usize;
    BrotliWriteBits(
        1i32 as (usize),
        0i32 as (usize),
        storage_ix,
        storage
    );
    BrotliEncodeMlen(
        length,
        &mut lenbits as (*mut usize),
        &mut nlenbits as (*mut usize),
        &mut nibblesbits as (*mut usize)
    );
    BrotliWriteBits(2i32 as (usize),nibblesbits,storage_ix,storage);
    BrotliWriteBits(nlenbits,lenbits,storage_ix,storage);
    BrotliWriteBits(
        1i32 as (usize),
        1i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliWriteBitsPrepareStorage(
    mut pos : usize, mut array : *mut u8
) {
    if pos & 7i32 as (usize) == 0i32 as (usize) {
        0i32;
    } else {
        __assert_fail(
            b"(pos & 7) == 0\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"BrotliWriteBitsPrepareStorage\0".as_ptr()
        );
    }
    *array.offset((pos >> 3i32) as (isize)) = 0i32 as (u8);
}

#[no_mangle]
pub unsafe extern fn BrotliStoreUncompressedMetaBlock(
    mut is_final_block : i32,
    mut input : *const u8,
    mut position : usize,
    mut mask : usize,
    mut len : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut masked_pos : usize = position & mask;
    BrotliStoreUncompressedMetaBlockHeader(len,storage_ix,storage);
    JumpToByteBoundary(storage_ix,storage);
    if masked_pos.wrapping_add(len) > mask.wrapping_add(
                                          1i32 as (usize)
                                      ) {
        let mut len1
            : usize
            = mask.wrapping_add(1i32 as (usize)).wrapping_sub(masked_pos);
        memcpy(
            &mut *storage.offset(
                      (*storage_ix >> 3i32) as (isize)
                  ) as (*mut u8) as (*mut std::os::raw::c_void),
            &*input.offset(
                  masked_pos as (isize)
              ) as (*const u8) as (*const std::os::raw::c_void),
            len1
        );
        *storage_ix = (*storage_ix).wrapping_add(len1 << 3i32);
        len = len.wrapping_sub(len1);
        masked_pos = 0i32 as (usize);
    }
    memcpy(
        &mut *storage.offset(
                  (*storage_ix >> 3i32) as (isize)
              ) as (*mut u8) as (*mut std::os::raw::c_void),
        &*input.offset(
              masked_pos as (isize)
          ) as (*const u8) as (*const std::os::raw::c_void),
        len
    );
    *storage_ix = (*storage_ix).wrapping_add(len << 3i32);
    BrotliWriteBitsPrepareStorage(*storage_ix,storage);
    if is_final_block != 0 {
        BrotliWriteBits(
            1i32 as (usize),
            1i32 as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            1i32 as (usize),
            1i32 as (usize),
            storage_ix,
            storage
        );
        JumpToByteBoundary(storage_ix,storage);
    }
}

#[no_mangle]
pub unsafe extern fn BrotliStoreSyncMetaBlock(
    mut storage_ix : *mut usize, mut storage : *mut u8
) {
    BrotliWriteBits(
        6i32 as (usize),
        6i32 as (usize),
        storage_ix,
        storage
    );
    JumpToByteBoundary(storage_ix,storage);
}
