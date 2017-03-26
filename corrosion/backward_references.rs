extern {
    static mut kStaticDictionaryHash : *const u16;
    fn memcpy(
        __dest : *mut std::os::raw::c_void,
        __src : *const std::os::raw::c_void,
        __n : usize
    ) -> *mut std::os::raw::c_void;
}

static mut kLog2Table
    : *const f32
    = 0.0000000000000000f32 as (*const f32);

static mut kInsBase : *mut u32 = 0i32 as (*mut u32);

static mut kInsExtra : *mut u32 = 0i32 as (*mut u32);

static mut kCopyBase : *mut u32 = 2i32 as (*mut u32);

static mut kCopyExtra : *mut u32 = 0i32 as (*mut u32);

static kBrotliMinWindowBits : i32 = 10i32;

static kBrotliMaxWindowBits : i32 = 24i32;

static kInvalidMatch : u32 = 0xfffffffi32 as (u32);

static kCutoffTransformsCount : u32 = 10i32 as (u32);

static kCutoffTransforms
    : usize
    = 0x71b520ai32 as (usize) << 32i32 | 0xda2d3200u32 as (usize);

static kHashMul32 : u32 = 0x1e35a7bdi32 as (u32);

static kHashMul64
    : usize
    = 0x1e35a7bdi32 as (usize) << 32i32 | 0x1e35a7bdi32 as (usize);

