/* Copyright 2010 Google Inc. All Rights Reserved.

   Distributed under MIT license.
   See file LICENSE for detail or copy at https://opensource.org/licenses/MIT
*/

/* Entropy encoding (Huffman) utilities. */

use super::util::brotli_max_uint32_t;


#[derive(Clone, Copy)]
pub struct HuffmanTree {
  pub total_count_: u32,
  pub index_left_: i16,
  pub index_right_or_value_: i16,
}


pub fn NewHuffmanTree(count: u32, left: i16, right: i16) -> HuffmanTree {
  return HuffmanTree {
           total_count_: count,
           index_left_: left,
           index_right_or_value_: right,
         };
}
pub fn InitHuffmanTree(xself: &mut HuffmanTree, count: u32, left: i16, right: i16) {
  *xself = NewHuffmanTree(count, left, right);
}


pub fn BrotliSetDepth(p0: i32, pool: &mut [HuffmanTree], depth: &mut [u8], max_depth: i32) -> bool {
  let mut stack: [i16; 16] = [0; 16];
  let mut level = 0i32;
  let mut p = p0;
  assert!(max_depth <= 15);
  stack[0] = -1;
  loop {
    if (pool[p as usize].index_left_ >= 0) {
      level += 1;
      if (level > max_depth) {
        return false;
      };
      stack[level as usize] = pool[p as usize].index_right_or_value_;
      p = pool[p as usize].index_left_ as i32;
      continue;
    } else {
      depth[pool[p as usize].index_right_or_value_ as usize] = level as u8;
    }
    while (level >= 0 && stack[level as usize] == -1) {
      level -= 1;
    }
    if (level < 0) {
      return true;
    }
    p = stack[level as usize] as i32;
    stack[level as usize] = -1;
  }
}


pub trait HuffmanComparator {
  fn Cmp(self: &Self, a: &HuffmanTree, b: &HuffmanTree) -> bool;
}
pub struct SortHuffmanTree {}
impl HuffmanComparator for SortHuffmanTree {
  fn Cmp(self: &Self, v0: &HuffmanTree, v1: &HuffmanTree) -> bool {
    if (*v0).total_count_ != (*v1).total_count_ {
      if !!((*v0).total_count_ < (*v1).total_count_) {
        true
      } else {
        false
      }
    } else if !!((*v0).index_right_or_value_ as (i32) > (*v1).index_right_or_value_ as (i32)) {
      true
    } else {
      false
    }
  }
}
pub fn SortHuffmanTreeItems<Comparator: HuffmanComparator>(mut items: &mut [HuffmanTree],
                                                           n: usize,
                                                           comparator: Comparator) {

  static gaps: [usize; 6] = [132i32 as (usize),
                             57i32 as (usize),
                             23i32 as (usize),
                             10i32 as (usize),
                             4i32 as (usize),
                             1i32 as (usize)];
  if n < 13 {
    let mut i: usize;
    i = 1;
    'loop14: loop {
      if i < n {
        let tmp: HuffmanTree = items[i as usize].clone();
        let mut k: usize = i;
        let mut j: usize = i.wrapping_sub(1);
        'loop17: loop {
          if comparator.Cmp(&tmp, &items[j as usize]) {
            items[k as usize] = items[j as usize].clone();
            k = j;
            if {
                 let _old = j;
                 j = j.wrapping_sub(1);
                 _old
               } == 0 {
              break 'loop17;
            } else {
              continue 'loop17;
            }
          } else {
            break 'loop17;
          }
        }
        items[k as usize] = tmp;
        i = i.wrapping_add(1);
        continue 'loop14;
      } else {
        break 'loop14;
      }
    }
  } else {
    let mut g: i32 = if n < 57 { 2i32 } else { 0i32 };
    'loop2: loop {
      if g < 6i32 {
        let gap: usize = gaps[g as usize];
        let mut i: usize;
        i = gap;
        'loop5: loop {
          if i < n {
            let mut j: usize = i;
            let tmp: HuffmanTree = items[i as usize].clone();
            'loop8: loop {
              if j >= gap && (comparator.Cmp(&tmp, &items[(j - gap) as usize])) {
                items[j as usize] = items[(j - gap) as usize].clone();
                j = j.wrapping_sub(gap);
                continue 'loop8;
              } else {
                break 'loop8;
              }
            }
            items[j as usize] = tmp;
            i = i.wrapping_add(1);
            continue 'loop5;
          } else {
            break 'loop5;
          }
        }
        g = g + 1;
        continue 'loop2;
      } else {
        break 'loop2;
      }
    }
  }
}



