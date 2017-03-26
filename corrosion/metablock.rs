extern {
    fn BrotliAllocate(
        m : *mut MemoryManager, n : usize
    ) -> *mut std::os::raw::c_void;
    fn BrotliBuildHistogramsWithContext(
        cmds : *const Command,
        num_commands : usize,
        literal_split : *const BlockSplit,
        insert_and_copy_split : *const BlockSplit,
        dist_split : *const BlockSplit,
        ringbuffer : *const u8,
        pos : usize,
        mask : usize,
        prev_byte : u8,
        prev_byte2 : u8,
        context_modes : *const ContextType,
        literal_histograms : *mut HistogramLiteral,
        insert_and_copy_histograms : *mut HistogramCommand,
        copy_dist_histograms : *mut HistogramDistance
    );
    fn BrotliClusterHistogramsDistance(
        m : *mut MemoryManager,
        in : *const HistogramDistance,
        in_size : usize,
        max_histograms : usize,
        out : *mut HistogramDistance,
        out_size : *mut usize,
        histogram_symbols : *mut u32
    );
    fn BrotliClusterHistogramsLiteral(
        m : *mut MemoryManager,
        in : *const HistogramLiteral,
        in_size : usize,
        max_histograms : usize,
        out : *mut HistogramLiteral,
        out_size : *mut usize,
        histogram_symbols : *mut u32
    );
    fn BrotliFree(
        m : *mut MemoryManager, p : *mut std::os::raw::c_void
    );
    fn BrotliOptimizeHuffmanCountsForRle(
        length : usize, counts : *mut u32, good_for_rle : *mut u8
    );
    fn BrotliSplitBlock(
        m : *mut MemoryManager,
        cmds : *const Command,
        num_commands : usize,
        data : *const u8,
        offset : usize,
        mask : usize,
        params : *const BrotliEncoderParams,
        literal_split : *mut BlockSplit,
        insert_and_copy_split : *mut BlockSplit,
        dist_split : *mut BlockSplit
    );
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
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
pub struct MemoryManager {
    pub alloc_func : unsafe extern fn(*mut std::os::raw::c_void, usize) -> *mut std::os::raw::c_void,
    pub free_func : unsafe extern fn(*mut std::os::raw::c_void, *mut std::os::raw::c_void),
    pub opaque : *mut std::os::raw::c_void,
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

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Command {
    pub insert_len_ : u32,
    pub copy_len_ : u32,
    pub dist_extra_ : u32,
    pub cmd_prefix_ : u16,
    pub dist_prefix_ : u16,
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum ContextType {
    CONTEXT_LSB6 = 0i32,
    CONTEXT_MSB6 = 1i32,
    CONTEXT_UTF8 = 2i32,
    CONTEXT_SIGNED = 3i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlockSplit {
    pub num_types : usize,
    pub num_blocks : usize,
    pub types : *mut u8,
    pub lengths : *mut u32,
    pub types_alloc_size : usize,
    pub lengths_alloc_size : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramLiteral {
    pub data_ : *mut u32,
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramCommand {
    pub data_ : *mut u32,
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramDistance {
    pub data_ : *mut u32,
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MetaBlockSplit {
    pub literal_split : BlockSplit,
    pub command_split : BlockSplit,
    pub distance_split : BlockSplit,
    pub literal_context_map : *mut u32,
    pub literal_context_map_size : usize,
    pub distance_context_map : *mut u32,
    pub distance_context_map_size : usize,
    pub literal_histograms : *mut HistogramLiteral,
    pub literal_histograms_size : usize,
    pub command_histograms : *mut HistogramCommand,
    pub command_histograms_size : usize,
    pub distance_histograms : *mut HistogramDistance,
    pub distance_histograms_size : usize,
}

unsafe extern fn HistogramClearLiteral(
    mut self : *mut HistogramLiteral
) {
    memset(
        (*self).data_ as (*mut std::os::raw::c_void),
        0i32,
        std::mem::size_of::<*mut u32>()
    );
    (*self).total_count_ = 0i32 as (usize);
    (*self).bit_cost_ = 3.402e+38f64;
}

unsafe extern fn ClearHistogramsLiteral(
    mut array : *mut HistogramLiteral, mut length : usize
) {
    let mut i : usize;
    i = 0i32 as (usize);
    while i < length {
        HistogramClearLiteral(array.offset(i as (isize)));
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn HistogramClearDistance(
    mut self : *mut HistogramDistance
) {
    memset(
        (*self).data_ as (*mut std::os::raw::c_void),
        0i32,
        std::mem::size_of::<*mut u32>()
    );
    (*self).total_count_ = 0i32 as (usize);
    (*self).bit_cost_ = 3.402e+38f64;
}

unsafe extern fn ClearHistogramsDistance(
    mut array : *mut HistogramDistance, mut length : usize
) {
    let mut i : usize;
    i = 0i32 as (usize);
    while i < length {
        HistogramClearDistance(array.offset(i as (isize)));
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn HistogramClearCommand(
    mut self : *mut HistogramCommand
) {
    memset(
        (*self).data_ as (*mut std::os::raw::c_void),
        0i32,
        std::mem::size_of::<*mut u32>()
    );
    (*self).total_count_ = 0i32 as (usize);
    (*self).bit_cost_ = 3.402e+38f64;
}

unsafe extern fn ClearHistogramsCommand(
    mut array : *mut HistogramCommand, mut length : usize
) {
    let mut i : usize;
    i = 0i32 as (usize);
    while i < length {
        HistogramClearCommand(array.offset(i as (isize)));
        i = i.wrapping_add(1 as (usize));
    }
}

#[no_mangle]
pub unsafe extern fn BrotliBuildMetaBlock(
    mut m : *mut MemoryManager,
    mut ringbuffer : *const u8,
    pos : usize,
    mask : usize,
    mut params : *const BrotliEncoderParams,
    mut prev_byte : u8,
    mut prev_byte2 : u8,
    mut cmds : *const Command,
    mut num_commands : usize,
    mut literal_context_mode : ContextType,
    mut mb : *mut MetaBlockSplit
) {
    static kMaxNumberOfHistograms : usize = 256i32 as (usize);
    let mut distance_histograms : *mut HistogramDistance;
    let mut literal_histograms : *mut HistogramLiteral;
    let mut literal_context_modes
        : *mut ContextType
        = 0i32 as (*mut std::os::raw::c_void) as (*mut ContextType);
    let mut literal_histograms_size : usize;
    let mut distance_histograms_size : usize;
    let mut i : usize;
    let mut literal_context_multiplier : usize = 1i32 as (usize);
    BrotliSplitBlock(
        m,
        cmds,
        num_commands,
        ringbuffer,
        pos,
        mask,
        params,
        &mut (*mb).literal_split as (*mut BlockSplit),
        &mut (*mb).command_split as (*mut BlockSplit),
        &mut (*mb).distance_split as (*mut BlockSplit)
    );
    if !(0i32 == 0) {
        return;
    }
    if (*params).disable_literal_context_modeling == 0 {
        literal_context_multiplier = (1i32 << 6i32) as (usize);
        literal_context_modes = if (*mb).literal_split.num_types != 0 {
                                    BrotliAllocate(
                                        m,
                                        (*mb).literal_split.num_types.wrapping_mul(
                                            std::mem::size_of::<ContextType>()
                                        )
                                    ) as (*mut ContextType)
                                } else {
                                    0i32 as (*mut std::os::raw::c_void) as (*mut ContextType)
                                };
        if !(0i32 == 0) {
            return;
        }
        i = 0i32 as (usize);
        while i < (*mb).literal_split.num_types {
            {
                *literal_context_modes.offset(i as (isize)) = literal_context_mode;
            }
            i = i.wrapping_add(1 as (usize));
        }
    }
    literal_histograms_size = (*mb).literal_split.num_types.wrapping_mul(
                                  literal_context_multiplier
                              );
    literal_histograms = if literal_histograms_size != 0 {
                             BrotliAllocate(
                                 m,
                                 literal_histograms_size.wrapping_mul(
                                     std::mem::size_of::<HistogramLiteral>()
                                 )
                             ) as (*mut HistogramLiteral)
                         } else {
                             0i32 as (*mut std::os::raw::c_void) as (*mut HistogramLiteral)
                         };
    if !(0i32 == 0) {
        return;
    }
    ClearHistogramsLiteral(literal_histograms,literal_histograms_size);
    distance_histograms_size = (*mb).distance_split.num_types << 2i32;
    distance_histograms = if distance_histograms_size != 0 {
                              BrotliAllocate(
                                  m,
                                  distance_histograms_size.wrapping_mul(
                                      std::mem::size_of::<HistogramDistance>()
                                  )
                              ) as (*mut HistogramDistance)
                          } else {
                              0i32 as (*mut std::os::raw::c_void) as (*mut HistogramDistance)
                          };
    if !(0i32 == 0) {
        return;
    }
    ClearHistogramsDistance(
        distance_histograms,
        distance_histograms_size
    );
    if (*mb).command_histograms == 0i32 as (*mut HistogramCommand) {
        0i32;
    } else {
        __assert_fail(
            b"mb->command_histograms == 0\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"BrotliBuildMetaBlock\0".as_ptr()
        );
    }
    (*mb).command_histograms_size = (*mb).command_split.num_types;
    (*mb).command_histograms = if (*mb).command_histograms_size != 0 {
                                   BrotliAllocate(
                                       m,
                                       (*mb).command_histograms_size.wrapping_mul(
                                           std::mem::size_of::<HistogramCommand>()
                                       )
                                   ) as (*mut HistogramCommand)
                               } else {
                                   0i32 as (*mut std::os::raw::c_void) as (*mut HistogramCommand)
                               };
    if !(0i32 == 0) {
        return;
    }
    ClearHistogramsCommand(
        (*mb).command_histograms,
        (*mb).command_histograms_size
    );
    BrotliBuildHistogramsWithContext(
        cmds,
        num_commands,
        &mut (*mb).literal_split as (*mut BlockSplit) as (*const BlockSplit),
        &mut (*mb).command_split as (*mut BlockSplit) as (*const BlockSplit),
        &mut (*mb).distance_split as (*mut BlockSplit) as (*const BlockSplit),
        ringbuffer,
        pos,
        mask,
        prev_byte,
        prev_byte2,
        literal_context_modes as (*const ContextType),
        literal_histograms,
        (*mb).command_histograms,
        distance_histograms
    );
    {
        BrotliFree(m,literal_context_modes as (*mut std::os::raw::c_void));
        literal_context_modes = 0i32 as (*mut std::os::raw::c_void) as (*mut ContextType);
    }
    if (*mb).literal_context_map == 0i32 as (*mut u32) {
        0i32;
    } else {
        __assert_fail(
            b"mb->literal_context_map == 0\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"BrotliBuildMetaBlock\0".as_ptr()
        );
    }
    (*mb).literal_context_map_size = (*mb).literal_split.num_types << 6i32;
    (*mb).literal_context_map = if (*mb).literal_context_map_size != 0 {
                                    BrotliAllocate(
                                        m,
                                        (*mb).literal_context_map_size.wrapping_mul(
                                            std::mem::size_of::<u32>()
                                        )
                                    ) as (*mut u32)
                                } else {
                                    0i32 as (*mut std::os::raw::c_void) as (*mut u32)
                                };
    if !(0i32 == 0) {
        return;
    }
    if (*mb).literal_histograms == 0i32 as (*mut HistogramLiteral) {
        0i32;
    } else {
        __assert_fail(
            b"mb->literal_histograms == 0\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"BrotliBuildMetaBlock\0".as_ptr()
        );
    }
    (*mb).literal_histograms_size = (*mb).literal_context_map_size;
    (*mb).literal_histograms = if (*mb).literal_histograms_size != 0 {
                                   BrotliAllocate(
                                       m,
                                       (*mb).literal_histograms_size.wrapping_mul(
                                           std::mem::size_of::<HistogramLiteral>()
                                       )
                                   ) as (*mut HistogramLiteral)
                               } else {
                                   0i32 as (*mut std::os::raw::c_void) as (*mut HistogramLiteral)
                               };
    if !(0i32 == 0) {
        return;
    }
    BrotliClusterHistogramsLiteral(
        m,
        literal_histograms as (*const HistogramLiteral),
        literal_histograms_size,
        kMaxNumberOfHistograms,
        (*mb).literal_histograms,
        &mut (*mb).literal_histograms_size as (*mut usize),
        (*mb).literal_context_map
    );
    if !(0i32 == 0) {
        return;
    }
    {
        BrotliFree(m,literal_histograms as (*mut std::os::raw::c_void));
        literal_histograms = 0i32 as (*mut std::os::raw::c_void) as (*mut HistogramLiteral);
    }
    if (*params).disable_literal_context_modeling != 0 {
        i = (*mb).literal_split.num_types;
        while i != 0i32 as (usize) {
            let mut j : usize = 0i32 as (usize);
            i = i.wrapping_sub(1 as (usize));
            while j < (1i32 << 6i32) as (usize) {
                {
                    *(*mb).literal_context_map.offset(
                         (i << 6i32).wrapping_add(j) as (isize)
                     ) = *(*mb).literal_context_map.offset(i as (isize));
                }
                j = j.wrapping_add(1 as (usize));
            }
        }
    }
    if (*mb).distance_context_map == 0i32 as (*mut u32) {
        0i32;
    } else {
        __assert_fail(
            b"mb->distance_context_map == 0\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"BrotliBuildMetaBlock\0".as_ptr()
        );
    }
    (*mb).distance_context_map_size = (*mb).distance_split.num_types << 2i32;
    (*mb).distance_context_map = if (*mb).distance_context_map_size != 0 {
                                     BrotliAllocate(
                                         m,
                                         (*mb).distance_context_map_size.wrapping_mul(
                                             std::mem::size_of::<u32>()
                                         )
                                     ) as (*mut u32)
                                 } else {
                                     0i32 as (*mut std::os::raw::c_void) as (*mut u32)
                                 };
    if !(0i32 == 0) {
        return;
    }
    if (*mb).distance_histograms == 0i32 as (*mut HistogramDistance) {
        0i32;
    } else {
        __assert_fail(
            b"mb->distance_histograms == 0\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"BrotliBuildMetaBlock\0".as_ptr()
        );
    }
    (*mb).distance_histograms_size = (*mb).distance_context_map_size;
    (*mb).distance_histograms = if (*mb).distance_histograms_size != 0 {
                                    BrotliAllocate(
                                        m,
                                        (*mb).distance_histograms_size.wrapping_mul(
                                            std::mem::size_of::<HistogramDistance>()
                                        )
                                    ) as (*mut HistogramDistance)
                                } else {
                                    0i32 as (*mut std::os::raw::c_void) as (*mut HistogramDistance)
                                };
    if !(0i32 == 0) {
        return;
    }
    BrotliClusterHistogramsDistance(
        m,
        distance_histograms as (*const HistogramDistance),
        (*mb).distance_context_map_size,
        kMaxNumberOfHistograms,
        (*mb).distance_histograms,
        &mut (*mb).distance_histograms_size as (*mut usize),
        (*mb).distance_context_map
    );
    if !(0i32 == 0) {
        return;
    }
    {
        BrotliFree(m,distance_histograms as (*mut std::os::raw::c_void));
        distance_histograms = 0i32 as (*mut std::os::raw::c_void) as (*mut HistogramDistance);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlockSplitterLiteral {
    pub alphabet_size_ : usize,
    pub min_block_size_ : usize,
    pub split_threshold_ : f64,
    pub num_blocks_ : usize,
    pub split_ : *mut BlockSplit,
    pub histograms_ : *mut HistogramLiteral,
    pub histograms_size_ : *mut usize,
    pub target_block_size_ : usize,
    pub block_size_ : usize,
    pub curr_histogram_ix_ : usize,
    pub last_histogram_ix_ : *mut usize,
    pub last_entropy_ : *mut f64,
    pub merge_last_count_ : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ContextBlockSplitter {
    pub alphabet_size_ : usize,
    pub num_contexts_ : usize,
    pub max_block_types_ : usize,
    pub min_block_size_ : usize,
    pub split_threshold_ : f64,
    pub num_blocks_ : usize,
    pub split_ : *mut BlockSplit,
    pub histograms_ : *mut HistogramLiteral,
    pub histograms_size_ : *mut usize,
    pub target_block_size_ : usize,
    pub block_size_ : usize,
    pub curr_histogram_ix_ : usize,
    pub last_histogram_ix_ : *mut usize,
    pub last_entropy_ : *mut f64,
    pub merge_last_count_ : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LitBlocks {
    pub plain : BlockSplitterLiteral,
    pub ctx : ContextBlockSplitter,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlockSplitterCommand {
    pub alphabet_size_ : usize,
    pub min_block_size_ : usize,
    pub split_threshold_ : f64,
    pub num_blocks_ : usize,
    pub split_ : *mut BlockSplit,
    pub histograms_ : *mut HistogramCommand,
    pub histograms_size_ : *mut usize,
    pub target_block_size_ : usize,
    pub block_size_ : usize,
    pub curr_histogram_ix_ : usize,
    pub last_histogram_ix_ : *mut usize,
    pub last_entropy_ : *mut f64,
    pub merge_last_count_ : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlockSplitterDistance {
    pub alphabet_size_ : usize,
    pub min_block_size_ : usize,
    pub split_threshold_ : f64,
    pub num_blocks_ : usize,
    pub split_ : *mut BlockSplit,
    pub histograms_ : *mut HistogramDistance,
    pub histograms_size_ : *mut usize,
    pub target_block_size_ : usize,
    pub block_size_ : usize,
    pub curr_histogram_ix_ : usize,
    pub last_histogram_ix_ : *mut usize,
    pub last_entropy_ : *mut f64,
    pub merge_last_count_ : usize,
}

unsafe extern fn brotli_min_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a < b { a } else { b }
}

unsafe extern fn InitBlockSplitterLiteral(
    mut m : *mut MemoryManager,
    mut self : *mut BlockSplitterLiteral,
    mut alphabet_size : usize,
    mut min_block_size : usize,
    mut split_threshold : f64,
    mut num_symbols : usize,
    mut split : *mut BlockSplit,
    mut histograms : *mut *mut HistogramLiteral,
    mut histograms_size : *mut usize
) {
    let mut max_num_blocks
        : usize
        = num_symbols.wrapping_div(min_block_size).wrapping_add(
              1i32 as (usize)
          );
    let mut max_num_types
        : usize
        = brotli_min_size_t(max_num_blocks,(256i32 + 1i32) as (usize));
    (*self).alphabet_size_ = alphabet_size;
    (*self).min_block_size_ = min_block_size;
    (*self).split_threshold_ = split_threshold;
    (*self).num_blocks_ = 0i32 as (usize);
    (*self).split_ = split;
    (*self).histograms_size_ = histograms_size;
    (*self).target_block_size_ = min_block_size;
    (*self).block_size_ = 0i32 as (usize);
    (*self).curr_histogram_ix_ = 0i32 as (usize);
    (*self).merge_last_count_ = 0i32 as (usize);
    {
        if (*split).types_alloc_size < max_num_blocks {
            let mut _new_size
                : usize
                = if (*split).types_alloc_size == 0i32 as (usize) {
                      max_num_blocks
                  } else {
                      (*split).types_alloc_size
                  };
            let mut new_array : *mut u8;
            while _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(std::mem::size_of::<u8>())
                            ) as (*mut u8)
                        } else {
                            0i32 as (*mut std::os::raw::c_void) as (*mut u8)
                        };
            if !!(0i32 == 0) && ((*split).types_alloc_size != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut std::os::raw::c_void),
                    (*split).types as (*const std::os::raw::c_void),
                    (*split).types_alloc_size.wrapping_mul(std::mem::size_of::<u8>())
                );
            }
            {
                BrotliFree(m,(*split).types as (*mut std::os::raw::c_void));
                (*split).types = 0i32 as (*mut std::os::raw::c_void) as (*mut u8);
            }
            (*split).types = new_array;
            (*split).types_alloc_size = _new_size;
        }
    }
    {
        if (*split).lengths_alloc_size < max_num_blocks {
            let mut _new_size
                : usize
                = if (*split).lengths_alloc_size == 0i32 as (usize) {
                      max_num_blocks
                  } else {
                      (*split).lengths_alloc_size
                  };
            let mut new_array : *mut u32;
            while _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(std::mem::size_of::<u32>())
                            ) as (*mut u32)
                        } else {
                            0i32 as (*mut std::os::raw::c_void) as (*mut u32)
                        };
            if !!(0i32 == 0) && ((*split).lengths_alloc_size != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut std::os::raw::c_void),
                    (*split).lengths as (*const std::os::raw::c_void),
                    (*split).lengths_alloc_size.wrapping_mul(
                        std::mem::size_of::<u32>()
                    )
                );
            }
            {
                BrotliFree(m,(*split).lengths as (*mut std::os::raw::c_void));
                (*split).lengths = 0i32 as (*mut std::os::raw::c_void) as (*mut u32);
            }
            (*split).lengths = new_array;
            (*split).lengths_alloc_size = _new_size;
        }
    }
    if !(0i32 == 0) {
        return;
    }
    (*(*self).split_).num_blocks = max_num_blocks;
    if *histograms == 0i32 as (*mut HistogramLiteral) {
        0i32;
    } else {
        __assert_fail(
            b"*histograms == 0\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"InitBlockSplitterLiteral\0".as_ptr()
        );
    }
    *histograms_size = max_num_types;
    *histograms = if *histograms_size != 0 {
                      BrotliAllocate(
                          m,
                          (*histograms_size).wrapping_mul(
                              std::mem::size_of::<HistogramLiteral>()
                          )
                      ) as (*mut HistogramLiteral)
                  } else {
                      0i32 as (*mut std::os::raw::c_void) as (*mut HistogramLiteral)
                  };
    (*self).histograms_ = *histograms;
    if !(0i32 == 0) {
        return;
    }
    HistogramClearLiteral(
        &mut *(*self).histograms_.offset(
                  0i32 as (isize)
              ) as (*mut HistogramLiteral)
    );
    *(*self).last_histogram_ix_.offset(0i32 as (isize)) = {
                                                              let _rhs = 0i32;
                                                              let _lhs
                                                                  = &mut *(*self).last_histogram_ix_.offset(
                                                                              1i32 as (isize)
                                                                          );
                                                              *_lhs = _rhs as (usize);
                                                              *_lhs
                                                          };
}

unsafe extern fn InitContextBlockSplitter(
    mut m : *mut MemoryManager,
    mut self : *mut ContextBlockSplitter,
    mut alphabet_size : usize,
    mut num_contexts : usize,
    mut min_block_size : usize,
    mut split_threshold : f64,
    mut num_symbols : usize,
    mut split : *mut BlockSplit,
    mut histograms : *mut *mut HistogramLiteral,
    mut histograms_size : *mut usize
) {
    let mut max_num_blocks
        : usize
        = num_symbols.wrapping_div(min_block_size).wrapping_add(
              1i32 as (usize)
          );
    let mut max_num_types : usize;
    if num_contexts <= 3i32 as (usize) {
        0i32;
    } else {
        __assert_fail(
            b"num_contexts <= BROTLI_MAX_STATIC_CONTEXTS\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"InitContextBlockSplitter\0".as_ptr()
        );
    }
    (*self).alphabet_size_ = alphabet_size;
    (*self).num_contexts_ = num_contexts;
    (*self).max_block_types_ = (256i32 as (usize)).wrapping_div(
                                   num_contexts
                               );
    (*self).min_block_size_ = min_block_size;
    (*self).split_threshold_ = split_threshold;
    (*self).num_blocks_ = 0i32 as (usize);
    (*self).split_ = split;
    (*self).histograms_size_ = histograms_size;
    (*self).target_block_size_ = min_block_size;
    (*self).block_size_ = 0i32 as (usize);
    (*self).curr_histogram_ix_ = 0i32 as (usize);
    (*self).merge_last_count_ = 0i32 as (usize);
    max_num_types = brotli_min_size_t(
                        max_num_blocks,
                        (*self).max_block_types_.wrapping_add(1i32 as (usize))
                    );
    {
        if (*split).types_alloc_size < max_num_blocks {
            let mut _new_size
                : usize
                = if (*split).types_alloc_size == 0i32 as (usize) {
                      max_num_blocks
                  } else {
                      (*split).types_alloc_size
                  };
            let mut new_array : *mut u8;
            while _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(std::mem::size_of::<u8>())
                            ) as (*mut u8)
                        } else {
                            0i32 as (*mut std::os::raw::c_void) as (*mut u8)
                        };
            if !!(0i32 == 0) && ((*split).types_alloc_size != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut std::os::raw::c_void),
                    (*split).types as (*const std::os::raw::c_void),
                    (*split).types_alloc_size.wrapping_mul(std::mem::size_of::<u8>())
                );
            }
            {
                BrotliFree(m,(*split).types as (*mut std::os::raw::c_void));
                (*split).types = 0i32 as (*mut std::os::raw::c_void) as (*mut u8);
            }
            (*split).types = new_array;
            (*split).types_alloc_size = _new_size;
        }
    }
    {
        if (*split).lengths_alloc_size < max_num_blocks {
            let mut _new_size
                : usize
                = if (*split).lengths_alloc_size == 0i32 as (usize) {
                      max_num_blocks
                  } else {
                      (*split).lengths_alloc_size
                  };
            let mut new_array : *mut u32;
            while _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(std::mem::size_of::<u32>())
                            ) as (*mut u32)
                        } else {
                            0i32 as (*mut std::os::raw::c_void) as (*mut u32)
                        };
            if !!(0i32 == 0) && ((*split).lengths_alloc_size != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut std::os::raw::c_void),
                    (*split).lengths as (*const std::os::raw::c_void),
                    (*split).lengths_alloc_size.wrapping_mul(
                        std::mem::size_of::<u32>()
                    )
                );
            }
            {
                BrotliFree(m,(*split).lengths as (*mut std::os::raw::c_void));
                (*split).lengths = 0i32 as (*mut std::os::raw::c_void) as (*mut u32);
            }
            (*split).lengths = new_array;
            (*split).lengths_alloc_size = _new_size;
        }
    }
    if !(0i32 == 0) {
        return;
    }
    (*split).num_blocks = max_num_blocks;
    if !(0i32 == 0) {
        return;
    }
    if *histograms == 0i32 as (*mut HistogramLiteral) {
        0i32;
    } else {
        __assert_fail(
            b"*histograms == 0\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"InitContextBlockSplitter\0".as_ptr()
        );
    }
    *histograms_size = max_num_types.wrapping_mul(num_contexts);
    *histograms = if *histograms_size != 0 {
                      BrotliAllocate(
                          m,
                          (*histograms_size).wrapping_mul(
                              std::mem::size_of::<HistogramLiteral>()
                          )
                      ) as (*mut HistogramLiteral)
                  } else {
                      0i32 as (*mut std::os::raw::c_void) as (*mut HistogramLiteral)
                  };
    (*self).histograms_ = *histograms;
    if !(0i32 == 0) {
        return;
    }
    ClearHistogramsLiteral(
        &mut *(*self).histograms_.offset(
                  0i32 as (isize)
              ) as (*mut HistogramLiteral),
        num_contexts
    );
    *(*self).last_histogram_ix_.offset(0i32 as (isize)) = {
                                                              let _rhs = 0i32;
                                                              let _lhs
                                                                  = &mut *(*self).last_histogram_ix_.offset(
                                                                              1i32 as (isize)
                                                                          );
                                                              *_lhs = _rhs as (usize);
                                                              *_lhs
                                                          };
}

unsafe extern fn InitBlockSplitterCommand(
    mut m : *mut MemoryManager,
    mut self : *mut BlockSplitterCommand,
    mut alphabet_size : usize,
    mut min_block_size : usize,
    mut split_threshold : f64,
    mut num_symbols : usize,
    mut split : *mut BlockSplit,
    mut histograms : *mut *mut HistogramCommand,
    mut histograms_size : *mut usize
) {
    let mut max_num_blocks
        : usize
        = num_symbols.wrapping_div(min_block_size).wrapping_add(
              1i32 as (usize)
          );
    let mut max_num_types
        : usize
        = brotli_min_size_t(max_num_blocks,(256i32 + 1i32) as (usize));
    (*self).alphabet_size_ = alphabet_size;
    (*self).min_block_size_ = min_block_size;
    (*self).split_threshold_ = split_threshold;
    (*self).num_blocks_ = 0i32 as (usize);
    (*self).split_ = split;
    (*self).histograms_size_ = histograms_size;
    (*self).target_block_size_ = min_block_size;
    (*self).block_size_ = 0i32 as (usize);
    (*self).curr_histogram_ix_ = 0i32 as (usize);
    (*self).merge_last_count_ = 0i32 as (usize);
    {
        if (*split).types_alloc_size < max_num_blocks {
            let mut _new_size
                : usize
                = if (*split).types_alloc_size == 0i32 as (usize) {
                      max_num_blocks
                  } else {
                      (*split).types_alloc_size
                  };
            let mut new_array : *mut u8;
            while _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(std::mem::size_of::<u8>())
                            ) as (*mut u8)
                        } else {
                            0i32 as (*mut std::os::raw::c_void) as (*mut u8)
                        };
            if !!(0i32 == 0) && ((*split).types_alloc_size != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut std::os::raw::c_void),
                    (*split).types as (*const std::os::raw::c_void),
                    (*split).types_alloc_size.wrapping_mul(std::mem::size_of::<u8>())
                );
            }
            {
                BrotliFree(m,(*split).types as (*mut std::os::raw::c_void));
                (*split).types = 0i32 as (*mut std::os::raw::c_void) as (*mut u8);
            }
            (*split).types = new_array;
            (*split).types_alloc_size = _new_size;
        }
    }
    {
        if (*split).lengths_alloc_size < max_num_blocks {
            let mut _new_size
                : usize
                = if (*split).lengths_alloc_size == 0i32 as (usize) {
                      max_num_blocks
                  } else {
                      (*split).lengths_alloc_size
                  };
            let mut new_array : *mut u32;
            while _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(std::mem::size_of::<u32>())
                            ) as (*mut u32)
                        } else {
                            0i32 as (*mut std::os::raw::c_void) as (*mut u32)
                        };
            if !!(0i32 == 0) && ((*split).lengths_alloc_size != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut std::os::raw::c_void),
                    (*split).lengths as (*const std::os::raw::c_void),
                    (*split).lengths_alloc_size.wrapping_mul(
                        std::mem::size_of::<u32>()
                    )
                );
            }
            {
                BrotliFree(m,(*split).lengths as (*mut std::os::raw::c_void));
                (*split).lengths = 0i32 as (*mut std::os::raw::c_void) as (*mut u32);
            }
            (*split).lengths = new_array;
            (*split).lengths_alloc_size = _new_size;
        }
    }
    if !(0i32 == 0) {
        return;
    }
    (*(*self).split_).num_blocks = max_num_blocks;
    if *histograms == 0i32 as (*mut HistogramCommand) {
        0i32;
    } else {
        __assert_fail(
            b"*histograms == 0\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"InitBlockSplitterCommand\0".as_ptr()
        );
    }
    *histograms_size = max_num_types;
    *histograms = if *histograms_size != 0 {
                      BrotliAllocate(
                          m,
                          (*histograms_size).wrapping_mul(
                              std::mem::size_of::<HistogramCommand>()
                          )
                      ) as (*mut HistogramCommand)
                  } else {
                      0i32 as (*mut std::os::raw::c_void) as (*mut HistogramCommand)
                  };
    (*self).histograms_ = *histograms;
    if !(0i32 == 0) {
        return;
    }
    HistogramClearCommand(
        &mut *(*self).histograms_.offset(
                  0i32 as (isize)
              ) as (*mut HistogramCommand)
    );
    *(*self).last_histogram_ix_.offset(0i32 as (isize)) = {
                                                              let _rhs = 0i32;
                                                              let _lhs
                                                                  = &mut *(*self).last_histogram_ix_.offset(
                                                                              1i32 as (isize)
                                                                          );
                                                              *_lhs = _rhs as (usize);
                                                              *_lhs
                                                          };
}