static kHashMul64Long
    : usize
    = 0x1fe35a7bu32 as (usize) << 32i32 | 0xd3579bd3u32 as (usize);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length : *mut u8,
    pub offsets_by_length : *mut u32,
    pub data : *mut u8,
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum BrotliEncoderMode {
    BROTLI_MODE_GENERIC = 0i32,
    BROTLI_MODE_TEXT = 1i32,
    BROTLI_MODE_FONT = 2i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrotliHasherParams {
    pub type_ : i32,
    pub bucket_bits : i32,
    pub block_bits : i32,
    pub hash_len : i32,
    pub num_last_distances_to_check : i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrotliEncoderParams {
    pub mode : BrotliEncoderMode,
    pub quality : i32,
    pub lgwin : i32,
    pub lgblock : i32,
    pub size_hint : usize,
    pub disable_literal_context_modeling : i32,
    pub hasher : BrotliHasherParams,
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

unsafe extern fn StoreLookaheadH2() -> usize { 8i32 as (usize) }

unsafe extern fn LiteralSpreeLengthForSparseSearch(
    mut params : *const BrotliEncoderParams
) -> usize {
    (if (*params).quality < 9i32 { 64i32 } else { 512i32 }) as (usize)
}

unsafe extern fn PrepareDistanceCacheH2(
    mut handle : *mut u8, mut distance_cache : *mut i32
) {
    handle;
    distance_cache;
}

unsafe extern fn HashTypeLengthH2() -> usize { 8i32 as (usize) }

unsafe extern fn brotli_min_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a < b { a } else { b }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HasherSearchResult {
    pub len : usize,
    pub len_x_code : usize,
    pub distance : usize,
    pub score : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H2 {
    pub buckets_ : *mut u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Struct1 {
    pub params : BrotliHasherParams,
    pub is_prepared_ : i32,
    pub dict_num_lookups : usize,
    pub dict_num_matches : usize,
}

unsafe extern fn GetHasherCommon(
    mut handle : *mut u8
) -> *mut Struct1 {
    handle as (*mut Struct1)
}

unsafe extern fn SelfH2(mut handle : *mut u8) -> *mut H2 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct1) as (*mut H2)
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

unsafe extern fn HashBytesH2(mut data : *const u8) -> u32 {
    let h
        : usize
        = (BROTLI_UNALIGNED_LOAD64(
               data as (*const std::os::raw::c_void)
           ) << 64i32 - 8i32 * 5i32).wrapping_mul(
              kHashMul64
          );
    (h >> 64i32 - 16i32) as (u32)
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

unsafe extern fn BackwardReferenceScoreUsingLastDistance(
    mut copy_length : usize
) -> usize {
    (135i32 as (usize)).wrapping_mul(
        copy_length as (usize)
    ).wrapping_add(
        ((30i32 * 8i32) as (usize)).wrapping_mul(
            std::mem::size_of::<usize>()
        )
    ).wrapping_add(
        15i32 as (usize)
    )
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

unsafe extern fn BackwardReferenceScore(
    mut copy_length : usize, mut backward_reference_offset : usize
) -> usize {
    ((30i32 * 8i32) as (usize)).wrapping_mul(
        std::mem::size_of::<usize>()
    ).wrapping_add(
        (135i32 as (usize)).wrapping_mul(copy_length as (usize))
    ).wrapping_sub(
        (30i32 as (u32)).wrapping_mul(
            Log2FloorNonZero(backward_reference_offset)
        ) as (usize)
    )
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

unsafe extern fn Hash14(mut data : *const u8) -> u32 {
    let mut h
        : u32
        = BROTLI_UNALIGNED_LOAD32(
              data as (*const std::os::raw::c_void)
          ).wrapping_mul(
              kHashMul32
          );
    h >> 32i32 - 14i32
}

unsafe extern fn TestStaticDictionaryItem(
    mut dictionary : *const BrotliDictionary,
    mut item : usize,
    mut data : *const u8,
    mut max_length : usize,
    mut max_backward : usize,
    mut out : *mut HasherSearchResult
) -> i32 {
    let mut len : usize;
    let mut dist : usize;
    let mut offset : usize;
    let mut matchlen : usize;
    let mut backward : usize;
    let mut score : usize;
    len = item & 0x1fi32 as (usize);
    dist = item >> 5i32;
    offset = (*(*dictionary).offsets_by_length.offset(
                   len as (isize)
               ) as (usize)).wrapping_add(
                 len.wrapping_mul(dist)
             );
    if len > max_length {
        return 0i32;
    }
    matchlen = FindMatchLengthWithLimit(
                   data,
                   &mut *(*dictionary).data.offset(
                             offset as (isize)
                         ) as (*mut u8) as (*const u8),
                   len
               );
    if matchlen.wrapping_add(
           kCutoffTransformsCount as (usize)
       ) <= len || matchlen == 0i32 as (usize) {
        return 0i32;
    }
    {
        let mut cut : usize = len.wrapping_sub(matchlen);
        let mut transform_id
            : usize
            = (cut << 2i32).wrapping_add(
                  (kCutoffTransforms >> cut.wrapping_mul(
                                            6i32 as (usize)
                                        ) & 0x3fi32 as (usize)) as (usize)
              );
        backward = max_backward.wrapping_add(dist).wrapping_add(
                       1i32 as (usize)
                   ).wrapping_add(
                       transform_id << *(*dictionary).size_bits_by_length.offset(
                                            len as (isize)
                                        ) as (i32)
                   );
    }
    score = BackwardReferenceScore(matchlen,backward);
    if score < (*out).score {
        return 0i32;
    }
    (*out).len = matchlen;
    (*out).len_x_code = len ^ matchlen;
    (*out).distance = backward;
    (*out).score = score;
    1i32
}

unsafe extern fn SearchInStaticDictionary(
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut handle : *mut u8,
    mut data : *const u8,
    mut max_length : usize,
    mut max_backward : usize,
    mut out : *mut HasherSearchResult,
    mut shallow : i32
) -> i32 {
    let mut key : usize;
    let mut i : usize;
    let mut is_match_found : i32 = 0i32;
    let mut self : *mut Struct1 = GetHasherCommon(handle);
    if (*self).dict_num_matches < (*self).dict_num_lookups >> 7i32 {
        return 0i32;
    }
    key = (Hash14(data) << 1i32) as (usize);
    i = 0i32 as (usize);
    while i < if shallow != 0 { 1u32 } else { 2u32 } as (usize) {
        {
            let mut item
                : usize
                = *dictionary_hash.offset(key as (isize)) as (usize);
            (*self).dict_num_lookups = (*self).dict_num_lookups.wrapping_add(
                                           1 as (usize)
                                       );
            if item != 0i32 as (usize) {
                let mut item_matches
                    : i32
                    = TestStaticDictionaryItem(
                          dictionary,
                          item,
                          data,
                          max_length,
                          max_backward,
                          out
                      );
                if item_matches != 0 {
                    (*self).dict_num_matches = (*self).dict_num_matches.wrapping_add(
                                                   1 as (usize)
                                               );
                    is_match_found = 1i32;
                }
            }
        }
        i = i.wrapping_add(1 as (usize));
        key = key.wrapping_add(1 as (usize));
    }
    is_match_found
}

unsafe extern fn FindLongestMatchH2(
    mut handle : *mut u8,
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut data : *const u8,
    ring_buffer_mask : usize,
    mut distance_cache : *const i32,
    cur_ix : usize,
    max_length : usize,
    max_backward : usize,
    mut out : *mut HasherSearchResult
) -> i32 {
    let mut self : *mut H2 = SelfH2(handle);
    let best_len_in : usize = (*out).len;
    let cur_ix_masked : usize = cur_ix & ring_buffer_mask;
    let key
        : u32
        = HashBytesH2(
              &*data.offset(cur_ix_masked as (isize)) as (*const u8)
          );
    let mut compare_char
        : i32
        = *data.offset(
               cur_ix_masked.wrapping_add(best_len_in) as (isize)
           ) as (i32);
    let mut best_score : usize = (*out).score;
    let mut best_len : usize = best_len_in;
    let mut cached_backward
        : usize
        = *distance_cache.offset(0i32 as (isize)) as (usize);
    let mut prev_ix : usize = cur_ix.wrapping_sub(cached_backward);
    let mut is_match_found : i32 = 0i32;
    (*out).len_x_code = 0i32 as (usize);
    if prev_ix < cur_ix {
        prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
        if compare_char == *data.offset(
                                prev_ix.wrapping_add(best_len) as (isize)
                            ) as (i32) {
            let mut len
                : usize
                = FindMatchLengthWithLimit(
                      &*data.offset(prev_ix as (isize)) as (*const u8),
                      &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                      max_length
                  );
            if len >= 4i32 as (usize) {
                best_score = BackwardReferenceScoreUsingLastDistance(len);
                best_len = len;
                (*out).len = len;
                (*out).distance = cached_backward;
                (*out).score = best_score;
                compare_char = *data.offset(
                                    cur_ix_masked.wrapping_add(best_len) as (isize)
                                ) as (i32);
                if 1i32 == 1i32 {
                    *(*self).buckets_.offset(key as (isize)) = cur_ix as (u32);
                    return 1i32;
                } else {
                    is_match_found = 1i32;
                }
            }
        }
    }
    if 1i32 == 1i32 {
        let mut backward : usize;
        let mut len : usize;
        prev_ix = *(*self).buckets_.offset(key as (isize)) as (usize);
        *(*self).buckets_.offset(key as (isize)) = cur_ix as (u32);
        backward = cur_ix.wrapping_sub(prev_ix);
        prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
        if compare_char != *data.offset(
                                prev_ix.wrapping_add(best_len_in) as (isize)
                            ) as (i32) {
            return 0i32;
        }
        if backward == 0i32 as (usize) || backward > max_backward {
            return 0i32;
        }
        len = FindMatchLengthWithLimit(
                  &*data.offset(prev_ix as (isize)) as (*const u8),
                  &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                  max_length
              );
        if len >= 4i32 as (usize) {
            (*out).len = len;
            (*out).distance = backward;
            (*out).score = BackwardReferenceScore(len,backward);
            return 1i32;
        }
    } else {
        let mut bucket
            : *mut u32
            = (*self).buckets_.offset(key as (isize));
        let mut i : i32;
        prev_ix = *{
                       let _old = bucket;
                       bucket = bucket.offset(1 as (isize));
                       _old
                   } as (usize);
        i = 0i32;
        while i < 1i32 {
            'continue3: loop {
                {
                    let backward : usize = cur_ix.wrapping_sub(prev_ix);
                    let mut len : usize;
                    prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
                    if compare_char != *data.offset(
                                            prev_ix.wrapping_add(best_len) as (isize)
                                        ) as (i32) {
                        if 1337i32 != 0 {
                            break 'continue3;
                        }
                    }
                    if backward == 0i32 as (usize) || backward > max_backward {
                        if 1337i32 != 0 {
                            break 'continue3;
                        }
                    }
                    len = FindMatchLengthWithLimit(
                              &*data.offset(prev_ix as (isize)) as (*const u8),
                              &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                              max_length
                          );
                    if len >= 4i32 as (usize) {
                        let score : usize = BackwardReferenceScore(len,backward);
                        if best_score < score {
                            best_score = score;
                            best_len = len;
                            (*out).len = best_len;
                            (*out).distance = backward;
                            (*out).score = score;
                            compare_char = *data.offset(
                                                cur_ix_masked.wrapping_add(best_len) as (isize)
                                            ) as (i32);
                            is_match_found = 1i32;
                        }
                    }
                }
                break;
            }
            i = i + 1;
            prev_ix = *{
                           let _old = bucket;
                           bucket = bucket.offset(1 as (isize));
                           _old
                       } as (usize);
        }
    }
    if 1i32 != 0 && (is_match_found == 0) {
        is_match_found = SearchInStaticDictionary(
                             dictionary,
                             dictionary_hash,
                             handle,
                             &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                             max_length,
                             max_backward,
                             out,
                             1i32
                         );
    }
    *(*self).buckets_.offset(
         (key as (usize)).wrapping_add(
             (cur_ix >> 3i32).wrapping_rem(1i32 as (usize))
         ) as (isize)
     ) = cur_ix as (u32);
    is_match_found
}

unsafe extern fn ComputeDistanceCode(
    mut distance : usize,
    mut max_distance : usize,
    mut dist_cache : *const i32
) -> usize {
    if distance <= max_distance {
        let mut distance_plus_3
            : usize
            = distance.wrapping_add(3i32 as (usize));
        let mut offset0
            : usize
            = distance_plus_3.wrapping_sub(
                  *dist_cache.offset(0i32 as (isize)) as (usize)
              );
        let mut offset1
            : usize
            = distance_plus_3.wrapping_sub(
                  *dist_cache.offset(1i32 as (isize)) as (usize)
              );
        if distance == *dist_cache.offset(0i32 as (isize)) as (usize) {
            return 0i32 as (usize);
        } else if distance == *dist_cache.offset(
                                   1i32 as (isize)
                               ) as (usize) {
            return 1i32 as (usize);
        } else if offset0 < 7i32 as (usize) {
            return
                (0x9750468i32 >> (4i32 as (usize)).wrapping_mul(
                                     offset0
                                 ) & 0xfi32) as (usize);
        } else if offset1 < 7i32 as (usize) {
            return
                (0xfdb1acei32 >> (4i32 as (usize)).wrapping_mul(
                                     offset1
                                 ) & 0xfi32) as (usize);
        } else if distance == *dist_cache.offset(
                                   2i32 as (isize)
                               ) as (usize) {
            return 2i32 as (usize);
        } else if distance == *dist_cache.offset(
                                   3i32 as (isize)
                               ) as (usize) {
            return 3i32 as (usize);
        }
    }
    distance.wrapping_add(16i32 as (usize)).wrapping_sub(
        1i32 as (usize)
    )
}

unsafe extern fn PrefixEncodeCopyDistance(
    mut distance_code : usize,
    mut num_direct_codes : usize,
    mut postfix_bits : usize,
    mut code : *mut u16,
    mut extra_bits : *mut u32
) { if distance_code < (16i32 as (usize)).wrapping_add(
                           num_direct_codes
                       ) {
        *code = distance_code as (u16);
        *extra_bits = 0i32 as (u32);
    } else {
        let mut dist
            : usize
            = (1i32 as (usize) << postfix_bits.wrapping_add(
                                      2u32 as (usize)
                                  )).wrapping_add(
                  distance_code.wrapping_sub(16i32 as (usize)).wrapping_sub(
                      num_direct_codes
                  )
              );
        let mut bucket
            : usize
            = Log2FloorNonZero(dist).wrapping_sub(1i32 as (u32)) as (usize);
        let mut postfix_mask
            : usize
            = (1u32 << postfix_bits).wrapping_sub(1i32 as (u32)) as (usize);
        let mut postfix : usize = dist & postfix_mask;
        let mut prefix : usize = dist >> bucket & 1i32 as (usize);
        let mut offset
            : usize
            = (2i32 as (usize)).wrapping_add(prefix) << bucket;
        let mut nbits : usize = bucket.wrapping_sub(postfix_bits);
        *code = (16i32 as (usize)).wrapping_add(
                    num_direct_codes
                ).wrapping_add(
                    (2i32 as (usize)).wrapping_mul(
                        nbits.wrapping_sub(1i32 as (usize))
                    ).wrapping_add(
                        prefix
                    ) << postfix_bits
                ).wrapping_add(
                    postfix
                ) as (u16);
        *extra_bits = (nbits << 24i32 | dist.wrapping_sub(
                                            offset
                                        ) >> postfix_bits) as (u32);
    }
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

unsafe extern fn CombineLengthCodes(
    mut inscode : u16, mut copycode : u16, mut use_last_distance : i32
) -> u16 {
    let mut bits64
        : u16
        = (copycode as (u32) & 0x7u32 | (inscode as (u32) & 0x7u32) << 3i32) as (u16);
    if use_last_distance != 0 && (inscode as (i32) < 8i32) && (copycode as (i32) < 16i32) {
        if copycode as (i32) < 8i32 {
            bits64 as (i32)
        } else {
            bits64 as (i32) | 64i32
        } as (u16)
    } else {
        let mut offset
            : i32
            = 2i32 * ((copycode as (i32) >> 3i32) + 3i32 * (inscode as (i32) >> 3i32));
        offset = (offset << 5i32) + 0x40i32 + (0x520d40i32 >> offset & 0xc0i32);
        (offset as (u16) as (i32) | bits64 as (i32)) as (u16)
    }
}

unsafe extern fn GetLengthCode(
    mut insertlen : usize,
    mut copylen : usize,
    mut use_last_distance : i32,
    mut code : *mut u16
) {
    let mut inscode : u16 = GetInsertLengthCode(insertlen);
    let mut copycode : u16 = GetCopyLengthCode(copylen);
    *code = CombineLengthCodes(inscode,copycode,use_last_distance);
}

unsafe extern fn InitCommand(
    mut self : *mut Command,
    mut insertlen : usize,
    mut copylen : usize,
    mut copylen_code : usize,
    mut distance_code : usize
) {
    (*self).insert_len_ = insertlen as (u32);
    (*self).copy_len_ = (copylen | (copylen_code ^ copylen) << 24i32) as (u32);
    PrefixEncodeCopyDistance(
        distance_code,
        0i32 as (usize),
        0i32 as (usize),
        &mut (*self).dist_prefix_ as (*mut u16),
        &mut (*self).dist_extra_ as (*mut u32)
    );
    GetLengthCode(
        insertlen,
        copylen_code,
        if !!((*self).dist_prefix_ as (i32) == 0i32) {
            1i32
        } else {
            0i32
        },
        &mut (*self).cmd_prefix_ as (*mut u16)
    );
}

unsafe extern fn StoreH2(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let key
        : u32
        = HashBytesH2(
              &*data.offset((ix & mask) as (isize)) as (*const u8)
          );
    let off
        : u32
        = (ix >> 3i32).wrapping_rem(1i32 as (usize)) as (u32);
    *(*SelfH2(handle)).buckets_.offset(
         key.wrapping_add(off) as (isize)
     ) = ix as (u32);
}

unsafe extern fn StoreRangeH2(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix_start : usize,
    ix_end : usize
) {
    let mut i : usize;
    i = ix_start;
    while i < ix_end {
        {
            StoreH2(handle,data,mask,i);
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn brotli_max_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a > b { a } else { b }
}

unsafe extern fn CreateBackwardReferencesH2(
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    mut hasher : *mut u8,
    mut dist_cache : *mut i32,
    mut last_insert_len : *mut usize,
    mut commands : *mut Command,
    mut num_commands : *mut usize,
    mut num_literals : *mut usize
) {
    let max_backward_limit
        : usize
        = (1i32 as (usize) << (*params).lgwin).wrapping_sub(
              16i32 as (usize)
          );
    let orig_commands : *const Command = commands as (*const Command);
    let mut insert_length : usize = *last_insert_len;
    let pos_end : usize = position.wrapping_add(num_bytes);
    let store_end
        : usize
        = if num_bytes >= StoreLookaheadH2() {
              position.wrapping_add(num_bytes).wrapping_sub(
                  StoreLookaheadH2()
              ).wrapping_add(
                  1i32 as (usize)
              )
          } else {
              position
          };
    let random_heuristics_window_size
        : usize
        = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics
        : usize
        = position.wrapping_add(random_heuristics_window_size);
    let kMinScore
        : usize
        = ((30i32 * 8i32) as (usize)).wrapping_mul(
              std::mem::size_of::<usize>()
          ).wrapping_add(
              100i32 as (usize)
          );
    PrepareDistanceCacheH2(hasher,dist_cache);
    while position.wrapping_add(HashTypeLengthH2()) < pos_end {
        let mut max_length : usize = pos_end.wrapping_sub(position);
        let mut max_distance
            : usize
            = brotli_min_size_t(position,max_backward_limit);
        let mut sr : HasherSearchResult;
        sr.len = 0i32 as (usize);
        sr.len_x_code = 0i32 as (usize);
        sr.distance = 0i32 as (usize);
        sr.score = kMinScore;
        if FindLongestMatchH2(
               hasher,
               dictionary,
               dictionary_hash,
               ringbuffer,
               ringbuffer_mask,
               dist_cache as (*const i32),
               position,
               max_length,
               max_distance,
               &mut sr as (*mut HasherSearchResult)
           ) != 0 {
            let mut delayed_backward_references_in_row : i32 = 0i32;
            max_length = max_length.wrapping_sub(1 as (usize));
            'break6: loop {
                'continue7: loop {
                    {
                        let cost_diff_lazy : usize = 175i32 as (usize);
                        let mut is_match_found : i32;
                        let mut sr2 : HasherSearchResult;
                        sr2.len = if (*params).quality < 5i32 {
                                      brotli_min_size_t(
                                          sr.len.wrapping_sub(1i32 as (usize)),
                                          max_length
                                      )
                                  } else {
                                      0i32 as (usize)
                                  };
                        sr2.len_x_code = 0i32 as (usize);
                        sr2.distance = 0i32 as (usize);
                        sr2.score = kMinScore;
                        max_distance = brotli_min_size_t(
                                           position.wrapping_add(1i32 as (usize)),
                                           max_backward_limit
                                       );
                        is_match_found = FindLongestMatchH2(
                                             hasher,
                                             dictionary,
                                             dictionary_hash,
                                             ringbuffer,
                                             ringbuffer_mask,
                                             dist_cache as (*const i32),
                                             position.wrapping_add(1i32 as (usize)),
                                             max_length,
                                             max_distance,
                                             &mut sr2 as (*mut HasherSearchResult)
                                         );
                        if is_match_found != 0 && (sr2.score >= sr.score.wrapping_add(
                                                                    cost_diff_lazy
                                                                )) {
                            position = position.wrapping_add(1 as (usize));
                            insert_length = insert_length.wrapping_add(1 as (usize));
                            sr = sr2;
                            if {
                                   delayed_backward_references_in_row = delayed_backward_references_in_row + 1;
                                   delayed_backward_references_in_row
                               } < 4i32 && (position.wrapping_add(HashTypeLengthH2()) < pos_end) {
                                if 1337i32 != 0 {
                                    break 'continue7;
                                }
                            }
                        }
                        {
                            if 1337i32 != 0 {
                                break 'break6;
                            }
                        }
                    }
                    break;
                }
                max_length = max_length.wrapping_sub(1 as (usize));
            }
            apply_random_heuristics = position.wrapping_add(
                                          (2i32 as (usize)).wrapping_mul(sr.len)
                                      ).wrapping_add(
                                          random_heuristics_window_size
                                      );
            max_distance = brotli_min_size_t(position,max_backward_limit);
            {
                let mut distance_code
                    : usize
                    = ComputeDistanceCode(
                          sr.distance,
                          max_distance,
                          dist_cache as (*const i32)
                      );
                if sr.distance <= max_distance && (distance_code > 0i32 as (usize)) {
                    *dist_cache.offset(3i32 as (isize)) = *dist_cache.offset(
                                                               2i32 as (isize)
                                                           );
                    *dist_cache.offset(2i32 as (isize)) = *dist_cache.offset(
                                                               1i32 as (isize)
                                                           );
                    *dist_cache.offset(1i32 as (isize)) = *dist_cache.offset(
                                                               0i32 as (isize)
                                                           );
                    *dist_cache.offset(0i32 as (isize)) = sr.distance as (i32);
                    PrepareDistanceCacheH2(hasher,dist_cache);
                }
                InitCommand(
                    {
                        let _old = commands;
                        commands = commands.offset(1 as (isize));
                        _old
                    },
                    insert_length,
                    sr.len,
                    sr.len ^ sr.len_x_code,
                    distance_code
                );
            }
            *num_literals = (*num_literals).wrapping_add(insert_length);
            insert_length = 0i32 as (usize);
            StoreRangeH2(
                hasher,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_add(2i32 as (usize)),
                brotli_min_size_t(position.wrapping_add(sr.len),store_end)
            );
            position = position.wrapping_add(sr.len);
        } else {
            insert_length = insert_length.wrapping_add(1 as (usize));
            position = position.wrapping_add(1 as (usize));
            if position > apply_random_heuristics {
                if position > apply_random_heuristics.wrapping_add(
                                  (4i32 as (usize)).wrapping_mul(random_heuristics_window_size)
                              ) {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH2().wrapping_sub(1i32 as (usize)),
                              4i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(16i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH2(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(4i32 as (usize));
                        }
                        position = position.wrapping_add(4i32 as (usize));
                    }
                } else {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH2().wrapping_sub(1i32 as (usize)),
                              2i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(8i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH2(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(2i32 as (usize));
                        }
                        position = position.wrapping_add(2i32 as (usize));
                    }
                }
            }
        }
    }
    insert_length = insert_length.wrapping_add(
                        pos_end.wrapping_sub(position)
                    );
    *last_insert_len = insert_length;
    *num_commands = (*num_commands).wrapping_add(
                        ((commands as (isize)).wrapping_sub(
                             orig_commands as (isize)
                         ) / std::mem::size_of::<*const Command>() as (isize)) as (usize)
                    );
}

unsafe extern fn StoreLookaheadH3() -> usize { 8i32 as (usize) }

unsafe extern fn PrepareDistanceCacheH3(
    mut handle : *mut u8, mut distance_cache : *mut i32
) {
    handle;
    distance_cache;
}

unsafe extern fn HashTypeLengthH3() -> usize { 8i32 as (usize) }

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H3 {
    pub buckets_ : *mut u32,
}

unsafe extern fn SelfH3(mut handle : *mut u8) -> *mut H3 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct1) as (*mut H3)
}

unsafe extern fn HashBytesH3(mut data : *const u8) -> u32 {
    let h
        : usize
        = (BROTLI_UNALIGNED_LOAD64(
               data as (*const std::os::raw::c_void)
           ) << 64i32 - 8i32 * 5i32).wrapping_mul(
              kHashMul64
          );
    (h >> 64i32 - 16i32) as (u32)
}

unsafe extern fn FindLongestMatchH3(
    mut handle : *mut u8,
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut data : *const u8,
    ring_buffer_mask : usize,
    mut distance_cache : *const i32,
    cur_ix : usize,
    max_length : usize,
    max_backward : usize,
    mut out : *mut HasherSearchResult
) -> i32 {
    let mut self : *mut H3 = SelfH3(handle);
    let best_len_in : usize = (*out).len;
    let cur_ix_masked : usize = cur_ix & ring_buffer_mask;
    let key
        : u32
        = HashBytesH3(
              &*data.offset(cur_ix_masked as (isize)) as (*const u8)
          );
    let mut compare_char
        : i32
        = *data.offset(
               cur_ix_masked.wrapping_add(best_len_in) as (isize)
           ) as (i32);
    let mut best_score : usize = (*out).score;
    let mut best_len : usize = best_len_in;
    let mut cached_backward
        : usize
        = *distance_cache.offset(0i32 as (isize)) as (usize);
    let mut prev_ix : usize = cur_ix.wrapping_sub(cached_backward);
    let mut is_match_found : i32 = 0i32;
    (*out).len_x_code = 0i32 as (usize);
    if prev_ix < cur_ix {
        prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
        if compare_char == *data.offset(
                                prev_ix.wrapping_add(best_len) as (isize)
                            ) as (i32) {
            let mut len
                : usize
                = FindMatchLengthWithLimit(
                      &*data.offset(prev_ix as (isize)) as (*const u8),
                      &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                      max_length
                  );
            if len >= 4i32 as (usize) {
                best_score = BackwardReferenceScoreUsingLastDistance(len);
                best_len = len;
                (*out).len = len;
                (*out).distance = cached_backward;
                (*out).score = best_score;
                compare_char = *data.offset(
                                    cur_ix_masked.wrapping_add(best_len) as (isize)
                                ) as (i32);
                if 2i32 == 1i32 {
                    *(*self).buckets_.offset(key as (isize)) = cur_ix as (u32);
                    return 1i32;
                } else {
                    is_match_found = 1i32;
                }
            }
        }
    }
    if 2i32 == 1i32 {
        let mut backward : usize;
        let mut len : usize;
        prev_ix = *(*self).buckets_.offset(key as (isize)) as (usize);
        *(*self).buckets_.offset(key as (isize)) = cur_ix as (u32);
        backward = cur_ix.wrapping_sub(prev_ix);
        prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
        if compare_char != *data.offset(
                                prev_ix.wrapping_add(best_len_in) as (isize)
                            ) as (i32) {
            return 0i32;
        }
        if backward == 0i32 as (usize) || backward > max_backward {
            return 0i32;
        }
        len = FindMatchLengthWithLimit(
                  &*data.offset(prev_ix as (isize)) as (*const u8),
                  &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                  max_length
              );
        if len >= 4i32 as (usize) {
            (*out).len = len;
            (*out).distance = backward;
            (*out).score = BackwardReferenceScore(len,backward);
            return 1i32;
        }
    } else {
        let mut bucket
            : *mut u32
            = (*self).buckets_.offset(key as (isize));
        let mut i : i32;
        prev_ix = *{
                       let _old = bucket;
                       bucket = bucket.offset(1 as (isize));
                       _old
                   } as (usize);
        i = 0i32;
        while i < 2i32 {
            'continue15: loop {
                {
                    let backward : usize = cur_ix.wrapping_sub(prev_ix);
                    let mut len : usize;
                    prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
                    if compare_char != *data.offset(
                                            prev_ix.wrapping_add(best_len) as (isize)
                                        ) as (i32) {
                        if 1337i32 != 0 {
                            break 'continue15;
                        }
                    }
                    if backward == 0i32 as (usize) || backward > max_backward {
                        if 1337i32 != 0 {
                            break 'continue15;
                        }
                    }
                    len = FindMatchLengthWithLimit(
                              &*data.offset(prev_ix as (isize)) as (*const u8),
                              &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                              max_length
                          );
                    if len >= 4i32 as (usize) {
                        let score : usize = BackwardReferenceScore(len,backward);
                        if best_score < score {
                            best_score = score;
                            best_len = len;
                            (*out).len = best_len;
                            (*out).distance = backward;
                            (*out).score = score;
                            compare_char = *data.offset(
                                                cur_ix_masked.wrapping_add(best_len) as (isize)
                                            ) as (i32);
                            is_match_found = 1i32;
                        }
                    }
                }
                break;
            }
            i = i + 1;
            prev_ix = *{
                           let _old = bucket;
                           bucket = bucket.offset(1 as (isize));
                           _old
                       } as (usize);
        }
    }
    if 0i32 != 0 && (is_match_found == 0) {
        is_match_found = SearchInStaticDictionary(
                             dictionary,
                             dictionary_hash,
                             handle,
                             &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                             max_length,
                             max_backward,
                             out,
                             1i32
                         );
    }
    *(*self).buckets_.offset(
         (key as (usize)).wrapping_add(
             (cur_ix >> 3i32).wrapping_rem(2i32 as (usize))
         ) as (isize)
     ) = cur_ix as (u32);
    is_match_found
}

unsafe extern fn StoreH3(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let key
        : u32
        = HashBytesH3(
              &*data.offset((ix & mask) as (isize)) as (*const u8)
          );
    let off
        : u32
        = (ix >> 3i32).wrapping_rem(2i32 as (usize)) as (u32);
    *(*SelfH3(handle)).buckets_.offset(
         key.wrapping_add(off) as (isize)
     ) = ix as (u32);
}

unsafe extern fn StoreRangeH3(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix_start : usize,
    ix_end : usize
) {
    let mut i : usize;
    i = ix_start;
    while i < ix_end {
        {
            StoreH3(handle,data,mask,i);
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn CreateBackwardReferencesH3(
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    mut hasher : *mut u8,
    mut dist_cache : *mut i32,
    mut last_insert_len : *mut usize,
    mut commands : *mut Command,
    mut num_commands : *mut usize,
    mut num_literals : *mut usize
) {
    let max_backward_limit
        : usize
        = (1i32 as (usize) << (*params).lgwin).wrapping_sub(
              16i32 as (usize)
          );
    let orig_commands : *const Command = commands as (*const Command);
    let mut insert_length : usize = *last_insert_len;
    let pos_end : usize = position.wrapping_add(num_bytes);
    let store_end
        : usize
        = if num_bytes >= StoreLookaheadH3() {
              position.wrapping_add(num_bytes).wrapping_sub(
                  StoreLookaheadH3()
              ).wrapping_add(
                  1i32 as (usize)
              )
          } else {
              position
          };
    let random_heuristics_window_size
        : usize
        = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics
        : usize
        = position.wrapping_add(random_heuristics_window_size);
    let kMinScore
        : usize
        = ((30i32 * 8i32) as (usize)).wrapping_mul(
              std::mem::size_of::<usize>()
          ).wrapping_add(
              100i32 as (usize)
          );
    PrepareDistanceCacheH3(hasher,dist_cache);
    while position.wrapping_add(HashTypeLengthH3()) < pos_end {
        let mut max_length : usize = pos_end.wrapping_sub(position);
        let mut max_distance
            : usize
            = brotli_min_size_t(position,max_backward_limit);
        let mut sr : HasherSearchResult;
        sr.len = 0i32 as (usize);
        sr.len_x_code = 0i32 as (usize);
        sr.distance = 0i32 as (usize);
        sr.score = kMinScore;
        if FindLongestMatchH3(
               hasher,
               dictionary,
               dictionary_hash,
               ringbuffer,
               ringbuffer_mask,
               dist_cache as (*const i32),
               position,
               max_length,
               max_distance,
               &mut sr as (*mut HasherSearchResult)
           ) != 0 {
            let mut delayed_backward_references_in_row : i32 = 0i32;
            max_length = max_length.wrapping_sub(1 as (usize));
            'break16: loop {
                'continue17: loop {
                    {
                        let cost_diff_lazy : usize = 175i32 as (usize);
                        let mut is_match_found : i32;
                        let mut sr2 : HasherSearchResult;
                        sr2.len = if (*params).quality < 5i32 {
                                      brotli_min_size_t(
                                          sr.len.wrapping_sub(1i32 as (usize)),
                                          max_length
                                      )
                                  } else {
                                      0i32 as (usize)
                                  };
                        sr2.len_x_code = 0i32 as (usize);
                        sr2.distance = 0i32 as (usize);
                        sr2.score = kMinScore;
                        max_distance = brotli_min_size_t(
                                           position.wrapping_add(1i32 as (usize)),
                                           max_backward_limit
                                       );
                        is_match_found = FindLongestMatchH3(
                                             hasher,
                                             dictionary,
                                             dictionary_hash,
                                             ringbuffer,
                                             ringbuffer_mask,
                                             dist_cache as (*const i32),
                                             position.wrapping_add(1i32 as (usize)),
                                             max_length,
                                             max_distance,
                                             &mut sr2 as (*mut HasherSearchResult)
                                         );
                        if is_match_found != 0 && (sr2.score >= sr.score.wrapping_add(
                                                                    cost_diff_lazy
                                                                )) {
                            position = position.wrapping_add(1 as (usize));
                            insert_length = insert_length.wrapping_add(1 as (usize));
                            sr = sr2;
                            if {
                                   delayed_backward_references_in_row = delayed_backward_references_in_row + 1;
                                   delayed_backward_references_in_row
                               } < 4i32 && (position.wrapping_add(HashTypeLengthH3()) < pos_end) {
                                if 1337i32 != 0 {
                                    break 'continue17;
                                }
                            }
                        }
                        {
                            if 1337i32 != 0 {
                                break 'break16;
                            }
                        }
                    }
                    break;
                }
                max_length = max_length.wrapping_sub(1 as (usize));
            }
            apply_random_heuristics = position.wrapping_add(
                                          (2i32 as (usize)).wrapping_mul(sr.len)
                                      ).wrapping_add(
                                          random_heuristics_window_size
                                      );
            max_distance = brotli_min_size_t(position,max_backward_limit);
            {
                let mut distance_code
                    : usize
                    = ComputeDistanceCode(
                          sr.distance,
                          max_distance,
                          dist_cache as (*const i32)
                      );
                if sr.distance <= max_distance && (distance_code > 0i32 as (usize)) {
                    *dist_cache.offset(3i32 as (isize)) = *dist_cache.offset(
                                                               2i32 as (isize)
                                                           );
                    *dist_cache.offset(2i32 as (isize)) = *dist_cache.offset(
                                                               1i32 as (isize)
                                                           );
                    *dist_cache.offset(1i32 as (isize)) = *dist_cache.offset(
                                                               0i32 as (isize)
                                                           );
                    *dist_cache.offset(0i32 as (isize)) = sr.distance as (i32);
                    PrepareDistanceCacheH3(hasher,dist_cache);
                }
                InitCommand(
                    {
                        let _old = commands;
                        commands = commands.offset(1 as (isize));
                        _old
                    },
                    insert_length,
                    sr.len,
                    sr.len ^ sr.len_x_code,
                    distance_code
                );
            }
            *num_literals = (*num_literals).wrapping_add(insert_length);
            insert_length = 0i32 as (usize);
            StoreRangeH3(
                hasher,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_add(2i32 as (usize)),
                brotli_min_size_t(position.wrapping_add(sr.len),store_end)
            );
            position = position.wrapping_add(sr.len);
        } else {
            insert_length = insert_length.wrapping_add(1 as (usize));
            position = position.wrapping_add(1 as (usize));
            if position > apply_random_heuristics {
                if position > apply_random_heuristics.wrapping_add(
                                  (4i32 as (usize)).wrapping_mul(random_heuristics_window_size)
                              ) {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH3().wrapping_sub(1i32 as (usize)),
                              4i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(16i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH3(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(4i32 as (usize));
                        }
                        position = position.wrapping_add(4i32 as (usize));
                    }
                } else {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH3().wrapping_sub(1i32 as (usize)),
                              2i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(8i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH3(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(2i32 as (usize));
                        }
                        position = position.wrapping_add(2i32 as (usize));
                    }
                }
            }
        }
    }
    insert_length = insert_length.wrapping_add(
                        pos_end.wrapping_sub(position)
                    );
    *last_insert_len = insert_length;
    *num_commands = (*num_commands).wrapping_add(
                        ((commands as (isize)).wrapping_sub(
                             orig_commands as (isize)
                         ) / std::mem::size_of::<*const Command>() as (isize)) as (usize)
                    );
}

unsafe extern fn StoreLookaheadH4() -> usize { 8i32 as (usize) }

unsafe extern fn PrepareDistanceCacheH4(
    mut handle : *mut u8, mut distance_cache : *mut i32
) {
    handle;
    distance_cache;
}

unsafe extern fn HashTypeLengthH4() -> usize { 8i32 as (usize) }

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H4 {
    pub buckets_ : *mut u32,
}

unsafe extern fn SelfH4(mut handle : *mut u8) -> *mut H4 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct1) as (*mut H4)
}

unsafe extern fn HashBytesH4(mut data : *const u8) -> u32 {
    let h
        : usize
        = (BROTLI_UNALIGNED_LOAD64(
               data as (*const std::os::raw::c_void)
           ) << 64i32 - 8i32 * 5i32).wrapping_mul(
              kHashMul64
          );
    (h >> 64i32 - 17i32) as (u32)
}

unsafe extern fn FindLongestMatchH4(
    mut handle : *mut u8,
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut data : *const u8,
    ring_buffer_mask : usize,
    mut distance_cache : *const i32,
    cur_ix : usize,
    max_length : usize,
    max_backward : usize,
    mut out : *mut HasherSearchResult
) -> i32 {
    let mut self : *mut H4 = SelfH4(handle);
    let best_len_in : usize = (*out).len;
    let cur_ix_masked : usize = cur_ix & ring_buffer_mask;
    let key
        : u32
        = HashBytesH4(
              &*data.offset(cur_ix_masked as (isize)) as (*const u8)
          );
    let mut compare_char
        : i32
        = *data.offset(
               cur_ix_masked.wrapping_add(best_len_in) as (isize)
           ) as (i32);
    let mut best_score : usize = (*out).score;
    let mut best_len : usize = best_len_in;
    let mut cached_backward
        : usize
        = *distance_cache.offset(0i32 as (isize)) as (usize);
    let mut prev_ix : usize = cur_ix.wrapping_sub(cached_backward);
    let mut is_match_found : i32 = 0i32;
    (*out).len_x_code = 0i32 as (usize);
    if prev_ix < cur_ix {
        prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
        if compare_char == *data.offset(
                                prev_ix.wrapping_add(best_len) as (isize)
                            ) as (i32) {
            let mut len
                : usize
                = FindMatchLengthWithLimit(
                      &*data.offset(prev_ix as (isize)) as (*const u8),
                      &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                      max_length
                  );
            if len >= 4i32 as (usize) {
                best_score = BackwardReferenceScoreUsingLastDistance(len);
                best_len = len;
                (*out).len = len;
                (*out).distance = cached_backward;
                (*out).score = best_score;
                compare_char = *data.offset(
                                    cur_ix_masked.wrapping_add(best_len) as (isize)
                                ) as (i32);
                if 4i32 == 1i32 {
                    *(*self).buckets_.offset(key as (isize)) = cur_ix as (u32);
                    return 1i32;
                } else {
                    is_match_found = 1i32;
                }
            }
        }
    }
    if 4i32 == 1i32 {
        let mut backward : usize;
        let mut len : usize;
        prev_ix = *(*self).buckets_.offset(key as (isize)) as (usize);
        *(*self).buckets_.offset(key as (isize)) = cur_ix as (u32);
        backward = cur_ix.wrapping_sub(prev_ix);
        prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
        if compare_char != *data.offset(
                                prev_ix.wrapping_add(best_len_in) as (isize)
                            ) as (i32) {
            return 0i32;
        }
        if backward == 0i32 as (usize) || backward > max_backward {
            return 0i32;
        }
        len = FindMatchLengthWithLimit(
                  &*data.offset(prev_ix as (isize)) as (*const u8),
                  &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                  max_length
              );
        if len >= 4i32 as (usize) {
            (*out).len = len;
            (*out).distance = backward;
            (*out).score = BackwardReferenceScore(len,backward);
            return 1i32;
        }
    } else {
        let mut bucket
            : *mut u32
            = (*self).buckets_.offset(key as (isize));
        let mut i : i32;
        prev_ix = *{
                       let _old = bucket;
                       bucket = bucket.offset(1 as (isize));
                       _old
                   } as (usize);
        i = 0i32;
        while i < 4i32 {
            'continue25: loop {
                {
                    let backward : usize = cur_ix.wrapping_sub(prev_ix);
                    let mut len : usize;
                    prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
                    if compare_char != *data.offset(
                                            prev_ix.wrapping_add(best_len) as (isize)
                                        ) as (i32) {
                        if 1337i32 != 0 {
                            break 'continue25;
                        }
                    }
                    if backward == 0i32 as (usize) || backward > max_backward {
                        if 1337i32 != 0 {
                            break 'continue25;
                        }
                    }
                    len = FindMatchLengthWithLimit(
                              &*data.offset(prev_ix as (isize)) as (*const u8),
                              &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                              max_length
                          );
                    if len >= 4i32 as (usize) {
                        let score : usize = BackwardReferenceScore(len,backward);
                        if best_score < score {
                            best_score = score;
                            best_len = len;
                            (*out).len = best_len;
                            (*out).distance = backward;
                            (*out).score = score;
                            compare_char = *data.offset(
                                                cur_ix_masked.wrapping_add(best_len) as (isize)
                                            ) as (i32);
                            is_match_found = 1i32;
                        }
                    }
                }
                break;
            }
            i = i + 1;
            prev_ix = *{
                           let _old = bucket;
                           bucket = bucket.offset(1 as (isize));
                           _old
                       } as (usize);
        }
    }
    if 1i32 != 0 && (is_match_found == 0) {
        is_match_found = SearchInStaticDictionary(
                             dictionary,
                             dictionary_hash,
                             handle,
                             &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                             max_length,
                             max_backward,
                             out,
                             1i32
                         );
    }
    *(*self).buckets_.offset(
         (key as (usize)).wrapping_add(
             (cur_ix >> 3i32).wrapping_rem(4i32 as (usize))
         ) as (isize)
     ) = cur_ix as (u32);
    is_match_found
}

unsafe extern fn StoreH4(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let key
        : u32
        = HashBytesH4(
              &*data.offset((ix & mask) as (isize)) as (*const u8)
          );
    let off
        : u32
        = (ix >> 3i32).wrapping_rem(4i32 as (usize)) as (u32);
    *(*SelfH4(handle)).buckets_.offset(
         key.wrapping_add(off) as (isize)
     ) = ix as (u32);
}

unsafe extern fn StoreRangeH4(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix_start : usize,
    ix_end : usize
) {
    let mut i : usize;
    i = ix_start;
    while i < ix_end {
        {
            StoreH4(handle,data,mask,i);
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn CreateBackwardReferencesH4(
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    mut hasher : *mut u8,
    mut dist_cache : *mut i32,
    mut last_insert_len : *mut usize,
    mut commands : *mut Command,
    mut num_commands : *mut usize,
    mut num_literals : *mut usize
) {
    let max_backward_limit
        : usize
        = (1i32 as (usize) << (*params).lgwin).wrapping_sub(
              16i32 as (usize)
          );
    let orig_commands : *const Command = commands as (*const Command);
    let mut insert_length : usize = *last_insert_len;
    let pos_end : usize = position.wrapping_add(num_bytes);
    let store_end
        : usize
        = if num_bytes >= StoreLookaheadH4() {
              position.wrapping_add(num_bytes).wrapping_sub(
                  StoreLookaheadH4()
              ).wrapping_add(
                  1i32 as (usize)
              )
          } else {
              position
          };
    let random_heuristics_window_size
        : usize
        = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics
        : usize
        = position.wrapping_add(random_heuristics_window_size);
    let kMinScore
        : usize
        = ((30i32 * 8i32) as (usize)).wrapping_mul(
              std::mem::size_of::<usize>()
          ).wrapping_add(
              100i32 as (usize)
          );
    PrepareDistanceCacheH4(hasher,dist_cache);
    while position.wrapping_add(HashTypeLengthH4()) < pos_end {
        let mut max_length : usize = pos_end.wrapping_sub(position);
        let mut max_distance
            : usize
            = brotli_min_size_t(position,max_backward_limit);
        let mut sr : HasherSearchResult;
        sr.len = 0i32 as (usize);
        sr.len_x_code = 0i32 as (usize);
        sr.distance = 0i32 as (usize);
        sr.score = kMinScore;
        if FindLongestMatchH4(
               hasher,
               dictionary,
               dictionary_hash,
               ringbuffer,
               ringbuffer_mask,
               dist_cache as (*const i32),
               position,
               max_length,
               max_distance,
               &mut sr as (*mut HasherSearchResult)
           ) != 0 {
            let mut delayed_backward_references_in_row : i32 = 0i32;
            max_length = max_length.wrapping_sub(1 as (usize));
            'break26: loop {
                'continue27: loop {
                    {
                        let cost_diff_lazy : usize = 175i32 as (usize);
                        let mut is_match_found : i32;
                        let mut sr2 : HasherSearchResult;
                        sr2.len = if (*params).quality < 5i32 {
                                      brotli_min_size_t(
                                          sr.len.wrapping_sub(1i32 as (usize)),
                                          max_length
                                      )
                                  } else {
                                      0i32 as (usize)
                                  };
                        sr2.len_x_code = 0i32 as (usize);
                        sr2.distance = 0i32 as (usize);
                        sr2.score = kMinScore;
                        max_distance = brotli_min_size_t(
                                           position.wrapping_add(1i32 as (usize)),
                                           max_backward_limit
                                       );
                        is_match_found = FindLongestMatchH4(
                                             hasher,
                                             dictionary,
                                             dictionary_hash,
                                             ringbuffer,
                                             ringbuffer_mask,
                                             dist_cache as (*const i32),
                                             position.wrapping_add(1i32 as (usize)),
                                             max_length,
                                             max_distance,
                                             &mut sr2 as (*mut HasherSearchResult)
                                         );
                        if is_match_found != 0 && (sr2.score >= sr.score.wrapping_add(
                                                                    cost_diff_lazy
                                                                )) {
                            position = position.wrapping_add(1 as (usize));
                            insert_length = insert_length.wrapping_add(1 as (usize));
                            sr = sr2;
                            if {
                                   delayed_backward_references_in_row = delayed_backward_references_in_row + 1;
                                   delayed_backward_references_in_row
                               } < 4i32 && (position.wrapping_add(HashTypeLengthH4()) < pos_end) {
                                if 1337i32 != 0 {
                                    break 'continue27;
                                }
                            }
                        }
                        {
                            if 1337i32 != 0 {
                                break 'break26;
                            }
                        }
                    }
                    break;
                }
                max_length = max_length.wrapping_sub(1 as (usize));
            }
            apply_random_heuristics = position.wrapping_add(
                                          (2i32 as (usize)).wrapping_mul(sr.len)
                                      ).wrapping_add(
                                          random_heuristics_window_size
                                      );
            max_distance = brotli_min_size_t(position,max_backward_limit);
            {
                let mut distance_code
                    : usize
                    = ComputeDistanceCode(
                          sr.distance,
                          max_distance,
                          dist_cache as (*const i32)
                      );
                if sr.distance <= max_distance && (distance_code > 0i32 as (usize)) {
                    *dist_cache.offset(3i32 as (isize)) = *dist_cache.offset(
                                                               2i32 as (isize)
                                                           );
                    *dist_cache.offset(2i32 as (isize)) = *dist_cache.offset(
                                                               1i32 as (isize)
                                                           );
                    *dist_cache.offset(1i32 as (isize)) = *dist_cache.offset(
                                                               0i32 as (isize)
                                                           );
                    *dist_cache.offset(0i32 as (isize)) = sr.distance as (i32);
                    PrepareDistanceCacheH4(hasher,dist_cache);
                }
                InitCommand(
                    {
                        let _old = commands;
                        commands = commands.offset(1 as (isize));
                        _old
                    },
                    insert_length,
                    sr.len,
                    sr.len ^ sr.len_x_code,
                    distance_code
                );
            }
            *num_literals = (*num_literals).wrapping_add(insert_length);
            insert_length = 0i32 as (usize);
            StoreRangeH4(
                hasher,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_add(2i32 as (usize)),
                brotli_min_size_t(position.wrapping_add(sr.len),store_end)
            );
            position = position.wrapping_add(sr.len);
        } else {
            insert_length = insert_length.wrapping_add(1 as (usize));
            position = position.wrapping_add(1 as (usize));
            if position > apply_random_heuristics {
                if position > apply_random_heuristics.wrapping_add(
                                  (4i32 as (usize)).wrapping_mul(random_heuristics_window_size)
                              ) {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH4().wrapping_sub(1i32 as (usize)),
                              4i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(16i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH4(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(4i32 as (usize));
                        }
                        position = position.wrapping_add(4i32 as (usize));
                    }
                } else {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH4().wrapping_sub(1i32 as (usize)),
                              2i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(8i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH4(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(2i32 as (usize));
                        }
                        position = position.wrapping_add(2i32 as (usize));
                    }
                }
            }
        }
    }
    insert_length = insert_length.wrapping_add(
                        pos_end.wrapping_sub(position)
                    );
    *last_insert_len = insert_length;
    *num_commands = (*num_commands).wrapping_add(
                        ((commands as (isize)).wrapping_sub(
                             orig_commands as (isize)
                         ) / std::mem::size_of::<*const Command>() as (isize)) as (usize)
                    );
}

unsafe extern fn StoreLookaheadH5() -> usize { 4i32 as (usize) }

unsafe extern fn PrepareDistanceCache(
    mut distance_cache : *mut i32, num_distances : i32
) { if num_distances > 4i32 {
        let mut last_distance
            : i32
            = *distance_cache.offset(0i32 as (isize));
        *distance_cache.offset(4i32 as (isize)) = last_distance - 1i32;
        *distance_cache.offset(5i32 as (isize)) = last_distance + 1i32;
        *distance_cache.offset(6i32 as (isize)) = last_distance - 2i32;
        *distance_cache.offset(7i32 as (isize)) = last_distance + 2i32;
        *distance_cache.offset(8i32 as (isize)) = last_distance - 3i32;
        *distance_cache.offset(9i32 as (isize)) = last_distance + 3i32;
        if num_distances > 10i32 {
            let mut next_last_distance
                : i32
                = *distance_cache.offset(1i32 as (isize));
            *distance_cache.offset(
                 10i32 as (isize)
             ) = next_last_distance - 1i32;
            *distance_cache.offset(
                 11i32 as (isize)
             ) = next_last_distance + 1i32;
            *distance_cache.offset(
                 12i32 as (isize)
             ) = next_last_distance - 2i32;
            *distance_cache.offset(
                 13i32 as (isize)
             ) = next_last_distance + 2i32;
            *distance_cache.offset(
                 14i32 as (isize)
             ) = next_last_distance - 3i32;
            *distance_cache.offset(
                 15i32 as (isize)
             ) = next_last_distance + 3i32;
        }
    }
}

unsafe extern fn PrepareDistanceCacheH5(
    mut handle : *mut u8, mut distance_cache : *mut i32
) {
    PrepareDistanceCache(
        distance_cache,
        (*GetHasherCommon(handle)).params.num_last_distances_to_check
    );
}

unsafe extern fn HashTypeLengthH5() -> usize { 4i32 as (usize) }

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H5 {
    pub bucket_size_ : usize,
    pub block_size_ : usize,
    pub hash_shift_ : i32,
    pub block_mask_ : u32,
}

unsafe extern fn SelfH5(mut handle : *mut u8) -> *mut H5 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct1) as (*mut H5)
}

unsafe extern fn NumH5(mut self : *mut H5) -> *mut u16 {
    &mut *self.offset(1i32 as (isize)) as (*mut H5) as (*mut u16)
}

unsafe extern fn BucketsH5(mut self : *mut H5) -> *mut u32 {
    &mut *NumH5(self).offset(
              (*self).bucket_size_ as (isize)
          ) as (*mut u16) as (*mut u32)
}

unsafe extern fn BackwardReferencePenaltyUsingLastDistance(
    mut distance_short_code : usize
) -> usize {
    (39i32 as (usize)).wrapping_add(
        (0x1ca10i32 >> (distance_short_code & 0xei32 as (usize)) & 0xei32) as (usize)
    )
}

unsafe extern fn HashBytesH5(
    mut data : *const u8, shift : i32
) -> u32 {
    let mut h
        : u32
        = BROTLI_UNALIGNED_LOAD32(
              data as (*const std::os::raw::c_void)
          ).wrapping_mul(
              kHashMul32
          );
    (h >> shift) as (u32)
}

unsafe extern fn FindLongestMatchH5(
    mut handle : *mut u8,
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut data : *const u8,
    ring_buffer_mask : usize,
    mut distance_cache : *const i32,
    cur_ix : usize,
    max_length : usize,
    max_backward : usize,
    mut out : *mut HasherSearchResult
) -> i32 {
    let mut common : *mut Struct1 = GetHasherCommon(handle);
    let mut self : *mut H5 = SelfH5(handle);
    let mut num : *mut u16 = NumH5(self);
    let mut buckets : *mut u32 = BucketsH5(self);
    let cur_ix_masked : usize = cur_ix & ring_buffer_mask;
    let mut is_match_found : i32 = 0i32;
    let mut best_score : usize = (*out).score;
    let mut best_len : usize = (*out).len;
    let mut i : usize;
    (*out).len = 0i32 as (usize);
    (*out).len_x_code = 0i32 as (usize);
    i = 0i32 as (usize);
    while i < (*common).params.num_last_distances_to_check as (usize) {
        'continue35: loop {
            {
                let backward
                    : usize
                    = *distance_cache.offset(i as (isize)) as (usize);
                let mut prev_ix : usize = cur_ix.wrapping_sub(backward) as (usize);
                if prev_ix >= cur_ix {
                    if 1337i32 != 0 {
                        break 'continue35;
                    }
                }
                if backward > max_backward {
                    if 1337i32 != 0 {
                        break 'continue35;
                    }
                }
                prev_ix = prev_ix & ring_buffer_mask;
                if cur_ix_masked.wrapping_add(
                       best_len
                   ) > ring_buffer_mask || prev_ix.wrapping_add(
                                               best_len
                                           ) > ring_buffer_mask || *data.offset(
                                                                        cur_ix_masked.wrapping_add(
                                                                            best_len
                                                                        ) as (isize)
                                                                    ) as (i32) != *data.offset(
                                                                                       prev_ix.wrapping_add(
                                                                                           best_len
                                                                                       ) as (isize)
                                                                                   ) as (i32) {
                    if 1337i32 != 0 {
                        break 'continue35;
                    }
                }
                {
                    let len
                        : usize
                        = FindMatchLengthWithLimit(
                              &*data.offset(prev_ix as (isize)) as (*const u8),
                              &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                              max_length
                          );
                    if len >= 3i32 as (usize) || len == 2i32 as (usize) && (i < 2i32 as (usize)) {
                        let mut score
                            : usize
                            = BackwardReferenceScoreUsingLastDistance(len);
                        if best_score < score {
                            if i != 0i32 as (usize) {
                                score = score.wrapping_sub(
                                            BackwardReferencePenaltyUsingLastDistance(i)
                                        );
                            }
                            if best_score < score {
                                best_score = score;
                                best_len = len;
                                (*out).len = best_len;
                                (*out).distance = backward;
                                (*out).score = best_score;
                                is_match_found = 1i32;
                            }
                        }
                    }
                }
            }
            break;
        }
        i = i.wrapping_add(1 as (usize));
    }
    {
        let key
            : u32
            = HashBytesH5(
                  &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                  (*self).hash_shift_
              );
        let mut bucket
            : *mut u32
            = &mut *buckets.offset(
                        (key << (*common).params.block_bits) as (isize)
                    ) as (*mut u32);
        let down
            : usize
            = if *num.offset(key as (isize)) as (usize) > (*self).block_size_ {
                  (*num.offset(key as (isize)) as (usize)).wrapping_sub(
                      (*self).block_size_
                  )
              } else {
                  0u32 as (usize)
              };
        i = *num.offset(key as (isize)) as (usize);
        while i > down {
            let mut prev_ix
                : usize
                = *bucket.offset(
                       ({
                            i = i.wrapping_sub(1 as (usize));
                            i
                        } & (*self).block_mask_ as (usize)) as (isize)
                   ) as (usize);
            let backward : usize = cur_ix.wrapping_sub(prev_ix);
            if backward > max_backward {
                if 1337i32 != 0 {
                    break;
                }
            }
            prev_ix = prev_ix & ring_buffer_mask;
            if cur_ix_masked.wrapping_add(
                   best_len
               ) > ring_buffer_mask || prev_ix.wrapping_add(
                                           best_len
                                       ) > ring_buffer_mask || *data.offset(
                                                                    cur_ix_masked.wrapping_add(
                                                                        best_len
                                                                    ) as (isize)
                                                                ) as (i32) != *data.offset(
                                                                                   prev_ix.wrapping_add(
                                                                                       best_len
                                                                                   ) as (isize)
                                                                               ) as (i32) {
                if 1337i32 != 0 {
                    continue;
                }
            }
            {
                let len
                    : usize
                    = FindMatchLengthWithLimit(
                          &*data.offset(prev_ix as (isize)) as (*const u8),
                          &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                          max_length
                      );
                if len >= 4i32 as (usize) {
                    let mut score : usize = BackwardReferenceScore(len,backward);
                    if best_score < score {
                        best_score = score;
                        best_len = len;
                        (*out).len = best_len;
                        (*out).distance = backward;
                        (*out).score = best_score;
                        is_match_found = 1i32;
                    }
                }
            }
        }
        *bucket.offset(
             (*num.offset(
                   key as (isize)
               ) as (u32) & (*self).block_mask_) as (isize)
         ) = cur_ix as (u32);
        {
            let _rhs = 1;
            let _lhs = &mut *num.offset(key as (isize));
            *_lhs = (*_lhs as (i32) + _rhs) as (u16);
        }
    }
    if is_match_found == 0 {
        is_match_found = SearchInStaticDictionary(
                             dictionary,
                             dictionary_hash,
                             handle,
                             &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                             max_length,
                             max_backward,
                             out,
                             0i32
                         );
    }
    is_match_found
}

