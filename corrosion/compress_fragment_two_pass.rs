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
    fn memcmp(
        __s1 : *const std::os::raw::c_void,
        __s2 : *const std::os::raw::c_void,
        __n : usize
    ) -> i32;
    fn memcpy(
        __dest : *mut std::os::raw::c_void,
        __src : *const std::os::raw::c_void,
        __n : usize
    ) -> *mut std::os::raw::c_void;
    fn memset(
        __s : *mut std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut std::os::raw::c_void;
}

static kCompressFragmentTwoPassBlockSize
    : usize
    = (1i32 << 17i32) as (usize);

static mut kLog2Table
    : *const f32
    = 0.0000000000000000f32 as (*const f32);

static mut kInsBase : *mut u32 = 0i32 as (*mut u32);

static mut kInsExtra : *mut u32 = 0i32 as (*mut u32);

static mut kCopyBase : *mut u32 = 2i32 as (*mut u32);

static mut kCopyExtra : *mut u32 = 0i32 as (*mut u32);

static kBrotliMinWindowBits : i32 = 10i32;

static kBrotliMaxWindowBits : i32 = 24i32;

static mut kUTF8ContextLookup : *const u8 = 0i32 as (*const u8);

static mut kSigned3BitContextLookup
    : *const u8
    = 0i32 as (*const u8);

static kHashMul32 : u32 = 0x1e35a7bdi32 as (u32);

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

unsafe extern fn brotli_min_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a < b { a } else { b }
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
           ) << 16i32).wrapping_mul(
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
                                                              ) as (i32)) && (*p1.offset(
                                                                                   5i32 as (isize)
                                                                               ) as (i32) == *p2.offset(
                                                                                                  5i32 as (isize)
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
    mut insertlen : u32, mut commands : *mut *mut u32
) {
    if insertlen < 6i32 as (u32) {
        **commands = insertlen;
    } else if insertlen < 130i32 as (u32) {
        let tail : u32 = insertlen.wrapping_sub(2i32 as (u32));
        let nbits
            : u32
            = Log2FloorNonZero(tail as (usize)).wrapping_sub(1u32);
        let prefix : u32 = tail >> nbits;
        let inscode
            : u32
            = (nbits << 1i32).wrapping_add(prefix).wrapping_add(2i32 as (u32));
        let extra : u32 = tail.wrapping_sub(prefix << nbits);
        **commands = inscode | extra << 8i32;
    } else if insertlen < 2114i32 as (u32) {
        let tail : u32 = insertlen.wrapping_sub(66i32 as (u32));
        let nbits : u32 = Log2FloorNonZero(tail as (usize));
        let code : u32 = nbits.wrapping_add(10i32 as (u32));
        let extra : u32 = tail.wrapping_sub(1u32 << nbits);
        **commands = code | extra << 8i32;
    } else if insertlen < 6210i32 as (u32) {
        let extra : u32 = insertlen.wrapping_sub(2114i32 as (u32));
        **commands = 21i32 as (u32) | extra << 8i32;
    } else if insertlen < 22594i32 as (u32) {
        let extra : u32 = insertlen.wrapping_sub(6210i32 as (u32));
        **commands = 22i32 as (u32) | extra << 8i32;
    } else {
        let extra : u32 = insertlen.wrapping_sub(22594i32 as (u32));
        **commands = 23i32 as (u32) | extra << 8i32;
    }
    *commands = (*commands).offset(1 as (isize));
}

unsafe extern fn EmitDistance(
    mut distance : u32, mut commands : *mut *mut u32
) {
    let mut d : u32 = distance.wrapping_add(3i32 as (u32));
    let mut nbits
        : u32
        = Log2FloorNonZero(d as (usize)).wrapping_sub(1i32 as (u32));
    let prefix : u32 = d >> nbits & 1i32 as (u32);
    let offset : u32 = (2i32 as (u32)).wrapping_add(prefix) << nbits;
    let distcode
        : u32
        = (2i32 as (u32)).wrapping_mul(
              nbits.wrapping_sub(1i32 as (u32))
          ).wrapping_add(
              prefix
          ).wrapping_add(
              80i32 as (u32)
          );
    let mut extra : u32 = d.wrapping_sub(offset);
    **commands = distcode | extra << 8i32;
    *commands = (*commands).offset(1 as (isize));
}

unsafe extern fn EmitCopyLenLastDistance(
    mut copylen : usize, mut commands : *mut *mut u32
) { if copylen < 12i32 as (usize) {
        **commands = copylen.wrapping_add(20i32 as (usize)) as (u32);
        *commands = (*commands).offset(1 as (isize));
    } else if copylen < 72i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(8i32 as (usize));
        let nbits
            : usize
            = Log2FloorNonZero(tail).wrapping_sub(1i32 as (u32)) as (usize);
        let prefix : usize = tail >> nbits;
        let code
            : usize
            = (nbits << 1i32).wrapping_add(prefix).wrapping_add(
                  28i32 as (usize)
              );
        let extra : usize = tail.wrapping_sub(prefix << nbits);
        **commands = (code | extra << 8i32) as (u32);
        *commands = (*commands).offset(1 as (isize));
    } else if copylen < 136i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(8i32 as (usize));
        let code : usize = (tail >> 5i32).wrapping_add(54i32 as (usize));
        let extra : usize = tail & 31i32 as (usize);
        **commands = (code | extra << 8i32) as (u32);
        *commands = (*commands).offset(1 as (isize));
        **commands = 64i32 as (u32);
        *commands = (*commands).offset(1 as (isize));
    } else if copylen < 2120i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(72i32 as (usize));
        let nbits : usize = Log2FloorNonZero(tail) as (usize);
        let code : usize = nbits.wrapping_add(52i32 as (usize));
        let extra : usize = tail.wrapping_sub(1i32 as (usize) << nbits);
        **commands = (code | extra << 8i32) as (u32);
        *commands = (*commands).offset(1 as (isize));
        **commands = 64i32 as (u32);
        *commands = (*commands).offset(1 as (isize));
    } else {
        let extra : usize = copylen.wrapping_sub(2120i32 as (usize));
        **commands = (63i32 as (usize) | extra << 8i32) as (u32);
        *commands = (*commands).offset(1 as (isize));
        **commands = 64i32 as (u32);
        *commands = (*commands).offset(1 as (isize));
    }
}

