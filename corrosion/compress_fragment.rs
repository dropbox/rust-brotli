extern {
    fn BrotliBuildAndStoreHuffmanTreeFast(
        m : *mut MemoryManager,
        histogram : *const u32,
        histogram_total : usize,
        max_bits : usize,
        depth : *mut u8,
        bits : *mut u16,
        storage_ix : *mut usize,
        storage : *mut u8
    );
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
    fn BrotliStoreHuffmanTree(
        depths : *const u8,
        num : usize,
        tree : *mut HuffmanTree,
        storage_ix : *mut usize,
        storage : *mut u8
    );
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn log2(__x : f64) -> f64;
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

static kHashMul32 : u32 = 0x1e35a7bdi32 as (u32);

static mut kCmdHistoSeed : *mut u32 = 0i32 as (*mut u32);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func : unsafe extern fn(*mut std::os::raw::c_void, usize) -> *mut std::os::raw::c_void,
    pub free_func : unsafe extern fn(*mut std::os::raw::c_void, *mut std::os::raw::c_void),
    pub opaque : *mut std::os::raw::c_void,
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

unsafe extern fn brotli_min_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a < b { a } else { b }
}

unsafe extern fn BrotliStoreMetaBlockHeader(
    mut len : usize,
    mut is_uncompressed : i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut nibbles : usize = 6i32 as (usize);
    BrotliWriteBits(
        1i32 as (usize),
        0i32 as (usize),
        storage_ix,
        storage
    );
    if len <= (1u32 << 16i32) as (usize) {
        nibbles = 4i32 as (usize);
    } else if len <= (1u32 << 20i32) as (usize) {
        nibbles = 5i32 as (usize);
    }
    BrotliWriteBits(
        2i32 as (usize),
        nibbles.wrapping_sub(4i32 as (usize)),
        storage_ix,
        storage
    );
    BrotliWriteBits(
        nibbles.wrapping_mul(4i32 as (usize)),
        len.wrapping_sub(1i32 as (usize)),
        storage_ix,
        storage
    );
    BrotliWriteBits(
        1i32 as (usize),
        is_uncompressed as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn brotli_min_uint32_t(
    mut a : u32, mut b : u32
) -> u32 {
    if a < b { a } else { b }
}

unsafe extern fn BuildAndStoreLiteralPrefixCode(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    input_size : usize,
    mut depths : *mut u8,
    mut bits : *mut u16,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) -> usize {
    let mut histogram : *mut u32 = 0i32 as (*mut u32);
    let mut histogram_total : usize;
    let mut i : usize;
    if input_size < (1i32 << 15i32) as (usize) {
        i = 0i32 as (usize);
        while i < input_size {
            {
                let _rhs = 1;
                let _lhs
                    = &mut *histogram.offset(*input.offset(i as (isize)) as (isize));
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            i = i.wrapping_add(1 as (usize));
        }
        histogram_total = input_size;
        i = 0i32 as (usize);
        while i < 256i32 as (usize) {
            {
                let adjust
                    : u32
                    = (2i32 as (u32)).wrapping_mul(
                          brotli_min_uint32_t(*histogram.offset(i as (isize)),11u32)
                      );
                {
                    let _rhs = adjust;
                    let _lhs = &mut *histogram.offset(i as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                }
                histogram_total = histogram_total.wrapping_add(adjust as (usize));
            }
            i = i.wrapping_add(1 as (usize));
        }
    } else {
        static kSampleRate : usize = 29i32 as (usize);
        i = 0i32 as (usize);
        while i < input_size {
            {
                let _rhs = 1;
                let _lhs
                    = &mut *histogram.offset(*input.offset(i as (isize)) as (isize));
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            i = i.wrapping_add(kSampleRate);
        }
        histogram_total = input_size.wrapping_add(
                              kSampleRate
                          ).wrapping_sub(
                              1i32 as (usize)
                          ).wrapping_div(
                              kSampleRate
                          );
        i = 0i32 as (usize);
        while i < 256i32 as (usize) {
            {
                let adjust
                    : u32
                    = (1i32 as (u32)).wrapping_add(
                          (2i32 as (u32)).wrapping_mul(
                              brotli_min_uint32_t(*histogram.offset(i as (isize)),11u32)
                          )
                      );
                {
                    let _rhs = adjust;
                    let _lhs = &mut *histogram.offset(i as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                }
                histogram_total = histogram_total.wrapping_add(adjust as (usize));
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    BrotliBuildAndStoreHuffmanTreeFast(
        m,
        histogram as (*const u32),
        histogram_total,
        8i32 as (usize),
        depths,
        bits,
        storage_ix,
        storage
    );
    if !(0i32 == 0) {
        return 0i32 as (usize);
    }
    {
        let mut literal_ratio : usize = 0i32 as (usize);
        i = 0i32 as (usize);
        while i < 256i32 as (usize) {
            {
                if *histogram.offset(i as (isize)) != 0 {
                    literal_ratio = literal_ratio.wrapping_add(
                                        (*histogram.offset(i as (isize))).wrapping_mul(
                                            *depths.offset(i as (isize)) as (u32)
                                        ) as (usize)
                                    );
                }
            }
            i = i.wrapping_add(1 as (usize));
        }
        literal_ratio.wrapping_mul(125i32 as (usize)).wrapping_div(
            histogram_total
        )
    }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum CodeBlockState {
    EMIT_REMAINDER,
    EMIT_COMMANDS,
    NEXT_BLOCK,
}

unsafe extern fn BROTLI_UNALIGNED_LOAD64(
    mut p : *const std::os::raw::c_void
) -> usize {
    let mut t : usize;
    memcpy(
        &mut t as (*mut usize) as (*mut std::os::raw::c_void),
        p,
        std::mem::size_of::<usize>()
    );
    t
}

unsafe extern fn Hash(
    mut p : *const u8, mut shift : usize
) -> u32 {
    let h
        : usize
        = (BROTLI_UNALIGNED_LOAD64(
               p as (*const std::os::raw::c_void)
           ) << 24i32).wrapping_mul(
              kHashMul32 as (usize)
          );
    (h >> shift) as (u32)
}

unsafe extern fn BROTLI_UNALIGNED_LOAD32(
    mut p : *const std::os::raw::c_void
) -> u32 {
    let mut t : u32;
    memcpy(
        &mut t as (*mut u32) as (*mut std::os::raw::c_void),
        p,
        std::mem::size_of::<u32>()
    );
    t
}

unsafe extern fn IsMatch(
    mut p1 : *const u8, mut p2 : *const u8
) -> i32 {
    if !!(BROTLI_UNALIGNED_LOAD32(
              p1 as (*const std::os::raw::c_void)
          ) == BROTLI_UNALIGNED_LOAD32(
                   p2 as (*const std::os::raw::c_void)
               ) && (*p1.offset(4i32 as (isize)) as (i32) == *p2.offset(
                                                                  4i32 as (isize)
                                                              ) as (i32))) {
        1i32
    } else {
        0i32
    }
}

unsafe extern fn unopt_ctzll(mut val : usize) -> u8 {
    let mut cnt : u8 = 0i32 as (u8);
    while val & 1i32 as (usize) == 0i32 as (usize) {
        val = val >> 1i32;
        cnt = (cnt as (i32) + 1) as (u8);
    }
    cnt
}

unsafe extern fn FindMatchLengthWithLimit(
    mut s1 : *const u8, mut s2 : *const u8, mut limit : usize
) -> usize {
    let mut matched : usize = 0i32 as (usize);
    let mut limit2
        : usize
        = (limit >> 3i32).wrapping_add(1i32 as (usize));
    while {
              limit2 = limit2.wrapping_sub(1 as (usize));
              limit2
          } != 0 {
        if BROTLI_UNALIGNED_LOAD64(
               s2 as (*const std::os::raw::c_void)
           ) == BROTLI_UNALIGNED_LOAD64(
                    s1.offset(matched as (isize)) as (*const std::os::raw::c_void)
                ) {
            s2 = s2.offset(8i32 as (isize));
            matched = matched.wrapping_add(8i32 as (usize));
        } else {
            let mut x
                : usize
                = BROTLI_UNALIGNED_LOAD64(
                      s2 as (*const std::os::raw::c_void)
                  ) ^ BROTLI_UNALIGNED_LOAD64(
                          s1.offset(matched as (isize)) as (*const std::os::raw::c_void)
                      );
            let mut matching_bits : usize = unopt_ctzll(x) as (usize);
            matched = matched.wrapping_add(matching_bits >> 3i32);
            return matched;
        }
    }
    limit = (limit & 7i32 as (usize)).wrapping_add(1i32 as (usize));
    while {
              limit = limit.wrapping_sub(1 as (usize));
              limit
          } != 0 {
        if *s1.offset(matched as (isize)) as (i32) == *s2 as (i32) {
            s2 = s2.offset(1 as (isize));
            matched = matched.wrapping_add(1 as (usize));
        } else {
            return matched;
        }
    }
    matched
}

unsafe extern fn EmitInsertLen(
    mut insertlen : usize,
    mut depth : *const u8,
    mut bits : *const u16,
    mut histo : *mut u32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) { if insertlen < 6i32 as (usize) {
        let code : usize = insertlen.wrapping_add(40i32 as (usize));
        BrotliWriteBits(
            *depth.offset(code as (isize)) as (usize),
            *bits.offset(code as (isize)) as (usize),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(code as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    } else if insertlen < 130i32 as (usize) {
        let tail : usize = insertlen.wrapping_sub(2i32 as (usize));
        let nbits : u32 = Log2FloorNonZero(tail).wrapping_sub(1u32);
        let prefix : usize = tail >> nbits;
        let inscode
            : usize
            = ((nbits << 1i32) as (usize)).wrapping_add(prefix).wrapping_add(
                  42i32 as (usize)
              );
        BrotliWriteBits(
            *depth.offset(inscode as (isize)) as (usize),
            *bits.offset(inscode as (isize)) as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            nbits as (usize),
            tail.wrapping_sub(prefix << nbits),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(inscode as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    } else if insertlen < 2114i32 as (usize) {
        let tail : usize = insertlen.wrapping_sub(66i32 as (usize));
        let nbits : u32 = Log2FloorNonZero(tail);
        let code : usize = nbits.wrapping_add(50i32 as (u32)) as (usize);
        BrotliWriteBits(
            *depth.offset(code as (isize)) as (usize),
            *bits.offset(code as (isize)) as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            nbits as (usize),
            tail.wrapping_sub(1i32 as (usize) << nbits),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(code as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    } else {
        BrotliWriteBits(
            *depth.offset(61i32 as (isize)) as (usize),
            *bits.offset(61i32 as (isize)) as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            12i32 as (usize),
            insertlen.wrapping_sub(2114i32 as (usize)),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(21i32 as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    }
}

unsafe extern fn ShouldUseUncompressedMode(
    mut metablock_start : *const u8,
    mut next_emit : *const u8,
    insertlen : usize,
    literal_ratio : usize
) -> i32 {
    let compressed
        : usize
        = ((next_emit as (isize)).wrapping_sub(
               metablock_start as (isize)
           ) / std::mem::size_of::<*const u8>() as (isize)) as (usize);
    if compressed.wrapping_mul(50i32 as (usize)) > insertlen {
        0i32
    } else if !!(literal_ratio > 980i32 as (usize)) {
        1i32
    } else {
        0i32
    }
}

unsafe extern fn RewindBitPosition(
    new_storage_ix : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let bitpos : usize = new_storage_ix & 7i32 as (usize);
    let mask
        : usize
        = (1u32 << bitpos).wrapping_sub(1i32 as (u32)) as (usize);
    {
        let _rhs = mask as (u8);
        let _lhs
            = &mut *storage.offset((new_storage_ix >> 3i32) as (isize));
        *_lhs = (*_lhs as (i32) & _rhs as (i32)) as (u8);
    }
    *storage_ix = new_storage_ix;
}

unsafe extern fn EmitUncompressedMetaBlock(
    mut begin : *const u8,
    mut end : *const u8,
    storage_ix_start : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let len
        : usize
        = ((end as (isize)).wrapping_sub(
               begin as (isize)
           ) / std::mem::size_of::<*const u8>() as (isize)) as (usize);
    RewindBitPosition(storage_ix_start,storage_ix,storage);
    BrotliStoreMetaBlockHeader(len,1i32,storage_ix,storage);
    *storage_ix = (*storage_ix).wrapping_add(
                      7u32 as (usize)
                  ) & !7u32 as (usize);
    memcpy(
        &mut *storage.offset(
                  (*storage_ix >> 3i32) as (isize)
              ) as (*mut u8) as (*mut std::os::raw::c_void),
        begin as (*const std::os::raw::c_void),
        len
    );
    *storage_ix = (*storage_ix).wrapping_add(len << 3i32);
    *storage.offset((*storage_ix >> 3i32) as (isize)) = 0i32 as (u8);
}

unsafe extern fn EmitLongInsertLen(
    mut insertlen : usize,
    mut depth : *const u8,
    mut bits : *const u16,
    mut histo : *mut u32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) { if insertlen < 22594i32 as (usize) {
        BrotliWriteBits(
            *depth.offset(62i32 as (isize)) as (usize),
            *bits.offset(62i32 as (isize)) as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            14i32 as (usize),
            insertlen.wrapping_sub(6210i32 as (usize)),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(22i32 as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    } else {
        BrotliWriteBits(
            *depth.offset(63i32 as (isize)) as (usize),
            *bits.offset(63i32 as (isize)) as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            24i32 as (usize),
            insertlen.wrapping_sub(22594i32 as (usize)),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(23i32 as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    }
}

unsafe extern fn EmitLiterals(
    mut input : *const u8,
    len : usize,
    mut depth : *const u8,
    mut bits : *const u16,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut j : usize;
    j = 0i32 as (usize);
    while j < len {
        {
            let lit : u8 = *input.offset(j as (isize));
            BrotliWriteBits(
                *depth.offset(lit as (isize)) as (usize),
                *bits.offset(lit as (isize)) as (usize),
                storage_ix,
                storage
            );
        }
        j = j.wrapping_add(1 as (usize));
    }
}

unsafe extern fn EmitDistance(
    mut distance : usize,
    mut depth : *const u8,
    mut bits : *const u16,
    mut histo : *mut u32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let d : usize = distance.wrapping_add(3i32 as (usize));
    let nbits : u32 = Log2FloorNonZero(d).wrapping_sub(1u32);
    let prefix : usize = d >> nbits & 1i32 as (usize);
    let offset
        : usize
        = (2i32 as (usize)).wrapping_add(prefix) << nbits;
    let distcode
        : usize
        = ((2i32 as (u32)).wrapping_mul(
               nbits.wrapping_sub(1i32 as (u32))
           ) as (usize)).wrapping_add(
              prefix
          ).wrapping_add(
              80i32 as (usize)
          );
    BrotliWriteBits(
        *depth.offset(distcode as (isize)) as (usize),
        *bits.offset(distcode as (isize)) as (usize),
        storage_ix,
        storage
    );
    BrotliWriteBits(
        nbits as (usize),
        d.wrapping_sub(offset),
        storage_ix,
        storage
    );
    {
        let _rhs = 1;
        let _lhs = &mut *histo.offset(distcode as (isize));
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
}

unsafe extern fn EmitCopyLenLastDistance(
    mut copylen : usize,
    mut depth : *const u8,
    mut bits : *const u16,
    mut histo : *mut u32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) { if copylen < 12i32 as (usize) {
        BrotliWriteBits(
            *depth.offset(
                 copylen.wrapping_sub(4i32 as (usize)) as (isize)
             ) as (usize),
            *bits.offset(
                 copylen.wrapping_sub(4i32 as (usize)) as (isize)
             ) as (usize),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs
                = &mut *histo.offset(
                            copylen.wrapping_sub(4i32 as (usize)) as (isize)
                        );
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    } else if copylen < 72i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(8i32 as (usize));
        let nbits
            : u32
            = Log2FloorNonZero(tail).wrapping_sub(1i32 as (u32));
        let prefix : usize = tail >> nbits;
        let code
            : usize
            = ((nbits << 1i32) as (usize)).wrapping_add(prefix).wrapping_add(
                  4i32 as (usize)
              );
        BrotliWriteBits(
            *depth.offset(code as (isize)) as (usize),
            *bits.offset(code as (isize)) as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            nbits as (usize),
            tail.wrapping_sub(prefix << nbits),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(code as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    } else if copylen < 136i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(8i32 as (usize));
        let code : usize = (tail >> 5i32).wrapping_add(30i32 as (usize));
        BrotliWriteBits(
            *depth.offset(code as (isize)) as (usize),
            *bits.offset(code as (isize)) as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            5i32 as (usize),
            tail & 31i32 as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            *depth.offset(64i32 as (isize)) as (usize),
            *bits.offset(64i32 as (isize)) as (usize),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(code as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(64i32 as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    } else if copylen < 2120i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(72i32 as (usize));
        let nbits : u32 = Log2FloorNonZero(tail);
        let code : usize = nbits.wrapping_add(28i32 as (u32)) as (usize);
        BrotliWriteBits(
            *depth.offset(code as (isize)) as (usize),
            *bits.offset(code as (isize)) as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            nbits as (usize),
            tail.wrapping_sub(1i32 as (usize) << nbits),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            *depth.offset(64i32 as (isize)) as (usize),
            *bits.offset(64i32 as (isize)) as (usize),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(code as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(64i32 as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    } else {
        BrotliWriteBits(
            *depth.offset(39i32 as (isize)) as (usize),
            *bits.offset(39i32 as (isize)) as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            24i32 as (usize),
            copylen.wrapping_sub(2120i32 as (usize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            *depth.offset(64i32 as (isize)) as (usize),
            *bits.offset(64i32 as (isize)) as (usize),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(47i32 as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(64i32 as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    }
}

unsafe extern fn HashBytesAtOffset(
    mut v : usize, mut offset : i32, mut shift : usize
) -> u32 {
    let h
        : usize
        = (v >> 8i32 * offset << 24i32).wrapping_mul(
              kHashMul32 as (usize)
          );
    (h >> shift) as (u32)
}

unsafe extern fn EmitCopyLen(
    mut copylen : usize,
    mut depth : *const u8,
    mut bits : *const u16,
    mut histo : *mut u32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) { if copylen < 10i32 as (usize) {
        BrotliWriteBits(
            *depth.offset(
                 copylen.wrapping_add(14i32 as (usize)) as (isize)
             ) as (usize),
            *bits.offset(
                 copylen.wrapping_add(14i32 as (usize)) as (isize)
             ) as (usize),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs
                = &mut *histo.offset(
                            copylen.wrapping_add(14i32 as (usize)) as (isize)
                        );
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    } else if copylen < 134i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(6i32 as (usize));
        let nbits : u32 = Log2FloorNonZero(tail).wrapping_sub(1u32);
        let prefix : usize = tail >> nbits;
        let code
            : usize
            = ((nbits << 1i32) as (usize)).wrapping_add(prefix).wrapping_add(
                  20i32 as (usize)
              );
        BrotliWriteBits(
            *depth.offset(code as (isize)) as (usize),
            *bits.offset(code as (isize)) as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            nbits as (usize),
            tail.wrapping_sub(prefix << nbits),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(code as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    } else if copylen < 2118i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(70i32 as (usize));
        let nbits : u32 = Log2FloorNonZero(tail);
        let code : usize = nbits.wrapping_add(28i32 as (u32)) as (usize);
        BrotliWriteBits(
            *depth.offset(code as (isize)) as (usize),
            *bits.offset(code as (isize)) as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            nbits as (usize),
            tail.wrapping_sub(1i32 as (usize) << nbits),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(code as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    } else {
        BrotliWriteBits(
            *depth.offset(39i32 as (isize)) as (usize),
            *bits.offset(39i32 as (isize)) as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            24i32 as (usize),
            copylen.wrapping_sub(2118i32 as (usize)),
            storage_ix,
            storage
        );
        {
            let _rhs = 1;
            let _lhs = &mut *histo.offset(47i32 as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
    }
}

unsafe extern fn FastLog2(mut v : usize) -> f64 {
    if v < std::mem::size_of::<*const f32>().wrapping_div(
               std::mem::size_of::<f32>()
           ) {
        return *kLog2Table.offset(v as (isize)) as (f64);
    }
    log2(v as (f64))
}

unsafe extern fn ShouldMergeBlock(
    mut data : *const u8, mut len : usize, mut depths : *const u8
) -> i32 {
    let mut histo : *mut usize = 0i32 as (*mut usize);
    static kSampleRate : usize = 43i32 as (usize);
    let mut i : usize;
    i = 0i32 as (usize);
    while i < len {
        {
            let _rhs = 1;
            let _lhs
                = &mut *histo.offset(*data.offset(i as (isize)) as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
        }
        i = i.wrapping_add(kSampleRate);
    }
    {
        let total
            : usize
            = len.wrapping_add(kSampleRate).wrapping_sub(
                  1i32 as (usize)
              ).wrapping_div(
                  kSampleRate
              );
        let mut r
            : f64
            = (FastLog2(total) + 0.5f64) * total as (f64) + 200i32 as (f64);
        i = 0i32 as (usize);
        while i < 256i32 as (usize) {
            {
                r = r - *histo.offset(i as (isize)) as (f64) * (*depths.offset(
                                                                     i as (isize)
                                                                 ) as (f64) + FastLog2(
                                                                                  *histo.offset(
                                                                                       i as (isize)
                                                                                   )
                                                                              ));
            }
            i = i.wrapping_add(1 as (usize));
        }
        if !!(r >= 0.0f64) { 1i32 } else { 0i32 }
    }
}

unsafe extern fn UpdateBits(
    mut n_bits : usize,
    mut bits : u32,
    mut pos : usize,
    mut array : *mut u8
) {
    while n_bits > 0i32 as (usize) {
        let mut byte_pos : usize = pos >> 3i32;
        let mut n_unchanged_bits : usize = pos & 7i32 as (usize);
        let mut n_changed_bits
            : usize
            = brotli_min_size_t(
                  n_bits,
                  (8i32 as (usize)).wrapping_sub(n_unchanged_bits)
              );
        let mut total_bits
            : usize
            = n_unchanged_bits.wrapping_add(n_changed_bits);
        let mut mask
            : u32
            = !(1u32 << total_bits).wrapping_sub(
                   1u32
               ) | (1u32 << n_unchanged_bits).wrapping_sub(1u32);
        let mut unchanged_bits
            : u32
            = *array.offset(byte_pos as (isize)) as (u32) & mask;
        let mut changed_bits
            : u32
            = bits & (1u32 << n_changed_bits).wrapping_sub(1u32);
        *array.offset(
             byte_pos as (isize)
         ) = (changed_bits << n_unchanged_bits | unchanged_bits) as (u8);
        n_bits = n_bits.wrapping_sub(n_changed_bits);
        bits = bits >> n_changed_bits;
        pos = pos.wrapping_add(n_changed_bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HuffmanTree {
    pub total_count_ : u32,
    pub index_left_ : i16,
    pub index_right_or_value_ : i16,
}

unsafe extern fn BuildAndStoreCommandPrefixCode(
    mut histogram : *const u32,
    mut depth : *mut u8,
    mut bits : *mut u16,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut tree : *mut HuffmanTree;
    let mut cmd_depth : *mut u8 = 0i32 as (*mut u8);
    let mut cmd_bits : *mut u16;
    BrotliCreateHuffmanTree(
        histogram,
        64i32 as (usize),
        15i32,
        tree,
        depth
    );
    BrotliCreateHuffmanTree(
        &*histogram.offset(64i32 as (isize)) as (*const u32),
        64i32 as (usize),
        14i32,
        tree,
        &mut *depth.offset(64i32 as (isize)) as (*mut u8)
    );
    memcpy(
        cmd_depth as (*mut std::os::raw::c_void),
        depth as (*const std::os::raw::c_void),
        24i32 as (usize)
    );
    memcpy(
        cmd_depth.offset(24i32 as (isize)) as (*mut std::os::raw::c_void),
        depth.offset(40i32 as (isize)) as (*const std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.offset(32i32 as (isize)) as (*mut std::os::raw::c_void),
        depth.offset(24i32 as (isize)) as (*const std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.offset(40i32 as (isize)) as (*mut std::os::raw::c_void),
        depth.offset(48i32 as (isize)) as (*const std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.offset(48i32 as (isize)) as (*mut std::os::raw::c_void),
        depth.offset(32i32 as (isize)) as (*const std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.offset(56i32 as (isize)) as (*mut std::os::raw::c_void),
        depth.offset(56i32 as (isize)) as (*const std::os::raw::c_void),
        8i32 as (usize)
    );
    BrotliConvertBitDepthsToSymbols(
        cmd_depth as (*const u8),
        64i32 as (usize),
        cmd_bits
    );
    memcpy(
        bits as (*mut std::os::raw::c_void),
        cmd_bits as (*const std::os::raw::c_void),
        48i32 as (usize)
    );
    memcpy(
        bits.offset(24i32 as (isize)) as (*mut std::os::raw::c_void),
        cmd_bits.offset(32i32 as (isize)) as (*const std::os::raw::c_void),
        16i32 as (usize)
    );
    memcpy(
        bits.offset(32i32 as (isize)) as (*mut std::os::raw::c_void),
        cmd_bits.offset(48i32 as (isize)) as (*const std::os::raw::c_void),
        16i32 as (usize)
    );
    memcpy(
        bits.offset(40i32 as (isize)) as (*mut std::os::raw::c_void),
        cmd_bits.offset(24i32 as (isize)) as (*const std::os::raw::c_void),
        16i32 as (usize)
    );
    memcpy(
        bits.offset(48i32 as (isize)) as (*mut std::os::raw::c_void),
        cmd_bits.offset(40i32 as (isize)) as (*const std::os::raw::c_void),
        16i32 as (usize)
    );
    memcpy(
        bits.offset(56i32 as (isize)) as (*mut std::os::raw::c_void),
        cmd_bits.offset(56i32 as (isize)) as (*const std::os::raw::c_void),
        16i32 as (usize)
    );
    BrotliConvertBitDepthsToSymbols(
        &mut *depth.offset(64i32 as (isize)) as (*mut u8) as (*const u8),
        64i32 as (usize),
        &mut *bits.offset(64i32 as (isize)) as (*mut u16)
    );
    {
        let mut i : usize;
        memset(
            cmd_depth as (*mut std::os::raw::c_void),
            0i32,
            64i32 as (usize)
        );
        memcpy(
            cmd_depth as (*mut std::os::raw::c_void),
            depth as (*const std::os::raw::c_void),
            8i32 as (usize)
        );
        memcpy(
            cmd_depth.offset(64i32 as (isize)) as (*mut std::os::raw::c_void),
            depth.offset(8i32 as (isize)) as (*const std::os::raw::c_void),
            8i32 as (usize)
        );
        memcpy(
            cmd_depth.offset(128i32 as (isize)) as (*mut std::os::raw::c_void),
            depth.offset(16i32 as (isize)) as (*const std::os::raw::c_void),
            8i32 as (usize)
        );
        memcpy(
            cmd_depth.offset(192i32 as (isize)) as (*mut std::os::raw::c_void),
            depth.offset(24i32 as (isize)) as (*const std::os::raw::c_void),
            8i32 as (usize)
        );
        memcpy(
            cmd_depth.offset(384i32 as (isize)) as (*mut std::os::raw::c_void),
            depth.offset(32i32 as (isize)) as (*const std::os::raw::c_void),
            8i32 as (usize)
        );
        i = 0i32 as (usize);
        while i < 8i32 as (usize) {
            {
                *cmd_depth.offset(
                     (128i32 as (usize)).wrapping_add(
                         (8i32 as (usize)).wrapping_mul(i)
                     ) as (isize)
                 ) = *depth.offset((40i32 as (usize)).wrapping_add(i) as (isize));
                *cmd_depth.offset(
                     (256i32 as (usize)).wrapping_add(
                         (8i32 as (usize)).wrapping_mul(i)
                     ) as (isize)
                 ) = *depth.offset((48i32 as (usize)).wrapping_add(i) as (isize));
                *cmd_depth.offset(
                     (448i32 as (usize)).wrapping_add(
                         (8i32 as (usize)).wrapping_mul(i)
                     ) as (isize)
                 ) = *depth.offset((56i32 as (usize)).wrapping_add(i) as (isize));
            }
            i = i.wrapping_add(1 as (usize));
        }
        BrotliStoreHuffmanTree(
            cmd_depth as (*const u8),
            704i32 as (usize),
            tree,
            storage_ix,
            storage
        );
    }
    BrotliStoreHuffmanTree(
        &mut *depth.offset(64i32 as (isize)) as (*mut u8) as (*const u8),
        64i32 as (usize),
        tree,
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentFastImpl(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut table : *mut i32,
    mut table_bits : usize,
    mut cmd_depth : *mut u8,
    mut cmd_bits : *mut u16,
    mut cmd_code_numbits : *mut usize,
    mut cmd_code : *mut u8,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut cmd_histo : *mut u32;
    let mut ip_end : *const u8;
    let mut next_emit : *const u8 = input;
    let mut base_ip : *const u8 = input;
    static kFirstBlockSize : usize = (3i32 << 15i32) as (usize);
    static kMergeBlockSize : usize = (1i32 << 16i32) as (usize);
    let kInputMarginBytes : usize = 16i32 as (usize);
    let kMinMatchLen : usize = 5i32 as (usize);
    let mut metablock_start : *const u8 = input;
    let mut block_size
        : usize
        = brotli_min_size_t(input_size,kFirstBlockSize);
    let mut total_block_size : usize = block_size;
    let mut mlen_storage_ix
        : usize
        = (*storage_ix).wrapping_add(3i32 as (usize));
    let mut lit_depth : *mut u8;
    let mut lit_bits : *mut u16;
    let mut literal_ratio : usize;
    let mut ip : *const u8;
    let mut last_distance : i32;
    let shift : usize = (64u32 as (usize)).wrapping_sub(table_bits);
    BrotliStoreMetaBlockHeader(block_size,0i32,storage_ix,storage);
    BrotliWriteBits(
        13i32 as (usize),
        0i32 as (usize),
        storage_ix,
        storage
    );
    literal_ratio = BuildAndStoreLiteralPrefixCode(
                        m,
                        input,
                        block_size,
                        lit_depth,
                        lit_bits,
                        storage_ix,
                        storage
                    );
    if !(0i32 == 0) {
        return;
    }
    {
        let mut i : usize;
        i = 0i32 as (usize);
        while i.wrapping_add(7i32 as (usize)) < *cmd_code_numbits {
            {
                BrotliWriteBits(
                    8i32 as (usize),
                    *cmd_code.offset((i >> 3i32) as (isize)) as (usize),
                    storage_ix,
                    storage
                );
            }
            i = i.wrapping_add(8i32 as (usize));
        }
    }
    BrotliWriteBits(
        *cmd_code_numbits & 7i32 as (usize),
        *cmd_code.offset(
             (*cmd_code_numbits >> 3i32) as (isize)
         ) as (usize),
        storage_ix,
        storage
    );
    let mut code_block_selection
        : CodeBlockState
        = CodeBlockState::EMIT_COMMANDS;
    while 1i32 != 0 {
        if code_block_selection as (i32) == CodeBlockState::EMIT_COMMANDS as (i32) {
            memcpy(
                cmd_histo as (*mut std::os::raw::c_void),
                kCmdHistoSeed as (*const std::os::raw::c_void),
                std::mem::size_of::<*mut u32>()
            );
            ip = input;
            last_distance = -1i32;
            ip_end = input.offset(block_size as (isize));
            if block_size >= kInputMarginBytes {
                let len_limit
                    : usize
                    = brotli_min_size_t(
                          block_size.wrapping_sub(kMinMatchLen),
                          input_size.wrapping_sub(kInputMarginBytes)
                      );
                let mut ip_limit : *const u8 = input.offset(len_limit as (isize));
                let mut next_hash : u32;
                next_hash = Hash(
                                {
                                    ip = ip.offset(1 as (isize));
                                    ip
                                },
                                shift
                            );
                loop {
                    let mut skip : u32 = 32i32 as (u32);
                    let mut next_ip : *const u8 = ip;
                    let mut candidate : *const u8;
                    loop {
                        {
                            'break15: loop {
                                {
                                    let mut hash : u32 = next_hash;
                                    let mut bytes_between_hash_lookups
                                        : u32
                                        = ({
                                               let _old = skip;
                                               skip = skip.wrapping_add(1 as (u32));
                                               _old
                                           }) >> 5i32;
                                    ip = next_ip;
                                    next_ip = ip.offset(bytes_between_hash_lookups as (isize));
                                    if next_ip > ip_limit {
                                        code_block_selection = CodeBlockState::EMIT_REMAINDER;
                                        {
                                            if 1337i32 != 0 {
                                                break 'break15;
                                            }
                                        }
                                    }
                                    next_hash = Hash(next_ip,shift);
                                    candidate = ip.offset(-(last_distance as (isize)));
                                    if IsMatch(ip,candidate) != 0 {
                                        if candidate < ip {
                                            *table.offset(
                                                 hash as (isize)
                                             ) = ((ip as (isize)).wrapping_sub(
                                                      base_ip as (isize)
                                                  ) / std::mem::size_of::<*const u8>(
                                                      ) as (isize)) as (i32);
                                            {
                                                if 1337i32 != 0 {
                                                    break 'break15;
                                                }
                                            }
                                        }
                                    }
                                    candidate = base_ip.offset(
                                                    *table.offset(hash as (isize)) as (isize)
                                                );
                                    *table.offset(hash as (isize)) = ((ip as (isize)).wrapping_sub(
                                                                          base_ip as (isize)
                                                                      ) / std::mem::size_of::<*const u8>(
                                                                          ) as (isize)) as (i32);
                                }
                                if !(IsMatch(ip,candidate) == 0) {
                                    break;
                                }
                            }
                        }
                        if !((ip as (isize)).wrapping_sub(
                                 candidate as (isize)
                             ) / std::mem::size_of::<*const u8>(
                                 ) as (isize) > (1i32 as (usize) << 18i32).wrapping_sub(
                                                    16i32 as (usize)
                                                ) as (isize) && (code_block_selection as (i32) == CodeBlockState::EMIT_COMMANDS as (i32))) {
                            break;
                        }
                    }
                    if code_block_selection as (i32) != CodeBlockState::EMIT_COMMANDS as (i32) {
                        if 1337i32 != 0 {
                            break;
                        }
                    }
                    {
                        let mut base : *const u8 = ip;
                        let mut matched
                            : usize
                            = (5i32 as (usize)).wrapping_add(
                                  FindMatchLengthWithLimit(
                                      candidate.offset(5i32 as (isize)),
                                      ip.offset(5i32 as (isize)),
                                      (((ip_end as (isize)).wrapping_sub(
                                            ip as (isize)
                                        ) / std::mem::size_of::<*const u8>(
                                            ) as (isize)) as (usize)).wrapping_sub(
                                          5i32 as (usize)
                                      )
                                  )
                              );
                        let mut distance
                            : i32
                            = ((base as (isize)).wrapping_sub(
                                   candidate as (isize)
                               ) / std::mem::size_of::<*const u8>() as (isize)) as (i32);
                        let mut insert
                            : usize
                            = ((base as (isize)).wrapping_sub(
                                   next_emit as (isize)
                               ) / std::mem::size_of::<*const u8>() as (isize)) as (usize);
                        ip = ip.offset(matched as (isize));
                        if insert < 6210i32 as (usize) {
                            EmitInsertLen(
                                insert,
                                cmd_depth as (*const u8),
                                cmd_bits as (*const u16),
                                cmd_histo,
                                storage_ix,
                                storage
                            );
                        } else if ShouldUseUncompressedMode(
                                      metablock_start,
                                      next_emit,
                                      insert,
                                      literal_ratio
                                  ) != 0 {
                            EmitUncompressedMetaBlock(
                                metablock_start,
                                base,
                                mlen_storage_ix.wrapping_sub(3i32 as (usize)),
                                storage_ix,
                                storage
                            );
                            input_size = input_size.wrapping_sub(
                                             ((base as (isize)).wrapping_sub(
                                                  input as (isize)
                                              ) / std::mem::size_of::<*const u8>(
                                                  ) as (isize)) as (usize)
                                         );
                            input = base;
                            next_emit = input;
                            code_block_selection = CodeBlockState::NEXT_BLOCK;
                            {
                                if 1337i32 != 0 {
                                    break;
                                }
                            }
                        } else {
                            EmitLongInsertLen(
                                insert,
                                cmd_depth as (*const u8),
                                cmd_bits as (*const u16),
                                cmd_histo,
                                storage_ix,
                                storage
                            );
                        }
                        EmitLiterals(
                            next_emit,
                            insert,
                            lit_depth as (*const u8),
                            lit_bits as (*const u16),
                            storage_ix,
                            storage
                        );
                        if distance == last_distance {
                            BrotliWriteBits(
                                *cmd_depth.offset(64i32 as (isize)) as (usize),
                                *cmd_bits.offset(64i32 as (isize)) as (usize),
                                storage_ix,
                                storage
                            );
                            {
                                let _rhs = 1;
                                let _lhs = &mut *cmd_histo.offset(64i32 as (isize));
                                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                            }
                        } else {
                            EmitDistance(
                                distance as (usize),
                                cmd_depth as (*const u8),
                                cmd_bits as (*const u16),
                                cmd_histo,
                                storage_ix,
                                storage
                            );
                            last_distance = distance;
                        }
                        EmitCopyLenLastDistance(
                            matched,
                            cmd_depth as (*const u8),
                            cmd_bits as (*const u16),
                            cmd_histo,
                            storage_ix,
                            storage
                        );
                        next_emit = ip;
                        if ip >= ip_limit {
                            code_block_selection = CodeBlockState::EMIT_REMAINDER;
                            {
                                if 1337i32 != 0 {
                                    break;
                                }
                            }
                        }
                        {
                            let mut input_bytes
                                : usize
                                = BROTLI_UNALIGNED_LOAD64(
                                      ip.offset(-(3i32 as (isize))) as (*const std::os::raw::c_void)
                                  );
                            let mut prev_hash
                                : u32
                                = HashBytesAtOffset(input_bytes,0i32,shift);
                            let mut cur_hash : u32 = HashBytesAtOffset(input_bytes,3i32,shift);
                            *table.offset(
                                 prev_hash as (isize)
                             ) = ((ip as (isize)).wrapping_sub(
                                      base_ip as (isize)
                                  ) / std::mem::size_of::<*const u8>(
                                      ) as (isize) - 3i32 as (isize)) as (i32);
                            prev_hash = HashBytesAtOffset(input_bytes,1i32,shift);
                            *table.offset(
                                 prev_hash as (isize)
                             ) = ((ip as (isize)).wrapping_sub(
                                      base_ip as (isize)
                                  ) / std::mem::size_of::<*const u8>(
                                      ) as (isize) - 2i32 as (isize)) as (i32);
                            prev_hash = HashBytesAtOffset(input_bytes,2i32,shift);
                            *table.offset(
                                 prev_hash as (isize)
                             ) = ((ip as (isize)).wrapping_sub(
                                      base_ip as (isize)
                                  ) / std::mem::size_of::<*const u8>(
                                      ) as (isize) - 1i32 as (isize)) as (i32);
                            candidate = base_ip.offset(
                                            *table.offset(cur_hash as (isize)) as (isize)
                                        );
                            *table.offset(cur_hash as (isize)) = ((ip as (isize)).wrapping_sub(
                                                                      base_ip as (isize)
                                                                  ) / std::mem::size_of::<*const u8>(
                                                                      ) as (isize)) as (i32);
                        }
                    }
                    while IsMatch(ip,candidate) != 0 {
                        let mut base : *const u8 = ip;
                        let mut matched
                            : usize
                            = (5i32 as (usize)).wrapping_add(
                                  FindMatchLengthWithLimit(
                                      candidate.offset(5i32 as (isize)),
                                      ip.offset(5i32 as (isize)),
                                      (((ip_end as (isize)).wrapping_sub(
                                            ip as (isize)
                                        ) / std::mem::size_of::<*const u8>(
                                            ) as (isize)) as (usize)).wrapping_sub(
                                          5i32 as (usize)
                                      )
                                  )
                              );
                        if (ip as (isize)).wrapping_sub(
                               candidate as (isize)
                           ) / std::mem::size_of::<*const u8>(
                               ) as (isize) > (1i32 as (usize) << 18i32).wrapping_sub(
                                                  16i32 as (usize)
                                              ) as (isize) {
                            if 1337i32 != 0 {
                                break;
                            }
                        }
                        ip = ip.offset(matched as (isize));
                        last_distance = ((base as (isize)).wrapping_sub(
                                             candidate as (isize)
                                         ) / std::mem::size_of::<*const u8>() as (isize)) as (i32);
                        EmitCopyLen(
                            matched,
                            cmd_depth as (*const u8),
                            cmd_bits as (*const u16),
                            cmd_histo,
                            storage_ix,
                            storage
                        );
                        EmitDistance(
                            last_distance as (usize),
                            cmd_depth as (*const u8),
                            cmd_bits as (*const u16),
                            cmd_histo,
                            storage_ix,
                            storage
                        );
                        next_emit = ip;
                        if ip >= ip_limit {
                            code_block_selection = CodeBlockState::EMIT_REMAINDER;
                            {
                                if 1337i32 != 0 {
                                    break;
                                }
                            }
                        }
                        {
                            let mut input_bytes
                                : usize
                                = BROTLI_UNALIGNED_LOAD64(
                                      ip.offset(-(3i32 as (isize))) as (*const std::os::raw::c_void)
                                  );
                            let mut prev_hash
                                : u32
                                = HashBytesAtOffset(input_bytes,0i32,shift);
                            let mut cur_hash : u32 = HashBytesAtOffset(input_bytes,3i32,shift);
                            *table.offset(
                                 prev_hash as (isize)
                             ) = ((ip as (isize)).wrapping_sub(
                                      base_ip as (isize)
                                  ) / std::mem::size_of::<*const u8>(
                                      ) as (isize) - 3i32 as (isize)) as (i32);
                            prev_hash = HashBytesAtOffset(input_bytes,1i32,shift);
                            *table.offset(
                                 prev_hash as (isize)
                             ) = ((ip as (isize)).wrapping_sub(
                                      base_ip as (isize)
                                  ) / std::mem::size_of::<*const u8>(
                                      ) as (isize) - 2i32 as (isize)) as (i32);
                            prev_hash = HashBytesAtOffset(input_bytes,2i32,shift);
                            *table.offset(
                                 prev_hash as (isize)
                             ) = ((ip as (isize)).wrapping_sub(
                                      base_ip as (isize)
                                  ) / std::mem::size_of::<*const u8>(
                                      ) as (isize) - 1i32 as (isize)) as (i32);
                            candidate = base_ip.offset(
                                            *table.offset(cur_hash as (isize)) as (isize)
                                        );
                            *table.offset(cur_hash as (isize)) = ((ip as (isize)).wrapping_sub(
                                                                      base_ip as (isize)
                                                                  ) / std::mem::size_of::<*const u8>(
                                                                      ) as (isize)) as (i32);
                        }
                    }
                    if code_block_selection as (i32) == CodeBlockState::EMIT_COMMANDS as (i32) {
                        next_hash = Hash(
                                        {
                                            ip = ip.offset(1 as (isize));
                                            ip
                                        },
                                        shift
                                    );
                    }
                }
            }
            code_block_selection = CodeBlockState::EMIT_REMAINDER;
            {
                if 1337i32 != 0 {
                    continue;
                }
            }
        } else if code_block_selection as (i32) == CodeBlockState::EMIT_REMAINDER as (i32) {
            input = input.offset(block_size as (isize));
            input_size = input_size.wrapping_sub(block_size);
            block_size = brotli_min_size_t(input_size,kMergeBlockSize);
            if input_size > 0i32 as (usize) && (total_block_size.wrapping_add(
                                                    block_size
                                                ) <= (1i32 << 20i32) as (usize)) && (ShouldMergeBlock(
                                                                                         input,
                                                                                         block_size,
                                                                                         lit_depth as (*const u8)
                                                                                     ) != 0) {
                total_block_size = total_block_size.wrapping_add(block_size);
                UpdateBits(
                    20i32 as (usize),
                    total_block_size.wrapping_sub(1i32 as (usize)) as (u32),
                    mlen_storage_ix,
                    storage
                );
                code_block_selection = CodeBlockState::EMIT_COMMANDS;
                {
                    if 1337i32 != 0 {
                        continue;
                    }
                }
            }
            if next_emit < ip_end {
                let insert
                    : usize
                    = ((ip_end as (isize)).wrapping_sub(
                           next_emit as (isize)
                       ) / std::mem::size_of::<*const u8>() as (isize)) as (usize);
                if insert < 6210i32 as (usize) {
                    EmitInsertLen(
                        insert,
                        cmd_depth as (*const u8),
                        cmd_bits as (*const u16),
                        cmd_histo,
                        storage_ix,
                        storage
                    );
                    EmitLiterals(
                        next_emit,
                        insert,
                        lit_depth as (*const u8),
                        lit_bits as (*const u16),
                        storage_ix,
                        storage
                    );
                } else if ShouldUseUncompressedMode(
                              metablock_start,
                              next_emit,
                              insert,
                              literal_ratio
                          ) != 0 {
                    EmitUncompressedMetaBlock(
                        metablock_start,
                        ip_end,
                        mlen_storage_ix.wrapping_sub(3i32 as (usize)),
                        storage_ix,
                        storage
                    );
                } else {
                    EmitLongInsertLen(
                        insert,
                        cmd_depth as (*const u8),
                        cmd_bits as (*const u16),
                        cmd_histo,
                        storage_ix,
                        storage
                    );
                    EmitLiterals(
                        next_emit,
                        insert,
                        lit_depth as (*const u8),
                        lit_bits as (*const u16),
                        storage_ix,
                        storage
                    );
                }
            }
            next_emit = ip_end;
            code_block_selection = CodeBlockState::NEXT_BLOCK;
        } else if code_block_selection as (i32) == CodeBlockState::NEXT_BLOCK as (i32) {
            if input_size > 0i32 as (usize) {
                metablock_start = input;
                block_size = brotli_min_size_t(input_size,kFirstBlockSize);
                total_block_size = block_size;
                mlen_storage_ix = (*storage_ix).wrapping_add(3i32 as (usize));
                BrotliStoreMetaBlockHeader(block_size,0i32,storage_ix,storage);
                BrotliWriteBits(
                    13i32 as (usize),
                    0i32 as (usize),
                    storage_ix,
                    storage
                );
                literal_ratio = BuildAndStoreLiteralPrefixCode(
                                    m,
                                    input,
                                    block_size,
                                    lit_depth,
                                    lit_bits,
                                    storage_ix,
                                    storage
                                );
                if !(0i32 == 0) {
                    return;
                }
                BuildAndStoreCommandPrefixCode(
                    cmd_histo as (*const u32),
                    cmd_depth,
                    cmd_bits,
                    storage_ix,
                    storage
                );
                code_block_selection = CodeBlockState::EMIT_COMMANDS;
                {
                    if 1337i32 != 0 {
                        continue;
                    }
                }
            }
            {
                if 1337i32 != 0 {
                    break;
                }
            }
        }
    }
    if is_last == 0 {
        *cmd_code.offset(0i32 as (isize)) = 0i32 as (u8);
        *cmd_code_numbits = 0i32 as (usize);
        BuildAndStoreCommandPrefixCode(
            cmd_histo as (*const u32),
            cmd_depth,
            cmd_bits,
            cmd_code_numbits,
            cmd_code
        );
    }
}

unsafe extern fn BrotliCompressFragmentFastImpl9(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut table : *mut i32,
    mut cmd_depth : *mut u8,
    mut cmd_bits : *mut u16,
    mut cmd_code_numbits : *mut usize,
    mut cmd_code : *mut u8,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentFastImpl(
        m,
        input,
        input_size,
        is_last,
        table,
        9i32 as (usize),
        cmd_depth,
        cmd_bits,
        cmd_code_numbits,
        cmd_code,
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentFastImpl11(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut table : *mut i32,
    mut cmd_depth : *mut u8,
    mut cmd_bits : *mut u16,
    mut cmd_code_numbits : *mut usize,
    mut cmd_code : *mut u8,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentFastImpl(
        m,
        input,
        input_size,
        is_last,
        table,
        11i32 as (usize),
        cmd_depth,
        cmd_bits,
        cmd_code_numbits,
        cmd_code,
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentFastImpl13(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut table : *mut i32,
    mut cmd_depth : *mut u8,
    mut cmd_bits : *mut u16,
    mut cmd_code_numbits : *mut usize,
    mut cmd_code : *mut u8,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentFastImpl(
        m,
        input,
        input_size,
        is_last,
        table,
        13i32 as (usize),
        cmd_depth,
        cmd_bits,
        cmd_code_numbits,
        cmd_code,
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentFastImpl15(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut table : *mut i32,
    mut cmd_depth : *mut u8,
    mut cmd_bits : *mut u16,
    mut cmd_code_numbits : *mut usize,
    mut cmd_code : *mut u8,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentFastImpl(
        m,
        input,
        input_size,
        is_last,
        table,
        15i32 as (usize),
        cmd_depth,
        cmd_bits,
        cmd_code_numbits,
        cmd_code,
        storage_ix,
        storage
    );
}

#[no_mangle]
pub unsafe extern fn BrotliCompressFragmentFast(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut table : *mut i32,
    mut table_size : usize,
    mut cmd_depth : *mut u8,
    mut cmd_bits : *mut u16,
    mut cmd_code_numbits : *mut usize,
    mut cmd_code : *mut u8,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let initial_storage_ix : usize = *storage_ix;
    let table_bits : usize = Log2FloorNonZero(table_size) as (usize);
    if input_size == 0i32 as (usize) {
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
        *storage_ix = (*storage_ix).wrapping_add(
                          7u32 as (usize)
                      ) & !7u32 as (usize);
        return;
    }
    if table_bits == 9i32 as (usize) {
        BrotliCompressFragmentFastImpl9(
            m,
            input,
            input_size,
            is_last,
            table,
            cmd_depth,
            cmd_bits,
            cmd_code_numbits,
            cmd_code,
            storage_ix,
            storage
        );
    }
    if table_bits == 11i32 as (usize) {
        BrotliCompressFragmentFastImpl11(
            m,
            input,
            input_size,
            is_last,
            table,
            cmd_depth,
            cmd_bits,
            cmd_code_numbits,
            cmd_code,
            storage_ix,
            storage
        );
    }
    if table_bits == 13i32 as (usize) {
        BrotliCompressFragmentFastImpl13(
            m,
            input,
            input_size,
            is_last,
            table,
            cmd_depth,
            cmd_bits,
            cmd_code_numbits,
            cmd_code,
            storage_ix,
            storage
        );
    }
    if table_bits == 15i32 as (usize) {
        BrotliCompressFragmentFastImpl15(
            m,
            input,
            input_size,
            is_last,
            table,
            cmd_depth,
            cmd_bits,
            cmd_code_numbits,
            cmd_code,
            storage_ix,
            storage
        );
    }
    if (*storage_ix).wrapping_sub(
           initial_storage_ix
       ) > (31i32 as (usize)).wrapping_add(input_size << 3i32) {
        EmitUncompressedMetaBlock(
            input,
            input.offset(input_size as (isize)),
            initial_storage_ix,
            storage_ix,
            storage
        );
    }
    if is_last != 0 {
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
        *storage_ix = (*storage_ix).wrapping_add(
                          7u32 as (usize)
                      ) & !7u32 as (usize);
    }
}