unsafe extern fn StoreH5(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let mut self : *mut H5 = SelfH5(handle);
    let mut num : *mut u16 = NumH5(self);
    let key
        : u32
        = HashBytesH5(
              &*data.offset((ix & mask) as (isize)) as (*const u8),
              (*self).hash_shift_
          );
    let minor_ix
        : usize
        = (*num.offset(
                key as (isize)
            ) as (u32) & (*self).block_mask_) as (usize);
    let offset
        : usize
        = minor_ix.wrapping_add(
              (key << (*GetHasherCommon(handle)).params.block_bits) as (usize)
          );
    *BucketsH5(self).offset(offset as (isize)) = ix as (u32);
    {
        let _rhs = 1;
        let _lhs = &mut *num.offset(key as (isize));
        *_lhs = (*_lhs as (i32) + _rhs) as (u16);
    }
}

unsafe extern fn StoreRangeH5(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix_start : usize,
    ix_end : usize
) {
    let mut i : usize;
    i = ix_start;
    while i < ix_end {
        {
            StoreH5(handle,data,mask,i);
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn CreateBackwardReferencesH5(
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    mut hasher : *mut u8,
    mut dist_cache : *mut i32,
    mut last_insert_len : *mut usize,
    mut commands : *mut Command,
    mut num_commands : *mut usize,
    mut num_literals : *mut usize
) {
    let max_backward_limit
        : usize
        = (1i32 as (usize) << (*params).lgwin).wrapping_sub(
              16i32 as (usize)
          );
    let orig_commands : *const Command = commands as (*const Command);
    let mut insert_length : usize = *last_insert_len;
    let pos_end : usize = position.wrapping_add(num_bytes);
    let store_end
        : usize
        = if num_bytes >= StoreLookaheadH5() {
              position.wrapping_add(num_bytes).wrapping_sub(
                  StoreLookaheadH5()
              ).wrapping_add(
                  1i32 as (usize)
              )
          } else {
              position
          };
    let random_heuristics_window_size
        : usize
        = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics
        : usize
        = position.wrapping_add(random_heuristics_window_size);
    let kMinScore
        : usize
        = ((30i32 * 8i32) as (usize)).wrapping_mul(
              std::mem::size_of::<usize>()
          ).wrapping_add(
              100i32 as (usize)
          );
    PrepareDistanceCacheH5(hasher,dist_cache);
    while position.wrapping_add(HashTypeLengthH5()) < pos_end {
        let mut max_length : usize = pos_end.wrapping_sub(position);
        let mut max_distance
            : usize
            = brotli_min_size_t(position,max_backward_limit);
        let mut sr : HasherSearchResult;
        sr.len = 0i32 as (usize);
        sr.len_x_code = 0i32 as (usize);
        sr.distance = 0i32 as (usize);
        sr.score = kMinScore;
        if FindLongestMatchH5(
               hasher,
               dictionary,
               dictionary_hash,
               ringbuffer,
               ringbuffer_mask,
               dist_cache as (*const i32),
               position,
               max_length,
               max_distance,
               &mut sr as (*mut HasherSearchResult)
           ) != 0 {
            let mut delayed_backward_references_in_row : i32 = 0i32;
            max_length = max_length.wrapping_sub(1 as (usize));
            'break36: loop {
                'continue37: loop {
                    {
                        let cost_diff_lazy : usize = 175i32 as (usize);
                        let mut is_match_found : i32;
                        let mut sr2 : HasherSearchResult;
                        sr2.len = if (*params).quality < 5i32 {
                                      brotli_min_size_t(
                                          sr.len.wrapping_sub(1i32 as (usize)),
                                          max_length
                                      )
                                  } else {
                                      0i32 as (usize)
                                  };
                        sr2.len_x_code = 0i32 as (usize);
                        sr2.distance = 0i32 as (usize);
                        sr2.score = kMinScore;
                        max_distance = brotli_min_size_t(
                                           position.wrapping_add(1i32 as (usize)),
                                           max_backward_limit
                                       );
                        is_match_found = FindLongestMatchH5(
                                             hasher,
                                             dictionary,
                                             dictionary_hash,
                                             ringbuffer,
                                             ringbuffer_mask,
                                             dist_cache as (*const i32),
                                             position.wrapping_add(1i32 as (usize)),
                                             max_length,
                                             max_distance,
                                             &mut sr2 as (*mut HasherSearchResult)
                                         );
                        if is_match_found != 0 && (sr2.score >= sr.score.wrapping_add(
                                                                    cost_diff_lazy
                                                                )) {
                            position = position.wrapping_add(1 as (usize));
                            insert_length = insert_length.wrapping_add(1 as (usize));
                            sr = sr2;
                            if {
                                   delayed_backward_references_in_row = delayed_backward_references_in_row + 1;
                                   delayed_backward_references_in_row
                               } < 4i32 && (position.wrapping_add(HashTypeLengthH5()) < pos_end) {
                                if 1337i32 != 0 {
                                    break 'continue37;
                                }
                            }
                        }
                        {
                            if 1337i32 != 0 {
                                break 'break36;
                            }
                        }
                    }
                    break;
                }
                max_length = max_length.wrapping_sub(1 as (usize));
            }
            apply_random_heuristics = position.wrapping_add(
                                          (2i32 as (usize)).wrapping_mul(sr.len)
                                      ).wrapping_add(
                                          random_heuristics_window_size
                                      );
            max_distance = brotli_min_size_t(position,max_backward_limit);
            {
                let mut distance_code
                    : usize
                    = ComputeDistanceCode(
                          sr.distance,
                          max_distance,
                          dist_cache as (*const i32)
                      );
                if sr.distance <= max_distance && (distance_code > 0i32 as (usize)) {
                    *dist_cache.offset(3i32 as (isize)) = *dist_cache.offset(
                                                               2i32 as (isize)
                                                           );
                    *dist_cache.offset(2i32 as (isize)) = *dist_cache.offset(
                                                               1i32 as (isize)
                                                           );
                    *dist_cache.offset(1i32 as (isize)) = *dist_cache.offset(
                                                               0i32 as (isize)
                                                           );
                    *dist_cache.offset(0i32 as (isize)) = sr.distance as (i32);
                    PrepareDistanceCacheH5(hasher,dist_cache);
                }
                InitCommand(
                    {
                        let _old = commands;
                        commands = commands.offset(1 as (isize));
                        _old
                    },
                    insert_length,
                    sr.len,
                    sr.len ^ sr.len_x_code,
                    distance_code
                );
            }
            *num_literals = (*num_literals).wrapping_add(insert_length);
            insert_length = 0i32 as (usize);
            StoreRangeH5(
                hasher,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_add(2i32 as (usize)),
                brotli_min_size_t(position.wrapping_add(sr.len),store_end)
            );
            position = position.wrapping_add(sr.len);
        } else {
            insert_length = insert_length.wrapping_add(1 as (usize));
            position = position.wrapping_add(1 as (usize));
            if position > apply_random_heuristics {
                if position > apply_random_heuristics.wrapping_add(
                                  (4i32 as (usize)).wrapping_mul(random_heuristics_window_size)
                              ) {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH5().wrapping_sub(1i32 as (usize)),
                              4i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(16i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH5(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(4i32 as (usize));
                        }
                        position = position.wrapping_add(4i32 as (usize));
                    }
                } else {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH5().wrapping_sub(1i32 as (usize)),
                              2i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(8i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH5(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(2i32 as (usize));
                        }
                        position = position.wrapping_add(2i32 as (usize));
                    }
                }
            }
        }
    }
    insert_length = insert_length.wrapping_add(
                        pos_end.wrapping_sub(position)
                    );
    *last_insert_len = insert_length;
    *num_commands = (*num_commands).wrapping_add(
                        ((commands as (isize)).wrapping_sub(
                             orig_commands as (isize)
                         ) / std::mem::size_of::<*const Command>() as (isize)) as (usize)
                    );
}

unsafe extern fn StoreLookaheadH6() -> usize { 8i32 as (usize) }

unsafe extern fn PrepareDistanceCacheH6(
    mut handle : *mut u8, mut distance_cache : *mut i32
) {
    PrepareDistanceCache(
        distance_cache,
        (*GetHasherCommon(handle)).params.num_last_distances_to_check
    );
}

unsafe extern fn HashTypeLengthH6() -> usize { 8i32 as (usize) }

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H6 {
    pub bucket_size_ : usize,
    pub block_size_ : usize,
    pub hash_shift_ : i32,
    pub hash_mask_ : usize,
    pub block_mask_ : u32,
}

unsafe extern fn SelfH6(mut handle : *mut u8) -> *mut H6 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct1) as (*mut H6)
}

