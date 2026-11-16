use core::cmp::{max, min};

use brotli_decompressor::dictionary::{
    kBrotliDictionary, kBrotliDictionaryOffsetsByLength, kBrotliDictionarySizeBitsByLength,
};

use crate::enc::static_dict_lut::{
    kDictHashMul32, kDictNumBits, kStaticDictionaryBuckets, kStaticDictionaryWords, DictWord,
};

pub const kNumDistanceCacheEntries: usize = 4;

const kUppercaseFirst: u8 = 10u8;

static kOmitLastNTransforms: [u8; 10] = [0, 12, 27, 23, 42, 63, 56, 48, 59, 64];

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

#[inline(always)]
pub fn BrotliGetDictionary() -> &'static BrotliDictionary {
    &kBrotliEncDictionary
}
#[inline(always)]
pub fn BROTLI_UNALIGNED_LOAD32(sl: &[u8]) -> u32 {
    let mut p = [0u8; 4];
    p[..].clone_from_slice(sl.split_at(4).0);
    (p[0] as u32) | ((p[1] as u32) << 8) | ((p[2] as u32) << 16) | ((p[3] as u32) << 24)
}
#[inline(always)]
pub fn Hash(data: &[u8]) -> u32 {
    let h: u32 = BROTLI_UNALIGNED_LOAD32(data).wrapping_mul(kDictHashMul32);
    h >> (32i32 - kDictNumBits)
}
#[inline(always)]
pub fn BROTLI_UNALIGNED_LOAD64(sl: &[u8]) -> u64 {
    let mut p = [0u8; 8];
    p[..].clone_from_slice(sl.split_at(8).0);
    (p[0] as u64)
        | ((p[1] as u64) << 8)
        | ((p[2] as u64) << 16)
        | ((p[3] as u64) << 24)
        | ((p[4] as u64) << 32)
        | ((p[5] as u64) << 40)
        | ((p[6] as u64) << 48)
        | ((p[7] as u64) << 56)
}
#[inline(always)]
pub fn BROTLI_UNALIGNED_STORE64(outp: &mut [u8], v: u64) {
    let p = [
        (v & 0xff) as u8,
        ((v >> 8) & 0xff) as u8,
        ((v >> 16) & 0xff) as u8,
        ((v >> 24) & 0xff) as u8,
        ((v >> 32) & 0xff) as u8,
        ((v >> 40) & 0xff) as u8,
        ((v >> 48) & 0xff) as u8,
        ((v >> 56) & 0xff) as u8,
    ];
    outp.split_at_mut(8).0.clone_from_slice(&p[..]);
}

macro_rules! sub_match {
    ($s1 : expr, $s2 : expr, $limit : expr, $matched : expr, $split_pair1 : expr, $split_pair2 : expr, $s1_lo : expr, $s2_lo : expr, $s1_as_64 : expr, $s2_as_64 : expr, $vec_len: expr) => {
        $split_pair1 = $s1.split_at($vec_len);
        $s1_lo[..$vec_len].clone_from_slice($split_pair1.0);
        $s1 = $split_pair1.1;
        $split_pair2 = $s2.split_at($vec_len);
        $s2_lo[..$vec_len].clone_from_slice($split_pair2.0);
        $s2 = $split_pair2.1;
        $limit -= $vec_len;
        for index in 0..($vec_len >> 3) {
            $s1_as_64 = BROTLI_UNALIGNED_LOAD64(&$s1_lo[(index << 3)..((index + 1) << 3)]);
            $s2_as_64 = BROTLI_UNALIGNED_LOAD64(&$s2_lo[(index << 3)..((index + 1) << 3)]);
            if $s2_as_64 == $s1_as_64 {
                $matched = $matched.wrapping_add(8) as u32 as usize;
            } else {
                $matched = $matched
                    .wrapping_add((($s2_as_64 ^ $s1_as_64).trailing_zeros() >> 3) as usize)
                    as u32 as usize;
                return $matched;
            }
        }
    };
}

macro_rules! sub_match8 {
    ($s1 : expr, $s2 : expr, $limit : expr, $matched : expr, $s1_as_64 : expr, $s2_as_64 : expr) => {
        $limit -= 8;
        $s1_as_64 = BROTLI_UNALIGNED_LOAD64($s1);
        $s1 = $s1.split_at(8).1;
        $s2_as_64 = BROTLI_UNALIGNED_LOAD64($s2);
        $s2 = $s2.split_at(8).1;
        if $s2_as_64 == $s1_as_64 {
            $matched = $matched.wrapping_add(8) as u32 as usize;
        } else {
            $matched = $matched
                .wrapping_add((($s2_as_64 ^ $s1_as_64).trailing_zeros() >> 3) as usize)
                as u32 as usize;
            return $matched;
        }
    };
}

