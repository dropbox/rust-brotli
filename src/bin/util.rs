
use brotli::dictionary::{kBrotliDictionary, kBrotliDictionarySizeBitsByLength,
                         kBrotliDictionaryOffsetsByLength};
use brotli::transform::{TransformDictionaryWord};
use brotli::interface;
use std::collections::BTreeMap;
use std::fmt;
use alloc_no_stdlib::SliceWrapper;

struct HexSlice<'a>(&'a [u8]);

impl<'a> fmt::Display for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0 {
            try!(write!(f, "{:02X}", byte));
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
                let final_size = TransformDictionaryWord(&mut transformed[..],
                                        word,
                                        wordlen as i32,
                                        transform as i32) as usize;
                let vec : Vec<u8> = transformed[..final_size].to_vec();
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
macro_rules! println_stderr(
    ($($val:tt)*) => { {
        writeln!(&mut ::std::io::stderr(), $($val)*).unwrap();
    } }
);

fn prediction_mode_str(prediction_mode_nibble:interface::LiteralPredictionModeNibble) -> &'static str {
   match prediction_mode_nibble.prediction_mode() {
         interface::LITERAL_PREDICTION_MODE_SIGN => "sign",
         interface::LITERAL_PREDICTION_MODE_LSB6 => "lsb6",
         interface::LITERAL_PREDICTION_MODE_MSB6 => "msb6",
         interface::LITERAL_PREDICTION_MODE_UTF8 => "utf8",
         _ => "unknown",
   }
}

struct SliceU8Ref<'a>(pub &'a[u8]);

impl<'a> fmt::LowerHex for SliceU8Ref<'a> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for item in self.0 {
            try!( fmtr.write_fmt(format_args!("{:02x}", item)));
        }
        Ok(())
    }
}

pub fn write_one<T:SliceWrapper<u8>>(cmd: &interface::Command<T>) {
    use std::io::Write;
    match cmd {
        &interface::Command::BlockSwitchLiteral(ref bsl) => {
            println_stderr!("ltype {} {}", bsl.0.block_type(), bsl.1);
        },
        &interface::Command::BlockSwitchCommand(ref bsc) => {
            println_stderr!("ctype {}", bsc.0);
        },
        &interface::Command::BlockSwitchDistance(ref bsd) => {
            println_stderr!("dtype {}", bsd.0);
        },
        &interface::Command::PredictionMode(ref prediction) => {
            println_stderr!("prediction {} lcontextmap{} dcontextmap{}",
                            prediction_mode_str(prediction.literal_prediction_mode),
                            prediction.literal_context_map.slice().iter().fold(::std::string::String::new(),
                                                                               |res, &val| res + " " + &val.to_string()),
                            prediction.distance_context_map.slice().iter().fold(::std::string::String::new(),
                                                                                |res, &val| res + " " + &val.to_string()));
        },
        &interface::Command::Copy(ref copy) => {
            println_stderr!("copy {} from {}", copy.num_bytes, copy.distance);
        },
        &interface::Command::Dict(ref dict) => {
            let mut transformed_word = [0u8;38];
            let word_index = dict.word_id as usize * dict.word_size as usize +
                kBrotliDictionaryOffsetsByLength[dict.word_size as usize] as usize;
            let raw_word = &kBrotliDictionary[word_index..(word_index + dict.word_size as usize)];
            let actual_copy_len = TransformDictionaryWord(&mut transformed_word[..],
                                                          raw_word,
                                                          dict.word_size as i32,
                                                          dict.transform as i32) as usize;
            
            transformed_word.split_at(actual_copy_len).0;
            assert_eq!(dict.final_size as usize, actual_copy_len);
            println_stderr!("dict {} word {},{} {:x} func {} {:x}",
                            actual_copy_len,
                            dict.word_size,
                            dict.word_id,
                            SliceU8Ref(raw_word),
                            dict.transform,
                            SliceU8Ref(transformed_word.split_at(actual_copy_len).0));
        },
        &interface::Command::Literal(ref lit) => {
            println_stderr!("{} {} {:x}",
                            if lit.high_entropy {"rndins"} else {"insert"},
                            lit.data.slice().len(),
                            SliceU8Ref(lit.data.slice()));
        },
    }
}
