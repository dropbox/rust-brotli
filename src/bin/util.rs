use brotli::dictionary::{kBrotliDictionary, kBrotliDictionarySizeBitsByLength,
                         kBrotliDictionaryOffsetsByLength};
use brotli::transform::{TransformDictionaryWord};
use std::collections::BTreeMap;
use std::fmt;

struct HexSlice<'a>(&'a [u8]);

impl<'a> fmt::Display for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}
pub fn permute_dictionary() -> BTreeMap<Vec<u8>, ()> {
    let mut ret = BTreeMap::<Vec<u8>, ()>::new();
    let mut transformed = [0u8;38];
    for wordlen in 4..kBrotliDictionaryOffsetsByLength.len() {
        let offset = kBrotliDictionaryOffsetsByLength[wordlen] as usize;
        for index in 0..(1 << kBrotliDictionarySizeBitsByLength[wordlen]) {
            let word = &kBrotliDictionary[offset + index..offset + index + wordlen];
            for transform in 0..121 {
                TransformDictionaryWord(&mut transformed[..],
                                        word,
                                        wordlen as i32,
                                        transform as i32);
                let vec : Vec<u8> = transformed[..wordlen].to_vec();
                ret.insert(vec, ());
            }
        }
    }
    ret
}

pub fn print_dictionary(dict :BTreeMap<Vec<u8>, ()>) {
    for (key, _) in dict {
        println!("{}", HexSlice(&key[..]));
    }
}
