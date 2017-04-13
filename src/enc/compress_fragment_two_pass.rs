use core;
use super::bit_cost::BitsEntropy;
use super::backward_references::kHashMul32;
use super::brotli_bit_stream::{BrotliBuildAndStoreHuffmanTreeFast, BrotliStoreHuffmanTree};
use super::entropy_encode::{BrotliConvertBitDepthsToSymbols, BrotliCreateHuffmanTree};
use super::static_dict::{BROTLI_UNALIGNED_LOAD32,BROTLI_UNALIGNED_LOAD64,
                         FindMatchLengthWithLimit};
use super::util::{brotli_min_size_t, Log2FloorNonZero};
static kCompressFragmentTwoPassBlockSize: usize = (1i32 << 17i32) as (usize);

// returns number of commands inserted
#[must_use]
fn EmitInsertLen(insertlen: u32, mut commands: &mut &mut [u32]) -> usize{
  if insertlen < 6u32 {
    (*commands)[0] = insertlen;
  } else if insertlen < 130u32 {
    let tail: u32 = insertlen.wrapping_sub(2u32);
    let nbits: u32 = Log2FloorNonZero(tail as (usize)).wrapping_sub(1u32);
    let prefix: u32 = tail >> nbits;
    let inscode: u32 = (nbits << 1i32).wrapping_add(prefix).wrapping_add(2u32);
    let extra: u32 = tail.wrapping_sub(prefix << nbits);
    (*commands)[0] = inscode | extra << 8i32;
  } else if insertlen < 2114u32 {
    let tail: u32 = insertlen.wrapping_sub(66u32);
    let nbits: u32 = Log2FloorNonZero(tail as (usize));
    let code: u32 = nbits.wrapping_add(10u32);
    let extra: u32 = tail.wrapping_sub(1u32 << nbits);
    (*commands)[0] = code | extra << 8i32;
  } else if insertlen < 6210u32 {
    let extra: u32 = insertlen.wrapping_sub(2114u32);
    (*commands)[0] = 21u32 | extra << 8i32;
  } else if insertlen < 22594u32 {
    let extra: u32 = insertlen.wrapping_sub(6210u32);
    (*commands)[0] = 22u32 | extra << 8i32;
  } else {
    let extra: u32 = insertlen.wrapping_sub(22594u32);
    (*commands)[0] = 23u32 | extra << 8i32;
  }
  let mut remainder = core::mem::replace(commands, &mut[]);
  core::mem::replace(commands, &mut remainder[1..]);
  1
}
#[must_use]
fn EmitDistance(distance: u32, mut commands: &mut &mut [u32]) -> usize {
  let d: u32 = distance.wrapping_add(3u32);
  let nbits: u32 = Log2FloorNonZero(d as (usize)).wrapping_sub(1u32);
  let prefix: u32 = d >> nbits & 1u32;
  let offset: u32 = (2u32).wrapping_add(prefix) << nbits;
  let distcode: u32 =
    (2u32).wrapping_mul(nbits.wrapping_sub(1u32)).wrapping_add(prefix).wrapping_add(80u32);
  let extra: u32 = d.wrapping_sub(offset);
  (*commands)[0] = distcode | extra << 8i32;
  let mut remainder = core::mem::replace(commands, &mut[]);
  core::mem::replace(commands, &mut remainder[1..]);
  1
}

#[must_use]
fn EmitCopyLenLastDistance(copylen: usize, mut commands: &mut &mut [u32]) -> usize {
  if copylen < 12usize {
    (*commands)[0] = copylen.wrapping_add(20usize) as (u32);
    let mut remainder = core::mem::replace(commands, &mut[]);
    core::mem::replace(commands, &mut remainder[1..]);
    1
  } else if copylen < 72usize {
    let tail: usize = copylen.wrapping_sub(8usize);
    let nbits: usize = Log2FloorNonZero(tail).wrapping_sub(1u32) as (usize);
    let prefix: usize = tail >> nbits;
    let code: usize = (nbits << 1i32).wrapping_add(prefix).wrapping_add(28usize);
    let extra: usize = tail.wrapping_sub(prefix << nbits);
    (*commands)[0] = (code | extra << 8i32) as (u32);
    let mut remainder = core::mem::replace(commands, &mut[]);
    core::mem::replace(commands, &mut remainder[1..]);
    1
  } else if copylen < 136usize {
    let tail: usize = copylen.wrapping_sub(8usize);
    let code: usize = (tail >> 5i32).wrapping_add(54usize);
    let extra: usize = tail & 31usize;
    (*commands)[0] = (code | extra << 8i32) as (u32);
    let mut remainder = core::mem::replace(commands, &mut[]);
    core::mem::replace(commands, &mut remainder[1..]);
    (*commands)[0] = 64u32;
    let mut remainder2 = core::mem::replace(commands, &mut[]);
    core::mem::replace(commands, &mut remainder2[1..]);
    2
  } else if copylen < 2120usize {
    let tail: usize = copylen.wrapping_sub(72usize);
    let nbits: usize = Log2FloorNonZero(tail) as (usize);
    let code: usize = nbits.wrapping_add(52usize);
    let extra: usize = tail.wrapping_sub(1usize << nbits);
    (*commands)[0] = (code | extra << 8i32) as (u32);
    let mut remainder = core::mem::replace(commands, &mut[]);
    core::mem::replace(commands, &mut remainder[1..]);
    (*commands)[0] = 64u32;
    let mut remainder2 = core::mem::replace(commands, &mut[]);
    core::mem::replace(commands, &mut remainder2[1..]);
    2
  } else {
    let extra: usize = copylen.wrapping_sub(2120usize);
    (*commands)[0] = (63usize | extra << 8i32) as (u32);
    let mut remainder = core::mem::replace(commands, &mut[]);
    core::mem::replace(commands, &mut remainder[1..]);
    (*commands)[0] = 64u32;
    let mut remainder2 = core::mem::replace(commands, &mut[]);
    core::mem::replace(commands, &mut remainder2[1..]);
    2
  }
}
fn HashBytesAtOffset(mut v: u64, offset: i32, shift: usize) -> u32 {
  0i32;
  0i32;
  {
    let h: u64 = (v >> 8i32 * offset << 16i32).wrapping_mul(kHashMul32 as (u64));
    (h >> shift) as (u32)
  }
}

