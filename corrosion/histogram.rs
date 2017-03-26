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
pub struct Command {
    pub insert_len_ : u32,
    pub copy_len_ : u32,
    pub dist_extra_ : u32,
    pub cmd_prefix_ : u16,
    pub dist_prefix_ : u16,
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
#[repr(i32)]
pub enum ContextType {
    CONTEXT_LSB6 = 0i32,
    CONTEXT_MSB6 = 1i32,
    CONTEXT_UTF8 = 2i32,
    CONTEXT_SIGNED = 3i32,
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
pub struct BlockSplitIterator {
    pub split_ : *const BlockSplit,
    pub idx_ : usize,
    pub type_ : usize,
    pub length_ : usize,
}

unsafe extern fn InitBlockSplitIterator(
    mut self : *mut BlockSplitIterator, mut split : *const BlockSplit
) {
    (*self).split_ = split;
    (*self).idx_ = 0i32 as (usize);
    (*self).type_ = 0i32 as (usize);
    (*self).length_ = if !(*split).lengths.is_null() {
                          *(*split).lengths.offset(0i32 as (isize))
                      } else {
                          0i32 as (u32)
                      } as (usize);
}

unsafe extern fn BlockSplitIteratorNext(
    mut self : *mut BlockSplitIterator
) {
    if (*self).length_ == 0i32 as (usize) {
        (*self).idx_ = (*self).idx_.wrapping_add(1 as (usize));
        (*self).type_ = *(*(*self).split_).types.offset(
                             (*self).idx_ as (isize)
                         ) as (usize);
        (*self).length_ = *(*(*self).split_).lengths.offset(
                               (*self).idx_ as (isize)
                           ) as (usize);
    }
    (*self).length_ = (*self).length_.wrapping_sub(1 as (usize));
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

unsafe extern fn CommandCopyLen(mut self : *const Command) -> u32 {
    (*self).copy_len_ & 0xffffffi32 as (u32)
}

unsafe extern fn CommandDistanceContext(
    mut self : *const Command
) -> u32 {
    let mut r : u32 = ((*self).cmd_prefix_ as (i32) >> 6i32) as (u32);
    let mut c : u32 = ((*self).cmd_prefix_ as (i32) & 7i32) as (u32);
    if (r == 0i32 as (u32) || r == 2i32 as (u32) || r == 4i32 as (u32) || r == 7i32 as (u32)) && (c <= 2i32 as (u32)) {
        return c;
    }
    3i32 as (u32)
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

#[no_mangle]
pub unsafe extern fn BrotliBuildHistogramsWithContext(
    mut cmds : *const Command,
    num_commands : usize,
    mut literal_split : *const BlockSplit,
    mut insert_and_copy_split : *const BlockSplit,
    mut dist_split : *const BlockSplit,
    mut ringbuffer : *const u8,
    mut start_pos : usize,
    mut mask : usize,
    mut prev_byte : u8,
    mut prev_byte2 : u8,
    mut context_modes : *const ContextType,
    mut literal_histograms : *mut HistogramLiteral,
    mut insert_and_copy_histograms : *mut HistogramCommand,
    mut copy_dist_histograms : *mut HistogramDistance
) {
    let mut pos : usize = start_pos;
    let mut literal_it : BlockSplitIterator;
    let mut insert_and_copy_it : BlockSplitIterator;
    let mut dist_it : BlockSplitIterator;
    let mut i : usize;
    InitBlockSplitIterator(
        &mut literal_it as (*mut BlockSplitIterator),
        literal_split
    );
    InitBlockSplitIterator(
        &mut insert_and_copy_it as (*mut BlockSplitIterator),
        insert_and_copy_split
    );
    InitBlockSplitIterator(
        &mut dist_it as (*mut BlockSplitIterator),
        dist_split
    );
    i = 0i32 as (usize);
    while i < num_commands {
        {
            let mut cmd
                : *const Command
                = &*cmds.offset(i as (isize)) as (*const Command);
            let mut j : usize;
            BlockSplitIteratorNext(
                &mut insert_and_copy_it as (*mut BlockSplitIterator)
            );
            HistogramAddCommand(
                &mut *insert_and_copy_histograms.offset(
                          insert_and_copy_it.type_ as (isize)
                      ) as (*mut HistogramCommand),
                (*cmd).cmd_prefix_ as (usize)
            );
            j = (*cmd).insert_len_ as (usize);
            while j != 0i32 as (usize) {
                {
                    let mut context : usize;
                    BlockSplitIteratorNext(
                        &mut literal_it as (*mut BlockSplitIterator)
                    );
                    context = if !context_modes.is_null() {
                                  (literal_it.type_ << 6i32).wrapping_add(
                                      Context(
                                          prev_byte,
                                          prev_byte2,
                                          *context_modes.offset(literal_it.type_ as (isize))
                                      ) as (usize)
                                  )
                              } else {
                                  literal_it.type_
                              };
                    HistogramAddLiteral(
                        &mut *literal_histograms.offset(
                                  context as (isize)
                              ) as (*mut HistogramLiteral),
                        *ringbuffer.offset((pos & mask) as (isize)) as (usize)
                    );
                    prev_byte2 = prev_byte;
                    prev_byte = *ringbuffer.offset((pos & mask) as (isize));
                    pos = pos.wrapping_add(1 as (usize));
                }
                j = j.wrapping_sub(1 as (usize));
            }
            pos = pos.wrapping_add(CommandCopyLen(cmd) as (usize));
            if CommandCopyLen(cmd) != 0 {
                prev_byte2 = *ringbuffer.offset(
                                  (pos.wrapping_sub(2i32 as (usize)) & mask) as (isize)
                              );
                prev_byte = *ringbuffer.offset(
                                 (pos.wrapping_sub(1i32 as (usize)) & mask) as (isize)
                             );
                if (*cmd).cmd_prefix_ as (i32) >= 128i32 {
                    let mut context : usize;
                    BlockSplitIteratorNext(&mut dist_it as (*mut BlockSplitIterator));
                    context = (dist_it.type_ << 2i32).wrapping_add(
                                  CommandDistanceContext(cmd) as (usize)
                              );
                    HistogramAddDistance(
                        &mut *copy_dist_histograms.offset(
                                  context as (isize)
                              ) as (*mut HistogramDistance),
                        (*cmd).dist_prefix_ as (usize)
                    );
                }
            }
        }
        i = i.wrapping_add(1 as (usize));
    }
}