unsafe extern fn InitBlockSplitterDistance(
    mut m : *mut MemoryManager,
    mut self : *mut BlockSplitterDistance,
    mut alphabet_size : usize,
    mut min_block_size : usize,
    mut split_threshold : f64,
    mut num_symbols : usize,
    mut split : *mut BlockSplit,
    mut histograms : *mut *mut HistogramDistance,
    mut histograms_size : *mut usize
) {
    let mut max_num_blocks
        : usize
        = num_symbols.wrapping_div(min_block_size).wrapping_add(
              1i32 as (usize)
          );
    let mut max_num_types
        : usize
        = brotli_min_size_t(max_num_blocks,(256i32 + 1i32) as (usize));
    (*self).alphabet_size_ = alphabet_size;
    (*self).min_block_size_ = min_block_size;
    (*self).split_threshold_ = split_threshold;
    (*self).num_blocks_ = 0i32 as (usize);
    (*self).split_ = split;
    (*self).histograms_size_ = histograms_size;
    (*self).target_block_size_ = min_block_size;
    (*self).block_size_ = 0i32 as (usize);
    (*self).curr_histogram_ix_ = 0i32 as (usize);
    (*self).merge_last_count_ = 0i32 as (usize);
    {
        if (*split).types_alloc_size < max_num_blocks {
            let mut _new_size
                : usize
                = if (*split).types_alloc_size == 0i32 as (usize) {
                      max_num_blocks
                  } else {
                      (*split).types_alloc_size
                  };
            let mut new_array : *mut u8;
            while _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(std::mem::size_of::<u8>())
                            ) as (*mut u8)
                        } else {
                            0i32 as (*mut std::os::raw::c_void) as (*mut u8)
                        };
            if !!(0i32 == 0) && ((*split).types_alloc_size != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut std::os::raw::c_void),
                    (*split).types as (*const std::os::raw::c_void),
                    (*split).types_alloc_size.wrapping_mul(std::mem::size_of::<u8>())
                );
            }
            {
                BrotliFree(m,(*split).types as (*mut std::os::raw::c_void));
                (*split).types = 0i32 as (*mut std::os::raw::c_void) as (*mut u8);
            }
            (*split).types = new_array;
            (*split).types_alloc_size = _new_size;
        }
    }
    {
        if (*split).lengths_alloc_size < max_num_blocks {
            let mut _new_size
                : usize
                = if (*split).lengths_alloc_size == 0i32 as (usize) {
                      max_num_blocks
                  } else {
                      (*split).lengths_alloc_size
                  };
            let mut new_array : *mut u32;
            while _new_size < max_num_blocks {
                _new_size = _new_size.wrapping_mul(2i32 as (usize));
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(std::mem::size_of::<u32>())
                            ) as (*mut u32)
                        } else {
                            0i32 as (*mut std::os::raw::c_void) as (*mut u32)
                        };
            if !!(0i32 == 0) && ((*split).lengths_alloc_size != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut std::os::raw::c_void),
                    (*split).lengths as (*const std::os::raw::c_void),
                    (*split).lengths_alloc_size.wrapping_mul(
                        std::mem::size_of::<u32>()
                    )
                );
            }
            {
                BrotliFree(m,(*split).lengths as (*mut std::os::raw::c_void));
                (*split).lengths = 0i32 as (*mut std::os::raw::c_void) as (*mut u32);
            }
            (*split).lengths = new_array;
            (*split).lengths_alloc_size = _new_size;
        }
    }
    if !(0i32 == 0) {
        return;
    }
    (*(*self).split_).num_blocks = max_num_blocks;
    if *histograms == 0i32 as (*mut HistogramDistance) {
        0i32;
    } else {
        __assert_fail(
            b"*histograms == 0\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"InitBlockSplitterDistance\0".as_ptr()
        );
    }
    *histograms_size = max_num_types;
    *histograms = if *histograms_size != 0 {
                      BrotliAllocate(
                          m,
                          (*histograms_size).wrapping_mul(
                              std::mem::size_of::<HistogramDistance>()
                          )
                      ) as (*mut HistogramDistance)
                  } else {
                      0i32 as (*mut std::os::raw::c_void) as (*mut HistogramDistance)
                  };
    (*self).histograms_ = *histograms;
    if !(0i32 == 0) {
        return;
    }
    HistogramClearDistance(
        &mut *(*self).histograms_.offset(
                  0i32 as (isize)
              ) as (*mut HistogramDistance)
    );
    *(*self).last_histogram_ix_.offset(0i32 as (isize)) = {
                                                              let _rhs = 0i32;
                                                              let _lhs
                                                                  = &mut *(*self).last_histogram_ix_.offset(
                                                                              1i32 as (isize)
                                                                          );
                                                              *_lhs = _rhs as (usize);
                                                              *_lhs
                                                          };
}

