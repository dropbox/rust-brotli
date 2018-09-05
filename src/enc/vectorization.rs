#![allow(unknown_lints)]
#![allow(unused_macros)]
use core;
use enc::{v8, s8};
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
    v256::new((x.extract(0) as f32).log2(),
              (x.extract(1) as f32).log2(),
              (x.extract(2) as f32).log2(),
              (x.extract(3) as f32).log2(),
              (x.extract(4) as f32).log2(),
              (x.extract(5) as f32).log2(),
              (x.extract(6) as f32).log2(),
              (x.extract(7) as f32).log2())
}
pub fn log2(x: v256) -> v256 {
    v256::new(x.extract(0).log2(),
              x.extract(1).log2(),
              x.extract(2).log2(),
              x.extract(3).log2(),
              x.extract(4).log2(),
              x.extract(5).log2(),
              x.extract(6).log2(),
              x.extract(7).log2())
}
pub fn cast_f32_to_i32(x: v256i) -> v256 {
    v256::new(x.extract(0) as f32,
              x.extract(1) as f32,
              x.extract(2) as f32,
              x.extract(3) as f32,
              x.extract(4) as f32,
              x.extract(5) as f32,
              x.extract(6) as f32,
              x.extract(7) as f32)
}