unsafe extern fn NumH6(mut self : *mut H6) -> *mut u16 {
    &mut *self.offset(1i32 as (isize)) as (*mut H6) as (*mut u16)
}

unsafe extern fn BucketsH6(mut self : *mut H6) -> *mut u32 {
    &mut *NumH6(self).offset(
              (*self).bucket_size_ as (isize)
          ) as (*mut u16) as (*mut u32)
}

unsafe extern fn HashBytesH6(
    mut data : *const u8, mask : usize, shift : i32
) -> u32 {
    let h
        : usize
        = (BROTLI_UNALIGNED_LOAD64(
               data as (*const std::os::raw::c_void)
           ) & mask).wrapping_mul(
              kHashMul64Long
          );
    (h >> shift) as (u32)
}

unsafe extern fn FindLongestMatchH6(
    mut handle : *mut u8,
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut data : *const u8,
    ring_buffer_mask : usize,
    mut distance_cache : *const i32,
    cur_ix : usize,
    max_length : usize,
    max_backward : usize,
    mut out : *mut HasherSearchResult
) -> i32 {
    let mut common : *mut Struct1 = GetHasherCommon(handle);
    let mut self : *mut H6 = SelfH6(handle);
    let mut num : *mut u16 = NumH6(self);
    let mut buckets : *mut u32 = BucketsH6(self);
    let cur_ix_masked : usize = cur_ix & ring_buffer_mask;
    let mut is_match_found : i32 = 0i32;
    let mut best_score : usize = (*out).score;
    let mut best_len : usize = (*out).len;
    let mut i : usize;
    (*out).len = 0i32 as (usize);
    (*out).len_x_code = 0i32 as (usize);
    i = 0i32 as (usize);
    while i < (*common).params.num_last_distances_to_check as (usize) {
        'continue45: loop {
            {
                let backward
                    : usize
                    = *distance_cache.offset(i as (isize)) as (usize);
                let mut prev_ix : usize = cur_ix.wrapping_sub(backward) as (usize);
                if prev_ix >= cur_ix {
                    if 1337i32 != 0 {
                        break 'continue45;
                    }
                }
                if backward > max_backward {
                    if 1337i32 != 0 {
                        break 'continue45;
                    }
                }
                prev_ix = prev_ix & ring_buffer_mask;
                if cur_ix_masked.wrapping_add(
                       best_len
                   ) > ring_buffer_mask || prev_ix.wrapping_add(
                                               best_len
                                           ) > ring_buffer_mask || *data.offset(
                                                                        cur_ix_masked.wrapping_add(
                                                                            best_len
                                                                        ) as (isize)
                                                                    ) as (i32) != *data.offset(
                                                                                       prev_ix.wrapping_add(
                                                                                           best_len
                                                                                       ) as (isize)
                                                                                   ) as (i32) {
                    if 1337i32 != 0 {
                        break 'continue45;
                    }
                }
                {
                    let len
                        : usize
                        = FindMatchLengthWithLimit(
                              &*data.offset(prev_ix as (isize)) as (*const u8),
                              &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                              max_length
                          );
                    if len >= 3i32 as (usize) || len == 2i32 as (usize) && (i < 2i32 as (usize)) {
                        let mut score
                            : usize
                            = BackwardReferenceScoreUsingLastDistance(len);
                        if best_score < score {
                            if i != 0i32 as (usize) {
                                score = score.wrapping_sub(
                                            BackwardReferencePenaltyUsingLastDistance(i)
                                        );
                            }
                            if best_score < score {
                                best_score = score;
                                best_len = len;
                                (*out).len = best_len;
                                (*out).distance = backward;
                                (*out).score = best_score;
                                is_match_found = 1i32;
                            }
                        }
                    }
                }
            }
            break;
        }
        i = i.wrapping_add(1 as (usize));
    }
    {
        let key
            : u32
            = HashBytesH6(
                  &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                  (*self).hash_mask_,
                  (*self).hash_shift_
              );
        let mut bucket
            : *mut u32
            = &mut *buckets.offset(
                        (key << (*common).params.block_bits) as (isize)
                    ) as (*mut u32);
        let down
            : usize
            = if *num.offset(key as (isize)) as (usize) > (*self).block_size_ {
                  (*num.offset(key as (isize)) as (usize)).wrapping_sub(
                      (*self).block_size_
                  )
              } else {
                  0u32 as (usize)
              };
        i = *num.offset(key as (isize)) as (usize);
        while i > down {
            let mut prev_ix
                : usize
                = *bucket.offset(
                       ({
                            i = i.wrapping_sub(1 as (usize));
                            i
                        } & (*self).block_mask_ as (usize)) as (isize)
                   ) as (usize);
            let backward : usize = cur_ix.wrapping_sub(prev_ix);
            if backward > max_backward {
                if 1337i32 != 0 {
                    break;
                }
            }
            prev_ix = prev_ix & ring_buffer_mask;
            if cur_ix_masked.wrapping_add(
                   best_len
               ) > ring_buffer_mask || prev_ix.wrapping_add(
                                           best_len
                                       ) > ring_buffer_mask || *data.offset(
                                                                    cur_ix_masked.wrapping_add(
                                                                        best_len
                                                                    ) as (isize)
                                                                ) as (i32) != *data.offset(
                                                                                   prev_ix.wrapping_add(
                                                                                       best_len
                                                                                   ) as (isize)
                                                                               ) as (i32) {
                if 1337i32 != 0 {
                    continue;
                }
            }
            {
                let len
                    : usize
                    = FindMatchLengthWithLimit(
                          &*data.offset(prev_ix as (isize)) as (*const u8),
                          &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                          max_length
                      );
                if len >= 4i32 as (usize) {
                    let mut score : usize = BackwardReferenceScore(len,backward);
                    if best_score < score {
                        best_score = score;
                        best_len = len;
                        (*out).len = best_len;
                        (*out).distance = backward;
                        (*out).score = best_score;
                        is_match_found = 1i32;
                    }
                }
            }
        }
        *bucket.offset(
             (*num.offset(
                   key as (isize)
               ) as (u32) & (*self).block_mask_) as (isize)
         ) = cur_ix as (u32);
        {
            let _rhs = 1;
            let _lhs = &mut *num.offset(key as (isize));
            *_lhs = (*_lhs as (i32) + _rhs) as (u16);
        }
    }
    if is_match_found == 0 {
        is_match_found = SearchInStaticDictionary(
                             dictionary,
                             dictionary_hash,
                             handle,
                             &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                             max_length,
                             max_backward,
                             out,
                             0i32
                         );
    }
    is_match_found
}