unsafe extern fn HistogramAddCommand(
    mut self : *mut HistogramCommand, mut val : usize
) {
    {
        let _rhs = 1;
        let _lhs = &mut *(*self).data_.offset(val as (isize));
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               1 as (usize)
                           );
}

unsafe extern fn brotli_max_size_t(
    mut a : usize, mut b : usize
) -> usize {
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

unsafe extern fn HistogramAddHistogramCommand(
    mut self : *mut HistogramCommand, mut v : *const HistogramCommand
) {
    let mut i : usize;
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               (*v).total_count_
                           );
    i = 0i32 as (usize);
    while i < 704i32 as (usize) {
        {
            let _rhs = *(*v).data_.offset(i as (isize));
            let _lhs = &mut *(*self).data_.offset(i as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs);
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn BlockSplitterFinishBlockCommand(
    mut self : *mut BlockSplitterCommand, mut is_final : i32
) {
    let mut split : *mut BlockSplit = (*self).split_;
    let mut last_entropy : *mut f64 = (*self).last_entropy_;
    let mut histograms : *mut HistogramCommand = (*self).histograms_;
    (*self).block_size_ = brotli_max_size_t(
                              (*self).block_size_,
                              (*self).min_block_size_
                          );
    if (*self).num_blocks_ == 0i32 as (usize) {
        *(*split).lengths.offset(
             0i32 as (isize)
         ) = (*self).block_size_ as (u32);
        *(*split).types.offset(0i32 as (isize)) = 0i32 as (u8);
        *last_entropy.offset(0i32 as (isize)) = BitsEntropy(
                                                    (*histograms.offset(
                                                          0i32 as (isize)
                                                      )).data_ as (*const u32),
                                                    (*self).alphabet_size_
                                                );
        *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                     0i32 as (isize)
                                                 );
        (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                  1 as (usize)
                              );
        (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
        (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                         1 as (usize)
                                     );
        if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
            HistogramClearCommand(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramCommand)
            );
        }
        (*self).block_size_ = 0i32 as (usize);
    } else if (*self).block_size_ > 0i32 as (usize) {
        let mut entropy
            : f64
            = BitsEntropy(
                  (*histograms.offset(
                        (*self).curr_histogram_ix_ as (isize)
                    )).data_ as (*const u32),
                  (*self).alphabet_size_
              );
        let mut combined_histo : *mut HistogramCommand;
        let mut combined_entropy : *mut f64;
        let mut diff : *mut f64;
        let mut j : usize;
        j = 0i32 as (usize);
        while j < 2i32 as (usize) {
            {
                let mut last_histogram_ix
                    : usize
                    = *(*self).last_histogram_ix_.offset(j as (isize));
                *combined_histo.offset(j as (isize)) = *histograms.offset(
                                                            (*self).curr_histogram_ix_ as (isize)
                                                        );
                HistogramAddHistogramCommand(
                    &mut *combined_histo.offset(
                              j as (isize)
                          ) as (*mut HistogramCommand),
                    &mut *histograms.offset(
                              last_histogram_ix as (isize)
                          ) as (*mut HistogramCommand) as (*const HistogramCommand)
                );
                *combined_entropy.offset(j as (isize)) = BitsEntropy(
                                                             &mut *(*combined_histo.offset(
                                                                         j as (isize)
                                                                     )).data_.offset(
                                                                       0i32 as (isize)
                                                                   ) as (*mut u32) as (*const u32),
                                                             (*self).alphabet_size_
                                                         );
                *diff.offset(j as (isize)) = *combined_entropy.offset(
                                                  j as (isize)
                                              ) - entropy - *last_entropy.offset(j as (isize));
            }
            j = j.wrapping_add(1 as (usize));
        }
        if (*split).num_types < 256i32 as (usize) && (*diff.offset(
                                                           0i32 as (isize)
                                                       ) > (*self).split_threshold_) && (*diff.offset(
                                                                                              1i32 as (isize)
                                                                                          ) > (*self).split_threshold_) {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*split).num_types as (u8);
            *(*self).last_histogram_ix_.offset(
                 1i32 as (isize)
             ) = *(*self).last_histogram_ix_.offset(0i32 as (isize));
            *(*self).last_histogram_ix_.offset(
                 0i32 as (isize)
             ) = (*split).num_types as (u8) as (usize);
            *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            *last_entropy.offset(0i32 as (isize)) = entropy;
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
            (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                             1 as (usize)
                                         );
            if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
                HistogramClearCommand(
                    &mut *histograms.offset(
                              (*self).curr_histogram_ix_ as (isize)
                          ) as (*mut HistogramCommand)
                );
            }
            (*self).block_size_ = 0i32 as (usize);
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else if *diff.offset(1i32 as (isize)) < *diff.offset(
                                                       0i32 as (isize)
                                                   ) - 20.0f64 {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = *(*split).types.offset(
                      (*self).num_blocks_.wrapping_sub(2i32 as (usize)) as (isize)
                  );
            {
                let mut __brotli_swap_tmp
                    : usize
                    = *(*self).last_histogram_ix_.offset(0i32 as (isize));
                *(*self).last_histogram_ix_.offset(
                     0i32 as (isize)
                 ) = *(*self).last_histogram_ix_.offset(1i32 as (isize));
                *(*self).last_histogram_ix_.offset(
                     1i32 as (isize)
                 ) = __brotli_swap_tmp;
            }
            *histograms.offset(
                 *(*self).last_histogram_ix_.offset(0i32 as (isize)) as (isize)
             ) = *combined_histo.offset(1i32 as (isize));
            *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            *last_entropy.offset(0i32 as (isize)) = *combined_entropy.offset(
                                                         1i32 as (isize)
                                                     );
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*self).block_size_ = 0i32 as (usize);
            HistogramClearCommand(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramCommand)
            );
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else {
            {
                let _rhs = (*self).block_size_ as (u32);
                let _lhs
                    = &mut *(*split).lengths.offset(
                                (*self).num_blocks_.wrapping_sub(1i32 as (usize)) as (isize)
                            );
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            *histograms.offset(
                 *(*self).last_histogram_ix_.offset(0i32 as (isize)) as (isize)
             ) = *combined_histo.offset(0i32 as (isize));
            *last_entropy.offset(0i32 as (isize)) = *combined_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            if (*split).num_types == 1i32 as (usize) {
                *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                             0i32 as (isize)
                                                         );
            }
            (*self).block_size_ = 0i32 as (usize);
            HistogramClearCommand(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramCommand)
            );
            if {
                   (*self).merge_last_count_ = (*self).merge_last_count_.wrapping_add(
                                                   1 as (usize)
                                               );
                   (*self).merge_last_count_
               } > 1i32 as (usize) {
                (*self).target_block_size_ = (*self).target_block_size_.wrapping_add(
                                                 (*self).min_block_size_
                                             );
            }
        }
    }
    if is_final != 0 {
        *(*self).histograms_size_ = (*split).num_types;
        (*split).num_blocks = (*self).num_blocks_;
    }
}

