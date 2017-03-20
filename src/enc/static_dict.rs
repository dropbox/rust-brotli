use super::static_dict_lut::{kDictHashMul32, kDictNumBits, kInvalidMatch, kStaticDictionaryBuckets,
                             kStaticDictionaryWords, DictWord};

static kUppercaseFirst: u8 = 10i32 as (u8);

static kOmitLastNTransforms: [u8; 10] = [0i32 as (u8),
                                         12i32 as (u8),
                                         27i32 as (u8),
                                         23i32 as (u8),
                                         42i32 as (u8),
                                         63i32 as (u8),
                                         56i32 as (u8),
                                         48i32 as (u8),
                                         59i32 as (u8),
                                         64i32 as (u8)];

pub struct BrotliDictionary {
  pub size_bits_by_length: [u8; 32],
  pub offsets_by_length: [u32; 32],
  pub data: [u8; 122784],
}

fn BROTLI_UNALIGNED_LOAD32(p: &[u8]) -> u32 {
  return (p[0] as u32) | ((p[1] as u32) << 8) | ((p[2] as u32) << 16) | ((p[3] as u32) << 24);
}

fn Hash(mut data: &[u8]) -> u32 {
  let mut h: u32 = BROTLI_UNALIGNED_LOAD32(data).wrapping_mul(kDictHashMul32);
  h >> 32i32 - kDictNumBits
}

fn BROTLI_UNALIGNED_LOAD64(mut p: &[u8]) -> u64 {
  return (p[0] as u64) | ((p[1] as u64) << 8) | ((p[2] as u64) << 16) |
         ((p[3] as u64) << 24) | ((p[4] as u64) << 32) | ((p[5] as u64) << 40) |
         ((p[6] as u64) << 48) | ((p[7] as u64) << 56);
}

fn unopt_ctzll(mut val: u64) -> u8 {
  let mut cnt: u8 = 0i32 as (u8);
  'loop1: loop {
    if val & 1 == 0 {
      val = val >> 1;
      cnt = (cnt as (i32) + 1) as (u8);
      continue 'loop1;
    } else {
      break 'loop1;
    }
  }
  cnt
}

fn FindMatchLengthWithLimit(mut s1: &[u8], mut s2: &[u8], mut limit: usize) -> usize {
  let mut matched: usize = 0i32 as (usize);
  let mut limit2: usize = (limit >> 3i32).wrapping_add(1i32 as (usize));
  'loop1: loop {
    if {
         limit2 = limit2.wrapping_sub(1 as (usize));
         limit2
       } != 0 {
      if BROTLI_UNALIGNED_LOAD64(s2) == BROTLI_UNALIGNED_LOAD64(&s1[matched..]) {
        s2 = &s2[8..];
        matched = matched.wrapping_add(8i32 as (usize));
        continue 'loop1;
      } else {
        break 'loop1;
      }
    } else {
      limit = (limit & 7i32 as (usize)).wrapping_add(1i32 as (usize));
      'loop3: loop {
        if {
             limit = limit.wrapping_sub(1 as (usize));
             limit
           } != 0 {
          if s1[matched as (usize)] as (i32) == s2[0] as (i32) {
            s2 = &s2[1..];
            matched = matched.wrapping_add(1 as (usize));
            continue 'loop3;
          } else {
            break 'loop3;
          }
        } else {
          return matched;
        }
      }
      return matched;
    }
  }
  let mut x: u64 = BROTLI_UNALIGNED_LOAD64(s2) ^ BROTLI_UNALIGNED_LOAD64(&s1[matched as (usize)..]);
  let mut matching_bits: u64 = unopt_ctzll(x) as u64;
  matched = matched.wrapping_add((matching_bits >> 3) as usize);
  matched
}

fn IsMatch(mut dictionary: &BrotliDictionary,
           mut w: DictWord,
           mut data: &[u8],
           mut max_length: usize)
           -> i32 {
  if w.len as (usize) > max_length {
    0i32
  } else {
    let offset: usize = ((*dictionary).offsets_by_length[w.len as (usize)] as (usize))
      .wrapping_add((w.len as (usize)).wrapping_mul(w.idx as (usize)));
    let mut dict: &[u8] = &(dictionary).data[offset..];
    if w.transform as (i32) == 0i32 {
      if !!(FindMatchLengthWithLimit(dict, data, w.len as (usize)) == w.len as (usize)) {
        1i32
      } else {
        0i32
      }
    } else if w.transform as (i32) == 10i32 {
      if !!(dict[0i32 as (usize)] as (i32) >= b'a' as (i32) &&
            (dict[0i32 as (usize)] as (i32) <= b'z' as (i32)) &&
            (dict[0i32 as (usize)] as (i32) ^ 32i32 == data[0i32 as (usize)] as (i32)) &&
            (FindMatchLengthWithLimit(&dict[1i32 as (usize)..],
                                      &data[1i32 as (usize)..],
                                      (w.len as (u32)).wrapping_sub(1u32) as (usize)) ==
             (w.len as (u32)).wrapping_sub(1u32) as (usize))) {
        1i32
      } else {
        0i32
      }
    } else {
      let mut i: usize;
      i = 0i32 as (usize);
      'loop4: loop {
        if i < w.len as (usize) {
          if dict[i as (usize)] as (i32) >= b'a' as (i32) &&
             (dict[i as (usize)] as (i32) <= b'z' as (i32)) {
            if dict[i as (usize)] as (i32) ^ 32i32 != data[i as (usize)] as (i32) {
              break 'loop4;
            }
          } else if dict[i as (usize)] as (i32) != data[i as (usize)] as (i32) {
            return 0i32;
          }
          i = i.wrapping_add(1 as (usize));
          continue 'loop4;
        } else {
          return 1i32;
        }
      }
      0i32
    }
  }
}