unsafe extern fn HashBytesAtOffset(
    mut v : usize, mut offset : i32, mut shift : usize
) -> u32 {
    if offset >= 0i32 {
        0i32;
    } else {
        __assert_fail(
            b"offset >= 0\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"HashBytesAtOffset\0".as_ptr()
        );
    }
    if offset <= 2i32 {
        0i32;
    } else {
        __assert_fail(
            b"offset <= 2\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"HashBytesAtOffset\0".as_ptr()
        );
    }
    {
        let h
            : usize
            = (v >> 8i32 * offset << 16i32).wrapping_mul(
                  kHashMul32 as (usize)
              );
        (h >> shift) as (u32)
    }
}

unsafe extern fn EmitCopyLen(
    mut copylen : usize, mut commands : *mut *mut u32
) {
    if copylen < 10i32 as (usize) {
        **commands = copylen.wrapping_add(38i32 as (usize)) as (u32);
    } else if copylen < 134i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(6i32 as (usize));
        let nbits
            : usize
            = Log2FloorNonZero(tail).wrapping_sub(1i32 as (u32)) as (usize);
        let prefix : usize = tail >> nbits;
        let code
            : usize
            = (nbits << 1i32).wrapping_add(prefix).wrapping_add(
                  44i32 as (usize)
              );
        let extra : usize = tail.wrapping_sub(prefix << nbits);
        **commands = (code | extra << 8i32) as (u32);
    } else if copylen < 2118i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(70i32 as (usize));
        let nbits : usize = Log2FloorNonZero(tail) as (usize);
        let code : usize = nbits.wrapping_add(52i32 as (usize));
        let extra : usize = tail.wrapping_sub(1i32 as (usize) << nbits);
        **commands = (code | extra << 8i32) as (u32);
    } else {
        let extra : usize = copylen.wrapping_sub(2118i32 as (usize));
        **commands = (63i32 as (usize) | extra << 8i32) as (u32);
    }
    *commands = (*commands).offset(1 as (isize));
}