#[must_use]
fn EmitCopyLen(copylen: usize, mut commands: &mut &mut[u32]) -> usize {
  if copylen < 10usize {
    (*commands)[0] = copylen.wrapping_add(38usize) as (u32);
  } else if copylen < 134usize {
    let tail: usize = copylen.wrapping_sub(6usize);
    let nbits: usize = Log2FloorNonZero(tail).wrapping_sub(1u32) as (usize);
    let prefix: usize = tail >> nbits;
    let code: usize = (nbits << 1i32).wrapping_add(prefix).wrapping_add(44usize);
    let extra: usize = tail.wrapping_sub(prefix << nbits);
    (*commands)[0] = (code | extra << 8i32) as (u32);
  } else if copylen < 2118usize {
    let tail: usize = copylen.wrapping_sub(70usize);
    let nbits: usize = Log2FloorNonZero(tail) as (usize);
    let code: usize = nbits.wrapping_add(52usize);
    let extra: usize = tail.wrapping_sub(1usize << nbits);
    (*commands)[0] = (code | extra << 8i32) as (u32);
  } else {
    let extra: usize = copylen.wrapping_sub(2118usize);
    (*commands)[0] = (63usize | extra << 8i32) as (u32);
  }
  let mut remainder = core::mem::replace(commands, &mut[]);
  core::mem::replace(commands, &mut remainder[1..]);
  1
}
fn Hash(p: &[u8], shift: usize) -> u32 {
  let h: u64 = (BROTLI_UNALIGNED_LOAD64(p) << 16i32).wrapping_mul(kHashMul32 as (u64));
  (h >> shift) as (u32)
}

fn IsMatch(p1: &[u8], p2: &[u8]) -> i32 {
  if !!(BROTLI_UNALIGNED_LOAD32(p1) == BROTLI_UNALIGNED_LOAD32(p2) &&
        (p1[(4usize)] as (i32) == p2[(4usize)] as (i32)) &&
        (p1[(5usize)] as (i32) == p2[(5usize)] as (i32))) {
    1i32
  } else {
    0i32
  }
}

