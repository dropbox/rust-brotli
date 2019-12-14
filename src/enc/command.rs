
use super::util::Log2FloorNonZero;
use super::encode::BROTLI_NUM_DISTANCE_SHORT_CODES;
#[derive(Copy,Clone, Debug)]
pub struct BrotliDistanceParams {
    pub distance_postfix_bits : u32,
    pub num_direct_distance_codes : u32,
    pub alphabet_size : u32,
    pub max_distance : usize,
}

#[derive(Clone, Copy, Debug)]
pub struct Command {
  // stores copy_len in low 25 bits and copy_code - copy_len in high 7 bit
  pub insert_len_: u32,
  pub copy_len_: u32,
  //stores distance_extra bits
  pub dist_extra_: u32,
  pub cmd_prefix_: u16,
  // stores distance code in low 10 bits and num extra bits in high 6 bits
  pub dist_prefix_: u16,
}
impl Default for Command {
  fn default() -> Command {
    Command {
      insert_len_: 0,
      copy_len_: 0,
      dist_extra_: 0,
      cmd_prefix_: 0,
      dist_prefix_: 0,
    }
  }
}
pub fn CommandCopyLen(xself: &Command) -> u32 {
  (*xself).copy_len_ & 0x1ffffffi32 as (u32)
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

#[inline(always)]
pub fn ComputeDistanceCode(distance: usize, max_distance: usize, dist_cache: &[i32]) -> usize {
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

#[inline(always)]
pub fn GetInsertLengthCode(insertlen: usize) -> u16 {
  if insertlen < 6usize {
    insertlen as (u16)
  } else if insertlen < 130usize {
    let nbits: u32 = Log2FloorNonZero(insertlen.wrapping_sub(2) as u64).wrapping_sub(1u32);
    ((nbits << 1i32) as (usize))
      .wrapping_add(insertlen.wrapping_sub(2usize) >> nbits)
      .wrapping_add(2usize) as (u16)
  } else if insertlen < 2114usize {
    Log2FloorNonZero(insertlen.wrapping_sub(66usize) as u64).wrapping_add(10u32) as (u16)
  } else if insertlen < 6210usize {
    21u32 as (u16)
  } else if insertlen < 22594usize {
    22u32 as (u16)
  } else {
    23u32 as (u16)
  }
}

#[inline(always)]
pub fn GetCopyLengthCode(copylen: usize) -> u16 {
  if copylen < 10usize {
    copylen.wrapping_sub(2usize) as (u16)
  } else if copylen < 134usize {
    let nbits: u32 = Log2FloorNonZero(copylen.wrapping_sub(6usize) as u64).wrapping_sub(1u32);
    ((nbits << 1i32) as (usize))
      .wrapping_add(copylen.wrapping_sub(6usize) >> nbits)
      .wrapping_add(4usize) as (u16)
  } else if copylen < 2118usize {
    Log2FloorNonZero(copylen.wrapping_sub(70usize) as u64).wrapping_add(12u32) as (u16)
  } else {
    23u32 as (u16)
  }
}

#[inline(always)]
pub fn CombineLengthCodes(inscode: u16, copycode: u16, use_last_distance: i32) -> u16 {
  let bits64: u16 = (copycode as (u32) & 0x7u32 | (inscode as (u32) & 0x7u32) << 3i32) as (u16);
  if use_last_distance != 0 && (inscode as (i32) < 8i32) && (copycode as (i32) < 16i32) {
    if copycode as (i32) < 8i32 {
      bits64
    } else {
      let s64: u16 = 64i32 as (u16);
      (bits64 as (i32) | s64 as (i32)) as (u16)
    }
  } else {
    let sub_offset: i32 = 2i32 * ((copycode as (i32) >> 3i32) + 3i32 * (inscode as (i32) >> 3i32));
    let offset = (sub_offset << 5i32) + 0x40i32 + (0x520d40i32 >> sub_offset & 0xc0i32);
    (offset as (u16) as (i32) | bits64 as (i32)) as (u16)
  }
}

#[inline(always)]
pub fn GetLengthCode(insertlen: usize,
                     copylen: usize,
                     use_last_distance: i32,
                     code: &mut u16) {
  let inscode: u16 = GetInsertLengthCode(insertlen);
  let copycode: u16 = GetCopyLengthCode(copylen);
  *code = CombineLengthCodes(inscode, copycode, use_last_distance);
}
pub fn PrefixEncodeCopyDistance(distance_code: usize,
                                num_direct_codes: usize,
                                postfix_bits: u64,
                                code: &mut u16,
                                extra_bits: &mut u32) {
  if distance_code < (BROTLI_NUM_DISTANCE_SHORT_CODES as usize).wrapping_add(num_direct_codes) {
    *code = distance_code as (u16);
    *extra_bits = 0u32;
  } else {
    let dist: u64 = (1u64 << (postfix_bits as u64).wrapping_add(2u32 as (u64)))
      .wrapping_add((distance_code as u64).wrapping_sub(BROTLI_NUM_DISTANCE_SHORT_CODES as u64).wrapping_sub(num_direct_codes as u64) as u64);
    let bucket: u64 = Log2FloorNonZero(dist).wrapping_sub(1u32) as (u64);
    let postfix_mask: u64 = (1u32 << postfix_bits).wrapping_sub(1u32) as (u64);
    let postfix: u64 = dist & postfix_mask;
    let prefix: u64 = (dist >> bucket) & 1;
    let offset: u64 = (2u64).wrapping_add(prefix) << bucket;
    let nbits: u64 = bucket.wrapping_sub(postfix_bits);
    *code = ((nbits << 10) |
             ((BROTLI_NUM_DISTANCE_SHORT_CODES as u64).wrapping_add(num_direct_codes as u64).wrapping_add(
                 2u64.wrapping_mul(nbits.wrapping_sub(1)).wrapping_add(prefix) << postfix_bits).wrapping_add(postfix))) as u16;
      *extra_bits = (dist.wrapping_sub(offset) >> postfix_bits) as u32;
               /*(16u64)
      .wrapping_add(num_direct_codes as u64)
      .wrapping_add((2u64).wrapping_mul(nbits.wrapping_sub(1)).wrapping_add(prefix) <<
                    postfix_bits)
      .wrapping_add(postfix) as (u16);*/
      //*extra_bits = (nbits << 24i32 | dist.wrapping_sub(offset) >> postfix_bits) as (u32);
  }
}
pub fn CommandRestoreDistanceCode(xself: &Command, dist:&BrotliDistanceParams) -> u32 {
  if ((*xself).dist_prefix_ as (i32) & 0x3ff) < BROTLI_NUM_DISTANCE_SHORT_CODES as i32 + dist.num_direct_distance_codes as i32 {
    (*xself).dist_prefix_ as (u32) & 0x3ff
  } else {
      let dcode = xself.dist_prefix_ as u32 & 0x3ff;
      let nbits: u32 = u32::from((*xself).dist_prefix_ >> 10);
      let extra: u32 = (*xself).dist_extra_;
      let postfix_mask = (1u32 << dist.distance_postfix_bits) - 1;
      let hcode = dcode.wrapping_sub(dist.num_direct_distance_codes).wrapping_sub(BROTLI_NUM_DISTANCE_SHORT_CODES as u32) >> dist.distance_postfix_bits;
      let lcode = dcode.wrapping_sub(dist.num_direct_distance_codes).wrapping_sub(BROTLI_NUM_DISTANCE_SHORT_CODES as u32) & postfix_mask;
      let offset = (2u32.wrapping_add((hcode & 1)) << nbits).wrapping_sub(4);
      (offset.wrapping_add(extra) << dist.distance_postfix_bits).wrapping_add(lcode).wrapping_add(dist.num_direct_distance_codes).wrapping_add(
          BROTLI_NUM_DISTANCE_SHORT_CODES)
  }
}



// returns which distance code to use ( 0 means none, 1 means last, 2 means penultimate, 3 means the prior to penultimate
pub fn CommandDistanceIndexAndOffset(cmd: &Command,
                                     dist: &BrotliDistanceParams) -> (usize, isize) {
    let n_postfix = dist.distance_postfix_bits;
    let n_direct = dist.num_direct_distance_codes;
    let dextra = cmd.dist_extra_;
    let dprefix = cmd.dist_prefix_ & 0x3ff;
    let n_dist_bits = cmd.dist_prefix_ >> 10;
    if u32::from(dprefix) < BROTLI_NUM_DISTANCE_SHORT_CODES {
        let table: [(usize, isize);16]= [(1,0), (2,0),(3,0),(4,0),
                                        (1,-1), (1, 1), (1,-2), (1,2),(1,-3),(1,3),
                                         (2,-1),(2,1),(2,-2),(2,2),(2,-3),(2,3)];
        //eprint!("AA {:?} {:?} -> {:?}\n",*cmd, *dist, table[dprefix as usize]);
        return table[dprefix as usize];
    }
    if (dprefix as usize) < BROTLI_NUM_DISTANCE_SHORT_CODES as usize + n_direct as usize {
        let ret = dprefix as isize + 1 - BROTLI_NUM_DISTANCE_SHORT_CODES as isize;
        //eprint!("BB {:?} {:?} -> {:?}\n",*cmd, *dist, ret);
        return (0, ret);
    }
    let postfix_mask = (1 << n_postfix) - 1;
    let dcode = dprefix as u32 - BROTLI_NUM_DISTANCE_SHORT_CODES as u32 - n_direct;
    let hcode = dcode >> n_postfix;
    let lcode = dcode & postfix_mask;
    let offset = ((2 + (hcode & 1)) << n_dist_bits) - 4;

    let ret = (((offset + dextra) << n_postfix) + lcode + n_direct + 1) as isize;
    //assert!(ret != 0);
    (0, ret)
}

mod test {
    // returns which distance code to use ( 0 means none, 1 means last, 2 means penultimate, 3 means the prior to penultimate
    #[cfg(test)]
    pub fn helperCommandDistanceIndexAndOffset(cmd: &super::Command,
                                               dist: &super::BrotliDistanceParams) -> (usize, isize) {
        
        let n_postfix = dist.distance_postfix_bits;
        let n_direct = dist.num_direct_distance_codes;
        let dextra = cmd.dist_extra_;
        let dist_prefix = cmd.dist_prefix_ & 0x3ff;
        if dist_prefix < 16 {
            let table: [(usize, isize);16]= [(1,0), (2,0),(3,0),(4,0),
                                             (1,-1), (1, 1), (1,-2), (1,2),(1,-3),(1,3),
                                             (2,-1),(2,1),(2,-2),(2,2),(2,-3),(2,3)];
            return table[cmd.dist_prefix_ as usize];
    }
        if (dist_prefix as usize) < 16 + n_direct as usize {
            return (0, dist_prefix as isize + 1 - 16);
        }
        let postfix_mask = (1 << n_postfix) - 1;
        let dcode = dist_prefix as u32 - 16 - n_direct;
        let n_dist_bits = 1 + (dcode >> (n_postfix + 1));
        
        let hcode = dcode >> n_postfix;
    let lcode = dcode & postfix_mask;
        let offset = ((2 + (hcode & 1)) << n_dist_bits) - 4;
        (0, (((offset + dextra) << n_postfix) + lcode + n_direct + 1) as isize)
    }
    #[test]
    fn test_command_return_distance_index_offset() {
        let param = super::BrotliDistanceParams {
            distance_postfix_bits: 2,
            num_direct_distance_codes: 16,
            alphabet_size: 224,
            max_distance: 268435456,
        };
        let mut cmd = super::Command::default();
        cmd.insert_len_ = 63;
        cmd.copy_len_ = 3;
        cmd.dist_extra_ = 3;
        cmd.cmd_prefix_ = 297;
        cmd.dist_prefix_= 2089;
        
        assert_eq!(super::CommandDistanceIndexAndOffset(&cmd, &param),
                   (0,46));
        assert_eq!(super::CommandDistanceIndexAndOffset(&cmd, &param),
                   helperCommandDistanceIndexAndOffset(&cmd, &param));
        cmd = super::Command { insert_len_: 27, copy_len_: 3, dist_extra_: 0, cmd_prefix_: 281, dist_prefix_: 6 };
        assert_eq!(super::CommandDistanceIndexAndOffset(&cmd, &param),
                   (1,-2));
        assert_eq!(super::CommandDistanceIndexAndOffset(&cmd, &param),
                   helperCommandDistanceIndexAndOffset(&cmd, &param));
        cmd = super::Command { insert_len_: 1, copy_len_: 3, dist_extra_: 0, cmd_prefix_: 137, dist_prefix_: 27 };
        assert_eq!(super::CommandDistanceIndexAndOffset(&cmd, &param),
                   (0,12));
        assert_eq!(super::CommandDistanceIndexAndOffset(&cmd, &param),
                   helperCommandDistanceIndexAndOffset(&cmd, &param));
        cmd = super::Command { insert_len_: 5, copy_len_: 4, dist_extra_: 297, cmd_prefix_: 170, dist_prefix_: 11377 };
        assert_eq!(super::CommandDistanceIndexAndOffset(&cmd, &param),
                   (0,17574));
        assert_eq!(super::CommandDistanceIndexAndOffset(&cmd, &param),
                   helperCommandDistanceIndexAndOffset(&cmd, &param));
        super::super::encode::InitInsertCommand(&mut cmd, 24);
        assert_eq!(super::CommandDistanceIndexAndOffset(&cmd, &param),
                   (0,1));
        
    }
    /*
    #[test]
    fn test_restore_distance_code() {
        for dist_code in 0..50000 {
            let mut cmd = super::Command::default();
            let param =super::BrotliDistanceParams{
                distance_postfix_bits:2,
                num_direct_distance_codes:16,
                alphabet_size:224,
                max_distance:268435456,
            };
            super::InitCommand(&mut cmd, &param, 4, 4, 4, dist_code);
            let exp_dist_code = super::CommandRestoreDistanceCode(&cmd, &param);
            assert_eq!(exp_dist_code as u32, dist_code as u32);
        }
    }*/
}
pub fn RecomputeDistancePrefixes(cmds: &mut [Command],
                                 num_commands: usize,
                                 num_direct_distance_codes: u32,
                                 distance_postfix_bits: u32,
                                 dist:&BrotliDistanceParams) {
  let mut i: usize;
  if num_direct_distance_codes == 0u32 && (distance_postfix_bits == 0u32) {
    return;
  }
  i = 0usize;
  while i < num_commands {
    {
      let cmd: &mut Command = &mut cmds[(i as (usize))];
      if CommandCopyLen(cmd) != 0 && ((*cmd).cmd_prefix_ as (i32) >= 128i32) {
        PrefixEncodeCopyDistance(CommandRestoreDistanceCode(cmd, dist) as (usize),
                                 num_direct_distance_codes as (usize),
                                 distance_postfix_bits as (u64),
                                 &mut (*cmd).dist_prefix_,
                                 &mut (*cmd).dist_extra_);
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
}


pub fn InitCommand(xself: &mut Command,
                   dist: &BrotliDistanceParams,
                   insertlen: usize,
                   copylen: usize,
                   copylen_code: usize,
                   distance_code: usize) {
  
  xself.insert_len_ = insertlen as (u32);
  let copylen_code_delta = (copylen_code as i32 - copylen as i32) as i8;
  xself.copy_len_ = (copylen as u32 | (u32::from(copylen_code_delta as u8) << 25));
  PrefixEncodeCopyDistance(distance_code,
                           dist.num_direct_distance_codes as usize,
                           u64::from(dist.distance_postfix_bits),
                           &mut xself.dist_prefix_,
                           &mut xself.dist_extra_);
  GetLengthCode(insertlen,
                copylen_code,
                if !!((xself.dist_prefix_ as (i32) & 0x3ff) == 0i32) {
                  1i32
                } else {
                  0i32
                },
                &mut xself.cmd_prefix_);
}
pub fn NewCommand(dist: &BrotliDistanceParams,
                  insertlen: usize,
                  copylen: usize,
                  copylen_code: usize,
                  distance_code: usize)
                  -> Command {
  let mut xself: Command = Command {
    insert_len_: insertlen as (u32),
    copy_len_: (copylen | ((copylen_code ^ copylen) << 25)) as (u32),
    dist_extra_: 0,
    cmd_prefix_: 0,
    dist_prefix_: 0,
  };
  InitCommand(&mut xself, dist, insertlen, copylen, copylen_code, distance_code);
  xself
}