/* This function will create a Huffman tree.

   The catch here is that the tree cannot be arbitrarily deep.
   Brotli specifies a maximum depth of 15 bits for "code trees"
   and 7 bits for "code length code trees."

   count_limit is the value that is to be faked as the minimum value
   and this minimum value is raised until the tree matches the
   maximum length requirement.

   This algorithm is not of excellent performance for very long data blocks,
   especially when population counts are longer than 2**tree_limit, but
   we are not planning to use this with extremely long blocks.

   See http://en.wikipedia.org/wiki/Huffman_coding */
pub fn BrotliCreateHuffmanTree(data: &[u32],
                               length: usize,
                               tree_limit: i32,
                               mut tree: &mut [HuffmanTree],
                               mut depth: &mut [u8]) {
  let mut count_limit: u32;
  let sentinel: HuffmanTree = NewHuffmanTree(!(0i32 as (u32)), -1i32 as (i16), -1i32 as (i16));

  /* For block sizes below 64 kB, we never need to do a second iteration
     of this loop. Probably all of our block sizes will be smaller than
     that, so this loop is mostly of academic interest. If we actually
     would need this, we would be better off with the Katajainen algorithm. */
  count_limit = 1i32 as (u32);
  'loop1: loop {
    let mut n: usize = 0i32 as (usize);
    let mut i: usize;
    let mut j: usize;
    let mut k: usize;
    i = length;
    'loop2: loop {
      if i != 0i32 as (usize) {
        i = i.wrapping_sub(1 as (usize));
        if data[i as (usize)] != 0 {
          let count: u32 = brotli_max_uint32_t(data[i as (usize)], count_limit);
          InitHuffmanTree(&mut tree[{
                                 let _old = n;
                                 n = n.wrapping_add(1 as (usize));
                                 _old
                               } as (usize)],
                          count,
                          -1i32 as (i16),
                          i as (i16));
          continue 'loop2;
        } else {
          continue 'loop2;
        }
      } else {
        break 'loop2;
      }
    }
    if n == 1i32 as (usize) {
      depth[(tree[0]).index_right_or_value_ as (usize)] = 1;
    } else {
      SortHuffmanTreeItems(tree, n, SortHuffmanTree {});
      /* The nodes are:
            [0, n): the sorted leaf nodes that we start with.
            [n]: we add a sentinel here.
            [n + 1, 2n): new parent nodes are added here, starting from
            (n+1). These are naturally in ascending order.
            [2n]: we add a sentinel at the end as well.
            There will be (2n+1) elements at the end. */
      tree[n as (usize)] = sentinel.clone();
      tree[n.wrapping_add(1i32 as (usize)) as (usize)] = sentinel.clone();
      i = 0usize; // Points to the next leaf node
      j = n.wrapping_add(1i32 as (usize)); // points to the next non-leaf node
      k = n.wrapping_sub(1i32 as (usize));
      'loop5: loop {
        if k != 0i32 as (usize) {
          let left: usize;
          let right: usize;
          if (tree[i as (usize)]).total_count_ <= (tree[j as usize]).total_count_ {
            left = i;
            i = i.wrapping_add(1 as (usize));
          } else {
            left = j;
            j = j.wrapping_add(1 as (usize));
          }
          if (tree[i as (usize)]).total_count_ <= (tree[j as (usize)]).total_count_ {
            right = i;
            i = i.wrapping_add(1 as (usize));
          } else {
            right = j;
            j = j.wrapping_add(1 as (usize));
          }
          // the sentinel node becomes the parent node
          let j_end: usize = (2i32 as (usize)).wrapping_mul(n).wrapping_sub(k);
          (tree[j_end as usize]).total_count_ =
            (tree[left as (usize)])
              .total_count_
              .wrapping_add((tree[right as (usize)]).total_count_);
          (tree[j_end as (usize)]).index_left_ = left as (i16);
          (tree[j_end as (usize)]).index_right_or_value_ = right as (i16);
          // add back the last sentinel node
          tree[j_end.wrapping_add(1i32 as (usize)) as (usize)] = sentinel.clone();
          k = k.wrapping_sub(1 as (usize));
          continue 'loop5;
        } else {
          break 'loop5;
        }
      }
      if BrotliSetDepth((2i32 as (usize)).wrapping_mul(n).wrapping_sub(1i32 as (usize)) as (i32),
                        tree,
                        depth,
                        tree_limit) {
        // We need to pack the Huffman tree in tree_limit bits. If this was
        // not successful, add fake entities to the lowest values and retry.
        break 'loop1;
      } else {
        count_limit = count_limit.wrapping_mul(2i32 as (u32));
        continue 'loop1;
      }
    }
  }
}
pub fn BrotliOptimizeHuffmanCountsForRle(mut length: usize,
                                         mut counts: &mut [u32],
                                         mut good_for_rle: &mut [u8]) {
  let mut nonzero_count: usize = 0i32 as (usize);
  let mut stride: usize;
  let mut limit: usize;
  let mut sum: usize;
  let streak_limit: usize = 1240i32 as (usize);
  // lets make the Huffman code more compatible with RLE encoding.
  let mut i: usize;
  i = 0i32 as (usize);
  'loop1: loop {
    if i < length {
      if counts[i as usize] != 0 {
        nonzero_count = nonzero_count.wrapping_add(1 as (usize));
      }
      i = i.wrapping_add(1 as (usize));
      continue 'loop1;
    } else {
      break 'loop1;
    }
  }
  if nonzero_count < 16i32 as (usize) {
  } else {
    'loop3: loop {
      if length != 0i32 as (usize) &&
         (counts[length.wrapping_sub(1i32 as (usize)) as (usize)] == 0i32 as (u32)) {
        length = length.wrapping_sub(1 as (usize));
        continue 'loop3;
      } else {
        break 'loop3;
      }
    }
    if length == 0i32 as (usize) {
      // all zeros (return)
    } else {
      // now counts[0..length - 1] does not have trailing zeros.
      let mut nonzeros: usize = 0i32 as (usize);
      let mut smallest_nonzero: u32 = (1i32 << 30i32) as (u32);
      i = 0i32 as (usize);
      'loop6: loop {
        if i < length {
          if counts[i as usize] != 0i32 as (u32) {
            nonzeros = nonzeros.wrapping_add(1 as (usize));
            if smallest_nonzero > counts[i as usize] {
              smallest_nonzero = counts[i as usize];
            }
          }
          i = i.wrapping_add(1 as (usize));
          continue 'loop6;
        } else {
          break 'loop6;
        }
      }
      if nonzeros < 5i32 as (usize) {
        // small histogram will model it well.
        //return
      } else {
        if smallest_nonzero < 4i32 as (u32) {
          let zeros: usize = length.wrapping_sub(nonzeros);
          if zeros < 6i32 as (usize) {
            i = 1i32 as (usize);
            'loop11: loop {
              if i < length.wrapping_sub(1i32 as (usize)) {
                if counts[i.wrapping_sub(1i32 as (usize)) as (usize)] != 0i32 as (u32) &&
                   (counts[i as (usize)] == 0i32 as (u32)) &&
                   (counts[i.wrapping_add(1i32 as (usize)) as (usize)] != 0i32 as (u32)) {
                  counts[i as usize] = 1i32 as (u32);
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop11;
              } else {
                break 'loop11;
              }
            }
          }
        }
        if nonzeros < 28i32 as (usize) {
        } else {
          // 2) Let's mark all population counts that already can be encoded
          // with an RLE code
          for rle_elem in good_for_rle[..length].iter_mut() {
            *rle_elem = 0; // memset
          }
          /* Let's not spoil any of the existing good RLE codes.
                       Mark any seq of 0's that is longer as 5 as a good_for_rle.
                       Mark any seq of non-0's that is longer as 7 as a good_for_rle. */
          let mut symbol: u32 = counts[0i32 as usize];
          let mut step: usize = 0i32 as (usize);
          i = 0i32 as (usize);
          'loop14: loop {
            if i <= length {
              if i == length || counts[i as usize] != symbol {
                if symbol == 0i32 as (u32) && (step >= 5i32 as (usize)) ||
                   symbol != 0i32 as (u32) && (step >= 7i32 as (usize)) {
                  let mut k: usize;
                  k = 0i32 as (usize);
                  'loop43: loop {
                    if k < step {
                      good_for_rle[i.wrapping_sub(k).wrapping_sub(1i32 as (usize)) as (usize)] =
                        1i32 as (u8);
                      k = k.wrapping_add(1 as (usize));
                      continue 'loop43;
                    } else {
                      break 'loop43;
                    }
                  }
                }
                step = 1i32 as (usize);
                if i != length {
                  symbol = counts[i as usize];
                }
              } else {
                step = step.wrapping_add(1 as (usize));
              }
              i = i.wrapping_add(1 as (usize));
              continue 'loop14;
            } else {
              break 'loop14;
            }
          }
          /* 3) Let's replace those population counts that lead to more RLE codes.
                          Math here is in 24.8 fixed point representation. */
          stride = 0i32 as (usize);
          limit = (256i32 as (u32))
            .wrapping_mul((counts[0i32 as usize])
                            .wrapping_add(counts[1i32 as usize])
                            .wrapping_add(counts[2i32 as usize]))
            .wrapping_div(3i32 as (u32))
            .wrapping_add(420i32 as (u32)) as (usize);
          sum = 0i32 as (usize);
          i = 0i32 as (usize);
          'loop16: loop {
            if i <= length {
              if i == length || good_for_rle[i as (usize)] != 0 ||
                 i != 0i32 as (usize) &&
                 (good_for_rle[i.wrapping_sub(1i32 as (usize)) as (usize)] != 0) ||
                 ((256i32 as (u32)).wrapping_mul(counts[i as (usize)]) as (usize))
                   .wrapping_sub(limit)
                   .wrapping_add(streak_limit) >=
                 (2i32 as (usize)).wrapping_mul(streak_limit) {
                if stride >= 4i32 as (usize) ||
                   stride >= 3i32 as (usize) && (sum == 0i32 as (usize)) {
                  let mut k: usize;
                  // the stride must end, collapse what we have, if we have enough (4).
                  let mut count: usize = sum.wrapping_add(stride.wrapping_div(2i32 as (usize)))
                    .wrapping_div(stride);
                  if count == 0i32 as (usize) {
                    count = 1i32 as (usize);
                  }
                  if sum == 0i32 as (usize) {
                    // Don't make an all zeros stride to be upgraded to ones.
                    count = 0i32 as (usize);
                  }
                  k = 0i32 as (usize);
                  'loop25: loop {
                    if k < stride {
                      // we don't want to change value at counts[i]
                      // that is already belogning to the next stride. Thus - 1.
                      counts[i.wrapping_sub(k).wrapping_sub(1i32 as (usize)) as (usize)] = count as
                                                                                           (u32);
                      k = k.wrapping_add(1 as (usize));
                      continue 'loop25;
                    } else {
                      break 'loop25;
                    }
                  }
                }
                stride = 0i32 as (usize);
                sum = 0i32 as (usize);
                if i < length.wrapping_sub(2i32 as (usize)) {
                  // All interesting strides have a count of at least 4,
                  // at least when non-zeros
                  limit = (256i32 as (u32))
                    .wrapping_mul((counts[i as usize])
                                    .wrapping_add(counts[i.wrapping_add(1i32 as (usize)) as
                                                  (usize)])
                                    .wrapping_add(counts[i.wrapping_add(2i32 as (usize)) as
                                                  (usize)]))
                    .wrapping_div(3i32 as (u32))
                    .wrapping_add(420i32 as (u32)) as (usize);
                } else if i < length {
                  limit = (256i32 as (u32)).wrapping_mul(counts[i as usize]) as (usize);
                } else {
                  limit = 0i32 as (usize);
                }
              }
              stride = stride.wrapping_add(1 as (usize));
              if i != length {
                sum = sum.wrapping_add(counts[i as usize] as (usize));
                if stride >= 4i32 as (usize) {
                  limit = (256i32 as (usize))
                    .wrapping_mul(sum)
                    .wrapping_add(stride.wrapping_div(2i32 as (usize)))
                    .wrapping_div(stride);
                }
                if stride == 4i32 as (usize) {
                  limit = limit.wrapping_add(120i32 as (usize));
                }
              }
              i = i.wrapping_add(1 as (usize));
              continue 'loop16;
            } else {
              break 'loop16;
            }
          }
        }
      }
    }
  }
}


