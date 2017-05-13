
#[derive(Clone)]
pub struct Mem256f([super::util::floatX;8]);

impl Default for Mem256f {
    fn default() -> Mem256f {
        Mem256f([0.0 as super::util::floatX; 8])
    }
}

pub struct v128i {
    pub x3: i32,
    pub x2: i32,
    pub x1: i32,
    pub x0: i32,
}

pub struct v256i {
    pub hi: v128i,
    pub lo: v128i,    
}

pub struct v128 {
    pub x3: super::util::floatX,
    pub x2: super::util::floatX,
    pub x1: super::util::floatX,
    pub x0: super::util::floatX,
}

pub struct v256 {
    pub hi: v128,
    pub lo: v128,    
}

macro_rules! vind{
    (($inp:expr)[0]) => (
        $inp.x0
    );
    (($inp:expr)[1]) => (
        $inp.x1
    );
    (($inp:expr)[2]) => (
        $inp.x2
    );
    (($inp:expr)[3]) => (
        $inp.x3
    );
    (($inp:expr)[4]) => (
        $inp.x4
    );
    (($inp:expr)[5]) => (
        $inp.x5
    );
    (($inp:expr)[6]) => (
        $inp.x6
    );
    (($inp:expr)[7]) => (
        $inp.x7
    );
}
use core::ops::Add;
macro_rules! apply128i {
    ($a: expr, $b : expr, $fun : tt) => (
        v128i{x3:$fun($a.x3, $b.x3),
              x2:$fun($a.x2, $b.x2),
              x1:$fun($a.x1, $b.x1),
              x0:$fun($a.x0, $b.x0),
        }
    );
}
macro_rules! apply256i {
    ($a: expr, $b : expr, $fun : tt) => (
        v256i{
            hi:apply128i!($a.hi, $b.hi, $fun),
            lo:apply128i!($a.lo, $b.lo, $fun),
        }
    );
}
macro_rules! op128i {
    ($a: expr, $b : expr, $fun : tt) => (
        v128i{x3:$a.x3.$fun($b.x3),
              x2:$a.x2.$fun($b.x2),
              x1:$a.x1.$fun($b.x1),
              x0:$a.x0.$fun($b.x0),
        }
    );
}
macro_rules! op256i {
    ($a: expr, $b : expr, $fun : tt) => (
        v256i{
            hi:op128i!($a.hi, $b.hi),
            lo:op128i!($a.lo, $b.lo),
        }
    );
}
macro_rules! bcast256i {
    ($inp: expr) => {
        v256i{lo:v128i{x3:$inp,
                       x2:$inp,
                       x1:$inp,
                       x0:$inp,
        },
              hi:v128i{
                  x3:$inp,
                  x2:$inp,
                  x1:$inp,
                  x0:$inp,
              },
        }
    };
}
macro_rules! bcast128i {
    ($inp: expr) => {
        v128i{x3:$inp,
              x2:$inp,
              x1:$inp,
              x0:$inp,
        }
    };
}
macro_rules! shuf128i {
    ($inp: expr, $i3 :tt, $i2 : tt, $i1 : tt, $i0: tt) => {
        v128i{x3:vind!(($inp)[$i3]),
              x2:vind!(($inp)[$i2]),
              x1:vind!(($inp)[$i1]),
              x0:vind!(($inp)[$i0]),
        }
    }
}

macro_rules! apply128 {
    ($a: expr, $b : expr, $fun : expr) => (
        v128{x3:$fun($a.x3, $b.x3),
              x2:$fun($a.x2, $b.x2),
              x1:$fun($a.x1, $b.x1),
              x0:$fun($a.x0, $b.x0),
        }
    );
}
macro_rules! apply256 {
    ($a: expr, $b : expr, $fun : expr) => (
        v256{
            hi:apply128!($a.hi, $b.hi, $fun),
            lo:apply128!($a.lo, $b.lo, $fun),
        }
    );
}
macro_rules! op128 {
    ($a: expr, $b : expr, $fun : expr) => (
        v128{x3:$a.x3.$fun($b.x3),
              x2:$a.x2.$fun($b.x2),
              x1:$a.x1.$fun($b.x1),
              x0:$a.x0.$fun($b.x0),
        }
    );
}
macro_rules! op256 {
    ($a: expr, $b : expr, $fun : expr) => (
        v256{
            hi:op128!($a.hi, $b.hi),
            lo:op128!($a.lo, $b.lo),
        }
    );
}
macro_rules! bcast256 {
    ($inp: expr) => {
        v256{lo:v128{x3:$inp,
                       x2:$inp,
                       x1:$inp,
                       x0:$inp,
        },
              hi:v128{
                  x3:$inp,
                  x2:$inp,
                  x1:$inp,
                  x0:$inp,
              },
        }
    };
}
macro_rules! bcast128 {
    ($inp: expr) => {
        v128{x3:$inp,
              x2:$inp,
              x1:$inp,
              x0:$inp,
        }
    };
}
macro_rules! shuf128 {
    ($inp: expr, $i3 :tt, $i2 : tt, $i1 : tt, $i0: tt) => {
        v128{x3:vind!(($inp)[$i3]),
              x2:vind!(($inp)[$i2]),
              x1:vind!(($inp)[$i1]),
              x0:vind!(($inp)[$i0]),
        }
    }
}

fn addf(a: super::util::floatX, b:super::util::floatX) -> super::util::floatX{
    a + b
}
fn addi(a: i32, b:i32) -> i32{
    a + b
}

fn sum8(x : v256i) -> i32 {
    // hiQuad = ( x7, x6, x5, x4 )
    let hiQuad = x.hi;
    // loQuad = ( x3, x2, x1, x0 )
    let loQuad = x.lo;
    let sumQuad = apply128i!(hiQuad, loQuad, addi);
    let shuf = shuf128i!(sumQuad, 1,0,3,2);
    let sumPair = apply128i!(sumQuad, shuf, addi);
    let sum23 = shuf128i!(sumPair, 1,0,3,2);
    let finalSum = apply128i!(sum23, sumPair, addi);
    finalSum.x0
}

