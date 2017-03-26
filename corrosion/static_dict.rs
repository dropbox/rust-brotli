extern {
    fn memcpy(
        __dest : *mut std::os::raw::c_void,
        __src : *const std::os::raw::c_void,
        __n : usize
    ) -> *mut std::os::raw::c_void;
}

static kInvalidMatch : u32 = 0xfffffffi32 as (u32);

static kDictNumBits : i32 = 15i32;

static kDictHashMul32 : u32 = 0x1e35a7bdi32 as (u32);

static mut kStaticDictionaryBuckets
    : *const u16
    = 1i32 as (*const u16);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct DictWord {
    pub len : u8,
    pub transform : u8,
    pub idx : u16,
}

static mut kStaticDictionaryWords
    : *const DictWord
    = 0i32 as (*const DictWord);

static kUppercaseFirst : u8 = 10i32 as (u8);

static mut kOmitLastNTransforms : *const u8 = 0i32 as (*const u8);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length : *mut u8,
    pub offsets_by_length : *mut u32,
    pub data : *mut u8,
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

unsafe extern fn Hash(mut data : *const u8) -> u32 {
    let mut h
        : u32
        = BROTLI_UNALIGNED_LOAD32(
              data as (*const std::os::raw::c_void)
          ).wrapping_mul(
              kDictHashMul32
          );
    h >> 32i32 - kDictNumBits
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

unsafe extern fn brotli_min_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a < b { a } else { b }
}

unsafe extern fn DictMatchLength(
    mut dictionary : *const BrotliDictionary,
    mut data : *const u8,
    mut id : usize,
    mut len : usize,
    mut maxlen : usize
) -> usize {
    let offset
        : usize
        = (*(*dictionary).offsets_by_length.offset(
                len as (isize)
            ) as (usize)).wrapping_add(
              len.wrapping_mul(id)
          );
    FindMatchLengthWithLimit(
        &mut *(*dictionary).data.offset(
                  offset as (isize)
              ) as (*mut u8) as (*const u8),
        data,
        brotli_min_size_t(len,maxlen)
    )
}

unsafe extern fn brotli_min_uint32_t(
    mut a : u32, mut b : u32
) -> u32 {
    if a < b { a } else { b }
}

unsafe extern fn AddMatch(
    mut distance : usize,
    mut len : usize,
    mut len_code : usize,
    mut matches : *mut u32
) {
    let mut match_
        : u32
        = (distance << 5i32).wrapping_add(len_code) as (u32);
    *matches.offset(len as (isize)) = brotli_min_uint32_t(
                                          *matches.offset(len as (isize)),
                                          match_
                                      );
}

unsafe extern fn brotli_max_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a > b { a } else { b }
}

unsafe extern fn IsMatch(
    mut dictionary : *const BrotliDictionary,
    mut w : DictWord,
    mut data : *const u8,
    mut max_length : usize
) -> i32 {
    if w.len as (usize) > max_length {
        0i32
    } else {
        let offset
            : usize
            = (*(*dictionary).offsets_by_length.offset(
                    w.len as (isize)
                ) as (usize)).wrapping_add(
                  (w.len as (usize)).wrapping_mul(w.idx as (usize))
              );
        let mut dict
            : *const u8
            = &mut *(*dictionary).data.offset(
                        offset as (isize)
                    ) as (*mut u8) as (*const u8);
        if w.transform as (i32) == 0i32 {
            if !!(FindMatchLengthWithLimit(
                      dict,
                      data,
                      w.len as (usize)
                  ) == w.len as (usize)) {
                1i32
            } else {
                0i32
            }
        } else if w.transform as (i32) == 10i32 {
            if !!(*dict.offset(
                       0i32 as (isize)
                   ) as (i32) >= b'a' as (i32) && (*dict.offset(
                                                        0i32 as (isize)
                                                    ) as (i32) <= b'z' as (i32)) && (*dict.offset(
                                                                                          0i32 as (isize)
                                                                                      ) as (i32) ^ 32i32 == *data.offset(
                                                                                                                 0i32 as (isize)
                                                                                                             ) as (i32)) && (FindMatchLengthWithLimit(
                                                                                                                                 &*dict.offset(
                                                                                                                                       1i32 as (isize)
                                                                                                                                   ) as (*const u8),
                                                                                                                                 &*data.offset(
                                                                                                                                       1i32 as (isize)
                                                                                                                                   ) as (*const u8),
                                                                                                                                 (w.len as (u32)).wrapping_sub(
                                                                                                                                     1u32
                                                                                                                                 ) as (usize)
                                                                                                                             ) == (w.len as (u32)).wrapping_sub(
                                                                                                                                      1u32
                                                                                                                                  ) as (usize))) {
                1i32
            } else {
                0i32
            }
        } else {
            let mut i : usize;
            i = 0i32 as (usize);
            while i < w.len as (usize) {
                {
                    if *dict.offset(
                            i as (isize)
                        ) as (i32) >= b'a' as (i32) && (*dict.offset(
                                                             i as (isize)
                                                         ) as (i32) <= b'z' as (i32)) {
                        if *dict.offset(i as (isize)) as (i32) ^ 32i32 != *data.offset(
                                                                               i as (isize)
                                                                           ) as (i32) {
                            return 0i32;
                        }
                    } else if *dict.offset(i as (isize)) as (i32) != *data.offset(
                                                                          i as (isize)
                                                                      ) as (i32) {
                        return 0i32;
                    }
                }
                i = i.wrapping_add(1 as (usize));
            }
            1i32
        }
    }
}