// factor of 10 slower (example takes 158s, not 30, and for the 30 seconds run it took 15 of them)
pub fn SlowerFindMatchLengthWithLimit(s1: &[u8], s2: &[u8], limit: usize) -> usize {
    for index in 0..limit {
        if s1[index] != s2[index] {
            return index;
        }
    }
    limit
}
// factor of 5 slower (example takes 90 seconds)
pub fn FindMatchLengthWithLimit(s1: &[u8], s2: &[u8], limit: usize) -> usize {
    for (index, pair) in s1[..limit].iter().zip(s2[..limit].iter()).enumerate() {
        if *pair.0 != *pair.1 {
            return index;
        }
    }
    limit
}
pub fn FindMatchLengthWithLimitMin4(s1: &[u8], s2: &[u8], limit: usize) -> usize {
    let (s1_start, s1_rest) = s1.split_at(5);
    let (s2_start, s2_rest) = s2.split_at(5);
    let v0 = BROTLI_UNALIGNED_LOAD32(s1_start);
    let v1 = BROTLI_UNALIGNED_LOAD32(s2_start);
    let beyond_ok = s1_start[4] != s2_start[4];
    if v0 != v1 {
        return 0;
    }
    if limit <= 4 || beyond_ok {
        return min(limit, 4);
    }
    ComplexFindMatchLengthWithLimit(s1_rest, s2_rest, limit - 5) + 5
}
#[inline]
pub fn ComplexFindMatchLengthWithLimit(mut s1: &[u8], mut s2: &[u8], mut limit: usize) -> usize {
    let mut matched: usize = 0usize;
    let mut s1_as_64: u64;
    let mut s2_as_64: u64;
    if limit >= 8 {
        sub_match8!(s1, s2, limit, matched, s1_as_64, s2_as_64);
        if limit >= 16 {
            let mut split_pair1: (&[u8], &[u8]);
            let mut split_pair2: (&[u8], &[u8]);
            {
                let mut s1_lo = [0u8; 16];
                let mut s1_hi = [0u8; 16];
                sub_match!(
                    s1,
                    s2,
                    limit,
                    matched,
                    split_pair1,
                    split_pair2,
                    s1_lo,
                    s1_hi,
                    s1_as_64,
                    s2_as_64,
                    16
                );
            }
            if limit >= 32 {
                let mut s1_lo_a = [0u8; 128];
                let mut s1_hi_a = [0u8; 128];
                sub_match!(
                    s1,
                    s2,
                    limit,
                    matched,
                    split_pair1,
                    split_pair2,
                    s1_lo_a,
                    s1_hi_a,
                    s1_as_64,
                    s2_as_64,
                    32
                );
                if limit >= 64 {
                    sub_match!(
                        s1,
                        s2,
                        limit,
                        matched,
                        split_pair1,
                        split_pair2,
                        s1_lo_a,
                        s1_hi_a,
                        s1_as_64,
                        s2_as_64,
                        64
                    );
                    while limit >= 128 {
                        sub_match!(
                            s1,
                            s2,
                            limit,
                            matched,
                            split_pair1,
                            split_pair2,
                            s1_lo_a,
                            s1_hi_a,
                            s1_as_64,
                            s2_as_64,
                            128
                        );
                    }
                }
            }
        }
        while limit >= 8 {
            sub_match8!(s1, s2, limit, matched, s1_as_64, s2_as_64);
        }
    }
    assert!(s1.len() >= (limit & 7));
    assert!(s2.len() >= (limit & 7));
    for index in 0..(limit & 7) {
        if s1[index] != s2[index] {
            return matched + index;
        }
    }
    matched + (limit & 7) // made it through the loop
}

pub fn slowFindMatchLengthWithLimit(s1: &[u8], s2: &[u8], limit: usize) -> usize {
    for (index, it) in s1[..limit].iter().zip(s2[..limit].iter()).enumerate() {
        if it.0 != it.1 {
            return index;
        }
    }
    limit
}

// TODO: delete because we probably don't want to make it public(?)
#[deprecated(note = "use BrotliDictionary::is_match(...) instead")]
pub fn IsMatch(dictionary: &BrotliDictionary, w: DictWord, data: &[u8], max_length: usize) -> i32 {
    if dictionary.is_match(w, data, max_length) {
        1
    } else {
        0
    }
}

impl BrotliDictionary {
    pub(crate) fn is_match(&self, w: DictWord, data: &[u8], max_length: usize) -> bool {
        if w.l as usize > max_length {
            false
        } else {
            let offset: usize = (self.offsets_by_length[w.l as usize] as usize)
                .wrapping_add((w.len() as usize).wrapping_mul(w.idx() as usize));
            let dict = self.data.split_at(offset).1;
            if w.transform() == 0 {
                FindMatchLengthWithLimit(dict, data, w.l as usize) == w.l as usize
            } else if w.transform() == 10 {
                dict[0] >= b'a'
                    && dict[0] <= b'z'
                    && dict[0] ^ 32 == data[0]
                    && (FindMatchLengthWithLimit(
                        dict.split_at(1).1,
                        data.split_at(1).1,
                        (w.len() as u32).wrapping_sub(1) as usize,
                    ) == (w.len() as u32).wrapping_sub(1) as usize)
            } else {
                for i in 0..w.len() as usize {
                    if dict[i] >= b'a' && dict[i] <= b'z' {
                        if dict[i] ^ 32 != data[i] {
                            return false;
                        }
                    } else if dict[i] != data[i] {
                        return false;
                    }
                }
                true
            }
        }
    }
}