unsafe extern fn StoreH6(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let mut self : *mut H6 = SelfH6(handle);
    let mut num : *mut u16 = NumH6(self);
    let key
        : u32
        = HashBytesH6(
              &*data.offset((ix & mask) as (isize)) as (*const u8),
              (*self).hash_mask_,
              (*self).hash_shift_
          );
    let minor_ix
        : usize
        = (*num.offset(
                key as (isize)
            ) as (u32) & (*self).block_mask_) as (usize);
    let offset
        : usize
        = minor_ix.wrapping_add(
              (key << (*GetHasherCommon(handle)).params.block_bits) as (usize)
          );
    *BucketsH6(self).offset(offset as (isize)) = ix as (u32);
    {
        let _rhs = 1;
        let _lhs = &mut *num.offset(key as (isize));
        *_lhs = (*_lhs as (i32) + _rhs) as (u16);
    }
}

unsafe extern fn StoreRangeH6(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix_start : usize,
    ix_end : usize
) {
    let mut i : usize;
    i = ix_start;
    while i < ix_end {
        {
            StoreH6(handle,data,mask,i);
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn CreateBackwardReferencesH6(
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    mut hasher : *mut u8,
    mut dist_cache : *mut i32,
    mut last_insert_len : *mut usize,
    mut commands : *mut Command,
    mut num_commands : *mut usize,
    mut num_literals : *mut usize
) {
    let max_backward_limit
        : usize
        = (1i32 as (usize) << (*params).lgwin).wrapping_sub(
              16i32 as (usize)
          );
    let orig_commands : *const Command = commands as (*const Command);
    let mut insert_length : usize = *last_insert_len;
    let pos_end : usize = position.wrapping_add(num_bytes);
    let store_end
        : usize
        = if num_bytes >= StoreLookaheadH6() {
              position.wrapping_add(num_bytes).wrapping_sub(
                  StoreLookaheadH6()
              ).wrapping_add(
                  1i32 as (usize)
              )
          } else {
              position
          };
    let random_heuristics_window_size
        : usize
        = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics
        : usize
        = position.wrapping_add(random_heuristics_window_size);
    let kMinScore
        : usize
        = ((30i32 * 8i32) as (usize)).wrapping_mul(
              std::mem::size_of::<usize>()
          ).wrapping_add(
              100i32 as (usize)
          );
    PrepareDistanceCacheH6(hasher,dist_cache);
    while position.wrapping_add(HashTypeLengthH6()) < pos_end {
        let mut max_length : usize = pos_end.wrapping_sub(position);
        let mut max_distance
            : usize
            = brotli_min_size_t(position,max_backward_limit);
        let mut sr : HasherSearchResult;
        sr.len = 0i32 as (usize);
        sr.len_x_code = 0i32 as (usize);
        sr.distance = 0i32 as (usize);
        sr.score = kMinScore;
        if FindLongestMatchH6(
               hasher,
               dictionary,
               dictionary_hash,
               ringbuffer,
               ringbuffer_mask,
               dist_cache as (*const i32),
               position,
               max_length,
               max_distance,
               &mut sr as (*mut HasherSearchResult)
           ) != 0 {
            let mut delayed_backward_references_in_row : i32 = 0i32;
            max_length = max_length.wrapping_sub(1 as (usize));
            'break46: loop {
                'continue47: loop {
                    {
                        let cost_diff_lazy : usize = 175i32 as (usize);
                        let mut is_match_found : i32;
                        let mut sr2 : HasherSearchResult;
                        sr2.len = if (*params).quality < 5i32 {
                                      brotli_min_size_t(
                                          sr.len.wrapping_sub(1i32 as (usize)),
                                          max_length
                                      )
                                  } else {
                                      0i32 as (usize)
                                  };
                        sr2.len_x_code = 0i32 as (usize);
                        sr2.distance = 0i32 as (usize);
                        sr2.score = kMinScore;
                        max_distance = brotli_min_size_t(
                                           position.wrapping_add(1i32 as (usize)),
                                           max_backward_limit
                                       );
                        is_match_found = FindLongestMatchH6(
                                             hasher,
                                             dictionary,
                                             dictionary_hash,
                                             ringbuffer,
                                             ringbuffer_mask,
                                             dist_cache as (*const i32),
                                             position.wrapping_add(1i32 as (usize)),
                                             max_length,
                                             max_distance,
                                             &mut sr2 as (*mut HasherSearchResult)
                                         );
                        if is_match_found != 0 && (sr2.score >= sr.score.wrapping_add(
                                                                    cost_diff_lazy
                                                                )) {
                            position = position.wrapping_add(1 as (usize));
                            insert_length = insert_length.wrapping_add(1 as (usize));
                            sr = sr2;
                            if {
                                   delayed_backward_references_in_row = delayed_backward_references_in_row + 1;
                                   delayed_backward_references_in_row
                               } < 4i32 && (position.wrapping_add(HashTypeLengthH6()) < pos_end) {
                                if 1337i32 != 0 {
                                    break 'continue47;
                                }
                            }
                        }
                        {
                            if 1337i32 != 0 {
                                break 'break46;
                            }
                        }
                    }
                    break;
                }
                max_length = max_length.wrapping_sub(1 as (usize));
            }
            apply_random_heuristics = position.wrapping_add(
                                          (2i32 as (usize)).wrapping_mul(sr.len)
                                      ).wrapping_add(
                                          random_heuristics_window_size
                                      );
            max_distance = brotli_min_size_t(position,max_backward_limit);
            {
                let mut distance_code
                    : usize
                    = ComputeDistanceCode(
                          sr.distance,
                          max_distance,
                          dist_cache as (*const i32)
                      );
                if sr.distance <= max_distance && (distance_code > 0i32 as (usize)) {
                    *dist_cache.offset(3i32 as (isize)) = *dist_cache.offset(
                                                               2i32 as (isize)
                                                           );
                    *dist_cache.offset(2i32 as (isize)) = *dist_cache.offset(
                                                               1i32 as (isize)
                                                           );
                    *dist_cache.offset(1i32 as (isize)) = *dist_cache.offset(
                                                               0i32 as (isize)
                                                           );
                    *dist_cache.offset(0i32 as (isize)) = sr.distance as (i32);
                    PrepareDistanceCacheH6(hasher,dist_cache);
                }
                InitCommand(
                    {
                        let _old = commands;
                        commands = commands.offset(1 as (isize));
                        _old
                    },
                    insert_length,
                    sr.len,
                    sr.len ^ sr.len_x_code,
                    distance_code
                );
            }
            *num_literals = (*num_literals).wrapping_add(insert_length);
            insert_length = 0i32 as (usize);
            StoreRangeH6(
                hasher,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_add(2i32 as (usize)),
                brotli_min_size_t(position.wrapping_add(sr.len),store_end)
            );
            position = position.wrapping_add(sr.len);
        } else {
            insert_length = insert_length.wrapping_add(1 as (usize));
            position = position.wrapping_add(1 as (usize));
            if position > apply_random_heuristics {
                if position > apply_random_heuristics.wrapping_add(
                                  (4i32 as (usize)).wrapping_mul(random_heuristics_window_size)
                              ) {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH6().wrapping_sub(1i32 as (usize)),
                              4i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(16i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH6(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(4i32 as (usize));
                        }
                        position = position.wrapping_add(4i32 as (usize));
                    }
                } else {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH6().wrapping_sub(1i32 as (usize)),
                              2i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(8i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH6(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(2i32 as (usize));
                        }
                        position = position.wrapping_add(2i32 as (usize));
                    }
                }
            }
        }
    }
    insert_length = insert_length.wrapping_add(
                        pos_end.wrapping_sub(position)
                    );
    *last_insert_len = insert_length;
    *num_commands = (*num_commands).wrapping_add(
                        ((commands as (isize)).wrapping_sub(
                             orig_commands as (isize)
                         ) / std::mem::size_of::<*const Command>() as (isize)) as (usize)
                    );
}

unsafe extern fn StoreLookaheadH40() -> usize { 4i32 as (usize) }

unsafe extern fn PrepareDistanceCacheH40(
    mut handle : *mut u8, mut distance_cache : *mut i32
) {
    handle;
    PrepareDistanceCache(distance_cache,4i32);
}

unsafe extern fn HashTypeLengthH40() -> usize { 4i32 as (usize) }

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SlotH40 {
    pub delta : u16,
    pub next : u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BankH40 {
    pub slots : *mut SlotH40,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H40 {
    pub addr : *mut u32,
    pub head : *mut u16,
    pub tiny_hash : *mut u8,
    pub banks : *mut BankH40,
    pub free_slot_idx : *mut u16,
    pub max_hops : usize,
}

unsafe extern fn SelfH40(mut handle : *mut u8) -> *mut H40 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct1) as (*mut H40)
}

unsafe extern fn HashBytesH40(mut data : *const u8) -> usize {
    let h
        : u32
        = BROTLI_UNALIGNED_LOAD32(
              data as (*const std::os::raw::c_void)
          ).wrapping_mul(
              kHashMul32
          );
    (h >> 32i32 - 15i32) as (usize)
}

unsafe extern fn StoreH40(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let mut self : *mut H40 = SelfH40(handle);
    let key
        : usize
        = HashBytesH40(
              &*data.offset((ix & mask) as (isize)) as (*const u8)
          );
    let bank : usize = key & (1i32 - 1i32) as (usize);
    let idx
        : usize
        = (({
                let _rhs = 1;
                let _lhs = &mut *(*self).free_slot_idx.offset(bank as (isize));
                let _old = *_lhs;
                *_lhs = (*_lhs as (i32) + _rhs) as (u16);
                _old
            }) as (i32) & 65536i32 - 1i32) as (usize);
    let mut delta
        : usize
        = ix.wrapping_sub(*(*self).addr.offset(key as (isize)) as (usize));
    *(*self).tiny_hash.offset(ix as (u16) as (isize)) = key as (u8);
    if delta > 0xffffi32 as (usize) {
        delta = if 0i32 != 0 { 0i32 } else { 0xffffi32 } as (usize);
    }
    (*(*(*self).banks.offset(bank as (isize))).slots.offset(
          idx as (isize)
      )).delta = delta as (u16);
    (*(*(*self).banks.offset(bank as (isize))).slots.offset(
          idx as (isize)
      )).next = *(*self).head.offset(key as (isize));
    *(*self).addr.offset(key as (isize)) = ix as (u32);
    *(*self).head.offset(key as (isize)) = idx as (u16);
}

unsafe extern fn FindLongestMatchH40(
    mut handle : *mut u8,
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut data : *const u8,
    ring_buffer_mask : usize,
    mut distance_cache : *const i32,
    cur_ix : usize,
    max_length : usize,
    max_backward : usize,
    mut out : *mut HasherSearchResult
) -> i32 {
    let mut self : *mut H40 = SelfH40(handle);
    let cur_ix_masked : usize = cur_ix & ring_buffer_mask;
    let mut is_match_found : i32 = 0i32;
    let mut best_score : usize = (*out).score;
    let mut best_len : usize = (*out).len;
    let mut i : usize;
    let key
        : usize
        = HashBytesH40(
              &*data.offset(cur_ix_masked as (isize)) as (*const u8)
          );
    let tiny_hash : u8 = key as (u8);
    (*out).len = 0i32 as (usize);
    (*out).len_x_code = 0i32 as (usize);
    i = 0i32 as (usize);
    while i < 4i32 as (usize) {
        'continue55: loop {
            {
                let backward
                    : usize
                    = *distance_cache.offset(i as (isize)) as (usize);
                let mut prev_ix : usize = cur_ix.wrapping_sub(backward);
                if i > 0i32 as (usize) && (*(*self).tiny_hash.offset(
                                                prev_ix as (u16) as (isize)
                                            ) as (i32) != tiny_hash as (i32)) {
                    if 1337i32 != 0 {
                        break 'continue55;
                    }
                }
                if prev_ix >= cur_ix || backward > max_backward {
                    if 1337i32 != 0 {
                        break 'continue55;
                    }
                }
                prev_ix = prev_ix & ring_buffer_mask;
                {
                    let len
                        : usize
                        = FindMatchLengthWithLimit(
                              &*data.offset(prev_ix as (isize)) as (*const u8),
                              &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                              max_length
                          );
                    if len >= 2i32 as (usize) {
                        let mut score
                            : usize
                            = BackwardReferenceScoreUsingLastDistance(len);
                        if best_score < score {
                            if i != 0i32 as (usize) {
                                score = score.wrapping_sub(
                                            BackwardReferencePenaltyUsingLastDistance(i)
                                        );
                            }
                            if best_score < score {
                                best_score = score;
                                best_len = len;
                                (*out).len = best_len;
                                (*out).distance = backward;
                                (*out).score = best_score;
                                is_match_found = 1i32;
                            }
                        }
                    }
                }
            }
            break;
        }
        i = i.wrapping_add(1 as (usize));
    }
    {
        let bank : usize = key & (1i32 - 1i32) as (usize);
        let mut backward : usize = 0i32 as (usize);
        let mut hops : usize = (*self).max_hops;
        let mut delta
            : usize
            = cur_ix.wrapping_sub(
                  *(*self).addr.offset(key as (isize)) as (usize)
              );
        let mut slot
            : usize
            = *(*self).head.offset(key as (isize)) as (usize);
        while {
                  let _old = hops;
                  hops = hops.wrapping_sub(1 as (usize));
                  _old
              } != 0 {
            let mut prev_ix : usize;
            let mut last : usize = slot;
            backward = backward.wrapping_add(delta);
            if backward > max_backward || 0i32 != 0 && (delta == 0) {
                if 1337i32 != 0 {
                    break;
                }
            }
            prev_ix = cur_ix.wrapping_sub(backward) & ring_buffer_mask;
            slot = (*(*(*self).banks.offset(bank as (isize))).slots.offset(
                         last as (isize)
                     )).next as (usize);
            delta = (*(*(*self).banks.offset(bank as (isize))).slots.offset(
                          last as (isize)
                      )).delta as (usize);
            if cur_ix_masked.wrapping_add(
                   best_len
               ) > ring_buffer_mask || prev_ix.wrapping_add(
                                           best_len
                                       ) > ring_buffer_mask || *data.offset(
                                                                    cur_ix_masked.wrapping_add(
                                                                        best_len
                                                                    ) as (isize)
                                                                ) as (i32) != *data.offset(
                                                                                   prev_ix.wrapping_add(
                                                                                       best_len
                                                                                   ) as (isize)
                                                                               ) as (i32) {
                if 1337i32 != 0 {
                    continue;
                }
            }
            {
                let len
                    : usize
                    = FindMatchLengthWithLimit(
                          &*data.offset(prev_ix as (isize)) as (*const u8),
                          &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                          max_length
                      );
                if len >= 4i32 as (usize) {
                    let mut score : usize = BackwardReferenceScore(len,backward);
                    if best_score < score {
                        best_score = score;
                        best_len = len;
                        (*out).len = best_len;
                        (*out).distance = backward;
                        (*out).score = best_score;
                        is_match_found = 1i32;
                    }
                }
            }
        }
        StoreH40(handle,data,ring_buffer_mask,cur_ix);
    }
    if is_match_found == 0 {
        is_match_found = SearchInStaticDictionary(
                             dictionary,
                             dictionary_hash,
                             handle,
                             &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                             max_length,
                             max_backward,
                             out,
                             0i32
                         );
    }
    is_match_found
}

