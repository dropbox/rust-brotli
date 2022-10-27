#![allow(dead_code)]
use super::backward_references::kHashMul32;
//use super::super::alloc::{SliceWrapper, SliceWrapperMut};
use super::bit_cost::BitsEntropy;
use super::brotli_bit_stream::{BrotliBuildAndStoreHuffmanTreeFast, BrotliStoreHuffmanTree};
use super::entropy_encode::{BrotliConvertBitDepthsToSymbols, BrotliCreateHuffmanTree, HuffmanTree,
                            NewHuffmanTree};
use super::static_dict::{BROTLI_UNALIGNED_LOAD32, BROTLI_UNALIGNED_LOAD64, BROTLI_UNALIGNED_STORE64,
                         FindMatchLengthWithLimit};
use super::super::alloc;
use super::util::{brotli_min_size_t, Log2FloorNonZero};
use core;
static kCompressFragmentTwoPassBlockSize: usize = (1i32 << 17i32) as (usize);

// returns number of commands inserted
fn EmitInsertLen(insertlen: u32, commands: &mut &mut [u32]) -> usize {
  if insertlen < 6u32 {
    (*commands)[0] = insertlen;
  } else if insertlen < 130u32 {
    let tail: u32 = insertlen.wrapping_sub(2u32);
    let nbits: u32 = Log2FloorNonZero(tail as (u64)).wrapping_sub(1u32);
    let prefix: u32 = tail >> nbits;
    let inscode: u32 = (nbits << 1i32).wrapping_add(prefix).wrapping_add(2u32);
    let extra: u32 = tail.wrapping_sub(prefix << nbits);
    (*commands)[0] = inscode | extra << 8i32;
  } else if insertlen < 2114u32 {
    let tail: u32 = insertlen.wrapping_sub(66u32);
    let nbits: u32 = Log2FloorNonZero(tail as (u64));
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
  let remainder = core::mem::replace(commands, &mut []);
  let _ = core::mem::replace(commands, &mut remainder[1..]);
  1
}

fn EmitDistance(distance: u32, commands: &mut &mut [u32]) -> usize {
  let d: u32 = distance.wrapping_add(3u32);
  let nbits: u32 = Log2FloorNonZero(d as (u64)).wrapping_sub(1u32);
  let prefix: u32 = d >> nbits & 1u32;
  let offset: u32 = (2u32).wrapping_add(prefix) << nbits;
  let distcode: u32 =
    (2u32).wrapping_mul(nbits.wrapping_sub(1u32)).wrapping_add(prefix).wrapping_add(80u32);
  let extra: u32 = d.wrapping_sub(offset);
  (*commands)[0] = distcode | extra << 8i32;
  let remainder = core::mem::replace(commands, &mut []);
  let _ = core::mem::replace(commands, &mut remainder[1..]);
  1
}

fn EmitCopyLenLastDistance(copylen: usize, commands: &mut &mut [u32]) -> usize {
  if copylen < 12usize {
    (*commands)[0] = copylen.wrapping_add(20usize) as (u32);
    let remainder = core::mem::replace(commands, &mut []);
    let _ = core::mem::replace(commands, &mut remainder[1..]);
    1
  } else if copylen < 72usize {
    let tail: usize = copylen.wrapping_sub(8usize);
    let nbits: usize = Log2FloorNonZero(tail as u64).wrapping_sub(1u32) as (usize);
    let prefix: usize = tail >> nbits;
    let code: usize = (nbits << 1i32).wrapping_add(prefix).wrapping_add(28usize);
    let extra: usize = tail.wrapping_sub(prefix << nbits);
    (*commands)[0] = (code | extra << 8i32) as (u32);
    let remainder = core::mem::replace(commands, &mut []);
    let _ = core::mem::replace(commands, &mut remainder[1..]);
    1
  } else if copylen < 136usize {
    let tail: usize = copylen.wrapping_sub(8usize);
    let code: usize = (tail >> 5i32).wrapping_add(54usize);
    let extra: usize = tail & 31usize;
    (*commands)[0] = (code | extra << 8i32) as (u32);
    let remainder = core::mem::replace(commands, &mut []);
    let _ = core::mem::replace(commands, &mut remainder[1..]);
    (*commands)[0] = 64u32;
    let remainder2 = core::mem::replace(commands, &mut []);
    let _ = core::mem::replace(commands, &mut remainder2[1..]);
    2
  } else if copylen < 2120usize {
    let tail: usize = copylen.wrapping_sub(72usize);
    let nbits: usize = Log2FloorNonZero(tail as u64) as (usize);
    let code: usize = nbits.wrapping_add(52usize);
    let extra: usize = tail.wrapping_sub(1usize << nbits);
    (*commands)[0] = (code | extra << 8i32) as (u32);
    let remainder = core::mem::replace(commands, &mut []);
    let _ = core::mem::replace(commands, &mut remainder[1..]);
    (*commands)[0] = 64u32;
    let remainder2 = core::mem::replace(commands, &mut []);
    let _ = core::mem::replace(commands, &mut remainder2[1..]);
    2
  } else {
    let extra: usize = copylen.wrapping_sub(2120usize);
    (*commands)[0] = (63usize | extra << 8i32) as (u32);
    let remainder = core::mem::replace(commands, &mut []);
    let _ = core::mem::replace(commands, &mut remainder[1..]);
    (*commands)[0] = 64u32;
    let remainder2 = core::mem::replace(commands, &mut []);
    let _ = core::mem::replace(commands, &mut remainder2[1..]);
    2
  }
}
fn HashBytesAtOffset(v: u64, offset: i32, shift: usize, length: usize) -> u32 {
  0i32;
  0i32;
  {
    let h: u64 = (v >> 8i32 * offset << ((8 - length) * 8)).wrapping_mul(kHashMul32 as (u64));
    (h >> shift) as (u32)
  }
}

fn EmitCopyLen(copylen: usize, commands: &mut &mut [u32]) -> usize {
  if copylen < 10usize {
    (*commands)[0] = copylen.wrapping_add(38usize) as (u32);
  } else if copylen < 134usize {
    let tail: usize = copylen.wrapping_sub(6usize);
    let nbits: usize = Log2FloorNonZero(tail as u64).wrapping_sub(1u32) as (usize);
    let prefix: usize = tail >> nbits;
    let code: usize = (nbits << 1i32).wrapping_add(prefix).wrapping_add(44usize);
    let extra: usize = tail.wrapping_sub(prefix << nbits);
    (*commands)[0] = (code | extra << 8i32) as (u32);
  } else if copylen < 2118usize {
    let tail: usize = copylen.wrapping_sub(70usize);
    let nbits: usize = Log2FloorNonZero(tail as u64) as (usize);
    let code: usize = nbits.wrapping_add(52usize);
    let extra: usize = tail.wrapping_sub(1usize << nbits);
    (*commands)[0] = (code | extra << 8i32) as (u32);
  } else {
    let extra: usize = copylen.wrapping_sub(2118usize);
    (*commands)[0] = (63usize | extra << 8i32) as (u32);
  }
  let remainder = core::mem::replace(commands, &mut []);
  let _ = core::mem::replace(commands, &mut remainder[1..]);
  1
}
fn Hash(p: &[u8], shift: usize, length:usize) -> u32 {
  let h: u64 = (BROTLI_UNALIGNED_LOAD64(p) << ((8 - length) * 8)).wrapping_mul(kHashMul32 as (u64));
  (h >> shift) as (u32)
}

fn IsMatch(p1: &[u8], p2: &[u8], length: usize) -> i32 {
    if BROTLI_UNALIGNED_LOAD32(p1) == BROTLI_UNALIGNED_LOAD32(p2) {
        if length == 4 {
            return 1;
        }
        return 
            ((p1[(4usize)] as (i32) == p2[(4usize)] as (i32)) &&
             (p1[(5usize)] as (i32) == p2[(5usize)] as (i32))) as i32
    }
    0
}

#[allow(unused_assignments)]
fn CreateCommands(input_index: usize,
                  block_size: usize,
                  input_size: usize,
                  base_ip: &[u8],
                  table: &mut [i32],
                  table_bits: usize,
                  min_match: usize,
                  literals: &mut &mut [u8],
                  num_literals: &mut usize,
                  commands: &mut &mut [u32],
                  num_commands: &mut usize) {
  let mut ip_index: usize = input_index;
  let shift: usize = (64u32 as (usize)).wrapping_sub(table_bits);
  let ip_end: usize = input_index.wrapping_add(block_size);
  let mut next_emit: usize = input_index;
  let mut last_distance: i32 = -1i32;
  let kInputMarginBytes: usize = 16usize;

  if block_size >= kInputMarginBytes {
    let len_limit: usize = brotli_min_size_t(block_size.wrapping_sub(min_match),
                                             input_size.wrapping_sub(kInputMarginBytes));
    let ip_limit: usize = input_index.wrapping_add(len_limit);
    let mut next_hash: u32;
    let mut goto_emit_remainder: i32 = 0i32;
    next_hash = Hash(&base_ip[({
                         ip_index = ip_index.wrapping_add(1 as (usize));
                         ip_index
                       } as (usize))..],
                     shift, min_match);
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
              next_hash = Hash(&base_ip[(next_ip as (usize))..], shift, min_match);
              0i32;
              candidate = ip_index.wrapping_sub(last_distance as (usize));
              if IsMatch(&base_ip[(ip_index as (usize))..],
                         &base_ip[(candidate as (usize))..], min_match) != 0 {
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
                         &base_ip[(candidate as (usize))..], min_match) == 0) {
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
        let matched: usize = min_match
          .wrapping_add(FindMatchLengthWithLimit(&base_ip[(candidate as (usize) + min_match)..],
                                                 &base_ip[(ip_index as (usize) + min_match)..],
                                                 ip_end.wrapping_sub(ip_index)
                                                   .wrapping_sub(min_match)));
        let distance: i32 = base.wrapping_sub(candidate) as (i32);
        let insert: i32 = base.wrapping_sub(next_emit) as (i32);
        ip_index = ip_index.wrapping_add(matched);
        0i32;
        *num_commands += EmitInsertLen(insert as (u32), commands);
        (*literals)[..(insert as usize)].clone_from_slice(&base_ip[(next_emit as usize)..
                                                           ((next_emit +
                                                             insert as usize))]);
        *num_literals += insert as usize;
        let new_literals = core::mem::replace(literals, &mut []);
        let _ = core::mem::replace(literals, &mut new_literals[(insert as usize)..]);
        if distance == last_distance {
          (*commands)[0] = 64u32;
          let remainder = core::mem::replace(commands, &mut []);
          let _ = core::mem::replace(commands, &mut remainder[1..]);
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
          let mut input_bytes: u64;
          let mut prev_hash: u32;
          let cur_hash: u32;
          if min_match == 4 {
              input_bytes = BROTLI_UNALIGNED_LOAD64(&base_ip[(ip_index as (usize) - 3)..]);
              cur_hash = HashBytesAtOffset(input_bytes, 3i32, shift, min_match);
              prev_hash = HashBytesAtOffset(input_bytes, 0i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(3usize) as (i32);
              prev_hash = HashBytesAtOffset(input_bytes, 1i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(2usize) as (i32);
              prev_hash = HashBytesAtOffset(input_bytes, 0i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(1usize) as (i32);
          }else {
              assert!(ip_index >= 5);
              // could this be off the end FIXME
              input_bytes = BROTLI_UNALIGNED_LOAD64(&base_ip[(ip_index as (usize) - 5)..]);
              prev_hash = HashBytesAtOffset(input_bytes, 0i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(5usize) as (i32);
              prev_hash = HashBytesAtOffset(input_bytes, 1i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(4usize) as (i32);
              prev_hash = HashBytesAtOffset(input_bytes, 2i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(3usize) as (i32);
              assert!(ip_index >= 2);
              input_bytes = BROTLI_UNALIGNED_LOAD64(&base_ip[(ip_index as (usize) - 2)..]);
              cur_hash = HashBytesAtOffset(input_bytes, 2i32, shift, min_match);
              prev_hash = HashBytesAtOffset(input_bytes, 0i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(2usize) as (i32);
              prev_hash = HashBytesAtOffset(input_bytes, 1i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(1usize) as (i32);
          }
          candidate = table[(cur_hash as (usize))] as (usize);
          table[(cur_hash as (usize))] = ip_index as (i32);
        }
      }
      while ip_index.wrapping_sub(candidate) <=
            (1usize << 18i32).wrapping_sub(16usize) as (isize) as (usize) &&
            (IsMatch(&base_ip[(ip_index as (usize))..],
                     &base_ip[(candidate as (usize))..], min_match) != 0) {
        let base_index: usize = ip_index;
        let matched: usize = min_match
          .wrapping_add(FindMatchLengthWithLimit(&base_ip[(candidate as (usize) + min_match)..],
                                                 &base_ip[(ip_index as (usize) + min_match)..],
                                                 ip_end.wrapping_sub(ip_index)
                                                   .wrapping_sub(min_match)));
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
          let mut input_bytes: u64;
          
          let cur_hash: u32;
          let mut prev_hash: u32;
          if min_match == 4 {
              input_bytes  = BROTLI_UNALIGNED_LOAD64(&base_ip[(ip_index as (usize) - 3)..]);
              cur_hash = HashBytesAtOffset(input_bytes, 3i32, shift, min_match);
              prev_hash = HashBytesAtOffset(input_bytes, 0i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(3usize) as (i32);
              prev_hash = HashBytesAtOffset(input_bytes, 1i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(2usize) as (i32);
              prev_hash = HashBytesAtOffset(input_bytes, 2i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(1usize) as (i32);
          } else {
              input_bytes  = BROTLI_UNALIGNED_LOAD64(&base_ip[(ip_index as (usize) - 5)..]);
              prev_hash = HashBytesAtOffset(input_bytes, 0i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(5usize) as (i32);
              prev_hash = HashBytesAtOffset(input_bytes, 1i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(4usize) as (i32);
              prev_hash = HashBytesAtOffset(input_bytes, 2i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(3usize) as (i32);
              assert!(ip_index >= 2);
              input_bytes = BROTLI_UNALIGNED_LOAD64(&base_ip[(ip_index as (usize) - 2)..]);
              cur_hash = HashBytesAtOffset(input_bytes, 2i32, shift, min_match);
              prev_hash = HashBytesAtOffset(input_bytes, 0i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(2usize) as (i32);
              prev_hash = HashBytesAtOffset(input_bytes, 1i32, shift, min_match);
              table[(prev_hash as (usize))] = ip_index.wrapping_sub(1usize) as (i32);
          }
          candidate = table[(cur_hash as (usize))] as (usize);
          table[(cur_hash as (usize))] = ip_index as (i32);
        }
      }
      if goto_emit_remainder == 0 {
        next_hash = Hash(&base_ip[({
                             ip_index = ip_index.wrapping_add(1 as (usize));
                             ip_index
                           } as (usize))..],
                         shift, min_match);
      }
    }
  }
  0i32;
  if next_emit < ip_end {
    let insert: u32 = ip_end.wrapping_sub(next_emit) as (u32);
    *num_commands += EmitInsertLen(insert, commands);
    literals[..insert as usize].clone_from_slice(&base_ip[(next_emit as (usize))..
                                                  (next_emit + insert as usize)]);
    let mut xliterals = core::mem::replace(literals, &mut []);
    *literals = &mut core::mem::replace(&mut xliterals, &mut [])[(insert as usize)..];
    *num_literals += insert as usize;
  }
}

fn ShouldCompress(input: &[u8], input_size: usize, num_literals: usize) -> i32 {
  let corpus_size: super::util::floatX = input_size as (super::util::floatX);
  if num_literals as (super::util::floatX) < 0.98 as super::util::floatX * corpus_size {
    1i32
  } else {
    let mut literal_histo: [u32; 256] = [0; 256];
    let max_total_bit_cost: super::util::floatX = corpus_size * 8i32 as (super::util::floatX) * 0.98 as super::util::floatX / 43i32 as (super::util::floatX);
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
    if !!(BitsEntropy(&mut literal_histo[..], 256usize) < max_total_bit_cost) {
      1i32
    } else {
      0i32
    }
  }
}

pub fn BrotliWriteBits(n_bits: usize, bits: u64, pos: &mut usize, array: &mut [u8]) {
  let p = &mut array[((*pos >> 3i32) as (usize))..];
  let mut v: u64 = p[0] as (u64);
  v = v | bits << (*pos & 7);
  BROTLI_UNALIGNED_STORE64(p, v);
  *pos = (*pos).wrapping_add(n_bits);
}
pub fn BrotliStoreMetaBlockHeader(len: usize,
                                  is_uncompressed: i32,
                                  storage_ix: &mut usize,
                                  storage: &mut [u8]) {
  let mut nibbles: u64 = 6;
  BrotliWriteBits(1, 0, storage_ix, storage);
  if len <= (1u32 << 16i32) as (usize) {
    nibbles = 4;
  } else if len <= (1u32 << 20i32) as (usize) {
    nibbles = 5;
  }
  BrotliWriteBits(2, nibbles.wrapping_sub(4), storage_ix, storage);
  BrotliWriteBits(nibbles.wrapping_mul(4) as usize,
                  len.wrapping_sub(1) as u64,
                  storage_ix,
                  storage);
  BrotliWriteBits(1usize, is_uncompressed as (u64), storage_ix, storage);
}


pub fn memcpy<T: Sized + Clone>(dst: &mut [T],
                                dst_offset: usize,
                                src: &[T],
                                src_offset: usize,
                                size_to_copy: usize) {
  dst[dst_offset..(dst_offset + size_to_copy)].clone_from_slice(&src[src_offset..
                                                                 (src_offset + size_to_copy)]);
}
fn BuildAndStoreCommandPrefixCode(histogram: &[u32],
                                  depth: &mut [u8],
                                  mut bits: &mut [u16],
                                  storage_ix: &mut usize,
                                  storage: &mut [u8]) {
  let mut tree: [HuffmanTree; 129] = [NewHuffmanTree(0, 0, 0); 129];
  let mut cmd_depth: [u8; 704] = [0; 704];
  let mut cmd_bits: [u16; 64] = [0; 64];
  BrotliCreateHuffmanTree(histogram, 64usize, 15i32, &mut tree[..], depth);
  BrotliCreateHuffmanTree(&histogram[(64usize)..],
                          64usize,
                          14i32,
                          &mut tree[..],
                          &mut depth[(64usize)..]);
  /* We have to jump through a few hoops here in order to compute
     the command bits because the symbols are in a different order than in
     the full alphabet. This looks complicated, but having the symbols
     in this order in the command bits saves a few branches in the Emit*
     functions. */
  memcpy(&mut cmd_depth[..], 0, depth, 24, 24);
  memcpy(&mut cmd_depth[..], 24, depth, 0, 8);
  memcpy(&mut cmd_depth[..],
         32i32 as (usize),
         depth,
         (48usize),
         8usize);
  memcpy(&mut cmd_depth[..],
         40i32 as (usize),
         depth,
         (8usize),
         8usize);
  memcpy(&mut cmd_depth[..],
         48i32 as (usize),
         depth,
         (56usize),
         8usize);
  memcpy(&mut cmd_depth[..],
         56i32 as (usize),
         depth,
         (16usize),
         8usize);
  BrotliConvertBitDepthsToSymbols(&mut cmd_depth[..], 64usize, &mut cmd_bits[..]);
  memcpy(&mut bits, 0, &cmd_bits[..], 24i32 as (usize), 16usize);
  memcpy(&mut bits,
         (8usize),
         &cmd_bits[..],
         40i32 as (usize),
         8usize);
  memcpy(&mut bits,
         (16usize),
         &cmd_bits[..],
         56i32 as (usize),
         8usize);
  memcpy(&mut bits, (24usize), &cmd_bits[..], 0, 48usize);
  memcpy(&mut bits,
         (48usize),
         &cmd_bits[..],
         32i32 as (usize),
         8usize);
  memcpy(&mut bits,
         (56usize),
         &cmd_bits[..],
         48i32 as (usize),
         8usize);
  BrotliConvertBitDepthsToSymbols(&mut depth[(64usize)..], 64usize, &mut bits[(64usize)..]);
  {
    let mut i: usize;
    for item in cmd_depth[..64].iter_mut() {
      *item = 0;
    }
    //memset(&mut cmd_depth[..], 0i32, 64usize);
    memcpy(&mut cmd_depth[..], 0, depth, (24usize), 8usize);
    memcpy(&mut cmd_depth[..],
           64i32 as (usize),
           depth,
           (32usize),
           8usize);
    memcpy(&mut cmd_depth[..],
           128i32 as (usize),
           depth,
           (40usize),
           8usize);
    memcpy(&mut cmd_depth[..],
           192i32 as (usize),
           depth,
           (48usize),
           8usize);
    memcpy(&mut cmd_depth[..],
           384i32 as (usize),
           depth,
           (56usize),
           8usize);
    i = 0usize;
    while i < 8usize {
      {
        cmd_depth[(128usize).wrapping_add((8usize).wrapping_mul(i))] = depth[(i as (usize))];
        cmd_depth[(256usize).wrapping_add((8usize).wrapping_mul(i))] = depth[i.wrapping_add(8)];
        cmd_depth[(448usize).wrapping_add((8usize).wrapping_mul(i))] = depth[i.wrapping_add(16)];
      }
      i = i.wrapping_add(1 as (usize));
    }
    BrotliStoreHuffmanTree(&mut cmd_depth[..],
                           704usize,
                           &mut tree[..],
                           storage_ix,
                           storage);
  }
  BrotliStoreHuffmanTree(&mut depth[(64usize)..],
                         64usize,
                         &mut tree[..],
                         storage_ix,
                         storage);
}

fn StoreCommands<AllocHT: alloc::Allocator<HuffmanTree>>(mht: &mut AllocHT,
                                                         mut literals: &[u8],
                                                         num_literals: usize,
                                                         commands: &[u32],
                                                         num_commands: usize,
                                                         storage_ix: &mut usize,
                                                         storage: &mut [u8]) {
  static kNumExtraBits: [u32; 128] =
    [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 1u32, 2u32, 2u32, 3u32, 3u32, 4u32, 4u32, 5u32,
     5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 12u32, 14u32, 24u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 1u32, 1u32, 2u32, 2u32, 3u32, 3u32, 4u32, 4u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 1u32, 1u32, 2u32, 2u32, 3u32, 3u32, 4u32, 4u32, 5u32, 5u32, 6u32, 7u32,
     8u32, 9u32, 10u32, 24u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
     0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 1u32, 2u32, 2u32, 3u32, 3u32, 4u32, 4u32, 5u32, 5u32,
     6u32, 6u32, 7u32, 7u32, 8u32, 8u32, 9u32, 9u32, 10u32, 10u32, 11u32, 11u32, 12u32, 12u32,
     13u32, 13u32, 14u32, 14u32, 15u32, 15u32, 16u32, 16u32, 17u32, 17u32, 18u32, 18u32, 19u32,
     19u32, 20u32, 20u32, 21u32, 21u32, 22u32, 22u32, 23u32, 23u32, 24u32, 24u32];
  static kInsertOffset: [u32; 24] = [0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 8u32, 10u32, 14u32,
                                     18u32, 26u32, 34u32, 50u32, 66u32, 98u32, 130u32, 194u32,
                                     322u32, 578u32, 1090u32, 2114u32, 6210u32, 22594u32];
  let mut lit_depths: [u8; 256] = [0; 256];
  let mut lit_bits: [u16; 256] = [0; 256]; // maybe return this instead
  let mut lit_histo: [u32; 256] = [0; 256]; // maybe return this instead of init
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
  BrotliBuildAndStoreHuffmanTreeFast(mht,
                                     &lit_histo[..],
                                     num_literals,
                                     8usize,
                                     &mut lit_depths[..],
                                     &mut lit_bits[..],
                                     storage_ix,
                                     storage);
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
  BuildAndStoreCommandPrefixCode(&mut cmd_histo[..],
                                 &mut cmd_depths[..],
                                 &mut cmd_bits[..],
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
                      cmd_bits[code as (usize)] as (u64),
                      storage_ix,
                      storage);
      BrotliWriteBits(kNumExtraBits[code as (usize)] as (usize),
                      extra as (u64),
                      storage_ix,
                      storage);
      if code < 24u32 {
        let insert: u32 = kInsertOffset[code as (usize)].wrapping_add(extra);
        for literal in literals[..(insert as usize)].iter() {
          let lit: u8 = *literal;
          BrotliWriteBits(lit_depths[lit as (usize)] as (usize),
                          lit_bits[lit as (usize)] as (u64),
                          storage_ix,
                          storage);
        }
        literals = &literals[insert as usize..];
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
}
fn EmitUncompressedMetaBlock(input: &[u8],
                             input_size: usize,
                             storage_ix: &mut usize,
                             storage: &mut [u8]) {
  BrotliStoreMetaBlockHeader(input_size, 1i32, storage_ix, storage);
  *storage_ix = (*storage_ix).wrapping_add(7u32 as (usize)) & !7u32 as (usize);
  memcpy(storage,
         ((*storage_ix >> 3i32) as (usize)),
         input,
         0,
         input_size);
  *storage_ix = (*storage_ix).wrapping_add(input_size << 3i32);
  storage[((*storage_ix >> 3i32) as (usize))] = 0i32 as (u8);
}
#[allow(unused_variables)]
#[inline(always)]
fn BrotliCompressFragmentTwoPassImpl<AllocHT:alloc::Allocator<HuffmanTree>>(m: &mut AllocHT,
                                     base_ip: &[u8],
                                     mut input_size: usize,
                                     is_last: i32,
                                     command_buf: &mut [u32],
                                     literal_buf: &mut [u8],
                                     table: &mut [i32],
                                     table_bits: usize,
                                     min_match: usize,
                                     storage_ix: &mut usize,
                                     storage: &mut [u8]){
  let mut input_index: usize = 0usize;
  while input_size > 0usize {
    let block_size: usize = brotli_min_size_t(input_size, kCompressFragmentTwoPassBlockSize);
    let mut num_literals: usize = 0;
    let mut num_commands: usize = 0;
    {
      let mut literals = &mut literal_buf[..];
      let mut commands = &mut command_buf[..];
      CreateCommands(input_index,
                     block_size,
                     input_size,
                     base_ip,
                     table,
                     table_bits,
                     min_match,
                     &mut literals,
                     &mut num_literals,
                     &mut commands,
                     &mut num_commands);
    }
    if ShouldCompress(&base_ip[(input_index as (usize))..],
                      block_size,
                      num_literals) != 0 {
      BrotliStoreMetaBlockHeader(block_size, 0i32, storage_ix, storage);
      BrotliWriteBits(13usize, 0, storage_ix, storage);
      StoreCommands(m,
                    literal_buf,
                    num_literals,
                    command_buf,
                    num_commands,
                    storage_ix,
                    storage);
    } else {
      EmitUncompressedMetaBlock(&base_ip[(input_index as (usize))..],
                                block_size,
                                storage_ix,
                                storage);
    }
    input_index = input_index.wrapping_add(block_size);
    input_size = input_size.wrapping_sub(block_size);
  }
}
macro_rules! compress_specialization {
    ($table_bits : expr, $fname: ident) => {
        fn $fname<AllocHT:alloc::Allocator<HuffmanTree>>(mht: &mut AllocHT,
                                      input: &[u8],
                                      input_size: usize,
                                      is_last: i32,
                                      command_buf: &mut [u32],
                                      literal_buf: &mut [u8],
                                      table: &mut [i32],
                                      storage_ix: &mut usize,
                                                         storage: &mut [u8]) {
            let min_match = if $table_bits < 15 {4} else {6};
            BrotliCompressFragmentTwoPassImpl(mht,
                                              input,
                                              input_size,
                                              is_last,
                                              command_buf,
                                              literal_buf,
                                              table,
                                              $table_bits,
                                              min_match,
                                              storage_ix,
                                              storage);
        }
    };
}
compress_specialization!(8, BrotliCompressFragmentTwoPassImpl8);
compress_specialization!(9, BrotliCompressFragmentTwoPassImpl9);
compress_specialization!(10, BrotliCompressFragmentTwoPassImpl10);
compress_specialization!(11, BrotliCompressFragmentTwoPassImpl11);
compress_specialization!(12, BrotliCompressFragmentTwoPassImpl12);
compress_specialization!(13, BrotliCompressFragmentTwoPassImpl13);
compress_specialization!(14, BrotliCompressFragmentTwoPassImpl14);
compress_specialization!(15, BrotliCompressFragmentTwoPassImpl15);
compress_specialization!(16, BrotliCompressFragmentTwoPassImpl16);
compress_specialization!(17, BrotliCompressFragmentTwoPassImpl17);

fn RewindBitPosition(new_storage_ix: usize, storage_ix: &mut usize, storage: &mut [u8]) {
  let bitpos: usize = new_storage_ix & 7usize;
  let mask: usize = (1u32 << bitpos).wrapping_sub(1u32) as (usize);
  {
    let _rhs = mask as (u8);
    let _lhs = &mut storage[((new_storage_ix >> 3i32) as (usize))];
    *_lhs = (*_lhs as (i32) & _rhs as (i32)) as (u8);
  }
  *storage_ix = new_storage_ix;
}

pub fn BrotliCompressFragmentTwoPass<AllocHT:alloc::Allocator<HuffmanTree>>(m: &mut AllocHT,
                                     input: &[u8],
                                     input_size: usize,
                                     is_last: i32,
                                     command_buf: &mut [u32],
                                     literal_buf: &mut [u8],
                                     table: &mut [i32],
                                     table_size: usize,
                                     storage_ix: &mut usize,
                                     storage: &mut [u8]){
  let initial_storage_ix: usize = *storage_ix;
  let table_bits: usize = Log2FloorNonZero(table_size as u64) as (usize);
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
    BrotliWriteBits(1, 1, storage_ix, storage);
    BrotliWriteBits(1, 1, storage_ix, storage);
    *storage_ix = (*storage_ix).wrapping_add(7u32 as (usize)) & !7u32 as (usize);
  }
}
