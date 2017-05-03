#![allow(dead_code)]
use super::static_dict_lut::{kDictHashMul32, kDictNumBits, kStaticDictionaryBuckets,
                             kStaticDictionaryWords, DictWord};
use super::super::dictionary::{kBrotliDictionary, kBrotliDictionarySizeBitsByLength,
                               kBrotliDictionaryOffsetsByLength};
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
  pub size_bits_by_length: &'static [u8; 25],
  pub offsets_by_length: &'static [u32; 25],
  pub data: &'static [u8; 122784],
}

pub static kBrotliEncDictionary: BrotliDictionary = BrotliDictionary {
  size_bits_by_length: &kBrotliDictionarySizeBitsByLength,
  offsets_by_length: &kBrotliDictionaryOffsetsByLength,
  data: &kBrotliDictionary,
};

pub fn BrotliGetDictionary() -> &'static BrotliDictionary {
  return &kBrotliEncDictionary;
}
pub fn BROTLI_UNALIGNED_LOAD32(sl: &[u8]) -> u32 {
  let mut p = [0u8;4];
  p[..].clone_from_slice(&sl[..4]);
  return (p[0] as u32) | ((p[1] as u32) << 8) | ((p[2] as u32) << 16) | ((p[3] as u32) << 24);
}

pub fn Hash(data: &[u8]) -> u32 {
  let h: u32 = BROTLI_UNALIGNED_LOAD32(data).wrapping_mul(kDictHashMul32);
  h >> 32i32 - kDictNumBits
}

