
#[derive(Clone, Debug)]
pub struct Mem256f(pub [super::util::floatX;8]);

impl Default for Mem256f {
    fn default() -> Mem256f {
        Mem256f([0.0 as super::util::floatX; 8])
    }
}

impl Mem256f {
    pub fn new(data: v256) -> Mem256f {
        Mem256f([data.hi.x3,
                 data.hi.x2,
                 data.hi.x1,
                 data.hi.x0,
                 data.lo.x3,
                 data.lo.x2,
                 data.lo.x1,
                 data.lo.x0])
    }
}









#[derive(Clone, Debug, Copy)]
pub struct Mem256i(pub [i32;8]);

impl Default for Mem256i {
    fn default() -> Mem256i {
        Mem256i([0; 8])
    }
}

impl Mem256i {
    pub fn new(data: v256i) -> Mem256i {
        Mem256i([data.hi.x3,
                 data.hi.x2,
                 data.hi.x1,
                 data.hi.x0,
                 data.lo.x3,
                 data.lo.x2,
                 data.lo.x1,
                 data.lo.x0])
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
impl v256i {
    pub fn new(data: &Mem256i) -> v256i {
        v256i{hi:v128i{x3:data.0[0],
                            x2:data.0[1],
                            x1:data.0[2],
                            x0:data.0[3]},
                    lo:v128i{x3:data.0[4],
                            x2:data.0[5],
                            x1:data.0[6],
                            x0:data.0[7]}}
    }
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
impl v256 {
    pub fn new(data: &Mem256f) -> v256 {
        v256{hi:v128{x3:data.0[0],
                            x2:data.0[1],
                            x1:data.0[2],
                            x0:data.0[3]},
                    lo:v128{x3:data.0[4],
                            x2:data.0[5],
                            x1:data.0[6],
                            x0:data.0[7]}}
    }
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

macro_rules! add128i {
    ($a: expr, $b : expr) => (
        v128i{x3:$a.x3.wrapping_add($b.x3),
              x2:$a.x2.wrapping_add($b.x2),
              x1:$a.x1.wrapping_add($b.x1),
              x0:$a.x0.wrapping_add($b.x0),
        }
    );
}
macro_rules! sub128i {
    ($a: expr, $b : expr) => (
        v128i{x3:$a.x3.wrapping_sub($b.x3),
              x2:$a.x2.wrapping_sub($b.x2),
              x1:$a.x1.wrapping_sub($b.x1),
              x0:$a.x0.wrapping_sub($b.x0),
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
macro_rules! sub256i {
    ($a: expr, $b : expr) => (
        v256i{
            hi:sub128i!($a.hi, $b.hi),
            lo:sub128i!($a.lo, $b.lo),
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
    ($a: expr, $b : expr) => (
        v256{
            hi:add128!($a.hi, $b.hi),
            lo:add128!($a.lo, $b.lo),
        }
    );
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
    ($a: expr, $b : expr) => (
        v256{
            hi:add128!($a.hi, $b.hi),
            lo:add128!($a.lo, $b.lo),
        }
    );
}

macro_rules! and128i {
    ($a: expr, $b : expr) => (
        v128i{x3:$a.x3 & $b.x3,
              x2:$a.x2 & $b.x2,
              x1:$a.x1 & $b.x1,
              x0:$a.x0 & $b.x0,
        }
    );
}
macro_rules! and256i {
    ($a: expr, $b : expr) => (
        v256i{
            hi:and128i!($a.hi, $b.hi),
            lo:and128i!($a.lo, $b.lo),
        }
    );
}

macro_rules! sub128 {
    ($a: expr, $b : expr) => (
        v128{x3:$a.x3 - $b.x3,
              x2:$a.x2 - $b.x2,
              x1:$a.x1 - $b.x1,
              x0:$a.x0 - $b.x0,
        }
    );
}
macro_rules! sub256 {
    ($a: expr, $b : expr) => (
        v256{
            hi:sub128!($a.hi, $b.hi),
            lo:sub128!($a.lo, $b.lo),
        }
    );
}


macro_rules! max128 { // should be able to use _mm_min_ps
($a: expr, $b : expr) => (
    // if src1 == 0 and src2 == 0 return src2 else if src1 or src2 is nan, return src2,
    // if src1 > src2 return src1 else return src2
    v128{x3:if $a.x3 > $b.x3 && !($a.x3 == 0.0 as super::util::floatX && $b.x3 == 0.0 as super::util::floatX) {$a.x3} else {$b.x3},
             x2:if $a.x2 > $b.x2 && !($a.x2 == 0.0 as super::util::floatX && $b.x2 == 0.0 as super::util::floatX) {$a.x2} else {$b.x2},
             x1:if $a.x1 > $b.x1 && !($a.x1 == 0.0 as super::util::floatX && $b.x1 == 0.0 as super::util::floatX) {$a.x1} else {$b.x1},
             x0:if $a.x0 > $b.x0 && !($a.x0 == 0.0 as super::util::floatX && $b.x0 == 0.0 as super::util::floatX) {$a.x0} else {$b.x0},
        }
    );
}
macro_rules! max256 {
    ($a: expr, $b : expr) => (
        v256{
            hi:max128!($a.hi, $b.hi),
            lo:max128!($a.lo, $b.lo),
        }
    );
}

macro_rules! miny128 { // should be able to use _mm_min_ps
($a: expr, $b : expr) => (
        // if src1 == 0 and src2 == 0 return src2 else if src1 or src2 is nan, return src2,
        // if src1 > src2 return src1 else return src2
        v128{x3:if $a.x3 < $b.x3 && !($a.x3 == 0.0 as super::util::floatX && $b.x3 == 0.0 as super::util::floatX) {$a.x3} else {$b.x3},
             x2:if $a.x2 < $b.x2 && !($a.x2 == 0.0 as super::util::floatX && $b.x2 == 0.0 as super::util::floatX) {$a.x2} else {$b.x2},
             x1:if $a.x1 < $b.x1 && !($a.x1 == 0.0 as super::util::floatX && $b.x1 == 0.0 as super::util::floatX) {$a.x1} else {$b.x1},
             x0:if $a.x0 < $b.x0 && !($a.x0 == 0.0 as super::util::floatX && $b.x0 == 0.0 as super::util::floatX) {$a.x0} else {$b.x0},
        }
    );
}

macro_rules! min128 { // should be able to use _mm_min_ps
($a: expr, $b : expr) => (
        // if src1 == 0 and src2 == 0 return src2 else if src1 or src2 is nan, return src2,
        // if src1 > src2 return src1 else return src2
        v128{x3:if $a.x3 < $b.x3 {$a.x3} else {$b.x3},
             x2:if $a.x2 < $b.x2 {$a.x2} else {$b.x2},
             x1:if $a.x1 < $b.x1 {$a.x1} else {$b.x1},
             x0:if $a.x0 < $b.x0 {$a.x0} else {$b.x0},
        }
    );
}

macro_rules! minx128 { // should be able to use _mm_min_ps
($a: expr, $b : expr) => (
        // if src1 == 0 and src2 == 0 return src2 else if src1 or src2 is nan, return src2,
        // if src1 > src2 return src1 else return src2
    v128{x3:$a.x3.min($b.x3),
         x2:$a.x2.min($b.x2),
         x1:$a.x1.min($b.x1),
         x0:$a.x0.min($b.x0)
        }
    );
}
macro_rules! min256 {
    ($a: expr, $b : expr) => (
        v256{
            hi:min128!($a.hi, $b.hi),
            lo:min128!($a.lo, $b.lo),
        }
    );
}

macro_rules! sub256 {
    ($a: expr, $b : expr) => (
        v256{
            hi:sub128!($a.hi, $b.hi),
            lo:sub128!($a.lo, $b.lo),
        }
    );
}
macro_rules! cmpge128 {
    ($a: expr, $b : expr) => (
        v128i{x3:if $a.x3 >= $b.x3 {-1} else {0},
              x2:if $a.x2 >= $b.x2 {-1} else {0},
              x1:if $a.x1 >= $b.x1 {-1} else {0},
              x0:if $a.x0 >= $b.x0 {-1} else {0},
        }
    );
}
macro_rules! cmpge256 {
    ($a: expr, $b : expr) => (
        v256i{
            hi:cmpge128!($a.hi, $b.hi),
            lo:cmpge128!($a.lo, $b.lo),
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


pub fn sum8(x : v256i) -> i32 {
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