unsafe extern fn BlockSplitterAddSymbolCommand(
    mut self : *mut BlockSplitterCommand, mut symbol : usize
) {
    HistogramAddCommand(
        &mut *(*self).histograms_.offset(
                  (*self).curr_histogram_ix_ as (isize)
              ) as (*mut HistogramCommand),
        symbol
    );
    (*self).block_size_ = (*self).block_size_.wrapping_add(
                              1 as (usize)
                          );
    if (*self).block_size_ == (*self).target_block_size_ {
        BlockSplitterFinishBlockCommand(self,0i32);
    }
}

unsafe extern fn HistogramAddLiteral(
    mut self : *mut HistogramLiteral, mut val : usize
) {
    {
        let _rhs = 1;
        let _lhs = &mut *(*self).data_.offset(val as (isize));
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               1 as (usize)
                           );
}

unsafe extern fn HistogramAddHistogramLiteral(
    mut self : *mut HistogramLiteral, mut v : *const HistogramLiteral
) {
    let mut i : usize;
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               (*v).total_count_
                           );
    i = 0i32 as (usize);
    while i < 256i32 as (usize) {
        {
            let _rhs = *(*v).data_.offset(i as (isize));
            let _lhs = &mut *(*self).data_.offset(i as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs);
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn BlockSplitterFinishBlockLiteral(
    mut self : *mut BlockSplitterLiteral, mut is_final : i32
) {
    let mut split : *mut BlockSplit = (*self).split_;
    let mut last_entropy : *mut f64 = (*self).last_entropy_;
    let mut histograms : *mut HistogramLiteral = (*self).histograms_;
    (*self).block_size_ = brotli_max_size_t(
                              (*self).block_size_,
                              (*self).min_block_size_
                          );
    if (*self).num_blocks_ == 0i32 as (usize) {
        *(*split).lengths.offset(
             0i32 as (isize)
         ) = (*self).block_size_ as (u32);
        *(*split).types.offset(0i32 as (isize)) = 0i32 as (u8);
        *last_entropy.offset(0i32 as (isize)) = BitsEntropy(
                                                    (*histograms.offset(
                                                          0i32 as (isize)
                                                      )).data_ as (*const u32),
                                                    (*self).alphabet_size_
                                                );
        *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                     0i32 as (isize)
                                                 );
        (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                  1 as (usize)
                              );
        (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
        (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                         1 as (usize)
                                     );
        if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
            HistogramClearLiteral(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramLiteral)
            );
        }
        (*self).block_size_ = 0i32 as (usize);
    } else if (*self).block_size_ > 0i32 as (usize) {
        let mut entropy
            : f64
            = BitsEntropy(
                  (*histograms.offset(
                        (*self).curr_histogram_ix_ as (isize)
                    )).data_ as (*const u32),
                  (*self).alphabet_size_
              );
        let mut combined_histo : *mut HistogramLiteral;
        let mut combined_entropy : *mut f64;
        let mut diff : *mut f64;
        let mut j : usize;
        j = 0i32 as (usize);
        while j < 2i32 as (usize) {
            {
                let mut last_histogram_ix
                    : usize
                    = *(*self).last_histogram_ix_.offset(j as (isize));
                *combined_histo.offset(j as (isize)) = *histograms.offset(
                                                            (*self).curr_histogram_ix_ as (isize)
                                                        );
                HistogramAddHistogramLiteral(
                    &mut *combined_histo.offset(
                              j as (isize)
                          ) as (*mut HistogramLiteral),
                    &mut *histograms.offset(
                              last_histogram_ix as (isize)
                          ) as (*mut HistogramLiteral) as (*const HistogramLiteral)
                );
                *combined_entropy.offset(j as (isize)) = BitsEntropy(
                                                             &mut *(*combined_histo.offset(
                                                                         j as (isize)
                                                                     )).data_.offset(
                                                                       0i32 as (isize)
                                                                   ) as (*mut u32) as (*const u32),
                                                             (*self).alphabet_size_
                                                         );
                *diff.offset(j as (isize)) = *combined_entropy.offset(
                                                  j as (isize)
                                              ) - entropy - *last_entropy.offset(j as (isize));
            }
            j = j.wrapping_add(1 as (usize));
        }
        if (*split).num_types < 256i32 as (usize) && (*diff.offset(
                                                           0i32 as (isize)
                                                       ) > (*self).split_threshold_) && (*diff.offset(
                                                                                              1i32 as (isize)
                                                                                          ) > (*self).split_threshold_) {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*split).num_types as (u8);
            *(*self).last_histogram_ix_.offset(
                 1i32 as (isize)
             ) = *(*self).last_histogram_ix_.offset(0i32 as (isize));
            *(*self).last_histogram_ix_.offset(
                 0i32 as (isize)
             ) = (*split).num_types as (u8) as (usize);
            *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            *last_entropy.offset(0i32 as (isize)) = entropy;
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
            (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                             1 as (usize)
                                         );
            if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
                HistogramClearLiteral(
                    &mut *histograms.offset(
                              (*self).curr_histogram_ix_ as (isize)
                          ) as (*mut HistogramLiteral)
                );
            }
            (*self).block_size_ = 0i32 as (usize);
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else if *diff.offset(1i32 as (isize)) < *diff.offset(
                                                       0i32 as (isize)
                                                   ) - 20.0f64 {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = *(*split).types.offset(
                      (*self).num_blocks_.wrapping_sub(2i32 as (usize)) as (isize)
                  );
            {
                let mut __brotli_swap_tmp
                    : usize
                    = *(*self).last_histogram_ix_.offset(0i32 as (isize));
                *(*self).last_histogram_ix_.offset(
                     0i32 as (isize)
                 ) = *(*self).last_histogram_ix_.offset(1i32 as (isize));
                *(*self).last_histogram_ix_.offset(
                     1i32 as (isize)
                 ) = __brotli_swap_tmp;
            }
            *histograms.offset(
                 *(*self).last_histogram_ix_.offset(0i32 as (isize)) as (isize)
             ) = *combined_histo.offset(1i32 as (isize));
            *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            *last_entropy.offset(0i32 as (isize)) = *combined_entropy.offset(
                                                         1i32 as (isize)
                                                     );
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*self).block_size_ = 0i32 as (usize);
            HistogramClearLiteral(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramLiteral)
            );
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else {
            {
                let _rhs = (*self).block_size_ as (u32);
                let _lhs
                    = &mut *(*split).lengths.offset(
                                (*self).num_blocks_.wrapping_sub(1i32 as (usize)) as (isize)
                            );
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            *histograms.offset(
                 *(*self).last_histogram_ix_.offset(0i32 as (isize)) as (isize)
             ) = *combined_histo.offset(0i32 as (isize));
            *last_entropy.offset(0i32 as (isize)) = *combined_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            if (*split).num_types == 1i32 as (usize) {
                *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                             0i32 as (isize)
                                                         );
            }
            (*self).block_size_ = 0i32 as (usize);
            HistogramClearLiteral(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramLiteral)
            );
            if {
                   (*self).merge_last_count_ = (*self).merge_last_count_.wrapping_add(
                                                   1 as (usize)
                                               );
                   (*self).merge_last_count_
               } > 1i32 as (usize) {
                (*self).target_block_size_ = (*self).target_block_size_.wrapping_add(
                                                 (*self).min_block_size_
                                             );
            }
        }
    }
    if is_final != 0 {
        *(*self).histograms_size_ = (*split).num_types;
        (*split).num_blocks = (*self).num_blocks_;
    }
}

