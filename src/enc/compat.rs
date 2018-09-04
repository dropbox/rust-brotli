#![cfg_attr(feature="simd", allow(unused))]
use core::ops::{Shr, Add, Sub, AddAssign};
#[derive(Default, Copy, Clone, Debug)]
pub struct Compat16x16([i16;16]);
impl Compat16x16 {
    pub fn splat(a: i16) -> Compat16x16 {
        Compat16x16([a, a, a, a, a, a, a, a,
                     a, a, a, a, a, a, a, a,])
    }
    pub fn new(a0: i16, a1: i16, a2: i16, a3: i16,
               a4: i16, a5: i16, a6: i16, a7: i16,
               a8: i16, a9: i16, a10: i16, a11: i16,
               a12: i16, a13: i16, a14:i16, a15:i16) -> Compat16x16 {
        Compat16x16([a0, a1, a2, a3, a4, a5, a6, a7,
                     a8, a9, a10, a11, a12, a13, a14, a15])
    }
    pub fn extract(&self, i: usize) -> i16 {
        self.0[i]
    }
    pub fn replace(&self, i: usize, data: i16) -> Compat16x16 {
        let mut ret = *self;
        ret.0[i] = data;
        ret
    }
}
macro_rules! op16 {
    ($a: expr, $b: expr, $op: expr) => (
        Compat16x16::new($op($a[0], $b[0]),
                         $op($a[1], $b[1]),
                         $op($a[2], $b[2]),
                         $op($a[3], $b[3]),
                         $op($a[4], $b[4]),
                         $op($a[5], $b[5]),
                         $op($a[6], $b[6]),
                         $op($a[7], $b[7]),
                         $op($a[8], $b[8]),
                         $op($a[9], $b[9]),
                         $op($a[10], $b[10]),
                         $op($a[11], $b[11]),
                         $op($a[12], $b[12]),
                         $op($a[13], $b[13]),
                         $op($a[14], $b[14]),
                         $op($a[15], $b[15]))
                         
        )
}
macro_rules! scalar_op16 {
    ($a: expr, $b: expr, $op: expr) => (
        Compat16x16::new($op($a[0], $b),
                         $op($a[1], $b),
                         $op($a[2], $b),
                         $op($a[3], $b),
                         $op($a[4], $b),
                         $op($a[5], $b),
                         $op($a[6], $b),
                         $op($a[7], $b),
                         $op($a[8], $b),
                         $op($a[9], $b),
                         $op($a[10], $b),
                         $op($a[11], $b),
                         $op($a[12], $b),
                         $op($a[13], $b),
                         $op($a[14], $b),
                         $op($a[15], $b))
                         
        )
}
fn wrapping_i16_add(a: i16, b:i16) -> i16 {
    a.wrapping_add(b)
}
fn wrapping_i16_sub(a: i16, b:i16) -> i16 {
    a.wrapping_sub(b)
}
fn shift16<Scalar>(a: i16, b:Scalar) -> i16 where i64:From<Scalar> {
    a >> i64::from(b)
}
impl Add for Compat16x16 {
    type Output = Compat16x16;
    fn add(self, other: Compat16x16) -> Compat16x16 {
        op16!(self.0, other.0, wrapping_i16_add)
    }
}
impl Sub for Compat16x16 {
    type Output = Compat16x16;
    fn sub(self, other: Compat16x16) -> Compat16x16 {
        op16!(self.0, other.0, wrapping_i16_sub)
    }
}
impl<Scalar:Clone> Shr<Scalar> for Compat16x16 where i64:From<Scalar> {
    type Output = Compat16x16;
    fn shr(self, other: Scalar) -> Compat16x16 {
        scalar_op16!(self.0, other.clone(), shift16)
    }
}


#[derive(Default, Copy, Clone, Debug)]
pub struct CompatF8([f32;8]);
impl CompatF8 {
    pub fn new(a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32) -> CompatF8 {
        CompatF8([a0, a1, a2, a3, a4, a5, a6, a7])
    }
    pub fn splat(a: f32) -> CompatF8 {
        CompatF8([a, a, a, a, a, a, a, a])
    }
    pub fn extract(&self, i: usize) -> f32 {
        self.0[i]
    }
    pub fn replace(&self, i: usize, data: f32) -> CompatF8 {
        let mut ret = *self;
        ret.0[i] = data;
        ret
    }
}
impl Add for CompatF8 {
    type Output = CompatF8;
    fn add(self, other: CompatF8) -> CompatF8 {
        CompatF8::new(self.0[0] + other.0[0],
                      self.0[1] + other.0[1],
                      self.0[2] + other.0[2],
                      self.0[3] + other.0[3],
                      self.0[4] + other.0[4],
                      self.0[5] + other.0[5],
                      self.0[6] + other.0[6],
                      self.0[7] + other.0[7])
    }
}
impl AddAssign for CompatF8 {
    fn add_assign(&mut self, other: CompatF8) {
        self.0[0] += other.0[0];
        self.0[1] += other.0[1];
        self.0[2] += other.0[2];
        self.0[3] += other.0[3];
        self.0[4] += other.0[4];
        self.0[5] += other.0[5];
        self.0[6] += other.0[6];
        self.0[7] += other.0[7];
    }
}
