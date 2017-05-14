
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
macro_rules! add128i {
    ($a: expr, $b : expr) => (
        v128i{x3:$a.x3.wrapping_add($b.x3),
              x2:$a.x2.wrapping_add($b.x2),
              x1:$a.x1.wrapping_add($b.x1),
              x0:$a.x0.wrapping_add($b.x0),
        }
    );
}
macro_rules! add256i {
    ($a: expr, $b : expr) => (
        v256i{
            hi:add128i!($a.hi, $b.hi),
            lo:add128i!($a.lo, $b.lo),
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

macro_rules! add128 {
    ($a: expr, $b : expr) => (
        v128{x3:$a.x3 + $b.x3,
              x2:$a.x2 + $b.x2,
              x1:$a.x1 + $b.x1,
              x0:$a.x0 + $b.x0,
        }
    );
}
macro_rules! add256 {
    ($a: expr, $b : expr, $fun : expr) => (
        v256{
            hi:add128!($a.hi, $b.hi),
            lo:add128!($a.lo, $b.lo),
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


fn sum8(x : v256i) -> i32 {
    // hiQuad = ( x7, x6, x5, x4 )
    let hiQuad = x.hi;
    // loQuad = ( x3, x2, x1, x0 )
    let loQuad = x.lo;
    let sumQuad = add128i!(hiQuad, loQuad);
    let shuf = shuf128i!(sumQuad, 1,0,3,2);
    let sumPair = add128i!(sumQuad, shuf);
    let sum23 = shuf128i!(sumPair, 1,0,3,2);
    let finalSum = add128i!(sum23, sumPair);
    finalSum.x0
}