pub fn DecideOverRleUse(depth: &[u8],
                        length: usize,
                        mut use_rle_for_non_zero: &mut i32,
                        mut use_rle_for_zero: &mut i32) {
  let mut total_reps_zero: usize = 0i32 as (usize);
  let mut total_reps_non_zero: usize = 0i32 as (usize);
  let mut count_reps_zero: usize = 1i32 as (usize);
  let mut count_reps_non_zero: usize = 1i32 as (usize);
  let mut i: usize;
  i = 0i32 as (usize);
  'loop1: loop {
    if i < length {
      let value: u8 = depth[i as (usize)];
      let mut reps: usize = 1i32 as (usize);
      let mut k: usize;
      k = i.wrapping_add(1i32 as (usize));
      'loop4: loop {
        if k < length && (depth[k as (usize)] as (i32) == value as (i32)) {
          reps = reps.wrapping_add(1 as (usize));
          k = k.wrapping_add(1 as (usize));
          continue 'loop4;
        } else {
          break 'loop4;
        }
      }
      if reps >= 3i32 as (usize) && (value as (i32) == 0i32) {
        total_reps_zero = total_reps_zero.wrapping_add(reps);
        count_reps_zero = count_reps_zero.wrapping_add(1 as (usize));
      }
      if reps >= 4i32 as (usize) && (value as (i32) != 0i32) {
        total_reps_non_zero = total_reps_non_zero.wrapping_add(reps);
        count_reps_non_zero = count_reps_non_zero.wrapping_add(1 as (usize));
      }
      i = i.wrapping_add(reps);
      continue 'loop1;
    } else {
      break 'loop1;
    }
  }
  *use_rle_for_non_zero = if !!(total_reps_non_zero >
                                count_reps_non_zero.wrapping_mul(2i32 as (usize))) {
    1i32
  } else {
    0i32
  };
  *use_rle_for_zero = if !!(total_reps_zero > count_reps_zero.wrapping_mul(2i32 as (usize))) {
    1i32
  } else {
    0i32
  };
}



