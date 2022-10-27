#![allow(unknown_lints)]
#![allow(unused_macros)]

use enc::{v8, s8};
use enc::util::FastLog2;
pub type Mem256f = v8;
pub type Mem256i = s8;
pub type v256 = v8;
pub type v256i = s8;
pub fn sum8(x: v256) -> f32 {
    x.extract(0) + x.extract(1) + x.extract(2) + x.extract(3) +
        x.extract(4) + x.extract(5) + x.extract(6) + x.extract(7)
}

pub fn sum8i(x: v256i) -> i32 {
    x.extract(0).wrapping_add(x.extract(1)).wrapping_add( x.extract(2)).wrapping_add( x.extract(3)).wrapping_add(
        x.extract(4)).wrapping_add( x.extract(5)).wrapping_add( x.extract(6)).wrapping_add(x.extract(7))
}

pub fn log2i(x: v256i) -> v256 {
    v256::new(FastLog2(x.extract(0) as u64),
              FastLog2(x.extract(1) as u64),
              FastLog2(x.extract(2) as u64),
              FastLog2(x.extract(3) as u64),
              FastLog2(x.extract(4) as u64),
              FastLog2(x.extract(5) as u64),
              FastLog2(x.extract(6) as u64),
              FastLog2(x.extract(7) as u64))
}
pub fn cast_i32_to_f32(x: v256i) -> v256 {
    v256::new(x.extract(0) as f32,
              x.extract(1) as f32,
              x.extract(2) as f32,
              x.extract(3) as f32,
              x.extract(4) as f32,
              x.extract(5) as f32,
              x.extract(6) as f32,
              x.extract(7) as f32)
}
pub fn cast_f32_to_i32(x: v256) -> v256i {
    v256i::new(x.extract(0) as i32,
              x.extract(1) as i32,
              x.extract(2) as i32,
              x.extract(3) as i32,
              x.extract(4) as i32,
              x.extract(5) as i32,
              x.extract(6) as i32,
              x.extract(7) as i32)
}