unsafe extern fn BlockSplitterAddSymbolLiteral(
    mut self : *mut BlockSplitterLiteral, mut symbol : usize
) {
    HistogramAddLiteral(
        &mut *(*self).histograms_.offset(
                  (*self).curr_histogram_ix_ as (isize)
              ) as (*mut HistogramLiteral),
        symbol
    );
    (*self).block_size_ = (*self).block_size_.wrapping_add(
                              1 as (usize)
                          );
    if (*self).block_size_ == (*self).target_block_size_ {
        BlockSplitterFinishBlockLiteral(self,0i32);
    }
}

unsafe extern fn Context(
    mut p1 : u8, mut p2 : u8, mut mode : ContextType
) -> u8 {
    if mode as (i32) == ContextType::CONTEXT_LSB6 as (i32) {
        return (p1 as (i32) & 0x3fi32) as (u8);
    }
    if mode as (i32) == ContextType::CONTEXT_MSB6 as (i32) {
        return (p1 as (i32) >> 2i32) as (u8);
    }
    if mode as (i32) == ContextType::CONTEXT_UTF8 as (i32) {
        return
            (*kUTF8ContextLookup.offset(
                  p1 as (isize)
              ) as (i32) | *kUTF8ContextLookup.offset(
                                (p2 as (i32) + 256i32) as (isize)
                            ) as (i32)) as (u8);
    }
    if mode as (i32) == ContextType::CONTEXT_SIGNED as (i32) {
        return
            ((*kSigned3BitContextLookup.offset(
                   p1 as (isize)
               ) as (i32) << 3i32) + *kSigned3BitContextLookup.offset(
                                          p2 as (isize)
                                      ) as (i32)) as (u8);
    }
    0i32 as (u8)
}