fn brotli_min_uint32_t(mut a: u32, mut b: u32) -> u32 {
  if a < b { a } else { b }
}

fn AddMatch(mut distance: usize, mut len: usize, mut len_code: usize, mut matches: &mut [u32]) {
  let mut match_: u32 = (distance << 5i32).wrapping_add(len_code) as (u32);
  matches[len as (usize)] = brotli_min_uint32_t(matches[len as (usize)], match_);
}

fn brotli_min_size_t(mut a: usize, mut b: usize) -> usize {
  if a < b { a } else { b }
}

fn DictMatchLength(mut dictionary: &BrotliDictionary,
                   data: &[u8],
                   mut id: usize,
                   mut len: usize,
                   mut maxlen: usize)
                   -> usize {
  let offset: usize = ((*dictionary).offsets_by_length[len] as (usize))
    .wrapping_add(len.wrapping_mul(id));
  FindMatchLengthWithLimit(&(*dictionary).data[offset..],
                           data,
                           brotli_min_size_t(len, maxlen))
}

fn brotli_max_size_t(mut a: usize, mut b: usize) -> usize {
  if a > b { a } else { b }
}

#[no_mangle]
fn BrotliFindAllStaticDictionaryMatches(mut dictionary: &mut BrotliDictionary,
                                        mut data: &[u8],
                                        mut min_length: usize,
                                        mut max_length: usize,
                                        mut matches: &mut [u32])
                                        -> i32 {
  let mut has_found_match: i32 = 0i32;
  let mut offset: usize = kStaticDictionaryBuckets[Hash(data) as (usize)] as (usize);
  let mut end: i32 = (offset == 0) as (i32);
  'loop1: loop {
    if end == 0 {
      let mut w: DictWord = kStaticDictionaryWords[{
        let _old = offset;
        offset = offset.wrapping_add(1 as (usize));
        _old
      }];
      let l: usize = (w.len as (i32) & 0x1fi32) as (usize);
      let n: usize = 1i32 as (usize) << (*dictionary).size_bits_by_length[l] as (i32);
      let id: usize = w.idx as (usize);
      end = !(w.len as (i32) & 0x80i32 == 0) as (i32);
      w.len = l as (u8);
      if w.transform as (i32) == 0i32 {
        let matchlen: usize = DictMatchLength(dictionary, data, id, l, max_length);
        let mut s: &[u8];
        let mut minlen: usize;
        let mut maxlen: usize;
        let mut len: usize;
        if matchlen == l {
          AddMatch(id, l, l, matches);
          has_found_match = 1i32;
        }
        if matchlen >= l.wrapping_sub(1i32 as (usize)) {
          AddMatch(id.wrapping_add((12i32 as (usize)).wrapping_mul(n)),
                   l.wrapping_sub(1i32 as (usize)),
                   l,
                   matches);
          if l.wrapping_add(2i32 as (usize)) < max_length &&
             (data[l.wrapping_sub(1i32 as (usize)) as (usize)] as (i32) == b'i' as (i32)) &&
             (data[l as (usize)] as (i32) == b'n' as (i32)) &&
             (data[l.wrapping_add(1i32 as (usize)) as (usize)] as (i32) == b'g' as (i32)) &&
             (data[l.wrapping_add(2i32 as (usize)) as (usize)] as (i32) == b' ' as (i32)) {
            AddMatch(id.wrapping_add((49i32 as (usize)).wrapping_mul(n)),
                     l.wrapping_add(3i32 as (usize)),
                     l,
                     matches);
          }
          has_found_match = 1i32;
        }
        minlen = min_length;
        if l > 9i32 as (usize) {
          minlen = brotli_max_size_t(minlen, l.wrapping_sub(9i32 as (usize)));
        }
        maxlen = brotli_min_size_t(matchlen, l.wrapping_sub(2i32 as (usize)));
        len = minlen;
        'loop94: loop {
          if len <= maxlen {
            AddMatch(id.wrapping_add((kOmitLastNTransforms[l.wrapping_sub(len)] as (usize))
                                       .wrapping_mul(n)),
                     len,
                     l,
                     matches);
            has_found_match = 1i32;
            len = len.wrapping_add(1 as (usize));
            continue 'loop94;
          } else {
            break 'loop94;
          }
        }
        if matchlen < l || l.wrapping_add(6i32 as (usize)) >= max_length {
          continue 'loop1;
        } else {
          s = &data[l as (usize)..];
          if s[0] as (i32) == b' ' as (i32) {
            AddMatch(id.wrapping_add(n), l.wrapping_add(1), l, matches);
            if s[1] as (i32) == b'a' as (i32) {
              if s[2] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((28) * (n)), l.wrapping_add(3), l, matches);
                continue 'loop1;
              } else if s[2] as (i32) == b's' as (i32) {
                if s[3] as (i32) == b' ' as (i32) {
                  AddMatch(id.wrapping_add((46) * (n)), l.wrapping_add(4), l, matches);
                  continue 'loop1;
                } else {
                  continue 'loop1;
                }
              } else if s[2] as (i32) == b't' as (i32) {
                if s[3] as (i32) == b' ' as (i32) {
                  AddMatch(id.wrapping_add((60) * (n)), l.wrapping_add(4), l, matches);
                  continue 'loop1;
                } else {
                  continue 'loop1;
                }
              } else if s[2] as (i32) == b'n' as (i32) {
                if s[3] as (i32) == b'd' as (i32) && (s[4] as (i32) == b' ' as (i32)) {
                  AddMatch(id.wrapping_add((10) * (n)), l.wrapping_add(5), l, matches);
                  continue 'loop1;
                } else {
                  continue 'loop1;
                }
              } else {
                continue 'loop1;
              }
            } else if s[1] as (i32) == b'b' as (i32) {
              if s[2] as (i32) == b'y' as (i32) && (s[3] as (i32) == b' ' as (i32)) {
                AddMatch(id.wrapping_add((38) * (n)), l.wrapping_add(4), l, matches);
                continue 'loop1;
              } else {
                continue 'loop1;
              }
            } else if s[1] as (i32) == b'i' as (i32) {
              if s[2] as (i32) == b'n' as (i32) {
                if s[3] as (i32) == b' ' as (i32) {
                  AddMatch(id.wrapping_add((16) * (n)), l.wrapping_add(4), l, matches);
                  continue 'loop1;
                } else {
                  continue 'loop1;
                }
              } else if s[2] as (i32) == b's' as (i32) {
                if s[3] as (i32) == b' ' as (i32) {
                  AddMatch(id.wrapping_add((47) * (n)), l.wrapping_add(4), l, matches);
                  continue 'loop1;
                } else {
                  continue 'loop1;
                }
              } else {
                continue 'loop1;
              }
            } else if s[1] as (i32) == b'f' as (i32) {
              if s[2] as (i32) == b'o' as (i32) {
                if s[3] as (i32) == b'r' as (i32) && (s[4] as (i32) == b' ' as (i32)) {
                  AddMatch(id.wrapping_add((25) * (n)), l.wrapping_add(5), l, matches);
                  continue 'loop1;
                } else {
                  continue 'loop1;
                }
              } else if s[2] as (i32) == b'r' as (i32) {
                if s[3] as (i32) == b'o' as (i32) && (s[4] as (i32) == b'm' as (i32)) &&
                   (s[5] as (i32) == b' ' as (i32)) {
                  AddMatch(id.wrapping_add((37) * (n)), l.wrapping_add(6), l, matches);
                  continue 'loop1;
                } else {
                  continue 'loop1;
                }
              } else {
                continue 'loop1;
              }
            } else if s[1] as (i32) == b'o' as (i32) {
              if s[2] as (i32) == b'f' as (i32) {
                if s[3] as (i32) == b' ' as (i32) {
                  AddMatch(id.wrapping_add((8) * (n)), l.wrapping_add(4), l, matches);
                  continue 'loop1;
                } else {
                  continue 'loop1;
                }
              } else if s[2] as (i32) == b'n' as (i32) {
                if s[3] as (i32) == b' ' as (i32) {
                  AddMatch(id.wrapping_add((45) * (n)), l.wrapping_add(4), l, matches);
                  continue 'loop1;
                } else {
                  continue 'loop1;
                }
              } else {
                continue 'loop1;
              }
            } else if s[1] as (i32) == b'n' as (i32) {
              if s[2] as (i32) == b'o' as (i32) && (s[3] as (i32) == b't' as (i32)) &&
                 (s[4] as (i32) == b' ' as (i32)) {
                AddMatch(id.wrapping_add((80) * (n)), l.wrapping_add(5), l, matches);
                continue 'loop1;
              } else {
                continue 'loop1;
              }
            } else if s[1] as (i32) == b't' as (i32) {
              if s[2] as (i32) == b'h' as (i32) {
                if s[3] as (i32) == b'e' as (i32) {
                  if s[4] as (i32) == b' ' as (i32) {
                    AddMatch(id.wrapping_add((5) * (n)), l.wrapping_add(5), l, matches);
                    continue 'loop1;
                  } else {
                    continue 'loop1;
                  }
                } else if s[3] as (i32) == b'a' as (i32) {
                  if s[4] as (i32) == b't' as (i32) && (s[5] as (i32) == b' ' as (i32)) {
                    AddMatch(id.wrapping_add((29) * (n)), l.wrapping_add(6), l, matches);
                    continue 'loop1;
                  } else {
                    continue 'loop1;
                  }
                } else {
                  continue 'loop1;
                }
              } else if s[2] as (i32) == b'o' as (i32) {
                if s[3] as (i32) == b' ' as (i32) {
                  AddMatch(id.wrapping_add((17) * (n)), l.wrapping_add(4), l, matches);
                  continue 'loop1;
                } else {
                  continue 'loop1;
                }
              } else {
                continue 'loop1;
              }
            } else if s[1] as (i32) == b'w' as (i32) {
              if s[2] as (i32) == b'i' as (i32) && (s[3] as (i32) == b't' as (i32)) &&
                 (s[4] as (i32) == b'h' as (i32)) &&
                 (s[5] as (i32) == b' ' as (i32)) {
                AddMatch(id.wrapping_add((35) * (n)), l.wrapping_add(6), l, matches);
                continue 'loop1;
              } else {
                continue 'loop1;
              }
            } else {
              continue 'loop1;
            }
          } else if s[0] as (i32) == b'\"' as (i32) {
            AddMatch(id.wrapping_add((19) * (n)), l.wrapping_add(1), l, matches);
            if s[1] as (i32) == b'>' as (i32) {
              AddMatch(id.wrapping_add((21) * (n)), l.wrapping_add(2), l, matches);
              continue 'loop1;
            } else {
              continue 'loop1;
            }
          } else if s[0] as (i32) == b'.' as (i32) {
            AddMatch(id.wrapping_add((20) * (n)), l.wrapping_add(1), l, matches);
            if s[1] as (i32) == b' ' as (i32) {
              AddMatch(id.wrapping_add((31) * (n)), l.wrapping_add(2), l, matches);
              if s[2] as (i32) == b'T' as (i32) && (s[3] as (i32) == b'h' as (i32)) {
                if s[4] as (i32) == b'e' as (i32) {
                  if s[5] as (i32) == b' ' as (i32) {
                    AddMatch(id.wrapping_add((43) * (n)), l.wrapping_add(6), l, matches);
                    continue 'loop1;
                  } else {
                    continue 'loop1;
                  }
                } else if s[4] as (i32) == b'i' as (i32) {
                  if s[5] as (i32) == b's' as (i32) && (s[6] as (i32) == b' ' as (i32)) {
                    AddMatch(id.wrapping_add((75) * (n)), l.wrapping_add(7), l, matches);
                    continue 'loop1;
                  } else {
                    continue 'loop1;
                  }
                } else {
                  continue 'loop1;
                }
              } else {
                continue 'loop1;
              }
            } else {
              continue 'loop1;
            }
          } else if s[0] as (i32) == b',' as (i32) {
            AddMatch(id.wrapping_add((76) * (n)), l.wrapping_add(1), l, matches);
            if s[1] as (i32) == b' ' as (i32) {
              AddMatch(id.wrapping_add((14) * (n)), l.wrapping_add(2), l, matches);
              continue 'loop1;
            } else {
              continue 'loop1;
            }
          } else if s[0] as (i32) == b'\n' as (i32) {
            AddMatch(id.wrapping_add((22) * (n)), l.wrapping_add(1), l, matches);
            if s[1] as (i32) == b'\t' as (i32) {
              AddMatch(id.wrapping_add((50) * (n)), l.wrapping_add(2), l, matches);
              continue 'loop1;
            } else {
              continue 'loop1;
            }
          } else if s[0] as (i32) == b']' as (i32) {
            AddMatch(id.wrapping_add((24) * (n)), l.wrapping_add(1), l, matches);
            continue 'loop1;
          } else if s[0] as (i32) == b'\'' as (i32) {
            AddMatch(id.wrapping_add((36) * (n)), l.wrapping_add(1), l, matches);
            continue 'loop1;
          } else if s[0] as (i32) == b':' as (i32) {
            AddMatch(id.wrapping_add((51) * (n)), l.wrapping_add(1), l, matches);
            continue 'loop1;
          } else if s[0] as (i32) == b'(' as (i32) {
            AddMatch(id.wrapping_add((57) * (n)), l.wrapping_add(1), l, matches);
            continue 'loop1;
          } else if s[0] as (i32) == b'=' as (i32) {
            if s[1] as (i32) == b'\"' as (i32) {
              AddMatch(id.wrapping_add((70) * (n)), l.wrapping_add(2), l, matches);
              continue 'loop1;
            } else if s[1] as (i32) == b'\'' as (i32) {
              AddMatch(id.wrapping_add((86) * (n)), l.wrapping_add(2), l, matches);
              continue 'loop1;
            } else {
              continue 'loop1;
            }
          } else if s[0] as (i32) == b'a' as (i32) {
            if s[1] as (i32) == b'l' as (i32) && (s[2] as (i32) == b' ' as (i32)) {
              AddMatch(id.wrapping_add((84) * (n)), l.wrapping_add(3), l, matches);
              continue 'loop1;
            } else {
              continue 'loop1;
            }
          } else if s[0] as (i32) == b'e' as (i32) {
            if s[1] as (i32) == b'd' as (i32) {
              if s[2] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((53) * (n)), l.wrapping_add(3), l, matches);
                continue 'loop1;
              } else {
                continue 'loop1;
              }
            } else if s[1] as (i32) == b'r' as (i32) {
              if s[2] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((82) * (n)), l.wrapping_add(3), l, matches);
                continue 'loop1;
              } else {
                continue 'loop1;
              }
            } else if s[1] as (i32) == b's' as (i32) {
              if s[2] as (i32) == b't' as (i32) && (s[3] as (i32) == b' ' as (i32)) {
                AddMatch(id.wrapping_add((95) * (n)), l.wrapping_add(4), l, matches);
                continue 'loop1;
              } else {
                continue 'loop1;
              }
            } else {
              continue 'loop1;
            }
          } else if s[0] as (i32) == b'f' as (i32) {
            if s[1] as (i32) == b'u' as (i32) && (s[2] as (i32) == b'l' as (i32)) &&
               (s[3] as (i32) == b' ' as (i32)) {
              AddMatch(id.wrapping_add((90) * (n)), l.wrapping_add(4), l, matches);
              continue 'loop1;
            } else {
              continue 'loop1;
            }
          } else if s[0] as (i32) == b'i' as (i32) {
            if s[1] as (i32) == b'v' as (i32) {
              if s[2] as (i32) == b'e' as (i32) && (s[3] as (i32) == b' ' as (i32)) {
                AddMatch(id.wrapping_add((92) * (n)), l.wrapping_add(4), l, matches);
                continue 'loop1;
              } else {
                continue 'loop1;
              }
            } else if s[1] as (i32) == b'z' as (i32) {
              if s[2] as (i32) == b'e' as (i32) && (s[3] as (i32) == b' ' as (i32)) {
                AddMatch(id.wrapping_add((100) * (n)), l.wrapping_add(4), l, matches);
                continue 'loop1;
              } else {
                continue 'loop1;
              }
            } else {
              continue 'loop1;
            }
          } else if s[0] as (i32) == b'l' as (i32) {
            if s[1] as (i32) == b'e' as (i32) {
              if s[2] as (i32) == b's' as (i32) && (s[3] as (i32) == b's' as (i32)) &&
                 (s[4] as (i32) == b' ' as (i32)) {
                AddMatch(id.wrapping_add((93) * (n)), l.wrapping_add(5), l, matches);
                continue 'loop1;
              } else {
                continue 'loop1;
              }
            } else if s[1] as (i32) == b'y' as (i32) {
              if s[2] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((61) * (n)), l.wrapping_add(3), l, matches);
                continue 'loop1;
              } else {
                continue 'loop1;
              }
            } else {
              continue 'loop1;
            }
          } else if s[0] as (i32) == b'o' as (i32) {
            if s[1] as (i32) == b'u' as (i32) && (s[2] as (i32) == b's' as (i32)) &&
               (s[3] as (i32) == b' ' as (i32)) {
              AddMatch(id.wrapping_add((106) * (n)), l.wrapping_add(4), l, matches);
              continue 'loop1;
            } else {
              continue 'loop1;
            }
          } else {
            continue 'loop1;
          }
        }
      } else {
        let is_all_caps: i32 = if !!(w.transform as (i32) != kUppercaseFirst as (i32)) {
          1i32
        } else {
          0i32
        };
        let mut s: &[u8];
        if IsMatch(dictionary, w, data, max_length) == 0 {
          continue 'loop1;
        } else {
          AddMatch(id.wrapping_add((if is_all_caps != 0 { 44i32 } else { 9i32 } as (usize)) * (n)),
                   l,
                   l,
                   matches);
          has_found_match = 1i32;
          if l.wrapping_add(1) >= max_length {
            continue 'loop1;
          } else {
            s = &data[l as (usize)..];
            if s[0] as (i32) == b' ' as (i32) {
              AddMatch(id.wrapping_add((if is_all_caps != 0 { 68i32 } else { 4i32 } as
                                        (usize)) * (n)),
                       l.wrapping_add(1),
                       l,
                       matches);
              continue 'loop1;
            } else if s[0] as (i32) == b'\"' as (i32) {
              AddMatch(id.wrapping_add((if is_all_caps != 0 { 87i32 } else { 66i32 } as
                                        (usize)) * (n)),
                       l.wrapping_add(1),
                       l,
                       matches);
              if s[1] as (i32) == b'>' as (i32) {
                AddMatch(id.wrapping_add((if is_all_caps != 0 { 97i32 } else { 69i32 } as
                                          (usize)) * (n)),
                         l.wrapping_add(2),
                         l,
                         matches);
                continue 'loop1;
              } else {
                continue 'loop1;
              }
            } else if s[0] as (i32) == b'.' as (i32) {
              AddMatch(id.wrapping_add((if is_all_caps != 0 { 101i32 } else { 79i32 } as
                                        (usize)) * (n)),
                       l.wrapping_add(1),
                       l,
                       matches);
              if s[1] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((if is_all_caps != 0 { 114i32 } else { 88i32 } as
                                          (usize)) * (n)),
                         l.wrapping_add(2),
                         l,
                         matches);
                continue 'loop1;
              } else {
                continue 'loop1;
              }
            } else if s[0] as (i32) == b',' as (i32) {
              AddMatch(id.wrapping_add((if is_all_caps != 0 { 112i32 } else { 99i32 } as
                                        (usize)) * (n)),
                       l.wrapping_add(1),
                       l,
                       matches);
              if s[1] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((if is_all_caps != 0 { 107i32 } else { 58i32 } as
                                          (usize)) * (n)),
                         l.wrapping_add(2),
                         l,
                         matches);
                continue 'loop1;
              } else {
                continue 'loop1;
              }
            } else if s[0] as (i32) == b'\'' as (i32) {
              AddMatch(id.wrapping_add((if is_all_caps != 0 { 94i32 } else { 74i32 } as
                                        (usize)) * (n)),
                       l.wrapping_add(1),
                       l,
                       matches);
              continue 'loop1;
            } else if s[0] as (i32) == b'(' as (i32) {
              AddMatch(id.wrapping_add((if is_all_caps != 0 { 113i32 } else { 78i32 } as
                                        (usize)) * (n)),
                       l.wrapping_add(1),
                       l,
                       matches);
              continue 'loop1;
            } else if s[0] as (i32) == b'=' as (i32) {
              if s[1] as (i32) == b'\"' as (i32) {
                AddMatch(id.wrapping_add((if is_all_caps != 0 { 105i32 } else { 104i32 } as
                                          (usize)) * (n)),
                         l.wrapping_add(2),
                         l,
                         matches);
                continue 'loop1;
              } else if s[1] as (i32) == b'\'' as (i32) {
                AddMatch(id.wrapping_add((if is_all_caps != 0 { 116i32 } else { 108i32 } as
                                          (usize)) * (n)),
                         l.wrapping_add(2),
                         l,
                         matches);
                continue 'loop1;
              } else {
                continue 'loop1;
              }
            } else {
              continue 'loop1;
            }
          }
        }
      }
    } else {
      break 'loop1;
    }
  }
  if max_length >= 5 && (data[0] as (i32) == b' ' as (i32) || data[0] as (i32) == b'.' as (i32)) {
    let mut is_space: i32 = if !!(data[0] as (i32) == b' ' as (i32)) {
      1i32
    } else {
      0i32
    };
    let mut offset: usize = kStaticDictionaryBuckets[Hash(&data[1..]) as (usize)] as (usize);
    let mut end: i32 = (offset == 0) as (i32);
    'loop4: loop {
      if end == 0 {
        let mut w: DictWord = kStaticDictionaryWords[{
          let _old = offset;
          offset = offset.wrapping_add(1 as (usize));
          _old
        }];
        let l: usize = (w.len as (i32) & 0x1fi32) as (usize);
        let n: usize = 1 << (*dictionary).size_bits_by_length[l] as (i32);
        let id: usize = w.idx as (usize);
        end = !(w.len as (i32) & 0x80i32 == 0) as (i32);
        w.len = l as (u8);
        if w.transform as (i32) == 0i32 {
          let mut s: &[u8];
          if IsMatch(dictionary, w, &data[1..], max_length.wrapping_sub(1)) == 0 {
            continue 'loop4;
          } else {
            AddMatch(id.wrapping_add((if is_space != 0 { 6i32 } else { 32i32 } as (usize)) * (n)),
                     l.wrapping_add(1),
                     l,
                     matches);
            has_found_match = 1i32;
            if l.wrapping_add(2) >= max_length {
              continue 'loop4;
            } else {
              s = &data[l.wrapping_add(1) as (usize)..];
              if s[0] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((if is_space != 0 { 2i32 } else { 77i32 } as (usize)) *
                                         (n)),
                         l.wrapping_add(2),
                         l,
                         matches);
                continue 'loop4;
              } else if s[0] as (i32) == b'(' as (i32) {
                AddMatch(id.wrapping_add((if is_space != 0 { 89i32 } else { 67i32 } as
                                          (usize)) * (n)),
                         l.wrapping_add(2),
                         l,
                         matches);
                continue 'loop4;
              } else if is_space != 0 {
                if s[0] as (i32) == b',' as (i32) {
                  AddMatch(id.wrapping_add((103) * (n)), l.wrapping_add(2), l, matches);
                  if s[1] as (i32) == b' ' as (i32) {
                    AddMatch(id.wrapping_add((33) * (n)), l.wrapping_add(3), l, matches);
                    continue 'loop4;
                  } else {
                    continue 'loop4;
                  }
                } else if s[0] as (i32) == b'.' as (i32) {
                  AddMatch(id.wrapping_add((71) * (n)), l.wrapping_add(2), l, matches);
                  if s[1] as (i32) == b' ' as (i32) {
                    AddMatch(id.wrapping_add((52) * (n)), l.wrapping_add(3), l, matches);
                    continue 'loop4;
                  } else {
                    continue 'loop4;
                  }
                } else if s[0] as (i32) == b'=' as (i32) {
                  if s[1] as (i32) == b'\"' as (i32) {
                    AddMatch(id.wrapping_add((81) * (n)), l.wrapping_add(3), l, matches);
                    continue 'loop4;
                  } else if s[1] as (i32) == b'\'' as (i32) {
                    AddMatch(id.wrapping_add((98) * (n)), l.wrapping_add(3), l, matches);
                    continue 'loop4;
                  } else {
                    continue 'loop4;
                  }
                } else {
                  continue 'loop4;
                }
              } else {
                continue 'loop4;
              }
            }
          }
        } else if is_space != 0 {
          let is_all_caps: i32 = if !!(w.transform as (i32) != kUppercaseFirst as (i32)) {
            1i32
          } else {
            0i32
          };
          let mut s: &[u8];
          if IsMatch(dictionary, w, &data[1..], max_length.wrapping_sub(1)) == 0 {
            continue 'loop4;
          } else {
            AddMatch(id.wrapping_add((if is_all_caps != 0 { 85i32 } else { 30i32 } as (usize)) *
                                     (n)),
                     l.wrapping_add(1),
                     l,
                     matches);
            has_found_match = 1i32;
            if l.wrapping_add(2) >= max_length {
              continue 'loop4;
            } else {
              s = &data[l.wrapping_add(1) as (usize)..];
              if s[0] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((if is_all_caps != 0 { 83i32 } else { 15i32 } as
                                          (usize)) * (n)),
                         l.wrapping_add(2),
                         l,
                         matches);
                continue 'loop4;
              } else if s[0] as (i32) == b',' as (i32) {
                if is_all_caps == 0 {
                  AddMatch(id.wrapping_add((109) * (n)), l.wrapping_add(2), l, matches);
                }
                if s[1] as (i32) == b' ' as (i32) {
                  AddMatch(id.wrapping_add((if is_all_caps != 0 { 111i32 } else { 65i32 } as
                                            (usize)) *
                                           (n)),
                           l.wrapping_add(3),
                           l,
                           matches);
                  continue 'loop4;
                } else {
                  continue 'loop4;
                }
              } else if s[0] as (i32) == b'.' as (i32) {
                AddMatch(id.wrapping_add((if is_all_caps != 0 { 115i32 } else { 96i32 } as
                                          (usize)) * (n)),
                         l.wrapping_add(2),
                         l,
                         matches);
                if s[1] as (i32) == b' ' as (i32) {
                  AddMatch(id.wrapping_add((if is_all_caps != 0 { 117i32 } else { 91i32 } as
                                            (usize)) *
                                           (n)),
                           l.wrapping_add(3),
                           l,
                           matches);
                  continue 'loop4;
                } else {
                  continue 'loop4;
                }
              } else if s[0] as (i32) == b'=' as (i32) {
                if s[1] as (i32) == b'\"' as (i32) {
                  AddMatch(id.wrapping_add((if is_all_caps != 0 { 110i32 } else { 118i32 } as
                                            (usize)) *
                                           (n)),
                           l.wrapping_add(3),
                           l,
                           matches);
                  continue 'loop4;
                } else if s[1] as (i32) == b'\'' as (i32) {
                  AddMatch(id.wrapping_add((if is_all_caps != 0 { 119i32 } else { 120i32 } as
                                            (usize)) *
                                           (n)),
                           l.wrapping_add(3),
                           l,
                           matches);
                  continue 'loop4;
                } else {
                  continue 'loop4;
                }
              } else {
                continue 'loop4;
              }
            }
          }
        } else {
          continue 'loop4;
        }
      } else {
        break 'loop4;
      }
    }
  }
  if max_length >= 6 {
    if data[1] as (i32) == b' ' as (i32) &&
       (data[0] as (i32) == b'e' as (i32) || data[0] as (i32) == b's' as (i32) ||
        data[0] as (i32) == b',' as (i32)) ||
       data[0] as (i32) == 0xc2i32 && (data[1] as (i32) == 0xa0i32) {
      let mut offset: usize = kStaticDictionaryBuckets[Hash(&data[2..]) as (usize)] as (usize);
      let mut end: i32 = (offset == 0) as (i32);
      'loop8: loop {
        if end == 0 {
          let mut w: DictWord = kStaticDictionaryWords[{
            let _old = offset;
            offset = offset.wrapping_add(1 as (usize));
            _old
          }];
          let l: usize = (w.len as (i32) & 0x1fi32) as (usize);
          let n: usize = 1 << (*dictionary).size_bits_by_length[l] as (i32);
          let id: usize = w.idx as (usize);
          end = !(w.len as (i32) & 0x80i32 == 0) as (i32);
          w.len = l as (u8);
          if w.transform as (i32) == 0i32 &&
             (IsMatch(dictionary, w, &data[2..], max_length.wrapping_sub(2)) != 0) {
            if data[0] as (i32) == 0xc2i32 {
              AddMatch(id.wrapping_add((102) * (n)), l.wrapping_add(2), l, matches);
              has_found_match = 1i32;
              continue 'loop8;
            } else if l.wrapping_add(2) < max_length &&
                      (data[l.wrapping_add(2) as (usize)] as (i32) == b' ' as (i32)) {
              let mut t: usize = (if data[0] as (i32) == b'e' as (i32) {
                                    18i32
                                  } else if data[0] as (i32) == b's' as (i32) {
                7i32
              } else {
                13i32
              }) as (usize);
              AddMatch(id.wrapping_add(t * (n)), l.wrapping_add(3), l, matches);
              has_found_match = 1i32;
              continue 'loop8;
            } else {
              continue 'loop8;
            }
          } else {
            continue 'loop8;
          }
        } else {
          break 'loop8;
        }
      }
    }
  }
  if max_length >= 9 {
    if data[0] as (i32) == b' ' as (i32) && (data[1] as (i32) == b't' as (i32)) &&
       (data[2] as (i32) == b'h' as (i32)) && (data[3] as (i32) == b'e' as (i32)) &&
       (data[4] as (i32) == b' ' as (i32)) ||
       data[0] as (i32) == b'.' as (i32) && (data[1] as (i32) == b'c' as (i32)) &&
       (data[2] as (i32) == b'o' as (i32)) && (data[3] as (i32) == b'm' as (i32)) &&
       (data[4] as (i32) == b'/' as (i32)) {
      let mut offset: usize = kStaticDictionaryBuckets[Hash(&data[5..]) as (usize)] as (usize);
      let mut end: i32 = (offset == 0) as (i32);
      'loop12: loop {
        if end == 0 {
          let mut w: DictWord = kStaticDictionaryWords[{
            let _old = offset;
            offset = offset.wrapping_add(1 as (usize));
            _old
          }];
          let l: usize = (w.len as (i32) & 0x1fi32) as (usize);
          let n: usize = 1 << (*dictionary).size_bits_by_length[l] as (i32);
          let id: usize = w.idx as (usize);
          end = !(w.len as (i32) & 0x80i32 == 0) as (i32);
          w.len = l as (u8);
          if w.transform as (i32) == 0i32 &&
             (IsMatch(dictionary, w, &data[5..], max_length.wrapping_sub(5)) != 0) {
            AddMatch(id.wrapping_add((if data[0] as (i32) == b' ' as (i32) {
                                        41i32
                                      } else {
                                        72i32
                                      } as (usize)) * (n)),
                     l.wrapping_add(5),
                     l,
                     matches);
            has_found_match = 1i32;
            if l.wrapping_add(5) < max_length {
              let mut s: &[u8] = &data[l.wrapping_add(5) as (usize)..];
              if data[0] as (i32) == b' ' as (i32) {
                if l.wrapping_add(8) < max_length && (s[0] as (i32) == b' ' as (i32)) &&
                   (s[1] as (i32) == b'o' as (i32)) &&
                   (s[2] as (i32) == b'f' as (i32)) &&
                   (s[3] as (i32) == b' ' as (i32)) {
                  AddMatch(id.wrapping_add((62) * (n)), l.wrapping_add(9), l, matches);
                  if l.wrapping_add(12) < max_length && (s[4] as (i32) == b't' as (i32)) &&
                     (s[5] as (i32) == b'h' as (i32)) &&
                     (s[6] as (i32) == b'e' as (i32)) &&
                     (s[7] as (i32) == b' ' as (i32)) {
                    AddMatch(id.wrapping_add((73) * (n)), l.wrapping_add(13), l, matches);
                    continue 'loop12;
                  } else {
                    continue 'loop12;
                  }
                } else {
                  continue 'loop12;
                }
              } else {
                continue 'loop12;
              }
            } else {
              continue 'loop12;
            }
          } else {
            continue 'loop12;
          }
        } else {
          break 'loop12;
        }
      }
    }
  }
  has_found_match
}
