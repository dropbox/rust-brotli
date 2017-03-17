extern {
    fn BrotliAllocate(
        m : *mut MemoryManager, n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn BrotliEstimateBitCostsForLiterals(
        pos : usize,
        len : usize,
        mask : usize,
        data : *const u8,
        cost : *mut f32
    );
    fn BrotliFindAllStaticDictionaryMatches(
        dictionary : *const BrotliDictionary,
        data : *const u8,
        min_length : usize,
        max_length : usize,
        matches : *mut u32
    ) -> i32;
    fn BrotliFree(
        m : *mut MemoryManager, p : *mut ::std::os::raw::c_void
    );
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn log2(__x : f64) -> f64;
    fn memcpy(
        __dest : *mut ::std::os::raw::c_void,
        __src : *const ::std::os::raw::c_void,
        __n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn memset(
        __s : *mut ::std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut ::std::os::raw::c_void;
}

static mut kLog2Table
    : [f32; 256]
    = [   0.0000000000000000f32,
          0.0000000000000000f32,
          1.0000000000000000f32,
          1.5849625007211563f32,
          2.0000000000000000f32,
          2.3219280948873622f32,
          2.5849625007211561f32,
          2.8073549220576042f32,
          3.0000000000000000f32,
          3.1699250014423126f32,
          3.3219280948873626f32,
          3.4594316186372978f32,
          3.5849625007211565f32,
          3.7004397181410922f32,
          3.8073549220576037f32,
          3.9068905956085187f32,
          4.0000000000000000f32,
          4.0874628412503400f32,
          4.1699250014423122f32,
          4.2479275134435852f32,
          4.3219280948873626f32,
          4.3923174227787607f32,
          4.4594316186372973f32,
          4.5235619560570131f32,
          4.5849625007211570f32,
          4.6438561897747244f32,
          4.7004397181410926f32,
          4.7548875021634691f32,
          4.8073549220576037f32,
          4.8579809951275728f32,
          4.9068905956085187f32,
          4.9541963103868758f32,
          5.0000000000000000f32,
          5.0443941193584534f32,
          5.0874628412503400f32,
          5.1292830169449664f32,
          5.1699250014423122f32,
          5.2094533656289501f32,
          5.2479275134435852f32,
          5.2854022188622487f32,
          5.3219280948873626f32,
          5.3575520046180838f32,
          5.3923174227787607f32,
          5.4262647547020979f32,
          5.4594316186372973f32,
          5.4918530963296748f32,
          5.5235619560570131f32,
          5.5545888516776376f32,
          5.5849625007211570f32,
          5.6147098441152083f32,
          5.6438561897747244f32,
          5.6724253419714961f32,
          5.7004397181410926f32,
          5.7279204545631996f32,
          5.7548875021634691f32,
          5.7813597135246599f32,
          5.8073549220576046f32,
          5.8328900141647422f32,
          5.8579809951275719f32,
          5.8826430493618416f32,
          5.9068905956085187f32,
          5.9307373375628867f32,
          5.9541963103868758f32,
          5.9772799234999168f32,
          6.0000000000000000f32,
          6.0223678130284544f32,
          6.0443941193584534f32,
          6.0660891904577721f32,
          6.0874628412503400f32,
          6.1085244567781700f32,
          6.1292830169449672f32,
          6.1497471195046822f32,
          6.1699250014423122f32,
          6.1898245588800176f32,
          6.2094533656289510f32,
          6.2288186904958804f32,
          6.2479275134435861f32,
          6.2667865406949019f32,
          6.2854022188622487f32,
          6.3037807481771031f32,
          6.3219280948873617f32,
          6.3398500028846252f32,
          6.3575520046180847f32,
          6.3750394313469254f32,
          6.3923174227787598f32,
          6.4093909361377026f32,
          6.4262647547020979f32,
          6.4429434958487288f32,
          6.4594316186372982f32,
          6.4757334309663976f32,
          6.4918530963296748f32,
          6.5077946401986964f32,
          6.5235619560570131f32,
          6.5391588111080319f32,
          6.5545888516776376f32,
          6.5698556083309478f32,
          6.5849625007211561f32,
          6.5999128421871278f32,
          6.6147098441152092f32,
          6.6293566200796095f32,
          6.6438561897747253f32,
          6.6582114827517955f32,
          6.6724253419714952f32,
          6.6865005271832185f32,
          6.7004397181410917f32,
          6.7142455176661224f32,
          6.7279204545631988f32,
          6.7414669864011465f32,
          6.7548875021634691f32,
          6.7681843247769260f32,
          6.7813597135246599f32,
          6.7944158663501062f32,
          6.8073549220576037f32,
          6.8201789624151887f32,
          6.8328900141647422f32,
          6.8454900509443757f32,
          6.8579809951275719f32,
          6.8703647195834048f32,
          6.8826430493618416f32,
          6.8948177633079437f32,
          6.9068905956085187f32,
          6.9188632372745955f32,
          6.9307373375628867f32,
          6.9425145053392399f32,
          6.9541963103868758f32,
          6.9657842846620879f32,
          6.9772799234999168f32,
          6.9886846867721664f32,
          7.0000000000000000f32,
          7.0112272554232540f32,
          7.0223678130284544f32,
          7.0334230015374501f32,
          7.0443941193584534f32,
          7.0552824355011898f32,
          7.0660891904577721f32,
          7.0768155970508317f32,
          7.0874628412503400f32,
          7.0980320829605272f32,
          7.1085244567781700f32,
          7.1189410727235076f32,
          7.1292830169449664f32,
          7.1395513523987937f32,
          7.1497471195046822f32,
          7.1598713367783891f32,
          7.1699250014423130f32,
          7.1799090900149345f32,
          7.1898245588800176f32,
          7.1996723448363644f32,
          7.2094533656289492f32,
          7.2191685204621621f32,
          7.2288186904958804f32,
          7.2384047393250794f32,
          7.2479275134435861f32,
          7.2573878426926521f32,
          7.2667865406949019f32,
          7.2761244052742384f32,
          7.2854022188622487f32,
          7.2946207488916270f32,
          7.3037807481771031f32,
          7.3128829552843557f32,
          7.3219280948873617f32,
          7.3309168781146177f32,
          7.3398500028846243f32,
          7.3487281542310781f32,
          7.3575520046180847f32,
          7.3663222142458151f32,
          7.3750394313469254f32,
          7.3837042924740528f32,
          7.3923174227787607f32,
          7.4008794362821844f32,
          7.4093909361377026f32,
          7.4178525148858991f32,
          7.4262647547020979f32,
          7.4346282276367255f32,
          7.4429434958487288f32,
          7.4512111118323299f32,
          7.4594316186372973f32,
          7.4676055500829976f32,
          7.4757334309663976f32,
          7.4838157772642564f32,
          7.4918530963296748f32,
          7.4998458870832057f32,
          7.5077946401986964f32,
          7.5156998382840436f32,
          7.5235619560570131f32,
          7.5313814605163119f32,
          7.5391588111080319f32,
          7.5468944598876373f32,
          7.5545888516776376f32,
          7.5622424242210728f32,
          7.5698556083309478f32,
          7.5774288280357487f32,
          7.5849625007211561f32,
          7.5924570372680806f32,
          7.5999128421871278f32,
          7.6073303137496113f32,
          7.6147098441152075f32,
          7.6220518194563764f32,
          7.6293566200796095f32,
          7.6366246205436488f32,
          7.6438561897747244f32,
          7.6510516911789290f32,
          7.6582114827517955f32,
          7.6653359171851765f32,
          7.6724253419714952f32,
          7.6794800995054464f32,
          7.6865005271832185f32,
          7.6934869574993252f32,
          7.7004397181410926f32,
          7.7073591320808825f32,
          7.7142455176661224f32,
          7.7210991887071856f32,
          7.7279204545631996f32,
          7.7347096202258392f32,
          7.7414669864011465f32,
          7.7481928495894596f32,
          7.7548875021634691f32,
          7.7615512324444795f32,
          7.7681843247769260f32,
          7.7747870596011737f32,
          7.7813597135246608f32,
          7.7879025593914317f32,
          7.7944158663501062f32,
          7.8008998999203047f32,
          7.8073549220576037f32,
          7.8137811912170374f32,
          7.8201789624151887f32,
          7.8265484872909159f32,
          7.8328900141647422f32,
          7.8392037880969445f32,
          7.8454900509443757f32,
          7.8517490414160571f32,
          7.8579809951275719f32,
          7.8641861446542798f32,
          7.8703647195834048f32,
          7.8765169465650002f32,
          7.8826430493618425f32,
          7.8887432488982601f32,
          7.8948177633079446f32,
          7.9008668079807496f32,
          7.9068905956085187f32,
          7.9128893362299619f32,
          7.9188632372745955f32,
          7.9248125036057813f32,
          7.9307373375628867f32,
          7.9366379390025719f32,
          7.9425145053392399f32,
          7.9483672315846778f32,
          7.9541963103868758f32,
          7.9600019320680806f32,
          7.9657842846620870f32,
          7.9715435539507720f32,
          7.9772799234999168f32,
          7.9829935746943104f32,
          7.9886846867721664f32,
          7.9943534368588578f32
      ];

static mut kInsBase
    : [u32; 24]
    = [   0i32 as (u32),
          1i32 as (u32),
          2i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          8i32 as (u32),
          10i32 as (u32),
          14i32 as (u32),
          18i32 as (u32),
          26i32 as (u32),
          34i32 as (u32),
          50i32 as (u32),
          66i32 as (u32),
          98i32 as (u32),
          130i32 as (u32),
          194i32 as (u32),
          322i32 as (u32),
          578i32 as (u32),
          1090i32 as (u32),
          2114i32 as (u32),
          6210i32 as (u32),
          22594i32 as (u32)
      ];

static mut kInsExtra
    : [u32; 24]
    = [   0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          1i32 as (u32),
          1i32 as (u32),
          2i32 as (u32),
          2i32 as (u32),
          3i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          7i32 as (u32),
          8i32 as (u32),
          9i32 as (u32),
          10i32 as (u32),
          12i32 as (u32),
          14i32 as (u32),
          24i32 as (u32)
      ];

static mut kCopyBase
    : [u32; 24]
    = [   2i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          7i32 as (u32),
          8i32 as (u32),
          9i32 as (u32),
          10i32 as (u32),
          12i32 as (u32),
          14i32 as (u32),
          18i32 as (u32),
          22i32 as (u32),
          30i32 as (u32),
          38i32 as (u32),
          54i32 as (u32),
          70i32 as (u32),
          102i32 as (u32),
          134i32 as (u32),
          198i32 as (u32),
          326i32 as (u32),
          582i32 as (u32),
          1094i32 as (u32),
          2118i32 as (u32)
      ];

static mut kCopyExtra
    : [u32; 24]
    = [   0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          1i32 as (u32),
          1i32 as (u32),
          2i32 as (u32),
          2i32 as (u32),
          3i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          7i32 as (u32),
          8i32 as (u32),
          9i32 as (u32),
          10i32 as (u32),
          24i32 as (u32)
      ];

static kBrotliMinWindowBits : i32 = 10i32;

static kBrotliMaxWindowBits : i32 = 24i32;

static kInvalidMatch : u32 = 0xfffffffi32 as (u32);

static kCutoffTransformsCount : u32 = 10i32 as (u32);

static kCutoffTransforms
    : usize
    = 0x71b520ai32 as (usize) << 32i32 | 0xda2d3200u32 as (usize);

static kHashMul32 : u32 = 0x1e35a7bdi32 as (u32);

static kHashMul64
    : usize
    = 0x1e35a7bdi32 as (usize) << 32i32 | 0x1e35a7bdi32 as (usize);

static kHashMul64Long
    : usize
    = 0x1fe35a7bu32 as (usize) << 32i32 | 0xd3579bd3u32 as (usize);

static kInfinity : f32 = 1.7e38f32;

static mut kDistanceCacheIndex
    : [u32; 16]
    = [   0i32 as (u32),
          1i32 as (u32),
          2i32 as (u32),
          3i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          1i32 as (u32),
          1i32 as (u32),
          1i32 as (u32),
          1i32 as (u32),
          1i32 as (u32),
          1i32 as (u32)
      ];

static mut kDistanceCacheOffset
    : [i32; 16]
    = [   0i32,
          0i32,
          0i32,
          0i32,
          -1i32,
          1i32,
          -2i32,
          2i32,
          -3i32,
          3i32,
          -1i32,
          1i32,
          -2i32,
          2i32,
          -3i32,
          3i32
      ];

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Struct1 {
    pub cost : f32,
    pub next : u32,
    pub shortcut : u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ZopfliNode {
    pub length : u32,
    pub distance : u32,
    pub insert_length : u32,
    pub u : Struct1,
}

#[no_mangle]
pub unsafe extern fn BrotliInitZopfliNodes(
    mut array : *mut ZopfliNode, mut length : usize
) {
    let mut stub : ZopfliNode;
    let mut i : usize;
    stub.length = 1i32 as (u32);
    stub.distance = 0i32 as (u32);
    stub.insert_length = 0i32 as (u32);
    stub.u.cost = kInfinity;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < length {
            *array.offset(i as (isize)) = stub;
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Command {
    pub insert_len_ : u32,
    pub copy_len_ : u32,
    pub dist_extra_ : u32,
    pub cmd_prefix_ : u16,
    pub dist_prefix_ : u16,
}

unsafe extern fn ZopfliNodeCopyLength(
    mut self : *const ZopfliNode
) -> u32 {
    (*self).length & 0xffffffi32 as (u32)
}

unsafe extern fn ZopfliNodeCopyDistance(
    mut self : *const ZopfliNode
) -> u32 {
    (*self).distance & 0x1ffffffi32 as (u32)
}

unsafe extern fn ZopfliNodeLengthCode(
    mut self : *const ZopfliNode
) -> u32 {
    let modifier : u32 = (*self).length >> 24i32;
    ZopfliNodeCopyLength(self).wrapping_add(9u32).wrapping_sub(
        modifier
    )
}

unsafe extern fn brotli_min_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a < b { a } else { b }
}

unsafe extern fn ZopfliNodeDistanceCode(
    mut self : *const ZopfliNode
) -> u32 {
    let short_code : u32 = (*self).distance >> 25i32;
    if short_code == 0i32 as (u32) {
        ZopfliNodeCopyDistance(self).wrapping_add(
            16i32 as (u32)
        ).wrapping_sub(
            1i32 as (u32)
        )
    } else {
        short_code.wrapping_sub(1i32 as (u32))
    }
}

unsafe extern fn Log2FloorNonZero(mut n : usize) -> u32 {
    let mut result : u32 = 0i32 as (u32);
    'loop1: loop {
        if {
               n = n >> 1i32;
               n
           } != 0 {
            result = result.wrapping_add(1 as (u32));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    result
}

unsafe extern fn PrefixEncodeCopyDistance(
    mut distance_code : usize,
    mut num_direct_codes : usize,
    mut postfix_bits : usize,
    mut code : *mut u16,
    mut extra_bits : *mut u32
) { if distance_code < (16i32 as (usize)).wrapping_add(
                           num_direct_codes
                       ) {
        *code = distance_code as (u16);
        *extra_bits = 0i32 as (u32);
    } else {
        let mut dist
            : usize
            = (1i32 as (usize) << postfix_bits.wrapping_add(
                                      2u32 as (usize)
                                  )).wrapping_add(
                  distance_code.wrapping_sub(16i32 as (usize)).wrapping_sub(
                      num_direct_codes
                  )
              );
        let mut bucket
            : usize
            = Log2FloorNonZero(dist).wrapping_sub(1i32 as (u32)) as (usize);
        let mut postfix_mask
            : usize
            = (1u32 << postfix_bits).wrapping_sub(1i32 as (u32)) as (usize);
        let mut postfix : usize = dist & postfix_mask;
        let mut prefix : usize = dist >> bucket & 1i32 as (usize);
        let mut offset
            : usize
            = (2i32 as (usize)).wrapping_add(prefix) << bucket;
        let mut nbits : usize = bucket.wrapping_sub(postfix_bits);
        *code = (16i32 as (usize)).wrapping_add(
                    num_direct_codes
                ).wrapping_add(
                    (2i32 as (usize)).wrapping_mul(
                        nbits.wrapping_sub(1i32 as (usize))
                    ).wrapping_add(
                        prefix
                    ) << postfix_bits
                ).wrapping_add(
                    postfix
                ) as (u16);
        *extra_bits = (nbits << 24i32 | dist.wrapping_sub(
                                            offset
                                        ) >> postfix_bits) as (u32);
    }
}

unsafe extern fn GetInsertLengthCode(
    mut insertlen : usize
) -> u16 {
    if insertlen < 6i32 as (usize) {
        insertlen as (u16)
    } else if insertlen < 130i32 as (usize) {
        let mut nbits
            : u32
            = Log2FloorNonZero(
                  insertlen.wrapping_sub(2i32 as (usize))
              ).wrapping_sub(
                  1u32
              );
        ((nbits << 1i32) as (usize)).wrapping_add(
            insertlen.wrapping_sub(2i32 as (usize)) >> nbits
        ).wrapping_add(
            2i32 as (usize)
        ) as (u16)
    } else if insertlen < 2114i32 as (usize) {
        Log2FloorNonZero(
            insertlen.wrapping_sub(66i32 as (usize))
        ).wrapping_add(
            10i32 as (u32)
        ) as (u16)
    } else if insertlen < 6210i32 as (usize) {
        21u32 as (u16)
    } else if insertlen < 22594i32 as (usize) {
        22u32 as (u16)
    } else {
        23u32 as (u16)
    }
}

unsafe extern fn GetCopyLengthCode(mut copylen : usize) -> u16 {
    if copylen < 10i32 as (usize) {
        copylen.wrapping_sub(2i32 as (usize)) as (u16)
    } else if copylen < 134i32 as (usize) {
        let mut nbits
            : u32
            = Log2FloorNonZero(
                  copylen.wrapping_sub(6i32 as (usize))
              ).wrapping_sub(
                  1u32
              );
        ((nbits << 1i32) as (usize)).wrapping_add(
            copylen.wrapping_sub(6i32 as (usize)) >> nbits
        ).wrapping_add(
            4i32 as (usize)
        ) as (u16)
    } else if copylen < 2118i32 as (usize) {
        Log2FloorNonZero(
            copylen.wrapping_sub(70i32 as (usize))
        ).wrapping_add(
            12i32 as (u32)
        ) as (u16)
    } else {
        23u32 as (u16)
    }
}

unsafe extern fn CombineLengthCodes(
    mut inscode : u16, mut copycode : u16, mut use_last_distance : i32
) -> u16 {
    let mut bits64
        : u16
        = (copycode as (u32) & 0x7u32 | (inscode as (u32) & 0x7u32) << 3i32) as (u16);
    if use_last_distance != 0 && (inscode as (i32) < 8i32) && (copycode as (i32) < 16i32) {
        if copycode as (i32) < 8i32 {
            bits64 as (i32)
        } else {
            bits64 as (i32) | 64i32
        } as (u16)
    } else {
        let mut offset
            : i32
            = 2i32 * ((copycode as (i32) >> 3i32) + 3i32 * (inscode as (i32) >> 3i32));
        offset = (offset << 5i32) + 0x40i32 + (0x520d40i32 >> offset & 0xc0i32);
        (offset as (u16) as (i32) | bits64 as (i32)) as (u16)
    }
}

unsafe extern fn GetLengthCode(
    mut insertlen : usize,
    mut copylen : usize,
    mut use_last_distance : i32,
    mut code : *mut u16
) {
    let mut inscode : u16 = GetInsertLengthCode(insertlen);
    let mut copycode : u16 = GetCopyLengthCode(copylen);
    *code = CombineLengthCodes(inscode,copycode,use_last_distance);
}

unsafe extern fn InitCommand(
    mut self : *mut Command,
    mut insertlen : usize,
    mut copylen : usize,
    mut copylen_code : usize,
    mut distance_code : usize
) {
    (*self).insert_len_ = insertlen as (u32);
    (*self).copy_len_ = (copylen | (copylen_code ^ copylen) << 24i32) as (u32);
    PrefixEncodeCopyDistance(
        distance_code,
        0i32 as (usize),
        0i32 as (usize),
        &mut (*self).dist_prefix_ as (*mut u16),
        &mut (*self).dist_extra_ as (*mut u32)
    );
    GetLengthCode(
        insertlen,
        copylen_code,
        if !!((*self).dist_prefix_ as (i32) == 0i32) {
            1i32
        } else {
            0i32
        },
        &mut (*self).cmd_prefix_ as (*mut u16)
    );
}

#[no_mangle]
pub unsafe extern fn BrotliZopfliCreateCommands(
    num_bytes : usize,
    block_start : usize,
    max_backward_limit : usize,
    mut nodes : *const ZopfliNode,
    mut dist_cache : *mut i32,
    mut last_insert_len : *mut usize,
    mut commands : *mut Command,
    mut num_literals : *mut usize
) {
    let mut pos : usize = 0i32 as (usize);
    let mut offset : u32 = (*nodes.offset(0i32 as (isize))).u.next;
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if offset != !(0i32 as (u32)) {
            let mut next
                : *const ZopfliNode
                = &*nodes.offset(
                        pos.wrapping_add(offset as (usize)) as (isize)
                    ) as (*const ZopfliNode);
            let mut copy_length
                : usize
                = ZopfliNodeCopyLength(next) as (usize);
            let mut insert_length : usize = (*next).insert_length as (usize);
            pos = pos.wrapping_add(insert_length);
            offset = (*next).u.next;
            if i == 0i32 as (usize) {
                insert_length = insert_length.wrapping_add(*last_insert_len);
                *last_insert_len = 0i32 as (usize);
            }
            let mut distance : usize = ZopfliNodeCopyDistance(next) as (usize);
            let mut len_code : usize = ZopfliNodeLengthCode(next) as (usize);
            let mut max_distance
                : usize
                = brotli_min_size_t(
                      block_start.wrapping_add(pos),
                      max_backward_limit
                  );
            let mut is_dictionary
                : i32
                = if !!(distance > max_distance) { 1i32 } else { 0i32 };
            let mut dist_code
                : usize
                = ZopfliNodeDistanceCode(next) as (usize);
            InitCommand(
                &mut *commands.offset(i as (isize)) as (*mut Command),
                insert_length,
                copy_length,
                len_code,
                dist_code
            );
            if is_dictionary == 0 && (dist_code > 0i32 as (usize)) {
                *dist_cache.offset(3i32 as (isize)) = *dist_cache.offset(
                                                           2i32 as (isize)
                                                       );
                *dist_cache.offset(2i32 as (isize)) = *dist_cache.offset(
                                                           1i32 as (isize)
                                                       );
                *dist_cache.offset(1i32 as (isize)) = *dist_cache.offset(
                                                           0i32 as (isize)
                                                       );
                *dist_cache.offset(0i32 as (isize)) = distance as (i32);
            }
            *num_literals = (*num_literals).wrapping_add(insert_length);
            pos = pos.wrapping_add(copy_length);
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    *last_insert_len = (*last_insert_len).wrapping_add(
                           num_bytes.wrapping_sub(pos)
                       );
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func : unsafe extern fn(*mut ::std::os::raw::c_void, usize) -> *mut ::std::os::raw::c_void,
    pub free_func : unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void),
    pub opaque : *mut ::std::os::raw::c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length : [u8; 32],
    pub offsets_by_length : [u32; 32],
    pub data : [u8; 122784],
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum BrotliEncoderMode {
    BROTLI_MODE_GENERIC = 0i32,
    BROTLI_MODE_TEXT = 1i32,
    BROTLI_MODE_FONT = 2i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrotliHasherParams {
    pub type_ : i32,
    pub bucket_bits : i32,
    pub block_bits : i32,
    pub hash_len : i32,
    pub num_last_distances_to_check : i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrotliEncoderParams {
    pub mode : BrotliEncoderMode,
    pub quality : i32,
    pub lgwin : i32,
    pub lgblock : i32,
    pub size_hint : usize,
    pub disable_literal_context_modeling : i32,
    pub hasher : BrotliHasherParams,
}

unsafe extern fn MaxZopfliLen(
    mut params : *const BrotliEncoderParams
) -> usize {
    (if (*params).quality <= 10i32 {
         150i32
     } else {
         325i32
     }) as (usize)
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ZopfliCostModel {
    pub cost_cmd_ : [f32; 704],
    pub cost_dist_ : [f32; 520],
    pub literal_costs_ : *mut f32,
    pub min_cost_cmd_ : f32,
    pub num_bytes_ : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PosData {
    pub pos : usize,
    pub distance_cache : [i32; 4],
    pub costdiff : f32,
    pub cost : f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StartPosQueue {
    pub q_ : [PosData; 8],
    pub idx_ : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BackwardMatch {
    pub distance : u32,
    pub length_and_code : u32,
}

unsafe extern fn StoreLookaheadH10() -> usize { 128i32 as (usize) }

unsafe extern fn InitZopfliCostModel(
    mut m : *mut MemoryManager,
    mut self : *mut ZopfliCostModel,
    mut num_bytes : usize
) {
    (*self).num_bytes_ = num_bytes;
    (*self).literal_costs_ = if num_bytes.wrapping_add(
                                    2i32 as (usize)
                                ) != 0 {
                                 BrotliAllocate(
                                     m,
                                     num_bytes.wrapping_add(2i32 as (usize)).wrapping_mul(
                                         ::std::mem::size_of::<f32>()
                                     )
                                 ) as (*mut f32)
                             } else {
                                 0i32 as (*mut ::std::os::raw::c_void) as (*mut f32)
                             };
    if !(0i32 == 0) { }
}

unsafe extern fn FastLog2(mut v : usize) -> f64 {
    if v < ::std::mem::size_of::<[f32; 256]>().wrapping_div(
               ::std::mem::size_of::<f32>()
           ) {
        kLog2Table[v] as (f64)
    } else {
        log2(v as (f64))
    }
}

unsafe extern fn ZopfliCostModelSetFromLiteralCosts(
    mut self : *mut ZopfliCostModel,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize
) {
    let mut literal_costs : *mut f32 = (*self).literal_costs_;
    let mut cost_dist : *mut f32 = (*self).cost_dist_.as_mut_ptr();
    let mut cost_cmd : *mut f32 = (*self).cost_cmd_.as_mut_ptr();
    let mut num_bytes : usize = (*self).num_bytes_;
    let mut i : usize;
    BrotliEstimateBitCostsForLiterals(
        position,
        num_bytes,
        ringbuffer_mask,
        ringbuffer,
        &mut *literal_costs.offset(1i32 as (isize)) as (*mut f32)
    );
    *literal_costs.offset(0i32 as (isize)) = 0.0f64 as (f32);
    i = 0i32 as (usize);
    'loop1: loop {
        if i < num_bytes {
            {
                let _rhs = *literal_costs.offset(i as (isize));
                let _lhs
                    = &mut *literal_costs.offset(
                                i.wrapping_add(1i32 as (usize)) as (isize)
                            );
                *_lhs = *_lhs + _rhs;
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    i = 0i32 as (usize);
    'loop3: loop {
        if i < 704i32 as (usize) {
            *cost_cmd.offset(i as (isize)) = FastLog2(
                                                 (11i32 as (u32)).wrapping_add(
                                                     i as (u32)
                                                 ) as (usize)
                                             ) as (f32);
            i = i.wrapping_add(1 as (usize));
            continue 'loop3;
        } else {
            break 'loop3;
        }
    }
    i = 0i32 as (usize);
    'loop5: loop {
        if i < 520i32 as (usize) {
            *cost_dist.offset(i as (isize)) = FastLog2(
                                                  (20i32 as (u32)).wrapping_add(
                                                      i as (u32)
                                                  ) as (usize)
                                              ) as (f32);
            i = i.wrapping_add(1 as (usize));
            continue 'loop5;
        } else {
            break 'loop5;
        }
    }
    (*self).min_cost_cmd_ = FastLog2(11i32 as (usize)) as (f32);
}

unsafe extern fn InitStartPosQueue(mut self : *mut StartPosQueue) {
    (*self).idx_ = 0i32 as (usize);
}

unsafe extern fn BROTLI_UNALIGNED_LOAD64(
    mut p : *const ::std::os::raw::c_void
) -> usize {
    let mut t : usize;
    memcpy(
        &mut t as (*mut usize) as (*mut ::std::os::raw::c_void),
        p,
        ::std::mem::size_of::<usize>()
    );
    t
}

unsafe extern fn unopt_ctzll(mut val : usize) -> u8 {
    let mut cnt : u8 = 0i32 as (u8);
    'loop1: loop {
        if val & 1i32 as (usize) == 0i32 as (usize) {
            val = val >> 1i32;
            cnt = (cnt as (i32) + 1) as (u8);
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    cnt
}

unsafe extern fn FindMatchLengthWithLimit(
    mut s1 : *const u8, mut s2 : *const u8, mut limit : usize
) -> usize {
    let mut matched : usize = 0i32 as (usize);
    let mut limit2
        : usize
        = (limit >> 3i32).wrapping_add(1i32 as (usize));
    'loop1: loop {
        if {
               limit2 = limit2.wrapping_sub(1 as (usize));
               limit2
           } != 0 {
            if BROTLI_UNALIGNED_LOAD64(
                   s2 as (*const ::std::os::raw::c_void)
               ) == BROTLI_UNALIGNED_LOAD64(
                        s1.offset(matched as (isize)) as (*const ::std::os::raw::c_void)
                    ) {
                s2 = s2.offset(8i32 as (isize));
                matched = matched.wrapping_add(8i32 as (usize));
                continue 'loop1;
            } else {
                break 'loop1;
            }
        } else {
            limit = (limit & 7i32 as (usize)).wrapping_add(1i32 as (usize));
            'loop3: loop {
                if {
                       limit = limit.wrapping_sub(1 as (usize));
                       limit
                   } != 0 {
                    if *s1.offset(matched as (isize)) as (i32) == *s2 as (i32) {
                        s2 = s2.offset(1 as (isize));
                        matched = matched.wrapping_add(1 as (usize));
                        continue 'loop3;
                    } else {
                        break 'loop3;
                    }
                } else {
                    return matched;
                }
            }
            return matched;
        }
    }
    let mut x
        : usize
        = BROTLI_UNALIGNED_LOAD64(
              s2 as (*const ::std::os::raw::c_void)
          ) ^ BROTLI_UNALIGNED_LOAD64(
                  s1.offset(matched as (isize)) as (*const ::std::os::raw::c_void)
              );
    let mut matching_bits : usize = unopt_ctzll(x) as (usize);
    matched = matched.wrapping_add(matching_bits >> 3i32);
    matched
}

unsafe extern fn InitBackwardMatch(
    mut self : *mut BackwardMatch, mut dist : usize, mut len : usize
) {
    (*self).distance = dist as (u32);
    (*self).length_and_code = (len << 5i32) as (u32);
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct H10 {
    pub window_mask_ : usize,
    pub buckets_ : [u32; 131072],
    pub invalid_pos_ : u32,
}

unsafe extern fn StoreAndFindMatchesH10(
    mut self : *mut H10,
    data : *const u8,
    cur_ix : usize,
    ring_buffer_mask : usize,
    max_length : usize,
    max_backward : usize,
    best_len : *mut usize,
    mut matches : *mut BackwardMatch
) -> *mut BackwardMatch {
  const size_t cur_ix_masked = cur_ix & ring_buffer_mask;
  const size_t max_comp_len =
      BROTLI_MIN(size_t, max_length, MAX_TREE_COMP_LENGTH);
  const BROTLI_BOOL should_reroot_tree =
      TO_BROTLI_BOOL(max_length >= MAX_TREE_COMP_LENGTH);
  const uint32_t key = FN(HashBytes)(&data[cur_ix_masked]);
  uint32_t* forest = FN(Forest)(self);
  size_t prev_ix = self->buckets_[key];
  /* The forest index of the rightmost node of the left subtree of the new
     root, updated as we traverse and re-root the tree of the hash bucket. */
  size_t node_left = FN(LeftChildIndex)(self, cur_ix);
  /* The forest index of the leftmost node of the right subtree of the new
     root, updated as we traverse and re-root the tree of the hash bucket. */
  size_t node_right = FN(RightChildIndex)(self, cur_ix);
  /* The match length of the rightmost node of the left subtree of the new
     root, updated as we traverse and re-root the tree of the hash bucket. */
  size_t best_len_left = 0;
  /* The match length of the leftmost node of the right subtree of the new
     root, updated as we traverse and re-root the tree of the hash bucket. */
  size_t best_len_right = 0;
  size_t depth_remaining;
  if (should_reroot_tree) {
    self->buckets_[key] = (uint32_t)cur_ix;
  }
  for (depth_remaining = MAX_TREE_SEARCH_DEPTH; ; --depth_remaining) {
    const size_t backward = cur_ix - prev_ix;
    const size_t prev_ix_masked = prev_ix & ring_buffer_mask;
    if (backward == 0 || backward > max_backward || depth_remaining == 0) {
      if (should_reroot_tree) {
        forest[node_left] = self->invalid_pos_;
        forest[node_right] = self->invalid_pos_;
      }
      break;
    }
    {
      const size_t cur_len = BROTLI_MIN(size_t, best_len_left, best_len_right);
      size_t len;
      assert(cur_len <= MAX_TREE_COMP_LENGTH);
      len = cur_len +
          FindMatchLengthWithLimit(&data[cur_ix_masked + cur_len],
                                   &data[prev_ix_masked + cur_len],
                                   max_length - cur_len);
      assert(0 == memcmp(&data[cur_ix_masked], &data[prev_ix_masked], len));
      if (matches && len > *best_len) {
        *best_len = len;
        InitBackwardMatch(matches++, backward, len);
      }
      if (len >= max_comp_len) {
        if (should_reroot_tree) {
          forest[node_left] = forest[FN(LeftChildIndex)(self, prev_ix)];
          forest[node_right] = forest[FN(RightChildIndex)(self, prev_ix)];
        }
        break;
      }
      if (data[cur_ix_masked + len] > data[prev_ix_masked + len]) {
        best_len_left = len;
        if (should_reroot_tree) {
          forest[node_left] = (uint32_t)prev_ix;
        }
        node_left = FN(RightChildIndex)(self, prev_ix);
        prev_ix = forest[node_left];
      } else {
        best_len_right = len;
        if (should_reroot_tree) {
          forest[node_right] = (uint32_t)prev_ix;
        }
        node_right = FN(LeftChildIndex)(self, prev_ix);
        prev_ix = forest[node_right];
      }
    }
  }
  return matches;
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Struct2 {
    pub params : BrotliHasherParams,
    pub is_prepared_ : i32,
    pub dict_num_lookups : usize,
    pub dict_num_matches : usize,
}

unsafe extern fn GetHasherCommon(
    mut handle : *mut u8
) -> *mut Struct2 {
    handle as (*mut Struct2)
}

unsafe extern fn SelfH10(mut handle : *mut u8) -> *mut H10 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct2) as (*mut H10)
}

unsafe extern fn brotli_max_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a > b { a } else { b }
}

unsafe extern fn InitDictionaryBackwardMatch(
    mut self : *mut BackwardMatch,
    mut dist : usize,
    mut len : usize,
    mut len_code : usize
) {
    (*self).distance = dist as (u32);
    (*self).length_and_code = (len << 5i32 | if len == len_code {
                                                 0i32 as (usize)
                                             } else {
                                                 len_code
                                             }) as (u32);
}

unsafe extern fn FindAllMatchesH10(
    mut handle : *mut u8,
    mut dictionary : *const BrotliDictionary,
    mut data : *const u8,
    ring_buffer_mask : usize,
    cur_ix : usize,
    max_length : usize,
    max_backward : usize,
    mut params : *const BrotliEncoderParams,
    mut matches : *mut BackwardMatch
) -> usize {
    let orig_matches : *mut BackwardMatch = matches;
    let cur_ix_masked : usize = cur_ix & ring_buffer_mask;
    let mut best_len : usize = 1i32 as (usize);
    let short_match_max_backward
        : usize
        = (if (*params).quality != 11i32 {
               16i32
           } else {
               64i32
           }) as (usize);
    let mut stop
        : usize
        = cur_ix.wrapping_sub(short_match_max_backward);
    let mut dict_matches : [u32; 38];
    let mut i : usize;
    if cur_ix < short_match_max_backward {
        stop = 0i32 as (usize);
    }
    i = cur_ix.wrapping_sub(1i32 as (usize));
    'loop3: loop {
        if i > stop && (best_len <= 2i32 as (usize)) {
            let mut prev_ix : usize = i;
            let backward : usize = cur_ix.wrapping_sub(prev_ix);
            if backward > max_backward {
                break 'loop3;
            } else {
                prev_ix = prev_ix & ring_buffer_mask;
                if !(*data.offset(
                          cur_ix_masked as (isize)
                      ) as (i32) != *data.offset(
                                         prev_ix as (isize)
                                     ) as (i32) || *data.offset(
                                                        cur_ix_masked.wrapping_add(
                                                            1i32 as (usize)
                                                        ) as (isize)
                                                    ) as (i32) != *data.offset(
                                                                       prev_ix.wrapping_add(
                                                                           1i32 as (usize)
                                                                       ) as (isize)
                                                                   ) as (i32)) {
                    let len
                        : usize
                        = FindMatchLengthWithLimit(
                              &*data.offset(prev_ix as (isize)) as (*const u8),
                              &*data.offset(cur_ix_masked as (isize)) as (*const u8),
                              max_length
                          );
                    if len > best_len {
                        best_len = len;
                        InitBackwardMatch(
                            {
                                let _old = matches;
                                matches = matches.offset(1 as (isize));
                                _old
                            },
                            backward,
                            len
                        );
                    }
                }
                i = i.wrapping_sub(1 as (usize));
                continue 'loop3;
            }
        } else {
            break 'loop3;
        }
    }
    if best_len < max_length {
        matches = StoreAndFindMatchesH10(
                      SelfH10(handle),
                      data,
                      cur_ix,
                      ring_buffer_mask,
                      max_length,
                      max_backward,
                      &mut best_len as (*mut usize),
                      matches
                  );
    }
    i = 0i32 as (usize);
    'loop12: loop {
        if i <= 37i32 as (usize) {
            dict_matches[i] = kInvalidMatch;
            i = i.wrapping_add(1 as (usize));
            continue 'loop12;
        } else {
            break 'loop12;
        }
    }
    let mut minlen
        : usize
        = brotli_max_size_t(
              4i32 as (usize),
              best_len.wrapping_add(1i32 as (usize))
          );
    if BrotliFindAllStaticDictionaryMatches(
           dictionary,
           &*data.offset(cur_ix_masked as (isize)) as (*const u8),
           minlen,
           max_length,
           &mut dict_matches[0i32 as (usize)] as (*mut u32)
       ) != 0 {
        let mut maxlen
            : usize
            = brotli_min_size_t(37i32 as (usize),max_length);
        let mut l : usize;
        l = minlen;
        'loop15: loop {
            if l <= maxlen {
                let mut dict_id : u32 = dict_matches[l];
                if dict_id < kInvalidMatch {
                    InitDictionaryBackwardMatch(
                        {
                            let _old = matches;
                            matches = matches.offset(1 as (isize));
                            _old
                        },
                        max_backward.wrapping_add(
                            (dict_id >> 5i32) as (usize)
                        ).wrapping_add(
                            1i32 as (usize)
                        ),
                        l,
                        (dict_id & 31i32 as (u32)) as (usize)
                    );
                }
                l = l.wrapping_add(1 as (usize));
                continue 'loop15;
            } else {
                break 'loop15;
            }
        }
    }
    ((matches as (isize)).wrapping_sub(
         orig_matches as (isize)
     ) / ::std::mem::size_of::<BackwardMatch>() as (isize)) as (usize)
}

unsafe extern fn BackwardMatchLength(
    mut self : *const BackwardMatch
) -> usize {
    ((*self).length_and_code >> 5i32) as (usize)
}

unsafe extern fn MaxZopfliCandidates(
    mut params : *const BrotliEncoderParams
) -> usize {
    (if (*params).quality <= 10i32 { 1i32 } else { 5i32 }) as (usize)
}

unsafe extern fn ComputeDistanceShortcut(
    block_start : usize,
    pos : usize,
    max_backward : usize,
    mut nodes : *const ZopfliNode
) -> u32 {
    let clen
        : usize
        = ZopfliNodeCopyLength(
              &*nodes.offset(pos as (isize)) as (*const ZopfliNode)
          ) as (usize);
    let ilen
        : usize
        = (*nodes.offset(pos as (isize))).insert_length as (usize);
    let dist
        : usize
        = ZopfliNodeCopyDistance(
              &*nodes.offset(pos as (isize)) as (*const ZopfliNode)
          ) as (usize);
    if pos == 0i32 as (usize) {
        0i32 as (u32)
    } else if dist.wrapping_add(clen) <= block_start.wrapping_add(
                                             pos
                                         ) && (dist <= max_backward) && (ZopfliNodeDistanceCode(
                                                                             &*nodes.offset(
                                                                                   pos as (isize)
                                                                               ) as (*const ZopfliNode)
                                                                         ) > 0i32 as (u32)) {
        pos as (u32)
    } else {
        (*nodes.offset(
              pos.wrapping_sub(clen).wrapping_sub(ilen) as (isize)
          )).u.shortcut
    }
}

unsafe extern fn ZopfliCostModelGetLiteralCosts(
    mut self : *const ZopfliCostModel, mut from : usize, mut to : usize
) -> f32 {
    *(*self).literal_costs_.offset(
         to as (isize)
     ) - *(*self).literal_costs_.offset(from as (isize))
}

unsafe extern fn ComputeDistanceCache(
    pos : usize,
    mut starting_dist_cache : *const i32,
    mut nodes : *const ZopfliNode,
    mut dist_cache : *mut i32
) {
    let mut idx : i32 = 0i32;
    let mut p
        : usize
        = (*nodes.offset(pos as (isize))).u.shortcut as (usize);
    'loop1: loop {
        if idx < 4i32 && (p > 0i32 as (usize)) {
            let ilen
                : usize
                = (*nodes.offset(p as (isize))).insert_length as (usize);
            let clen
                : usize
                = ZopfliNodeCopyLength(
                      &*nodes.offset(p as (isize)) as (*const ZopfliNode)
                  ) as (usize);
            let dist
                : usize
                = ZopfliNodeCopyDistance(
                      &*nodes.offset(p as (isize)) as (*const ZopfliNode)
                  ) as (usize);
            *dist_cache.offset(
                 {
                     let _old = idx;
                     idx = idx + 1;
                     _old
                 } as (isize)
             ) = dist as (i32);
            p = (*nodes.offset(
                      p.wrapping_sub(clen).wrapping_sub(ilen) as (isize)
                  )).u.shortcut as (usize);
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    'loop2: loop {
        if idx < 4i32 {
            *dist_cache.offset(idx as (isize)) = *{
                                                      let _old = starting_dist_cache;
                                                      starting_dist_cache = starting_dist_cache.offset(
                                                                                1 as (isize)
                                                                            );
                                                      _old
                                                  };
            idx = idx + 1;
            continue 'loop2;
        } else {
            break 'loop2;
        }
    }
}

unsafe extern fn StartPosQueueSize(
    mut self : *const StartPosQueue
) -> usize {
    brotli_min_size_t((*self).idx_,8i32 as (usize))
}

unsafe extern fn StartPosQueuePush(
    mut self : *mut StartPosQueue, mut posdata : *const PosData
) {
    let mut offset
        : usize
        = !{
               let _old = (*self).idx_;
               (*self).idx_ = (*self).idx_.wrapping_add(1 as (usize));
               _old
           } & 7i32 as (usize);
    let mut len
        : usize
        = StartPosQueueSize(self as (*const StartPosQueue));
    let mut i : usize;
    let mut q : *mut PosData = (*self).q_.as_mut_ptr();
    *q.offset(offset as (isize)) = *posdata;
    i = 1i32 as (usize);
    'loop1: loop {
        if i < len {
            if (*q.offset(
                     (offset & 7i32 as (usize)) as (isize)
                 )).costdiff > (*q.offset(
                                     (offset.wrapping_add(
                                          1i32 as (usize)
                                      ) & 7i32 as (usize)) as (isize)
                                 )).costdiff {
                let mut __brotli_swap_tmp
                    : PosData
                    = *q.offset((offset & 7i32 as (usize)) as (isize));
                *q.offset((offset & 7i32 as (usize)) as (isize)) = *q.offset(
                                                                        (offset.wrapping_add(
                                                                             1i32 as (usize)
                                                                         ) & 7i32 as (usize)) as (isize)
                                                                    );
                *q.offset(
                     (offset.wrapping_add(1i32 as (usize)) & 7i32 as (usize)) as (isize)
                 ) = __brotli_swap_tmp;
            }
            offset = offset.wrapping_add(1 as (usize));
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn EvaluateNode(
    block_start : usize,
    pos : usize,
    max_backward_limit : usize,
    mut starting_dist_cache : *const i32,
    mut model : *const ZopfliCostModel,
    mut queue : *mut StartPosQueue,
    mut nodes : *mut ZopfliNode
) {
    let mut node_cost : f32 = (*nodes.offset(pos as (isize))).u.cost;
    (*nodes.offset(
          pos as (isize)
      )).u.shortcut = ComputeDistanceShortcut(
                          block_start,
                          pos,
                          max_backward_limit,
                          nodes as (*const ZopfliNode)
                      );
    if node_cost <= ZopfliCostModelGetLiteralCosts(
                        model,
                        0i32 as (usize),
                        pos
                    ) {
        let mut posdata : PosData;
        posdata.pos = pos;
        posdata.cost = node_cost;
        posdata.costdiff = node_cost - ZopfliCostModelGetLiteralCosts(
                                           model,
                                           0i32 as (usize),
                                           pos
                                       );
        ComputeDistanceCache(
            pos,
            starting_dist_cache,
            nodes as (*const ZopfliNode),
            posdata.distance_cache.as_mut_ptr()
        );
        StartPosQueuePush(
            queue,
            &mut posdata as (*mut PosData) as (*const PosData)
        );
    }
}

unsafe extern fn StartPosQueueAt(
    mut self : *const StartPosQueue, mut k : usize
) -> *const PosData {
    &mut (*self).q_[
             k.wrapping_sub((*self).idx_) & 7i32 as (usize)
         ] as (*mut PosData) as (*const PosData)
}

unsafe extern fn ZopfliCostModelGetMinCostCmd(
    mut self : *const ZopfliCostModel
) -> f32 {
    (*self).min_cost_cmd_
}

unsafe extern fn ComputeMinimumCopyLength(
    start_cost : f32,
    mut nodes : *const ZopfliNode,
    num_bytes : usize,
    pos : usize
) -> usize {
    let mut min_cost : f32 = start_cost;
    let mut len : usize = 2i32 as (usize);
    let mut next_len_bucket : usize = 4i32 as (usize);
    let mut next_len_offset : usize = 10i32 as (usize);
    'loop1: loop {
        if pos.wrapping_add(len) <= num_bytes && ((*nodes.offset(
                                                        pos.wrapping_add(len) as (isize)
                                                    )).u.cost <= min_cost) {
            len = len.wrapping_add(1 as (usize));
            if len == next_len_offset {
                min_cost = min_cost + 1.0f32;
                next_len_offset = next_len_offset.wrapping_add(next_len_bucket);
                next_len_bucket = next_len_bucket.wrapping_mul(2i32 as (usize));
                continue 'loop1;
            } else {
                continue 'loop1;
            }
        } else {
            break 'loop1;
        }
    }
    len
}

unsafe extern fn GetInsertExtra(mut inscode : u16) -> u32 {
    kInsExtra[inscode as (usize)]
}

unsafe extern fn ZopfliCostModelGetDistanceCost(
    mut self : *const ZopfliCostModel, mut distcode : usize
) -> f32 {
    (*self).cost_dist_[distcode]
}

unsafe extern fn GetCopyExtra(mut copycode : u16) -> u32 {
    kCopyExtra[copycode as (usize)]
}

unsafe extern fn ZopfliCostModelGetCommandCost(
    mut self : *const ZopfliCostModel, mut cmdcode : u16
) -> f32 {
    (*self).cost_cmd_[cmdcode as (usize)]
}

unsafe extern fn UpdateZopfliNode(
    mut nodes : *mut ZopfliNode,
    mut pos : usize,
    mut start_pos : usize,
    mut len : usize,
    mut len_code : usize,
    mut dist : usize,
    mut short_code : usize,
    mut cost : f32
) {
    let mut next
        : *mut ZopfliNode
        = &mut *nodes.offset(
                    pos.wrapping_add(len) as (isize)
                ) as (*mut ZopfliNode);
    (*next).length = (len | len.wrapping_add(
                                9u32 as (usize)
                            ).wrapping_sub(
                                len_code
                            ) << 24i32) as (u32);
    (*next).distance = (dist | short_code << 25i32) as (u32);
    (*next).insert_length = pos.wrapping_sub(start_pos) as (u32);
    (*next).u.cost = cost;
}

unsafe extern fn BackwardMatchLengthCode(
    mut self : *const BackwardMatch
) -> usize {
    let mut code
        : usize
        = ((*self).length_and_code & 31i32 as (u32)) as (usize);
    if code != 0 { code } else { BackwardMatchLength(self) }
}

unsafe extern fn UpdateNodes(
    num_bytes : usize,
    block_start : usize,
    pos : usize,
    mut ringbuffer : *const u8,
    ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    max_backward_limit : usize,
    mut starting_dist_cache : *const i32,
    num_matches : usize,
    mut matches : *const BackwardMatch,
    mut model : *const ZopfliCostModel,
    mut queue : *mut StartPosQueue,
    mut nodes : *mut ZopfliNode
) -> usize {
    let cur_ix : usize = block_start.wrapping_add(pos);
    let cur_ix_masked : usize = cur_ix & ringbuffer_mask;
    let max_distance
        : usize
        = brotli_min_size_t(cur_ix,max_backward_limit);
    let max_len : usize = num_bytes.wrapping_sub(pos);
    let max_zopfli_len : usize = MaxZopfliLen(params);
    let max_iters : usize = MaxZopfliCandidates(params);
    let mut min_len : usize;
    let mut result : usize = 0i32 as (usize);
    let mut k : usize;
    EvaluateNode(
        block_start,
        pos,
        max_backward_limit,
        starting_dist_cache,
        model,
        queue,
        nodes
    );
    let mut posdata
        : *const PosData
        = StartPosQueueAt(queue as (*const StartPosQueue),0i32 as (usize));
    let mut min_cost
        : f32
        = (*posdata).cost + ZopfliCostModelGetMinCostCmd(
                                model
                            ) + ZopfliCostModelGetLiteralCosts(model,(*posdata).pos,pos);
    min_len = ComputeMinimumCopyLength(
                  min_cost,
                  nodes as (*const ZopfliNode),
                  num_bytes,
                  pos
              );
    k = 0i32 as (usize);
    'loop1: loop {
        if k < max_iters && (k < StartPosQueueSize(
                                     queue as (*const StartPosQueue)
                                 )) {
            let mut posdata
                : *const PosData
                = StartPosQueueAt(queue as (*const StartPosQueue),k);
            let start : usize = (*posdata).pos;
            let inscode : u16 = GetInsertLengthCode(pos.wrapping_sub(start));
            let start_costdiff : f32 = (*posdata).costdiff;
            let base_cost
                : f32
                = start_costdiff + GetInsertExtra(
                                       inscode
                                   ) as (f32) + ZopfliCostModelGetLiteralCosts(
                                                    model,
                                                    0i32 as (usize),
                                                    pos
                                                );
            let mut best_len : usize = min_len.wrapping_sub(1i32 as (usize));
            let mut j : usize = 0i32 as (usize);
            'loop4: loop {
                if j < 16i32 as (usize) && (best_len < max_len) {
                    let idx : usize = kDistanceCacheIndex[j] as (usize);
                    let backward
                        : usize
                        = ((*posdata).distance_cache[idx] + kDistanceCacheOffset[
                                                                j
                                                            ]) as (usize);
                    let mut prev_ix : usize = cur_ix.wrapping_sub(backward);
                    if !(prev_ix >= cur_ix) {
                        if !(backward > max_distance) {
                            prev_ix = prev_ix & ringbuffer_mask;
                            if !(cur_ix_masked.wrapping_add(
                                     best_len
                                 ) > ringbuffer_mask || prev_ix.wrapping_add(
                                                            best_len
                                                        ) > ringbuffer_mask || *ringbuffer.offset(
                                                                                    cur_ix_masked.wrapping_add(
                                                                                        best_len
                                                                                    ) as (isize)
                                                                                ) as (i32) != *ringbuffer.offset(
                                                                                                   prev_ix.wrapping_add(
                                                                                                       best_len
                                                                                                   ) as (isize)
                                                                                               ) as (i32)) {
                                let len
                                    : usize
                                    = FindMatchLengthWithLimit(
                                          &*ringbuffer.offset(prev_ix as (isize)) as (*const u8),
                                          &*ringbuffer.offset(
                                                cur_ix_masked as (isize)
                                            ) as (*const u8),
                                          max_len
                                      );
                                let dist_cost
                                    : f32
                                    = base_cost + ZopfliCostModelGetDistanceCost(model,j);
                                let mut l : usize;
                                l = best_len.wrapping_add(1i32 as (usize));
                                'loop20: loop {
                                    if l <= len {
                                        let copycode : u16 = GetCopyLengthCode(l);
                                        let cmdcode
                                            : u16
                                            = CombineLengthCodes(
                                                  inscode,
                                                  copycode,
                                                  (j == 0i32 as (usize)) as (i32)
                                              );
                                        let cost
                                            : f32
                                            = (if cmdcode as (i32) < 128i32 {
                                                   base_cost
                                               } else {
                                                   dist_cost
                                               }) + GetCopyExtra(
                                                        copycode
                                                    ) as (f32) + ZopfliCostModelGetCommandCost(
                                                                     model,
                                                                     cmdcode
                                                                 );
                                        if cost < (*nodes.offset(
                                                        pos.wrapping_add(l) as (isize)
                                                    )).u.cost {
                                            UpdateZopfliNode(
                                                nodes,
                                                pos,
                                                start,
                                                l,
                                                l,
                                                backward,
                                                j.wrapping_add(1i32 as (usize)),
                                                cost
                                            );
                                            result = brotli_max_size_t(result,l);
                                        }
                                        best_len = l;
                                        l = l.wrapping_add(1 as (usize));
                                        continue 'loop20;
                                    } else {
                                        break 'loop20;
                                    }
                                }
                            }
                        }
                    }
                    j = j.wrapping_add(1 as (usize));
                    continue 'loop4;
                } else {
                    break 'loop4;
                }
            }
            if !(k >= 2i32 as (usize)) {
                let mut len : usize = min_len;
                j = 0i32 as (usize);
                'loop7: loop {
                    if j < num_matches {
                        let mut match_ : BackwardMatch = *matches.offset(j as (isize));
                        let mut dist : usize = match_.distance as (usize);
                        let mut is_dictionary_match
                            : i32
                            = if !!(dist > max_distance) { 1i32 } else { 0i32 };
                        let mut dist_code
                            : usize
                            = dist.wrapping_add(16i32 as (usize)).wrapping_sub(
                                  1i32 as (usize)
                              );
                        let mut dist_symbol : u16;
                        let mut distextra : u32;
                        let mut distnumextra : u32;
                        let mut dist_cost : f32;
                        let mut max_match_len : usize;
                        PrefixEncodeCopyDistance(
                            dist_code,
                            0i32 as (usize),
                            0i32 as (usize),
                            &mut dist_symbol as (*mut u16),
                            &mut distextra as (*mut u32)
                        );
                        distnumextra = distextra >> 24i32;
                        dist_cost = base_cost + distnumextra as (f32) + ZopfliCostModelGetDistanceCost(
                                                                            model,
                                                                            dist_symbol as (usize)
                                                                        );
                        max_match_len = BackwardMatchLength(
                                            &mut match_ as (*mut BackwardMatch) as (*const BackwardMatch)
                                        );
                        if len < max_match_len && (is_dictionary_match != 0 || max_match_len > max_zopfli_len) {
                            len = max_match_len;
                        }
                        'loop10: loop {
                            if len <= max_match_len {
                                let len_code
                                    : usize
                                    = if is_dictionary_match != 0 {
                                          BackwardMatchLengthCode(
                                              &mut match_ as (*mut BackwardMatch) as (*const BackwardMatch)
                                          )
                                      } else {
                                          len
                                      };
                                let copycode : u16 = GetCopyLengthCode(len_code);
                                let cmdcode : u16 = CombineLengthCodes(inscode,copycode,0i32);
                                let cost
                                    : f32
                                    = dist_cost + GetCopyExtra(
                                                      copycode
                                                  ) as (f32) + ZopfliCostModelGetCommandCost(
                                                                   model,
                                                                   cmdcode
                                                               );
                                if cost < (*nodes.offset(
                                                pos.wrapping_add(len) as (isize)
                                            )).u.cost {
                                    UpdateZopfliNode(
                                        nodes,
                                        pos,
                                        start,
                                        len,
                                        len_code,
                                        dist,
                                        0i32 as (usize),
                                        cost
                                    );
                                    result = brotli_max_size_t(result,len);
                                }
                                len = len.wrapping_add(1 as (usize));
                                continue 'loop10;
                            } else {
                                break 'loop10;
                            }
                        }
                        j = j.wrapping_add(1 as (usize));
                        continue 'loop7;
                    } else {
                        break 'loop7;
                    }
                }
            }
            k = k.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    result
}

unsafe extern fn StoreH10(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix : usize
) {
    let mut self : *mut H10 = SelfH10(handle);
    let max_backward
        : usize
        = (*self).window_mask_.wrapping_sub(16i32 as (usize)).wrapping_add(
              1i32 as (usize)
          );
    StoreAndFindMatchesH10(
        self,
        data,
        ix,
        mask,
        128i32 as (usize),
        max_backward,
        0i32 as (*mut ::std::os::raw::c_void) as (*mut usize),
        0i32 as (*mut ::std::os::raw::c_void) as (*mut BackwardMatch)
    );
}

unsafe extern fn StoreRangeH10(
    mut handle : *mut u8,
    mut data : *const u8,
    mask : usize,
    ix_start : usize,
    ix_end : usize
) {
    let mut i : usize = ix_start;
    let mut j : usize = ix_start;
    if ix_start.wrapping_add(63i32 as (usize)) <= ix_end {
        i = ix_end.wrapping_sub(63i32 as (usize));
    }
    if ix_start.wrapping_add(512i32 as (usize)) <= i {
        'loop3: loop {
            if j < i {
                StoreH10(handle,data,mask,j);
                j = j.wrapping_add(8i32 as (usize));
                continue 'loop3;
            } else {
                break 'loop3;
            }
        }
    }
    'loop4: loop {
        if i < ix_end {
            StoreH10(handle,data,mask,i);
            i = i.wrapping_add(1 as (usize));
            continue 'loop4;
        } else {
            break 'loop4;
        }
    }
}

unsafe extern fn HashTypeLengthH10() -> usize { 4i32 as (usize) }

unsafe extern fn CleanupZopfliCostModel(
    mut m : *mut MemoryManager, mut self : *mut ZopfliCostModel
) {
    BrotliFree(
        m,
        (*self).literal_costs_ as (*mut ::std::os::raw::c_void)
    );
    (*self).literal_costs_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut f32);
}

unsafe extern fn ZopfliNodeCommandLength(
    mut self : *const ZopfliNode
) -> u32 {
    ZopfliNodeCopyLength(self).wrapping_add((*self).insert_length)
}

unsafe extern fn ComputeShortestPathFromNodes(
    mut num_bytes : usize, mut nodes : *mut ZopfliNode
) -> usize {
    let mut index : usize = num_bytes;
    let mut num_commands : usize = 0i32 as (usize);
    'loop1: loop {
        if (*nodes.offset(
                 index as (isize)
             )).insert_length == 0i32 as (u32) && ((*nodes.offset(
                                                         index as (isize)
                                                     )).length == 1i32 as (u32)) {
            index = index.wrapping_sub(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    (*nodes.offset(index as (isize))).u.next = !(0i32 as (u32));
    'loop3: loop {
        if index != 0i32 as (usize) {
            let mut len
                : usize
                = ZopfliNodeCommandLength(
                      &mut *nodes.offset(
                                index as (isize)
                            ) as (*mut ZopfliNode) as (*const ZopfliNode)
                  ) as (usize);
            index = index.wrapping_sub(len);
            (*nodes.offset(index as (isize))).u.next = len as (u32);
            num_commands = num_commands.wrapping_add(1 as (usize));
            continue 'loop3;
        } else {
            break 'loop3;
        }
    }
    num_commands
}

#[no_mangle]
pub unsafe extern fn BrotliZopfliComputeShortestPath(
    mut m : *mut MemoryManager,
    mut dictionary : *const BrotliDictionary,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    max_backward_limit : usize,
    mut dist_cache : *const i32,
    mut hasher : *mut u8,
    mut nodes : *mut ZopfliNode
) -> usize {
    let max_zopfli_len : usize = MaxZopfliLen(params);
    let mut model : ZopfliCostModel;
    let mut queue : StartPosQueue;
    let mut matches : [BackwardMatch; 128];
    let store_end
        : usize
        = if num_bytes >= StoreLookaheadH10() {
              position.wrapping_add(num_bytes).wrapping_sub(
                  StoreLookaheadH10()
              ).wrapping_add(
                  1i32 as (usize)
              )
          } else {
              position
          };
    let mut i : usize;
    (*nodes.offset(0i32 as (isize))).length = 0i32 as (u32);
    (*nodes.offset(0i32 as (isize))).u.cost = 0i32 as (f32);
    InitZopfliCostModel(
        m,
        &mut model as (*mut ZopfliCostModel),
        num_bytes
    );
    if !(0i32 == 0) {
        0i32 as (usize)
    } else {
        ZopfliCostModelSetFromLiteralCosts(
            &mut model as (*mut ZopfliCostModel),
            position,
            ringbuffer,
            ringbuffer_mask
        );
        InitStartPosQueue(&mut queue as (*mut StartPosQueue));
        i = 0i32 as (usize);
        'loop2: loop {
            if i.wrapping_add(HashTypeLengthH10()).wrapping_sub(
                   1i32 as (usize)
               ) < num_bytes {
                let pos : usize = position.wrapping_add(i);
                let max_distance
                    : usize
                    = brotli_min_size_t(pos,max_backward_limit);
                let mut num_matches
                    : usize
                    = FindAllMatchesH10(
                          hasher,
                          dictionary,
                          ringbuffer,
                          ringbuffer_mask,
                          pos,
                          num_bytes.wrapping_sub(i),
                          max_distance,
                          params,
                          matches.as_mut_ptr()
                      );
                let mut skip : usize;
                if num_matches > 0i32 as (usize) && (BackwardMatchLength(
                                                         &mut matches[
                                                                  num_matches.wrapping_sub(
                                                                      1i32 as (usize)
                                                                  )
                                                              ] as (*mut BackwardMatch) as (*const BackwardMatch)
                                                     ) > max_zopfli_len) {
                    matches[0i32 as (usize)] = matches[
                                                   num_matches.wrapping_sub(1i32 as (usize))
                                               ];
                    num_matches = 1i32 as (usize);
                }
                skip = UpdateNodes(
                           num_bytes,
                           position,
                           i,
                           ringbuffer,
                           ringbuffer_mask,
                           params,
                           max_backward_limit,
                           dist_cache,
                           num_matches,
                           matches.as_mut_ptr() as (*const BackwardMatch),
                           &mut model as (*mut ZopfliCostModel) as (*const ZopfliCostModel),
                           &mut queue as (*mut StartPosQueue),
                           nodes
                       );
                if skip < 16384i32 as (usize) {
                    skip = 0i32 as (usize);
                }
                if num_matches == 1i32 as (usize) && (BackwardMatchLength(
                                                          &mut matches[
                                                                   0i32 as (usize)
                                                               ] as (*mut BackwardMatch) as (*const BackwardMatch)
                                                      ) > max_zopfli_len) {
                    skip = brotli_max_size_t(
                               BackwardMatchLength(
                                   &mut matches[
                                            0i32 as (usize)
                                        ] as (*mut BackwardMatch) as (*const BackwardMatch)
                               ),
                               skip
                           );
                }
                if skip > 1i32 as (usize) {
                    StoreRangeH10(
                        hasher,
                        ringbuffer,
                        ringbuffer_mask,
                        pos.wrapping_add(1i32 as (usize)),
                        brotli_min_size_t(pos.wrapping_add(skip),store_end)
                    );
                    skip = skip.wrapping_sub(1 as (usize));
                    'loop12: loop {
                        if skip != 0 {
                            i = i.wrapping_add(1 as (usize));
                            if i.wrapping_add(HashTypeLengthH10()).wrapping_sub(
                                   1i32 as (usize)
                               ) >= num_bytes {
                                break 'loop12;
                            } else {
                                EvaluateNode(
                                    position,
                                    i,
                                    max_backward_limit,
                                    dist_cache,
                                    &mut model as (*mut ZopfliCostModel) as (*const ZopfliCostModel),
                                    &mut queue as (*mut StartPosQueue),
                                    nodes
                                );
                                skip = skip.wrapping_sub(1 as (usize));
                                continue 'loop12;
                            }
                        } else {
                            break 'loop12;
                        }
                    }
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        CleanupZopfliCostModel(m,&mut model as (*mut ZopfliCostModel));
        ComputeShortestPathFromNodes(num_bytes,nodes)
    }
}

#[no_mangle]
pub unsafe extern fn BrotliCreateZopfliBackwardReferences(
    mut m : *mut MemoryManager,
    mut dictionary : *const BrotliDictionary,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    mut hasher : *mut u8,
    mut dist_cache : *mut i32,
    mut last_insert_len : *mut usize,
    mut commands : *mut Command,
    mut num_commands : *mut usize,
    mut num_literals : *mut usize
) {
    let max_backward_limit
        : usize
        = (1i32 as (usize) << (*params).lgwin).wrapping_sub(
              16i32 as (usize)
          );
    let mut nodes : *mut ZopfliNode;
    nodes = if num_bytes.wrapping_add(1i32 as (usize)) != 0 {
                BrotliAllocate(
                    m,
                    num_bytes.wrapping_add(1i32 as (usize)).wrapping_mul(
                        ::std::mem::size_of::<ZopfliNode>()
                    )
                ) as (*mut ZopfliNode)
            } else {
                0i32 as (*mut ::std::os::raw::c_void) as (*mut ZopfliNode)
            };
    if !(0i32 == 0) {
    } else {
        BrotliInitZopfliNodes(
            nodes,
            num_bytes.wrapping_add(1i32 as (usize))
        );
        *num_commands = (*num_commands).wrapping_add(
                            BrotliZopfliComputeShortestPath(
                                m,
                                dictionary,
                                num_bytes,
                                position,
                                ringbuffer,
                                ringbuffer_mask,
                                params,
                                max_backward_limit,
                                dist_cache as (*const i32),
                                hasher,
                                nodes
                            )
                        );
        if !(0i32 == 0) {
        } else {
            BrotliZopfliCreateCommands(
                num_bytes,
                position,
                max_backward_limit,
                nodes as (*const ZopfliNode),
                dist_cache,
                last_insert_len,
                commands,
                num_literals
            );
            BrotliFree(m,nodes as (*mut ::std::os::raw::c_void));
            nodes = 0i32 as (*mut ::std::os::raw::c_void) as (*mut ZopfliNode);
        }
    }
}

unsafe extern fn CommandCopyLen(mut self : *const Command) -> u32 {
    (*self).copy_len_ & 0xffffffi32 as (u32)
}

unsafe extern fn SetCost(
    mut histogram : *const u32,
    mut histogram_size : usize,
    mut cost : *mut f32
) {
    let mut sum : usize = 0i32 as (usize);
    let mut log2sum : f32;
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < histogram_size {
            sum = sum.wrapping_add(*histogram.offset(i as (isize)) as (usize));
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    log2sum = FastLog2(sum) as (f32);
    i = 0i32 as (usize);
    'loop3: loop {
        if i < histogram_size {
            if *histogram.offset(i as (isize)) == 0i32 as (u32) {
                *cost.offset(i as (isize)) = log2sum + 2i32 as (f32);
            } else {
                *cost.offset(i as (isize)) = log2sum - FastLog2(
                                                           *histogram.offset(
                                                                i as (isize)
                                                            ) as (usize)
                                                       ) as (f32);
                if *cost.offset(i as (isize)) < 1i32 as (f32) {
                    *cost.offset(i as (isize)) = 1i32 as (f32);
                }
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop3;
        } else {
            break 'loop3;
        }
    }
}

unsafe extern fn brotli_min_float(
    mut a : f32, mut b : f32
) -> f32 {
    if a < b { a } else { b }
}

unsafe extern fn ZopfliCostModelSetFromCommands(
    mut self : *mut ZopfliCostModel,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut commands : *const Command,
    mut num_commands : usize,
    mut last_insert_len : usize
) {
    let mut histogram_literal : [u32; 256];
    let mut histogram_cmd : [u32; 704];
    let mut histogram_dist : [u32; 520];
    let mut cost_literal : [f32; 256];
    let mut pos : usize = position.wrapping_sub(last_insert_len);
    let mut min_cost_cmd : f32 = kInfinity;
    let mut i : usize;
    let mut cost_cmd : *mut f32 = (*self).cost_cmd_.as_mut_ptr();
    memset(
        histogram_literal.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u32; 256]>()
    );
    memset(
        histogram_cmd.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u32; 704]>()
    );
    memset(
        histogram_dist.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u32; 520]>()
    );
    i = 0i32 as (usize);
    'loop1: loop {
        if i < num_commands {
            let mut inslength
                : usize
                = (*commands.offset(i as (isize))).insert_len_ as (usize);
            let mut copylength
                : usize
                = CommandCopyLen(
                      &*commands.offset(i as (isize)) as (*const Command)
                  ) as (usize);
            let mut distcode
                : usize
                = (*commands.offset(i as (isize))).dist_prefix_ as (usize);
            let mut cmdcode
                : usize
                = (*commands.offset(i as (isize))).cmd_prefix_ as (usize);
            let mut j : usize;
            {
                let _rhs = 1;
                let _lhs = &mut histogram_cmd[cmdcode];
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            if cmdcode >= 128i32 as (usize) {
                let _rhs = 1;
                let _lhs = &mut histogram_dist[distcode];
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            j = 0i32 as (usize);
            'loop14: loop {
                if j < inslength {
                    {
                        let _rhs = 1;
                        let _lhs
                            = &mut histogram_literal[
                                       *ringbuffer.offset(
                                            (pos.wrapping_add(j) & ringbuffer_mask) as (isize)
                                        ) as (usize)
                                   ];
                        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                    }
                    j = j.wrapping_add(1 as (usize));
                    continue 'loop14;
                } else {
                    break 'loop14;
                }
            }
            pos = pos.wrapping_add(inslength.wrapping_add(copylength));
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    SetCost(
        histogram_literal.as_mut_ptr() as (*const u32),
        256i32 as (usize),
        cost_literal.as_mut_ptr()
    );
    SetCost(
        histogram_cmd.as_mut_ptr() as (*const u32),
        704i32 as (usize),
        cost_cmd
    );
    SetCost(
        histogram_dist.as_mut_ptr() as (*const u32),
        520i32 as (usize),
        (*self).cost_dist_.as_mut_ptr()
    );
    i = 0i32 as (usize);
    'loop3: loop {
        if i < 704i32 as (usize) {
            min_cost_cmd = brotli_min_float(
                               min_cost_cmd,
                               *cost_cmd.offset(i as (isize))
                           );
            i = i.wrapping_add(1 as (usize));
            continue 'loop3;
        } else {
            break 'loop3;
        }
    }
    (*self).min_cost_cmd_ = min_cost_cmd;
    let mut literal_costs : *mut f32 = (*self).literal_costs_;
    let mut num_bytes : usize = (*self).num_bytes_;
    *literal_costs.offset(0i32 as (isize)) = 0.0f64 as (f32);
    i = 0i32 as (usize);
    'loop5: loop {
        if i < num_bytes {
            *literal_costs.offset(
                 i.wrapping_add(1i32 as (usize)) as (isize)
             ) = *literal_costs.offset(i as (isize)) + cost_literal[
                                                           *ringbuffer.offset(
                                                                (position.wrapping_add(
                                                                     i
                                                                 ) & ringbuffer_mask) as (isize)
                                                            ) as (usize)
                                                       ];
            i = i.wrapping_add(1 as (usize));
            continue 'loop5;
        } else {
            break 'loop5;
        }
    }
}

unsafe extern fn ZopfliIterate(
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    max_backward_limit : usize,
    mut dist_cache : *const i32,
    mut model : *const ZopfliCostModel,
    mut num_matches : *const u32,
    mut matches : *const BackwardMatch,
    mut nodes : *mut ZopfliNode
) -> usize {
    let max_zopfli_len : usize = MaxZopfliLen(params);
    let mut queue : StartPosQueue;
    let mut cur_match_pos : usize = 0i32 as (usize);
    let mut i : usize;
    (*nodes.offset(0i32 as (isize))).length = 0i32 as (u32);
    (*nodes.offset(0i32 as (isize))).u.cost = 0i32 as (f32);
    InitStartPosQueue(&mut queue as (*mut StartPosQueue));
    i = 0i32 as (usize);
    'loop1: loop {
        if i.wrapping_add(3i32 as (usize)) < num_bytes {
            let mut skip
                : usize
                = UpdateNodes(
                      num_bytes,
                      position,
                      i,
                      ringbuffer,
                      ringbuffer_mask,
                      params,
                      max_backward_limit,
                      dist_cache,
                      *num_matches.offset(i as (isize)) as (usize),
                      &*matches.offset(
                            cur_match_pos as (isize)
                        ) as (*const BackwardMatch),
                      model,
                      &mut queue as (*mut StartPosQueue),
                      nodes
                  );
            if skip < 16384i32 as (usize) {
                skip = 0i32 as (usize);
            }
            cur_match_pos = cur_match_pos.wrapping_add(
                                *num_matches.offset(i as (isize)) as (usize)
                            );
            if *num_matches.offset(
                    i as (isize)
                ) == 1i32 as (u32) && (BackwardMatchLength(
                                           &*matches.offset(
                                                 cur_match_pos.wrapping_sub(
                                                     1i32 as (usize)
                                                 ) as (isize)
                                             ) as (*const BackwardMatch)
                                       ) > max_zopfli_len) {
                skip = brotli_max_size_t(
                           BackwardMatchLength(
                               &*matches.offset(
                                     cur_match_pos.wrapping_sub(1i32 as (usize)) as (isize)
                                 ) as (*const BackwardMatch)
                           ),
                           skip
                       );
            }
            if skip > 1i32 as (usize) {
                skip = skip.wrapping_sub(1 as (usize));
                'loop9: loop {
                    if skip != 0 {
                        i = i.wrapping_add(1 as (usize));
                        if i.wrapping_add(3i32 as (usize)) >= num_bytes {
                            break 'loop9;
                        } else {
                            EvaluateNode(
                                position,
                                i,
                                max_backward_limit,
                                dist_cache,
                                model,
                                &mut queue as (*mut StartPosQueue),
                                nodes
                            );
                            cur_match_pos = cur_match_pos.wrapping_add(
                                                *num_matches.offset(i as (isize)) as (usize)
                                            );
                            skip = skip.wrapping_sub(1 as (usize));
                            continue 'loop9;
                        }
                    } else {
                        break 'loop9;
                    }
                }
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    ComputeShortestPathFromNodes(num_bytes,nodes)
}

#[no_mangle]
pub unsafe extern fn BrotliCreateHqZopfliBackwardReferences(
    mut m : *mut MemoryManager,
    mut dictionary : *const BrotliDictionary,
    mut num_bytes : usize,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize,
    mut params : *const BrotliEncoderParams,
    mut hasher : *mut u8,
    mut dist_cache : *mut i32,
    mut last_insert_len : *mut usize,
    mut commands : *mut Command,
    mut num_commands : *mut usize,
    mut num_literals : *mut usize
) {
    let max_backward_limit
        : usize
        = (1i32 as (usize) << (*params).lgwin).wrapping_sub(
              16i32 as (usize)
          );
    let mut num_matches
        : *mut u32
        = if num_bytes != 0 {
              BrotliAllocate(
                  m,
                  num_bytes.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut matches_size
        : usize
        = (4i32 as (usize)).wrapping_mul(num_bytes);
    let store_end
        : usize
        = if num_bytes >= StoreLookaheadH10() {
              position.wrapping_add(num_bytes).wrapping_sub(
                  StoreLookaheadH10()
              ).wrapping_add(
                  1i32 as (usize)
              )
          } else {
              position
          };
    let mut cur_match_pos : usize = 0i32 as (usize);
    let mut i : usize;
    let mut orig_num_literals : usize;
    let mut orig_last_insert_len : usize;
    let mut orig_dist_cache : [i32; 4];
    let mut orig_num_commands : usize;
    let mut model : ZopfliCostModel;
    let mut nodes : *mut ZopfliNode;
    let mut matches
        : *mut BackwardMatch
        = if matches_size != 0 {
              BrotliAllocate(
                  m,
                  matches_size.wrapping_mul(::std::mem::size_of::<BackwardMatch>())
              ) as (*mut BackwardMatch)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut BackwardMatch)
          };
    if !(0i32 == 0) {
    } else {
        i = 0i32 as (usize);
        'loop2: loop {
            if i.wrapping_add(HashTypeLengthH10()).wrapping_sub(
                   1i32 as (usize)
               ) < num_bytes {
                let pos : usize = position.wrapping_add(i);
                let mut max_distance
                    : usize
                    = brotli_min_size_t(pos,max_backward_limit);
                let mut max_length : usize = num_bytes.wrapping_sub(i);
                let mut num_found_matches : usize;
                let mut cur_match_end : usize;
                let mut j : usize;
                if matches_size < cur_match_pos.wrapping_add(128i32 as (usize)) {
                    let mut _new_size
                        : usize
                        = if matches_size == 0i32 as (usize) {
                              cur_match_pos.wrapping_add(128i32 as (usize))
                          } else {
                              matches_size
                          };
                    let mut new_array : *mut BackwardMatch;
                    'loop17: loop {
                        if _new_size < cur_match_pos.wrapping_add(128i32 as (usize)) {
                            _new_size = _new_size.wrapping_mul(2i32 as (usize));
                            continue 'loop17;
                        } else {
                            break 'loop17;
                        }
                    }
                    new_array = if _new_size != 0 {
                                    BrotliAllocate(
                                        m,
                                        _new_size.wrapping_mul(
                                            ::std::mem::size_of::<BackwardMatch>()
                                        )
                                    ) as (*mut BackwardMatch)
                                } else {
                                    0i32 as (*mut ::std::os::raw::c_void) as (*mut BackwardMatch)
                                };
                    if !!(0i32 == 0) && (matches_size != 0i32 as (usize)) {
                        memcpy(
                            new_array as (*mut ::std::os::raw::c_void),
                            matches as (*const ::std::os::raw::c_void),
                            matches_size.wrapping_mul(::std::mem::size_of::<BackwardMatch>())
                        );
                    }
                    BrotliFree(m,matches as (*mut ::std::os::raw::c_void));
                    matches = 0i32 as (*mut ::std::os::raw::c_void) as (*mut BackwardMatch);
                    matches = new_array;
                    matches_size = _new_size;
                }
                if !(0i32 == 0) {
                    break 'loop2;
                } else {
                    num_found_matches = FindAllMatchesH10(
                                            hasher,
                                            dictionary,
                                            ringbuffer,
                                            ringbuffer_mask,
                                            pos,
                                            max_length,
                                            max_distance,
                                            params,
                                            &mut *matches.offset(
                                                      cur_match_pos as (isize)
                                                  ) as (*mut BackwardMatch)
                                        );
                    cur_match_end = cur_match_pos.wrapping_add(num_found_matches);
                    j = cur_match_pos;
                    'loop23: loop {
                        if j.wrapping_add(1i32 as (usize)) < cur_match_end {
                            if BackwardMatchLength(
                                   &mut *matches.offset(
                                             j as (isize)
                                         ) as (*mut BackwardMatch) as (*const BackwardMatch)
                               ) < BackwardMatchLength(
                                       &mut *matches.offset(
                                                 j.wrapping_add(1i32 as (usize)) as (isize)
                                             ) as (*mut BackwardMatch) as (*const BackwardMatch)
                                   ) {
                                0i32;
                            } else {
                                __assert_fail(
                                    (*b"BackwardMatchLength(&matches[j]) < BackwardMatchLength(&matches[j + 1])\0").as_ptr(
                                    ),
                                    file!().as_ptr(),
                                    line!(),
                                    (*b"BrotliCreateHqZopfliBackwardReferences\0").as_ptr()
                                );
                            }
                            if (*matches.offset(
                                     j as (isize)
                                 )).distance as (usize) > max_distance || (*matches.offset(
                                                                                j as (isize)
                                                                            )).distance <= (*matches.offset(
                                                                                                 j.wrapping_add(
                                                                                                     1i32 as (usize)
                                                                                                 ) as (isize)
                                                                                             )).distance {
                                0i32;
                            } else {
                                __assert_fail(
                                    (*b"matches[j].distance > max_distance || matches[j].distance <= matches[j + 1].distance\0").as_ptr(
                                    ),
                                    file!().as_ptr(),
                                    line!(),
                                    (*b"BrotliCreateHqZopfliBackwardReferences\0").as_ptr()
                                );
                            }
                            j = j.wrapping_add(1 as (usize));
                            continue 'loop23;
                        } else {
                            break 'loop23;
                        }
                    }
                    *num_matches.offset(i as (isize)) = num_found_matches as (u32);
                    if num_found_matches > 0i32 as (usize) {
                        let match_len
                            : usize
                            = BackwardMatchLength(
                                  &mut *matches.offset(
                                            cur_match_end.wrapping_sub(1i32 as (usize)) as (isize)
                                        ) as (*mut BackwardMatch) as (*const BackwardMatch)
                              );
                        if match_len > 325i32 as (usize) {
                            let skip : usize = match_len.wrapping_sub(1i32 as (usize));
                            *matches.offset(
                                 {
                                     let _old = cur_match_pos;
                                     cur_match_pos = cur_match_pos.wrapping_add(1 as (usize));
                                     _old
                                 } as (isize)
                             ) = *matches.offset(
                                      cur_match_end.wrapping_sub(1i32 as (usize)) as (isize)
                                  );
                            *num_matches.offset(i as (isize)) = 1i32 as (u32);
                            StoreRangeH10(
                                hasher,
                                ringbuffer,
                                ringbuffer_mask,
                                pos.wrapping_add(1i32 as (usize)),
                                brotli_min_size_t(pos.wrapping_add(match_len),store_end)
                            );
                            memset(
                                &mut *num_matches.offset(
                                          i.wrapping_add(1i32 as (usize)) as (isize)
                                      ) as (*mut u32) as (*mut ::std::os::raw::c_void),
                                0i32,
                                skip.wrapping_mul(::std::mem::size_of::<u32>())
                            );
                            i = i.wrapping_add(skip);
                        } else {
                            cur_match_pos = cur_match_end;
                        }
                    }
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop2;
                }
            } else {
                orig_num_literals = *num_literals;
                orig_last_insert_len = *last_insert_len;
                memcpy(
                    orig_dist_cache.as_mut_ptr() as (*mut ::std::os::raw::c_void),
                    dist_cache as (*const ::std::os::raw::c_void),
                    (4i32 as (usize)).wrapping_mul(::std::mem::size_of::<i32>())
                );
                orig_num_commands = *num_commands;
                nodes = if num_bytes.wrapping_add(1i32 as (usize)) != 0 {
                            BrotliAllocate(
                                m,
                                num_bytes.wrapping_add(1i32 as (usize)).wrapping_mul(
                                    ::std::mem::size_of::<ZopfliNode>()
                                )
                            ) as (*mut ZopfliNode)
                        } else {
                            0i32 as (*mut ::std::os::raw::c_void) as (*mut ZopfliNode)
                        };
                if !(0i32 == 0) {
                    return;
                } else {
                    InitZopfliCostModel(
                        m,
                        &mut model as (*mut ZopfliCostModel),
                        num_bytes
                    );
                    if !(0i32 == 0) {
                        return;
                    } else {
                        i = 0i32 as (usize);
                        'loop6: loop {
                            if i < 2i32 as (usize) {
                                BrotliInitZopfliNodes(
                                    nodes,
                                    num_bytes.wrapping_add(1i32 as (usize))
                                );
                                if i == 0i32 as (usize) {
                                    ZopfliCostModelSetFromLiteralCosts(
                                        &mut model as (*mut ZopfliCostModel),
                                        position,
                                        ringbuffer,
                                        ringbuffer_mask
                                    );
                                } else {
                                    ZopfliCostModelSetFromCommands(
                                        &mut model as (*mut ZopfliCostModel),
                                        position,
                                        ringbuffer,
                                        ringbuffer_mask,
                                        commands as (*const Command),
                                        (*num_commands).wrapping_sub(orig_num_commands),
                                        orig_last_insert_len
                                    );
                                }
                                *num_commands = orig_num_commands;
                                *num_literals = orig_num_literals;
                                *last_insert_len = orig_last_insert_len;
                                memcpy(
                                    dist_cache as (*mut ::std::os::raw::c_void),
                                    orig_dist_cache.as_mut_ptr() as (*const ::std::os::raw::c_void),
                                    (4i32 as (usize)).wrapping_mul(::std::mem::size_of::<i32>())
                                );
                                *num_commands = (*num_commands).wrapping_add(
                                                    ZopfliIterate(
                                                        num_bytes,
                                                        position,
                                                        ringbuffer,
                                                        ringbuffer_mask,
                                                        params,
                                                        max_backward_limit,
                                                        dist_cache as (*const i32),
                                                        &mut model as (*mut ZopfliCostModel) as (*const ZopfliCostModel),
                                                        num_matches as (*const u32),
                                                        matches as (*const BackwardMatch),
                                                        nodes
                                                    )
                                                );
                                BrotliZopfliCreateCommands(
                                    num_bytes,
                                    position,
                                    max_backward_limit,
                                    nodes as (*const ZopfliNode),
                                    dist_cache,
                                    last_insert_len,
                                    commands,
                                    num_literals
                                );
                                i = i.wrapping_add(1 as (usize));
                                continue 'loop6;
                            } else {
                                break 'loop6;
                            }
                        }
                        CleanupZopfliCostModel(m,&mut model as (*mut ZopfliCostModel));
                        BrotliFree(m,nodes as (*mut ::std::os::raw::c_void));
                        nodes = 0i32 as (*mut ::std::os::raw::c_void) as (*mut ZopfliNode);
                        BrotliFree(m,matches as (*mut ::std::os::raw::c_void));
                        matches = 0i32 as (*mut ::std::os::raw::c_void) as (*mut BackwardMatch);
                        BrotliFree(m,num_matches as (*mut ::std::os::raw::c_void));
                        num_matches = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
                    }
                }
            }
        }
    }
}
