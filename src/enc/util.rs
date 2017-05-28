#![allow(dead_code)]
use super::fast_log::{kLog2Table, kLog64k};
#[cfg(feature="float64")]
pub type floatX = f64;

#[cfg(not(feature="float64"))]
pub type floatX = f32;

pub fn brotli_max_uint32_t(a: u32, b: u32) -> u32 {
  if a > b { a } else { b }
}
pub fn brotli_min_uint32_t(a: u32, b: u32) -> u32 {
  if a > b { b } else { a }
}

pub fn brotli_min_size_t(a: usize, b: usize) -> usize {
  if a > b { b } else { a }
}
pub fn brotli_max_size_t(a: usize, b: usize) -> usize {
  if a > b { a } else { b }
}
pub fn brotli_max_uint8_t(a: u8, b: u8) -> u8 {
  (if a as (i32) > b as (i32) {
     a as (i32)
   } else {
     b as (i32)
   }) as (u8)
}




#[inline(always)]
pub fn FastLog2u16(v: u16) -> floatX {
    kLog64k[v as usize] as floatX
}

#[cfg(not(feature="no-stdlib"))]
#[inline(always)]
pub fn FastLog2(v: u64) -> floatX {
    if v < 256 {
        return kLog2Table[v as usize] as floatX;
    }
    (v as f32).log2() as floatX
}

#[cfg(feature="no-stdlib")]
#[inline(always)]
pub fn FastLog2(v: u64) -> floatX {
  if v < 256 {
    return kLog2Table[v as usize] as floatX;
  }
  FastLog2u64(v)
}

/*
#[cfg(not(feature="no-stdlib"))]
#[inline(always)]
pub fn FastLog2Minus2If0(v: u64) -> floatX {
    if v < 256 {
        return kBiasedLog2Table[v as usize] as floatX;
    }
    (v as f32).log2() as floatX
}

#[cfg(feature="no-stdlib")]
#[inline(always)]
pub fn FastLog2Minus2If0(v: u64) -> floatX {
  if v < 256 {
    return kBiasedLog2Table[v as usize] as floatX;
  }
  FastLog2u64(v)
}
*/


#[inline]
pub fn FastLog2u64(v: u64) -> floatX {
  let bsr_8 = 56i8 - v.leading_zeros() as i8;
  let offset = bsr_8 & -((bsr_8 >= 0) as i8);
  offset as floatX + kLog2Table[(v >> offset) as u8 as usize] as (floatX)
}

#[inline(always)]
pub fn FastLog2u32(v: i32) -> floatX {
  let bsr_8 = 24i8 - v.leading_zeros() as i8;
  let offset = bsr_8 & -((bsr_8 >= 0) as i8);
  offset as floatX + kLog2Table[(v >> offset) as u8 as usize] as (floatX)
}

#[inline(always)]
pub fn xFastLog2u16(v: u16) -> floatX {
  let bsr_8 = 8i8 - v.leading_zeros() as i8;
  let offset = (bsr_8 & -((bsr_8 >= 0) as i8));
  offset as floatX + kLog2Table[(v >> offset) as u8 as usize] as (floatX)
}

#[cfg(not(feature="no-stdlib"))]
pub fn FastPow2(v: super::util::floatX) -> super::util::floatX {
  return (2 as super::util::floatX).powf(v);
}


#[cfg(feature="no-stdlib")]
pub fn FastPow2(v: super::util::floatX) -> super::util::floatX {
   assert!(v >= 0 as super::util::floatX);
   let round_down = v as i32;
   let remainder = v - round_down as super::util::floatX;
   let mut x = 1 as super::util::floatX;
   // (1 + (x/n) * ln2) ^ n 
   // let n = 8
   x += remainder * (0.693147180559945309417232121458 / 256.0) as super::util::floatX;
   x *= x;
   x *= x;
   x *= x;
   x *= x;
   x *= x;
   x *= x;
   x *= x;
   x *= x;
   return (1 << round_down) as super::util::floatX * x;
}

pub fn Log2FloorNonZero(v: u64) -> u32 {
  (63u32 ^ v.leading_zeros()) as u32
}
mod test {
  fn baseline_log2_floor_non_zero(mut n:u64) -> u32 {
    let mut result: u32 = 0u32;
    while {
        n = n >> 1i32;
        n
    } != 0 {
      result = result.wrapping_add(1 as (u32));
    }
    result
  }

  #[test]
  fn log2floor_non_zero_works(){
    let examples = [4u64, 254u64, 256u64,
                  1428u64, 25412509u64, 21350891256u64,
                  65536u64, 1258912591u64, 60968101u64,
                  1u64, 12589125190825u64, 105912059215091u64,
                  0u64];
    for example in examples.iter() {
      let fast_version = super::Log2FloorNonZero(*example);
      let baseline_version = baseline_log2_floor_non_zero(*example);
      if *example != 0 { // make sure we don't panic when computing...but don't care about result
        assert_eq!(fast_version,
                 baseline_version);
      }
    }
  }
pub fn approx_eq(a : f64, b: f64, tol: f64) {
    let mut t0 = a - b;
    let mut t1 = b - a;
    if t0 < 0.0 {
       t0 = -t0;
    }
    if t1 < 0.0 {
       t1 = -t1;
    }
    if (!(t1 < tol)) {
       assert_eq!(a, b);
    }
    if (!(t0 < tol)) {
       assert_eq!(a, b);
    }
}
  #[test]
  fn fast_log2_works(){
    let examples = [4u64, 254u64, 256u64,
                  1428u64, 25412509u64, 21350891256u64,
                  65536u64, 1258912591u64, 60968101u64,
                  1u64, 12589125190825u64, 105912059215091u64,
                  0u64];
    let tol = [0.00001, 0.0001, 0.0001, 0.005, 0.007, 0.008, 0.01, 0.01, 0.01, 0.000001, 0.01, 0.01, 0.0001];
    for (index, example) in examples.iter().enumerate() {
      let fast_version = super::FastLog2(*example);
      if *example != 0 { // make sure we don't panic when computing...but don't care about result
        let baseline_version = (*example as f64).log2();
        approx_eq(fast_version as f64, baseline_version, tol[index]);
      } else {
        //assert_eq!(fast_version as f64, 0.0 as f64);
      }
    }
  }
}