unsafe extern fn ContextBlockSplitterFinishBlock(
    mut self : *mut ContextBlockSplitter, mut is_final : i32
) {
    let mut split : *mut BlockSplit = (*self).split_;
    let num_contexts : usize = (*self).num_contexts_;
    let mut last_entropy : *mut f64 = (*self).last_entropy_;
    let mut histograms : *mut HistogramLiteral = (*self).histograms_;
    if (*self).block_size_ < (*self).min_block_size_ {
        (*self).block_size_ = (*self).min_block_size_;
    }
    if (*self).num_blocks_ == 0i32 as (usize) {
        let mut i : usize;
        *(*split).lengths.offset(
             0i32 as (isize)
         ) = (*self).block_size_ as (u32);
        *(*split).types.offset(0i32 as (isize)) = 0i32 as (u8);
        i = 0i32 as (usize);
        while i < num_contexts {
            {
                *last_entropy.offset(i as (isize)) = BitsEntropy(
                                                         (*histograms.offset(
                                                               i as (isize)
                                                           )).data_ as (*const u32),
                                                         (*self).alphabet_size_
                                                     );
                *last_entropy.offset(
                     num_contexts.wrapping_add(i) as (isize)
                 ) = *last_entropy.offset(i as (isize));
            }
            i = i.wrapping_add(1 as (usize));
        }
        (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                  1 as (usize)
                              );
        (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
        (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                         num_contexts
                                     );
        if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
            ClearHistogramsLiteral(
                &mut *(*self).histograms_.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramLiteral),
                (*self).num_contexts_
            );
        }
        (*self).block_size_ = 0i32 as (usize);
    } else if (*self).block_size_ > 0i32 as (usize) {
        let mut entropy : *mut f64;
        let mut combined_histo : *mut HistogramLiteral;
        let mut combined_entropy : *mut f64;
        let mut diff : *mut f64 = 0.0f64 as (*mut f64);
        let mut i : usize;
        i = 0i32 as (usize);
        while i < num_contexts {
            {
                let mut curr_histo_ix
                    : usize
                    = (*self).curr_histogram_ix_.wrapping_add(i);
                let mut j : usize;
                *entropy.offset(i as (isize)) = BitsEntropy(
                                                    (*histograms.offset(
                                                          curr_histo_ix as (isize)
                                                      )).data_ as (*const u32),
                                                    (*self).alphabet_size_
                                                );
                j = 0i32 as (usize);
                while j < 2i32 as (usize) {
                    {
                        let mut jx : usize = j.wrapping_mul(num_contexts).wrapping_add(i);
                        let mut last_histogram_ix
                            : usize
                            = (*(*self).last_histogram_ix_.offset(j as (isize))).wrapping_add(
                                  i
                              );
                        *combined_histo.offset(jx as (isize)) = *histograms.offset(
                                                                     curr_histo_ix as (isize)
                                                                 );
                        HistogramAddHistogramLiteral(
                            &mut *combined_histo.offset(
                                      jx as (isize)
                                  ) as (*mut HistogramLiteral),
                            &mut *histograms.offset(
                                      last_histogram_ix as (isize)
                                  ) as (*mut HistogramLiteral) as (*const HistogramLiteral)
                        );
                        *combined_entropy.offset(jx as (isize)) = BitsEntropy(
                                                                      &mut *(*combined_histo.offset(
                                                                                  jx as (isize)
                                                                              )).data_.offset(
                                                                                0i32 as (isize)
                                                                            ) as (*mut u32) as (*const u32),
                                                                      (*self).alphabet_size_
                                                                  );
                        {
                            let _rhs
                                = *combined_entropy.offset(jx as (isize)) - *entropy.offset(
                                                                                 i as (isize)
                                                                             ) - *last_entropy.offset(
                                                                                      jx as (isize)
                                                                                  );
                            let _lhs = &mut *diff.offset(j as (isize));
                            *_lhs = *_lhs + _rhs;
                        }
                    }
                    j = j.wrapping_add(1 as (usize));
                }
            }
            i = i.wrapping_add(1 as (usize));
        }
        if (*split).num_types < (*self).max_block_types_ && (*diff.offset(
                                                                  0i32 as (isize)
                                                              ) > (*self).split_threshold_) && (*diff.offset(
                                                                                                     1i32 as (isize)
                                                                                                 ) > (*self).split_threshold_) {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*split).num_types as (u8);
            *(*self).last_histogram_ix_.offset(
                 1i32 as (isize)
             ) = *(*self).last_histogram_ix_.offset(0i32 as (isize));
            *(*self).last_histogram_ix_.offset(
                 0i32 as (isize)
             ) = (*split).num_types.wrapping_mul(num_contexts);
            i = 0i32 as (usize);
            while i < num_contexts {
                {
                    *last_entropy.offset(
                         num_contexts.wrapping_add(i) as (isize)
                     ) = *last_entropy.offset(i as (isize));
                    *last_entropy.offset(i as (isize)) = *entropy.offset(i as (isize));
                }
                i = i.wrapping_add(1 as (usize));
            }
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
            (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                             num_contexts
                                         );
            if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
                ClearHistogramsLiteral(
                    &mut *(*self).histograms_.offset(
                              (*self).curr_histogram_ix_ as (isize)
                          ) as (*mut HistogramLiteral),
                    (*self).num_contexts_
                );
            }
            (*self).block_size_ = 0i32 as (usize);
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else if *diff.offset(1i32 as (isize)) < *diff.offset(
                                                       0i32 as (isize)
                                                   ) - 20.0f64 {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = *(*split).types.offset(
                      (*self).num_blocks_.wrapping_sub(2i32 as (usize)) as (isize)
                  );
            {
                let mut __brotli_swap_tmp
                    : usize
                    = *(*self).last_histogram_ix_.offset(0i32 as (isize));
                *(*self).last_histogram_ix_.offset(
                     0i32 as (isize)
                 ) = *(*self).last_histogram_ix_.offset(1i32 as (isize));
                *(*self).last_histogram_ix_.offset(
                     1i32 as (isize)
                 ) = __brotli_swap_tmp;
            }
            i = 0i32 as (usize);
            while i < num_contexts {
                {
                    *histograms.offset(
                         (*(*self).last_histogram_ix_.offset(0i32 as (isize))).wrapping_add(
                             i
                         ) as (isize)
                     ) = *combined_histo.offset(
                              num_contexts.wrapping_add(i) as (isize)
                          );
                    *last_entropy.offset(
                         num_contexts.wrapping_add(i) as (isize)
                     ) = *last_entropy.offset(i as (isize));
                    *last_entropy.offset(i as (isize)) = *combined_entropy.offset(
                                                              num_contexts.wrapping_add(
                                                                  i
                                                              ) as (isize)
                                                          );
                    HistogramClearLiteral(
                        &mut *histograms.offset(
                                  (*self).curr_histogram_ix_.wrapping_add(i) as (isize)
                              ) as (*mut HistogramLiteral)
                    );
                }
                i = i.wrapping_add(1 as (usize));
            }
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*self).block_size_ = 0i32 as (usize);
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else {
            {
                let _rhs = (*self).block_size_ as (u32);
                let _lhs
                    = &mut *(*split).lengths.offset(
                                (*self).num_blocks_.wrapping_sub(1i32 as (usize)) as (isize)
                            );
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            i = 0i32 as (usize);
            while i < num_contexts {
                {
                    *histograms.offset(
                         (*(*self).last_histogram_ix_.offset(0i32 as (isize))).wrapping_add(
                             i
                         ) as (isize)
                     ) = *combined_histo.offset(i as (isize));
                    *last_entropy.offset(i as (isize)) = *combined_entropy.offset(
                                                              i as (isize)
                                                          );
                    if (*split).num_types == 1i32 as (usize) {
                        *last_entropy.offset(
                             num_contexts.wrapping_add(i) as (isize)
                         ) = *last_entropy.offset(i as (isize));
                    }
                    HistogramClearLiteral(
                        &mut *histograms.offset(
                                  (*self).curr_histogram_ix_.wrapping_add(i) as (isize)
                              ) as (*mut HistogramLiteral)
                    );
                }
                i = i.wrapping_add(1 as (usize));
            }
            (*self).block_size_ = 0i32 as (usize);
            if {
                   (*self).merge_last_count_ = (*self).merge_last_count_.wrapping_add(
                                                   1 as (usize)
                                               );
                   (*self).merge_last_count_
               } > 1i32 as (usize) {
                (*self).target_block_size_ = (*self).target_block_size_.wrapping_add(
                                                 (*self).min_block_size_
                                             );
            }
        }
    }
    if is_final != 0 {
        *(*self).histograms_size_ = (*split).num_types.wrapping_mul(
                                        num_contexts
                                    );
        (*split).num_blocks = (*self).num_blocks_;
    }
}