fn Reverse(mut v: &mut [u8], mut start: usize, mut end: usize) {
  end = end.wrapping_sub(1 as (usize));
  'loop1: loop {
    if start < end {
      let tmp: u8 = v[start as (usize)];
      v[start as (usize)] = v[end as (usize)];
      v[end as (usize)] = tmp;
      start = start.wrapping_add(1 as (usize));
      end = end.wrapping_sub(1 as (usize));
      continue 'loop1;
    } else {
      break 'loop1;
    }
  }
}


fn BrotliWriteHuffmanTreeRepetitions(previous_value: u8,
                                     value: u8,
                                     mut repetitions: usize,
                                     mut tree_size: &mut usize,
                                     mut tree: &mut [u8],
                                     mut extra_bits_data: &mut [u8]) {
  0i32;
  if previous_value as (i32) != value as (i32) {
    tree[*tree_size as usize] = value;
    extra_bits_data[*tree_size as usize] = 0i32 as (u8);
    *tree_size = (*tree_size).wrapping_add(1 as (usize));
    repetitions = repetitions.wrapping_sub(1 as (usize));
  }
  if repetitions == 7i32 as (usize) {
    tree[*tree_size as usize] = value;
    extra_bits_data[*tree_size as usize] = 0i32 as (u8);
    *tree_size = (*tree_size).wrapping_add(1 as (usize));
    repetitions = repetitions.wrapping_sub(1 as (usize));
  }
  if repetitions < 3i32 as (usize) {
    let mut i: usize;
    i = 0i32 as (usize);
    'loop11: loop {
      if i < repetitions {
        tree[*tree_size as usize] = value;
        extra_bits_data[*tree_size as usize] = 0i32 as (u8);
        *tree_size = (*tree_size).wrapping_add(1 as (usize));
        i = i.wrapping_add(1 as (usize));
        continue 'loop11;
      } else {
        break 'loop11;
      }
    }
  } else {
    let start: usize = *tree_size;
    repetitions = repetitions.wrapping_sub(3i32 as (usize));
    'loop6: loop {
      if 1i32 != 0 {
        tree[*tree_size as usize] = 16i32 as (u8);
        extra_bits_data[*tree_size as (usize)] = (repetitions & 0x3i32 as (usize)) as (u8);
        *tree_size = (*tree_size).wrapping_add(1 as (usize));
        repetitions = repetitions >> 2i32;
        if repetitions == 0i32 as (usize) {
          break 'loop6;
        } else {
          repetitions = repetitions.wrapping_sub(1 as (usize));
          continue 'loop6;
        }
      } else {
        break 'loop6;
      }
    }
    Reverse(tree, start, *tree_size);
    Reverse(extra_bits_data, start, *tree_size);
  }
}