#[no_mangle]
pub unsafe extern fn BrotliFindAllStaticDictionaryMatches(
    mut dictionary : *const BrotliDictionary,
    mut data : *const u8,
    mut min_length : usize,
    mut max_length : usize,
    mut matches : *mut u32
) -> i32 {
    let mut has_found_match : i32 = 0i32;
    {
        let mut offset
            : usize
            = *kStaticDictionaryBuckets.offset(
                   Hash(data) as (isize)
               ) as (usize);
        let mut end : i32 = (offset == 0) as (i32);
        while end == 0 {
            let mut w
                : DictWord
                = *kStaticDictionaryWords.offset(
                       {
                           let _old = offset;
                           offset = offset.wrapping_add(1 as (usize));
                           _old
                       } as (isize)
                   );
            let l : usize = (w.len as (i32) & 0x1fi32) as (usize);
            let n
                : usize
                = 1i32 as (usize) << *(*dictionary).size_bits_by_length.offset(
                                          l as (isize)
                                      ) as (i32);
            let id : usize = w.idx as (usize);
            end = !(w.len as (i32) & 0x80i32 == 0) as (i32);
            w.len = l as (u8);
            if w.transform as (i32) == 0i32 {
                let matchlen
                    : usize
                    = DictMatchLength(dictionary,data,id,l,max_length);
                let mut s : *const u8;
                let mut minlen : usize;
                let mut maxlen : usize;
                let mut len : usize;
                if matchlen == l {
                    AddMatch(id,l,l,matches);
                    has_found_match = 1i32;
                }
                if matchlen >= l.wrapping_sub(1i32 as (usize)) {
                    AddMatch(
                        id.wrapping_add((12i32 as (usize)).wrapping_mul(n)),
                        l.wrapping_sub(1i32 as (usize)),
                        l,
                        matches
                    );
                    if l.wrapping_add(2i32 as (usize)) < max_length && (*data.offset(
                                                                             l.wrapping_sub(
                                                                                 1i32 as (usize)
                                                                             ) as (isize)
                                                                         ) as (i32) == b'i' as (i32)) && (*data.offset(
                                                                                                               l as (isize)
                                                                                                           ) as (i32) == b'n' as (i32)) && (*data.offset(
                                                                                                                                                 l.wrapping_add(
                                                                                                                                                     1i32 as (usize)
                                                                                                                                                 ) as (isize)
                                                                                                                                             ) as (i32) == b'g' as (i32)) && (*data.offset(
                                                                                                                                                                                   l.wrapping_add(
                                                                                                                                                                                       2i32 as (usize)
                                                                                                                                                                                   ) as (isize)
                                                                                                                                                                               ) as (i32) == b' ' as (i32)) {
                        AddMatch(
                            id.wrapping_add((49i32 as (usize)).wrapping_mul(n)),
                            l.wrapping_add(3i32 as (usize)),
                            l,
                            matches
                        );
                    }
                    has_found_match = 1i32;
                }
                minlen = min_length;
                if l > 9i32 as (usize) {
                    minlen = brotli_max_size_t(minlen,l.wrapping_sub(9i32 as (usize)));
                }
                maxlen = brotli_min_size_t(
                             matchlen,
                             l.wrapping_sub(2i32 as (usize))
                         );
                len = minlen;
                while len <= maxlen {
                    {
                        AddMatch(
                            id.wrapping_add(
                                (*kOmitLastNTransforms.offset(
                                      l.wrapping_sub(len) as (isize)
                                  ) as (usize)).wrapping_mul(
                                    n
                                )
                            ),
                            len,
                            l,
                            matches
                        );
                        has_found_match = 1i32;
                    }
                    len = len.wrapping_add(1 as (usize));
                }
                if matchlen < l || l.wrapping_add(6i32 as (usize)) >= max_length {
                    if 1337i32 != 0 {
                        continue;
                    }
                }
                s = &*data.offset(l as (isize)) as (*const u8);
                if *s.offset(0i32 as (isize)) as (i32) == b' ' as (i32) {
                    AddMatch(
                        id.wrapping_add(n),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                    if *s.offset(1i32 as (isize)) as (i32) == b'a' as (i32) {
                        if *s.offset(2i32 as (isize)) as (i32) == b' ' as (i32) {
                            AddMatch(
                                id.wrapping_add((28i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(3i32 as (usize)),
                                l,
                                matches
                            );
                        } else if *s.offset(2i32 as (isize)) as (i32) == b's' as (i32) {
                            if *s.offset(3i32 as (isize)) as (i32) == b' ' as (i32) {
                                AddMatch(
                                    id.wrapping_add((46i32 as (usize)).wrapping_mul(n)),
                                    l.wrapping_add(4i32 as (usize)),
                                    l,
                                    matches
                                );
                            }
                        } else if *s.offset(2i32 as (isize)) as (i32) == b't' as (i32) {
                            if *s.offset(3i32 as (isize)) as (i32) == b' ' as (i32) {
                                AddMatch(
                                    id.wrapping_add((60i32 as (usize)).wrapping_mul(n)),
                                    l.wrapping_add(4i32 as (usize)),
                                    l,
                                    matches
                                );
                            }
                        } else if *s.offset(2i32 as (isize)) as (i32) == b'n' as (i32) {
                            if *s.offset(
                                    3i32 as (isize)
                                ) as (i32) == b'd' as (i32) && (*s.offset(
                                                                     4i32 as (isize)
                                                                 ) as (i32) == b' ' as (i32)) {
                                AddMatch(
                                    id.wrapping_add((10i32 as (usize)).wrapping_mul(n)),
                                    l.wrapping_add(5i32 as (usize)),
                                    l,
                                    matches
                                );
                            }
                        }
                    } else if *s.offset(1i32 as (isize)) as (i32) == b'b' as (i32) {
                        if *s.offset(
                                2i32 as (isize)
                            ) as (i32) == b'y' as (i32) && (*s.offset(
                                                                 3i32 as (isize)
                                                             ) as (i32) == b' ' as (i32)) {
                            AddMatch(
                                id.wrapping_add((38i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(4i32 as (usize)),
                                l,
                                matches
                            );
                        }
                    } else if *s.offset(1i32 as (isize)) as (i32) == b'i' as (i32) {
                        if *s.offset(2i32 as (isize)) as (i32) == b'n' as (i32) {
                            if *s.offset(3i32 as (isize)) as (i32) == b' ' as (i32) {
                                AddMatch(
                                    id.wrapping_add((16i32 as (usize)).wrapping_mul(n)),
                                    l.wrapping_add(4i32 as (usize)),
                                    l,
                                    matches
                                );
                            }
                        } else if *s.offset(2i32 as (isize)) as (i32) == b's' as (i32) {
                            if *s.offset(3i32 as (isize)) as (i32) == b' ' as (i32) {
                                AddMatch(
                                    id.wrapping_add((47i32 as (usize)).wrapping_mul(n)),
                                    l.wrapping_add(4i32 as (usize)),
                                    l,
                                    matches
                                );
                            }
                        }
                    } else if *s.offset(1i32 as (isize)) as (i32) == b'f' as (i32) {
                        if *s.offset(2i32 as (isize)) as (i32) == b'o' as (i32) {
                            if *s.offset(
                                    3i32 as (isize)
                                ) as (i32) == b'r' as (i32) && (*s.offset(
                                                                     4i32 as (isize)
                                                                 ) as (i32) == b' ' as (i32)) {
                                AddMatch(
                                    id.wrapping_add((25i32 as (usize)).wrapping_mul(n)),
                                    l.wrapping_add(5i32 as (usize)),
                                    l,
                                    matches
                                );
                            }
                        } else if *s.offset(2i32 as (isize)) as (i32) == b'r' as (i32) {
                            if *s.offset(
                                    3i32 as (isize)
                                ) as (i32) == b'o' as (i32) && (*s.offset(
                                                                     4i32 as (isize)
                                                                 ) as (i32) == b'm' as (i32)) && (*s.offset(
                                                                                                       5i32 as (isize)
                                                                                                   ) as (i32) == b' ' as (i32)) {
                                AddMatch(
                                    id.wrapping_add((37i32 as (usize)).wrapping_mul(n)),
                                    l.wrapping_add(6i32 as (usize)),
                                    l,
                                    matches
                                );
                            }
                        }
                    } else if *s.offset(1i32 as (isize)) as (i32) == b'o' as (i32) {
                        if *s.offset(2i32 as (isize)) as (i32) == b'f' as (i32) {
                            if *s.offset(3i32 as (isize)) as (i32) == b' ' as (i32) {
                                AddMatch(
                                    id.wrapping_add((8i32 as (usize)).wrapping_mul(n)),
                                    l.wrapping_add(4i32 as (usize)),
                                    l,
                                    matches
                                );
                            }
                        } else if *s.offset(2i32 as (isize)) as (i32) == b'n' as (i32) {
                            if *s.offset(3i32 as (isize)) as (i32) == b' ' as (i32) {
                                AddMatch(
                                    id.wrapping_add((45i32 as (usize)).wrapping_mul(n)),
                                    l.wrapping_add(4i32 as (usize)),
                                    l,
                                    matches
                                );
                            }
                        }
                    } else if *s.offset(1i32 as (isize)) as (i32) == b'n' as (i32) {
                        if *s.offset(
                                2i32 as (isize)
                            ) as (i32) == b'o' as (i32) && (*s.offset(
                                                                 3i32 as (isize)
                                                             ) as (i32) == b't' as (i32)) && (*s.offset(
                                                                                                   4i32 as (isize)
                                                                                               ) as (i32) == b' ' as (i32)) {
                            AddMatch(
                                id.wrapping_add((80i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(5i32 as (usize)),
                                l,
                                matches
                            );
                        }
                    } else if *s.offset(1i32 as (isize)) as (i32) == b't' as (i32) {
                        if *s.offset(2i32 as (isize)) as (i32) == b'h' as (i32) {
                            if *s.offset(3i32 as (isize)) as (i32) == b'e' as (i32) {
                                if *s.offset(4i32 as (isize)) as (i32) == b' ' as (i32) {
                                    AddMatch(
                                        id.wrapping_add((5i32 as (usize)).wrapping_mul(n)),
                                        l.wrapping_add(5i32 as (usize)),
                                        l,
                                        matches
                                    );
                                }
                            } else if *s.offset(3i32 as (isize)) as (i32) == b'a' as (i32) {
                                if *s.offset(
                                        4i32 as (isize)
                                    ) as (i32) == b't' as (i32) && (*s.offset(
                                                                         5i32 as (isize)
                                                                     ) as (i32) == b' ' as (i32)) {
                                    AddMatch(
                                        id.wrapping_add((29i32 as (usize)).wrapping_mul(n)),
                                        l.wrapping_add(6i32 as (usize)),
                                        l,
                                        matches
                                    );
                                }
                            }
                        } else if *s.offset(2i32 as (isize)) as (i32) == b'o' as (i32) {
                            if *s.offset(3i32 as (isize)) as (i32) == b' ' as (i32) {
                                AddMatch(
                                    id.wrapping_add((17i32 as (usize)).wrapping_mul(n)),
                                    l.wrapping_add(4i32 as (usize)),
                                    l,
                                    matches
                                );
                            }
                        }
                    } else if *s.offset(1i32 as (isize)) as (i32) == b'w' as (i32) {
                        if *s.offset(
                                2i32 as (isize)
                            ) as (i32) == b'i' as (i32) && (*s.offset(
                                                                 3i32 as (isize)
                                                             ) as (i32) == b't' as (i32)) && (*s.offset(
                                                                                                   4i32 as (isize)
                                                                                               ) as (i32) == b'h' as (i32)) && (*s.offset(
                                                                                                                                     5i32 as (isize)
                                                                                                                                 ) as (i32) == b' ' as (i32)) {
                            AddMatch(
                                id.wrapping_add((35i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(6i32 as (usize)),
                                l,
                                matches
                            );
                        }
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b'\"' as (i32) {
                    AddMatch(
                        id.wrapping_add((19i32 as (usize)).wrapping_mul(n)),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                    if *s.offset(1i32 as (isize)) as (i32) == b'>' as (i32) {
                        AddMatch(
                            id.wrapping_add((21i32 as (usize)).wrapping_mul(n)),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b'.' as (i32) {
                    AddMatch(
                        id.wrapping_add((20i32 as (usize)).wrapping_mul(n)),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                    if *s.offset(1i32 as (isize)) as (i32) == b' ' as (i32) {
                        AddMatch(
                            id.wrapping_add((31i32 as (usize)).wrapping_mul(n)),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                        if *s.offset(
                                2i32 as (isize)
                            ) as (i32) == b'T' as (i32) && (*s.offset(
                                                                 3i32 as (isize)
                                                             ) as (i32) == b'h' as (i32)) {
                            if *s.offset(4i32 as (isize)) as (i32) == b'e' as (i32) {
                                if *s.offset(5i32 as (isize)) as (i32) == b' ' as (i32) {
                                    AddMatch(
                                        id.wrapping_add((43i32 as (usize)).wrapping_mul(n)),
                                        l.wrapping_add(6i32 as (usize)),
                                        l,
                                        matches
                                    );
                                }
                            } else if *s.offset(4i32 as (isize)) as (i32) == b'i' as (i32) {
                                if *s.offset(
                                        5i32 as (isize)
                                    ) as (i32) == b's' as (i32) && (*s.offset(
                                                                         6i32 as (isize)
                                                                     ) as (i32) == b' ' as (i32)) {
                                    AddMatch(
                                        id.wrapping_add((75i32 as (usize)).wrapping_mul(n)),
                                        l.wrapping_add(7i32 as (usize)),
                                        l,
                                        matches
                                    );
                                }
                            }
                        }
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b',' as (i32) {
                    AddMatch(
                        id.wrapping_add((76i32 as (usize)).wrapping_mul(n)),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                    if *s.offset(1i32 as (isize)) as (i32) == b' ' as (i32) {
                        AddMatch(
                            id.wrapping_add((14i32 as (usize)).wrapping_mul(n)),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b'\n' as (i32) {
                    AddMatch(
                        id.wrapping_add((22i32 as (usize)).wrapping_mul(n)),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                    if *s.offset(1i32 as (isize)) as (i32) == b'\t' as (i32) {
                        AddMatch(
                            id.wrapping_add((50i32 as (usize)).wrapping_mul(n)),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b']' as (i32) {
                    AddMatch(
                        id.wrapping_add((24i32 as (usize)).wrapping_mul(n)),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                } else if *s.offset(0i32 as (isize)) as (i32) == b'\'' as (i32) {
                    AddMatch(
                        id.wrapping_add((36i32 as (usize)).wrapping_mul(n)),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                } else if *s.offset(0i32 as (isize)) as (i32) == b':' as (i32) {
                    AddMatch(
                        id.wrapping_add((51i32 as (usize)).wrapping_mul(n)),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                } else if *s.offset(0i32 as (isize)) as (i32) == b'(' as (i32) {
                    AddMatch(
                        id.wrapping_add((57i32 as (usize)).wrapping_mul(n)),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                } else if *s.offset(0i32 as (isize)) as (i32) == b'=' as (i32) {
                    if *s.offset(1i32 as (isize)) as (i32) == b'\"' as (i32) {
                        AddMatch(
                            id.wrapping_add((70i32 as (usize)).wrapping_mul(n)),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                    } else if *s.offset(1i32 as (isize)) as (i32) == b'\'' as (i32) {
                        AddMatch(
                            id.wrapping_add((86i32 as (usize)).wrapping_mul(n)),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b'a' as (i32) {
                    if *s.offset(
                            1i32 as (isize)
                        ) as (i32) == b'l' as (i32) && (*s.offset(
                                                             2i32 as (isize)
                                                         ) as (i32) == b' ' as (i32)) {
                        AddMatch(
                            id.wrapping_add((84i32 as (usize)).wrapping_mul(n)),
                            l.wrapping_add(3i32 as (usize)),
                            l,
                            matches
                        );
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b'e' as (i32) {
                    if *s.offset(1i32 as (isize)) as (i32) == b'd' as (i32) {
                        if *s.offset(2i32 as (isize)) as (i32) == b' ' as (i32) {
                            AddMatch(
                                id.wrapping_add((53i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(3i32 as (usize)),
                                l,
                                matches
                            );
                        }
                    } else if *s.offset(1i32 as (isize)) as (i32) == b'r' as (i32) {
                        if *s.offset(2i32 as (isize)) as (i32) == b' ' as (i32) {
                            AddMatch(
                                id.wrapping_add((82i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(3i32 as (usize)),
                                l,
                                matches
                            );
                        }
                    } else if *s.offset(1i32 as (isize)) as (i32) == b's' as (i32) {
                        if *s.offset(
                                2i32 as (isize)
                            ) as (i32) == b't' as (i32) && (*s.offset(
                                                                 3i32 as (isize)
                                                             ) as (i32) == b' ' as (i32)) {
                            AddMatch(
                                id.wrapping_add((95i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(4i32 as (usize)),
                                l,
                                matches
                            );
                        }
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b'f' as (i32) {
                    if *s.offset(
                            1i32 as (isize)
                        ) as (i32) == b'u' as (i32) && (*s.offset(
                                                             2i32 as (isize)
                                                         ) as (i32) == b'l' as (i32)) && (*s.offset(
                                                                                               3i32 as (isize)
                                                                                           ) as (i32) == b' ' as (i32)) {
                        AddMatch(
                            id.wrapping_add((90i32 as (usize)).wrapping_mul(n)),
                            l.wrapping_add(4i32 as (usize)),
                            l,
                            matches
                        );
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b'i' as (i32) {
                    if *s.offset(1i32 as (isize)) as (i32) == b'v' as (i32) {
                        if *s.offset(
                                2i32 as (isize)
                            ) as (i32) == b'e' as (i32) && (*s.offset(
                                                                 3i32 as (isize)
                                                             ) as (i32) == b' ' as (i32)) {
                            AddMatch(
                                id.wrapping_add((92i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(4i32 as (usize)),
                                l,
                                matches
                            );
                        }
                    } else if *s.offset(1i32 as (isize)) as (i32) == b'z' as (i32) {
                        if *s.offset(
                                2i32 as (isize)
                            ) as (i32) == b'e' as (i32) && (*s.offset(
                                                                 3i32 as (isize)
                                                             ) as (i32) == b' ' as (i32)) {
                            AddMatch(
                                id.wrapping_add((100i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(4i32 as (usize)),
                                l,
                                matches
                            );
                        }
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b'l' as (i32) {
                    if *s.offset(1i32 as (isize)) as (i32) == b'e' as (i32) {
                        if *s.offset(
                                2i32 as (isize)
                            ) as (i32) == b's' as (i32) && (*s.offset(
                                                                 3i32 as (isize)
                                                             ) as (i32) == b's' as (i32)) && (*s.offset(
                                                                                                   4i32 as (isize)
                                                                                               ) as (i32) == b' ' as (i32)) {
                            AddMatch(
                                id.wrapping_add((93i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(5i32 as (usize)),
                                l,
                                matches
                            );
                        }
                    } else if *s.offset(1i32 as (isize)) as (i32) == b'y' as (i32) {
                        if *s.offset(2i32 as (isize)) as (i32) == b' ' as (i32) {
                            AddMatch(
                                id.wrapping_add((61i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(3i32 as (usize)),
                                l,
                                matches
                            );
                        }
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b'o' as (i32) {
                    if *s.offset(
                            1i32 as (isize)
                        ) as (i32) == b'u' as (i32) && (*s.offset(
                                                             2i32 as (isize)
                                                         ) as (i32) == b's' as (i32)) && (*s.offset(
                                                                                               3i32 as (isize)
                                                                                           ) as (i32) == b' ' as (i32)) {
                        AddMatch(
                            id.wrapping_add((106i32 as (usize)).wrapping_mul(n)),
                            l.wrapping_add(4i32 as (usize)),
                            l,
                            matches
                        );
                    }
                }
            } else {
                let is_all_caps
                    : i32
                    = if !!(w.transform as (i32) != kUppercaseFirst as (i32)) {
                          1i32
                      } else {
                          0i32
                      };
                let mut s : *const u8;
                if IsMatch(dictionary,w,data,max_length) == 0 {
                    if 1337i32 != 0 {
                        continue;
                    }
                }
                AddMatch(
                    id.wrapping_add(
                        (if is_all_caps != 0 {
                             44i32
                         } else {
                             9i32
                         } as (usize)).wrapping_mul(
                            n
                        )
                    ),
                    l,
                    l,
                    matches
                );
                has_found_match = 1i32;
                if l.wrapping_add(1i32 as (usize)) >= max_length {
                    if 1337i32 != 0 {
                        continue;
                    }
                }
                s = &*data.offset(l as (isize)) as (*const u8);
                if *s.offset(0i32 as (isize)) as (i32) == b' ' as (i32) {
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps != 0 {
                                 68i32
                             } else {
                                 4i32
                             } as (usize)).wrapping_mul(
                                n
                            )
                        ),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                } else if *s.offset(0i32 as (isize)) as (i32) == b'\"' as (i32) {
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps != 0 {
                                 87i32
                             } else {
                                 66i32
                             } as (usize)).wrapping_mul(
                                n
                            )
                        ),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                    if *s.offset(1i32 as (isize)) as (i32) == b'>' as (i32) {
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps != 0 {
                                     97i32
                                 } else {
                                     69i32
                                 } as (usize)).wrapping_mul(
                                    n
                                )
                            ),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b'.' as (i32) {
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps != 0 {
                                 101i32
                             } else {
                                 79i32
                             } as (usize)).wrapping_mul(
                                n
                            )
                        ),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                    if *s.offset(1i32 as (isize)) as (i32) == b' ' as (i32) {
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps != 0 {
                                     114i32
                                 } else {
                                     88i32
                                 } as (usize)).wrapping_mul(
                                    n
                                )
                            ),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b',' as (i32) {
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps != 0 {
                                 112i32
                             } else {
                                 99i32
                             } as (usize)).wrapping_mul(
                                n
                            )
                        ),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                    if *s.offset(1i32 as (isize)) as (i32) == b' ' as (i32) {
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps != 0 {
                                     107i32
                                 } else {
                                     58i32
                                 } as (usize)).wrapping_mul(
                                    n
                                )
                            ),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b'\'' as (i32) {
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps != 0 {
                                 94i32
                             } else {
                                 74i32
                             } as (usize)).wrapping_mul(
                                n
                            )
                        ),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                } else if *s.offset(0i32 as (isize)) as (i32) == b'(' as (i32) {
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps != 0 {
                                 113i32
                             } else {
                                 78i32
                             } as (usize)).wrapping_mul(
                                n
                            )
                        ),
                        l.wrapping_add(1i32 as (usize)),
                        l,
                        matches
                    );
                } else if *s.offset(0i32 as (isize)) as (i32) == b'=' as (i32) {
                    if *s.offset(1i32 as (isize)) as (i32) == b'\"' as (i32) {
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps != 0 {
                                     105i32
                                 } else {
                                     104i32
                                 } as (usize)).wrapping_mul(
                                    n
                                )
                            ),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                    } else if *s.offset(1i32 as (isize)) as (i32) == b'\'' as (i32) {
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps != 0 {
                                     116i32
                                 } else {
                                     108i32
                                 } as (usize)).wrapping_mul(
                                    n
                                )
                            ),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                    }
                }
            }
        }
    }
    if max_length >= 5i32 as (usize) && (*data.offset(
                                              0i32 as (isize)
                                          ) as (i32) == b' ' as (i32) || *data.offset(
                                                                              0i32 as (isize)
                                                                          ) as (i32) == b'.' as (i32)) {
        let mut is_space
            : i32
            = if !!(*data.offset(0i32 as (isize)) as (i32) == b' ' as (i32)) {
                  1i32
              } else {
                  0i32
              };
        let mut offset
            : usize
            = *kStaticDictionaryBuckets.offset(
                   Hash(&*data.offset(1i32 as (isize)) as (*const u8)) as (isize)
               ) as (usize);
        let mut end : i32 = (offset == 0) as (i32);
        while end == 0 {
            let mut w
                : DictWord
                = *kStaticDictionaryWords.offset(
                       {
                           let _old = offset;
                           offset = offset.wrapping_add(1 as (usize));
                           _old
                       } as (isize)
                   );
            let l : usize = (w.len as (i32) & 0x1fi32) as (usize);
            let n
                : usize
                = 1i32 as (usize) << *(*dictionary).size_bits_by_length.offset(
                                          l as (isize)
                                      ) as (i32);
            let id : usize = w.idx as (usize);
            end = !(w.len as (i32) & 0x80i32 == 0) as (i32);
            w.len = l as (u8);
            if w.transform as (i32) == 0i32 {
                let mut s : *const u8;
                if IsMatch(
                       dictionary,
                       w,
                       &*data.offset(1i32 as (isize)) as (*const u8),
                       max_length.wrapping_sub(1i32 as (usize))
                   ) == 0 {
                    if 1337i32 != 0 {
                        continue;
                    }
                }
                AddMatch(
                    id.wrapping_add(
                        (if is_space != 0 { 6i32 } else { 32i32 } as (usize)).wrapping_mul(
                            n
                        )
                    ),
                    l.wrapping_add(1i32 as (usize)),
                    l,
                    matches
                );
                has_found_match = 1i32;
                if l.wrapping_add(2i32 as (usize)) >= max_length {
                    if 1337i32 != 0 {
                        continue;
                    }
                }
                s = &*data.offset(
                          l.wrapping_add(1i32 as (usize)) as (isize)
                      ) as (*const u8);
                if *s.offset(0i32 as (isize)) as (i32) == b' ' as (i32) {
                    AddMatch(
                        id.wrapping_add(
                            (if is_space != 0 { 2i32 } else { 77i32 } as (usize)).wrapping_mul(
                                n
                            )
                        ),
                        l.wrapping_add(2i32 as (usize)),
                        l,
                        matches
                    );
                } else if *s.offset(0i32 as (isize)) as (i32) == b'(' as (i32) {
                    AddMatch(
                        id.wrapping_add(
                            (if is_space != 0 {
                                 89i32
                             } else {
                                 67i32
                             } as (usize)).wrapping_mul(
                                n
                            )
                        ),
                        l.wrapping_add(2i32 as (usize)),
                        l,
                        matches
                    );
                } else if is_space != 0 {
                    if *s.offset(0i32 as (isize)) as (i32) == b',' as (i32) {
                        AddMatch(
                            id.wrapping_add((103i32 as (usize)).wrapping_mul(n)),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                        if *s.offset(1i32 as (isize)) as (i32) == b' ' as (i32) {
                            AddMatch(
                                id.wrapping_add((33i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(3i32 as (usize)),
                                l,
                                matches
                            );
                        }
                    } else if *s.offset(0i32 as (isize)) as (i32) == b'.' as (i32) {
                        AddMatch(
                            id.wrapping_add((71i32 as (usize)).wrapping_mul(n)),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                        if *s.offset(1i32 as (isize)) as (i32) == b' ' as (i32) {
                            AddMatch(
                                id.wrapping_add((52i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(3i32 as (usize)),
                                l,
                                matches
                            );
                        }
                    } else if *s.offset(0i32 as (isize)) as (i32) == b'=' as (i32) {
                        if *s.offset(1i32 as (isize)) as (i32) == b'\"' as (i32) {
                            AddMatch(
                                id.wrapping_add((81i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(3i32 as (usize)),
                                l,
                                matches
                            );
                        } else if *s.offset(1i32 as (isize)) as (i32) == b'\'' as (i32) {
                            AddMatch(
                                id.wrapping_add((98i32 as (usize)).wrapping_mul(n)),
                                l.wrapping_add(3i32 as (usize)),
                                l,
                                matches
                            );
                        }
                    }
                }
            } else if is_space != 0 {
                let is_all_caps
                    : i32
                    = if !!(w.transform as (i32) != kUppercaseFirst as (i32)) {
                          1i32
                      } else {
                          0i32
                      };
                let mut s : *const u8;
                if IsMatch(
                       dictionary,
                       w,
                       &*data.offset(1i32 as (isize)) as (*const u8),
                       max_length.wrapping_sub(1i32 as (usize))
                   ) == 0 {
                    if 1337i32 != 0 {
                        continue;
                    }
                }
                AddMatch(
                    id.wrapping_add(
                        (if is_all_caps != 0 {
                             85i32
                         } else {
                             30i32
                         } as (usize)).wrapping_mul(
                            n
                        )
                    ),
                    l.wrapping_add(1i32 as (usize)),
                    l,
                    matches
                );
                has_found_match = 1i32;
                if l.wrapping_add(2i32 as (usize)) >= max_length {
                    if 1337i32 != 0 {
                        continue;
                    }
                }
                s = &*data.offset(
                          l.wrapping_add(1i32 as (usize)) as (isize)
                      ) as (*const u8);
                if *s.offset(0i32 as (isize)) as (i32) == b' ' as (i32) {
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps != 0 {
                                 83i32
                             } else {
                                 15i32
                             } as (usize)).wrapping_mul(
                                n
                            )
                        ),
                        l.wrapping_add(2i32 as (usize)),
                        l,
                        matches
                    );
                } else if *s.offset(0i32 as (isize)) as (i32) == b',' as (i32) {
                    if is_all_caps == 0 {
                        AddMatch(
                            id.wrapping_add((109i32 as (usize)).wrapping_mul(n)),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                    }
                    if *s.offset(1i32 as (isize)) as (i32) == b' ' as (i32) {
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps != 0 {
                                     111i32
                                 } else {
                                     65i32
                                 } as (usize)).wrapping_mul(
                                    n
                                )
                            ),
                            l.wrapping_add(3i32 as (usize)),
                            l,
                            matches
                        );
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b'.' as (i32) {
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps != 0 {
                                 115i32
                             } else {
                                 96i32
                             } as (usize)).wrapping_mul(
                                n
                            )
                        ),
                        l.wrapping_add(2i32 as (usize)),
                        l,
                        matches
                    );
                    if *s.offset(1i32 as (isize)) as (i32) == b' ' as (i32) {
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps != 0 {
                                     117i32
                                 } else {
                                     91i32
                                 } as (usize)).wrapping_mul(
                                    n
                                )
                            ),
                            l.wrapping_add(3i32 as (usize)),
                            l,
                            matches
                        );
                    }
                } else if *s.offset(0i32 as (isize)) as (i32) == b'=' as (i32) {
                    if *s.offset(1i32 as (isize)) as (i32) == b'\"' as (i32) {
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps != 0 {
                                     110i32
                                 } else {
                                     118i32
                                 } as (usize)).wrapping_mul(
                                    n
                                )
                            ),
                            l.wrapping_add(3i32 as (usize)),
                            l,
                            matches
                        );
                    } else if *s.offset(1i32 as (isize)) as (i32) == b'\'' as (i32) {
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps != 0 {
                                     119i32
                                 } else {
                                     120i32
                                 } as (usize)).wrapping_mul(
                                    n
                                )
                            ),
                            l.wrapping_add(3i32 as (usize)),
                            l,
                            matches
                        );
                    }
                }
            }
        }
    }
    if max_length >= 6i32 as (usize) {
        if *data.offset(
                1i32 as (isize)
            ) as (i32) == b' ' as (i32) && (*data.offset(
                                                 0i32 as (isize)
                                             ) as (i32) == b'e' as (i32) || *data.offset(
                                                                                 0i32 as (isize)
                                                                             ) as (i32) == b's' as (i32) || *data.offset(
                                                                                                                 0i32 as (isize)
                                                                                                             ) as (i32) == b',' as (i32)) || *data.offset(
                                                                                                                                                  0i32 as (isize)
                                                                                                                                              ) as (i32) == 0xc2i32 && (*data.offset(
                                                                                                                                                                             1i32 as (isize)
                                                                                                                                                                         ) as (i32) == 0xa0i32) {
            let mut offset
                : usize
                = *kStaticDictionaryBuckets.offset(
                       Hash(&*data.offset(2i32 as (isize)) as (*const u8)) as (isize)
                   ) as (usize);
            let mut end : i32 = (offset == 0) as (i32);
            while end == 0 {
                let mut w
                    : DictWord
                    = *kStaticDictionaryWords.offset(
                           {
                               let _old = offset;
                               offset = offset.wrapping_add(1 as (usize));
                               _old
                           } as (isize)
                       );
                let l : usize = (w.len as (i32) & 0x1fi32) as (usize);
                let n
                    : usize
                    = 1i32 as (usize) << *(*dictionary).size_bits_by_length.offset(
                                              l as (isize)
                                          ) as (i32);
                let id : usize = w.idx as (usize);
                end = !(w.len as (i32) & 0x80i32 == 0) as (i32);
                w.len = l as (u8);
                if w.transform as (i32) == 0i32 && (IsMatch(
                                                        dictionary,
                                                        w,
                                                        &*data.offset(
                                                              2i32 as (isize)
                                                          ) as (*const u8),
                                                        max_length.wrapping_sub(2i32 as (usize))
                                                    ) != 0) {
                    if *data.offset(0i32 as (isize)) as (i32) == 0xc2i32 {
                        AddMatch(
                            id.wrapping_add((102i32 as (usize)).wrapping_mul(n)),
                            l.wrapping_add(2i32 as (usize)),
                            l,
                            matches
                        );
                        has_found_match = 1i32;
                    } else if l.wrapping_add(
                                  2i32 as (usize)
                              ) < max_length && (*data.offset(
                                                      l.wrapping_add(2i32 as (usize)) as (isize)
                                                  ) as (i32) == b' ' as (i32)) {
                        let mut t
                            : usize
                            = (if *data.offset(0i32 as (isize)) as (i32) == b'e' as (i32) {
                                   18i32
                               } else if *data.offset(0i32 as (isize)) as (i32) == b's' as (i32) {
                                   7i32
                               } else {
                                   13i32
                               }) as (usize);
                        AddMatch(
                            id.wrapping_add(t.wrapping_mul(n)),
                            l.wrapping_add(3i32 as (usize)),
                            l,
                            matches
                        );
                        has_found_match = 1i32;
                    }
                }
            }
        }
    }
    if max_length >= 9i32 as (usize) {
        if *data.offset(
                0i32 as (isize)
            ) as (i32) == b' ' as (i32) && (*data.offset(
                                                 1i32 as (isize)
                                             ) as (i32) == b't' as (i32)) && (*data.offset(
                                                                                   2i32 as (isize)
                                                                               ) as (i32) == b'h' as (i32)) && (*data.offset(
                                                                                                                     3i32 as (isize)
                                                                                                                 ) as (i32) == b'e' as (i32)) && (*data.offset(
                                                                                                                                                       4i32 as (isize)
                                                                                                                                                   ) as (i32) == b' ' as (i32)) || *data.offset(
                                                                                                                                                                                        0i32 as (isize)
                                                                                                                                                                                    ) as (i32) == b'.' as (i32) && (*data.offset(
                                                                                                                                                                                                                         1i32 as (isize)
                                                                                                                                                                                                                     ) as (i32) == b'c' as (i32)) && (*data.offset(
                                                                                                                                                                                                                                                           2i32 as (isize)
                                                                                                                                                                                                                                                       ) as (i32) == b'o' as (i32)) && (*data.offset(
                                                                                                                                                                                                                                                                                             3i32 as (isize)
                                                                                                                                                                                                                                                                                         ) as (i32) == b'm' as (i32)) && (*data.offset(
                                                                                                                                                                                                                                                                                                                               4i32 as (isize)
                                                                                                                                                                                                                                                                                                                           ) as (i32) == b'/' as (i32)) {
            let mut offset
                : usize
                = *kStaticDictionaryBuckets.offset(
                       Hash(&*data.offset(5i32 as (isize)) as (*const u8)) as (isize)
                   ) as (usize);
            let mut end : i32 = (offset == 0) as (i32);
            while end == 0 {
                let mut w
                    : DictWord
                    = *kStaticDictionaryWords.offset(
                           {
                               let _old = offset;
                               offset = offset.wrapping_add(1 as (usize));
                               _old
                           } as (isize)
                       );
                let l : usize = (w.len as (i32) & 0x1fi32) as (usize);
                let n
                    : usize
                    = 1i32 as (usize) << *(*dictionary).size_bits_by_length.offset(
                                              l as (isize)
                                          ) as (i32);
                let id : usize = w.idx as (usize);
                end = !(w.len as (i32) & 0x80i32 == 0) as (i32);
                w.len = l as (u8);
                if w.transform as (i32) == 0i32 && (IsMatch(
                                                        dictionary,
                                                        w,
                                                        &*data.offset(
                                                              5i32 as (isize)
                                                          ) as (*const u8),
                                                        max_length.wrapping_sub(5i32 as (usize))
                                                    ) != 0) {
                    AddMatch(
                        id.wrapping_add(
                            (if *data.offset(0i32 as (isize)) as (i32) == b' ' as (i32) {
                                 41i32
                             } else {
                                 72i32
                             } as (usize)).wrapping_mul(
                                n
                            )
                        ),
                        l.wrapping_add(5i32 as (usize)),
                        l,
                        matches
                    );
                    has_found_match = 1i32;
                    if l.wrapping_add(5i32 as (usize)) < max_length {
                        let mut s
                            : *const u8
                            = &*data.offset(
                                    l.wrapping_add(5i32 as (usize)) as (isize)
                                ) as (*const u8);
                        if *data.offset(0i32 as (isize)) as (i32) == b' ' as (i32) {
                            if l.wrapping_add(8i32 as (usize)) < max_length && (*s.offset(
                                                                                     0i32 as (isize)
                                                                                 ) as (i32) == b' ' as (i32)) && (*s.offset(
                                                                                                                       1i32 as (isize)
                                                                                                                   ) as (i32) == b'o' as (i32)) && (*s.offset(
                                                                                                                                                         2i32 as (isize)
                                                                                                                                                     ) as (i32) == b'f' as (i32)) && (*s.offset(
                                                                                                                                                                                           3i32 as (isize)
                                                                                                                                                                                       ) as (i32) == b' ' as (i32)) {
                                AddMatch(
                                    id.wrapping_add((62i32 as (usize)).wrapping_mul(n)),
                                    l.wrapping_add(9i32 as (usize)),
                                    l,
                                    matches
                                );
                                if l.wrapping_add(12i32 as (usize)) < max_length && (*s.offset(
                                                                                          4i32 as (isize)
                                                                                      ) as (i32) == b't' as (i32)) && (*s.offset(
                                                                                                                            5i32 as (isize)
                                                                                                                        ) as (i32) == b'h' as (i32)) && (*s.offset(
                                                                                                                                                              6i32 as (isize)
                                                                                                                                                          ) as (i32) == b'e' as (i32)) && (*s.offset(
                                                                                                                                                                                                7i32 as (isize)
                                                                                                                                                                                            ) as (i32) == b' ' as (i32)) {
                                    AddMatch(
                                        id.wrapping_add((73i32 as (usize)).wrapping_mul(n)),
                                        l.wrapping_add(13i32 as (usize)),
                                        l,
                                        matches
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    has_found_match
}