fn CreateCommands(input_index: usize,
                  block_size: usize,
                  input_size: usize,
                  base_ip: &[u8],
                  mut table: &mut [i32],
                  table_bits: usize,
                  mut literals: &mut &mut[u8],
                  num_literals: &mut usize,
                  mut commands: &mut &mut[u32],
                  num_commands: &mut usize) {
  let mut ip_index: usize = input_index;
  let shift: usize = (64u32 as (usize)).wrapping_sub(table_bits);
  let ip_end: usize = input_index.wrapping_add(block_size);
  let mut next_emit: usize = input_index;
  let mut last_distance: i32 = -1i32;
  let kInputMarginBytes: usize = 16usize;
  let kMinMatchLen: usize = 6usize;
  if block_size >= kInputMarginBytes {
    let len_limit: usize = brotli_min_size_t(block_size.wrapping_sub(kMinMatchLen),
                                             input_size.wrapping_sub(kInputMarginBytes));
    let ip_limit: usize = input_index.wrapping_add(len_limit);
    let mut next_hash: u32;
    let mut goto_emit_remainder: i32 = 0i32;
    next_hash = Hash(&base_ip[({
                        ip_index = ip_index.wrapping_add(1 as (usize));
                        ip_index
                      } as (usize))..],
                     shift);
    while goto_emit_remainder == 0 {
      let mut skip: u32 = 32u32;
      let mut next_ip: usize = ip_index;
      let mut candidate: usize = 0;
      0i32;
      loop {
        {
          'break3: loop {
            {
              let hash: u32 = next_hash;
              let bytes_between_hash_lookups: u32 = ({
                                                           let _old = skip;
                                                           skip = skip.wrapping_add(1 as (u32));
                                                           _old
                                                         }) >>
                                                        5i32;
              ip_index = next_ip;
              0i32;
              next_ip = ip_index.wrapping_add(bytes_between_hash_lookups as (usize));
              if next_ip > ip_limit {
                goto_emit_remainder = 1i32;
                {
                  {
                    break 'break3;
                  }
                }
              }
              next_hash = Hash(&base_ip[(next_ip as (usize))..], shift);
              0i32;
              candidate = ip_index.wrapping_sub(last_distance as (usize));
              if IsMatch(&base_ip[(ip_index as (usize))..],
                         &base_ip[(candidate as (usize))..]) != 0 {
                if candidate < ip_index {
                  table[(hash as (usize))] = ip_index.wrapping_sub(0usize) as (i32);
                  {
                    {
                      break 'break3;
                    }
                  }
                }
              }
              candidate = table[(hash as (usize))] as (usize);
              0i32;
              0i32;
              table[(hash as (usize))] = ip_index.wrapping_sub(0usize) as (i32);
            }
            if !(IsMatch(&base_ip[(ip_index as (usize))..],
                         &base_ip[(candidate as (usize))..]) == 0) {
              break;
            }
          }
        }
        if !(ip_index.wrapping_sub(candidate) >
             (1usize << 18i32).wrapping_sub(16usize) as (isize) as (usize) &&
             (goto_emit_remainder == 0)) {
          break;
        }
      }
      if goto_emit_remainder != 0 {
        {
          break;
        }
      }
      {
        let base: usize = ip_index;
        let matched: usize = (6usize)
          .wrapping_add(FindMatchLengthWithLimit(&base_ip[(candidate as (usize) + 6)..],
                                                 &base_ip[(ip_index as (usize) + 6)..],
                                                 ip_end.wrapping_sub(ip_index)
                                                   .wrapping_sub(6usize)));
        let distance: i32 = base.wrapping_sub(candidate) as (i32);
        let insert: i32 = base.wrapping_sub(next_emit) as (i32);
        ip_index = ip_index.wrapping_add(matched);
        0i32;
        *num_commands += EmitInsertLen(insert as (u32), commands);
        (*literals)[..(insert as usize)].clone_from_slice(&base_ip[(next_emit as usize)..
                              ((next_emit + insert as usize))]);
        let mut new_literals = core::mem::replace(literals, &mut[]);
        core::mem::replace(literals, &mut new_literals[(insert as usize)..]);
        if distance == last_distance {
          (*commands)[0] = 64u32;
          let mut remainder = core::mem::replace(commands, &mut[]);
          core::mem::replace(commands, &mut remainder[1..]);
          *num_commands += 1;
        } else {
          *num_commands += EmitDistance(distance as (u32), commands);
          last_distance = distance;
        }
        *num_commands += EmitCopyLenLastDistance(matched, commands);
        next_emit = ip_index;
        if ip_index >= ip_limit {
          goto_emit_remainder = 1i32;
          {
            {
              break;
            }
          }
        }
        {
          assert!(ip_index >= 5);
          let mut input_bytes: u64 = BROTLI_UNALIGNED_LOAD64(&base_ip[(ip_index as (usize) - 5)..]); // could this be off the end FIXME
          let mut prev_hash: u32 = HashBytesAtOffset(input_bytes, 0i32, shift);
          let cur_hash: u32;
          table[(prev_hash as (usize))] = ip_index.wrapping_sub(5usize) as (i32);
          prev_hash = HashBytesAtOffset(input_bytes, 1i32, shift);
          table[(prev_hash as (usize))] = ip_index.wrapping_sub(4usize) as (i32);
          prev_hash = HashBytesAtOffset(input_bytes, 2i32, shift);
          table[(prev_hash as (usize))] = ip_index.wrapping_sub(3usize) as (i32);
          assert!(ip_index >= 2);
          input_bytes = BROTLI_UNALIGNED_LOAD64(&base_ip[(ip_index as (usize) - 2)..]);
          cur_hash = HashBytesAtOffset(input_bytes, 2i32, shift);
          prev_hash = HashBytesAtOffset(input_bytes, 0i32, shift);
          table[(prev_hash as (usize))] = ip_index.wrapping_sub(2usize) as (i32);
          prev_hash = HashBytesAtOffset(input_bytes, 1i32, shift);
          table[(prev_hash as (usize))] = ip_index.wrapping_sub(1usize) as (i32);
          candidate = table[(cur_hash as (usize))] as (usize);
          table[(cur_hash as (usize))] = ip_index as (i32);
        }
      }
      while ip_index.wrapping_sub(candidate) <=
            (1usize << 18i32).wrapping_sub(16usize) as (isize) as (usize) &&
            (IsMatch(&base_ip[(ip_index as (usize))..],
                     &base_ip[(candidate as (usize))..]) != 0) {
        let base_index: usize = ip_index;
        let matched: usize = (6usize)
          .wrapping_add(FindMatchLengthWithLimit(&base_ip[(candidate as (usize) + 6)..],
                                                 &base_ip[(ip_index as (usize) + 6)..],
                                                 ip_end.wrapping_sub(ip_index)
                                                   .wrapping_sub(6usize)));
        ip_index = ip_index.wrapping_add(matched);
        last_distance = base_index.wrapping_sub(candidate) as (i32);
        0i32;
        *num_commands += EmitCopyLen(matched, commands);
        *num_commands += EmitDistance(last_distance as (u32), commands);
        next_emit = ip_index;
        if ip_index >= ip_limit {
          goto_emit_remainder = 1i32;
          {
            {
              break;
            }
          }
        }
        {
          assert!(ip_index >= 5);
          let mut input_bytes: u64 = BROTLI_UNALIGNED_LOAD64(&base_ip[(ip_index as (usize) - 5)..]);
          let mut prev_hash: u32 = HashBytesAtOffset(input_bytes, 0i32, shift);
          let cur_hash: u32;
          table[(prev_hash as (usize))] = ip_index.wrapping_sub(5usize) as (i32);
          prev_hash = HashBytesAtOffset(input_bytes, 1i32, shift);
          table[(prev_hash as (usize))] = ip_index.wrapping_sub(4usize) as (i32);
          prev_hash = HashBytesAtOffset(input_bytes, 2i32, shift);
          table[(prev_hash as (usize))] = ip_index.wrapping_sub(3usize) as (i32);
          assert!(ip_index >= 2);
          input_bytes = BROTLI_UNALIGNED_LOAD64(&base_ip[(ip_index as (usize) - 2)..]);
          cur_hash = HashBytesAtOffset(input_bytes, 2i32, shift);
          prev_hash = HashBytesAtOffset(input_bytes, 0i32, shift);
          table[(prev_hash as (usize))] = ip_index.wrapping_sub(2usize) as (i32);
          prev_hash = HashBytesAtOffset(input_bytes, 1i32, shift);
          table[(prev_hash as (usize))] = ip_index.wrapping_sub(1usize) as (i32);
          candidate = table[(cur_hash as (usize))] as (usize);
          table[(cur_hash as (usize))] = ip_index as (i32);
        }
      }
      if goto_emit_remainder == 0 {
        next_hash = Hash(&base_ip[({
                            ip_index = ip_index.wrapping_add(1 as (usize));
                            ip_index
                          } as (usize))..],
                         shift);
      }
    }
  }
  0i32;
  if next_emit < ip_end {
    let insert: u32 = ip_end.wrapping_sub(next_emit) as (u32);
    *num_commands += EmitInsertLen(insert, commands);
    literals[..insert as usize].clone_from_slice(&base_ip[(next_emit as (usize))..(next_emit + insert as usize)]);
    let mut xliterals = core::mem::replace(literals, &mut[]);
    *literals = &mut core::mem::replace(&mut xliterals, &mut[])[(insert as usize)..];
    *num_literals += insert as usize;
  }
}
/*

fn ShouldCompress(mut input: &[u8], mut input_size: usize, mut num_literals: usize) -> i32 {
  let mut corpus_size: f64 = input_size as (f64);
  if num_literals as (f64) < 0.98f64 * corpus_size {
    1i32
  } else {
    let mut literal_histo: [u32; 256] = [0;256];
    let max_total_bit_cost: f64 = corpus_size * 8i32 as (f64) * 0.98f64 / 43i32 as (f64);
    let mut i: usize;
    i = 0usize;
    while i < input_size {
      {
        let _rhs = 1;
        let _lhs = &mut literal_histo[input[(i as (usize))] as (usize)];
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
      }
      i = i.wrapping_add(43usize);
    }
    if !!(BitsEntropy(literal_histo.as_mut_ptr(), 256usize) < max_total_bit_cost) {
      1i32
    } else {
      0i32
    }
  }
}

fn BROTLI_UNALIGNED_STORE64(mut p: &mut [::std::os::raw::c_void], mut v: usize) {
  memcpy(p, &mut v, ::std::mem::size_of::<usize>());
}

fn BrotliWriteBits(mut n_bits: usize,
                   mut bits: usize,
                   mut pos: &mut [usize],
                   mut array: &mut [u8]) {
  let mut p: *mut u8 = &mut array[((*pos >> 3i32) as (usize))];
  let mut v: usize = *p as (usize);
  0i32;
  0i32;
  v = v | bits << (*pos & 7usize);
  BROTLI_UNALIGNED_STORE64(p, v);
  *pos = (*pos).wrapping_add(n_bits);
}

fn BrotliStoreMetaBlockHeader(mut len: usize,
                              mut is_uncompressed: i32,
                              mut storage_ix: &mut [usize],
                              mut storage: &mut [u8]) {
  let mut nibbles: usize = 6usize;
  BrotliWriteBits(1usize, 0usize, storage_ix, storage);
  if len <= (1u32 << 16i32) as (usize) {
    nibbles = 4usize;
  } else if len <= (1u32 << 20i32) as (usize) {
    nibbles = 5usize;
  }
  BrotliWriteBits(2usize, nibbles.wrapping_sub(4usize), storage_ix, storage);
  BrotliWriteBits(nibbles.wrapping_mul(4usize),
                  len.wrapping_sub(1usize),
                  storage_ix,
                  storage);
  BrotliWriteBits(1usize, is_uncompressed as (usize), storage_ix, storage);
}



pub struct HuffmanTree {
  pub total_count_: u32,
  pub index_left_: i16,
  pub index_right_or_value_: i16,
}

fn BuildAndStoreCommandPrefixCode(mut histogram: &[u32],
                                  mut depth: &mut [u8],
                                  mut bits: &mut [u16],
                                  mut storage_ix: &mut [usize],
                                  mut storage: &mut [u8]) {
  let mut tree: [HuffmanTree; 129];
  let mut cmd_depth: [u8; 704] = [0; 704];
  let mut cmd_bits: [u16; 64];
  BrotliCreateHuffmanTree(histogram, 64usize, 15i32, tree.as_mut_ptr(), depth);
  BrotliCreateHuffmanTree(&histogram[(64usize)],
                          64usize,
                          14i32,
                          tree.as_mut_ptr(),
                          &mut depth[(64usize)]);
  memcpy(cmd_depth.as_mut_ptr(), depth[(24usize)..], 24usize);
  memcpy(cmd_depth.as_mut_ptr().offset(24i32 as (isize)),
         depth,
         8usize);
  memcpy(cmd_depth.as_mut_ptr().offset(32i32 as (isize)),
         depth[(48usize)..],
         8usize);
  memcpy(cmd_depth.as_mut_ptr().offset(40i32 as (isize)),
         depth[(8usize)..],
         8usize);
  memcpy(cmd_depth.as_mut_ptr().offset(48i32 as (isize)),
         depth[(56usize)..],
         8usize);
  memcpy(cmd_depth.as_mut_ptr().offset(56i32 as (isize)),
         depth[(16usize)..],
         8usize);
  BrotliConvertBitDepthsToSymbols(cmd_depth.as_mut_ptr(), 64usize, cmd_bits.as_mut_ptr());
  memcpy(bits,
         cmd_bits.as_mut_ptr().offset(24i32 as (isize)),
         16usize);
  memcpy(bits[(8usize)..],
         cmd_bits.as_mut_ptr().offset(40i32 as (isize)),
         16usize);
  memcpy(bits[(16usize)..],
         cmd_bits.as_mut_ptr().offset(56i32 as (isize)),
         16usize);
  memcpy(bits[(24usize)..], cmd_bits.as_mut_ptr(), 48usize);
  memcpy(bits[(48usize)..],
         cmd_bits.as_mut_ptr().offset(32i32 as (isize)),
         16usize);
  memcpy(bits[(56usize)..],
         cmd_bits.as_mut_ptr().offset(48i32 as (isize)),
         16usize);
  BrotliConvertBitDepthsToSymbols(&mut depth[(64usize)], 64usize, &mut bits[(64usize)]);
  {
    let mut i: usize;
    memset(cmd_depth.as_mut_ptr(), 0i32, 64usize);
    memcpy(cmd_depth.as_mut_ptr(), depth[(24usize)..], 8usize);
    memcpy(cmd_depth.as_mut_ptr().offset(64i32 as (isize)),
           depth[(32usize)..],
           8usize);
    memcpy(cmd_depth.as_mut_ptr().offset(128i32 as (isize)),
           depth[(40usize)..],
           8usize);
    memcpy(cmd_depth.as_mut_ptr().offset(192i32 as (isize)),
           depth[(48usize)..],
           8usize);
    memcpy(cmd_depth.as_mut_ptr().offset(384i32 as (isize)),
           depth[(56usize)..],
           8usize);
    i = 0usize;
    while i < 8usize {
      {
        cmd_depth[(128usize).wrapping_add((8usize).wrapping_mul(i))] = depth[(i as (usize))];
        cmd_depth[(256usize).wrapping_add((8usize).wrapping_mul(i))] = depth[((8usize).wrapping_add(i) as
         (usize))];
        cmd_depth[(448usize).wrapping_add((8usize).wrapping_mul(i))] = depth[((16usize).wrapping_add(i) as
         (usize))];
      }
      i = i.wrapping_add(1 as (usize));
    }
    BrotliStoreHuffmanTree(cmd_depth.as_mut_ptr(),
                           704usize,
                           tree.as_mut_ptr(),
                           storage_ix,
                           storage);
  }
  BrotliStoreHuffmanTree(&mut depth[(64usize)],
                         64usize,
                         tree.as_mut_ptr(),
                         storage_ix,
                         storage);
}

fn StoreCommands(mut m: &mut [MemoryManager],
                 mut literals: &[u8],
                 num_literals: usize,
                 mut commands: &[u32],
                 num_commands: usize,
                 mut storage_ix: &mut [usize],
                 mut storage: &mut [u8]) {
  static mut kNumExtraBits: [u32; 128] =
    [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 1u32, 2u32, 2u32, 3u32, 3u32, 4u32, 4u32, 5u32,
     5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 12u32, 14u32, 24u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 1u32, 1u32, 2u32, 2u32, 3u32, 3u32, 4u32, 4u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 1u32, 1u32, 2u32, 2u32, 3u32, 3u32, 4u32, 4u32, 5u32, 5u32, 6u32, 7u32,
     8u32, 9u32, 10u32, 24u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 1u32, 2u32, 2u32, 3u32, 3u32, 4u32, 4u32, 5u32, 5u32,
     6u32, 6u32, 7u32, 7u32, 8u32, 8u32, 9u32, 9u32, 10u32, 10u32, 11u32, 11u32, 12u32, 12u32,
     13u32, 13u32, 14u32, 14u32, 15u32, 15u32, 16u32, 16u32, 17u32, 17u32, 18u32, 18u32, 19u32,
     19u32, 20u32, 20u32, 21u32, 21u32, 22u32, 22u32, 23u32, 23u32, 24u32, 24u32];
  static mut kInsertOffset: [u32; 24] =
    [0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 8u32, 10u32, 14u32, 18u32, 26u32, 34u32, 50u32,
     66u32, 98u32, 130u32, 194u32, 322u32, 578u32, 1090u32, 2114u32, 6210u32, 22594u32];
  let mut lit_depths: [u8; 256];
  let mut lit_bits: [u16; 256];
  let mut lit_histo: [u32; 256] = [0;256];
  let mut cmd_depths: [u8; 128] = [0; 128];
  let mut cmd_bits: [u16; 128] = [0; 128];
  let mut cmd_histo: [u32; 128] = [0; 128];
  let mut i: usize;
  i = 0usize;
  while i < num_literals {
    {
      let _rhs = 1;
      let _lhs = &mut lit_histo[literals[(i as (usize))] as (usize)];
      *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    i = i.wrapping_add(1 as (usize));
  }
  BrotliBuildAndStoreHuffmanTreeFast(m,
                                     lit_histo.as_mut_ptr(),
                                     num_literals,
                                     8usize,
                                     lit_depths.as_mut_ptr(),
                                     lit_bits.as_mut_ptr(),
                                     storage_ix,
                                     storage);
  if !(0i32 == 0) {
    return;
  }
  i = 0usize;
  while i < num_commands {
    {
      let code: u32 = commands[(i as (usize))] & 0xffu32;
      0i32;
      {
        let _rhs = 1;
        let _lhs = &mut cmd_histo[code as (usize)];
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  {
    let _rhs = 1i32;
    let _lhs = &mut cmd_histo[1usize];
    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
  }
  {
    let _rhs = 1i32;
    let _lhs = &mut cmd_histo[2usize];
    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
  }
  {
    let _rhs = 1i32;
    let _lhs = &mut cmd_histo[64usize];
    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
  }
  {
    let _rhs = 1i32;
    let _lhs = &mut cmd_histo[84usize];
    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
  }
  BuildAndStoreCommandPrefixCode(cmd_histo.as_mut_ptr(),
                                 cmd_depths.as_mut_ptr(),
                                 cmd_bits.as_mut_ptr(),
                                 storage_ix,
                                 storage);
  i = 0usize;
  while i < num_commands {
    {
      let cmd: u32 = commands[(i as (usize))];
      let code: u32 = cmd & 0xffu32;
      let extra: u32 = cmd >> 8i32;
      0i32;
      BrotliWriteBits(cmd_depths[code as (usize)] as (usize),
                      cmd_bits[code as (usize)] as (usize),
                      storage_ix,
                      storage);
      BrotliWriteBits(kNumExtraBits[code as (usize)] as (usize),
                      extra as (usize),
                      storage_ix,
                      storage);
      if code < 24u32 {
        let insert: u32 = kInsertOffset[code as (usize)].wrapping_add(extra);
        let mut j: u32;
        j = 0u32;
        while j < insert {
          {
            let lit: u8 = *literals;
            BrotliWriteBits(lit_depths[lit as (usize)] as (usize),
                            lit_bits[lit as (usize)] as (usize),
                            storage_ix,
                            storage);
            literals = literals[(1 as (usize))..];
          }
          j = j.wrapping_add(1 as (u32));
        }
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
}

fn EmitUncompressedMetaBlock(mut input: &[u8],
                             mut input_size: usize,
                             mut storage_ix: &mut [usize],
                             mut storage: &mut [u8]) {
  BrotliStoreMetaBlockHeader(input_size, 1i32, storage_ix, storage);
  *storage_ix = (*storage_ix).wrapping_add(7u32 as (usize)) & !7u32 as (usize);
  memcpy(&mut storage[((*storage_ix >> 3i32) as (usize))],
         input,
         input_size);
  *storage_ix = (*storage_ix).wrapping_add(input_size << 3i32);
  storage[((*storage_ix >> 3i32) as (usize))] = 0i32 as (u8);
}

fn BrotliCompressFragmentTwoPassImpl(mut m: &mut [MemoryManager],
                                     mut base_ip: &[u8],
                                     mut input_size: usize,
                                     mut is_last: i32,
                                     mut command_buf: &mut [u32],
                                     mut literal_buf: &mut [u8],
                                     mut table: &mut [i32],
                                     mut table_bits: usize,
                                     mut storage_ix: &mut [usize],
                                     mut storage: &mut [u8]) {
  let mut input_index: usize = 0usize;
  is_last;
  while input_size > 0usize {
    let mut block_size: usize = brotli_min_size_t(input_size, kCompressFragmentTwoPassBlockSize);
    let mut commands: *mut u32 = command_buf;
    let mut literals: *mut u8 = literal_buf;
    let mut num_literals: usize;
    CreateCommands(input_index,
                   block_size,
                   input_size,
                   base_ip,
                   table,
                   table_bits,
                   &mut literals,
                   &mut commands);
    num_literals = ((literals as (isize)).wrapping_sub(literal_buf as (isize)) /
                    ::std::mem::size_of::<*mut u8>() as (isize)) as (usize);
    if ShouldCompress(base_ip[(input_index as (usize))..],
                      block_size,
                      num_literals) != 0 {
      let num_commands: usize = ((commands as (isize)).wrapping_sub(command_buf as (isize)) /
                                 ::std::mem::size_of::<*mut u32>() as (isize)) as
                                (usize);
      BrotliStoreMetaBlockHeader(block_size, 0i32, storage_ix, storage);
      BrotliWriteBits(13usize, 0usize, storage_ix, storage);
      StoreCommands(m,
                    literal_buf,
                    num_literals,
                    command_buf,
                    num_commands,
                    storage_ix,
                    storage);
      if !(0i32 == 0) {
        return;
      }
    } else {
      EmitUncompressedMetaBlock(base_ip[(input_index as (usize))..],
                                block_size,
                                storage_ix,
                                storage);
    }
    input_index = input_index.wrapping_add(block_size);
    input_size = input_size.wrapping_sub(block_size);
  }
}

fn BrotliCompressFragmentTwoPassImpl8(mut m: &mut [MemoryManager],
                                      mut input: &[u8],
                                      mut input_size: usize,
                                      mut is_last: i32,
                                      mut command_buf: &mut [u32],
                                      mut literal_buf: &mut [u8],
                                      mut table: &mut [i32],
                                      mut storage_ix: &mut [usize],
                                      mut storage: &mut [u8]) {
  BrotliCompressFragmentTwoPassImpl(m,
                                    input,
                                    input_size,
                                    is_last,
                                    command_buf,
                                    literal_buf,
                                    table,
                                    8usize,
                                    storage_ix,
                                    storage);
}

fn BrotliCompressFragmentTwoPassImpl9(mut m: &mut [MemoryManager],
                                      mut input: &[u8],
                                      mut input_size: usize,
                                      mut is_last: i32,
                                      mut command_buf: &mut [u32],
                                      mut literal_buf: &mut [u8],
                                      mut table: &mut [i32],
                                      mut storage_ix: &mut [usize],
                                      mut storage: &mut [u8]) {
  BrotliCompressFragmentTwoPassImpl(m,
                                    input,
                                    input_size,
                                    is_last,
                                    command_buf,
                                    literal_buf,
                                    table,
                                    9usize,
                                    storage_ix,
                                    storage);
}

fn BrotliCompressFragmentTwoPassImpl10(mut m: &mut [MemoryManager],
                                       mut input: &[u8],
                                       mut input_size: usize,
                                       mut is_last: i32,
                                       mut command_buf: &mut [u32],
                                       mut literal_buf: &mut [u8],
                                       mut table: &mut [i32],
                                       mut storage_ix: &mut [usize],
                                       mut storage: &mut [u8]) {
  BrotliCompressFragmentTwoPassImpl(m,
                                    input,
                                    input_size,
                                    is_last,
                                    command_buf,
                                    literal_buf,
                                    table,
                                    10usize,
                                    storage_ix,
                                    storage);
}

fn BrotliCompressFragmentTwoPassImpl11(mut m: &mut [MemoryManager],
                                       mut input: &[u8],
                                       mut input_size: usize,
                                       mut is_last: i32,
                                       mut command_buf: &mut [u32],
                                       mut literal_buf: &mut [u8],
                                       mut table: &mut [i32],
                                       mut storage_ix: &mut [usize],
                                       mut storage: &mut [u8]) {
  BrotliCompressFragmentTwoPassImpl(m,
                                    input,
                                    input_size,
                                    is_last,
                                    command_buf,
                                    literal_buf,
                                    table,
                                    11usize,
                                    storage_ix,
                                    storage);
}

fn BrotliCompressFragmentTwoPassImpl12(mut m: &mut [MemoryManager],
                                       mut input: &[u8],
                                       mut input_size: usize,
                                       mut is_last: i32,
                                       mut command_buf: &mut [u32],
                                       mut literal_buf: &mut [u8],
                                       mut table: &mut [i32],
                                       mut storage_ix: &mut [usize],
                                       mut storage: &mut [u8]) {
  BrotliCompressFragmentTwoPassImpl(m,
                                    input,
                                    input_size,
                                    is_last,
                                    command_buf,
                                    literal_buf,
                                    table,
                                    12usize,
                                    storage_ix,
                                    storage);
}

fn BrotliCompressFragmentTwoPassImpl13(mut m: &mut [MemoryManager],
                                       mut input: &[u8],
                                       mut input_size: usize,
                                       mut is_last: i32,
                                       mut command_buf: &mut [u32],
                                       mut literal_buf: &mut [u8],
                                       mut table: &mut [i32],
                                       mut storage_ix: &mut [usize],
                                       mut storage: &mut [u8]) {
  BrotliCompressFragmentTwoPassImpl(m,
                                    input,
                                    input_size,
                                    is_last,
                                    command_buf,
                                    literal_buf,
                                    table,
                                    13usize,
                                    storage_ix,
                                    storage);
}

fn BrotliCompressFragmentTwoPassImpl14(mut m: &mut [MemoryManager],
                                       mut input: &[u8],
                                       mut input_size: usize,
                                       mut is_last: i32,
                                       mut command_buf: &mut [u32],
                                       mut literal_buf: &mut [u8],
                                       mut table: &mut [i32],
                                       mut storage_ix: &mut [usize],
                                       mut storage: &mut [u8]) {
  BrotliCompressFragmentTwoPassImpl(m,
                                    input,
                                    input_size,
                                    is_last,
                                    command_buf,
                                    literal_buf,
                                    table,
                                    14usize,
                                    storage_ix,
                                    storage);
}

fn BrotliCompressFragmentTwoPassImpl15(mut m: &mut [MemoryManager],
                                       mut input: &[u8],
                                       mut input_size: usize,
                                       mut is_last: i32,
                                       mut command_buf: &mut [u32],
                                       mut literal_buf: &mut [u8],
                                       mut table: &mut [i32],
                                       mut storage_ix: &mut [usize],
                                       mut storage: &mut [u8]) {
  BrotliCompressFragmentTwoPassImpl(m,
                                    input,
                                    input_size,
                                    is_last,
                                    command_buf,
                                    literal_buf,
                                    table,
                                    15usize,
                                    storage_ix,
                                    storage);
}

fn BrotliCompressFragmentTwoPassImpl16(mut m: &mut [MemoryManager],
                                       mut input: &[u8],
                                       mut input_size: usize,
                                       mut is_last: i32,
                                       mut command_buf: &mut [u32],
                                       mut literal_buf: &mut [u8],
                                       mut table: &mut [i32],
                                       mut storage_ix: &mut [usize],
                                       mut storage: &mut [u8]) {
  BrotliCompressFragmentTwoPassImpl(m,
                                    input,
                                    input_size,
                                    is_last,
                                    command_buf,
                                    literal_buf,
                                    table,
                                    16usize,
                                    storage_ix,
                                    storage);
}

fn BrotliCompressFragmentTwoPassImpl17(mut m: &mut [MemoryManager],
                                       mut input: &[u8],
                                       mut input_size: usize,
                                       mut is_last: i32,
                                       mut command_buf: &mut [u32],
                                       mut literal_buf: &mut [u8],
                                       mut table: &mut [i32],
                                       mut storage_ix: &mut [usize],
                                       mut storage: &mut [u8]) {
  BrotliCompressFragmentTwoPassImpl(m,
                                    input,
                                    input_size,
                                    is_last,
                                    command_buf,
                                    literal_buf,
                                    table,
                                    17usize,
                                    storage_ix,
                                    storage);
}

fn RewindBitPosition(new_storage_ix: usize, mut storage_ix: &mut [usize], mut storage: &mut [u8]) {
  let bitpos: usize = new_storage_ix & 7usize;
  let mask: usize = (1u32 << bitpos).wrapping_sub(1u32) as (usize);
  {
    let _rhs = mask as (u8);
    let _lhs = &mut storage[((new_storage_ix >> 3i32) as (usize))];
    *_lhs = (*_lhs as (i32) & _rhs as (i32)) as (u8);
  }
  *storage_ix = new_storage_ix;
}


pub fn BrotliCompressFragmentTwoPass(mut m: &mut [MemoryManager],
                                     mut input: &[u8],
                                     mut input_size: usize,
                                     mut is_last: i32,
                                     mut command_buf: &mut [u32],
                                     mut literal_buf: &mut [u8],
                                     mut table: &mut [i32],
                                     mut table_size: usize,
                                     mut storage_ix: &mut [usize],
                                     mut storage: &mut [u8]) {
  let initial_storage_ix: usize = *storage_ix;
  let table_bits: usize = Log2FloorNonZero(table_size) as (usize);
  if table_bits == 8usize {
    BrotliCompressFragmentTwoPassImpl8(m,
                                       input,
                                       input_size,
                                       is_last,
                                       command_buf,
                                       literal_buf,
                                       table,
                                       storage_ix,
                                       storage);
  }
  if table_bits == 9usize {
    BrotliCompressFragmentTwoPassImpl9(m,
                                       input,
                                       input_size,
                                       is_last,
                                       command_buf,
                                       literal_buf,
                                       table,
                                       storage_ix,
                                       storage);
  }
  if table_bits == 10usize {
    BrotliCompressFragmentTwoPassImpl10(m,
                                        input,
                                        input_size,
                                        is_last,
                                        command_buf,
                                        literal_buf,
                                        table,
                                        storage_ix,
                                        storage);
  }
  if table_bits == 11usize {
    BrotliCompressFragmentTwoPassImpl11(m,
                                        input,
                                        input_size,
                                        is_last,
                                        command_buf,
                                        literal_buf,
                                        table,
                                        storage_ix,
                                        storage);
  }
  if table_bits == 12usize {
    BrotliCompressFragmentTwoPassImpl12(m,
                                        input,
                                        input_size,
                                        is_last,
                                        command_buf,
                                        literal_buf,
                                        table,
                                        storage_ix,
                                        storage);
  }
  if table_bits == 13usize {
    BrotliCompressFragmentTwoPassImpl13(m,
                                        input,
                                        input_size,
                                        is_last,
                                        command_buf,
                                        literal_buf,
                                        table,
                                        storage_ix,
                                        storage);
  }
  if table_bits == 14usize {
    BrotliCompressFragmentTwoPassImpl14(m,
                                        input,
                                        input_size,
                                        is_last,
                                        command_buf,
                                        literal_buf,
                                        table,
                                        storage_ix,
                                        storage);
  }
  if table_bits == 15usize {
    BrotliCompressFragmentTwoPassImpl15(m,
                                        input,
                                        input_size,
                                        is_last,
                                        command_buf,
                                        literal_buf,
                                        table,
                                        storage_ix,
                                        storage);
  }
  if table_bits == 16usize {
    BrotliCompressFragmentTwoPassImpl16(m,
                                        input,
                                        input_size,
                                        is_last,
                                        command_buf,
                                        literal_buf,
                                        table,
                                        storage_ix,
                                        storage);
  }
  if table_bits == 17usize {
    BrotliCompressFragmentTwoPassImpl17(m,
                                        input,
                                        input_size,
                                        is_last,
                                        command_buf,
                                        literal_buf,
                                        table,
                                        storage_ix,
                                        storage);
  }
  if (*storage_ix).wrapping_sub(initial_storage_ix) > (31usize).wrapping_add(input_size << 3i32) {
    RewindBitPosition(initial_storage_ix, storage_ix, storage);
    EmitUncompressedMetaBlock(input, input_size, storage_ix, storage);
  }
  if is_last != 0 {
    BrotliWriteBits(1usize, 1usize, storage_ix, storage);
    BrotliWriteBits(1usize, 1usize, storage_ix, storage);
    *storage_ix = (*storage_ix).wrapping_add(7u32 as (usize)) & !7u32 as (usize);
  }
}
*/