pub fn BROTLI_UNALIGNED_LOAD64(sl: &[u8]) -> u64 {
  let mut p = [0u8;8];
  p[..].clone_from_slice(&sl[..8]);
  return (p[0] as u64) | ((p[1] as u64) << 8) | ((p[2] as u64) << 16) |
         ((p[3] as u64) << 24) | ((p[4] as u64) << 32) | ((p[5] as u64) << 40) |
         ((p[6] as u64) << 48) | ((p[7] as u64) << 56);
}
pub fn BROTLI_UNALIGNED_STORE64(outp: &mut [u8], v: u64) {
  let p = [(v & 0xff) as u8,
       ((v >> 8) & 0xff) as u8,
       ((v >> 16) & 0xff) as u8,
       ((v >> 24) & 0xff) as u8,
       ((v >> 32) & 0xff) as u8,
       ((v >> 40) & 0xff) as u8,
       ((v >> 48) & 0xff) as u8,
       ((v >> 56) & 0xff) as u8];
  outp[..8].clone_from_slice(&p[..]);
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

pub fn FindMatchLengthWithLimit(s1: &[u8], mut s2: &[u8], mut limit: usize) -> usize {
  let mut matched: usize = 0usize;
  let mut limit2: usize = (limit >> 3i32).wrapping_add(1usize);
  while {
          limit2 = limit2.wrapping_sub(1 as (usize));
          limit2
        } != 0 {
    if BROTLI_UNALIGNED_LOAD64(s2) == BROTLI_UNALIGNED_LOAD64(&s1[(matched as u32 as (usize))..(matched as u32 + 8)as usize]) {
      s2 = &s2[(8usize)..];
      matched = matched.wrapping_add(8usize) as u32 as usize;
    } else {
      let x: u64 = BROTLI_UNALIGNED_LOAD64(s2) ^
                   BROTLI_UNALIGNED_LOAD64(&s1[(matched as u32 as (usize))..(matched as u32 + 8) as usize]);
      let matching_bits: usize = unopt_ctzll(x) as (usize);
      matched = matched.wrapping_add(matching_bits >> 3i32) as u32 as usize;
      return matched;
    }
  }
  limit = (limit & 7usize).wrapping_add(1usize);
  while {
          limit = limit.wrapping_sub(1 as (usize));
          limit
        } != 0 {
    let (s2_0, s2_rest) = s2.split_at(1);
    if s1[(matched as (usize))] as (i32) == s2_0[0] as (i32) {
      s2 = s2_rest;
      matched = matched.wrapping_add(1 as (usize)) as u32 as usize;
    } else {
      return matched;
    }
  }
  matched
}

pub fn IsMatch(dictionary: &BrotliDictionary, w: DictWord, data: &[u8], max_length: usize) -> i32 {
  if w.len as (usize) > max_length {
    0i32
  } else {
    let offset: usize = ((*dictionary).offsets_by_length[w.len as (usize)] as (usize))
      .wrapping_add((w.len as (usize)).wrapping_mul(w.idx as (usize)));
    let dict = &(*dictionary).data[offset..];
    if w.transform as (i32) == 0i32 {
      if !!(FindMatchLengthWithLimit(dict, data, w.len as (usize)) == w.len as (usize)) {
        1i32
      } else {
        0i32
      }
    } else if w.transform as (i32) == 10i32 {
      if !!(dict[(0usize)] as (i32) >= b'a' as (i32) &&
            (dict[(0usize)] as (i32) <= b'z' as (i32)) &&
            (dict[(0usize)] as (i32) ^ 32i32 == data[(0usize)] as (i32)) &&
            (FindMatchLengthWithLimit(&dict[(1usize)..],
                                      &data[(1usize)..],
                                      (w.len as (u32)).wrapping_sub(1u32) as (usize)) ==
             (w.len as (u32)).wrapping_sub(1u32) as (usize))) {
        1i32
      } else {
        0i32
      }
    } else {
      let mut i: usize;
      i = 0usize;
      while i < w.len as (usize) {
        {
          if dict[(i as (usize))] as (i32) >= b'a' as (i32) &&
             (dict[(i as (usize))] as (i32) <= b'z' as (i32)) {
            if dict[(i as (usize))] as (i32) ^ 32i32 != data[(i as (usize))] as (i32) {
              return 0i32;
            }
          } else if dict[(i as (usize))] as (i32) != data[(i as (usize))] as (i32) {
            return 0i32;
          }
        }
        i = i.wrapping_add(1 as (usize));
      }
      1i32
    }
  }
}

fn brotli_min_uint32_t(a: u32, b: u32) -> u32 {
  if a < b { a } else { b }
}

fn AddMatch(distance: usize, len: usize, len_code: usize, mut matches: &mut [u32]) {
  let match_: u32 = (distance << 5i32).wrapping_add(len_code) as (u32);
  matches[len as (usize)] = brotli_min_uint32_t(matches[len as (usize)], match_);
}

fn brotli_min_size_t(a: usize, b: usize) -> usize {
  if a < b { a } else { b }
}

fn DictMatchLength(dictionary: &BrotliDictionary,
                   data: &[u8],
                   id: usize,
                   len: usize,
                   maxlen: usize)
                   -> usize {
  let offset: usize = ((*dictionary).offsets_by_length[len] as (usize))
    .wrapping_add(len.wrapping_mul(id));
  FindMatchLengthWithLimit(&(*dictionary).data[offset..],
                           data,
                           brotli_min_size_t(len, maxlen))
}

fn brotli_max_size_t(a: usize, b: usize) -> usize {
  if a > b { a } else { b }
}

fn BrotliFindAllStaticDictionaryMatches(dictionary: &BrotliDictionary,
                                        data: &[u8],
                                        min_length: usize,
                                        max_length: usize,
                                        mut matches: &mut [u32])
                                        -> i32 {
  let mut has_found_match: i32 = 0i32;
  {
    let mut offset: usize = kStaticDictionaryBuckets[Hash(data) as (usize)] as (usize);
    let mut end: i32 = (offset == 0) as (i32);
    while end == 0 {
      let mut w: DictWord = kStaticDictionaryWords[{
        let _old = offset;
        offset = offset.wrapping_add(1 as (usize));
        _old
      }];
      let l: usize = (w.len as (i32) & 0x1fi32) as (usize);
      let n: usize = 1usize << (*dictionary).size_bits_by_length[l] as (i32);
      let id: usize = w.idx as (usize);
      end = !(w.len as (i32) & 0x80i32 == 0) as (i32);
      w.len = l as (u8);
      if w.transform as (i32) == 0i32 {
        let matchlen: usize = DictMatchLength(dictionary, data, id, l, max_length);
        let s: &[u8];
        let mut minlen: usize;
        let maxlen: usize;
        let mut len: usize;
        if matchlen == l {
          AddMatch(id, l, l, matches);
          has_found_match = 1i32;
        }
        if matchlen >= l.wrapping_sub(1usize) {
          AddMatch(id.wrapping_add((12usize).wrapping_mul(n)),
                   l.wrapping_sub(1usize),
                   l,
                   matches);
          if l.wrapping_add(2usize) < max_length &&
             (data[(l.wrapping_sub(1usize) as (usize))] as (i32) == b'i' as (i32)) &&
             (data[(l as (usize))] as (i32) == b'n' as (i32)) &&
             (data[(l.wrapping_add(1usize) as (usize))] as (i32) == b'g' as (i32)) &&
             (data[(l.wrapping_add(2usize) as (usize))] as (i32) == b' ' as (i32)) {
            AddMatch(id.wrapping_add((49usize).wrapping_mul(n)),
                     l.wrapping_add(3usize),
                     l,
                     matches);
          }
          has_found_match = 1i32;
        }
        minlen = min_length;
        if l > 9usize {
          minlen = brotli_max_size_t(minlen, l.wrapping_sub(9usize));
        }
        maxlen = brotli_min_size_t(matchlen, l.wrapping_sub(2usize));
        len = minlen;
        while len <= maxlen {
          {
            AddMatch(id.wrapping_add((kOmitLastNTransforms[l.wrapping_sub(len)] as (usize))
                                       .wrapping_mul(n)),
                     len,
                     l,
                     matches);
            has_found_match = 1i32;
          }
          len = len.wrapping_add(1 as (usize));
        }
        if matchlen < l || l.wrapping_add(6usize) >= max_length {
          {
            continue;
          }
        }
        s = &data[(l as (usize))..];
        if s[(0usize)] as (i32) == b' ' as (i32) {
          AddMatch(id.wrapping_add(n), l.wrapping_add(1usize), l, matches);
          if s[(1usize)] as (i32) == b'a' as (i32) {
            if s[(2usize)] as (i32) == b' ' as (i32) {
              AddMatch(id.wrapping_add((28usize).wrapping_mul(n)),
                       l.wrapping_add(3usize),
                       l,
                       matches);
            } else if s[(2usize)] as (i32) == b's' as (i32) {
              if s[(3usize)] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((46usize).wrapping_mul(n)),
                         l.wrapping_add(4usize),
                         l,
                         matches);
              }
            } else if s[(2usize)] as (i32) == b't' as (i32) {
              if s[(3usize)] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((60usize).wrapping_mul(n)),
                         l.wrapping_add(4usize),
                         l,
                         matches);
              }
            } else if s[(2usize)] as (i32) == b'n' as (i32) {
              if s[(3usize)] as (i32) == b'd' as (i32) && (s[(4usize)] as (i32) == b' ' as (i32)) {
                AddMatch(id.wrapping_add((10usize).wrapping_mul(n)),
                         l.wrapping_add(5usize),
                         l,
                         matches);
              }
            }
          } else if s[(1usize)] as (i32) == b'b' as (i32) {
            if s[(2usize)] as (i32) == b'y' as (i32) && (s[(3usize)] as (i32) == b' ' as (i32)) {
              AddMatch(id.wrapping_add((38usize).wrapping_mul(n)),
                       l.wrapping_add(4usize),
                       l,
                       matches);
            }
          } else if s[(1usize)] as (i32) == b'i' as (i32) {
            if s[(2usize)] as (i32) == b'n' as (i32) {
              if s[(3usize)] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((16usize).wrapping_mul(n)),
                         l.wrapping_add(4usize),
                         l,
                         matches);
              }
            } else if s[(2usize)] as (i32) == b's' as (i32) {
              if s[(3usize)] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((47usize).wrapping_mul(n)),
                         l.wrapping_add(4usize),
                         l,
                         matches);
              }
            }
          } else if s[(1usize)] as (i32) == b'f' as (i32) {
            if s[(2usize)] as (i32) == b'o' as (i32) {
              if s[(3usize)] as (i32) == b'r' as (i32) && (s[(4usize)] as (i32) == b' ' as (i32)) {
                AddMatch(id.wrapping_add((25usize).wrapping_mul(n)),
                         l.wrapping_add(5usize),
                         l,
                         matches);
              }
            } else if s[(2usize)] as (i32) == b'r' as (i32) {
              if s[(3usize)] as (i32) == b'o' as (i32) && (s[(4usize)] as (i32) == b'm' as (i32)) &&
                 (s[(5usize)] as (i32) == b' ' as (i32)) {
                AddMatch(id.wrapping_add((37usize).wrapping_mul(n)),
                         l.wrapping_add(6usize),
                         l,
                         matches);
              }
            }
          } else if s[(1usize)] as (i32) == b'o' as (i32) {
            if s[(2usize)] as (i32) == b'f' as (i32) {
              if s[(3usize)] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((8usize).wrapping_mul(n)),
                         l.wrapping_add(4usize),
                         l,
                         matches);
              }
            } else if s[(2usize)] as (i32) == b'n' as (i32) {
              if s[(3usize)] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((45usize).wrapping_mul(n)),
                         l.wrapping_add(4usize),
                         l,
                         matches);
              }
            }
          } else if s[(1usize)] as (i32) == b'n' as (i32) {
            if s[(2usize)] as (i32) == b'o' as (i32) && (s[(3usize)] as (i32) == b't' as (i32)) &&
               (s[(4usize)] as (i32) == b' ' as (i32)) {
              AddMatch(id.wrapping_add((80usize).wrapping_mul(n)),
                       l.wrapping_add(5usize),
                       l,
                       matches);
            }
          } else if s[(1usize)] as (i32) == b't' as (i32) {
            if s[(2usize)] as (i32) == b'h' as (i32) {
              if s[(3usize)] as (i32) == b'e' as (i32) {
                if s[(4usize)] as (i32) == b' ' as (i32) {
                  AddMatch(id.wrapping_add((5usize).wrapping_mul(n)),
                           l.wrapping_add(5usize),
                           l,
                           matches);
                }
              } else if s[(3usize)] as (i32) == b'a' as (i32) {
                if s[(4usize)] as (i32) == b't' as (i32) &&
                   (s[(5usize)] as (i32) == b' ' as (i32)) {
                  AddMatch(id.wrapping_add((29usize).wrapping_mul(n)),
                           l.wrapping_add(6usize),
                           l,
                           matches);
                }
              }
            } else if s[(2usize)] as (i32) == b'o' as (i32) {
              if s[(3usize)] as (i32) == b' ' as (i32) {
                AddMatch(id.wrapping_add((17usize).wrapping_mul(n)),
                         l.wrapping_add(4usize),
                         l,
                         matches);
              }
            }
          } else if s[(1usize)] as (i32) == b'w' as (i32) {
            if s[(2usize)] as (i32) == b'i' as (i32) && (s[(3usize)] as (i32) == b't' as (i32)) &&
               (s[(4usize)] as (i32) == b'h' as (i32)) &&
               (s[(5usize)] as (i32) == b' ' as (i32)) {
              AddMatch(id.wrapping_add((35usize).wrapping_mul(n)),
                       l.wrapping_add(6usize),
                       l,
                       matches);
            }
          }
        } else if s[(0usize)] as (i32) == b'\"' as (i32) {
          AddMatch(id.wrapping_add((19usize).wrapping_mul(n)),
                   l.wrapping_add(1usize),
                   l,
                   matches);
          if s[(1usize)] as (i32) == b'>' as (i32) {
            AddMatch(id.wrapping_add((21usize).wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
          }
        } else if s[(0usize)] as (i32) == b'.' as (i32) {
          AddMatch(id.wrapping_add((20usize).wrapping_mul(n)),
                   l.wrapping_add(1usize),
                   l,
                   matches);
          if s[(1usize)] as (i32) == b' ' as (i32) {
            AddMatch(id.wrapping_add((31usize).wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
            if s[(2usize)] as (i32) == b'T' as (i32) && (s[(3usize)] as (i32) == b'h' as (i32)) {
              if s[(4usize)] as (i32) == b'e' as (i32) {
                if s[(5usize)] as (i32) == b' ' as (i32) {
                  AddMatch(id.wrapping_add((43usize).wrapping_mul(n)),
                           l.wrapping_add(6usize),
                           l,
                           matches);
                }
              } else if s[(4usize)] as (i32) == b'i' as (i32) {
                if s[(5usize)] as (i32) == b's' as (i32) &&
                   (s[(6usize)] as (i32) == b' ' as (i32)) {
                  AddMatch(id.wrapping_add((75usize).wrapping_mul(n)),
                           l.wrapping_add(7usize),
                           l,
                           matches);
                }
              }
            }
          }
        } else if s[(0usize)] as (i32) == b',' as (i32) {
          AddMatch(id.wrapping_add((76usize).wrapping_mul(n)),
                   l.wrapping_add(1usize),
                   l,
                   matches);
          if s[(1usize)] as (i32) == b' ' as (i32) {
            AddMatch(id.wrapping_add((14usize).wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
          }
        } else if s[(0usize)] as (i32) == b'\n' as (i32) {
          AddMatch(id.wrapping_add((22usize).wrapping_mul(n)),
                   l.wrapping_add(1usize),
                   l,
                   matches);
          if s[(1usize)] as (i32) == b'\t' as (i32) {
            AddMatch(id.wrapping_add((50usize).wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
          }
        } else if s[(0usize)] as (i32) == b']' as (i32) {
          AddMatch(id.wrapping_add((24usize).wrapping_mul(n)),
                   l.wrapping_add(1usize),
                   l,
                   matches);
        } else if s[(0usize)] as (i32) == b'\'' as (i32) {
          AddMatch(id.wrapping_add((36usize).wrapping_mul(n)),
                   l.wrapping_add(1usize),
                   l,
                   matches);
        } else if s[(0usize)] as (i32) == b':' as (i32) {
          AddMatch(id.wrapping_add((51usize).wrapping_mul(n)),
                   l.wrapping_add(1usize),
                   l,
                   matches);
        } else if s[(0usize)] as (i32) == b'(' as (i32) {
          AddMatch(id.wrapping_add((57usize).wrapping_mul(n)),
                   l.wrapping_add(1usize),
                   l,
                   matches);
        } else if s[(0usize)] as (i32) == b'=' as (i32) {
          if s[(1usize)] as (i32) == b'\"' as (i32) {
            AddMatch(id.wrapping_add((70usize).wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
          } else if s[(1usize)] as (i32) == b'\'' as (i32) {
            AddMatch(id.wrapping_add((86usize).wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
          }
        } else if s[(0usize)] as (i32) == b'a' as (i32) {
          if s[(1usize)] as (i32) == b'l' as (i32) && (s[(2usize)] as (i32) == b' ' as (i32)) {
            AddMatch(id.wrapping_add((84usize).wrapping_mul(n)),
                     l.wrapping_add(3usize),
                     l,
                     matches);
          }
        } else if s[(0usize)] as (i32) == b'e' as (i32) {
          if s[(1usize)] as (i32) == b'd' as (i32) {
            if s[(2usize)] as (i32) == b' ' as (i32) {
              AddMatch(id.wrapping_add((53usize).wrapping_mul(n)),
                       l.wrapping_add(3usize),
                       l,
                       matches);
            }
          } else if s[(1usize)] as (i32) == b'r' as (i32) {
            if s[(2usize)] as (i32) == b' ' as (i32) {
              AddMatch(id.wrapping_add((82usize).wrapping_mul(n)),
                       l.wrapping_add(3usize),
                       l,
                       matches);
            }
          } else if s[(1usize)] as (i32) == b's' as (i32) {
            if s[(2usize)] as (i32) == b't' as (i32) && (s[(3usize)] as (i32) == b' ' as (i32)) {
              AddMatch(id.wrapping_add((95usize).wrapping_mul(n)),
                       l.wrapping_add(4usize),
                       l,
                       matches);
            }
          }
        } else if s[(0usize)] as (i32) == b'f' as (i32) {
          if s[(1usize)] as (i32) == b'u' as (i32) && (s[(2usize)] as (i32) == b'l' as (i32)) &&
             (s[(3usize)] as (i32) == b' ' as (i32)) {
            AddMatch(id.wrapping_add((90usize).wrapping_mul(n)),
                     l.wrapping_add(4usize),
                     l,
                     matches);
          }
        } else if s[(0usize)] as (i32) == b'i' as (i32) {
          if s[(1usize)] as (i32) == b'v' as (i32) {
            if s[(2usize)] as (i32) == b'e' as (i32) && (s[(3usize)] as (i32) == b' ' as (i32)) {
              AddMatch(id.wrapping_add((92usize).wrapping_mul(n)),
                       l.wrapping_add(4usize),
                       l,
                       matches);
            }
          } else if s[(1usize)] as (i32) == b'z' as (i32) {
            if s[(2usize)] as (i32) == b'e' as (i32) && (s[(3usize)] as (i32) == b' ' as (i32)) {
              AddMatch(id.wrapping_add((100usize).wrapping_mul(n)),
                       l.wrapping_add(4usize),
                       l,
                       matches);
            }
          }
        } else if s[(0usize)] as (i32) == b'l' as (i32) {
          if s[(1usize)] as (i32) == b'e' as (i32) {
            if s[(2usize)] as (i32) == b's' as (i32) && (s[(3usize)] as (i32) == b's' as (i32)) &&
               (s[(4usize)] as (i32) == b' ' as (i32)) {
              AddMatch(id.wrapping_add((93usize).wrapping_mul(n)),
                       l.wrapping_add(5usize),
                       l,
                       matches);
            }
          } else if s[(1usize)] as (i32) == b'y' as (i32) {
            if s[(2usize)] as (i32) == b' ' as (i32) {
              AddMatch(id.wrapping_add((61usize).wrapping_mul(n)),
                       l.wrapping_add(3usize),
                       l,
                       matches);
            }
          }
        } else if s[(0usize)] as (i32) == b'o' as (i32) {
          if s[(1usize)] as (i32) == b'u' as (i32) && (s[(2usize)] as (i32) == b's' as (i32)) &&
             (s[(3usize)] as (i32) == b' ' as (i32)) {
            AddMatch(id.wrapping_add((106usize).wrapping_mul(n)),
                     l.wrapping_add(4usize),
                     l,
                     matches);
          }
        }
      } else {
        let is_all_caps: i32 = if !!(w.transform as (i32) != kUppercaseFirst as (i32)) {
          1i32
        } else {
          0i32
        };
        let s: &[u8];
        if IsMatch(dictionary, w, data, max_length) == 0 {
          {
            continue;
          }
        }
        AddMatch(id.wrapping_add((if is_all_caps != 0 { 44i32 } else { 9i32 } as (usize))
                                   .wrapping_mul(n)),
                 l,
                 l,
                 matches);
        has_found_match = 1i32;
        if l.wrapping_add(1usize) >= max_length {
          {
            continue;
          }
        }
        s = &data[(l as (usize))..];
        if s[(0usize)] as (i32) == b' ' as (i32) {
          AddMatch(id.wrapping_add((if is_all_caps != 0 { 68i32 } else { 4i32 } as (usize))
                                     .wrapping_mul(n)),
                   l.wrapping_add(1usize),
                   l,
                   matches);
        } else if s[(0usize)] as (i32) == b'\"' as (i32) {
          AddMatch(id.wrapping_add((if is_all_caps != 0 { 87i32 } else { 66i32 } as (usize))
                                     .wrapping_mul(n)),
                   l.wrapping_add(1usize),
                   l,
                   matches);
          if s[(1usize)] as (i32) == b'>' as (i32) {
            AddMatch(id.wrapping_add((if is_all_caps != 0 { 97i32 } else { 69i32 } as (usize))
                                       .wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
          }
        } else if s[(0usize)] as (i32) == b'.' as (i32) {
          AddMatch(id.wrapping_add((if is_all_caps != 0 { 101i32 } else { 79i32 } as (usize))
                                     .wrapping_mul(n)),
                   l.wrapping_add(1usize),
                   l,
                   matches);
          if s[(1usize)] as (i32) == b' ' as (i32) {
            AddMatch(id.wrapping_add((if is_all_caps != 0 { 114i32 } else { 88i32 } as (usize))
                                       .wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
          }
        } else if s[(0usize)] as (i32) == b',' as (i32) {
          AddMatch(id.wrapping_add((if is_all_caps != 0 { 112i32 } else { 99i32 } as (usize))
                                     .wrapping_mul(n)),
                   l.wrapping_add(1usize),
                   l,
                   matches);
          if s[(1usize)] as (i32) == b' ' as (i32) {
            AddMatch(id.wrapping_add((if is_all_caps != 0 { 107i32 } else { 58i32 } as (usize))
                                       .wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
          }
        } else if s[(0usize)] as (i32) == b'\'' as (i32) {
          AddMatch(id.wrapping_add((if is_all_caps != 0 { 94i32 } else { 74i32 } as (usize))
                                     .wrapping_mul(n)),
                   l.wrapping_add(1usize),
                   l,
                   matches);
        } else if s[(0usize)] as (i32) == b'(' as (i32) {
          AddMatch(id.wrapping_add((if is_all_caps != 0 { 113i32 } else { 78i32 } as (usize))
                                     .wrapping_mul(n)),
                   l.wrapping_add(1usize),
                   l,
                   matches);
        } else if s[(0usize)] as (i32) == b'=' as (i32) {
          if s[(1usize)] as (i32) == b'\"' as (i32) {
            AddMatch(id.wrapping_add((if is_all_caps != 0 { 105i32 } else { 104i32 } as (usize))
                                       .wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
          } else if s[(1usize)] as (i32) == b'\'' as (i32) {
            AddMatch(id.wrapping_add((if is_all_caps != 0 { 116i32 } else { 108i32 } as (usize))
                                       .wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
          }
        }
      }
    }
  }
  if max_length >= 5usize &&
     (data[(0usize)] as (i32) == b' ' as (i32) || data[(0usize)] as (i32) == b'.' as (i32)) {
    let is_space: i32 = if !!(data[(0usize)] as (i32) == b' ' as (i32)) {
      1i32
    } else {
      0i32
    };
    let mut offset: usize = kStaticDictionaryBuckets[Hash(&data[(1usize)..]) as (usize)] as (usize);
    let mut end: i32 = (offset == 0) as (i32);
    while end == 0 {
      let mut w: DictWord = kStaticDictionaryWords[{
        let _old = offset;
        offset = offset.wrapping_add(1 as (usize));
        _old
      }];
      let l: usize = (w.len as (i32) & 0x1fi32) as (usize);
      let n: usize = 1usize << (*dictionary).size_bits_by_length[l] as (i32);
      let id: usize = w.idx as (usize);
      end = !(w.len as (i32) & 0x80i32 == 0) as (i32);
      w.len = l as (u8);
      if w.transform as (i32) == 0i32 {
        let s: &[u8];
        if IsMatch(dictionary,
                   w,
                   &data[(1usize)..],
                   max_length.wrapping_sub(1usize)) == 0 {
          {
            continue;
          }
        }
        AddMatch(id.wrapping_add((if is_space != 0 { 6i32 } else { 32i32 } as (usize))
                                   .wrapping_mul(n)),
                 l.wrapping_add(1usize),
                 l,
                 matches);
        has_found_match = 1i32;
        if l.wrapping_add(2usize) >= max_length {
          {
            continue;
          }
        }
        s = &data[(l.wrapping_add(1usize) as (usize))..];
        if s[(0usize)] as (i32) == b' ' as (i32) {
          AddMatch(id.wrapping_add((if is_space != 0 { 2i32 } else { 77i32 } as (usize))
                                     .wrapping_mul(n)),
                   l.wrapping_add(2usize),
                   l,
                   matches);
        } else if s[(0usize)] as (i32) == b'(' as (i32) {
          AddMatch(id.wrapping_add((if is_space != 0 { 89i32 } else { 67i32 } as (usize))
                                     .wrapping_mul(n)),
                   l.wrapping_add(2usize),
                   l,
                   matches);
        } else if is_space != 0 {
          if s[(0usize)] as (i32) == b',' as (i32) {
            AddMatch(id.wrapping_add((103usize).wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
            if s[(1usize)] as (i32) == b' ' as (i32) {
              AddMatch(id.wrapping_add((33usize).wrapping_mul(n)),
                       l.wrapping_add(3usize),
                       l,
                       matches);
            }
          } else if s[(0usize)] as (i32) == b'.' as (i32) {
            AddMatch(id.wrapping_add((71usize).wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
            if s[(1usize)] as (i32) == b' ' as (i32) {
              AddMatch(id.wrapping_add((52usize).wrapping_mul(n)),
                       l.wrapping_add(3usize),
                       l,
                       matches);
            }
          } else if s[(0usize)] as (i32) == b'=' as (i32) {
            if s[(1usize)] as (i32) == b'\"' as (i32) {
              AddMatch(id.wrapping_add((81usize).wrapping_mul(n)),
                       l.wrapping_add(3usize),
                       l,
                       matches);
            } else if s[(1usize)] as (i32) == b'\'' as (i32) {
              AddMatch(id.wrapping_add((98usize).wrapping_mul(n)),
                       l.wrapping_add(3usize),
                       l,
                       matches);
            }
          }
        }
      } else if is_space != 0 {
        let is_all_caps: i32 = if !!(w.transform as (i32) != kUppercaseFirst as (i32)) {
          1i32
        } else {
          0i32
        };
        let s: &[u8];
        if IsMatch(dictionary,
                   w,
                   &data[(1usize)..],
                   max_length.wrapping_sub(1usize)) == 0 {
          {
            continue;
          }
        }
        AddMatch(id.wrapping_add((if is_all_caps != 0 { 85i32 } else { 30i32 } as (usize))
                                   .wrapping_mul(n)),
                 l.wrapping_add(1usize),
                 l,
                 matches);
        has_found_match = 1i32;
        if l.wrapping_add(2usize) >= max_length {
          {
            continue;
          }
        }
        s = &data[(l.wrapping_add(1usize) as (usize))..];
        if s[(0usize)] as (i32) == b' ' as (i32) {
          AddMatch(id.wrapping_add((if is_all_caps != 0 { 83i32 } else { 15i32 } as (usize))
                                     .wrapping_mul(n)),
                   l.wrapping_add(2usize),
                   l,
                   matches);
        } else if s[(0usize)] as (i32) == b',' as (i32) {
          if is_all_caps == 0 {
            AddMatch(id.wrapping_add((109usize).wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
          }
          if s[(1usize)] as (i32) == b' ' as (i32) {
            AddMatch(id.wrapping_add((if is_all_caps != 0 { 111i32 } else { 65i32 } as (usize))
                                       .wrapping_mul(n)),
                     l.wrapping_add(3usize),
                     l,
                     matches);
          }
        } else if s[(0usize)] as (i32) == b'.' as (i32) {
          AddMatch(id.wrapping_add((if is_all_caps != 0 { 115i32 } else { 96i32 } as (usize))
                                     .wrapping_mul(n)),
                   l.wrapping_add(2usize),
                   l,
                   matches);
          if s[(1usize)] as (i32) == b' ' as (i32) {
            AddMatch(id.wrapping_add((if is_all_caps != 0 { 117i32 } else { 91i32 } as (usize))
                                       .wrapping_mul(n)),
                     l.wrapping_add(3usize),
                     l,
                     matches);
          }
        } else if s[(0usize)] as (i32) == b'=' as (i32) {
          if s[(1usize)] as (i32) == b'\"' as (i32) {
            AddMatch(id.wrapping_add((if is_all_caps != 0 { 110i32 } else { 118i32 } as (usize))
                                       .wrapping_mul(n)),
                     l.wrapping_add(3usize),
                     l,
                     matches);
          } else if s[(1usize)] as (i32) == b'\'' as (i32) {
            AddMatch(id.wrapping_add((if is_all_caps != 0 { 119i32 } else { 120i32 } as (usize))
                                       .wrapping_mul(n)),
                     l.wrapping_add(3usize),
                     l,
                     matches);
          }
        }
      }
    }
  }
  if max_length >= 6usize {
    if data[(1usize)] as (i32) == b' ' as (i32) &&
       (data[(0usize)] as (i32) == b'e' as (i32) || data[(0usize)] as (i32) == b's' as (i32) ||
        data[(0usize)] as (i32) == b',' as (i32)) ||
       data[(0usize)] as (i32) == 0xc2i32 && (data[(1usize)] as (i32) == 0xa0i32) {
      let mut offset: usize = kStaticDictionaryBuckets[Hash(&data[(2usize)..]) as (usize)] as
                              (usize);
      let mut end: i32 = (offset == 0) as (i32);
      while end == 0 {
        let mut w: DictWord = kStaticDictionaryWords[{
          let _old = offset;
          offset = offset.wrapping_add(1 as (usize));
          _old
        }];
        let l: usize = (w.len as (i32) & 0x1fi32) as (usize);
        let n: usize = 1usize << (*dictionary).size_bits_by_length[l] as (i32);
        let id: usize = w.idx as (usize);
        end = !(w.len as (i32) & 0x80i32 == 0) as (i32);
        w.len = l as (u8);
        if w.transform as (i32) == 0i32 &&
           (IsMatch(dictionary,
                    w,
                    &data[(2usize)..],
                    max_length.wrapping_sub(2usize)) != 0) {
          if data[(0usize)] as (i32) == 0xc2i32 {
            AddMatch(id.wrapping_add((102usize).wrapping_mul(n)),
                     l.wrapping_add(2usize),
                     l,
                     matches);
            has_found_match = 1i32;
          } else if l.wrapping_add(2usize) < max_length &&
                    (data[(l.wrapping_add(2usize) as (usize))] as (i32) == b' ' as (i32)) {
            let t: usize = (if data[(0usize)] as (i32) == b'e' as (i32) {
                              18i32
                            } else if data[(0usize)] as (i32) == b's' as (i32) {
              7i32
            } else {
              13i32
            }) as (usize);
            AddMatch(id.wrapping_add(t.wrapping_mul(n)),
                     l.wrapping_add(3usize),
                     l,
                     matches);
            has_found_match = 1i32;
          }
        }
      }
    }
  }
  if max_length >= 9usize {
    if data[(0usize)] as (i32) == b' ' as (i32) && (data[(1usize)] as (i32) == b't' as (i32)) &&
       (data[(2usize)] as (i32) == b'h' as (i32)) &&
       (data[(3usize)] as (i32) == b'e' as (i32)) &&
       (data[(4usize)] as (i32) == b' ' as (i32)) ||
       data[(0usize)] as (i32) == b'.' as (i32) && (data[(1usize)] as (i32) == b'c' as (i32)) &&
       (data[(2usize)] as (i32) == b'o' as (i32)) &&
       (data[(3usize)] as (i32) == b'm' as (i32)) &&
       (data[(4usize)] as (i32) == b'/' as (i32)) {
      let mut offset: usize = kStaticDictionaryBuckets[Hash(&data[(5usize)..]) as (usize)] as
                              (usize);
      let mut end: i32 = (offset == 0) as (i32);
      while end == 0 {
        let mut w: DictWord = kStaticDictionaryWords[{
          let _old = offset;
          offset = offset.wrapping_add(1 as (usize));
          _old
        }];
        let l: usize = (w.len as (i32) & 0x1fi32) as (usize);
        let n: usize = 1usize << (*dictionary).size_bits_by_length[l] as (i32);
        let id: usize = w.idx as (usize);
        end = !(w.len as (i32) & 0x80i32 == 0) as (i32);
        w.len = l as (u8);
        if w.transform as (i32) == 0i32 &&
           (IsMatch(dictionary,
                    w,
                    &data[(5usize)..],
                    max_length.wrapping_sub(5usize)) != 0) {
          AddMatch(id.wrapping_add((if data[(0usize)] as (i32) == b' ' as (i32) {
                                      41i32
                                    } else {
                                      72i32
                                    } as (usize))
                                       .wrapping_mul(n)),
                   l.wrapping_add(5usize),
                   l,
                   matches);
          has_found_match = 1i32;
          if l.wrapping_add(5usize) < max_length {
            let s: &[u8] = &data[(l.wrapping_add(5usize) as (usize))..];
            if data[(0usize)] as (i32) == b' ' as (i32) {
              if l.wrapping_add(8usize) < max_length && (s[(0usize)] as (i32) == b' ' as (i32)) &&
                 (s[(1usize)] as (i32) == b'o' as (i32)) &&
                 (s[(2usize)] as (i32) == b'f' as (i32)) &&
                 (s[(3usize)] as (i32) == b' ' as (i32)) {
                AddMatch(id.wrapping_add((62usize).wrapping_mul(n)),
                         l.wrapping_add(9usize),
                         l,
                         matches);
                if l.wrapping_add(12usize) < max_length &&
                   (s[(4usize)] as (i32) == b't' as (i32)) &&
                   (s[(5usize)] as (i32) == b'h' as (i32)) &&
                   (s[(6usize)] as (i32) == b'e' as (i32)) &&
                   (s[(7usize)] as (i32) == b' ' as (i32)) {
                  AddMatch(id.wrapping_add((73usize).wrapping_mul(n)),
                           l.wrapping_add(13usize),
                           l,
                           matches);
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