fn AddMatch(distance: usize, len: usize, len_code: usize, matches: &mut [u32]) {
    let match_: u32 = (distance << 5).wrapping_add(len_code) as u32;
    matches[len] = min(matches[len], match_);
}

// TODO: delete because we probably don't want to make it public(?)
#[deprecated(note = "use BrotliDictionary::find_all_matches(...) instead")]
pub fn BrotliFindAllStaticDictionaryMatches(
    dictionary: &BrotliDictionary,
    data: &[u8],
    min_length: usize,
    max_length: usize,
    matches: &mut [u32],
) -> i32 {
    if dictionary.find_all_matches(data, min_length, max_length, matches) {
        1
    } else {
        0
    }
}

impl BrotliDictionary {
    fn dict_match_length(&self, data: &[u8], id: usize, len: usize, maxlen: usize) -> usize {
        let offset: usize =
            (self.offsets_by_length[len] as usize).wrapping_add(len.wrapping_mul(id));
        FindMatchLengthWithLimit(self.data.split_at(offset).1, data, min(len, maxlen))
    }

    pub(crate) fn find_all_matches(
        &self,
        data: &[u8],
        min_length: usize,
        max_length: usize,
        matches: &mut [u32],
    ) -> bool {
        let mut has_found_match = false;
        let mut offset: usize = kStaticDictionaryBuckets[Hash(data) as usize] as usize;
        let mut end = (offset == 0);
        while !end {
            let mut w: DictWord = kStaticDictionaryWords[offset];
            offset = offset.wrapping_add(1);
            let l = (w.len() & 0x1f) as usize;
            let n: usize = 1usize << self.size_bits_by_length[l];
            let id = w.idx() as usize;
            end = !(w.len() & 0x80 == 0);
            w.l = l as u8;
            if w.transform() == 0 {
                let matchlen: usize = self.dict_match_length(data, id, l, max_length);
                let mut minlen: usize;

                if matchlen == l {
                    //eprint!("Adding match {} {} {}\n", w.len(), w.transform(), w.idx());
                    AddMatch(id, l, l, matches);
                    has_found_match = true;
                }
                if matchlen >= l.wrapping_sub(1) {
                    //eprint!("Bdding match {} {} {}\n", w.len(), w.transform(), w.idx());
                    AddMatch(
                        id.wrapping_add((12usize).wrapping_mul(n)),
                        l.wrapping_sub(1),
                        l,
                        matches,
                    );
                    if l.wrapping_add(2) < max_length
                        && data[l.wrapping_sub(1)] == b'i'
                        && data[l] == b'n'
                        && data[l.wrapping_add(1)] == b'g'
                        && data[l.wrapping_add(2)] == b' '
                    {
                        //eprint!("Cdding match {} {} {}\n", w.len(), w.transform(), w.idx());
                        AddMatch(
                            id.wrapping_add((49usize).wrapping_mul(n)),
                            l.wrapping_add(3),
                            l,
                            matches,
                        );
                    }
                    has_found_match = true;
                }
                minlen = min_length;
                if l > 9 {
                    minlen = max(minlen, l.wrapping_sub(9));
                }
                let maxlen: usize = min(matchlen, l.wrapping_sub(2));
                for len in minlen..=maxlen {
                    //eprint!("Ddding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                    AddMatch(
                        id.wrapping_add(
                            (kOmitLastNTransforms[l.wrapping_sub(len)] as usize).wrapping_mul(n),
                        ),
                        len,
                        l,
                        matches,
                    );
                    has_found_match = true;
                }

                if matchlen < l || l.wrapping_add(6) >= max_length {
                    continue;
                }
                let s: &[u8] = data.split_at(l).1;
                if s[0] == b' ' {
                    //eprint!("Edding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                    AddMatch(id.wrapping_add(n), l.wrapping_add(1), l, matches);
                    if s[1] == b'a' {
                        if s[2] == b' ' {
                            //eprint!("Fdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                            AddMatch(
                                id.wrapping_add((28usize).wrapping_mul(n)),
                                l.wrapping_add(3),
                                l,
                                matches,
                            );
                        } else if s[2] == b's' {
                            if s[3] == b' ' {
                                //eprint!("Gdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                                AddMatch(
                                    id.wrapping_add((46usize).wrapping_mul(n)),
                                    l.wrapping_add(4),
                                    l,
                                    matches,
                                );
                            }
                        } else if s[2] == b't' {
                            if s[3] == b' ' {
                                //eprint!("Hdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                                AddMatch(
                                    id.wrapping_add((60usize).wrapping_mul(n)),
                                    l.wrapping_add(4),
                                    l,
                                    matches,
                                );
                            }
                        } else if s[2] == b'n' && s[3] == b'd' && (s[4] == b' ') {
                            //eprint!("Idding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                            AddMatch(
                                id.wrapping_add((10usize).wrapping_mul(n)),
                                l.wrapping_add(5),
                                l,
                                matches,
                            );
                        }
                    } else if s[1] == b'b' {
                        if s[2] == b'y' && s[3] == b' ' {
                            //eprint!("Jdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                            AddMatch(
                                id.wrapping_add((38usize).wrapping_mul(n)),
                                l.wrapping_add(4),
                                l,
                                matches,
                            );
                        }
                    } else if s[1] == b'i' {
                        if s[2] == b'n' {
                            if s[3] == b' ' {
                                //eprint!("Kdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                                AddMatch(
                                    id.wrapping_add((16usize).wrapping_mul(n)),
                                    l.wrapping_add(4),
                                    l,
                                    matches,
                                );
                            }
                        } else if s[2] == b's' && s[3] == b' ' {
                            //eprint!("Ldding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                            AddMatch(
                                id.wrapping_add((47usize).wrapping_mul(n)),
                                l.wrapping_add(4),
                                l,
                                matches,
                            );
                        }
                    } else if s[1] == b'f' {
                        if s[2] == b'o' {
                            if s[3] == b'r' && s[4] == b' ' {
                                //eprint!("Mdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                                AddMatch(
                                    id.wrapping_add((25usize).wrapping_mul(n)),
                                    l.wrapping_add(5),
                                    l,
                                    matches,
                                );
                            }
                        } else if s[2] == b'r' && s[3] == b'o' && s[4] == b'm' && s[5] == b' ' {
                            //eprint!("Ndding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                            AddMatch(
                                id.wrapping_add((37usize).wrapping_mul(n)),
                                l.wrapping_add(6),
                                l,
                                matches,
                            );
                        }
                    } else if s[1] == b'o' {
                        if s[2] == b'f' {
                            if s[3] == b' ' {
                                //eprint!("Odding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                                AddMatch(
                                    id.wrapping_add((8usize).wrapping_mul(n)),
                                    l.wrapping_add(4),
                                    l,
                                    matches,
                                );
                            }
                        } else if s[2] == b'n' && s[3] == b' ' {
                            //eprint!("Pdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                            AddMatch(
                                id.wrapping_add((45usize).wrapping_mul(n)),
                                l.wrapping_add(4),
                                l,
                                matches,
                            );
                        }
                    } else if s[1] == b'n' {
                        if s[2] == b'o' && s[3] == b't' && s[4] == b' ' {
                            //eprint!("Qdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                            AddMatch(
                                id.wrapping_add((80usize).wrapping_mul(n)),
                                l.wrapping_add(5),
                                l,
                                matches,
                            );
                        }
                    } else if s[1] == b't' {
                        if s[2] == b'h' {
                            if s[3] == b'e' {
                                if s[4] == b' ' {
                                    //eprint!("Rdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                                    AddMatch(
                                        id.wrapping_add((5usize).wrapping_mul(n)),
                                        l.wrapping_add(5),
                                        l,
                                        matches,
                                    );
                                }
                            } else if s[3] == b'a' && s[4] == b't' && s[5] == b' ' {
                                //eprint!("Sdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                                AddMatch(
                                    id.wrapping_add((29usize).wrapping_mul(n)),
                                    l.wrapping_add(6),
                                    l,
                                    matches,
                                );
                            }
                        } else if s[2] == b'o' && s[3] == b' ' {
                            //eprint!("Tdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                            AddMatch(
                                id.wrapping_add((17usize).wrapping_mul(n)),
                                l.wrapping_add(4),
                                l,
                                matches,
                            );
                        }
                    } else if s[1] == b'w'
                        && s[2] == b'i'
                        && s[3] == b't'
                        && s[4] == b'h'
                        && s[5] == b' '
                    {
                        //eprint!("Udding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                        AddMatch(
                            id.wrapping_add((35usize).wrapping_mul(n)),
                            l.wrapping_add(6),
                            l,
                            matches,
                        );
                    }
                } else if s[0] == b'\"' {
                    //eprint!("Vdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                    AddMatch(
                        id.wrapping_add((19usize).wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    if s[1] == b'>' {
                        //eprint!("Wdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                        AddMatch(
                            id.wrapping_add((21usize).wrapping_mul(n)),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                } else if s[0] == b'.' {
                    //eprint!("Xdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                    AddMatch(
                        id.wrapping_add((20usize).wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    if s[1] == b' ' {
                        //eprint!("Ydding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                        AddMatch(
                            id.wrapping_add((31usize).wrapping_mul(n)),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                        if s[2] == b'T' && s[3] == b'h' {
                            if s[4] == b'e' {
                                if s[5] == b' ' {
                                    //eprint!("Zdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                                    AddMatch(
                                        id.wrapping_add((43usize).wrapping_mul(n)),
                                        l.wrapping_add(6),
                                        l,
                                        matches,
                                    );
                                }
                            } else if s[4] == b'i' && s[5] == b's' && s[6] == b' ' {
                                //eprint!("AAdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                                AddMatch(
                                    id.wrapping_add((75usize).wrapping_mul(n)),
                                    l.wrapping_add(7),
                                    l,
                                    matches,
                                );
                            }
                        }
                    }
                } else if s[0] == b',' {
                    //eprint!("ABdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                    AddMatch(
                        id.wrapping_add((76usize).wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    if s[1] == b' ' {
                        //eprint!("ACdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                        AddMatch(
                            id.wrapping_add((14usize).wrapping_mul(n)),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                } else if s[0] == b'\n' {
                    //eprint!("ADdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                    AddMatch(
                        id.wrapping_add((22usize).wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    if s[1] == b'\t' {
                        //eprint!("AEdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                        AddMatch(
                            id.wrapping_add((50usize).wrapping_mul(n)),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                } else if s[0] == b']' {
                    //eprint!("AFdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                    AddMatch(
                        id.wrapping_add((24usize).wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                } else if s[0] == b'\'' {
                    //eprint!("AGdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                    AddMatch(
                        id.wrapping_add((36usize).wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                } else if s[0] == b':' {
                    //eprint!("AHdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                    AddMatch(
                        id.wrapping_add((51usize).wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                } else if s[0] == b'(' {
                    //eprint!("AIdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                    AddMatch(
                        id.wrapping_add((57usize).wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                } else if s[0] == b'=' {
                    if s[1] == b'\"' {
                        //eprint!("AJdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                        AddMatch(
                            id.wrapping_add((70usize).wrapping_mul(n)),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    } else if s[1] == b'\'' {
                        //eprint!("AKdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                        AddMatch(
                            id.wrapping_add((86usize).wrapping_mul(n)),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                } else if s[0] == b'a' {
                    if s[1] == b'l' && s[2] == b' ' {
                        //eprint!("ALdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                        AddMatch(
                            id.wrapping_add((84usize).wrapping_mul(n)),
                            l.wrapping_add(3),
                            l,
                            matches,
                        );
                    }
                } else if s[0] == b'e' {
                    if s[1] == b'd' {
                        if s[2] == b' ' {
                            //eprint!("AMdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                            AddMatch(
                                id.wrapping_add((53usize).wrapping_mul(n)),
                                l.wrapping_add(3),
                                l,
                                matches,
                            );
                        }
                    } else if s[1] == b'r' {
                        if s[2] == b' ' {
                            //eprint!("ANdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                            AddMatch(
                                id.wrapping_add((82usize).wrapping_mul(n)),
                                l.wrapping_add(3),
                                l,
                                matches,
                            );
                        }
                    } else if s[1] == b's' && s[2] == b't' && s[3] == b' ' {
                        //eprint!("AOdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                        AddMatch(
                            id.wrapping_add((95usize).wrapping_mul(n)),
                            l.wrapping_add(4),
                            l,
                            matches,
                        );
                    }
                } else if s[0] == b'f' {
                    if s[1] == b'u' && s[2] == b'l' && s[3] == b' ' {
                        //eprint!("APdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                        AddMatch(
                            id.wrapping_add((90usize).wrapping_mul(n)),
                            l.wrapping_add(4),
                            l,
                            matches,
                        );
                    }
                } else if s[0] == b'i' {
                    if s[1] == b'v' {
                        if s[2] == b'e' && s[3] == b' ' {
                            //eprint!("AQdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                            AddMatch(
                                id.wrapping_add((92usize).wrapping_mul(n)),
                                l.wrapping_add(4),
                                l,
                                matches,
                            );
                        }
                    } else if s[1] == b'z' && s[2] == b'e' && s[3] == b' ' {
                        //eprint!("ARdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                        AddMatch(
                            id.wrapping_add((100usize).wrapping_mul(n)),
                            l.wrapping_add(4),
                            l,
                            matches,
                        );
                    }
                } else if s[0] == b'l' {
                    if s[1] == b'e' {
                        if s[2] == b's' && s[3] == b's' && s[4] == b' ' {
                            //eprint!("ASdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                            AddMatch(
                                id.wrapping_add((93usize).wrapping_mul(n)),
                                l.wrapping_add(5),
                                l,
                                matches,
                            );
                        }
                    } else if s[1] == b'y' && s[2] == b' ' {
                        //eprint!("ATdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                        AddMatch(
                            id.wrapping_add((61usize).wrapping_mul(n)),
                            l.wrapping_add(3),
                            l,
                            matches,
                        );
                    }
                } else if s[0] == b'o' && s[1] == b'u' && s[2] == b's' && s[3] == b' ' {
                    //eprint!("AUdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), len);
                    AddMatch(
                        id.wrapping_add((106usize).wrapping_mul(n)),
                        l.wrapping_add(4),
                        l,
                        matches,
                    );
                }
            } else {
                let is_all_caps = w.transform() != kUppercaseFirst;

                if !self.is_match(w, data, max_length) {
                    continue;
                }
                //eprint!("AVdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                AddMatch(
                    id.wrapping_add((if is_all_caps { 44usize } else { 9usize }).wrapping_mul(n)),
                    l,
                    l,
                    matches,
                );
                has_found_match = true;
                if l.wrapping_add(1) >= max_length {
                    continue;
                }
                let s: &[u8] = data.split_at(l).1;
                if s[0] == b' ' {
                    //eprint!("AWdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps { 68usize } else { 4usize }).wrapping_mul(n),
                        ),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                } else if s[0] == b'\"' {
                    //eprint!("AXdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps { 87usize } else { 66usize }).wrapping_mul(n),
                        ),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    if s[1] == b'>' {
                        //eprint!("AYdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps { 97usize } else { 69usize }).wrapping_mul(n),
                            ),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                } else if s[0] == b'.' {
                    //eprint!("AZdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps { 101usize } else { 79usize }).wrapping_mul(n),
                        ),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    if s[1] == b' ' {
                        //eprint!("BAdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps { 114usize } else { 88usize }).wrapping_mul(n),
                            ),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                } else if s[0] == b',' {
                    //eprint!("BBdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps { 112usize } else { 99usize }).wrapping_mul(n),
                        ),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    if s[1] == b' ' {
                        //eprint!("BCdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps { 107usize } else { 58usize }).wrapping_mul(n),
                            ),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                } else if s[0] == b'\'' {
                    //eprint!("BDdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps { 94usize } else { 74usize }).wrapping_mul(n),
                        ),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                } else if s[0] == b'(' {
                    //eprint!("BEdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps { 113usize } else { 78usize }).wrapping_mul(n),
                        ),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                } else if s[0] == b'=' {
                    if s[1] == b'\"' {
                        //eprint!("BFdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps { 105usize } else { 104usize }).wrapping_mul(n),
                            ),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    } else if s[1] == b'\'' {
                        //eprint!("BGdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps { 116usize } else { 108usize }).wrapping_mul(n),
                            ),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                }
            }
        }

        if max_length >= 5 && (data[0] == b' ' || data[0] == b'.') {
            let is_space = data[0] == b' ';
            let mut offset: usize =
                kStaticDictionaryBuckets[Hash(data.split_at(1).1) as usize] as usize;
            let mut end = (offset == 0);
            while !end {
                let mut w: DictWord = kStaticDictionaryWords[offset];
                offset = offset.wrapping_add(1);
                let l = (w.len() & 0x1f) as usize;
                let n: usize = 1usize << self.size_bits_by_length[l];
                let id = w.idx() as usize;
                end = !(w.len() & 0x80 == 0);
                w.l = l as u8;
                if w.transform() == 0 {
                    if !self.is_match(w, data.split_at(1).1, max_length.wrapping_sub(1)) {
                        continue;
                    }
                    //eprint!("BHdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                    AddMatch(
                        id.wrapping_add((if is_space { 6usize } else { 32usize }).wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    has_found_match = true;
                    if l.wrapping_add(2) >= max_length {
                        continue;
                    }
                    let s: &[u8] = data.split_at(l.wrapping_add(1)).1;
                    if s[0] == b' ' {
                        //eprint!("BIdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                        AddMatch(
                            id.wrapping_add(
                                (if is_space { 2usize } else { 77usize }).wrapping_mul(n),
                            ),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    } else if s[0] == b'(' {
                        //eprint!("BJdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                        AddMatch(
                            id.wrapping_add(
                                (if is_space { 89usize } else { 67usize }).wrapping_mul(n),
                            ),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    } else if is_space {
                        if s[0] == b',' {
                            //eprint!("BKdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                            AddMatch(
                                id.wrapping_add((103usize).wrapping_mul(n)),
                                l.wrapping_add(2),
                                l,
                                matches,
                            );
                            if s[1] == b' ' {
                                //eprint!("BLdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                                AddMatch(
                                    id.wrapping_add((33usize).wrapping_mul(n)),
                                    l.wrapping_add(3),
                                    l,
                                    matches,
                                );
                            }
                        } else if s[0] == b'.' {
                            //eprint!("BMdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                            AddMatch(
                                id.wrapping_add((71usize).wrapping_mul(n)),
                                l.wrapping_add(2),
                                l,
                                matches,
                            );
                            if s[1] == b' ' {
                                //eprint!("BNdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                                AddMatch(
                                    id.wrapping_add((52usize).wrapping_mul(n)),
                                    l.wrapping_add(3),
                                    l,
                                    matches,
                                );
                            }
                        } else if s[0] == b'=' {
                            if s[1] == b'\"' {
                                //eprint!("BOdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                                AddMatch(
                                    id.wrapping_add((81usize).wrapping_mul(n)),
                                    l.wrapping_add(3),
                                    l,
                                    matches,
                                );
                            } else if s[1] == b'\'' {
                                //eprint!("BPdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                                AddMatch(
                                    id.wrapping_add((98usize).wrapping_mul(n)),
                                    l.wrapping_add(3),
                                    l,
                                    matches,
                                );
                            }
                        }
                    }
                } else if is_space {
                    let is_all_caps = w.transform() != kUppercaseFirst;

                    if !self.is_match(w, data.split_at(1).1, max_length.wrapping_sub(1)) {
                        continue;
                    }
                    //eprint!("CAdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                    AddMatch(
                        id.wrapping_add(
                            (if is_all_caps { 85usize } else { 30usize }).wrapping_mul(n),
                        ),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    has_found_match = true;
                    if l.wrapping_add(2) >= max_length {
                        continue;
                    }
                    let s: &[u8] = data.split_at(l.wrapping_add(1)).1;
                    if s[0] == b' ' {
                        //eprint!("CBdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps { 83usize } else { 15usize }).wrapping_mul(n),
                            ),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    } else if s[0] == b',' {
                        if !is_all_caps {
                            //eprint!("CCdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                            AddMatch(
                                id.wrapping_add((109usize).wrapping_mul(n)),
                                l.wrapping_add(2),
                                l,
                                matches,
                            );
                        }
                        if s[1] == b' ' {
                            //eprint!("CDdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                            AddMatch(
                                id.wrapping_add(
                                    (if is_all_caps { 111usize } else { 65usize }).wrapping_mul(n),
                                ),
                                l.wrapping_add(3),
                                l,
                                matches,
                            );
                        }
                    } else if s[0] == b'.' {
                        //eprint!("CEdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                        AddMatch(
                            id.wrapping_add(
                                (if is_all_caps { 115usize } else { 96usize }).wrapping_mul(n),
                            ),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                        if s[1] == b' ' {
                            //eprint!("CFdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                            AddMatch(
                                id.wrapping_add(
                                    (if is_all_caps { 117usize } else { 91usize }).wrapping_mul(n),
                                ),
                                l.wrapping_add(3),
                                l,
                                matches,
                            );
                        }
                    } else if s[0] == b'=' {
                        if s[1] == b'\"' {
                            //eprint!("CGdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                            AddMatch(
                                id.wrapping_add(
                                    (if is_all_caps { 110usize } else { 118usize }).wrapping_mul(n),
                                ),
                                l.wrapping_add(3),
                                l,
                                matches,
                            );
                        } else if s[1] == b'\'' {
                            //eprint!("CHdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                            AddMatch(
                                id.wrapping_add(
                                    (if is_all_caps { 119usize } else { 120usize }).wrapping_mul(n),
                                ),
                                l.wrapping_add(3),
                                l,
                                matches,
                            );
                        }
                    }
                }
            }
        }
        if max_length >= 6
            && (data[1] == b' ' && (data[0] == b'e' || data[0] == b's' || data[0] == b',')
                || data[0] == 0xc2 && data[1] == 0xa0)
        {
            let mut offset = kStaticDictionaryBuckets[Hash(data.split_at(2).1) as usize] as usize;
            let mut end = (offset == 0);
            while !end {
                let mut w: DictWord = kStaticDictionaryWords[offset];
                offset = offset.wrapping_add(1);
                let l = (w.len() & 0x1f) as usize;
                let n: usize = 1usize << self.size_bits_by_length[l];
                let id = w.idx() as usize;
                end = !(w.len() & 0x80 == 0);
                w.l = l as u8;
                if w.transform() == 0
                    && self.is_match(w, data.split_at(2).1, max_length.wrapping_sub(2))
                {
                    if data[0] == 0xc2 {
                        //eprint!("CIdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                        AddMatch(
                            id.wrapping_add((102usize).wrapping_mul(n)),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                        has_found_match = true;
                    } else if l.wrapping_add(2) < max_length && data[l.wrapping_add(2)] == b' ' {
                        let t = (if data[0] == b'e' {
                            18
                        } else if data[0] == b's' {
                            7
                        } else {
                            13
                        }) as usize;
                        //eprint!("CJdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                        AddMatch(
                            id.wrapping_add(t.wrapping_mul(n)),
                            l.wrapping_add(3),
                            l,
                            matches,
                        );
                        has_found_match = true;
                    }
                }
            }
        }
        if max_length >= 9
            && (data[0] == b' '
                && data[1] == b't'
                && data[2] == b'h'
                && data[3] == b'e'
                && data[4] == b' '
                || data[0] == b'.'
                    && data[1] == b'c'
                    && data[2] == b'o'
                    && data[3] == b'm'
                    && data[4] == b'/')
        {
            let mut offset = kStaticDictionaryBuckets[Hash(data.split_at(5).1) as usize] as usize;
            let mut end = (offset == 0);
            while !end {
                let mut w: DictWord = kStaticDictionaryWords[offset];
                offset = offset.wrapping_add(1);
                let l = (w.len() & 0x1f) as usize;
                let n: usize = 1usize << self.size_bits_by_length[l];
                let id = w.idx() as usize;
                end = !(w.len() & 0x80 == 0);
                w.l = l as u8;
                if w.transform() == 0
                    && self.is_match(w, data.split_at(5).1, max_length.wrapping_sub(5))
                {
                    //eprint!("CKdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                    AddMatch(
                        id.wrapping_add(
                            (if data[0] == b' ' { 41_usize } else { 72_usize }).wrapping_mul(n),
                        ),
                        l.wrapping_add(5),
                        l,
                        matches,
                    );
                    has_found_match = true;
                    if l.wrapping_add(5) < max_length {
                        let s: &[u8] = data.split_at(l.wrapping_add(5)).1;
                        if data[0] == b' '
                            && l.wrapping_add(8) < max_length
                            && s[0] == b' '
                            && s[1] == b'o'
                            && s[2] == b'f'
                            && s[3] == b' '
                        {
                            //eprint!("CLdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                            AddMatch(
                                id.wrapping_add((62usize).wrapping_mul(n)),
                                l.wrapping_add(9),
                                l,
                                matches,
                            );
                            if l.wrapping_add(12) < max_length
                                && s[4] == b't'
                                && s[5] == b'h'
                                && s[6] == b'e'
                                && s[7] == b' '
                            {
                                //eprint!("BQdding match {} {} {} {}\n", w.len(), w.transform(), w.idx(), 666);
                                AddMatch(
                                    id.wrapping_add((73usize).wrapping_mul(n)),
                                    l.wrapping_add(13),
                                    l,
                                    matches,
                                );
                            }
                        }
                    }
                }
            }
        }
        has_found_match
    }
}

