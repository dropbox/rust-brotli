extern {
    fn BrotliAllocate(
        m : *mut MemoryManager, n : usize
    ) -> *mut std::os::raw::c_void;
    fn BrotliEstimateBitCostsForLiterals(
        pos : usize,
        len : usize,
        mask : usize,
        data : *const u8,
        cost : *mut f32
    );
    fn BrotliFindAllStaticDictionaryMatches(
        dictionary : *const BrotliEncoderDictionary,
        data : *const u8,
        min_length : usize,
        max_length : usize,
        matches : *mut u32
    ) -> i32;
    fn BrotliFree(
        m : *mut MemoryManager, p : *mut std::os::raw::c_void
    );
    fn log2(__x : f64) -> f64;
    fn memcpy(
        __dest : *mut std::os::raw::c_void,
        __src : *const std::os::raw::c_void,
        __n : usize
    ) -> *mut std::os::raw::c_void;
    fn memset(
        __s : *mut std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut std::os::raw::c_void;
}

static mut kLog2Table
    : *const f32
    = 0.0000000000000000f32 as (*const f32);

static kDictNumBits : i32 = 15i32;

static kDictHashMul32 : u32 = 0x1e35a7bdi32 as (u32);

static mut kStaticDictionaryBuckets
    : *const u16
    = 1i32 as (*const u16);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct DictWord {
    pub len : u8,
    pub transform : u8,
    pub idx : u16,
}

static mut kStaticDictionaryWords
    : *const DictWord
    = 0i32 as (*const DictWord);

static mut kInsBase : *mut u32 = 0i32 as (*mut u32);

static mut kInsExtra : *mut u32 = 0i32 as (*mut u32);

static mut kCopyBase : *mut u32 = 2i32 as (*mut u32);

static mut kCopyExtra : *mut u32 = 0i32 as (*mut u32);

#[no_mangle]
pub unsafe extern fn ctzll(mut x : usize) -> usize {
    let mut count : u8 = 0i32 as (u8);
    while x & 0i32 as (usize) != 0 {
        count = (count as (i32) + 1i32) as (u8);
        x = x >> 1i32;
    }
    count as (usize)
}

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

static mut kDistanceCacheIndex : *const u32 = 0i32 as (*const u32);

static mut kDistanceCacheOffset
    : *const i32
    = 0i32 as (*const i32);

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
    pub dcode_insert_length : u32,
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
    stub.dcode_insert_length = 0i32 as (u32);
    stub.u.cost = kInfinity;
    i = 0i32 as (usize);
    while i < length {
        *array.offset(i as (isize)) = stub;
        i = i.wrapping_add(1 as (usize));
    }
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
pub struct BrotliDistanceParams {
    pub distance_postfix_bits : u32,
    pub num_direct_distance_codes : u32,
    pub alphabet_size : u32,
    pub max_distance : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length : *mut u8,
    pub offsets_by_length : *mut u32,
    pub data_size : usize,
    pub data : *const u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrotliEncoderDictionary {
    pub words : *const BrotliDictionary,
    pub cutoffTransformsCount : u32,
    pub cutoffTransforms : usize,
    pub hash_table : *const u16,
    pub buckets : *const u16,
    pub dict_words : *const DictWord,
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
    pub large_window : i32,
    pub hasher : BrotliHasherParams,
    pub dist : BrotliDistanceParams,
    pub dictionary : BrotliEncoderDictionary,
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
    (*self).length & 0x1ffffffi32 as (u32)
}

unsafe extern fn ZopfliNodeCopyDistance(
    mut self : *const ZopfliNode
) -> u32 {
    (*self).distance
}

unsafe extern fn ZopfliNodeLengthCode(
    mut self : *const ZopfliNode
) -> u32 {
    let modifier : u32 = (*self).length >> 25i32;
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
    let short_code : u32 = (*self).dcode_insert_length >> 27i32;
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
    while {
              n = n >> 1i32;
              n
          } != 0 {
        result = result.wrapping_add(1 as (u32));
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
        *code = (nbits << 10i32 | (16i32 as (usize)).wrapping_add(
                                      num_direct_codes
                                  ).wrapping_add(
                                      (2i32 as (usize)).wrapping_mul(
                                          nbits.wrapping_sub(1i32 as (usize))
                                      ).wrapping_add(
                                          prefix
                                      ) << postfix_bits
                                  ).wrapping_add(
                                      postfix
                                  )) as (u16);
        *extra_bits = (dist.wrapping_sub(offset) >> postfix_bits) as (u32);
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
    mut dist : *const BrotliDistanceParams,
    mut insertlen : usize,
    mut copylen : usize,
    mut copylen_code_delta : i32,
    mut distance_code : usize
) {
    let mut delta : u32 = copylen_code_delta as (i8) as (u8) as (u32);
    (*self).insert_len_ = insertlen as (u32);
    (*self).copy_len_ = (copylen | (delta << 25i32) as (usize)) as (u32);
    PrefixEncodeCopyDistance(
        distance_code,
        (*dist).num_direct_distance_codes as (usize),
        (*dist).distance_postfix_bits as (usize),
        &mut (*self).dist_prefix_ as (*mut u16),
        &mut (*self).dist_extra_ as (*mut u32)
    );
    GetLengthCode(
        insertlen,
        (copylen as (i32) + copylen_code_delta) as (usize),
        if !!((*self).dist_prefix_ as (i32) & 0x3ffi32 == 0i32) {
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
    mut params : *const BrotliEncoderParams,
    mut commands : *mut Command,
    mut num_literals : *mut usize
) {
    let mut pos : usize = 0i32 as (usize);
    let mut offset : u32 = (*nodes.offset(0i32 as (isize))).u.next;
    let mut i : usize;
    let mut gap : usize = 0i32 as (usize);
    i = 0i32 as (usize);
    while offset != !(0i32 as (u32)) {
        {
            let mut next
                : *const ZopfliNode
                = &*nodes.offset(
                        pos.wrapping_add(offset as (usize)) as (isize)
                    ) as (*const ZopfliNode);
            let mut copy_length
                : usize
                = ZopfliNodeCopyLength(next) as (usize);
            let mut insert_length
                : usize
                = ((*next).dcode_insert_length & 0x7ffffffi32 as (u32)) as (usize);
            pos = pos.wrapping_add(insert_length);
            offset = (*next).u.next;
            if i == 0i32 as (usize) {
                insert_length = insert_length.wrapping_add(*last_insert_len);
                *last_insert_len = 0i32 as (usize);
            }
            {
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
                    = if !!(distance > max_distance.wrapping_add(gap)) {
                          1i32
                      } else {
                          0i32
                      };
                let mut dist_code
                    : usize
                    = ZopfliNodeDistanceCode(next) as (usize);
                InitCommand(
                    &mut *commands.offset(i as (isize)) as (*mut Command),
                    &(*params).dist as (*const BrotliDistanceParams),
                    insert_length,
                    copy_length,
                    len_code as (i32) - copy_length as (i32),
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
            }
            *num_literals = (*num_literals).wrapping_add(insert_length);
            pos = pos.wrapping_add(copy_length);
        }
        i = i.wrapping_add(1 as (usize));
    }
    *last_insert_len = (*last_insert_len).wrapping_add(
                           num_bytes.wrapping_sub(pos)
                       );
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func : unsafe extern fn(*mut std::os::raw::c_void, usize) -> *mut std::os::raw::c_void,
    pub free_func : unsafe extern fn(*mut std::os::raw::c_void, *mut std::os::raw::c_void),
    pub opaque : *mut std::os::raw::c_void,
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
    pub cost_cmd_ : *mut f32,
    pub cost_dist_ : *mut f32,
    pub distance_histogram_size : u32,
    pub literal_costs_ : *mut f32,
    pub min_cost_cmd_ : f32,
    pub num_bytes_ : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PosData {
    pub pos : usize,
    pub distance_cache : *mut i32,
    pub costdiff : f32,
    pub cost : f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StartPosQueue {
    pub q_ : *mut PosData,
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
    mut dist : *const BrotliDistanceParams,
    mut num_bytes : usize
) {
    let mut distance_histogram_size : u32 = (*dist).alphabet_size;
    if distance_histogram_size > 544i32 as (u32) {
        distance_histogram_size = 544i32 as (u32);
    }
    (*self).num_bytes_ = num_bytes;
    (*self).literal_costs_ = if num_bytes.wrapping_add(
                                    2i32 as (usize)
                                ) > 0i32 as (usize) {
                                 BrotliAllocate(
                                     m,
                                     num_bytes.wrapping_add(2i32 as (usize)).wrapping_mul(
                                         std::mem::size_of::<f32>()
                                     )
                                 ) as (*mut f32)
                             } else {
                                 0i32 as (*mut std::os::raw::c_void) as (*mut f32)
                             };
    (*self).cost_dist_ = if (*dist).alphabet_size > 0i32 as (u32) {
                             BrotliAllocate(
                                 m,
                                 ((*dist).alphabet_size as (usize)).wrapping_mul(
                                     std::mem::size_of::<f32>()
                                 )
                             ) as (*mut f32)
                         } else {
                             0i32 as (*mut std::os::raw::c_void) as (*mut f32)
                         };
    (*self).distance_histogram_size = distance_histogram_size;
    if !(0i32 == 0) { }
}

unsafe extern fn FastLog2(mut v : usize) -> f64 {
    if v < std::mem::size_of::<*const f32>().wrapping_div(
               std::mem::size_of::<f32>()
           ) {
        return *kLog2Table.offset(v as (isize)) as (f64);
    }
    log2(v as (f64))
}

unsafe extern fn ZopfliCostModelSetFromLiteralCosts(
    mut self : *mut ZopfliCostModel,
    mut position : usize,
    mut ringbuffer : *const u8,
    mut ringbuffer_mask : usize
) {
    let mut literal_costs : *mut f32 = (*self).literal_costs_;
    let mut literal_carry : f32 = 0.0f64 as (f32);
    let mut cost_dist : *mut f32 = (*self).cost_dist_;
    let mut cost_cmd : *mut f32 = (*self).cost_cmd_;
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
    while i < num_bytes {
        {
            literal_carry = literal_carry + *literal_costs.offset(
                                                 i.wrapping_add(1i32 as (usize)) as (isize)
                                             );
            *literal_costs.offset(
                 i.wrapping_add(1i32 as (usize)) as (isize)
             ) = *literal_costs.offset(i as (isize)) + literal_carry;
            literal_carry = literal_carry - (*literal_costs.offset(
                                                  i.wrapping_add(1i32 as (usize)) as (isize)
                                              ) - *literal_costs.offset(i as (isize)));
        }
        i = i.wrapping_add(1 as (usize));
    }
    i = 0i32 as (usize);
    while i < 704i32 as (usize) {
        {
            *cost_cmd.offset(i as (isize)) = FastLog2(
                                                 (11i32 as (u32)).wrapping_add(
                                                     i as (u32)
                                                 ) as (usize)
                                             ) as (f32);
        }
        i = i.wrapping_add(1 as (usize));
    }
    i = 0i32 as (usize);
    while i < (*self).distance_histogram_size as (usize) {
        {
            *cost_dist.offset(i as (isize)) = FastLog2(
                                                  (20i32 as (u32)).wrapping_add(
                                                      i as (u32)
                                                  ) as (usize)
                                              ) as (f32);
        }
        i = i.wrapping_add(1 as (usize));
    }
    (*self).min_cost_cmd_ = FastLog2(11i32 as (usize)) as (f32);
}

unsafe extern fn InitStartPosQueue(mut self : *mut StartPosQueue) {
    (*self).idx_ = 0i32 as (usize);
}

unsafe extern fn BrotliUnalignedRead64(
    mut p : *const std::os::raw::c_void
) -> usize {
    *(p as (*const usize))
}

unsafe extern fn FindMatchLengthWithLimit(
    mut s1 : *const u8, mut s2 : *const u8, mut limit : usize
) -> usize {
    let mut matched : usize = 0i32 as (usize);
    let mut limit2
        : usize
        = (limit >> 3i32).wrapping_add(1i32 as (usize));
    while {
              limit2 = limit2.wrapping_sub(1 as (usize));
              limit2
          } != 0 {
        if BrotliUnalignedRead64(
               s2 as (*const std::os::raw::c_void)
           ) == BrotliUnalignedRead64(
                    s1.offset(matched as (isize)) as (*const std::os::raw::c_void)
                ) {
            s2 = s2.offset(8i32 as (isize));
            matched = matched.wrapping_add(8i32 as (usize));
        } else {
            let mut x
                : usize
                = BrotliUnalignedRead64(
                      s2 as (*const std::os::raw::c_void)
                  ) ^ BrotliUnalignedRead64(
                          s1.offset(matched as (isize)) as (*const std::os::raw::c_void)
                      );
            let mut matching_bits : usize = ctzll(x) as (usize);
            matched = matched.wrapping_add(matching_bits >> 3i32);
            return matched;
        }
    }
    limit = (limit & 7i32 as (usize)).wrapping_add(1i32 as (usize));
    while {
              limit = limit.wrapping_sub(1 as (usize));
              limit
          } != 0 {
        if *s1.offset(matched as (isize)) as (i32) == *s2 as (i32) {
            s2 = s2.offset(1 as (isize));
            matched = matched.wrapping_add(1 as (usize));
        } else {
            return matched;
        }
    }
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
    pub buckets_ : *mut u32,
    pub invalid_pos_ : u32,
}

unsafe extern fn BrotliUnalignedRead32(
    mut p : *const std::os::raw::c_void
) -> u32 {
    *(p as (*const u32))
}

unsafe extern fn HashBytesH10(mut data : *const u8) -> u32 {
    let mut h
        : u32
        = BrotliUnalignedRead32(
              data as (*const std::os::raw::c_void)
          ).wrapping_mul(
              kHashMul32
          );
    h >> 32i32 - 17i32
}

unsafe extern fn ForestH10(mut self : *mut H10) -> *mut u32 {
    &mut *self.offset(1i32 as (isize)) as (*mut H10) as (*mut u32)
}

unsafe extern fn LeftChildIndexH10(
    mut self : *mut H10, pos : usize
) -> usize {
    (2i32 as (usize)).wrapping_mul(pos & (*self).window_mask_)
}

unsafe extern fn RightChildIndexH10(
    mut self : *mut H10, pos : usize
) -> usize {
    (2i32 as (usize)).wrapping_mul(
        pos & (*self).window_mask_
    ).wrapping_add(
        1i32 as (usize)
    )
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
    let cur_ix_masked : usize = cur_ix & ring_buffer_mask;
    let max_comp_len
        : usize
        = brotli_min_size_t(max_length,128i32 as (usize));
    let should_reroot_tree
        : i32
        = if !!(max_length >= 128i32 as (usize)) { 1i32 } else { 0i32 };
    let key
        : u32
        = HashBytesH10(
              &*data.offset(cur_ix_masked as (isize)) as (*const u8)
          );
    let mut forest : *mut u32 = ForestH10(self);
    let mut prev_ix
        : usize
        = *(*self).buckets_.offset(key as (isize)) as (usize);
    let mut node_left : usize = LeftChildIndexH10(self,cur_ix);
    let mut node_right : usize = RightChildIndexH10(self,cur_ix);
    let mut best_len_left : usize = 0i32 as (usize);
    let mut best_len_right : usize = 0i32 as (usize);
    let mut depth_remaining : usize;
    if should_reroot_tree != 0 {
        *(*self).buckets_.offset(key as (isize)) = cur_ix as (u32);
    }
    depth_remaining = 64i32 as (usize);
    'break16: loop {
        {
            let backward : usize = cur_ix.wrapping_sub(prev_ix);
            let prev_ix_masked : usize = prev_ix & ring_buffer_mask;
            if backward == 0i32 as (usize) || backward > max_backward || depth_remaining == 0i32 as (usize) {
                if should_reroot_tree != 0 {
                    *forest.offset(node_left as (isize)) = (*self).invalid_pos_;
                    *forest.offset(node_right as (isize)) = (*self).invalid_pos_;
                }
                break 'break16;
            }
            {
                let cur_len
                    : usize
                    = brotli_min_size_t(best_len_left,best_len_right);
                let mut len : usize;
                len = cur_len.wrapping_add(
                          FindMatchLengthWithLimit(
                              &*data.offset(
                                    cur_ix_masked.wrapping_add(cur_len) as (isize)
                                ) as (*const u8),
                              &*data.offset(
                                    prev_ix_masked.wrapping_add(cur_len) as (isize)
                                ) as (*const u8),
                              max_length.wrapping_sub(cur_len)
                          )
                      );
                if !matches.is_null() && (len > *best_len) {
                    *best_len = len;
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
                if len >= max_comp_len {
                    if should_reroot_tree != 0 {
                        *forest.offset(node_left as (isize)) = *forest.offset(
                                                                    LeftChildIndexH10(
                                                                        self,
                                                                        prev_ix
                                                                    ) as (isize)
                                                                );
                        *forest.offset(node_right as (isize)) = *forest.offset(
                                                                     RightChildIndexH10(
                                                                         self,
                                                                         prev_ix
                                                                     ) as (isize)
                                                                 );
                    }
                    break 'break16;
                }
                if *data.offset(
                        cur_ix_masked.wrapping_add(len) as (isize)
                    ) as (i32) > *data.offset(
                                      prev_ix_masked.wrapping_add(len) as (isize)
                                  ) as (i32) {
                    best_len_left = len;
                    if should_reroot_tree != 0 {
                        *forest.offset(node_left as (isize)) = prev_ix as (u32);
                    }
                    node_left = RightChildIndexH10(self,prev_ix);
                    prev_ix = *forest.offset(node_left as (isize)) as (usize);
                } else {
                    best_len_right = len;
                    if should_reroot_tree != 0 {
                        *forest.offset(node_right as (isize)) = prev_ix as (u32);
                    }
                    node_right = LeftChildIndexH10(self,prev_ix);
                    prev_ix = *forest.offset(node_right as (isize)) as (usize);
                }
            }
        }
        depth_remaining = depth_remaining.wrapping_sub(1 as (usize));
    }
    matches
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Struct18 {
    pub params : BrotliHasherParams,
    pub is_prepared_ : i32,
    pub dict_num_lookups : usize,
    pub dict_num_matches : usize,
}

unsafe extern fn GetHasherCommon(
    mut handle : *mut u8
) -> *mut Struct18 {
    handle as (*mut Struct18)
}

unsafe extern fn SelfH10(mut handle : *mut u8) -> *mut H10 {
    &mut *GetHasherCommon(handle).offset(
              1i32 as (isize)
          ) as (*mut Struct18) as (*mut H10)
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
    mut dictionary : *const BrotliEncoderDictionary,
    mut data : *const u8,
    ring_buffer_mask : usize,
    cur_ix : usize,
    max_length : usize,
    max_backward : usize,
    gap : usize,
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
    let mut dict_matches : *mut u32;
    let mut i : usize;
    if cur_ix < short_match_max_backward {
        stop = 0i32 as (usize);
    }
    i = cur_ix.wrapping_sub(1i32 as (usize));
    'break14: while i > stop && (best_len <= 2i32 as (usize)) {
        'continue15: loop {
            {
                let mut prev_ix : usize = i;
                let backward : usize = cur_ix.wrapping_sub(prev_ix);
                if backward > max_backward {
                    break 'break14;
                }
                prev_ix = prev_ix & ring_buffer_mask;
                if *data.offset(cur_ix_masked as (isize)) as (i32) != *data.offset(
                                                                           prev_ix as (isize)
                                                                       ) as (i32) || *data.offset(
                                                                                          cur_ix_masked.wrapping_add(
                                                                                              1i32 as (usize)
                                                                                          ) as (isize)
                                                                                      ) as (i32) != *data.offset(
                                                                                                         prev_ix.wrapping_add(
                                                                                                             1i32 as (usize)
                                                                                                         ) as (isize)
                                                                                                     ) as (i32) {
                    break 'continue15;
                }
                {
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
            }
            break;
        }
        i = i.wrapping_sub(1 as (usize));
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
    while i <= 37i32 as (usize) {
        {
            *dict_matches.offset(i as (isize)) = kInvalidMatch;
        }
        i = i.wrapping_add(1 as (usize));
    }
    {
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
               &mut *dict_matches.offset(0i32 as (isize)) as (*mut u32)
           ) != 0 {
            let mut maxlen
                : usize
                = brotli_min_size_t(37i32 as (usize),max_length);
            let mut l : usize;
            l = minlen;
            while l <= maxlen {
                {
                    let mut dict_id : u32 = *dict_matches.offset(l as (isize));
                    if dict_id < kInvalidMatch {
                        let mut distance
                            : usize
                            = max_backward.wrapping_add(gap).wrapping_add(
                                  (dict_id >> 5i32) as (usize)
                              ).wrapping_add(
                                  1i32 as (usize)
                              );
                        if distance <= (*params).dist.max_distance {
                            InitDictionaryBackwardMatch(
                                {
                                    let _old = matches;
                                    matches = matches.offset(1 as (isize));
                                    _old
                                },
                                distance,
                                l,
                                (dict_id & 31i32 as (u32)) as (usize)
                            );
                        }
                    }
                }
                l = l.wrapping_add(1 as (usize));
            }
        }
    }
    ((matches as (isize)).wrapping_sub(
         orig_matches as (isize)
     ) / std::mem::size_of::<*mut BackwardMatch>(
         ) as (isize)) as (usize)
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
    gap : usize,
    mut nodes : *const ZopfliNode
) -> u32 {
    let clen
        : usize
        = ZopfliNodeCopyLength(
              &*nodes.offset(pos as (isize)) as (*const ZopfliNode)
          ) as (usize);
    let ilen
        : usize
        = ((*nodes.offset(
                 pos as (isize)
             )).dcode_insert_length & 0x7ffffffi32 as (u32)) as (usize);
    let dist
        : usize
        = ZopfliNodeCopyDistance(
              &*nodes.offset(pos as (isize)) as (*const ZopfliNode)
          ) as (usize);
    if pos == 0i32 as (usize) {
        0i32 as (u32)
    } else if dist.wrapping_add(clen) <= block_start.wrapping_add(
                                             pos
                                         ).wrapping_add(
                                             gap
                                         ) && (dist <= max_backward.wrapping_add(
                                                           gap
                                                       )) && (ZopfliNodeDistanceCode(
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
    while idx < 4i32 && (p > 0i32 as (usize)) {
        let ilen
            : usize
            = ((*nodes.offset(
                     p as (isize)
                 )).dcode_insert_length & 0x7ffffffi32 as (u32)) as (usize);
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
    }
    while idx < 4i32 {
        {
            *dist_cache.offset(idx as (isize)) = *{
                                                      let _old = starting_dist_cache;
                                                      starting_dist_cache = starting_dist_cache.offset(
                                                                                1 as (isize)
                                                                            );
                                                      _old
                                                  };
        }
        idx = idx + 1;
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
    let mut q : *mut PosData = (*self).q_;
    *q.offset(offset as (isize)) = *posdata;
    i = 1i32 as (usize);
    while i < len {
        {
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
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn EvaluateNode(
    block_start : usize,
    pos : usize,
    max_backward_limit : usize,
    gap : usize,
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
                          gap,
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
            posdata.distance_cache
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
    &mut *(*self).q_.offset(
              (k.wrapping_sub((*self).idx_) & 7i32 as (usize)) as (isize)
          ) as (*mut PosData) as (*const PosData)
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
    while pos.wrapping_add(len) <= num_bytes && ((*nodes.offset(
                                                       pos.wrapping_add(len) as (isize)
                                                   )).u.cost <= min_cost) {
        len = len.wrapping_add(1 as (usize));
        if len == next_len_offset {
            min_cost = min_cost + 1.0f32;
            next_len_offset = next_len_offset.wrapping_add(next_len_bucket);
            next_len_bucket = next_len_bucket.wrapping_mul(2i32 as (usize));
        }
    }
    len
}

unsafe extern fn GetInsertExtra(mut inscode : u16) -> u32 {
    *kInsExtra.offset(inscode as (isize))
}

unsafe extern fn ZopfliCostModelGetDistanceCost(
    mut self : *const ZopfliCostModel, mut distcode : usize
) -> f32 {
    *(*self).cost_dist_.offset(distcode as (isize))
}

unsafe extern fn GetCopyExtra(mut copycode : u16) -> u32 {
    *kCopyExtra.offset(copycode as (isize))
}

unsafe extern fn ZopfliCostModelGetCommandCost(
    mut self : *const ZopfliCostModel, mut cmdcode : u16
) -> f32 {
    *(*self).cost_cmd_.offset(cmdcode as (isize))
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
                            ) << 25i32) as (u32);
    (*next).distance = dist as (u32);
    (*next).dcode_insert_length = (short_code << 27i32 | pos.wrapping_sub(
                                                             start_pos
                                                         )) as (u32);
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
    let mut gap : usize = 0i32 as (usize);
    EvaluateNode(
        block_start,
        pos,
        max_backward_limit,
        gap,
        starting_dist_cache,
        model,
        queue,
        nodes
    );
    {
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
    }
    k = 0i32 as (usize);
    while k < max_iters && (k < StartPosQueueSize(
                                    queue as (*const StartPosQueue)
                                )) {
        'continue28: loop {
            {
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
                'break29: while j < 16i32 as (usize) && (best_len < max_len) {
                    'continue30: loop {
                        {
                            let idx
                                : usize
                                = *kDistanceCacheIndex.offset(j as (isize)) as (usize);
                            let backward
                                : usize
                                = (*(*posdata).distance_cache.offset(
                                        idx as (isize)
                                    ) + *kDistanceCacheOffset.offset(j as (isize))) as (usize);
                            let mut prev_ix : usize = cur_ix.wrapping_sub(backward);
                            let mut len : usize = 0i32 as (usize);
                            let mut continuation
                                : u8
                                = *ringbuffer.offset(
                                       cur_ix_masked.wrapping_add(best_len) as (isize)
                                   );
                            if cur_ix_masked.wrapping_add(best_len) > ringbuffer_mask {
                                break 'break29;
                            }
                            if backward > max_distance.wrapping_add(gap) {
                                break 'continue30;
                            }
                            if backward <= max_distance {
                                if prev_ix >= cur_ix {
                                    break 'continue30;
                                }
                                prev_ix = prev_ix & ringbuffer_mask;
                                if prev_ix.wrapping_add(
                                       best_len
                                   ) > ringbuffer_mask || continuation as (i32) != *ringbuffer.offset(
                                                                                        prev_ix.wrapping_add(
                                                                                            best_len
                                                                                        ) as (isize)
                                                                                    ) as (i32) {
                                    break 'continue30;
                                }
                                len = FindMatchLengthWithLimit(
                                          &*ringbuffer.offset(prev_ix as (isize)) as (*const u8),
                                          &*ringbuffer.offset(
                                                cur_ix_masked as (isize)
                                            ) as (*const u8),
                                          max_len
                                      );
                            } else {
                                break 'continue30;
                            }
                            {
                                let dist_cost
                                    : f32
                                    = base_cost + ZopfliCostModelGetDistanceCost(model,j);
                                let mut l : usize;
                                l = best_len.wrapping_add(1i32 as (usize));
                                while l <= len {
                                    {
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
                                    }
                                    l = l.wrapping_add(1 as (usize));
                                }
                            }
                        }
                        break;
                    }
                    j = j.wrapping_add(1 as (usize));
                }
                if k >= 2i32 as (usize) {
                    break 'continue28;
                }
                {
                    let mut len : usize = min_len;
                    j = 0i32 as (usize);
                    while j < num_matches {
                        {
                            let mut match_ : BackwardMatch = *matches.offset(j as (isize));
                            let mut dist : usize = match_.distance as (usize);
                            let mut is_dictionary_match
                                : i32
                                = if !!(dist > max_distance.wrapping_add(gap)) {
                                      1i32
                                  } else {
                                      0i32
                                  };
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
                                (*params).dist.num_direct_distance_codes as (usize),
                                (*params).dist.distance_postfix_bits as (usize),
                                &mut dist_symbol as (*mut u16),
                                &mut distextra as (*mut u32)
                            );
                            distnumextra = (dist_symbol as (i32) >> 10i32) as (u32);
                            dist_cost = base_cost + distnumextra as (f32) + ZopfliCostModelGetDistanceCost(
                                                                                model,
                                                                                (dist_symbol as (i32) & 0x3ffi32) as (usize)
                                                                            );
                            max_match_len = BackwardMatchLength(
                                                &mut match_ as (*mut BackwardMatch) as (*const BackwardMatch)
                                            );
                            if len < max_match_len && (is_dictionary_match != 0 || max_match_len > max_zopfli_len) {
                                len = max_match_len;
                            }
                            while len <= max_match_len {
                                {
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
                                }
                                len = len.wrapping_add(1 as (usize));
                            }
                        }
                        j = j.wrapping_add(1 as (usize));
                    }
                }
            }
            break;
        }
        k = k.wrapping_add(1 as (usize));
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
        0i32 as (*mut std::os::raw::c_void) as (*mut usize),
        0i32 as (*mut std::os::raw::c_void) as (*mut BackwardMatch)
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
        while j < i {
            {
                StoreH10(handle,data,mask,j);
            }
            j = j.wrapping_add(8i32 as (usize));
        }
    }
    while i < ix_end {
        {
            StoreH10(handle,data,mask,i);
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn HashTypeLengthH10() -> usize { 4i32 as (usize) }

unsafe extern fn CleanupZopfliCostModel(
    mut m : *mut MemoryManager, mut self : *mut ZopfliCostModel
) {
    {
        BrotliFree(
            m,
            (*self).literal_costs_ as (*mut std::os::raw::c_void)
        );
        (*self).literal_costs_ = 0i32 as (*mut std::os::raw::c_void) as (*mut f32);
    }
    {
        BrotliFree(m,(*self).cost_dist_ as (*mut std::os::raw::c_void));
        (*self).cost_dist_ = 0i32 as (*mut std::os::raw::c_void) as (*mut f32);
    }
}

unsafe extern fn ZopfliNodeCommandLength(
    mut self : *const ZopfliNode
) -> u32 {
    ZopfliNodeCopyLength(self).wrapping_add(
        (*self).dcode_insert_length & 0x7ffffffi32 as (u32)
    )
}

unsafe extern fn ComputeShortestPathFromNodes(
    mut num_bytes : usize, mut nodes : *mut ZopfliNode
) -> usize {
    let mut index : usize = num_bytes;
    let mut num_commands : usize = 0i32 as (usize);
    while (*nodes.offset(
                index as (isize)
            )).dcode_insert_length & 0x7ffffffi32 as (u32) == 0i32 as (u32) && ((*nodes.offset(
                                                                                      index as (isize)
                                                                                  )).length == 1i32 as (u32)) {
        index = index.wrapping_sub(1 as (usize));
    }
    (*nodes.offset(index as (isize))).u.next = !(0i32 as (u32));
    while index != 0i32 as (usize) {
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
    }
    num_commands
}

#[no_mangle]
pub unsafe extern fn BrotliZopfliComputeShortestPath(
    mut m : *mut MemoryManager,
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
    let mut matches : *mut BackwardMatch;
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
    let mut gap : usize = 0i32 as (usize);
    let mut lz_matches_offset : usize = 0i32 as (usize);
    (*nodes.offset(0i32 as (isize))).length = 0i32 as (u32);
    (*nodes.offset(0i32 as (isize))).u.cost = 0i32 as (f32);
    InitZopfliCostModel(
        m,
        &mut model as (*mut ZopfliCostModel),
        &(*params).dist as (*const BrotliDistanceParams),
        num_bytes
    );
    if !(0i32 == 0) {
        return 0i32 as (usize);
    }
    ZopfliCostModelSetFromLiteralCosts(
        &mut model as (*mut ZopfliCostModel),
        position,
        ringbuffer,
        ringbuffer_mask
    );
    InitStartPosQueue(&mut queue as (*mut StartPosQueue));
    i = 0i32 as (usize);
    while i.wrapping_add(HashTypeLengthH10()).wrapping_sub(
              1i32 as (usize)
          ) < num_bytes {
        {
            let pos : usize = position.wrapping_add(i);
            let max_distance
                : usize
                = brotli_min_size_t(pos,max_backward_limit);
            let mut skip : usize;
            let mut num_matches
                : usize
                = FindAllMatchesH10(
                      hasher,
                      &(*params).dictionary as (*const BrotliEncoderDictionary),
                      ringbuffer,
                      ringbuffer_mask,
                      pos,
                      num_bytes.wrapping_sub(i),
                      max_distance,
                      gap,
                      params,
                      &mut *matches.offset(
                                lz_matches_offset as (isize)
                            ) as (*mut BackwardMatch)
                  );
            if num_matches > 0i32 as (usize) && (BackwardMatchLength(
                                                     &mut *matches.offset(
                                                               num_matches.wrapping_sub(
                                                                   1i32 as (usize)
                                                               ) as (isize)
                                                           ) as (*mut BackwardMatch) as (*const BackwardMatch)
                                                 ) > max_zopfli_len) {
                *matches.offset(0i32 as (isize)) = *matches.offset(
                                                        num_matches.wrapping_sub(
                                                            1i32 as (usize)
                                                        ) as (isize)
                                                    );
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
                       matches as (*const BackwardMatch),
                       &mut model as (*mut ZopfliCostModel) as (*const ZopfliCostModel),
                       &mut queue as (*mut StartPosQueue),
                       nodes
                   );
            if skip < 16384i32 as (usize) {
                skip = 0i32 as (usize);
            }
            if num_matches == 1i32 as (usize) && (BackwardMatchLength(
                                                      &mut *matches.offset(
                                                                0i32 as (isize)
                                                            ) as (*mut BackwardMatch) as (*const BackwardMatch)
                                                  ) > max_zopfli_len) {
                skip = brotli_max_size_t(
                           BackwardMatchLength(
                               &mut *matches.offset(
                                         0i32 as (isize)
                                     ) as (*mut BackwardMatch) as (*const BackwardMatch)
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
                while skip != 0 {
                    i = i.wrapping_add(1 as (usize));
                    if i.wrapping_add(HashTypeLengthH10()).wrapping_sub(
                           1i32 as (usize)
                       ) >= num_bytes {
                        break;
                    }
                    EvaluateNode(
                        position,
                        i,
                        max_backward_limit,
                        gap,
                        dist_cache,
                        &mut model as (*mut ZopfliCostModel) as (*const ZopfliCostModel),
                        &mut queue as (*mut StartPosQueue),
                        nodes
                    );
                    skip = skip.wrapping_sub(1 as (usize));
                }
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    CleanupZopfliCostModel(m,&mut model as (*mut ZopfliCostModel));
    ComputeShortestPathFromNodes(num_bytes,nodes)
}

#[no_mangle]
pub unsafe extern fn BrotliCreateZopfliBackwardReferences(
    mut m : *mut MemoryManager,
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
    nodes = if num_bytes.wrapping_add(
                   1i32 as (usize)
               ) > 0i32 as (usize) {
                BrotliAllocate(
                    m,
                    num_bytes.wrapping_add(1i32 as (usize)).wrapping_mul(
                        std::mem::size_of::<ZopfliNode>()
                    )
                ) as (*mut ZopfliNode)
            } else {
                0i32 as (*mut std::os::raw::c_void) as (*mut ZopfliNode)
            };
    if !(0i32 == 0) {
        return;
    }
    BrotliInitZopfliNodes(
        nodes,
        num_bytes.wrapping_add(1i32 as (usize))
    );
    *num_commands = (*num_commands).wrapping_add(
                        BrotliZopfliComputeShortestPath(
                            m,
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
        return;
    }
    BrotliZopfliCreateCommands(
        num_bytes,
        position,
        max_backward_limit,
        nodes as (*const ZopfliNode),
        dist_cache,
        last_insert_len,
        params,
        commands,
        num_literals
    );
    {
        BrotliFree(m,nodes as (*mut std::os::raw::c_void));
        nodes = 0i32 as (*mut std::os::raw::c_void) as (*mut ZopfliNode);
    }
}

unsafe extern fn CommandCopyLen(mut self : *const Command) -> u32 {
    (*self).copy_len_ & 0x1ffffffi32 as (u32)
}

unsafe extern fn SetCost(
    mut histogram : *const u32,
    mut histogram_size : usize,
    mut literal_histogram : i32,
    mut cost : *mut f32
) {
    let mut sum : usize = 0i32 as (usize);
    let mut missing_symbol_sum : usize;
    let mut log2sum : f32;
    let mut missing_symbol_cost : f32;
    let mut i : usize;
    i = 0i32 as (usize);
    while i < histogram_size {
        {
            sum = sum.wrapping_add(*histogram.offset(i as (isize)) as (usize));
        }
        i = i.wrapping_add(1 as (usize));
    }
    log2sum = FastLog2(sum) as (f32);
    missing_symbol_sum = sum;
    if literal_histogram == 0 {
        i = 0i32 as (usize);
        while i < histogram_size {
            {
                if *histogram.offset(i as (isize)) == 0i32 as (u32) {
                    missing_symbol_sum = missing_symbol_sum.wrapping_add(1 as (usize));
                }
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    missing_symbol_cost = FastLog2(
                              missing_symbol_sum
                          ) as (f32) + 2i32 as (f32);
    i = 0i32 as (usize);
    while i < histogram_size {
        'continue56: loop {
            {
                if *histogram.offset(i as (isize)) == 0i32 as (u32) {
                    *cost.offset(i as (isize)) = missing_symbol_cost;
                    break 'continue56;
                }
                *cost.offset(i as (isize)) = log2sum - FastLog2(
                                                           *histogram.offset(
                                                                i as (isize)
                                                            ) as (usize)
                                                       ) as (f32);
                if *cost.offset(i as (isize)) < 1i32 as (f32) {
                    *cost.offset(i as (isize)) = 1i32 as (f32);
                }
            }
            break;
        }
        i = i.wrapping_add(1 as (usize));
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
    let mut histogram_literal : *mut u32;
    let mut histogram_cmd : *mut u32;
    let mut histogram_dist : *mut u32;
    let mut cost_literal : *mut f32;
    let mut pos : usize = position.wrapping_sub(last_insert_len);
    let mut min_cost_cmd : f32 = kInfinity;
    let mut i : usize;
    let mut cost_cmd : *mut f32 = (*self).cost_cmd_;
    memset(
        histogram_literal as (*mut std::os::raw::c_void),
        0i32,
        std::mem::size_of::<*mut u32>()
    );
    memset(
        histogram_cmd as (*mut std::os::raw::c_void),
        0i32,
        std::mem::size_of::<*mut u32>()
    );
    memset(
        histogram_dist as (*mut std::os::raw::c_void),
        0i32,
        std::mem::size_of::<*mut u32>()
    );
    i = 0i32 as (usize);
    while i < num_commands {
        {
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
                = ((*commands.offset(
                         i as (isize)
                     )).dist_prefix_ as (i32) & 0x3ffi32) as (usize);
            let mut cmdcode
                : usize
                = (*commands.offset(i as (isize))).cmd_prefix_ as (usize);
            let mut j : usize;
            {
                let _rhs = 1;
                let _lhs = &mut *histogram_cmd.offset(cmdcode as (isize));
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            if cmdcode >= 128i32 as (usize) {
                let _rhs = 1;
                let _lhs = &mut *histogram_dist.offset(distcode as (isize));
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            j = 0i32 as (usize);
            while j < inslength {
                {
                    let _rhs = 1;
                    let _lhs
                        = &mut *histogram_literal.offset(
                                    *ringbuffer.offset(
                                         (pos.wrapping_add(j) & ringbuffer_mask) as (isize)
                                     ) as (isize)
                                );
                    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                }
                j = j.wrapping_add(1 as (usize));
            }
            pos = pos.wrapping_add(inslength.wrapping_add(copylength));
        }
        i = i.wrapping_add(1 as (usize));
    }
    SetCost(
        histogram_literal as (*const u32),
        256i32 as (usize),
        1i32,
        cost_literal
    );
    SetCost(
        histogram_cmd as (*const u32),
        704i32 as (usize),
        0i32,
        cost_cmd
    );
    SetCost(
        histogram_dist as (*const u32),
        (*self).distance_histogram_size as (usize),
        0i32,
        (*self).cost_dist_
    );
    i = 0i32 as (usize);
    while i < 704i32 as (usize) {
        {
            min_cost_cmd = brotli_min_float(
                               min_cost_cmd,
                               *cost_cmd.offset(i as (isize))
                           );
        }
        i = i.wrapping_add(1 as (usize));
    }
    (*self).min_cost_cmd_ = min_cost_cmd;
    {
        let mut literal_costs : *mut f32 = (*self).literal_costs_;
        let mut literal_carry : f32 = 0.0f64 as (f32);
        let mut num_bytes : usize = (*self).num_bytes_;
        *literal_costs.offset(0i32 as (isize)) = 0.0f64 as (f32);
        i = 0i32 as (usize);
        while i < num_bytes {
            {
                literal_carry = literal_carry + *cost_literal.offset(
                                                     *ringbuffer.offset(
                                                          (position.wrapping_add(
                                                               i
                                                           ) & ringbuffer_mask) as (isize)
                                                      ) as (isize)
                                                 );
                *literal_costs.offset(
                     i.wrapping_add(1i32 as (usize)) as (isize)
                 ) = *literal_costs.offset(i as (isize)) + literal_carry;
                literal_carry = literal_carry - (*literal_costs.offset(
                                                      i.wrapping_add(1i32 as (usize)) as (isize)
                                                  ) - *literal_costs.offset(i as (isize)));
            }
            i = i.wrapping_add(1 as (usize));
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
    gap : usize,
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
    while i.wrapping_add(3i32 as (usize)) < num_bytes {
        {
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
                while skip != 0 {
                    i = i.wrapping_add(1 as (usize));
                    if i.wrapping_add(3i32 as (usize)) >= num_bytes {
                        break;
                    }
                    EvaluateNode(
                        position,
                        i,
                        max_backward_limit,
                        gap,
                        dist_cache,
                        model,
                        &mut queue as (*mut StartPosQueue),
                        nodes
                    );
                    cur_match_pos = cur_match_pos.wrapping_add(
                                        *num_matches.offset(i as (isize)) as (usize)
                                    );
                    skip = skip.wrapping_sub(1 as (usize));
                }
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    ComputeShortestPathFromNodes(num_bytes,nodes)
}

#[no_mangle]
pub unsafe extern fn BrotliCreateHqZopfliBackwardReferences(
    mut m : *mut MemoryManager,
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
        = if num_bytes > 0i32 as (usize) {
              BrotliAllocate(
                  m,
                  num_bytes.wrapping_mul(std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut std::os::raw::c_void) as (*mut u32)
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
    let mut orig_dist_cache : *mut i32;
    let mut orig_num_commands : usize;
    let mut model : ZopfliCostModel;
    let mut nodes : *mut ZopfliNode;
    let mut matches
        : *mut BackwardMatch
        = if matches_size > 0i32 as (usize) {
              BrotliAllocate(
                  m,
                  matches_size.wrapping_mul(std::mem::size_of::<BackwardMatch>())
              ) as (*mut BackwardMatch)
          } else {
              0i32 as (*mut std::os::raw::c_void) as (*mut BackwardMatch)
          };
    let mut gap : usize = 0i32 as (usize);
    let mut shadow_matches : usize = 0i32 as (usize);
    if !(0i32 == 0) {
        return;
    }
    i = 0i32 as (usize);
    while i.wrapping_add(HashTypeLengthH10()).wrapping_sub(
              1i32 as (usize)
          ) < num_bytes {
        {
            let pos : usize = position.wrapping_add(i);
            let mut max_distance
                : usize
                = brotli_min_size_t(pos,max_backward_limit);
            let mut max_length : usize = num_bytes.wrapping_sub(i);
            let mut num_found_matches : usize;
            let mut cur_match_end : usize;
            let mut j : usize;
            {
                if matches_size < cur_match_pos.wrapping_add(
                                      128i32 as (usize)
                                  ).wrapping_add(
                                      shadow_matches
                                  ) {
                    let mut _new_size
                        : usize
                        = if matches_size == 0i32 as (usize) {
                              cur_match_pos.wrapping_add(128i32 as (usize)).wrapping_add(
                                  shadow_matches
                              )
                          } else {
                              matches_size
                          };
                    let mut new_array : *mut BackwardMatch;
                    while _new_size < cur_match_pos.wrapping_add(
                                          128i32 as (usize)
                                      ).wrapping_add(
                                          shadow_matches
                                      ) {
                        _new_size = _new_size.wrapping_mul(2i32 as (usize));
                    }
                    new_array = if _new_size > 0i32 as (usize) {
                                    BrotliAllocate(
                                        m,
                                        _new_size.wrapping_mul(std::mem::size_of::<BackwardMatch>())
                                    ) as (*mut BackwardMatch)
                                } else {
                                    0i32 as (*mut std::os::raw::c_void) as (*mut BackwardMatch)
                                };
                    if !!(0i32 == 0) && (matches_size != 0i32 as (usize)) {
                        memcpy(
                            new_array as (*mut std::os::raw::c_void),
                            matches as (*const std::os::raw::c_void),
                            matches_size.wrapping_mul(std::mem::size_of::<BackwardMatch>())
                        );
                    }
                    {
                        BrotliFree(m,matches as (*mut std::os::raw::c_void));
                        matches = 0i32 as (*mut std::os::raw::c_void) as (*mut BackwardMatch);
                    }
                    matches = new_array;
                    matches_size = _new_size;
                }
            }
            if !(0i32 == 0) {
                return;
            }
            num_found_matches = FindAllMatchesH10(
                                    hasher,
                                    &(*params).dictionary as (*const BrotliEncoderDictionary),
                                    ringbuffer,
                                    ringbuffer_mask,
                                    pos,
                                    max_length,
                                    max_distance,
                                    gap,
                                    params,
                                    &mut *matches.offset(
                                              cur_match_pos.wrapping_add(shadow_matches) as (isize)
                                          ) as (*mut BackwardMatch)
                                );
            cur_match_end = cur_match_pos.wrapping_add(num_found_matches);
            j = cur_match_pos;
            while j.wrapping_add(1i32 as (usize)) < cur_match_end {
                { }
                j = j.wrapping_add(1 as (usize));
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
                              ) as (*mut u32) as (*mut std::os::raw::c_void),
                        0i32,
                        skip.wrapping_mul(std::mem::size_of::<u32>())
                    );
                    i = i.wrapping_add(skip);
                } else {
                    cur_match_pos = cur_match_end;
                }
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    orig_num_literals = *num_literals;
    orig_last_insert_len = *last_insert_len;
    memcpy(
        orig_dist_cache as (*mut std::os::raw::c_void),
        dist_cache as (*const std::os::raw::c_void),
        (4i32 as (usize)).wrapping_mul(std::mem::size_of::<i32>())
    );
    orig_num_commands = *num_commands;
    nodes = if num_bytes.wrapping_add(
                   1i32 as (usize)
               ) > 0i32 as (usize) {
                BrotliAllocate(
                    m,
                    num_bytes.wrapping_add(1i32 as (usize)).wrapping_mul(
                        std::mem::size_of::<ZopfliNode>()
                    )
                ) as (*mut ZopfliNode)
            } else {
                0i32 as (*mut std::os::raw::c_void) as (*mut ZopfliNode)
            };
    if !(0i32 == 0) {
        return;
    }
    InitZopfliCostModel(
        m,
        &mut model as (*mut ZopfliCostModel),
        &(*params).dist as (*const BrotliDistanceParams),
        num_bytes
    );
    if !(0i32 == 0) {
        return;
    }
    i = 0i32 as (usize);
    while i < 2i32 as (usize) {
        {
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
                dist_cache as (*mut std::os::raw::c_void),
                orig_dist_cache as (*const std::os::raw::c_void),
                (4i32 as (usize)).wrapping_mul(std::mem::size_of::<i32>())
            );
            *num_commands = (*num_commands).wrapping_add(
                                ZopfliIterate(
                                    num_bytes,
                                    position,
                                    ringbuffer,
                                    ringbuffer_mask,
                                    params,
                                    max_backward_limit,
                                    gap,
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
                params,
                commands,
                num_literals
            );
        }
        i = i.wrapping_add(1 as (usize));
    }
    CleanupZopfliCostModel(m,&mut model as (*mut ZopfliCostModel));
    {
        BrotliFree(m,nodes as (*mut std::os::raw::c_void));
        nodes = 0i32 as (*mut std::os::raw::c_void) as (*mut ZopfliNode);
    }
    {
        BrotliFree(m,matches as (*mut std::os::raw::c_void));
        matches = 0i32 as (*mut std::os::raw::c_void) as (*mut BackwardMatch);
    }
    {
        BrotliFree(m,num_matches as (*mut std::os::raw::c_void));
        num_matches = 0i32 as (*mut std::os::raw::c_void) as (*mut u32);
    }
}
