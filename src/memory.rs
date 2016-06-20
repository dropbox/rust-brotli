//use core::slice;

#[cfg(not(feature="unsafe"))]
macro_rules! fast {
   ($slice : expr,[$index: expr]) => (
       (&$slice)[$index]
   );
   ($slice : expr,[$start: expr ; $end : expr]) => (
       &($slice)[$start .. $end]
   );
   ($slice : expr,[$start: expr ;]) => (
       &($slice)[$start .. ]
   );
   ($slice : expr,[; $end]) => (
       &($slice)[.. $end ]
   );
}
#[cfg(not(feature="unsafe"))]
macro_rules! fast_uninitialized {
    ($size : expr ) => ([0; $size]);
}

#[cfg(not(feature="unsafe"))]
macro_rules! fast_mut {
   ($slice : expr,[$index: expr]) => (
       *&mut($slice)[$index]       
   );
   ($slice : expr,[$start: expr ; $end : expr]) => (
       &mut $slice[$start..$end]
   );
   ($slice : expr,[$start: expr ;]) => (
       &mut $slice[$start..]       
   );
   ($slice : expr,[; $end]) => (
       &mut $slice[..$end]              
   );
}

#[cfg(feature="unsafe")]
macro_rules! fast {
   ($slice : expr,[$index: expr]) => (
       unsafe{$slice.get_unchecked($index)}
   );
   ($slice : expr,[$start: expr ; $end : expr]) => (
       unsafe{::core::slice::from_raw_parts(($item).as_ptr().offset($start as isize), $end - $start)};
   );
   ($slice : expr,[$start: expr ;]) => (
       unsafe{::core::slice::from_raw_parts(($item).as_ptr().offset($start as isize), $item.len() - $start)};
   );
   ($slice : expr,[; $end]) => (
       unsafe{::core::slice::from_raw_parts(($item).as_ptr(), $item.len())};
   );
}

#[cfg(feature="unsafe")]
macro_rules! fast_mut {
   ($slice : expr,[$index: expr]) => (
       unsafe{$slice.get_unchecked_mut($index)}
   );
   ($slice : expr,[$start: expr ; $end : expr]) => (
       unsafe{::core::slice::from_raw_parts_mut(($item).as_ptr().offset($start as isize), $end - $start)};
   );
   ($slice : expr,[$start: expr ;]) => (
       unsafe{::core::slice::from_raw_parts_mut(($item).as_ptr().offset($start as isize), $item.len() - $start)};
   );
   ($slice : expr,[; $end]) => (
       unsafe{::core::slice::from_raw_parts_mut(($item).as_ptr(), $item.len())};
   );
}
#[cfg(feature="unsafe")]
macro_rules! fast_uninitialized {
    ($size : expr ) => mem::uninitialized();
}

/*
pub fn indexk<T>(item : &[T], index : usize) -> &T {
//   return &item[index];
   return unsafe{item.get_unchecked(index)};
}

pub fn indexm<T>(item : &mut [T], index : usize) -> &mut T {
// return &mut item[index]
   return unsafe{item.get_unchecked_mut(index)};
}


pub fn slicek<T>(item : &[T], start : usize, end :usize) -> &[T] {
   return unsafe{slice::from_raw_parts(item.as_ptr().offset(start as isize), end - start)};
}

pub fn slicem<T>(item : &mut [T], start : usize, end :usize) -> &mut [T] {
   return unsafe{slice::from_raw_parts_mut(item.as_mut_ptr().offset(start as isize), end - start)};
}

pub fn slicemend<T>(item : &mut [T], start : usize) -> &mut [T] {
   return unsafe{slice::from_raw_parts_mut(item.as_mut_ptr().offset(start as isize), item.len() - start)};
}
*/