unsafe extern fn StoreRangeH40(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix_start : usize,
    ix_end : usize
) {
    let mut i : usize;
    i = ix_start;
    while i < ix_end {
        {
            StoreH40(handle,data,mask,i);
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn CreateBackwardReferencesH40(
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    mut hasher : *mut u8,
    mut dist_cache : *mut i32,
    mut last_insert_len : *mut usize,
    mut commands : *mut Command,
    mut num_commands : *mut usize,
    mut num_literals : *mut usize
) {
    let max_backward_limit
        : usize
        = (1i32 as (usize) << (*params).lgwin).wrapping_sub(
              16i32 as (usize)
          );
    let orig_commands : *const Command = commands as (*const Command);
    let mut insert_length : usize = *last_insert_len;
    let pos_end : usize = position.wrapping_add(num_bytes);
    let store_end
        : usize
        = if num_bytes >= StoreLookaheadH40() {
              position.wrapping_add(num_bytes).wrapping_sub(
                  StoreLookaheadH40()
              ).wrapping_add(
                  1i32 as (usize)
              )
          } else {
              position
          };
    let random_heuristics_window_size
        : usize
        = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics
        : usize
        = position.wrapping_add(random_heuristics_window_size);
    let kMinScore
        : usize
        = ((30i32 * 8i32) as (usize)).wrapping_mul(
              std::mem::size_of::<usize>()
          ).wrapping_add(
              100i32 as (usize)
          );
    PrepareDistanceCacheH40(hasher,dist_cache);
    while position.wrapping_add(HashTypeLengthH40()) < pos_end {
        let mut max_length : usize = pos_end.wrapping_sub(position);
        let mut max_distance
            : usize
            = brotli_min_size_t(position,max_backward_limit);
        let mut sr : HasherSearchResult;
        sr.len = 0i32 as (usize);
        sr.len_x_code = 0i32 as (usize);
        sr.distance = 0i32 as (usize);
        sr.score = kMinScore;
        if FindLongestMatchH40(
               hasher,
               dictionary,
               dictionary_hash,
               ringbuffer,
               ringbuffer_mask,
               dist_cache as (*const i32),
               position,
               max_length,
               max_distance,
               &mut sr as (*mut HasherSearchResult)
           ) != 0 {
            let mut delayed_backward_references_in_row : i32 = 0i32;
            max_length = max_length.wrapping_sub(1 as (usize));
            'break56: loop {
                'continue57: loop {
                    {
                        let cost_diff_lazy : usize = 175i32 as (usize);
                        let mut is_match_found : i32;
                        let mut sr2 : HasherSearchResult;
                        sr2.len = if (*params).quality < 5i32 {
                                      brotli_min_size_t(
                                          sr.len.wrapping_sub(1i32 as (usize)),
                                          max_length
                                      )
                                  } else {
                                      0i32 as (usize)
                                  };
                        sr2.len_x_code = 0i32 as (usize);
                        sr2.distance = 0i32 as (usize);
                        sr2.score = kMinScore;
                        max_distance = brotli_min_size_t(
                                           position.wrapping_add(1i32 as (usize)),
                                           max_backward_limit
                                       );
                        is_match_found = FindLongestMatchH40(
                                             hasher,
                                             dictionary,
                                             dictionary_hash,
                                             ringbuffer,
                                             ringbuffer_mask,
                                             dist_cache as (*const i32),
                                             position.wrapping_add(1i32 as (usize)),
                                             max_length,
                                             max_distance,
                                             &mut sr2 as (*mut HasherSearchResult)
                                         );
                        if is_match_found != 0 && (sr2.score >= sr.score.wrapping_add(
                                                                    cost_diff_lazy
                                                                )) {
                            position = position.wrapping_add(1 as (usize));
                            insert_length = insert_length.wrapping_add(1 as (usize));
                            sr = sr2;
                            if {
                                   delayed_backward_references_in_row = delayed_backward_references_in_row + 1;
                                   delayed_backward_references_in_row
                               } < 4i32 && (position.wrapping_add(
                                                HashTypeLengthH40()
                                            ) < pos_end) {
                                if 1337i32 != 0 {
                                    break 'continue57;
                                }
                            }
                        }
                        {
                            if 1337i32 != 0 {
                                break 'break56;
                            }
                        }
                    }
                    break;
                }
                max_length = max_length.wrapping_sub(1 as (usize));
            }
            apply_random_heuristics = position.wrapping_add(
                                          (2i32 as (usize)).wrapping_mul(sr.len)
                                      ).wrapping_add(
                                          random_heuristics_window_size
                                      );
            max_distance = brotli_min_size_t(position,max_backward_limit);
            {
                let mut distance_code
                    : usize
                    = ComputeDistanceCode(
                          sr.distance,
                          max_distance,
                          dist_cache as (*const i32)
                      );
                if sr.distance <= max_distance && (distance_code > 0i32 as (usize)) {
                    *dist_cache.offset(3i32 as (isize)) = *dist_cache.offset(
                                                               2i32 as (isize)
                                                           );
                    *dist_cache.offset(2i32 as (isize)) = *dist_cache.offset(
                                                               1i32 as (isize)
                                                           );
                    *dist_cache.offset(1i32 as (isize)) = *dist_cache.offset(
                                                               0i32 as (isize)
                                                           );
                    *dist_cache.offset(0i32 as (isize)) = sr.distance as (i32);
                    PrepareDistanceCacheH40(hasher,dist_cache);
                }
                InitCommand(
                    {
                        let _old = commands;
                        commands = commands.offset(1 as (isize));
                        _old
                    },
                    insert_length,
                    sr.len,
                    sr.len ^ sr.len_x_code,
                    distance_code
                );
            }
            *num_literals = (*num_literals).wrapping_add(insert_length);
            insert_length = 0i32 as (usize);
            StoreRangeH40(
                hasher,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_add(2i32 as (usize)),
                brotli_min_size_t(position.wrapping_add(sr.len),store_end)
            );
            position = position.wrapping_add(sr.len);
        } else {
            insert_length = insert_length.wrapping_add(1 as (usize));
            position = position.wrapping_add(1 as (usize));
            if position > apply_random_heuristics {
                if position > apply_random_heuristics.wrapping_add(
                                  (4i32 as (usize)).wrapping_mul(random_heuristics_window_size)
                              ) {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH40().wrapping_sub(1i32 as (usize)),
                              4i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(16i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH40(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(4i32 as (usize));
                        }
                        position = position.wrapping_add(4i32 as (usize));
                    }
                } else {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH40().wrapping_sub(1i32 as (usize)),
                              2i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(8i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH40(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(2i32 as (usize));
                        }
                        position = position.wrapping_add(2i32 as (usize));
                    }
                }
            }
        }
    }
    insert_length = insert_length.wrapping_add(
                        pos_end.wrapping_sub(position)
                    );
    *last_insert_len = insert_length;
    *num_commands = (*num_commands).wrapping_add(
                        ((commands as (isize)).wrapping_sub(
                             orig_commands as (isize)
                         ) / std::mem::size_of::<*const Command>() as (isize)) as (usize)
                    );
}

unsafe extern fn StoreLookaheadH41() -> usize { 4i32 as (usize) }

unsafe extern fn PrepareDistanceCacheH41(
    mut handle : *mut u8, mut distance_cache : *mut i32
) {
    handle;
    PrepareDistanceCache(distance_cache,10i32);
}

unsafe extern fn HashTypeLengthH41() -> usize { 4i32 as (usize) }

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SlotH41 {
    pub delta : u16,
    pub next : u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BankH41 {
    pub slots : *mut SlotH41,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H41 {
    pub addr : *mut u32,
    pub head : *mut u16,
    pub tiny_hash : *mut u8,
    pub banks : *mut BankH41,
    pub free_slot_idx : *mut u16,
    pub max_hops : usize,
}

unsafe extern fn SelfH41(mut handle : *mut u8) -> *mut H41 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct1) as (*mut H41)
}

unsafe extern fn HashBytesH41(mut data : *const u8) -> usize {
    let h
        : u32
        = BROTLI_UNALIGNED_LOAD32(
              data as (*const std::os::raw::c_void)
          ).wrapping_mul(
              kHashMul32
          );
    (h >> 32i32 - 15i32) as (usize)
}

unsafe extern fn StoreH41(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let mut self : *mut H41 = SelfH41(handle);
    let key
        : usize
        = HashBytesH41(
              &*data.offset((ix & mask) as (isize)) as (*const u8)
          );
    let bank : usize = key & (1i32 - 1i32) as (usize);
    let idx
        : usize
        = (({
                let _rhs = 1;
                let _lhs = &mut *(*self).free_slot_idx.offset(bank as (isize));
                let _old = *_lhs;
                *_lhs = (*_lhs as (i32) + _rhs) as (u16);
                _old
            }) as (i32) & 65536i32 - 1i32) as (usize);
    let mut delta
        : usize
        = ix.wrapping_sub(*(*self).addr.offset(key as (isize)) as (usize));
    *(*self).tiny_hash.offset(ix as (u16) as (isize)) = key as (u8);
    if delta > 0xffffi32 as (usize) {
        delta = if 0i32 != 0 { 0i32 } else { 0xffffi32 } as (usize);
    }
    (*(*(*self).banks.offset(bank as (isize))).slots.offset(
          idx as (isize)
      )).delta = delta as (u16);
    (*(*(*self).banks.offset(bank as (isize))).slots.offset(
          idx as (isize)
      )).next = *(*self).head.offset(key as (isize));
    *(*self).addr.offset(key as (isize)) = ix as (u32);
    *(*self).head.offset(key as (isize)) = idx as (u16);
}

unsafe extern fn FindLongestMatchH41(
    mut handle : *mut u8,
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut data : *const u8,
    ring_buffer_mask : usize,
    mut distance_cache : *const i32,
    cur_ix : usize,
    max_length : usize,
    max_backward : usize,
    mut out : *mut HasherSearchResult
) -> i32 {
    let mut self : *mut H41 = SelfH41(handle);
    let cur_ix_masked : usize = cur_ix & ring_buffer_mask;
    let mut is_match_found : i32 = 0i32;
    let mut best_score : usize = (*out).score;
    let mut best_len : usize = (*out).len;
    let mut i : usize;
    let key
        : usize
        = HashBytesH41(
              &*data.offset(cur_ix_masked as (isize)) as (*const u8)
          );
    let tiny_hash : u8 = key as (u8);
    (*out).len = 0i32 as (usize);
    (*out).len_x_code = 0i32 as (usize);
    i = 0i32 as (usize);
    while i < 10i32 as (usize) {
        'continue65: loop {
            {
                let backward
                    : usize
                    = *distance_cache.offset(i as (isize)) as (usize);
                let mut prev_ix : usize = cur_ix.wrapping_sub(backward);
                if i > 0i32 as (usize) && (*(*self).tiny_hash.offset(
                                                prev_ix as (u16) as (isize)
                                            ) as (i32) != tiny_hash as (i32)) {
                    if 1337i32 != 0 {
                        break 'continue65;
                    }
                }
                if prev_ix >= cur_ix || backward > max_backward {
                    if 1337i32 != 0 {
                        break 'continue65;
                    }
                }
                prev_ix = prev_ix & ring_buffer_mask;
                {
                    let len
                        : usize
                        = FindMatchLengthWithLimit(
                              &*data.offset(prev_ix as (isize)) as (*const u8),
                              &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                              max_length
                          );
                    if len >= 2i32 as (usize) {
                        let mut score
                            : usize
                            = BackwardReferenceScoreUsingLastDistance(len);
                        if best_score < score {
                            if i != 0i32 as (usize) {
                                score = score.wrapping_sub(
                                            BackwardReferencePenaltyUsingLastDistance(i)
                                        );
                            }
                            if best_score < score {
                                best_score = score;
                                best_len = len;
                                (*out).len = best_len;
                                (*out).distance = backward;
                                (*out).score = best_score;
                                is_match_found = 1i32;
                            }
                        }
                    }
                }
            }
            break;
        }
        i = i.wrapping_add(1 as (usize));
    }
    {
        let bank : usize = key & (1i32 - 1i32) as (usize);
        let mut backward : usize = 0i32 as (usize);
        let mut hops : usize = (*self).max_hops;
        let mut delta
            : usize
            = cur_ix.wrapping_sub(
                  *(*self).addr.offset(key as (isize)) as (usize)
              );
        let mut slot
            : usize
            = *(*self).head.offset(key as (isize)) as (usize);
        while {
                  let _old = hops;
                  hops = hops.wrapping_sub(1 as (usize));
                  _old
              } != 0 {
            let mut prev_ix : usize;
            let mut last : usize = slot;
            backward = backward.wrapping_add(delta);
            if backward > max_backward || 0i32 != 0 && (delta == 0) {
                if 1337i32 != 0 {
                    break;
                }
            }
            prev_ix = cur_ix.wrapping_sub(backward) & ring_buffer_mask;
            slot = (*(*(*self).banks.offset(bank as (isize))).slots.offset(
                         last as (isize)
                     )).next as (usize);
            delta = (*(*(*self).banks.offset(bank as (isize))).slots.offset(
                          last as (isize)
                      )).delta as (usize);
            if cur_ix_masked.wrapping_add(
                   best_len
               ) > ring_buffer_mask || prev_ix.wrapping_add(
                                           best_len
                                       ) > ring_buffer_mask || *data.offset(
                                                                    cur_ix_masked.wrapping_add(
                                                                        best_len
                                                                    ) as (isize)
                                                                ) as (i32) != *data.offset(
                                                                                   prev_ix.wrapping_add(
                                                                                       best_len
                                                                                   ) as (isize)
                                                                               ) as (i32) {
                if 1337i32 != 0 {
                    continue;
                }
            }
            {
                let len
                    : usize
                    = FindMatchLengthWithLimit(
                          &*data.offset(prev_ix as (isize)) as (*const u8),
                          &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                          max_length
                      );
                if len >= 4i32 as (usize) {
                    let mut score : usize = BackwardReferenceScore(len,backward);
                    if best_score < score {
                        best_score = score;
                        best_len = len;
                        (*out).len = best_len;
                        (*out).distance = backward;
                        (*out).score = best_score;
                        is_match_found = 1i32;
                    }
                }
            }
        }
        StoreH41(handle,data,ring_buffer_mask,cur_ix);
    }
    if is_match_found == 0 {
        is_match_found = SearchInStaticDictionary(
                             dictionary,
                             dictionary_hash,
                             handle,
                             &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                             max_length,
                             max_backward,
                             out,
                             0i32
                         );
    }
    is_match_found
}