fn BrotliWriteHuffmanTreeRepetitionsZeros(mut repetitions: usize,
                                          mut tree_size: &mut usize,
                                          mut tree: &mut [u8],
                                          mut extra_bits_data: &mut [u8]) {
  if repetitions == 11i32 as (usize) {
    tree[*tree_size as usize] = 0i32 as (u8);
    extra_bits_data[*tree_size as usize] = 0i32 as (u8);
    *tree_size = (*tree_size).wrapping_add(1 as (usize));
    repetitions = repetitions.wrapping_sub(1 as (usize));
  }
  if repetitions < 3i32 as (usize) {
    let mut i: usize;
    i = 0i32 as (usize);
    'loop9: loop {
      if i < repetitions {
        tree[*tree_size as usize] = 0i32 as (u8);
        extra_bits_data[*tree_size as usize] = 0i32 as (u8);
        *tree_size = (*tree_size).wrapping_add(1 as (usize));
        i = i.wrapping_add(1 as (usize));
        continue 'loop9;
      } else {
        break 'loop9;
      }
    }
  } else {
    let mut start: usize = *tree_size;
    repetitions = repetitions.wrapping_sub(3i32 as (usize));
    'loop4: loop {
      if 1i32 != 0 {
        tree[*tree_size as usize] = 17i32 as (u8);
        extra_bits_data[*tree_size as (usize)] = (repetitions & 0x7i32 as (usize)) as (u8);
        *tree_size = (*tree_size).wrapping_add(1 as (usize));
        repetitions = repetitions >> 3i32;
        if repetitions == 0i32 as (usize) {
          break 'loop4;
        } else {
          repetitions = repetitions.wrapping_sub(1 as (usize));
          continue 'loop4;
        }
      } else {
        break 'loop4;
      }
    }
    Reverse(tree, start, *tree_size);
    Reverse(extra_bits_data, start, *tree_size);
  }
}