unsafe extern fn ContextBlockSplitterAddSymbol(
    mut self : *mut ContextBlockSplitter,
    mut symbol : usize,
    mut context : usize
) {
    HistogramAddLiteral(
        &mut *(*self).histograms_.offset(
                  (*self).curr_histogram_ix_.wrapping_add(context) as (isize)
              ) as (*mut HistogramLiteral),
        symbol
    );
    (*self).block_size_ = (*self).block_size_.wrapping_add(
                              1 as (usize)
                          );
    if (*self).block_size_ == (*self).target_block_size_ {
        ContextBlockSplitterFinishBlock(self,0i32);
    }
}

unsafe extern fn CommandCopyLen(mut self : *const Command) -> u32 {
    (*self).copy_len_ & 0xffffffi32 as (u32)
}

unsafe extern fn HistogramAddDistance(
    mut self : *mut HistogramDistance, mut val : usize
) {
    {
        let _rhs = 1;
        let _lhs = &mut *(*self).data_.offset(val as (isize));
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               1 as (usize)
                           );
}

unsafe extern fn HistogramAddHistogramDistance(
    mut self : *mut HistogramDistance, mut v : *const HistogramDistance
) {
    let mut i : usize;
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               (*v).total_count_
                           );
    i = 0i32 as (usize);
    while i < 520i32 as (usize) {
        {
            let _rhs = *(*v).data_.offset(i as (isize));
            let _lhs = &mut *(*self).data_.offset(i as (isize));
            *_lhs = (*_lhs).wrapping_add(_rhs);
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn BlockSplitterFinishBlockDistance(
    mut self : *mut BlockSplitterDistance, mut is_final : i32
) {
    let mut split : *mut BlockSplit = (*self).split_;
    let mut last_entropy : *mut f64 = (*self).last_entropy_;
    let mut histograms : *mut HistogramDistance = (*self).histograms_;
    (*self).block_size_ = brotli_max_size_t(
                              (*self).block_size_,
                              (*self).min_block_size_
                          );
    if (*self).num_blocks_ == 0i32 as (usize) {
        *(*split).lengths.offset(
             0i32 as (isize)
         ) = (*self).block_size_ as (u32);
        *(*split).types.offset(0i32 as (isize)) = 0i32 as (u8);
        *last_entropy.offset(0i32 as (isize)) = BitsEntropy(
                                                    (*histograms.offset(
                                                          0i32 as (isize)
                                                      )).data_ as (*const u32),
                                                    (*self).alphabet_size_
                                                );
        *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                     0i32 as (isize)
                                                 );
        (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                  1 as (usize)
                              );
        (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
        (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                         1 as (usize)
                                     );
        if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
            HistogramClearDistance(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramDistance)
            );
        }
        (*self).block_size_ = 0i32 as (usize);
    } else if (*self).block_size_ > 0i32 as (usize) {
        let mut entropy
            : f64
            = BitsEntropy(
                  (*histograms.offset(
                        (*self).curr_histogram_ix_ as (isize)
                    )).data_ as (*const u32),
                  (*self).alphabet_size_
              );
        let mut combined_histo : *mut HistogramDistance;
        let mut combined_entropy : *mut f64;
        let mut diff : *mut f64;
        let mut j : usize;
        j = 0i32 as (usize);
        while j < 2i32 as (usize) {
            {
                let mut last_histogram_ix
                    : usize
                    = *(*self).last_histogram_ix_.offset(j as (isize));
                *combined_histo.offset(j as (isize)) = *histograms.offset(
                                                            (*self).curr_histogram_ix_ as (isize)
                                                        );
                HistogramAddHistogramDistance(
                    &mut *combined_histo.offset(
                              j as (isize)
                          ) as (*mut HistogramDistance),
                    &mut *histograms.offset(
                              last_histogram_ix as (isize)
                          ) as (*mut HistogramDistance) as (*const HistogramDistance)
                );
                *combined_entropy.offset(j as (isize)) = BitsEntropy(
                                                             &mut *(*combined_histo.offset(
                                                                         j as (isize)
                                                                     )).data_.offset(
                                                                       0i32 as (isize)
                                                                   ) as (*mut u32) as (*const u32),
                                                             (*self).alphabet_size_
                                                         );
                *diff.offset(j as (isize)) = *combined_entropy.offset(
                                                  j as (isize)
                                              ) - entropy - *last_entropy.offset(j as (isize));
            }
            j = j.wrapping_add(1 as (usize));
        }
        if (*split).num_types < 256i32 as (usize) && (*diff.offset(
                                                           0i32 as (isize)
                                                       ) > (*self).split_threshold_) && (*diff.offset(
                                                                                              1i32 as (isize)
                                                                                          ) > (*self).split_threshold_) {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*split).num_types as (u8);
            *(*self).last_histogram_ix_.offset(
                 1i32 as (isize)
             ) = *(*self).last_histogram_ix_.offset(0i32 as (isize));
            *(*self).last_histogram_ix_.offset(
                 0i32 as (isize)
             ) = (*split).num_types as (u8) as (usize);
            *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            *last_entropy.offset(0i32 as (isize)) = entropy;
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*split).num_types = (*split).num_types.wrapping_add(1 as (usize));
            (*self).curr_histogram_ix_ = (*self).curr_histogram_ix_.wrapping_add(
                                             1 as (usize)
                                         );
            if (*self).curr_histogram_ix_ < *(*self).histograms_size_ {
                HistogramClearDistance(
                    &mut *histograms.offset(
                              (*self).curr_histogram_ix_ as (isize)
                          ) as (*mut HistogramDistance)
                );
            }
            (*self).block_size_ = 0i32 as (usize);
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else if *diff.offset(1i32 as (isize)) < *diff.offset(
                                                       0i32 as (isize)
                                                   ) - 20.0f64 {
            *(*split).lengths.offset(
                 (*self).num_blocks_ as (isize)
             ) = (*self).block_size_ as (u32);
            *(*split).types.offset(
                 (*self).num_blocks_ as (isize)
             ) = *(*split).types.offset(
                      (*self).num_blocks_.wrapping_sub(2i32 as (usize)) as (isize)
                  );
            {
                let mut __brotli_swap_tmp
                    : usize
                    = *(*self).last_histogram_ix_.offset(0i32 as (isize));
                *(*self).last_histogram_ix_.offset(
                     0i32 as (isize)
                 ) = *(*self).last_histogram_ix_.offset(1i32 as (isize));
                *(*self).last_histogram_ix_.offset(
                     1i32 as (isize)
                 ) = __brotli_swap_tmp;
            }
            *histograms.offset(
                 *(*self).last_histogram_ix_.offset(0i32 as (isize)) as (isize)
             ) = *combined_histo.offset(1i32 as (isize));
            *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            *last_entropy.offset(0i32 as (isize)) = *combined_entropy.offset(
                                                         1i32 as (isize)
                                                     );
            (*self).num_blocks_ = (*self).num_blocks_.wrapping_add(
                                      1 as (usize)
                                  );
            (*self).block_size_ = 0i32 as (usize);
            HistogramClearDistance(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramDistance)
            );
            (*self).merge_last_count_ = 0i32 as (usize);
            (*self).target_block_size_ = (*self).min_block_size_;
        } else {
            {
                let _rhs = (*self).block_size_ as (u32);
                let _lhs
                    = &mut *(*split).lengths.offset(
                                (*self).num_blocks_.wrapping_sub(1i32 as (usize)) as (isize)
                            );
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            *histograms.offset(
                 *(*self).last_histogram_ix_.offset(0i32 as (isize)) as (isize)
             ) = *combined_histo.offset(0i32 as (isize));
            *last_entropy.offset(0i32 as (isize)) = *combined_entropy.offset(
                                                         0i32 as (isize)
                                                     );
            if (*split).num_types == 1i32 as (usize) {
                *last_entropy.offset(1i32 as (isize)) = *last_entropy.offset(
                                                             0i32 as (isize)
                                                         );
            }
            (*self).block_size_ = 0i32 as (usize);
            HistogramClearDistance(
                &mut *histograms.offset(
                          (*self).curr_histogram_ix_ as (isize)
                      ) as (*mut HistogramDistance)
            );
            if {
                   (*self).merge_last_count_ = (*self).merge_last_count_.wrapping_add(
                                                   1 as (usize)
                                               );
                   (*self).merge_last_count_
               } > 1i32 as (usize) {
                (*self).target_block_size_ = (*self).target_block_size_.wrapping_add(
                                                 (*self).min_block_size_
                                             );
            }
        }
    }
    if is_final != 0 {
        *(*self).histograms_size_ = (*split).num_types;
        (*split).num_blocks = (*self).num_blocks_;
    }
}

unsafe extern fn BlockSplitterAddSymbolDistance(
    mut self : *mut BlockSplitterDistance, mut symbol : usize
) {
    HistogramAddDistance(
        &mut *(*self).histograms_.offset(
                  (*self).curr_histogram_ix_ as (isize)
              ) as (*mut HistogramDistance),
        symbol
    );
    (*self).block_size_ = (*self).block_size_.wrapping_add(
                              1 as (usize)
                          );
    if (*self).block_size_ == (*self).target_block_size_ {
        BlockSplitterFinishBlockDistance(self,0i32);
    }
}

