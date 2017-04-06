#![allow(dead_code)]
use super::util::{Log2FloorNonZero};
pub struct Command {
  pub insert_len_: u32,
  pub copy_len_: u32,
  pub dist_extra_: u32,
  pub cmd_prefix_: u16,
  pub dist_prefix_: u16,
}
pub fn CommandCopyLen(xself: &Command) -> u32 {
  (*xself).copy_len_ & 0xffffffi32 as (u32)
}

pub fn CommandDistanceContext(xself: &Command) -> u32 {
  let r: u32 = ((*xself).cmd_prefix_ as (i32) >> 6i32) as (u32);
  let c: u32 = ((*xself).cmd_prefix_ as (i32) & 7i32) as (u32);
  if (r == 0i32 as (u32) || r == 2i32 as (u32) || r == 4i32 as (u32) || r == 7i32 as (u32)) &&
     (c <= 2i32 as (u32)) {
    c
  } else {
    3i32 as (u32)
  }
}

pub fn ComputeDistanceCode(distance: usize,
                       max_distance: usize,
                       dist_cache: &[i32])
                       -> usize {
  if distance <= max_distance {
    let distance_plus_3: usize = distance.wrapping_add(3usize);
    let offset0: usize = distance_plus_3.wrapping_sub(dist_cache[(0usize)] as (usize));
    let offset1: usize = distance_plus_3.wrapping_sub(dist_cache[(1usize)] as (usize));
    if distance == dist_cache[(0usize)] as (usize) {
      return 0usize;
    } else if distance == dist_cache[(1usize)] as (usize) {
      return 1usize;
    } else if offset0 < 7usize {
      return (0x9750468i32 >> (4usize).wrapping_mul(offset0) & 0xfi32) as (usize);
    } else if offset1 < 7usize {
      return (0xfdb1acei32 >> (4usize).wrapping_mul(offset1) & 0xfi32) as (usize);
    } else if distance == dist_cache[(2usize)] as (usize) {
      return 2usize;
    } else if distance == dist_cache[(3usize)] as (usize) {
      return 3usize;
    }
  }
  distance.wrapping_add(16usize).wrapping_sub(1usize)
}


fn GetInsertLengthCode(insertlen: usize) -> u16 {
  if insertlen < 6usize {
    insertlen as (u16)
  } else if insertlen < 130usize {
    let nbits: u32 = Log2FloorNonZero(insertlen.wrapping_sub(2usize)).wrapping_sub(1u32);
    ((nbits << 1i32) as (usize))
      .wrapping_add(insertlen.wrapping_sub(2usize) >> nbits)
      .wrapping_add(2usize) as (u16)
  } else if insertlen < 2114usize {
    Log2FloorNonZero(insertlen.wrapping_sub(66usize)).wrapping_add(10u32) as (u16)
  } else if insertlen < 6210usize {
    21u32 as (u16)
  } else if insertlen < 22594usize {
    22u32 as (u16)
  } else {
    23u32 as (u16)
  }
}

fn GetCopyLengthCode(copylen: usize) -> u16 {
  if copylen < 10usize {
    copylen.wrapping_sub(2usize) as (u16)
  } else if copylen < 134usize {
    let nbits: u32 = Log2FloorNonZero(copylen.wrapping_sub(6usize)).wrapping_sub(1u32);
    ((nbits << 1i32) as (usize))
      .wrapping_add(copylen.wrapping_sub(6usize) >> nbits)
      .wrapping_add(4usize) as (u16)
  } else if copylen < 2118usize {
    Log2FloorNonZero(copylen.wrapping_sub(70usize)).wrapping_add(12u32) as (u16)
  } else {
    23u32 as (u16)
  }
}

fn CombineLengthCodes(inscode: u16, copycode: u16, use_last_distance: i32) -> u16 {
  let bits64: u16 = (copycode as (u32) & 0x7u32 | (inscode as (u32) & 0x7u32) << 3i32) as (u16);
  if use_last_distance != 0 && (inscode as (i32) < 8i32) && (copycode as (i32) < 16i32) {
    if copycode as (i32) < 8i32 {
      bits64
    } else {
      let s64: u16 = 64i32 as (u16);
      (bits64 as (i32) | s64 as (i32)) as (u16)
    }
  } else {
    let mut offset: i32 = 2i32 * ((copycode as (i32) >> 3i32) + 3i32 * (inscode as (i32) >> 3i32));
    offset = (offset << 5i32) + 0x40i32 + (0x520d40i32 >> offset & 0xc0i32);
    (offset as (u16) as (i32) | bits64 as (i32)) as (u16)
  }
}

fn GetLengthCode(insertlen: usize,
                 copylen: usize,
                 use_last_distance: i32,
                 mut code: &mut u16) {
  let inscode: u16 = GetInsertLengthCode(insertlen);
  let copycode: u16 = GetCopyLengthCode(copylen);
  *code = CombineLengthCodes(inscode, copycode, use_last_distance);
}
fn PrefixEncodeCopyDistance(distance_code: usize,
                            num_direct_codes: usize,
                            postfix_bits: usize,
                            mut code: &mut u16,
                            mut extra_bits: &mut u32) {
  if distance_code < (16usize).wrapping_add(num_direct_codes) {
    *code = distance_code as (u16);
    *extra_bits = 0u32;
  } else {
    let dist: usize =
      (1usize << postfix_bits.wrapping_add(2u32 as (usize)))
        .wrapping_add(distance_code.wrapping_sub(16usize).wrapping_sub(num_direct_codes));
    let bucket: usize = Log2FloorNonZero(dist).wrapping_sub(1u32) as (usize);
    let postfix_mask: usize = (1u32 << postfix_bits).wrapping_sub(1u32) as (usize);
    let postfix: usize = dist & postfix_mask;
    let prefix: usize = dist >> bucket & 1usize;
    let offset: usize = (2usize).wrapping_add(prefix) << bucket;
    let nbits: usize = bucket.wrapping_sub(postfix_bits);
    *code = (16usize)
      .wrapping_add(num_direct_codes)
      .wrapping_add((2usize).wrapping_mul(nbits.wrapping_sub(1usize)).wrapping_add(prefix) <<
                    postfix_bits)
      .wrapping_add(postfix) as (u16);
    *extra_bits = (nbits << 24i32 | dist.wrapping_sub(offset) >> postfix_bits) as (u32);
  }
}

pub fn InitCommand(xself: &mut Command,
                  insertlen: usize,
                  copylen: usize,
                  copylen_code: usize,
                  distance_code: usize) {
    xself.insert_len_ = insertlen as (u32);
    xself.copy_len_ = (copylen | (copylen_code ^ copylen) << 24i32) as (u32);
  PrefixEncodeCopyDistance(distance_code,
                           0usize,
                           0usize,
                           &mut xself.dist_prefix_,
                           &mut xself.dist_extra_);
  GetLengthCode(insertlen,
                copylen_code,
                if !!(xself.dist_prefix_ as (i32) == 0i32) {
                  1i32
                } else {
                  0i32
                },
                &mut xself.cmd_prefix_);
}
pub fn NewCommand(insertlen: usize,
               copylen: usize,
               copylen_code: usize,
               distance_code: usize) -> Command {
  let mut xself : Command = Command {
           insert_len_: insertlen as (u32),
           copy_len_: (copylen | (copylen_code ^ copylen) << 24i32) as (u32),
           dist_extra_:0,
           cmd_prefix_:0,
           dist_prefix_:0,
   };
   InitCommand(&mut xself, insertlen, copylen, copylen_code, distance_code);
   xself
}