pub fn BrotliWriteHuffmanTree(depth: &[u8],
                              length: usize,
                              mut tree_size: &mut usize,
                              mut tree: &mut [u8],
                              mut extra_bits_data: &mut [u8]) {
  let mut previous_value: u8 = 8i32 as (u8);
  let mut i: usize;
  let mut use_rle_for_non_zero: i32 = 0i32;
  let mut use_rle_for_zero: i32 = 0i32;
  // throw away trailing zeros
  let mut new_length: usize = length;
  i = 0i32 as (usize);
  'loop1: loop {
    if i < length {
      if depth[length.wrapping_sub(i).wrapping_sub(1i32 as (usize)) as (usize)] as (i32) == 0i32 {
        new_length = new_length.wrapping_sub(1 as (usize));
        i = i.wrapping_add(1 as (usize));
        continue 'loop1;
      } else {
        break 'loop1;
      }
    } else {
      break 'loop1;
    }
  }
  // first gather statistics on if it is a good idea to do RLE
  if length > 50 {
    // find RLE coding for longer codes.
    // Shorter codes seem to not benefit from RLE.
    DecideOverRleUse(depth,
                     new_length,
                     &mut use_rle_for_non_zero,
                     &mut use_rle_for_zero);
  }
  // Actual RLE coding.
  i = 0i32 as (usize);
  'loop6: loop {
    if i < new_length {
      let value: u8 = depth[i as (usize)];
      let mut reps: usize = 1i32 as (usize);
      if value as (i32) != 0i32 && (use_rle_for_non_zero != 0) ||
         value as (i32) == 0i32 && (use_rle_for_zero != 0) {
        let mut k: usize;
        k = i.wrapping_add(1i32 as (usize));
        'loop10: loop {
          if k < new_length && (depth[k as (usize)] as (i32) == value as (i32)) {
            reps = reps.wrapping_add(1 as (usize));
            k = k.wrapping_add(1 as (usize));
            continue 'loop10;
          } else {
            break 'loop10;
          }
        }
      }
      if value as (i32) == 0i32 {
        BrotliWriteHuffmanTreeRepetitionsZeros(reps, tree_size, tree, extra_bits_data);
      } else {
        BrotliWriteHuffmanTreeRepetitions(previous_value,
                                          value,
                                          reps,
                                          tree_size,
                                          tree,
                                          extra_bits_data);
        previous_value = value;
      }
      i = i.wrapping_add(reps);
      continue 'loop6;
    } else {
      break 'loop6;
    }
  }
}