unsafe extern fn MapStaticContexts(
    mut m : *mut MemoryManager,
    mut num_contexts : usize,
    mut static_context_map : *const u32,
    mut mb : *mut MetaBlockSplit
) {
    let mut i : usize;
    if (*mb).literal_context_map == 0i32 as (*mut u32) {
        0i32;
    } else {
        __assert_fail(
            b"mb->literal_context_map == 0\0".as_ptr(),
            file!().as_ptr(),
            line!(),
            b"MapStaticContexts\0".as_ptr()
        );
    }
    (*mb).literal_context_map_size = (*mb).literal_split.num_types << 6i32;
    (*mb).literal_context_map = if (*mb).literal_context_map_size != 0 {
                                    BrotliAllocate(
                                        m,
                                        (*mb).literal_context_map_size.wrapping_mul(
                                            std::mem::size_of::<u32>()
                                        )
                                    ) as (*mut u32)
                                } else {
                                    0i32 as (*mut std::os::raw::c_void) as (*mut u32)
                                };
    if !(0i32 == 0) {
        return;
    }
    i = 0i32 as (usize);
    while i < (*mb).literal_split.num_types {
        {
            let mut offset : u32 = i.wrapping_mul(num_contexts) as (u32);
            let mut j : usize;
            j = 0i32 as (usize);
            while j < (1u32 << 6i32) as (usize) {
                {
                    *(*mb).literal_context_map.offset(
                         (i << 6i32).wrapping_add(j) as (isize)
                     ) = offset.wrapping_add(*static_context_map.offset(j as (isize)));
                }
                j = j.wrapping_add(1 as (usize));
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
}

unsafe extern fn BrotliBuildMetaBlockGreedyInternal(
    mut m : *mut MemoryManager,
    mut ringbuffer : *const u8,
    mut pos : usize,
    mut mask : usize,
    mut prev_byte : u8,
    mut prev_byte2 : u8,
    mut literal_context_mode : ContextType,
    num_contexts : usize,
    mut static_context_map : *const u32,
    mut commands : *const Command,
    mut n_commands : usize,
    mut mb : *mut MetaBlockSplit
) {
    let mut lit_blocks : LitBlocks;
    let mut cmd_blocks : BlockSplitterCommand;
    let mut dist_blocks : BlockSplitterDistance;
    let mut num_literals : usize = 0i32 as (usize);
    let mut i : usize;
    i = 0i32 as (usize);
    while i < n_commands {
        {
            num_literals = num_literals.wrapping_add(
                               (*commands.offset(i as (isize))).insert_len_ as (usize)
                           );
        }
        i = i.wrapping_add(1 as (usize));
    }
    if num_contexts == 1i32 as (usize) {
        InitBlockSplitterLiteral(
            m,
            &mut lit_blocks.plain as (*mut BlockSplitterLiteral),
            256i32 as (usize),
            512i32 as (usize),
            400.0f64,
            num_literals,
            &mut (*mb).literal_split as (*mut BlockSplit),
            &mut (*mb).literal_histograms as (*mut *mut HistogramLiteral),
            &mut (*mb).literal_histograms_size as (*mut usize)
        );
    } else {
        InitContextBlockSplitter(
            m,
            &mut lit_blocks.ctx as (*mut ContextBlockSplitter),
            256i32 as (usize),
            num_contexts,
            512i32 as (usize),
            400.0f64,
            num_literals,
            &mut (*mb).literal_split as (*mut BlockSplit),
            &mut (*mb).literal_histograms as (*mut *mut HistogramLiteral),
            &mut (*mb).literal_histograms_size as (*mut usize)
        );
    }
    if !(0i32 == 0) {
        return;
    }
    InitBlockSplitterCommand(
        m,
        &mut cmd_blocks as (*mut BlockSplitterCommand),
        704i32 as (usize),
        1024i32 as (usize),
        500.0f64,
        n_commands,
        &mut (*mb).command_split as (*mut BlockSplit),
        &mut (*mb).command_histograms as (*mut *mut HistogramCommand),
        &mut (*mb).command_histograms_size as (*mut usize)
    );
    if !(0i32 == 0) {
        return;
    }
    InitBlockSplitterDistance(
        m,
        &mut dist_blocks as (*mut BlockSplitterDistance),
        64i32 as (usize),
        512i32 as (usize),
        100.0f64,
        n_commands,
        &mut (*mb).distance_split as (*mut BlockSplit),
        &mut (*mb).distance_histograms as (*mut *mut HistogramDistance),
        &mut (*mb).distance_histograms_size as (*mut usize)
    );
    if !(0i32 == 0) {
        return;
    }
    i = 0i32 as (usize);
    while i < n_commands {
        {
            let cmd : Command = *commands.offset(i as (isize));
            let mut j : usize;
            BlockSplitterAddSymbolCommand(
                &mut cmd_blocks as (*mut BlockSplitterCommand),
                cmd.cmd_prefix_ as (usize)
            );
            j = cmd.insert_len_ as (usize);
            while j != 0i32 as (usize) {
                {
                    let mut literal : u8 = *ringbuffer.offset((pos & mask) as (isize));
                    if num_contexts == 1i32 as (usize) {
                        BlockSplitterAddSymbolLiteral(
                            &mut lit_blocks.plain as (*mut BlockSplitterLiteral),
                            literal as (usize)
                        );
                    } else {
                        let mut context
                            : usize
                            = Context(prev_byte,prev_byte2,literal_context_mode) as (usize);
                        ContextBlockSplitterAddSymbol(
                            &mut lit_blocks.ctx as (*mut ContextBlockSplitter),
                            literal as (usize),
                            *static_context_map.offset(context as (isize)) as (usize)
                        );
                    }
                    prev_byte2 = prev_byte;
                    prev_byte = literal;
                    pos = pos.wrapping_add(1 as (usize));
                }
                j = j.wrapping_sub(1 as (usize));
            }
            pos = pos.wrapping_add(
                      CommandCopyLen(&cmd as (*const Command)) as (usize)
                  );
            if CommandCopyLen(&cmd as (*const Command)) != 0 {
                prev_byte2 = *ringbuffer.offset(
                                  (pos.wrapping_sub(2i32 as (usize)) & mask) as (isize)
                              );
                prev_byte = *ringbuffer.offset(
                                 (pos.wrapping_sub(1i32 as (usize)) & mask) as (isize)
                             );
                if cmd.cmd_prefix_ as (i32) >= 128i32 {
                    BlockSplitterAddSymbolDistance(
                        &mut dist_blocks as (*mut BlockSplitterDistance),
                        cmd.dist_prefix_ as (usize)
                    );
                }
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
    if num_contexts == 1i32 as (usize) {
        BlockSplitterFinishBlockLiteral(
            &mut lit_blocks.plain as (*mut BlockSplitterLiteral),
            1i32
        );
    } else {
        ContextBlockSplitterFinishBlock(
            &mut lit_blocks.ctx as (*mut ContextBlockSplitter),
            1i32
        );
    }
    BlockSplitterFinishBlockCommand(
        &mut cmd_blocks as (*mut BlockSplitterCommand),
        1i32
    );
    BlockSplitterFinishBlockDistance(
        &mut dist_blocks as (*mut BlockSplitterDistance),
        1i32
    );
    if num_contexts > 1i32 as (usize) {
        MapStaticContexts(m,num_contexts,static_context_map,mb);
    }
}

#[no_mangle]
pub unsafe extern fn BrotliBuildMetaBlockGreedy(
    mut m : *mut MemoryManager,
    mut ringbuffer : *const u8,
    mut pos : usize,
    mut mask : usize,
    mut prev_byte : u8,
    mut prev_byte2 : u8,
    mut literal_context_mode : ContextType,
    mut num_contexts : usize,
    mut static_context_map : *const u32,
    mut commands : *const Command,
    mut n_commands : usize,
    mut mb : *mut MetaBlockSplit
) { if num_contexts == 1i32 as (usize) {
        BrotliBuildMetaBlockGreedyInternal(
            m,
            ringbuffer,
            pos,
            mask,
            prev_byte,
            prev_byte2,
            literal_context_mode,
            1i32 as (usize),
            0i32 as (*mut std::os::raw::c_void) as (*const u32),
            commands,
            n_commands,
            mb
        );
    } else {
        BrotliBuildMetaBlockGreedyInternal(
            m,
            ringbuffer,
            pos,
            mask,
            prev_byte,
            prev_byte2,
            literal_context_mode,
            num_contexts,
            static_context_map,
            commands,
            n_commands,
            mb
        );
    }
}

#[no_mangle]
pub unsafe extern fn BrotliOptimizeHistograms(
    mut num_direct_distance_codes : usize,
    mut distance_postfix_bits : usize,
    mut mb : *mut MetaBlockSplit
) {
    let mut good_for_rle : *mut u8;
    let mut num_distance_codes : usize;
    let mut i : usize;
    i = 0i32 as (usize);
    while i < (*mb).literal_histograms_size {
        {
            BrotliOptimizeHuffmanCountsForRle(
                256i32 as (usize),
                (*(*mb).literal_histograms.offset(i as (isize))).data_,
                good_for_rle
            );
        }
        i = i.wrapping_add(1 as (usize));
    }
    i = 0i32 as (usize);
    while i < (*mb).command_histograms_size {
        {
            BrotliOptimizeHuffmanCountsForRle(
                704i32 as (usize),
                (*(*mb).command_histograms.offset(i as (isize))).data_,
                good_for_rle
            );
        }
        i = i.wrapping_add(1 as (usize));
    }
    num_distance_codes = (16i32 as (usize)).wrapping_add(
                             num_direct_distance_codes
                         ).wrapping_add(
                             ((2i32 as (u32)).wrapping_mul(
                                  24u32
                              ) << distance_postfix_bits) as (usize)
                         );
    i = 0i32 as (usize);
    while i < (*mb).distance_histograms_size {
        {
            BrotliOptimizeHuffmanCountsForRle(
                num_distance_codes,
                (*(*mb).distance_histograms.offset(i as (isize))).data_,
                good_for_rle
            );
        }
        i = i.wrapping_add(1 as (usize));
    }
}
