extern {
    fn memset(
        __b : *mut ::std::os::raw::c_void, __c : i32, __len : usize
    ) -> *mut ::std::os::raw::c_void;
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HuffmanTree {
    pub total_count_ : u32,
    pub index_left_ : i16,
    pub index_right_or_value_ : i16,
}

#[no_mangle]
pub unsafe extern fn BrotliSetDepth(
    mut p0 : i32,
    mut pool : *mut HuffmanTree,
    mut depth : *mut u8,
    mut max_depth : i32
) -> i32 {
    let mut stack : [i32; 16];
    let mut level : i32 = 0i32;
    let mut p : i32 = p0;
    0i32;
    stack[0i32 as (usize)] = -1i32;
    while 1i32 != 0 {
        if (*pool.offset(p as (isize))).index_left_ as (i32) >= 0i32 {
            level = level + 1;
            if level > max_depth {
                return 0i32;
            }
            stack[level as (usize)] = (*pool.offset(
                                            p as (isize)
                                        )).index_right_or_value_ as (i32);
            p = (*pool.offset(p as (isize))).index_left_ as (i32);
            {
                if 1337i32 != 0 {
                    continue;
                }
            }
        } else {
            *depth.offset(
                 (*pool.offset(p as (isize))).index_right_or_value_ as (isize)
             ) = level as (u8);
        }
        while level >= 0i32 && (stack[level as (usize)] == -1i32) {
            level = level - 1;
        }
        if level < 0i32 {
            return 1i32;
        }
        p = stack[level as (usize)];
        stack[level as (usize)] = -1i32;
    }
}

unsafe extern fn InitHuffmanTree(
    mut self : *mut HuffmanTree,
    mut count : u32,
    mut left : i16,
    mut right : i16
) {
    (*self).total_count_ = count;
    (*self).index_left_ = left;
    (*self).index_right_or_value_ = right;
}

unsafe extern fn brotli_max_uint32_t(
    mut a : u32, mut b : u32
) -> u32 {
    if a > b { a } else { b }
}

unsafe extern fn SortHuffmanTreeItems(
    mut items : *mut HuffmanTree,
    n : usize,
    mut
    comparator
    :
    unsafe extern fn(*const HuffmanTree, *const HuffmanTree) -> i32
) {
    static mut gaps
        : [usize; 6]
        = [   132i32 as (usize),
              57i32 as (usize),
              23i32 as (usize),
              10i32 as (usize),
              4i32 as (usize),
              1i32 as (usize)
          ];
    if n < 13i32 as (usize) {
        let mut i : usize;
        i = 1i32 as (usize);
        while i < n {
            {
                let mut tmp : HuffmanTree = *items.offset(i as (isize));
                let mut k : usize = i;
                let mut j : usize = i.wrapping_sub(1i32 as (usize));
                while comparator(
                          &mut tmp as (*mut HuffmanTree) as (*const HuffmanTree),
                          &mut *items.offset(
                                    j as (isize)
                                ) as (*mut HuffmanTree) as (*const HuffmanTree)
                      ) != 0 {
                    *items.offset(k as (isize)) = *items.offset(j as (isize));
                    k = j;
                    if {
                           let _old = j;
                           j = j.wrapping_sub(1 as (usize));
                           _old
                       } == 0 {
                        if 1337i32 != 0 {
                            break;
                        }
                    }
                }
                *items.offset(k as (isize)) = tmp;
            }
            i = i.wrapping_add(1 as (usize));
        }
    } else {
        let mut g : i32 = if n < 57i32 as (usize) { 2i32 } else { 0i32 };
        while g < 6i32 {
            {
                let mut gap : usize = gaps[g as (usize)];
                let mut i : usize;
                i = gap;
                while i < n {
                    {
                        let mut j : usize = i;
                        let mut tmp : HuffmanTree = *items.offset(i as (isize));
                        while j >= gap && (comparator(
                                               &mut tmp as (*mut HuffmanTree) as (*const HuffmanTree),
                                               &mut *items.offset(
                                                         j.wrapping_sub(gap) as (isize)
                                                     ) as (*mut HuffmanTree) as (*const HuffmanTree)
                                           ) != 0) {
                            {
                                *items.offset(j as (isize)) = *items.offset(
                                                                   j.wrapping_sub(gap) as (isize)
                                                               );
                            }
                            j = j.wrapping_sub(gap);
                        }
                        *items.offset(j as (isize)) = tmp;
                    }
                    i = i.wrapping_add(1 as (usize));
                }
            }
            g = g + 1;
        }
    }
}

unsafe extern fn SortHuffmanTree(
    mut v0 : *const HuffmanTree, mut v1 : *const HuffmanTree
) -> i32 {
    if (*v0).total_count_ != (*v1).total_count_ {
        return
            if !!((*v0).total_count_ < (*v1).total_count_) {
                1i32
            } else {
                0i32
            };
    }
    if !!((*v0).index_right_or_value_ as (i32) > (*v1).index_right_or_value_ as (i32)) {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern fn BrotliCreateHuffmanTree(
    mut data : *const u32,
    length : usize,
    tree_limit : i32,
    mut tree : *mut HuffmanTree,
    mut depth : *mut u8
) {
    let mut count_limit : u32;
    let mut sentinel : HuffmanTree;
    InitHuffmanTree(
        &mut sentinel as (*mut HuffmanTree),
        !(0i32 as (u32)),
        -1i32 as (i16),
        -1i32 as (i16)
    );
    count_limit = 1i32 as (u32);
    'break1: loop {
        {
            let mut n : usize = 0i32 as (usize);
            let mut i : usize;
            let mut j : usize;
            let mut k : usize;
            i = length;
            while i != 0i32 as (usize) {
                i = i.wrapping_sub(1 as (usize));
                if *data.offset(i as (isize)) != 0 {
                    let count
                        : u32
                        = brotli_max_uint32_t(*data.offset(i as (isize)),count_limit);
                    InitHuffmanTree(
                        &mut *tree.offset(
                                  {
                                      let _old = n;
                                      n = n.wrapping_add(1 as (usize));
                                      _old
                                  } as (isize)
                              ) as (*mut HuffmanTree),
                        count,
                        -1i32 as (i16),
                        i as (i16)
                    );
                }
            }
            if n == 1i32 as (usize) {
                *depth.offset(
                     (*tree.offset(0i32 as (isize))).index_right_or_value_ as (isize)
                 ) = 1i32 as (u8);
                {
                    if 1337i32 != 0 {
                        break 'break1;
                    }
                }
            }
            SortHuffmanTreeItems(tree,n,SortHuffmanTree);
            *tree.offset(n as (isize)) = sentinel;
            *tree.offset(
                 n.wrapping_add(1i32 as (usize)) as (isize)
             ) = sentinel;
            i = 0i32 as (usize);
            j = n.wrapping_add(1i32 as (usize));
            k = n.wrapping_sub(1i32 as (usize));
            while k != 0i32 as (usize) {
                {
                    let mut left : usize;
                    let mut right : usize;
                    if (*tree.offset(i as (isize))).total_count_ <= (*tree.offset(
                                                                          j as (isize)
                                                                      )).total_count_ {
                        left = i;
                        i = i.wrapping_add(1 as (usize));
                    } else {
                        left = j;
                        j = j.wrapping_add(1 as (usize));
                    }
                    if (*tree.offset(i as (isize))).total_count_ <= (*tree.offset(
                                                                          j as (isize)
                                                                      )).total_count_ {
                        right = i;
                        i = i.wrapping_add(1 as (usize));
                    } else {
                        right = j;
                        j = j.wrapping_add(1 as (usize));
                    }
                    {
                        let mut j_end
                            : usize
                            = (2i32 as (usize)).wrapping_mul(n).wrapping_sub(k);
                        (*tree.offset(j_end as (isize))).total_count_ = (*tree.offset(
                                                                              left as (isize)
                                                                          )).total_count_.wrapping_add(
                                                                            (*tree.offset(
                                                                                  right as (isize)
                                                                              )).total_count_
                                                                        );
                        (*tree.offset(j_end as (isize))).index_left_ = left as (i16);
                        (*tree.offset(
                              j_end as (isize)
                          )).index_right_or_value_ = right as (i16);
                        *tree.offset(
                             j_end.wrapping_add(1i32 as (usize)) as (isize)
                         ) = sentinel;
                    }
                }
                k = k.wrapping_sub(1 as (usize));
            }
            if BrotliSetDepth(
                   (2i32 as (usize)).wrapping_mul(n).wrapping_sub(
                       1i32 as (usize)
                   ) as (i32),
                   &mut *tree.offset(0i32 as (isize)) as (*mut HuffmanTree),
                   depth,
                   tree_limit
               ) != 0 {
                if 1337i32 != 0 {
                    break 'break1;
                }
            }
        }
        count_limit = count_limit.wrapping_mul(2i32 as (u32));
    }
}

#[no_mangle]
pub unsafe extern fn BrotliOptimizeHuffmanCountsForRle(
    mut length : usize,
    mut counts : *mut u32,
    mut good_for_rle : *mut u8
) {
    let mut nonzero_count : usize = 0i32 as (usize);
    let mut stride : usize;
    let mut limit : usize;
    let mut sum : usize;
    let streak_limit : usize = 1240i32 as (usize);
    let mut i : usize;
    i = 0i32 as (usize);
    while i < length {
        {
            if *counts.offset(i as (isize)) != 0 {
                nonzero_count = nonzero_count.wrapping_add(1 as (usize));
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    if nonzero_count < 16i32 as (usize) {
        return;
    }
    while length != 0i32 as (usize) && (*counts.offset(
                                             length.wrapping_sub(1i32 as (usize)) as (isize)
                                         ) == 0i32 as (u32)) {
        length = length.wrapping_sub(1 as (usize));
    }
    if length == 0i32 as (usize) {
        return;
    }
    {
        let mut nonzeros : usize = 0i32 as (usize);
        let mut smallest_nonzero : u32 = (1i32 << 30i32) as (u32);
        i = 0i32 as (usize);
        while i < length {
            {
                if *counts.offset(i as (isize)) != 0i32 as (u32) {
                    nonzeros = nonzeros.wrapping_add(1 as (usize));
                    if smallest_nonzero > *counts.offset(i as (isize)) {
                        smallest_nonzero = *counts.offset(i as (isize));
                    }
                }
            }
            i = i.wrapping_add(1 as (usize));
        }
        if nonzeros < 5i32 as (usize) {
            return;
        }
        if smallest_nonzero < 4i32 as (u32) {
            let mut zeros : usize = length.wrapping_sub(nonzeros);
            if zeros < 6i32 as (usize) {
                i = 1i32 as (usize);
                while i < length.wrapping_sub(1i32 as (usize)) {
                    {
                        if *counts.offset(
                                i.wrapping_sub(1i32 as (usize)) as (isize)
                            ) != 0i32 as (u32) && (*counts.offset(
                                                        i as (isize)
                                                    ) == 0i32 as (u32)) && (*counts.offset(
                                                                                 i.wrapping_add(
                                                                                     1i32 as (usize)
                                                                                 ) as (isize)
                                                                             ) != 0i32 as (u32)) {
                            *counts.offset(i as (isize)) = 1i32 as (u32);
                        }
                    }
                    i = i.wrapping_add(1 as (usize));
                }
            }
        }
        if nonzeros < 28i32 as (usize) {
            return;
        }
    }
    memset(good_for_rle as (*mut ::std::os::raw::c_void),0i32,length);
    {
        let mut symbol : u32 = *counts.offset(0i32 as (isize));
        let mut step : usize = 0i32 as (usize);
        i = 0i32 as (usize);
        while i <= length {
            {
                if i == length || *counts.offset(i as (isize)) != symbol {
                    if symbol == 0i32 as (u32) && (step >= 5i32 as (usize)) || symbol != 0i32 as (u32) && (step >= 7i32 as (usize)) {
                        let mut k : usize;
                        k = 0i32 as (usize);
                        while k < step {
                            {
                                *good_for_rle.offset(
                                     i.wrapping_sub(k).wrapping_sub(1i32 as (usize)) as (isize)
                                 ) = 1i32 as (u8);
                            }
                            k = k.wrapping_add(1 as (usize));
                        }
                    }
                    step = 1i32 as (usize);
                    if i != length {
                        symbol = *counts.offset(i as (isize));
                    }
                } else {
                    step = step.wrapping_add(1 as (usize));
                }
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    stride = 0i32 as (usize);
    limit = (256i32 as (u32)).wrapping_mul(
                (*counts.offset(0i32 as (isize))).wrapping_add(
                    *counts.offset(1i32 as (isize))
                ).wrapping_add(
                    *counts.offset(2i32 as (isize))
                )
            ).wrapping_div(
                3i32 as (u32)
            ).wrapping_add(
                420i32 as (u32)
            ) as (usize);
    sum = 0i32 as (usize);
    i = 0i32 as (usize);
    while i <= length {
        {
            if i == length || *good_for_rle.offset(
                                   i as (isize)
                               ) != 0 || i != 0i32 as (usize) && (*good_for_rle.offset(
                                                                       i.wrapping_sub(
                                                                           1i32 as (usize)
                                                                       ) as (isize)
                                                                   ) != 0) || ((256i32 as (u32)).wrapping_mul(
                                                                                   *counts.offset(
                                                                                        i as (isize)
                                                                                    )
                                                                               ) as (usize)).wrapping_sub(
                                                                                  limit
                                                                              ).wrapping_add(
                                                                                  streak_limit
                                                                              ) >= (2i32 as (usize)).wrapping_mul(
                                                                                       streak_limit
                                                                                   ) {
                if stride >= 4i32 as (usize) || stride >= 3i32 as (usize) && (sum == 0i32 as (usize)) {
                    let mut k : usize;
                    let mut count
                        : usize
                        = sum.wrapping_add(
                              stride.wrapping_div(2i32 as (usize))
                          ).wrapping_div(
                              stride
                          );
                    if count == 0i32 as (usize) {
                        count = 1i32 as (usize);
                    }
                    if sum == 0i32 as (usize) {
                        count = 0i32 as (usize);
                    }
                    k = 0i32 as (usize);
                    while k < stride {
                        {
                            *counts.offset(
                                 i.wrapping_sub(k).wrapping_sub(1i32 as (usize)) as (isize)
                             ) = count as (u32);
                        }
                        k = k.wrapping_add(1 as (usize));
                    }
                }
                stride = 0i32 as (usize);
                sum = 0i32 as (usize);
                if i < length.wrapping_sub(2i32 as (usize)) {
                    limit = (256i32 as (u32)).wrapping_mul(
                                (*counts.offset(i as (isize))).wrapping_add(
                                    *counts.offset(i.wrapping_add(1i32 as (usize)) as (isize))
                                ).wrapping_add(
                                    *counts.offset(i.wrapping_add(2i32 as (usize)) as (isize))
                                )
                            ).wrapping_div(
                                3i32 as (u32)
                            ).wrapping_add(
                                420i32 as (u32)
                            ) as (usize);
                } else if i < length {
                    limit = (256i32 as (u32)).wrapping_mul(
                                *counts.offset(i as (isize))
                            ) as (usize);
                } else {
                    limit = 0i32 as (usize);
                }
            }
            stride = stride.wrapping_add(1 as (usize));
            if i != length {
                sum = sum.wrapping_add(*counts.offset(i as (isize)) as (usize));
                if stride >= 4i32 as (usize) {
                    limit = (256i32 as (usize)).wrapping_mul(sum).wrapping_add(
                                stride.wrapping_div(2i32 as (usize))
                            ).wrapping_div(
                                stride
                            );
                }
                if stride == 4i32 as (usize) {
                    limit = limit.wrapping_add(120i32 as (usize));
                }
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn DecideOverRleUse(
    mut depth : *const u8,
    length : usize,
    mut use_rle_for_non_zero : *mut i32,
    mut use_rle_for_zero : *mut i32
) {
    let mut total_reps_zero : usize = 0i32 as (usize);
    let mut total_reps_non_zero : usize = 0i32 as (usize);
    let mut count_reps_zero : usize = 1i32 as (usize);
    let mut count_reps_non_zero : usize = 1i32 as (usize);
    let mut i : usize;
    i = 0i32 as (usize);
    while i < length {
        let value : u8 = *depth.offset(i as (isize));
        let mut reps : usize = 1i32 as (usize);
        let mut k : usize;
        k = i.wrapping_add(1i32 as (usize));
        while k < length && (*depth.offset(
                                  k as (isize)
                              ) as (i32) == value as (i32)) {
            {
                reps = reps.wrapping_add(1 as (usize));
            }
            k = k.wrapping_add(1 as (usize));
        }
        if reps >= 3i32 as (usize) && (value as (i32) == 0i32) {
            total_reps_zero = total_reps_zero.wrapping_add(reps);
            count_reps_zero = count_reps_zero.wrapping_add(1 as (usize));
        }
        if reps >= 4i32 as (usize) && (value as (i32) != 0i32) {
            total_reps_non_zero = total_reps_non_zero.wrapping_add(reps);
            count_reps_non_zero = count_reps_non_zero.wrapping_add(
                                      1 as (usize)
                                  );
        }
        i = i.wrapping_add(reps);
    }
    *use_rle_for_non_zero = if !!(total_reps_non_zero > count_reps_non_zero.wrapping_mul(
                                                            2i32 as (usize)
                                                        )) {
                                1i32
                            } else {
                                0i32
                            };
    *use_rle_for_zero = if !!(total_reps_zero > count_reps_zero.wrapping_mul(
                                                    2i32 as (usize)
                                                )) {
                            1i32
                        } else {
                            0i32
                        };
}

unsafe extern fn Reverse(
    mut v : *mut u8, mut start : usize, mut end : usize
) {
    end = end.wrapping_sub(1 as (usize));
    while start < end {
        let mut tmp : u8 = *v.offset(start as (isize));
        *v.offset(start as (isize)) = *v.offset(end as (isize));
        *v.offset(end as (isize)) = tmp;
        start = start.wrapping_add(1 as (usize));
        end = end.wrapping_sub(1 as (usize));
    }
}

unsafe extern fn BrotliWriteHuffmanTreeRepetitionsZeros(
    mut repetitions : usize,
    mut tree_size : *mut usize,
    mut tree : *mut u8,
    mut extra_bits_data : *mut u8
) {
    if repetitions == 11i32 as (usize) {
        *tree.offset(*tree_size as (isize)) = 0i32 as (u8);
        *extra_bits_data.offset(*tree_size as (isize)) = 0i32 as (u8);
        *tree_size = (*tree_size).wrapping_add(1 as (usize));
        repetitions = repetitions.wrapping_sub(1 as (usize));
    }
    if repetitions < 3i32 as (usize) {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < repetitions {
            {
                *tree.offset(*tree_size as (isize)) = 0i32 as (u8);
                *extra_bits_data.offset(*tree_size as (isize)) = 0i32 as (u8);
                *tree_size = (*tree_size).wrapping_add(1 as (usize));
            }
            i = i.wrapping_add(1 as (usize));
        }
    } else {
        let mut start : usize = *tree_size;
        repetitions = repetitions.wrapping_sub(3i32 as (usize));
        while 1i32 != 0 {
            *tree.offset(*tree_size as (isize)) = 17i32 as (u8);
            *extra_bits_data.offset(
                 *tree_size as (isize)
             ) = (repetitions & 0x7i32 as (usize)) as (u8);
            *tree_size = (*tree_size).wrapping_add(1 as (usize));
            repetitions = repetitions >> 3i32;
            if repetitions == 0i32 as (usize) {
                if 1337i32 != 0 {
                    break;
                }
            }
            repetitions = repetitions.wrapping_sub(1 as (usize));
        }
        Reverse(tree,start,*tree_size);
        Reverse(extra_bits_data,start,*tree_size);
    }
}

unsafe extern fn BrotliWriteHuffmanTreeRepetitions(
    previous_value : u8,
    value : u8,
    mut repetitions : usize,
    mut tree_size : *mut usize,
    mut tree : *mut u8,
    mut extra_bits_data : *mut u8
) {
    0i32;
    if previous_value as (i32) != value as (i32) {
        *tree.offset(*tree_size as (isize)) = value;
        *extra_bits_data.offset(*tree_size as (isize)) = 0i32 as (u8);
        *tree_size = (*tree_size).wrapping_add(1 as (usize));
        repetitions = repetitions.wrapping_sub(1 as (usize));
    }
    if repetitions == 7i32 as (usize) {
        *tree.offset(*tree_size as (isize)) = value;
        *extra_bits_data.offset(*tree_size as (isize)) = 0i32 as (u8);
        *tree_size = (*tree_size).wrapping_add(1 as (usize));
        repetitions = repetitions.wrapping_sub(1 as (usize));
    }
    if repetitions < 3i32 as (usize) {
        let mut i : usize;
        i = 0i32 as (usize);
        while i < repetitions {
            {
                *tree.offset(*tree_size as (isize)) = value;
                *extra_bits_data.offset(*tree_size as (isize)) = 0i32 as (u8);
                *tree_size = (*tree_size).wrapping_add(1 as (usize));
            }
            i = i.wrapping_add(1 as (usize));
        }
    } else {
        let mut start : usize = *tree_size;
        repetitions = repetitions.wrapping_sub(3i32 as (usize));
        while 1i32 != 0 {
            *tree.offset(*tree_size as (isize)) = 16i32 as (u8);
            *extra_bits_data.offset(
                 *tree_size as (isize)
             ) = (repetitions & 0x3i32 as (usize)) as (u8);
            *tree_size = (*tree_size).wrapping_add(1 as (usize));
            repetitions = repetitions >> 2i32;
            if repetitions == 0i32 as (usize) {
                if 1337i32 != 0 {
                    break;
                }
            }
            repetitions = repetitions.wrapping_sub(1 as (usize));
        }
        Reverse(tree,start,*tree_size);
        Reverse(extra_bits_data,start,*tree_size);
    }
}

#[no_mangle]
pub unsafe extern fn BrotliWriteHuffmanTree(
    mut depth : *const u8,
    mut length : usize,
    mut tree_size : *mut usize,
    mut tree : *mut u8,
    mut extra_bits_data : *mut u8
) {
    let mut previous_value : u8 = 8i32 as (u8);
    let mut i : usize;
    let mut use_rle_for_non_zero : i32 = 0i32;
    let mut use_rle_for_zero : i32 = 0i32;
    let mut new_length : usize = length;
    i = 0i32 as (usize);
    'break27: while i < length {
        {
            if *depth.offset(
                    length.wrapping_sub(i).wrapping_sub(1i32 as (usize)) as (isize)
                ) as (i32) == 0i32 {
                new_length = new_length.wrapping_sub(1 as (usize));
            } else if 1337i32 != 0 {
                break 'break27;
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    if length > 50i32 as (usize) {
        DecideOverRleUse(
            depth,
            new_length,
            &mut use_rle_for_non_zero as (*mut i32),
            &mut use_rle_for_zero as (*mut i32)
        );
    }
    i = 0i32 as (usize);
    while i < new_length {
        let value : u8 = *depth.offset(i as (isize));
        let mut reps : usize = 1i32 as (usize);
        if value as (i32) != 0i32 && (use_rle_for_non_zero != 0) || value as (i32) == 0i32 && (use_rle_for_zero != 0) {
            let mut k : usize;
            k = i.wrapping_add(1i32 as (usize));
            while k < new_length && (*depth.offset(
                                          k as (isize)
                                      ) as (i32) == value as (i32)) {
                {
                    reps = reps.wrapping_add(1 as (usize));
                }
                k = k.wrapping_add(1 as (usize));
            }
        }
        if value as (i32) == 0i32 {
            BrotliWriteHuffmanTreeRepetitionsZeros(
                reps,
                tree_size,
                tree,
                extra_bits_data
            );
        } else {
            BrotliWriteHuffmanTreeRepetitions(
                previous_value,
                value,
                reps,
                tree_size,
                tree,
                extra_bits_data
            );
            previous_value = value;
        }
        i = i.wrapping_add(reps);
    }
}

unsafe extern fn BrotliReverseBits(
    mut num_bits : usize, mut bits : u16
) -> u16 {
    static mut kLut
        : [usize; 16]
        = [   0x0i32 as (usize),
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
              0xfi32 as (usize)
          ];
    let mut retval : usize = kLut[(bits as (i32) & 0xfi32) as (usize)];
    let mut i : usize;
    i = 4i32 as (usize);
    while i < num_bits {
        {
            retval = retval << 4i32;
            bits = (bits as (i32) >> 4i32) as (u16);
            retval = retval | kLut[(bits as (i32) & 0xfi32) as (usize)];
        }
        i = i.wrapping_add(4i32 as (usize));
    }
    retval = retval >> ((0i32 as (usize)).wrapping_sub(
                            num_bits
                        ) & 0x3i32 as (usize));
    retval as (u16)
}

#[no_mangle]
pub unsafe extern fn BrotliConvertBitDepthsToSymbols(
    mut depth : *const u8, mut len : usize, mut bits : *mut u16
) {
    let mut bl_count
        : [u16; 16]
        = [   0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16)
          ];
    let mut next_code : [u16; 16];
    let mut i : usize;
    let mut code : i32 = 0i32;
    i = 0i32 as (usize);
    while i < len {
        {
            let _rhs = 1;
            let _lhs = &mut bl_count[*depth.offset(i as (isize)) as (usize)];
            *_lhs = (*_lhs as (i32) + _rhs) as (u16);
        }
        i = i.wrapping_add(1 as (usize));
    }
    bl_count[0i32 as (usize)] = 0i32 as (u16);
    next_code[0i32 as (usize)] = 0i32 as (u16);
    i = 1i32 as (usize);
    while i < 16i32 as (usize) {
        {
            code = code + bl_count[
                              i.wrapping_sub(1i32 as (usize))
                          ] as (i32) << 1i32;
            next_code[i] = code as (u16);
        }
        i = i.wrapping_add(1 as (usize));
    }
    i = 0i32 as (usize);
    while i < len {
        {
            if *depth.offset(i as (isize)) != 0 {
                *bits.offset(i as (isize)) = BrotliReverseBits(
                                                 *depth.offset(i as (isize)) as (usize),
                                                 {
                                                     let _rhs = 1;
                                                     let _lhs
                                                         = &mut next_code[
                                                                    *depth.offset(
                                                                         i as (isize)
                                                                     ) as (usize)
                                                                ];
                                                     let _old = *_lhs;
                                                     *_lhs = (*_lhs as (i32) + _rhs) as (u16);
                                                     _old
                                                 }
                                             );
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
}