fn BrotliReverseBits(num_bits: usize, mut bits: u16) -> u16 {
  static kLut: [usize; 16] = [0x0i32 as (usize),
                              0x8i32 as (usize),
                              0x4i32 as (usize),
                              0xci32 as (usize),
                              0x2i32 as (usize),
                              0xai32 as (usize),
                              0x6i32 as (usize),
                              0xei32 as (usize),
                              0x1i32 as (usize),
                              0x9i32 as (usize),
                              0x5i32 as (usize),
                              0xdi32 as (usize),
                              0x3i32 as (usize),
                              0xbi32 as (usize),
                              0x7i32 as (usize),
                              0xfi32 as (usize)];
  let mut retval: usize = kLut[(bits as (i32) & 0xfi32) as (usize)];
  let mut i: usize;
  i = 4i32 as (usize);
  'loop1: loop {
    if i < num_bits {
      retval = retval << 4i32;
      bits = (bits as (i32) >> 4i32) as (u16);
      retval = retval | kLut[(bits as (i32) & 0xfi32) as (usize)];
      i = i.wrapping_add(4i32 as (usize));
      continue 'loop1;
    } else {
      break 'loop1;
    }
  }
  retval = retval >> ((0i32 as (usize)).wrapping_sub(num_bits) & 0x3i32 as (usize));
  retval as (u16)
}
const MAX_HUFFMAN_BITS: usize = 16;
pub fn BrotliConvertBitDepthsToSymbols(depth: &[u8], len: usize, mut bits: &mut [u16]) {
  /* In Brotli, all bit depths are [1..15]
     0 bit depth means that the symbol does not exist. */

  let mut bl_count: [u16; MAX_HUFFMAN_BITS] = [0; MAX_HUFFMAN_BITS];
  let mut next_code: [u16; MAX_HUFFMAN_BITS] = [0; MAX_HUFFMAN_BITS];
  let mut i: usize;
  let mut code: i32 = 0i32;
  i = 0i32 as (usize);
  'loop1: loop {
    if i < len {
      {
        let _rhs = 1;
        let _lhs = &mut bl_count[depth[i as (usize)] as (usize)];
        *_lhs = (*_lhs as (i32) + _rhs) as (u16);
      }
      i = i.wrapping_add(1 as (usize));
      continue 'loop1;
    } else {
      break 'loop1;
    }
  }
  bl_count[0i32 as (usize)] = 0i32 as (u16);
  next_code[0i32 as (usize)] = 0i32 as (u16);
  i = 1i32 as (usize);
  'loop3: loop {
    if i < MAX_HUFFMAN_BITS {
      code = code + bl_count[i.wrapping_sub(1i32 as (usize))] as (i32) << 1i32;
      next_code[i] = code as (u16);
      i = i.wrapping_add(1 as (usize));
      continue 'loop3;
    } else {
      break 'loop3;
    }
  }
  i = 0i32 as (usize);
  'loop5: loop {
    if i < len {
      if depth[i as (usize)] != 0 {
        bits[i as (usize)] = BrotliReverseBits(depth[i as (usize)] as (usize), {
          let _rhs = 1;
          let _lhs = &mut next_code[depth[i as (usize)] as (usize)];
          let _old = *_lhs;
          *_lhs = (*_lhs as (i32) + _rhs) as (u16);
          _old
        });
      }
      i = i.wrapping_add(1 as (usize));
      continue 'loop5;
    } else {
      break 'loop5;
    }
  }
}
