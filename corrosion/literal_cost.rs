extern {
    fn BrotliIsMostlyUTF8(
        data : *const u8,
        pos : usize,
        mask : usize,
        length : usize,
        min_fraction : f64
    ) -> i32;
    fn log2(__x : f64) -> f64;
}

static mut kLog2Table
    : *const f32
    = 0.0000000000000000f32 as (*const f32);

static kMinUTF8Ratio : f64 = 0.75f64;

unsafe extern fn brotli_min_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a < b { a } else { b }
}

unsafe extern fn UTF8Position(
    mut last : usize, mut c : usize, mut clamp : usize
) -> usize {
    if c < 128i32 as (usize) {
        0i32 as (usize)
    } else if c >= 192i32 as (usize) {
        brotli_min_size_t(1i32 as (usize),clamp)
    } else if last < 0xe0i32 as (usize) {
        0i32 as (usize)
    } else {
        brotli_min_size_t(2i32 as (usize),clamp)
    }
}

unsafe extern fn DecideMultiByteStatsLevel(
    mut pos : usize,
    mut len : usize,
    mut mask : usize,
    mut data : *const u8
) -> usize {
    let mut counts : *mut usize = 0i32 as (*mut usize);
    let mut max_utf8 : usize = 1i32 as (usize);
    let mut last_c : usize = 0i32 as (usize);
    let mut i : usize;
    i = 0i32 as (usize);
    while i < len {
        {
            let mut c
                : usize
                = *data.offset((pos.wrapping_add(i) & mask) as (isize)) as (usize);
            {
                let _rhs = 1;
                let _lhs
                    = &mut *counts.offset(
                                UTF8Position(last_c,c,2i32 as (usize)) as (isize)
                            );
                *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
            }
            last_c = c;
        }
        i = i.wrapping_add(1 as (usize));
    }
    if *counts.offset(2i32 as (isize)) < 500i32 as (usize) {
        max_utf8 = 1i32 as (usize);
    }
    if (*counts.offset(1i32 as (isize))).wrapping_add(
           *counts.offset(2i32 as (isize))
       ) < 25i32 as (usize) {
        max_utf8 = 0i32 as (usize);
    }
    max_utf8
}

unsafe extern fn FastLog2(mut v : usize) -> f64 {
    if v < std::mem::size_of::<*const f32>().wrapping_div(
               std::mem::size_of::<f32>()
           ) {
        return *kLog2Table.offset(v as (isize)) as (f64);
    }
    log2(v as (f64))
}