unsafe extern fn StoreRangeH41(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix_start : usize,
    ix_end : usize
) {
    let mut i : usize;
    i = ix_start;
    while i < ix_end {
        {
            StoreH41(handle,data,mask,i);
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn CreateBackwardReferencesH41(
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    mut hasher : *mut u8,
    mut dist_cache : *mut i32,
    mut last_insert_len : *mut usize,
    mut commands : *mut Command,
    mut num_commands : *mut usize,
    mut num_literals : *mut usize
) {
    let max_backward_limit
        : usize
        = (1i32 as (usize) << (*params).lgwin).wrapping_sub(
              16i32 as (usize)
          );
    let orig_commands : *const Command = commands as (*const Command);
    let mut insert_length : usize = *last_insert_len;
    let pos_end : usize = position.wrapping_add(num_bytes);
    let store_end
        : usize
        = if num_bytes >= StoreLookaheadH41() {
              position.wrapping_add(num_bytes).wrapping_sub(
                  StoreLookaheadH41()
              ).wrapping_add(
                  1i32 as (usize)
              )
          } else {
              position
          };
    let random_heuristics_window_size
        : usize
        = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics
        : usize
        = position.wrapping_add(random_heuristics_window_size);
    let kMinScore
        : usize
        = ((30i32 * 8i32) as (usize)).wrapping_mul(
              std::mem::size_of::<usize>()
          ).wrapping_add(
              100i32 as (usize)
          );
    PrepareDistanceCacheH41(hasher,dist_cache);
    while position.wrapping_add(HashTypeLengthH41()) < pos_end {
        let mut max_length : usize = pos_end.wrapping_sub(position);
        let mut max_distance
            : usize
            = brotli_min_size_t(position,max_backward_limit);
        let mut sr : HasherSearchResult;
        sr.len = 0i32 as (usize);
        sr.len_x_code = 0i32 as (usize);
        sr.distance = 0i32 as (usize);
        sr.score = kMinScore;
        if FindLongestMatchH41(
               hasher,
               dictionary,
               dictionary_hash,
               ringbuffer,
               ringbuffer_mask,
               dist_cache as (*const i32),
               position,
               max_length,
               max_distance,
               &mut sr as (*mut HasherSearchResult)
           ) != 0 {
            let mut delayed_backward_references_in_row : i32 = 0i32;
            max_length = max_length.wrapping_sub(1 as (usize));
            'break66: loop {
                'continue67: loop {
                    {
                        let cost_diff_lazy : usize = 175i32 as (usize);
                        let mut is_match_found : i32;
                        let mut sr2 : HasherSearchResult;
                        sr2.len = if (*params).quality < 5i32 {
                                      brotli_min_size_t(
                                          sr.len.wrapping_sub(1i32 as (usize)),
                                          max_length
                                      )
                                  } else {
                                      0i32 as (usize)
                                  };
                        sr2.len_x_code = 0i32 as (usize);
                        sr2.distance = 0i32 as (usize);
                        sr2.score = kMinScore;
                        max_distance = brotli_min_size_t(
                                           position.wrapping_add(1i32 as (usize)),
                                           max_backward_limit
                                       );
                        is_match_found = FindLongestMatchH41(
                                             hasher,
                                             dictionary,
                                             dictionary_hash,
                                             ringbuffer,
                                             ringbuffer_mask,
                                             dist_cache as (*const i32),
                                             position.wrapping_add(1i32 as (usize)),
                                             max_length,
                                             max_distance,
                                             &mut sr2 as (*mut HasherSearchResult)
                                         );
                        if is_match_found != 0 && (sr2.score >= sr.score.wrapping_add(
                                                                    cost_diff_lazy
                                                                )) {
                            position = position.wrapping_add(1 as (usize));
                            insert_length = insert_length.wrapping_add(1 as (usize));
                            sr = sr2;
                            if {
                                   delayed_backward_references_in_row = delayed_backward_references_in_row + 1;
                                   delayed_backward_references_in_row
                               } < 4i32 && (position.wrapping_add(
                                                HashTypeLengthH41()
                                            ) < pos_end) {
                                if 1337i32 != 0 {
                                    break 'continue67;
                                }
                            }
                        }
                        {
                            if 1337i32 != 0 {
                                break 'break66;
                            }
                        }
                    }
                    break;
                }
                max_length = max_length.wrapping_sub(1 as (usize));
            }
            apply_random_heuristics = position.wrapping_add(
                                          (2i32 as (usize)).wrapping_mul(sr.len)
                                      ).wrapping_add(
                                          random_heuristics_window_size
                                      );
            max_distance = brotli_min_size_t(position,max_backward_limit);
            {
                let mut distance_code
                    : usize
                    = ComputeDistanceCode(
                          sr.distance,
                          max_distance,
                          dist_cache as (*const i32)
                      );
                if sr.distance <= max_distance && (distance_code > 0i32 as (usize)) {
                    *dist_cache.offset(3i32 as (isize)) = *dist_cache.offset(
                                                               2i32 as (isize)
                                                           );
                    *dist_cache.offset(2i32 as (isize)) = *dist_cache.offset(
                                                               1i32 as (isize)
                                                           );
                    *dist_cache.offset(1i32 as (isize)) = *dist_cache.offset(
                                                               0i32 as (isize)
                                                           );
                    *dist_cache.offset(0i32 as (isize)) = sr.distance as (i32);
                    PrepareDistanceCacheH41(hasher,dist_cache);
                }
                InitCommand(
                    {
                        let _old = commands;
                        commands = commands.offset(1 as (isize));
                        _old
                    },
                    insert_length,
                    sr.len,
                    sr.len ^ sr.len_x_code,
                    distance_code
                );
            }
            *num_literals = (*num_literals).wrapping_add(insert_length);
            insert_length = 0i32 as (usize);
            StoreRangeH41(
                hasher,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_add(2i32 as (usize)),
                brotli_min_size_t(position.wrapping_add(sr.len),store_end)
            );
            position = position.wrapping_add(sr.len);
        } else {
            insert_length = insert_length.wrapping_add(1 as (usize));
            position = position.wrapping_add(1 as (usize));
            if position > apply_random_heuristics {
                if position > apply_random_heuristics.wrapping_add(
                                  (4i32 as (usize)).wrapping_mul(random_heuristics_window_size)
                              ) {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH41().wrapping_sub(1i32 as (usize)),
                              4i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(16i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH41(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(4i32 as (usize));
                        }
                        position = position.wrapping_add(4i32 as (usize));
                    }
                } else {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH41().wrapping_sub(1i32 as (usize)),
                              2i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(8i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH41(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(2i32 as (usize));
                        }
                        position = position.wrapping_add(2i32 as (usize));
                    }
                }
            }
        }
    }
    insert_length = insert_length.wrapping_add(
                        pos_end.wrapping_sub(position)
                    );
    *last_insert_len = insert_length;
    *num_commands = (*num_commands).wrapping_add(
                        ((commands as (isize)).wrapping_sub(
                             orig_commands as (isize)
                         ) / std::mem::size_of::<*const Command>() as (isize)) as (usize)
                    );
}

unsafe extern fn StoreLookaheadH42() -> usize { 4i32 as (usize) }

unsafe extern fn PrepareDistanceCacheH42(
    mut handle : *mut u8, mut distance_cache : *mut i32
) {
    handle;
    PrepareDistanceCache(distance_cache,16i32);
}

unsafe extern fn HashTypeLengthH42() -> usize { 4i32 as (usize) }

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SlotH42 {
    pub delta : u16,
    pub next : u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BankH42 {
    pub slots : *mut SlotH42,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H42 {
    pub addr : *mut u32,
    pub head : *mut u16,
    pub tiny_hash : *mut u8,
    pub banks : *mut BankH42,
    pub free_slot_idx : *mut u16,
    pub max_hops : usize,
}

unsafe extern fn SelfH42(mut handle : *mut u8) -> *mut H42 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct1) as (*mut H42)
}

unsafe extern fn HashBytesH42(mut data : *const u8) -> usize {
    let h
        : u32
        = BROTLI_UNALIGNED_LOAD32(
              data as (*const std::os::raw::c_void)
          ).wrapping_mul(
              kHashMul32
          );
    (h >> 32i32 - 15i32) as (usize)
}

unsafe extern fn StoreH42(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let mut self : *mut H42 = SelfH42(handle);
    let key
        : usize
        = HashBytesH42(
              &*data.offset((ix & mask) as (isize)) as (*const u8)
          );
    let bank : usize = key & (512i32 - 1i32) as (usize);
    let idx
        : usize
        = (({
                let _rhs = 1;
                let _lhs = &mut *(*self).free_slot_idx.offset(bank as (isize));
                let _old = *_lhs;
                *_lhs = (*_lhs as (i32) + _rhs) as (u16);
                _old
            }) as (i32) & 512i32 - 1i32) as (usize);
    let mut delta
        : usize
        = ix.wrapping_sub(*(*self).addr.offset(key as (isize)) as (usize));
    *(*self).tiny_hash.offset(ix as (u16) as (isize)) = key as (u8);
    if delta > 0xffffi32 as (usize) {
        delta = if 0i32 != 0 { 0i32 } else { 0xffffi32 } as (usize);
    }
    (*(*(*self).banks.offset(bank as (isize))).slots.offset(
          idx as (isize)
      )).delta = delta as (u16);
    (*(*(*self).banks.offset(bank as (isize))).slots.offset(
          idx as (isize)
      )).next = *(*self).head.offset(key as (isize));
    *(*self).addr.offset(key as (isize)) = ix as (u32);
    *(*self).head.offset(key as (isize)) = idx as (u16);
}

unsafe extern fn FindLongestMatchH42(
    mut handle : *mut u8,
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut data : *const u8,
    ring_buffer_mask : usize,
    mut distance_cache : *const i32,
    cur_ix : usize,
    max_length : usize,
    max_backward : usize,
    mut out : *mut HasherSearchResult
) -> i32 {
    let mut self : *mut H42 = SelfH42(handle);
    let cur_ix_masked : usize = cur_ix & ring_buffer_mask;
    let mut is_match_found : i32 = 0i32;
    let mut best_score : usize = (*out).score;
    let mut best_len : usize = (*out).len;
    let mut i : usize;
    let key
        : usize
        = HashBytesH42(
              &*data.offset(cur_ix_masked as (isize)) as (*const u8)
          );
    let tiny_hash : u8 = key as (u8);
    (*out).len = 0i32 as (usize);
    (*out).len_x_code = 0i32 as (usize);
    i = 0i32 as (usize);
    while i < 16i32 as (usize) {
        'continue75: loop {
            {
                let backward
                    : usize
                    = *distance_cache.offset(i as (isize)) as (usize);
                let mut prev_ix : usize = cur_ix.wrapping_sub(backward);
                if i > 0i32 as (usize) && (*(*self).tiny_hash.offset(
                                                prev_ix as (u16) as (isize)
                                            ) as (i32) != tiny_hash as (i32)) {
                    if 1337i32 != 0 {
                        break 'continue75;
                    }
                }
                if prev_ix >= cur_ix || backward > max_backward {
                    if 1337i32 != 0 {
                        break 'continue75;
                    }
                }
                prev_ix = prev_ix & ring_buffer_mask;
                {
                    let len
                        : usize
                        = FindMatchLengthWithLimit(
                              &*data.offset(prev_ix as (isize)) as (*const u8),
                              &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                              max_length
                          );
                    if len >= 2i32 as (usize) {
                        let mut score
                            : usize
                            = BackwardReferenceScoreUsingLastDistance(len);
                        if best_score < score {
                            if i != 0i32 as (usize) {
                                score = score.wrapping_sub(
                                            BackwardReferencePenaltyUsingLastDistance(i)
                                        );
                            }
                            if best_score < score {
                                best_score = score;
                                best_len = len;
                                (*out).len = best_len;
                                (*out).distance = backward;
                                (*out).score = best_score;
                                is_match_found = 1i32;
                            }
                        }
                    }
                }
            }
            break;
        }
        i = i.wrapping_add(1 as (usize));
    }
    {
        let bank : usize = key & (512i32 - 1i32) as (usize);
        let mut backward : usize = 0i32 as (usize);
        let mut hops : usize = (*self).max_hops;
        let mut delta
            : usize
            = cur_ix.wrapping_sub(
                  *(*self).addr.offset(key as (isize)) as (usize)
              );
        let mut slot
            : usize
            = *(*self).head.offset(key as (isize)) as (usize);
        while {
                  let _old = hops;
                  hops = hops.wrapping_sub(1 as (usize));
                  _old
              } != 0 {
            let mut prev_ix : usize;
            let mut last : usize = slot;
            backward = backward.wrapping_add(delta);
            if backward > max_backward || 0i32 != 0 && (delta == 0) {
                if 1337i32 != 0 {
                    break;
                }
            }
            prev_ix = cur_ix.wrapping_sub(backward) & ring_buffer_mask;
            slot = (*(*(*self).banks.offset(bank as (isize))).slots.offset(
                         last as (isize)
                     )).next as (usize);
            delta = (*(*(*self).banks.offset(bank as (isize))).slots.offset(
                          last as (isize)
                      )).delta as (usize);
            if cur_ix_masked.wrapping_add(
                   best_len
               ) > ring_buffer_mask || prev_ix.wrapping_add(
                                           best_len
                                       ) > ring_buffer_mask || *data.offset(
                                                                    cur_ix_masked.wrapping_add(
                                                                        best_len
                                                                    ) as (isize)
                                                                ) as (i32) != *data.offset(
                                                                                   prev_ix.wrapping_add(
                                                                                       best_len
                                                                                   ) as (isize)
                                                                               ) as (i32) {
                if 1337i32 != 0 {
                    continue;
                }
            }
            {
                let len
                    : usize
                    = FindMatchLengthWithLimit(
                          &*data.offset(prev_ix as (isize)) as (*const u8),
                          &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                          max_length
                      );
                if len >= 4i32 as (usize) {
                    let mut score : usize = BackwardReferenceScore(len,backward);
                    if best_score < score {
                        best_score = score;
                        best_len = len;
                        (*out).len = best_len;
                        (*out).distance = backward;
                        (*out).score = best_score;
                        is_match_found = 1i32;
                    }
                }
            }
        }
        StoreH42(handle,data,ring_buffer_mask,cur_ix);
    }
    if is_match_found == 0 {
        is_match_found = SearchInStaticDictionary(
                             dictionary,
                             dictionary_hash,
                             handle,
                             &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                             max_length,
                             max_backward,
                             out,
                             0i32
                         );
    }
    is_match_found
}

unsafe extern fn StoreRangeH42(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix_start : usize,
    ix_end : usize
) {
    let mut i : usize;
    i = ix_start;
    while i < ix_end {
        {
            StoreH42(handle,data,mask,i);
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn CreateBackwardReferencesH42(
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    mut hasher : *mut u8,
    mut dist_cache : *mut i32,
    mut last_insert_len : *mut usize,
    mut commands : *mut Command,
    mut num_commands : *mut usize,
    mut num_literals : *mut usize
) {
    let max_backward_limit
        : usize
        = (1i32 as (usize) << (*params).lgwin).wrapping_sub(
              16i32 as (usize)
          );
    let orig_commands : *const Command = commands as (*const Command);
    let mut insert_length : usize = *last_insert_len;
    let pos_end : usize = position.wrapping_add(num_bytes);
    let store_end
        : usize
        = if num_bytes >= StoreLookaheadH42() {
              position.wrapping_add(num_bytes).wrapping_sub(
                  StoreLookaheadH42()
              ).wrapping_add(
                  1i32 as (usize)
              )
          } else {
              position
          };
    let random_heuristics_window_size
        : usize
        = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics
        : usize
        = position.wrapping_add(random_heuristics_window_size);
    let kMinScore
        : usize
        = ((30i32 * 8i32) as (usize)).wrapping_mul(
              std::mem::size_of::<usize>()
          ).wrapping_add(
              100i32 as (usize)
          );
    PrepareDistanceCacheH42(hasher,dist_cache);
    while position.wrapping_add(HashTypeLengthH42()) < pos_end {
        let mut max_length : usize = pos_end.wrapping_sub(position);
        let mut max_distance
            : usize
            = brotli_min_size_t(position,max_backward_limit);
        let mut sr : HasherSearchResult;
        sr.len = 0i32 as (usize);
        sr.len_x_code = 0i32 as (usize);
        sr.distance = 0i32 as (usize);
        sr.score = kMinScore;
        if FindLongestMatchH42(
               hasher,
               dictionary,
               dictionary_hash,
               ringbuffer,
               ringbuffer_mask,
               dist_cache as (*const i32),
               position,
               max_length,
               max_distance,
               &mut sr as (*mut HasherSearchResult)
           ) != 0 {
            let mut delayed_backward_references_in_row : i32 = 0i32;
            max_length = max_length.wrapping_sub(1 as (usize));
            'break76: loop {
                'continue77: loop {
                    {
                        let cost_diff_lazy : usize = 175i32 as (usize);
                        let mut is_match_found : i32;
                        let mut sr2 : HasherSearchResult;
                        sr2.len = if (*params).quality < 5i32 {
                                      brotli_min_size_t(
                                          sr.len.wrapping_sub(1i32 as (usize)),
                                          max_length
                                      )
                                  } else {
                                      0i32 as (usize)
                                  };
                        sr2.len_x_code = 0i32 as (usize);
                        sr2.distance = 0i32 as (usize);
                        sr2.score = kMinScore;
                        max_distance = brotli_min_size_t(
                                           position.wrapping_add(1i32 as (usize)),
                                           max_backward_limit
                                       );
                        is_match_found = FindLongestMatchH42(
                                             hasher,
                                             dictionary,
                                             dictionary_hash,
                                             ringbuffer,
                                             ringbuffer_mask,
                                             dist_cache as (*const i32),
                                             position.wrapping_add(1i32 as (usize)),
                                             max_length,
                                             max_distance,
                                             &mut sr2 as (*mut HasherSearchResult)
                                         );
                        if is_match_found != 0 && (sr2.score >= sr.score.wrapping_add(
                                                                    cost_diff_lazy
                                                                )) {
                            position = position.wrapping_add(1 as (usize));
                            insert_length = insert_length.wrapping_add(1 as (usize));
                            sr = sr2;
                            if {
                                   delayed_backward_references_in_row = delayed_backward_references_in_row + 1;
                                   delayed_backward_references_in_row
                               } < 4i32 && (position.wrapping_add(
                                                HashTypeLengthH42()
                                            ) < pos_end) {
                                if 1337i32 != 0 {
                                    break 'continue77;
                                }
                            }
                        }
                        {
                            if 1337i32 != 0 {
                                break 'break76;
                            }
                        }
                    }
                    break;
                }
                max_length = max_length.wrapping_sub(1 as (usize));
            }
            apply_random_heuristics = position.wrapping_add(
                                          (2i32 as (usize)).wrapping_mul(sr.len)
                                      ).wrapping_add(
                                          random_heuristics_window_size
                                      );
            max_distance = brotli_min_size_t(position,max_backward_limit);
            {
                let mut distance_code
                    : usize
                    = ComputeDistanceCode(
                          sr.distance,
                          max_distance,
                          dist_cache as (*const i32)
                      );
                if sr.distance <= max_distance && (distance_code > 0i32 as (usize)) {
                    *dist_cache.offset(3i32 as (isize)) = *dist_cache.offset(
                                                               2i32 as (isize)
                                                           );
                    *dist_cache.offset(2i32 as (isize)) = *dist_cache.offset(
                                                               1i32 as (isize)
                                                           );
                    *dist_cache.offset(1i32 as (isize)) = *dist_cache.offset(
                                                               0i32 as (isize)
                                                           );
                    *dist_cache.offset(0i32 as (isize)) = sr.distance as (i32);
                    PrepareDistanceCacheH42(hasher,dist_cache);
                }
                InitCommand(
                    {
                        let _old = commands;
                        commands = commands.offset(1 as (isize));
                        _old
                    },
                    insert_length,
                    sr.len,
                    sr.len ^ sr.len_x_code,
                    distance_code
                );
            }
            *num_literals = (*num_literals).wrapping_add(insert_length);
            insert_length = 0i32 as (usize);
            StoreRangeH42(
                hasher,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_add(2i32 as (usize)),
                brotli_min_size_t(position.wrapping_add(sr.len),store_end)
            );
            position = position.wrapping_add(sr.len);
        } else {
            insert_length = insert_length.wrapping_add(1 as (usize));
            position = position.wrapping_add(1 as (usize));
            if position > apply_random_heuristics {
                if position > apply_random_heuristics.wrapping_add(
                                  (4i32 as (usize)).wrapping_mul(random_heuristics_window_size)
                              ) {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH42().wrapping_sub(1i32 as (usize)),
                              4i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(16i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH42(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(4i32 as (usize));
                        }
                        position = position.wrapping_add(4i32 as (usize));
                    }
                } else {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH42().wrapping_sub(1i32 as (usize)),
                              2i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(8i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH42(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(2i32 as (usize));
                        }
                        position = position.wrapping_add(2i32 as (usize));
                    }
                }
            }
        }
    }
    insert_length = insert_length.wrapping_add(
                        pos_end.wrapping_sub(position)
                    );
    *last_insert_len = insert_length;
    *num_commands = (*num_commands).wrapping_add(
                        ((commands as (isize)).wrapping_sub(
                             orig_commands as (isize)
                         ) / std::mem::size_of::<*const Command>() as (isize)) as (usize)
                    );
}

unsafe extern fn StoreLookaheadH54() -> usize { 8i32 as (usize) }

unsafe extern fn PrepareDistanceCacheH54(
    mut handle : *mut u8, mut distance_cache : *mut i32
) {
    handle;
    distance_cache;
}

unsafe extern fn HashTypeLengthH54() -> usize { 8i32 as (usize) }

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H54 {
    pub buckets_ : *mut u32,
}

unsafe extern fn SelfH54(mut handle : *mut u8) -> *mut H54 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct1) as (*mut H54)
}