unsafe extern fn CreateCommands(
    mut input : *const u8,
    mut block_size : usize,
    mut input_size : usize,
    mut base_ip : *const u8,
    mut table : *mut i32,
    mut table_bits : usize,
    mut literals : *mut *mut u8,
    mut commands : *mut *mut u32
) {
    let mut ip : *const u8 = input;
    let shift : usize = (64u32 as (usize)).wrapping_sub(table_bits);
    let mut ip_end : *const u8 = input.offset(block_size as (isize));
    let mut next_emit : *const u8 = input;
    let mut last_distance : i32 = -1i32;
    let kInputMarginBytes : usize = 16i32 as (usize);
    let kMinMatchLen : usize = 6i32 as (usize);
    if block_size >= kInputMarginBytes {
        let len_limit
            : usize
            = brotli_min_size_t(
                  block_size.wrapping_sub(kMinMatchLen),
                  input_size.wrapping_sub(kInputMarginBytes)
              );
        let mut ip_limit : *const u8 = input.offset(len_limit as (isize));
        let mut next_hash : u32;
        let mut goto_emit_remainder : i32 = 0i32;
        next_hash = Hash(
                        {
                            ip = ip.offset(1 as (isize));
                            ip
                        },
                        shift
                    );
        while goto_emit_remainder == 0 {
            let mut skip : u32 = 32i32 as (u32);
            let mut next_ip : *const u8 = ip;
            let mut candidate : *const u8;
            if next_emit < ip {
                0i32;
            } else {
                __assert_fail(
                    b"next_emit < ip\0".as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    b"CreateCommands\0".as_ptr()
                );
            }
            loop {
                {
                    'break3: loop {
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
                            if hash == Hash(ip,shift) {
                                0i32;
                            } else {
                                __assert_fail(
                                    b"hash == Hash(ip, shift)\0".as_ptr(),
                                    file!().as_ptr(),
                                    line!(),
                                    b"CreateCommands\0".as_ptr()
                                );
                            }
                            next_ip = ip.offset(bytes_between_hash_lookups as (isize));
                            if next_ip > ip_limit {
                                goto_emit_remainder = 1i32;
                                {
                                    if 1337i32 != 0 {
                                        break 'break3;
                                    }
                                }
                            }
                            next_hash = Hash(next_ip,shift);
                            candidate = ip.offset(-(last_distance as (isize)));
                            if IsMatch(ip,candidate) != 0 {
                                if candidate < ip {
                                    *table.offset(hash as (isize)) = ((ip as (isize)).wrapping_sub(
                                                                          base_ip as (isize)
                                                                      ) / std::mem::size_of::<*const u8>(
                                                                          ) as (isize)) as (i32);
                                    {
                                        if 1337i32 != 0 {
                                            break 'break3;
                                        }
                                    }
                                }
                            }
                            candidate = base_ip.offset(
                                            *table.offset(hash as (isize)) as (isize)
                                        );
                            if candidate >= base_ip {
                                0i32;
                            } else {
                                __assert_fail(
                                    b"candidate >= base_ip\0".as_ptr(),
                                    file!().as_ptr(),
                                    line!(),
                                    b"CreateCommands\0".as_ptr()
                                );
                            }
                            if candidate < ip {
                                0i32;
                            } else {
                                __assert_fail(
                                    b"candidate < ip\0".as_ptr(),
                                    file!().as_ptr(),
                                    line!(),
                                    b"CreateCommands\0".as_ptr()
                                );
                            }
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
                                        ) as (isize) && (goto_emit_remainder == 0)) {
                    break;
                }
            }
            if goto_emit_remainder != 0 {
                if 1337i32 != 0 {
                    break;
                }
            }
            {
                let mut base : *const u8 = ip;
                let mut matched
                    : usize
                    = (6i32 as (usize)).wrapping_add(
                          FindMatchLengthWithLimit(
                              candidate.offset(6i32 as (isize)),
                              ip.offset(6i32 as (isize)),
                              (((ip_end as (isize)).wrapping_sub(
                                    ip as (isize)
                                ) / std::mem::size_of::<*const u8>(
                                    ) as (isize)) as (usize)).wrapping_sub(
                                  6i32 as (usize)
                              )
                          )
                      );
                let mut distance
                    : i32
                    = ((base as (isize)).wrapping_sub(
                           candidate as (isize)
                       ) / std::mem::size_of::<*const u8>() as (isize)) as (i32);
                let mut insert
                    : i32
                    = ((base as (isize)).wrapping_sub(
                           next_emit as (isize)
                       ) / std::mem::size_of::<*const u8>() as (isize)) as (i32);
                ip = ip.offset(matched as (isize));
                if 0i32 == memcmp(
                               base as (*const std::os::raw::c_void),
                               candidate as (*const std::os::raw::c_void),
                               matched
                           ) {
                    0i32;
                } else {
                    __assert_fail(
                        b"0 == memcmp(base, candidate, matched)\0".as_ptr(),
                        file!().as_ptr(),
                        line!(),
                        b"CreateCommands\0".as_ptr()
                    );
                }
                EmitInsertLen(insert as (u32),commands);
                memcpy(
                    *literals as (*mut std::os::raw::c_void),
                    next_emit as (*const std::os::raw::c_void),
                    insert as (usize)
                );
                *literals = (*literals).offset(insert as (isize));
                if distance == last_distance {
                    **commands = 64i32 as (u32);
                    *commands = (*commands).offset(1 as (isize));
                } else {
                    EmitDistance(distance as (u32),commands);
                    last_distance = distance;
                }
                EmitCopyLenLastDistance(matched,commands);
                next_emit = ip;
                if ip >= ip_limit {
                    goto_emit_remainder = 1i32;
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
                              ip.offset(-(5i32 as (isize))) as (*const std::os::raw::c_void)
                          );
                    let mut prev_hash
                        : u32
                        = HashBytesAtOffset(input_bytes,0i32,shift);
                    let mut cur_hash : u32;
                    *table.offset(
                         prev_hash as (isize)
                     ) = ((ip as (isize)).wrapping_sub(
                              base_ip as (isize)
                          ) / std::mem::size_of::<*const u8>(
                              ) as (isize) - 5i32 as (isize)) as (i32);
                    prev_hash = HashBytesAtOffset(input_bytes,1i32,shift);
                    *table.offset(
                         prev_hash as (isize)
                     ) = ((ip as (isize)).wrapping_sub(
                              base_ip as (isize)
                          ) / std::mem::size_of::<*const u8>(
                              ) as (isize) - 4i32 as (isize)) as (i32);
                    prev_hash = HashBytesAtOffset(input_bytes,2i32,shift);
                    *table.offset(
                         prev_hash as (isize)
                     ) = ((ip as (isize)).wrapping_sub(
                              base_ip as (isize)
                          ) / std::mem::size_of::<*const u8>(
                              ) as (isize) - 3i32 as (isize)) as (i32);
                    input_bytes = BROTLI_UNALIGNED_LOAD64(
                                      ip.offset(-(2i32 as (isize))) as (*const std::os::raw::c_void)
                                  );
                    cur_hash = HashBytesAtOffset(input_bytes,2i32,shift);
                    prev_hash = HashBytesAtOffset(input_bytes,0i32,shift);
                    *table.offset(
                         prev_hash as (isize)
                     ) = ((ip as (isize)).wrapping_sub(
                              base_ip as (isize)
                          ) / std::mem::size_of::<*const u8>(
                              ) as (isize) - 2i32 as (isize)) as (i32);
                    prev_hash = HashBytesAtOffset(input_bytes,1i32,shift);
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
            while (ip as (isize)).wrapping_sub(
                      candidate as (isize)
                  ) / std::mem::size_of::<*const u8>(
                      ) as (isize) <= (1i32 as (usize) << 18i32).wrapping_sub(
                                          16i32 as (usize)
                                      ) as (isize) && (IsMatch(ip,candidate) != 0) {
                let mut base : *const u8 = ip;
                let mut matched
                    : usize
                    = (6i32 as (usize)).wrapping_add(
                          FindMatchLengthWithLimit(
                              candidate.offset(6i32 as (isize)),
                              ip.offset(6i32 as (isize)),
                              (((ip_end as (isize)).wrapping_sub(
                                    ip as (isize)
                                ) / std::mem::size_of::<*const u8>(
                                    ) as (isize)) as (usize)).wrapping_sub(
                                  6i32 as (usize)
                              )
                          )
                      );
                ip = ip.offset(matched as (isize));
                last_distance = ((base as (isize)).wrapping_sub(
                                     candidate as (isize)
                                 ) / std::mem::size_of::<*const u8>() as (isize)) as (i32);
                if 0i32 == memcmp(
                               base as (*const std::os::raw::c_void),
                               candidate as (*const std::os::raw::c_void),
                               matched
                           ) {
                    0i32;
                } else {
                    __assert_fail(
                        b"0 == memcmp(base, candidate, matched)\0".as_ptr(),
                        file!().as_ptr(),
                        line!(),
                        b"CreateCommands\0".as_ptr()
                    );
                }
                EmitCopyLen(matched,commands);
                EmitDistance(last_distance as (u32),commands);
                next_emit = ip;
                if ip >= ip_limit {
                    goto_emit_remainder = 1i32;
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
                              ip.offset(-(5i32 as (isize))) as (*const std::os::raw::c_void)
                          );
                    let mut prev_hash
                        : u32
                        = HashBytesAtOffset(input_bytes,0i32,shift);
                    let mut cur_hash : u32;
                    *table.offset(
                         prev_hash as (isize)
                     ) = ((ip as (isize)).wrapping_sub(
                              base_ip as (isize)
                          ) / std::mem::size_of::<*const u8>(
                              ) as (isize) - 5i32 as (isize)) as (i32);
                    prev_hash = HashBytesAtOffset(input_bytes,1i32,shift);
                    *table.offset(
                         prev_hash as (isize)
                     ) = ((ip as (isize)).wrapping_sub(
                              base_ip as (isize)
                          ) / std::mem::size_of::<*const u8>(
                              ) as (isize) - 4i32 as (isize)) as (i32);
                    prev_hash = HashBytesAtOffset(input_bytes,2i32,shift);
                    *table.offset(
                         prev_hash as (isize)
                     ) = ((ip as (isize)).wrapping_sub(
                              base_ip as (isize)
                          ) / std::mem::size_of::<*const u8>(
                              ) as (isize) - 3i32 as (isize)) as (i32);
                    input_bytes = BROTLI_UNALIGNED_LOAD64(
                                      ip.offset(-(2i32 as (isize))) as (*const std::os::raw::c_void)
                                  );
                    cur_hash = HashBytesAtOffset(input_bytes,2i32,shift);
                    prev_hash = HashBytesAtOffset(input_bytes,0i32,shift);
                    *table.offset(
                         prev_hash as (isize)
                     ) = ((ip as (isize)).wrapping_sub(
                              base_ip as (isize)
                          ) / std::mem::size_of::<*const u8>(
                              ) as (isize) - 2i32 as (isize)) as (i32);
                    prev_hash = HashBytesAtOffset(input_bytes,1i32,shift);
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
            if goto_emit_remainder == 0 {
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
    if next_emit <= ip_end {
        0i32;
    } else {
        __assert_fail(
            b"next_emit <= ip_end\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"CreateCommands\0".as_ptr()
        );
    }
    if next_emit < ip_end {
        let insert
            : u32
            = ((ip_end as (isize)).wrapping_sub(
                   next_emit as (isize)
               ) / std::mem::size_of::<*const u8>() as (isize)) as (u32);
        EmitInsertLen(insert,commands);
        memcpy(
            *literals as (*mut std::os::raw::c_void),
            next_emit as (*const std::os::raw::c_void),
            insert as (usize)
        );
        *literals = (*literals).offset(insert as (isize));
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

unsafe extern fn ShannonEntropy(
    mut population : *const u32,
    mut size : usize,
    mut total : *mut usize
) -> f64 {
    let mut sum : usize = 0i32 as (usize);
    let mut retval : f64 = 0i32 as (f64);
    let mut population_end
        : *const u32
        = population.offset(size as (isize));
    let mut p : usize;
    let mut odd_number_of_elements_left : i32 = 0i32;
    if size & 1i32 as (usize) != 0 {
        odd_number_of_elements_left = 1i32;
    }
    while population < population_end {
        if odd_number_of_elements_left == 0 {
            p = *{
                     let _old = population;
                     population = population.offset(1 as (isize));
                     _old
                 } as (usize);
            sum = sum.wrapping_add(p);
            retval = retval - p as (f64) * FastLog2(p);
        }
        odd_number_of_elements_left = 0i32;
        p = *{
                 let _old = population;
                 population = population.offset(1 as (isize));
                 _old
             } as (usize);
        sum = sum.wrapping_add(p);
        retval = retval - p as (f64) * FastLog2(p);
    }
    if sum != 0 {
        retval = retval + sum as (f64) * FastLog2(sum);
    }
    *total = sum;
    retval
}

unsafe extern fn BitsEntropy(
    mut population : *const u32, mut size : usize
) -> f64 {
    let mut sum : usize;
    let mut retval
        : f64
        = ShannonEntropy(population,size,&mut sum as (*mut usize));
    if retval < sum as (f64) {
        retval = sum as (f64);
    }
    retval
}

unsafe extern fn ShouldCompress(
    mut input : *const u8,
    mut input_size : usize,
    mut num_literals : usize
) -> i32 {
    let mut corpus_size : f64 = input_size as (f64);
    if num_literals as (f64) < 0.98f64 * corpus_size {
        1i32
    } else {
        let mut literal_histo : *mut u32 = 0i32 as (*mut u32);
        let max_total_bit_cost
            : f64
            = corpus_size * 8i32 as (f64) * 0.98f64 / 43i32 as (f64);
        let mut i : usize;
        i = 0i32 as (usize);
        while i < input_size {
            {
                let _rhs = 1;
                let _lhs
                    = &mut *literal_histo.offset(
                                *input.offset(i as (isize)) as (isize)
                            );
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            i = i.wrapping_add(43i32 as (usize));
        }
        if !!(BitsEntropy(
                  literal_histo as (*const u32),
                  256i32 as (usize)
              ) < max_total_bit_cost) {
            1i32
        } else {
            0i32
        }
    }
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
        depth.offset(24i32 as (isize)) as (*const std::os::raw::c_void),
        24i32 as (usize)
    );
    memcpy(
        cmd_depth.offset(24i32 as (isize)) as (*mut std::os::raw::c_void),
        depth as (*const std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.offset(32i32 as (isize)) as (*mut std::os::raw::c_void),
        depth.offset(48i32 as (isize)) as (*const std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.offset(40i32 as (isize)) as (*mut std::os::raw::c_void),
        depth.offset(8i32 as (isize)) as (*const std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.offset(48i32 as (isize)) as (*mut std::os::raw::c_void),
        depth.offset(56i32 as (isize)) as (*const std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.offset(56i32 as (isize)) as (*mut std::os::raw::c_void),
        depth.offset(16i32 as (isize)) as (*const std::os::raw::c_void),
        8i32 as (usize)
    );
    BrotliConvertBitDepthsToSymbols(
        cmd_depth as (*const u8),
        64i32 as (usize),
        cmd_bits
    );
    memcpy(
        bits as (*mut std::os::raw::c_void),
        cmd_bits.offset(24i32 as (isize)) as (*const std::os::raw::c_void),
        16i32 as (usize)
    );
    memcpy(
        bits.offset(8i32 as (isize)) as (*mut std::os::raw::c_void),
        cmd_bits.offset(40i32 as (isize)) as (*const std::os::raw::c_void),
        16i32 as (usize)
    );
    memcpy(
        bits.offset(16i32 as (isize)) as (*mut std::os::raw::c_void),
        cmd_bits.offset(56i32 as (isize)) as (*const std::os::raw::c_void),
        16i32 as (usize)
    );
    memcpy(
        bits.offset(24i32 as (isize)) as (*mut std::os::raw::c_void),
        cmd_bits as (*const std::os::raw::c_void),
        48i32 as (usize)
    );
    memcpy(
        bits.offset(48i32 as (isize)) as (*mut std::os::raw::c_void),
        cmd_bits.offset(32i32 as (isize)) as (*const std::os::raw::c_void),
        16i32 as (usize)
    );
    memcpy(
        bits.offset(56i32 as (isize)) as (*mut std::os::raw::c_void),
        cmd_bits.offset(48i32 as (isize)) as (*const std::os::raw::c_void),
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
            depth.offset(24i32 as (isize)) as (*const std::os::raw::c_void),
            8i32 as (usize)
        );
        memcpy(
            cmd_depth.offset(64i32 as (isize)) as (*mut std::os::raw::c_void),
            depth.offset(32i32 as (isize)) as (*const std::os::raw::c_void),
            8i32 as (usize)
        );
        memcpy(
            cmd_depth.offset(128i32 as (isize)) as (*mut std::os::raw::c_void),
            depth.offset(40i32 as (isize)) as (*const std::os::raw::c_void),
            8i32 as (usize)
        );
        memcpy(
            cmd_depth.offset(192i32 as (isize)) as (*mut std::os::raw::c_void),
            depth.offset(48i32 as (isize)) as (*const std::os::raw::c_void),
            8i32 as (usize)
        );
        memcpy(
            cmd_depth.offset(384i32 as (isize)) as (*mut std::os::raw::c_void),
            depth.offset(56i32 as (isize)) as (*const std::os::raw::c_void),
            8i32 as (usize)
        );
        i = 0i32 as (usize);
        while i < 8i32 as (usize) {
            {
                *cmd_depth.offset(
                     (128i32 as (usize)).wrapping_add(
                         (8i32 as (usize)).wrapping_mul(i)
                     ) as (isize)
                 ) = *depth.offset(i as (isize));
                *cmd_depth.offset(
                     (256i32 as (usize)).wrapping_add(
                         (8i32 as (usize)).wrapping_mul(i)
                     ) as (isize)
                 ) = *depth.offset((8i32 as (usize)).wrapping_add(i) as (isize));
                *cmd_depth.offset(
                     (448i32 as (usize)).wrapping_add(
                         (8i32 as (usize)).wrapping_mul(i)
                     ) as (isize)
                 ) = *depth.offset((16i32 as (usize)).wrapping_add(i) as (isize));
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

unsafe extern fn StoreCommands(
    mut m : *mut MemoryManager,
    mut literals : *const u8,
    num_literals : usize,
    mut commands : *const u32,
    num_commands : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    static mut kNumExtraBits : *const u32 = 0i32 as (*const u32);
    static mut kInsertOffset : *const u32 = 0i32 as (*const u32);
    let mut lit_depths : *mut u8;
    let mut lit_bits : *mut u16;
    let mut lit_histo : *mut u32 = 0i32 as (*mut u32);
    let mut cmd_depths : *mut u8 = 0i32 as (*mut u8);
    let mut cmd_bits : *mut u16 = 0i32 as (*mut u16);
    let mut cmd_histo : *mut u32 = 0i32 as (*mut u32);
    let mut i : usize;
    i = 0i32 as (usize);
    while i < num_literals {
        {
            let _rhs = 1;
            let _lhs
                = &mut *lit_histo.offset(
                            *literals.offset(i as (isize)) as (isize)
                        );
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        i = i.wrapping_add(1 as (usize));
    }
    BrotliBuildAndStoreHuffmanTreeFast(
        m,
        lit_histo as (*const u32),
        num_literals,
        8i32 as (usize),
        lit_depths,
        lit_bits,
        storage_ix,
        storage
    );
    if !(0i32 == 0) {
        return;
    }
    i = 0i32 as (usize);
    while i < num_commands {
        {
            let code : u32 = *commands.offset(i as (isize)) & 0xffi32 as (u32);
            if code < 128i32 as (u32) {
                0i32;
            } else {
                __assert_fail(
                    b"code < 128\0".as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    b"StoreCommands\0".as_ptr()
                );
            }
            {
                let _rhs = 1;
                let _lhs = &mut *cmd_histo.offset(code as (isize));
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    {
        let _rhs = 1i32;
        let _lhs = &mut *cmd_histo.offset(1i32 as (isize));
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    {
        let _rhs = 1i32;
        let _lhs = &mut *cmd_histo.offset(2i32 as (isize));
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    {
        let _rhs = 1i32;
        let _lhs = &mut *cmd_histo.offset(64i32 as (isize));
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    {
        let _rhs = 1i32;
        let _lhs = &mut *cmd_histo.offset(84i32 as (isize));
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    BuildAndStoreCommandPrefixCode(
        cmd_histo as (*const u32),
        cmd_depths,
        cmd_bits,
        storage_ix,
        storage
    );
    i = 0i32 as (usize);
    while i < num_commands {
        {
            let cmd : u32 = *commands.offset(i as (isize));
            let code : u32 = cmd & 0xffi32 as (u32);
            let extra : u32 = cmd >> 8i32;
            if code < 128i32 as (u32) {
                0i32;
            } else {
                __assert_fail(
                    b"code < 128\0".as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    b"StoreCommands\0".as_ptr()
                );
            }
            BrotliWriteBits(
                *cmd_depths.offset(code as (isize)) as (usize),
                *cmd_bits.offset(code as (isize)) as (usize),
                storage_ix,
                storage
            );
            BrotliWriteBits(
                *kNumExtraBits.offset(code as (isize)) as (usize),
                extra as (usize),
                storage_ix,
                storage
            );
            if code < 24i32 as (u32) {
                let insert
                    : u32
                    = (*kInsertOffset.offset(code as (isize))).wrapping_add(extra);
                let mut j : u32;
                j = 0i32 as (u32);
                while j < insert {
                    {
                        let lit : u8 = *literals;
                        BrotliWriteBits(
                            *lit_depths.offset(lit as (isize)) as (usize),
                            *lit_bits.offset(lit as (isize)) as (usize),
                            storage_ix,
                            storage
                        );
                        literals = literals.offset(1 as (isize));
                    }
                    j = j.wrapping_add(1 as (u32));
                }
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn EmitUncompressedMetaBlock(
    mut input : *const u8,
    mut input_size : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliStoreMetaBlockHeader(input_size,1i32,storage_ix,storage);
    *storage_ix = (*storage_ix).wrapping_add(
                      7u32 as (usize)
                  ) & !7u32 as (usize);
    memcpy(
        &mut *storage.offset(
                  (*storage_ix >> 3i32) as (isize)
              ) as (*mut u8) as (*mut std::os::raw::c_void),
        input as (*const std::os::raw::c_void),
        input_size
    );
    *storage_ix = (*storage_ix).wrapping_add(input_size << 3i32);
    *storage.offset((*storage_ix >> 3i32) as (isize)) = 0i32 as (u8);
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut table_bits : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut base_ip : *const u8 = input;
    is_last;
    while input_size > 0i32 as (usize) {
        let mut block_size
            : usize
            = brotli_min_size_t(input_size,kCompressFragmentTwoPassBlockSize);
        let mut commands : *mut u32 = command_buf;
        let mut literals : *mut u8 = literal_buf;
        let mut num_literals : usize;
        CreateCommands(
            input,
            block_size,
            input_size,
            base_ip,
            table,
            table_bits,
            &mut literals as (*mut *mut u8),
            &mut commands as (*mut *mut u32)
        );
        num_literals = ((literals as (isize)).wrapping_sub(
                            literal_buf as (isize)
                        ) / std::mem::size_of::<*mut u8>() as (isize)) as (usize);
        if ShouldCompress(input,block_size,num_literals) != 0 {
            let num_commands
                : usize
                = ((commands as (isize)).wrapping_sub(
                       command_buf as (isize)
                   ) / std::mem::size_of::<*mut u32>() as (isize)) as (usize);
            BrotliStoreMetaBlockHeader(block_size,0i32,storage_ix,storage);
            BrotliWriteBits(
                13i32 as (usize),
                0i32 as (usize),
                storage_ix,
                storage
            );
            StoreCommands(
                m,
                literal_buf as (*const u8),
                num_literals,
                command_buf as (*const u32),
                num_commands,
                storage_ix,
                storage
            );
            if !(0i32 == 0) {
                return;
            }
        } else {
            EmitUncompressedMetaBlock(input,block_size,storage_ix,storage);
        }
        input = input.offset(block_size as (isize));
        input_size = input_size.wrapping_sub(block_size);
    }
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl8(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        8i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl9(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        9i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl10(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        10i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl11(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        11i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl12(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        12i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl13(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        13i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl14(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        14i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl15(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        15i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl16(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        16i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl17(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        17i32 as (usize),
        storage_ix,
        storage
    );
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

#[no_mangle]
pub unsafe extern fn BrotliCompressFragmentTwoPass(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut table_size : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let initial_storage_ix : usize = *storage_ix;
    let table_bits : usize = Log2FloorNonZero(table_size) as (usize);
    if table_bits == 8i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl8(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    }
    if table_bits == 9i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl9(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    }
    if table_bits == 10i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl10(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    }
    if table_bits == 11i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl11(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    }
    if table_bits == 12i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl12(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    }
    if table_bits == 13i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl13(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    }
    if table_bits == 14i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl14(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    }
    if table_bits == 15i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl15(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    }
    if table_bits == 16i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl16(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    }
    if table_bits == 17i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl17(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    }
    if (*storage_ix).wrapping_sub(
           initial_storage_ix
       ) > (31i32 as (usize)).wrapping_add(input_size << 3i32) {
        RewindBitPosition(initial_storage_ix,storage_ix,storage);
        EmitUncompressedMetaBlock(input,input_size,storage_ix,storage);
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