unsafe extern fn EstimateBitCostsForLiteralsUTF8(
    mut pos : usize,
    mut len : usize,
    mut mask : usize,
    mut data : *const u8,
    mut cost : *mut f32
) {
    let max_utf8
        : usize
        = DecideMultiByteStatsLevel(pos,len,mask,data);
    let mut histogram : *mut *mut usize = 0i32 as (*mut *mut usize);
    let mut window_half : usize = 495i32 as (usize);
    let mut in_window : usize = brotli_min_size_t(window_half,len);
    let mut in_window_utf8 : *mut usize = 0i32 as (*mut usize);
    let mut i : usize;
    {
        let mut last_c : usize = 0i32 as (usize);
        let mut utf8_pos : usize = 0i32 as (usize);
        i = 0i32 as (usize);
        while i < in_window {
            {
                let mut c
                    : usize
                    = *data.offset((pos.wrapping_add(i) & mask) as (isize)) as (usize);
                {
                    let _rhs = 1;
                    let _lhs
                        = &mut *(*histogram.offset(utf8_pos as (isize))).offset(
                                    c as (isize)
                                );
                    *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
                }
                {
                    let _rhs = 1;
                    let _lhs = &mut *in_window_utf8.offset(utf8_pos as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
                }
                utf8_pos = UTF8Position(last_c,c,max_utf8);
                last_c = c;
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    i = 0i32 as (usize);
    while i < len {
        {
            if i >= window_half {
                let mut c
                    : usize
                    = (if i < window_half.wrapping_add(1i32 as (usize)) {
                           0i32
                       } else {
                           *data.offset(
                                (pos.wrapping_add(i).wrapping_sub(window_half).wrapping_sub(
                                     1i32 as (usize)
                                 ) & mask) as (isize)
                            ) as (i32)
                       }) as (usize);
                let mut last_c
                    : usize
                    = (if i < window_half.wrapping_add(2i32 as (usize)) {
                           0i32
                       } else {
                           *data.offset(
                                (pos.wrapping_add(i).wrapping_sub(window_half).wrapping_sub(
                                     2i32 as (usize)
                                 ) & mask) as (isize)
                            ) as (i32)
                       }) as (usize);
                let mut utf8_pos2 : usize = UTF8Position(last_c,c,max_utf8);
                {
                    let _rhs = 1;
                    let _lhs
                        = &mut *(*histogram.offset(utf8_pos2 as (isize))).offset(
                                    *data.offset(
                                         (pos.wrapping_add(i).wrapping_sub(
                                              window_half
                                          ) & mask) as (isize)
                                     ) as (isize)
                                );
                    *_lhs = (*_lhs).wrapping_sub(_rhs as (usize));
                }
                {
                    let _rhs = 1;
                    let _lhs = &mut *in_window_utf8.offset(utf8_pos2 as (isize));
                    *_lhs = (*_lhs).wrapping_sub(_rhs as (usize));
                }
            }
            if i.wrapping_add(window_half) < len {
                let mut c
                    : usize
                    = *data.offset(
                           (pos.wrapping_add(i).wrapping_add(window_half).wrapping_sub(
                                1i32 as (usize)
                            ) & mask) as (isize)
                       ) as (usize);
                let mut last_c
                    : usize
                    = *data.offset(
                           (pos.wrapping_add(i).wrapping_add(window_half).wrapping_sub(
                                2i32 as (usize)
                            ) & mask) as (isize)
                       ) as (usize);
                let mut utf8_pos2 : usize = UTF8Position(last_c,c,max_utf8);
                {
                    let _rhs = 1;
                    let _lhs
                        = &mut *(*histogram.offset(utf8_pos2 as (isize))).offset(
                                    *data.offset(
                                         (pos.wrapping_add(i).wrapping_add(
                                              window_half
                                          ) & mask) as (isize)
                                     ) as (isize)
                                );
                    *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
                }
                {
                    let _rhs = 1;
                    let _lhs = &mut *in_window_utf8.offset(utf8_pos2 as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
                }
            }
            {
                let mut c
                    : usize
                    = (if i < 1i32 as (usize) {
                           0i32
                       } else {
                           *data.offset(
                                (pos.wrapping_add(i).wrapping_sub(
                                     1i32 as (usize)
                                 ) & mask) as (isize)
                            ) as (i32)
                       }) as (usize);
                let mut last_c
                    : usize
                    = (if i < 2i32 as (usize) {
                           0i32
                       } else {
                           *data.offset(
                                (pos.wrapping_add(i).wrapping_sub(
                                     2i32 as (usize)
                                 ) & mask) as (isize)
                            ) as (i32)
                       }) as (usize);
                let mut utf8_pos : usize = UTF8Position(last_c,c,max_utf8);
                let mut masked_pos : usize = pos.wrapping_add(i) & mask;
                let mut histo
                    : usize
                    = *(*histogram.offset(utf8_pos as (isize))).offset(
                           *data.offset(masked_pos as (isize)) as (isize)
                       );
                let mut lit_cost : f64;
                if histo == 0i32 as (usize) {
                    histo = 1i32 as (usize);
                }
                lit_cost = FastLog2(
                               *in_window_utf8.offset(utf8_pos as (isize))
                           ) - FastLog2(histo);
                lit_cost = lit_cost + 0.02905f64;
                if lit_cost < 1.0f64 {
                    lit_cost = lit_cost * 0.5f64;
                    lit_cost = lit_cost + 0.5f64;
                }
                if i < 2000i32 as (usize) {
                    lit_cost = lit_cost + (0.7f64 - (2000i32 as (usize)).wrapping_sub(
                                                        i
                                                    ) as (f64) / 2000.0f64 * 0.35f64);
                }
                *cost.offset(i as (isize)) = lit_cost as (f32);
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
}

#[no_mangle]
pub unsafe extern fn BrotliEstimateBitCostsForLiterals(
    mut pos : usize,
    mut len : usize,
    mut mask : usize,
    mut data : *const u8,
    mut cost : *mut f32
) { if BrotliIsMostlyUTF8(data,pos,mask,len,kMinUTF8Ratio) != 0 {
        EstimateBitCostsForLiteralsUTF8(pos,len,mask,data,cost);
    } else {
        let mut histogram : *mut usize = 0i32 as (*mut usize);
        let mut window_half : usize = 2000i32 as (usize);
        let mut in_window : usize = brotli_min_size_t(window_half,len);
        let mut i : usize;
        i = 0i32 as (usize);
        while i < in_window {
            {
                let _rhs = 1;
                let _lhs
                    = &mut *histogram.offset(
                                *data.offset((pos.wrapping_add(i) & mask) as (isize)) as (isize)
                            );
                *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
            }
            i = i.wrapping_add(1 as (usize));
        }
        i = 0i32 as (usize);
        while i < len {
            {
                let mut histo : usize;
                if i >= window_half {
                    {
                        let _rhs = 1;
                        let _lhs
                            = &mut *histogram.offset(
                                        *data.offset(
                                             (pos.wrapping_add(i).wrapping_sub(
                                                  window_half
                                              ) & mask) as (isize)
                                         ) as (isize)
                                    );
                        *_lhs = (*_lhs).wrapping_sub(_rhs as (usize));
                    }
                    in_window = in_window.wrapping_sub(1 as (usize));
                }
                if i.wrapping_add(window_half) < len {
                    {
                        let _rhs = 1;
                        let _lhs
                            = &mut *histogram.offset(
                                        *data.offset(
                                             (pos.wrapping_add(i).wrapping_add(
                                                  window_half
                                              ) & mask) as (isize)
                                         ) as (isize)
                                    );
                        *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
                    }
                    in_window = in_window.wrapping_add(1 as (usize));
                }
                histo = *histogram.offset(
                             *data.offset((pos.wrapping_add(i) & mask) as (isize)) as (isize)
                         );
                if histo == 0i32 as (usize) {
                    histo = 1i32 as (usize);
                }
                {
                    let mut lit_cost : f64 = FastLog2(in_window) - FastLog2(histo);
                    lit_cost = lit_cost + 0.029f64;
                    if lit_cost < 1.0f64 {
                        lit_cost = lit_cost * 0.5f64;
                        lit_cost = lit_cost + 0.5f64;
                    }
                    *cost.offset(i as (isize)) = lit_cost as (f32);
                }
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
}
