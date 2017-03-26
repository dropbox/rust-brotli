extern {
    fn log2(__x : f64) -> f64;
}

static mut kLog2Table
    : *const f32
    = 0.0000000000000000f32 as (*const f32);

static mut kInsBase : *mut u32 = 0i32 as (*mut u32);

static mut kInsExtra : *mut u32 = 0i32 as (*mut u32);

static mut kCopyBase : *mut u32 = 2i32 as (*mut u32);

static mut kCopyExtra : *mut u32 = 0i32 as (*mut u32);

static kBrotliMinWindowBits : i32 = 10i32;

static kBrotliMaxWindowBits : i32 = 24i32;

static mut kUTF8ContextLookup : *const u8 = 0i32 as (*const u8);

static mut kSigned3BitContextLookup
    : *const u8
    = 0i32 as (*const u8);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramLiteral {
    pub data_ : *mut u32,
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

unsafe extern fn HistogramDataSizeLiteral() -> usize {
    256i32 as (usize)
}

unsafe extern fn brotli_max_uint32_t(
    mut a : u32, mut b : u32
) -> u32 {
    if a > b { a } else { b }
}

unsafe extern fn FastLog2(mut v : usize) -> f64 {
    if v < std::mem::size_of::<*const f32>().wrapping_div(
               std::mem::size_of::<f32>()
           ) {
        return *kLog2Table.offset(v as (isize)) as (f64);
    }
    log2(v as (f64))
}

unsafe extern fn ShannonEntropy(
    mut population : *const u32,
    mut size : usize,
    mut total : *mut usize
) -> f64 {
    let mut sum : usize = 0i32 as (usize);
    let mut retval : f64 = 0i32 as (f64);
    let mut population_end
        : *const u32
        = population.offset(size as (isize));
    let mut p : usize;
    let mut odd_number_of_elements_left : i32 = 0i32;
    if size & 1i32 as (usize) != 0 {
        odd_number_of_elements_left = 1i32;
    }
    while population < population_end {
        if odd_number_of_elements_left == 0 {
            p = *{
                     let _old = population;
                     population = population.offset(1 as (isize));
                     _old
                 } as (usize);
            sum = sum.wrapping_add(p);
            retval = retval - p as (f64) * FastLog2(p);
        }
        odd_number_of_elements_left = 0i32;
        p = *{
                 let _old = population;
                 population = population.offset(1 as (isize));
                 _old
             } as (usize);
        sum = sum.wrapping_add(p);
        retval = retval - p as (f64) * FastLog2(p);
    }
    if sum != 0 {
        retval = retval + sum as (f64) * FastLog2(sum);
    }
    *total = sum;
    retval
}

unsafe extern fn BitsEntropy(
    mut population : *const u32, mut size : usize
) -> f64 {
    let mut sum : usize;
    let mut retval
        : f64
        = ShannonEntropy(population,size,&mut sum as (*mut usize));
    if retval < sum as (f64) {
        retval = sum as (f64);
    }
    retval
}

#[no_mangle]
pub unsafe extern fn BrotliPopulationCostLiteral(
    mut histogram : *const HistogramLiteral
) -> f64 {
    static kOneSymbolHistogramCost : f64 = 12i32 as (f64);
    static kTwoSymbolHistogramCost : f64 = 20i32 as (f64);
    static kThreeSymbolHistogramCost : f64 = 28i32 as (f64);
    static kFourSymbolHistogramCost : f64 = 37i32 as (f64);
    let data_size : usize = HistogramDataSizeLiteral();
    let mut count : i32 = 0i32;
    let mut s : *mut usize;
    let mut bits : f64 = 0.0f64;
    let mut i : usize;
    if (*histogram).total_count_ == 0i32 as (usize) {
        return kOneSymbolHistogramCost;
    }
    i = 0i32 as (usize);
    'break1: while i < data_size {
        {
            if *(*histogram).data_.offset(i as (isize)) > 0i32 as (u32) {
                *s.offset(count as (isize)) = i;
                count = count + 1;
                if count > 4i32 {
                    if 1337i32 != 0 {
                        break 'break1;
                    }
                }
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    if count == 1i32 {
        return kOneSymbolHistogramCost;
    }
    if count == 2i32 {
        return
            kTwoSymbolHistogramCost + (*histogram).total_count_ as (f64);
    }
    if count == 3i32 {
        let histo0
            : u32
            = *(*histogram).data_.offset(
                   *s.offset(0i32 as (isize)) as (isize)
               );
        let histo1
            : u32
            = *(*histogram).data_.offset(
                   *s.offset(1i32 as (isize)) as (isize)
               );
        let histo2
            : u32
            = *(*histogram).data_.offset(
                   *s.offset(2i32 as (isize)) as (isize)
               );
        let histomax
            : u32
            = brotli_max_uint32_t(histo0,brotli_max_uint32_t(histo1,histo2));
        return
            kThreeSymbolHistogramCost + (2i32 as (u32)).wrapping_mul(
                                            histo0.wrapping_add(histo1).wrapping_add(histo2)
                                        ) as (f64) - histomax as (f64);
    }
    if count == 4i32 {
        let mut histo : *mut u32;
        let mut h23 : u32;
        let mut histomax : u32;
        i = 0i32 as (usize);
        while i < 4i32 as (usize) {
            {
                *histo.offset(i as (isize)) = *(*histogram).data_.offset(
                                                   *s.offset(i as (isize)) as (isize)
                                               );
            }
            i = i.wrapping_add(1 as (usize));
        }
        i = 0i32 as (usize);
        while i < 4i32 as (usize) {
            {
                let mut j : usize;
                j = i.wrapping_add(1i32 as (usize));
                while j < 4i32 as (usize) {
                    {
                        if *histo.offset(j as (isize)) > *histo.offset(i as (isize)) {
                            let mut __brotli_swap_tmp : u32 = *histo.offset(j as (isize));
                            *histo.offset(j as (isize)) = *histo.offset(i as (isize));
                            *histo.offset(i as (isize)) = __brotli_swap_tmp;
                        }
                    }
                    j = j.wrapping_add(1 as (usize));
                }
            }
            i = i.wrapping_add(1 as (usize));
        }
        h23 = (*histo.offset(2i32 as (isize))).wrapping_add(
                  *histo.offset(3i32 as (isize))
              );
        histomax = brotli_max_uint32_t(h23,*histo.offset(0i32 as (isize)));
        return
            kFourSymbolHistogramCost + (3i32 as (u32)).wrapping_mul(
                                           h23
                                       ) as (f64) + (2i32 as (u32)).wrapping_mul(
                                                        (*histo.offset(
                                                              0i32 as (isize)
                                                          )).wrapping_add(
                                                            *histo.offset(1i32 as (isize))
                                                        )
                                                    ) as (f64) - histomax as (f64);
    }
    {
        let mut max_depth : usize = 1i32 as (usize);
        let mut depth_histo : *mut u32 = 0i32 as (*mut u32);
        let log2total : f64 = FastLog2((*histogram).total_count_);
        i = 0i32 as (usize);
        while i < data_size {
            if *(*histogram).data_.offset(i as (isize)) > 0i32 as (u32) {
                let mut log2p
                    : f64
                    = log2total - FastLog2(
                                      *(*histogram).data_.offset(i as (isize)) as (usize)
                                  );
                let mut depth : usize = (log2p + 0.5f64) as (usize);
                bits = bits + *(*histogram).data_.offset(
                                   i as (isize)
                               ) as (f64) * log2p;
                if depth > 15i32 as (usize) {
                    depth = 15i32 as (usize);
                }
                if depth > max_depth {
                    max_depth = depth;
                }
                {
                    let _rhs = 1;
                    let _lhs = &mut *depth_histo.offset(depth as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                }
                i = i.wrapping_add(1 as (usize));
            } else {
                let mut reps : u32 = 1i32 as (u32);
                let mut k : usize;
                k = i.wrapping_add(1i32 as (usize));
                while k < data_size && (*(*histogram).data_.offset(
                                             k as (isize)
                                         ) == 0i32 as (u32)) {
                    {
                        reps = reps.wrapping_add(1 as (u32));
                    }
                    k = k.wrapping_add(1 as (usize));
                }
                i = i.wrapping_add(reps as (usize));
                if i == data_size {
                    if 1337i32 != 0 {
                        break;
                    }
                }
                if reps < 3i32 as (u32) {
                    let _rhs = reps;
                    let _lhs = &mut *depth_histo.offset(0i32 as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                } else {
                    reps = reps.wrapping_sub(2i32 as (u32));
                    while reps > 0i32 as (u32) {
                        {
                            let _rhs = 1;
                            let _lhs = &mut *depth_histo.offset(17i32 as (isize));
                            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                        }
                        bits = bits + 3i32 as (f64);
                        reps = reps >> 3i32;
                    }
                }
            }
        }
        bits = bits + (18i32 as (usize)).wrapping_add(
                          (2i32 as (usize)).wrapping_mul(max_depth)
                      ) as (f64);
        bits = bits + BitsEntropy(
                          depth_histo as (*const u32),
                          18i32 as (usize)
                      );
    }
    bits
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramCommand {
    pub data_ : *mut u32,
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

unsafe extern fn HistogramDataSizeCommand() -> usize {
    704i32 as (usize)
}

#[no_mangle]
pub unsafe extern fn BrotliPopulationCostCommand(
    mut histogram : *const HistogramCommand
) -> f64 {
    static kOneSymbolHistogramCost : f64 = 12i32 as (f64);
    static kTwoSymbolHistogramCost : f64 = 20i32 as (f64);
    static kThreeSymbolHistogramCost : f64 = 28i32 as (f64);
    static kFourSymbolHistogramCost : f64 = 37i32 as (f64);
    let data_size : usize = HistogramDataSizeCommand();
    let mut count : i32 = 0i32;
    let mut s : *mut usize;
    let mut bits : f64 = 0.0f64;
    let mut i : usize;
    if (*histogram).total_count_ == 0i32 as (usize) {
        return kOneSymbolHistogramCost;
    }
    i = 0i32 as (usize);
    'break11: while i < data_size {
        {
            if *(*histogram).data_.offset(i as (isize)) > 0i32 as (u32) {
                *s.offset(count as (isize)) = i;
                count = count + 1;
                if count > 4i32 {
                    if 1337i32 != 0 {
                        break 'break11;
                    }
                }
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    if count == 1i32 {
        return kOneSymbolHistogramCost;
    }
    if count == 2i32 {
        return
            kTwoSymbolHistogramCost + (*histogram).total_count_ as (f64);
    }
    if count == 3i32 {
        let histo0
            : u32
            = *(*histogram).data_.offset(
                   *s.offset(0i32 as (isize)) as (isize)
               );
        let histo1
            : u32
            = *(*histogram).data_.offset(
                   *s.offset(1i32 as (isize)) as (isize)
               );
        let histo2
            : u32
            = *(*histogram).data_.offset(
                   *s.offset(2i32 as (isize)) as (isize)
               );
        let histomax
            : u32
            = brotli_max_uint32_t(histo0,brotli_max_uint32_t(histo1,histo2));
        return
            kThreeSymbolHistogramCost + (2i32 as (u32)).wrapping_mul(
                                            histo0.wrapping_add(histo1).wrapping_add(histo2)
                                        ) as (f64) - histomax as (f64);
    }
    if count == 4i32 {
        let mut histo : *mut u32;
        let mut h23 : u32;
        let mut histomax : u32;
        i = 0i32 as (usize);
        while i < 4i32 as (usize) {
            {
                *histo.offset(i as (isize)) = *(*histogram).data_.offset(
                                                   *s.offset(i as (isize)) as (isize)
                                               );
            }
            i = i.wrapping_add(1 as (usize));
        }
        i = 0i32 as (usize);
        while i < 4i32 as (usize) {
            {
                let mut j : usize;
                j = i.wrapping_add(1i32 as (usize));
                while j < 4i32 as (usize) {
                    {
                        if *histo.offset(j as (isize)) > *histo.offset(i as (isize)) {
                            let mut __brotli_swap_tmp : u32 = *histo.offset(j as (isize));
                            *histo.offset(j as (isize)) = *histo.offset(i as (isize));
                            *histo.offset(i as (isize)) = __brotli_swap_tmp;
                        }
                    }
                    j = j.wrapping_add(1 as (usize));
                }
            }
            i = i.wrapping_add(1 as (usize));
        }
        h23 = (*histo.offset(2i32 as (isize))).wrapping_add(
                  *histo.offset(3i32 as (isize))
              );
        histomax = brotli_max_uint32_t(h23,*histo.offset(0i32 as (isize)));
        return
            kFourSymbolHistogramCost + (3i32 as (u32)).wrapping_mul(
                                           h23
                                       ) as (f64) + (2i32 as (u32)).wrapping_mul(
                                                        (*histo.offset(
                                                              0i32 as (isize)
                                                          )).wrapping_add(
                                                            *histo.offset(1i32 as (isize))
                                                        )
                                                    ) as (f64) - histomax as (f64);
    }
    {
        let mut max_depth : usize = 1i32 as (usize);
        let mut depth_histo : *mut u32 = 0i32 as (*mut u32);
        let log2total : f64 = FastLog2((*histogram).total_count_);
        i = 0i32 as (usize);
        while i < data_size {
            if *(*histogram).data_.offset(i as (isize)) > 0i32 as (u32) {
                let mut log2p
                    : f64
                    = log2total - FastLog2(
                                      *(*histogram).data_.offset(i as (isize)) as (usize)
                                  );
                let mut depth : usize = (log2p + 0.5f64) as (usize);
                bits = bits + *(*histogram).data_.offset(
                                   i as (isize)
                               ) as (f64) * log2p;
                if depth > 15i32 as (usize) {
                    depth = 15i32 as (usize);
                }
                if depth > max_depth {
                    max_depth = depth;
                }
                {
                    let _rhs = 1;
                    let _lhs = &mut *depth_histo.offset(depth as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                }
                i = i.wrapping_add(1 as (usize));
            } else {
                let mut reps : u32 = 1i32 as (u32);
                let mut k : usize;
                k = i.wrapping_add(1i32 as (usize));
                while k < data_size && (*(*histogram).data_.offset(
                                             k as (isize)
                                         ) == 0i32 as (u32)) {
                    {
                        reps = reps.wrapping_add(1 as (u32));
                    }
                    k = k.wrapping_add(1 as (usize));
                }
                i = i.wrapping_add(reps as (usize));
                if i == data_size {
                    if 1337i32 != 0 {
                        break;
                    }
                }
                if reps < 3i32 as (u32) {
                    let _rhs = reps;
                    let _lhs = &mut *depth_histo.offset(0i32 as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                } else {
                    reps = reps.wrapping_sub(2i32 as (u32));
                    while reps > 0i32 as (u32) {
                        {
                            let _rhs = 1;
                            let _lhs = &mut *depth_histo.offset(17i32 as (isize));
                            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                        }
                        bits = bits + 3i32 as (f64);
                        reps = reps >> 3i32;
                    }
                }
            }
        }
        bits = bits + (18i32 as (usize)).wrapping_add(
                          (2i32 as (usize)).wrapping_mul(max_depth)
                      ) as (f64);
        bits = bits + BitsEntropy(
                          depth_histo as (*const u32),
                          18i32 as (usize)
                      );
    }
    bits
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramDistance {
    pub data_ : *mut u32,
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

unsafe extern fn HistogramDataSizeDistance() -> usize {
    520i32 as (usize)
}

#[no_mangle]
pub unsafe extern fn BrotliPopulationCostDistance(
    mut histogram : *const HistogramDistance
) -> f64 {
    static kOneSymbolHistogramCost : f64 = 12i32 as (f64);
    static kTwoSymbolHistogramCost : f64 = 20i32 as (f64);
    static kThreeSymbolHistogramCost : f64 = 28i32 as (f64);
    static kFourSymbolHistogramCost : f64 = 37i32 as (f64);
    let data_size : usize = HistogramDataSizeDistance();
    let mut count : i32 = 0i32;
    let mut s : *mut usize;
    let mut bits : f64 = 0.0f64;
    let mut i : usize;
    if (*histogram).total_count_ == 0i32 as (usize) {
        return kOneSymbolHistogramCost;
    }
    i = 0i32 as (usize);
    'break21: while i < data_size {
        {
            if *(*histogram).data_.offset(i as (isize)) > 0i32 as (u32) {
                *s.offset(count as (isize)) = i;
                count = count + 1;
                if count > 4i32 {
                    if 1337i32 != 0 {
                        break 'break21;
                    }
                }
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    if count == 1i32 {
        return kOneSymbolHistogramCost;
    }
    if count == 2i32 {
        return
            kTwoSymbolHistogramCost + (*histogram).total_count_ as (f64);
    }
    if count == 3i32 {
        let histo0
            : u32
            = *(*histogram).data_.offset(
                   *s.offset(0i32 as (isize)) as (isize)
               );
        let histo1
            : u32
            = *(*histogram).data_.offset(
                   *s.offset(1i32 as (isize)) as (isize)
               );
        let histo2
            : u32
            = *(*histogram).data_.offset(
                   *s.offset(2i32 as (isize)) as (isize)
               );
        let histomax
            : u32
            = brotli_max_uint32_t(histo0,brotli_max_uint32_t(histo1,histo2));
        return
            kThreeSymbolHistogramCost + (2i32 as (u32)).wrapping_mul(
                                            histo0.wrapping_add(histo1).wrapping_add(histo2)
                                        ) as (f64) - histomax as (f64);
    }
    if count == 4i32 {
        let mut histo : *mut u32;
        let mut h23 : u32;
        let mut histomax : u32;
        i = 0i32 as (usize);
        while i < 4i32 as (usize) {
            {
                *histo.offset(i as (isize)) = *(*histogram).data_.offset(
                                                   *s.offset(i as (isize)) as (isize)
                                               );
            }
            i = i.wrapping_add(1 as (usize));
        }
        i = 0i32 as (usize);
        while i < 4i32 as (usize) {
            {
                let mut j : usize;
                j = i.wrapping_add(1i32 as (usize));
                while j < 4i32 as (usize) {
                    {
                        if *histo.offset(j as (isize)) > *histo.offset(i as (isize)) {
                            let mut __brotli_swap_tmp : u32 = *histo.offset(j as (isize));
                            *histo.offset(j as (isize)) = *histo.offset(i as (isize));
                            *histo.offset(i as (isize)) = __brotli_swap_tmp;
                        }
                    }
                    j = j.wrapping_add(1 as (usize));
                }
            }
            i = i.wrapping_add(1 as (usize));
        }
        h23 = (*histo.offset(2i32 as (isize))).wrapping_add(
                  *histo.offset(3i32 as (isize))
              );
        histomax = brotli_max_uint32_t(h23,*histo.offset(0i32 as (isize)));
        return
            kFourSymbolHistogramCost + (3i32 as (u32)).wrapping_mul(
                                           h23
                                       ) as (f64) + (2i32 as (u32)).wrapping_mul(
                                                        (*histo.offset(
                                                              0i32 as (isize)
                                                          )).wrapping_add(
                                                            *histo.offset(1i32 as (isize))
                                                        )
                                                    ) as (f64) - histomax as (f64);
    }
    {
        let mut max_depth : usize = 1i32 as (usize);
        let mut depth_histo : *mut u32 = 0i32 as (*mut u32);
        let log2total : f64 = FastLog2((*histogram).total_count_);
        i = 0i32 as (usize);
        while i < data_size {
            if *(*histogram).data_.offset(i as (isize)) > 0i32 as (u32) {
                let mut log2p
                    : f64
                    = log2total - FastLog2(
                                      *(*histogram).data_.offset(i as (isize)) as (usize)
                                  );
                let mut depth : usize = (log2p + 0.5f64) as (usize);
                bits = bits + *(*histogram).data_.offset(
                                   i as (isize)
                               ) as (f64) * log2p;
                if depth > 15i32 as (usize) {
                    depth = 15i32 as (usize);
                }
                if depth > max_depth {
                    max_depth = depth;
                }
                {
                    let _rhs = 1;
                    let _lhs = &mut *depth_histo.offset(depth as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                }
                i = i.wrapping_add(1 as (usize));
            } else {
                let mut reps : u32 = 1i32 as (u32);
                let mut k : usize;
                k = i.wrapping_add(1i32 as (usize));
                while k < data_size && (*(*histogram).data_.offset(
                                             k as (isize)
                                         ) == 0i32 as (u32)) {
                    {
                        reps = reps.wrapping_add(1 as (u32));
                    }
                    k = k.wrapping_add(1 as (usize));
                }
                i = i.wrapping_add(reps as (usize));
                if i == data_size {
                    if 1337i32 != 0 {
                        break;
                    }
                }
                if reps < 3i32 as (u32) {
                    let _rhs = reps;
                    let _lhs = &mut *depth_histo.offset(0i32 as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                } else {
                    reps = reps.wrapping_sub(2i32 as (u32));
                    while reps > 0i32 as (u32) {
                        {
                            let _rhs = 1;
                            let _lhs = &mut *depth_histo.offset(17i32 as (isize));
                            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                        }
                        bits = bits + 3i32 as (f64);
                        reps = reps >> 3i32;
                    }
                }
            }
        }
        bits = bits + (18i32 as (usize)).wrapping_add(
                          (2i32 as (usize)).wrapping_mul(max_depth)
                      ) as (f64);
        bits = bits + BitsEntropy(
                          depth_histo as (*const u32),
                          18i32 as (usize)
                      );
    }
    bits
}