unsafe extern fn HashBytesH54(mut data : *const u8) -> u32 {
    let h
        : usize
        = (BROTLI_UNALIGNED_LOAD64(
               data as (*const std::os::raw::c_void)
           ) << 64i32 - 8i32 * 7i32).wrapping_mul(
              kHashMul64
          );
    (h >> 64i32 - 20i32) as (u32)
}

unsafe extern fn FindLongestMatchH54(
    mut handle : *mut u8,
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut data : *const u8,
    ring_buffer_mask : usize,
    mut distance_cache : *const i32,
    cur_ix : usize,
    max_length : usize,
    max_backward : usize,
    mut out : *mut HasherSearchResult
) -> i32 {
    let mut self : *mut H54 = SelfH54(handle);
    let best_len_in : usize = (*out).len;
    let cur_ix_masked : usize = cur_ix & ring_buffer_mask;
    let key
        : u32
        = HashBytesH54(
              &*data.offset(cur_ix_masked as (isize)) as (*const u8)
          );
    let mut compare_char
        : i32
        = *data.offset(
               cur_ix_masked.wrapping_add(best_len_in) as (isize)
           ) as (i32);
    let mut best_score : usize = (*out).score;
    let mut best_len : usize = best_len_in;
    let mut cached_backward
        : usize
        = *distance_cache.offset(0i32 as (isize)) as (usize);
    let mut prev_ix : usize = cur_ix.wrapping_sub(cached_backward);
    let mut is_match_found : i32 = 0i32;
    (*out).len_x_code = 0i32 as (usize);
    if prev_ix < cur_ix {
        prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
        if compare_char == *data.offset(
                                prev_ix.wrapping_add(best_len) as (isize)
                            ) as (i32) {
            let mut len
                : usize
                = FindMatchLengthWithLimit(
                      &*data.offset(prev_ix as (isize)) as (*const u8),
                      &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                      max_length
                  );
            if len >= 4i32 as (usize) {
                best_score = BackwardReferenceScoreUsingLastDistance(len);
                best_len = len;
                (*out).len = len;
                (*out).distance = cached_backward;
                (*out).score = best_score;
                compare_char = *data.offset(
                                    cur_ix_masked.wrapping_add(best_len) as (isize)
                                ) as (i32);
                if 4i32 == 1i32 {
                    *(*self).buckets_.offset(key as (isize)) = cur_ix as (u32);
                    return 1i32;
                } else {
                    is_match_found = 1i32;
                }
            }
        }
    }
    if 4i32 == 1i32 {
        let mut backward : usize;
        let mut len : usize;
        prev_ix = *(*self).buckets_.offset(key as (isize)) as (usize);
        *(*self).buckets_.offset(key as (isize)) = cur_ix as (u32);
        backward = cur_ix.wrapping_sub(prev_ix);
        prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
        if compare_char != *data.offset(
                                prev_ix.wrapping_add(best_len_in) as (isize)
                            ) as (i32) {
            return 0i32;
        }
        if backward == 0i32 as (usize) || backward > max_backward {
            return 0i32;
        }
        len = FindMatchLengthWithLimit(
                  &*data.offset(prev_ix as (isize)) as (*const u8),
                  &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                  max_length
              );
        if len >= 4i32 as (usize) {
            (*out).len = len;
            (*out).distance = backward;
            (*out).score = BackwardReferenceScore(len,backward);
            return 1i32;
        }
    } else {
        let mut bucket
            : *mut u32
            = (*self).buckets_.offset(key as (isize));
        let mut i : i32;
        prev_ix = *{
                       let _old = bucket;
                       bucket = bucket.offset(1 as (isize));
                       _old
                   } as (usize);
        i = 0i32;
        while i < 4i32 {
            'continue85: loop {
                {
                    let backward : usize = cur_ix.wrapping_sub(prev_ix);
                    let mut len : usize;
                    prev_ix = prev_ix & ring_buffer_mask as (u32) as (usize);
                    if compare_char != *data.offset(
                                            prev_ix.wrapping_add(best_len) as (isize)
                                        ) as (i32) {
                        if 1337i32 != 0 {
                            break 'continue85;
                        }
                    }
                    if backward == 0i32 as (usize) || backward > max_backward {
                        if 1337i32 != 0 {
                            break 'continue85;
                        }
                    }
                    len = FindMatchLengthWithLimit(
                              &*data.offset(prev_ix as (isize)) as (*const u8),
                              &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                              max_length
                          );
                    if len >= 4i32 as (usize) {
                        let score : usize = BackwardReferenceScore(len,backward);
                        if best_score < score {
                            best_score = score;
                            best_len = len;
                            (*out).len = best_len;
                            (*out).distance = backward;
                            (*out).score = score;
                            compare_char = *data.offset(
                                                cur_ix_masked.wrapping_add(best_len) as (isize)
                                            ) as (i32);
                            is_match_found = 1i32;
                        }
                    }
                }
                break;
            }
            i = i + 1;
            prev_ix = *{
                           let _old = bucket;
                           bucket = bucket.offset(1 as (isize));
                           _old
                       } as (usize);
        }
    }
    if 0i32 != 0 && (is_match_found == 0) {
        is_match_found = SearchInStaticDictionary(
                             dictionary,
                             dictionary_hash,
                             handle,
                             &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                             max_length,
                             max_backward,
                             out,
                             1i32
                         );
    }
    *(*self).buckets_.offset(
         (key as (usize)).wrapping_add(
             (cur_ix >> 3i32).wrapping_rem(4i32 as (usize))
         ) as (isize)
     ) = cur_ix as (u32);
    is_match_found
}

unsafe extern fn StoreH54(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let key
        : u32
        = HashBytesH54(
              &*data.offset((ix & mask) as (isize)) as (*const u8)
          );
    let off
        : u32
        = (ix >> 3i32).wrapping_rem(4i32 as (usize)) as (u32);
    *(*SelfH54(handle)).buckets_.offset(
         key.wrapping_add(off) as (isize)
     ) = ix as (u32);
}

unsafe extern fn StoreRangeH54(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix_start : usize,
    ix_end : usize
) {
    let mut i : usize;
    i = ix_start;
    while i < ix_end {
        {
            StoreH54(handle,data,mask,i);
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn CreateBackwardReferencesH54(
    mut dictionary : *const BrotliDictionary,
    mut dictionary_hash : *const u16,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    mut hasher : *mut u8,
    mut dist_cache : *mut i32,
    mut last_insert_len : *mut usize,
    mut commands : *mut Command,
    mut num_commands : *mut usize,
    mut num_literals : *mut usize
) {
    let max_backward_limit
        : usize
        = (1i32 as (usize) << (*params).lgwin).wrapping_sub(
              16i32 as (usize)
          );
    let orig_commands : *const Command = commands as (*const Command);
    let mut insert_length : usize = *last_insert_len;
    let pos_end : usize = position.wrapping_add(num_bytes);
    let store_end
        : usize
        = if num_bytes >= StoreLookaheadH54() {
              position.wrapping_add(num_bytes).wrapping_sub(
                  StoreLookaheadH54()
              ).wrapping_add(
                  1i32 as (usize)
              )
          } else {
              position
          };
    let random_heuristics_window_size
        : usize
        = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics
        : usize
        = position.wrapping_add(random_heuristics_window_size);
    let kMinScore
        : usize
        = ((30i32 * 8i32) as (usize)).wrapping_mul(
              std::mem::size_of::<usize>()
          ).wrapping_add(
              100i32 as (usize)
          );
    PrepareDistanceCacheH54(hasher,dist_cache);
    while position.wrapping_add(HashTypeLengthH54()) < pos_end {
        let mut max_length : usize = pos_end.wrapping_sub(position);
        let mut max_distance
            : usize
            = brotli_min_size_t(position,max_backward_limit);
        let mut sr : HasherSearchResult;
        sr.len = 0i32 as (usize);
        sr.len_x_code = 0i32 as (usize);
        sr.distance = 0i32 as (usize);
        sr.score = kMinScore;
        if FindLongestMatchH54(
               hasher,
               dictionary,
               dictionary_hash,
               ringbuffer,
               ringbuffer_mask,
               dist_cache as (*const i32),
               position,
               max_length,
               max_distance,
               &mut sr as (*mut HasherSearchResult)
           ) != 0 {
            let mut delayed_backward_references_in_row : i32 = 0i32;
            max_length = max_length.wrapping_sub(1 as (usize));
            'break86: loop {
                'continue87: loop {
                    {
                        let cost_diff_lazy : usize = 175i32 as (usize);
                        let mut is_match_found : i32;
                        let mut sr2 : HasherSearchResult;
                        sr2.len = if (*params).quality < 5i32 {
                                      brotli_min_size_t(
                                          sr.len.wrapping_sub(1i32 as (usize)),
                                          max_length
                                      )
                                  } else {
                                      0i32 as (usize)
                                  };
                        sr2.len_x_code = 0i32 as (usize);
                        sr2.distance = 0i32 as (usize);
                        sr2.score = kMinScore;
                        max_distance = brotli_min_size_t(
                                           position.wrapping_add(1i32 as (usize)),
                                           max_backward_limit
                                       );
                        is_match_found = FindLongestMatchH54(
                                             hasher,
                                             dictionary,
                                             dictionary_hash,
                                             ringbuffer,
                                             ringbuffer_mask,
                                             dist_cache as (*const i32),
                                             position.wrapping_add(1i32 as (usize)),
                                             max_length,
                                             max_distance,
                                             &mut sr2 as (*mut HasherSearchResult)
                                         );
                        if is_match_found != 0 && (sr2.score >= sr.score.wrapping_add(
                                                                    cost_diff_lazy
                                                                )) {
                            position = position.wrapping_add(1 as (usize));
                            insert_length = insert_length.wrapping_add(1 as (usize));
                            sr = sr2;
                            if {
                                   delayed_backward_references_in_row = delayed_backward_references_in_row + 1;
                                   delayed_backward_references_in_row
                               } < 4i32 && (position.wrapping_add(
                                                HashTypeLengthH54()
                                            ) < pos_end) {
                                if 1337i32 != 0 {
                                    break 'continue87;
                                }
                            }
                        }
                        {
                            if 1337i32 != 0 {
                                break 'break86;
                            }
                        }
                    }
                    break;
                }
                max_length = max_length.wrapping_sub(1 as (usize));
            }
            apply_random_heuristics = position.wrapping_add(
                                          (2i32 as (usize)).wrapping_mul(sr.len)
                                      ).wrapping_add(
                                          random_heuristics_window_size
                                      );
            max_distance = brotli_min_size_t(position,max_backward_limit);
            {
                let mut distance_code
                    : usize
                    = ComputeDistanceCode(
                          sr.distance,
                          max_distance,
                          dist_cache as (*const i32)
                      );
                if sr.distance <= max_distance && (distance_code > 0i32 as (usize)) {
                    *dist_cache.offset(3i32 as (isize)) = *dist_cache.offset(
                                                               2i32 as (isize)
                                                           );
                    *dist_cache.offset(2i32 as (isize)) = *dist_cache.offset(
                                                               1i32 as (isize)
                                                           );
                    *dist_cache.offset(1i32 as (isize)) = *dist_cache.offset(
                                                               0i32 as (isize)
                                                           );
                    *dist_cache.offset(0i32 as (isize)) = sr.distance as (i32);
                    PrepareDistanceCacheH54(hasher,dist_cache);
                }
                InitCommand(
                    {
                        let _old = commands;
                        commands = commands.offset(1 as (isize));
                        _old
                    },
                    insert_length,
                    sr.len,
                    sr.len ^ sr.len_x_code,
                    distance_code
                );
            }
            *num_literals = (*num_literals).wrapping_add(insert_length);
            insert_length = 0i32 as (usize);
            StoreRangeH54(
                hasher,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_add(2i32 as (usize)),
                brotli_min_size_t(position.wrapping_add(sr.len),store_end)
            );
            position = position.wrapping_add(sr.len);
        } else {
            insert_length = insert_length.wrapping_add(1 as (usize));
            position = position.wrapping_add(1 as (usize));
            if position > apply_random_heuristics {
                if position > apply_random_heuristics.wrapping_add(
                                  (4i32 as (usize)).wrapping_mul(random_heuristics_window_size)
                              ) {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH54().wrapping_sub(1i32 as (usize)),
                              4i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(16i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH54(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(4i32 as (usize));
                        }
                        position = position.wrapping_add(4i32 as (usize));
                    }
                } else {
                    let kMargin
                        : usize
                        = brotli_max_size_t(
                              StoreLookaheadH54().wrapping_sub(1i32 as (usize)),
                              2i32 as (usize)
                          );
                    let mut pos_jump
                        : usize
                        = brotli_min_size_t(
                              position.wrapping_add(8i32 as (usize)),
                              pos_end.wrapping_sub(kMargin)
                          );
                    while position < pos_jump {
                        {
                            StoreH54(hasher,ringbuffer,ringbuffer_mask,position);
                            insert_length = insert_length.wrapping_add(2i32 as (usize));
                        }
                        position = position.wrapping_add(2i32 as (usize));
                    }
                }
            }
        }
    }
    insert_length = insert_length.wrapping_add(
                        pos_end.wrapping_sub(position)
                    );
    *last_insert_len = insert_length;
    *num_commands = (*num_commands).wrapping_add(
                        ((commands as (isize)).wrapping_sub(
                             orig_commands as (isize)
                         ) / std::mem::size_of::<*const Command>() as (isize)) as (usize)
                    );
}

#[no_mangle]
pub unsafe extern fn BrotliCreateBackwardReferences(
    mut dictionary : *const BrotliDictionary,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    mut hasher : *mut u8,
    mut dist_cache : *mut i32,
    mut last_insert_len : *mut usize,
    mut commands : *mut Command,
    mut num_commands : *mut usize,
    mut num_literals : *mut usize
) {
    let mut hasher_type : i32 = (*params).hasher.type_;
    if hasher_type == 2i32 {
        CreateBackwardReferencesH2(
            dictionary,
            kStaticDictionaryHash,
            num_bytes,
            position,
            ringbuffer,
            ringbuffer_mask,
            params,
            hasher,
            dist_cache,
            last_insert_len,
            commands,
            num_commands,
            num_literals
        );
    }
    if hasher_type == 3i32 {
        CreateBackwardReferencesH3(
            dictionary,
            kStaticDictionaryHash,
            num_bytes,
            position,
            ringbuffer,
            ringbuffer_mask,
            params,
            hasher,
            dist_cache,
            last_insert_len,
            commands,
            num_commands,
            num_literals
        );
    }
    if hasher_type == 4i32 {
        CreateBackwardReferencesH4(
            dictionary,
            kStaticDictionaryHash,
            num_bytes,
            position,
            ringbuffer,
            ringbuffer_mask,
            params,
            hasher,
            dist_cache,
            last_insert_len,
            commands,
            num_commands,
            num_literals
        );
    }
    if hasher_type == 5i32 {
        CreateBackwardReferencesH5(
            dictionary,
            kStaticDictionaryHash,
            num_bytes,
            position,
            ringbuffer,
            ringbuffer_mask,
            params,
            hasher,
            dist_cache,
            last_insert_len,
            commands,
            num_commands,
            num_literals
        );
    }
    if hasher_type == 6i32 {
        CreateBackwardReferencesH6(
            dictionary,
            kStaticDictionaryHash,
            num_bytes,
            position,
            ringbuffer,
            ringbuffer_mask,
            params,
            hasher,
            dist_cache,
            last_insert_len,
            commands,
            num_commands,
            num_literals
        );
    }
    if hasher_type == 40i32 {
        CreateBackwardReferencesH40(
            dictionary,
            kStaticDictionaryHash,
            num_bytes,
            position,
            ringbuffer,
            ringbuffer_mask,
            params,
            hasher,
            dist_cache,
            last_insert_len,
            commands,
            num_commands,
            num_literals
        );
    }
    if hasher_type == 41i32 {
        CreateBackwardReferencesH41(
            dictionary,
            kStaticDictionaryHash,
            num_bytes,
            position,
            ringbuffer,
            ringbuffer_mask,
            params,
            hasher,
            dist_cache,
            last_insert_len,
            commands,
            num_commands,
            num_literals
        );
    }
    if hasher_type == 42i32 {
        CreateBackwardReferencesH42(
            dictionary,
            kStaticDictionaryHash,
            num_bytes,
            position,
            ringbuffer,
            ringbuffer_mask,
            params,
            hasher,
            dist_cache,
            last_insert_len,
            commands,
            num_commands,
            num_literals
        );
    }
    if hasher_type == 54i32 {
        CreateBackwardReferencesH54(
            dictionary,
            kStaticDictionaryHash,
            num_bytes,
            position,
            ringbuffer,
            ringbuffer_mask,
            params,
            hasher,
            dist_cache,
            last_insert_len,
            commands,
            num_commands,
            num_literals
        );
    }
}