#[cfg(test)]
mod test {
    #[allow(unused)]
    fn construct_situation(seed: &[u8], mut output: &mut [u8], limit: usize, matchfor: usize) {
        output[..].clone_from_slice(seed);
        if matchfor >= limit {
            return;
        }
        output[matchfor] = output[matchfor].wrapping_add((matchfor as u8 % 253u8).wrapping_add(1));
    }
    #[test]
    fn test_find_match_length() {
        let mut a = [91u8; 600000];
        let mut b = [0u8; 600000];
        for i in 1..a.len() {
            a[i] = (a[i - 1] % 19u8).wrapping_add(17);
        }
        construct_situation(&a[..], &mut b[..], a.len(), 0);
        assert_eq!(super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()), 0);
        construct_situation(&a[..], &mut b[..], a.len(), 1);
        assert_eq!(super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()), 1);
        construct_situation(&a[..], &mut b[..], a.len(), 10);
        assert_eq!(super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()), 10);
        construct_situation(&a[..], &mut b[..], a.len(), 9);
        assert_eq!(super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()), 9);
        construct_situation(&a[..], &mut b[..], a.len(), 7);
        assert_eq!(super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()), 7);
        construct_situation(&a[..], &mut b[..], a.len(), 8);
        assert_eq!(super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()), 8);
        construct_situation(&a[..], &mut b[..], a.len(), 48);
        assert_eq!(super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()), 48);
        construct_situation(&a[..], &mut b[..], a.len(), 49);
        assert_eq!(super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()), 49);
        construct_situation(&a[..], &mut b[..], a.len(), 63);
        assert_eq!(super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()), 63);
        construct_situation(&a[..], &mut b[..], a.len(), 222);
        assert_eq!(
            super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()),
            222
        );
        construct_situation(&a[..], &mut b[..], a.len(), 1590);
        assert_eq!(
            super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()),
            1590
        );
        construct_situation(&a[..], &mut b[..], a.len(), 12590);
        assert_eq!(
            super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()),
            12590
        );
        construct_situation(&a[..], &mut b[..], a.len(), 52592);
        assert_eq!(
            super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()),
            52592
        );
        construct_situation(&a[..], &mut b[..], a.len(), 152592);
        assert_eq!(
            super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()),
            152592
        );
        construct_situation(&a[..], &mut b[..], a.len(), 252591);
        assert_eq!(
            super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()),
            252591
        );
        construct_situation(&a[..], &mut b[..], a.len(), 131072);
        assert_eq!(
            super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()),
            131072
        );
        construct_situation(&a[..], &mut b[..], a.len(), 131073);
        assert_eq!(
            super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()),
            131073
        );
        construct_situation(&a[..], &mut b[..], a.len(), 131072 + 64 + 32 + 16 + 8);
        assert_eq!(
            super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()),
            131072 + 64 + 32 + 16 + 8
        );
        construct_situation(&a[..], &mut b[..], a.len(), 272144 + 64 + 32 + 16 + 8 + 1);
        assert_eq!(
            super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()),
            272144 + 64 + 32 + 16 + 8 + 1
        );
        construct_situation(&a[..], &mut b[..], a.len(), 2 * 272144 + 64 + 32 + 16 + 8);
        assert_eq!(
            super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()),
            2 * 272144 + 64 + 32 + 16 + 8
        );
        construct_situation(&a[..], &mut b[..], a.len(), a.len());
        assert_eq!(
            super::FindMatchLengthWithLimit(&a[..], &b[..], a.len()),
            a.len()
        );
    }
}
