#![allow(dead_code)]
static kMinUTF8Ratio: super::util::floatX = 0.75 as super::util::floatX;

fn BrotliParseAsUTF8(symbol: &mut i32, input: &[u8], size: usize) -> usize {
  if input[0i32 as (usize)] as (i32) & 0x80i32 == 0i32 {
    *symbol = input[0i32 as (usize)] as (i32);
    if *symbol > 0i32 {
      return 1i32 as (usize);
    }
  }
  if size > 1u32 as (usize) && (input[0i32 as (usize)] as (i32) & 0xe0i32 == 0xc0i32) &&
     (input[1i32 as (usize)] as (i32) & 0xc0i32 == 0x80i32) {
    *symbol = (input[0i32 as (usize)] as (i32) & 0x1fi32) << 6i32 |
              input[1i32 as (usize)] as (i32) & 0x3fi32;
    if *symbol > 0x7fi32 {
      return 2i32 as (usize);
    }
  }
  if size > 2u32 as (usize) && (input[0i32 as (usize)] as (i32) & 0xf0i32 == 0xe0i32) &&
     (input[1i32 as (usize)] as (i32) & 0xc0i32 == 0x80i32) &&
     (input[2i32 as (usize)] as (i32) & 0xc0i32 == 0x80i32) {
    *symbol = (input[0i32 as (usize)] as (i32) & 0xfi32) << 12i32 |
              (input[1i32 as (usize)] as (i32) & 0x3fi32) << 6i32 |
              input[2i32 as (usize)] as (i32) & 0x3fi32;
    if *symbol > 0x7ffi32 {
      return 3i32 as (usize);
    }
  }
  if size > 3u32 as (usize) && (input[0i32 as (usize)] as (i32) & 0xf8i32 == 0xf0i32) &&
     (input[1i32 as (usize)] as (i32) & 0xc0i32 == 0x80i32) &&
     (input[2i32 as (usize)] as (i32) & 0xc0i32 == 0x80i32) &&
     (input[3i32 as (usize)] as (i32) & 0xc0i32 == 0x80i32) {
    *symbol = (input[0i32 as (usize)] as (i32) & 0x7i32) << 18i32 |
              (input[1i32 as (usize)] as (i32) & 0x3fi32) << 12i32 |
              (input[2i32 as (usize)] as (i32) & 0x3fi32) << 6i32 |
              input[3i32 as (usize)] as (i32) & 0x3fi32;
    if *symbol > 0xffffi32 && (*symbol <= 0x10ffffi32) {
      return 4i32 as (usize);
    }
  }
  *symbol = 0x110000i32 | input[0i32 as (usize)] as (i32);
  1i32 as (usize)
}

pub fn BrotliIsMostlyUTF8(data: &[u8],
                          pos: usize,
                          mask: usize,
                          length: usize,
                          min_fraction: super::util::floatX)
                          -> i32 {
  let mut size_utf8: usize = 0usize;
  let mut i: usize = 0usize;
  while i < length {
    let mut symbol: i32 = 0;
    let bytes_read: usize = BrotliParseAsUTF8(&mut symbol,
                                              &data[((pos.wrapping_add(i) & mask) as (usize))..],
                                              length.wrapping_sub(i));
    i = i.wrapping_add(bytes_read);
    if symbol < 0x110000i32 {
      size_utf8 = size_utf8.wrapping_add(bytes_read);
    }
  }
  if !!(size_utf8 as (super::util::floatX) > min_fraction * length as (super::util::floatX)) {
    1i32
  } else {
    0i32
  }
}
