#![allow(dead_code)]
use super::utf8_util::BrotliIsMostlyUTF8;
use super::util::FastLog2;

static kMinUTF8Ratio: super::util::floatX = 0.75 as super::util::floatX;

fn brotli_min_size_t(a: usize, b: usize) -> usize {
  if a < b { a } else { b }
}

fn UTF8Position(last: usize, c: usize, clamp: usize) -> usize {
  if c < 128i32 as (usize) {
    0i32 as (usize)
  } else if c >= 192i32 as (usize) {
    brotli_min_size_t(1i32 as (usize), clamp)
  } else if last < 0xe0i32 as (usize) {
    0i32 as (usize)
  } else {
    brotli_min_size_t(2i32 as (usize), clamp)
  }
}

fn DecideMultiByteStatsLevel(pos: usize, len: usize, mask: usize, data: &[u8]) -> usize {
  let mut counts: [usize; 3] = [0usize, 0usize, 0usize];
  let mut max_utf8: usize = 1usize;
  let mut last_c: usize = 0usize;
  let mut i: usize;
  i = 0usize;
  while i < len {
    {
      let c: usize = data[((pos.wrapping_add(i) & mask) as (usize))] as (usize);
      {
        let _rhs = 1;
        let _lhs = &mut counts[UTF8Position(last_c, c, 2usize)];
        *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
      }
      last_c = c;
    }
    i = i.wrapping_add(1 as (usize));
  }
  if counts[2usize] < 500usize {
    max_utf8 = 1usize;
  }
  if counts[1usize].wrapping_add(counts[2usize]) < 25usize {
    max_utf8 = 0usize;
  }
  max_utf8
}

fn EstimateBitCostsForLiteralsUTF8(pos: usize,
                                   len: usize,
                                   mask: usize,
                                   data: &[u8],
                                   cost: &mut [f32]) {
  let max_utf8: usize = DecideMultiByteStatsLevel(pos, len, mask, data);
  let mut histogram: [[usize; 256]; 3] = [[0; 256]; 3];
  let window_half: usize = 495usize;
  let in_window: usize = brotli_min_size_t(window_half, len);
  let mut in_window_utf8: [usize; 3] = [0usize, 0usize, 0usize];
  let mut i: usize;
  {
    let mut last_c: usize = 0usize;
    let mut utf8_pos: usize = 0usize;
    i = 0usize;
    while i < in_window {
      {
        let c: usize = data[((pos.wrapping_add(i) & mask) as (usize))] as (usize);
        {
          let _rhs = 1;
          let _lhs = &mut histogram[utf8_pos][c];
          *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
        }
        {
          let _rhs = 1;
          let _lhs = &mut in_window_utf8[utf8_pos];
          *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
        }
        utf8_pos = UTF8Position(last_c, c, max_utf8);
        last_c = c;
      }
      i = i.wrapping_add(1 as (usize));
    }
  }
  i = 0usize;
  while i < len {
    {
      if i >= window_half {
        let c: usize = (if i < window_half.wrapping_add(1usize) {
                          0i32
                        } else {
                          data[((pos.wrapping_add(i).wrapping_sub(window_half).wrapping_sub(1usize) &
                            mask) as (usize))] as (i32)
                        }) as (usize);
        let last_c: usize =
          (if i < window_half.wrapping_add(2usize) {
             0i32
           } else {
             data[((pos.wrapping_add(i).wrapping_sub(window_half).wrapping_sub(2usize) &
               mask) as (usize))] as (i32)
           }) as (usize);
        let utf8_pos2: usize = UTF8Position(last_c, c, max_utf8);
        {
          let _rhs = 1;
          let _lhs = &mut histogram[utf8_pos2][data[((pos.wrapping_add(i).wrapping_sub(window_half) & mask) as
                           (usize))] as (usize)];
          *_lhs = (*_lhs).wrapping_sub(_rhs as (usize));
        }
        {
          let _rhs = 1;
          let _lhs = &mut in_window_utf8[utf8_pos2];
          *_lhs = (*_lhs).wrapping_sub(_rhs as (usize));
        }
      }
      if i.wrapping_add(window_half) < len {
        let c: usize = data[((pos.wrapping_add(i)
            .wrapping_add(window_half)
            .wrapping_sub(1usize) & mask) as (usize))] as (usize);
        let last_c: usize = data[((pos.wrapping_add(i)
            .wrapping_add(window_half)
            .wrapping_sub(2usize) & mask) as (usize))] as (usize);
        let utf8_pos2: usize = UTF8Position(last_c, c, max_utf8);
        {
          let _rhs = 1;
          let _lhs = &mut histogram[utf8_pos2][data[((pos.wrapping_add(i).wrapping_add(window_half) & mask) as
                           (usize))] as (usize)];
          *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
        }
        {
          let _rhs = 1;
          let _lhs = &mut in_window_utf8[utf8_pos2];
          *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
        }
      }
      {
        let c: usize = (if i < 1usize {
                          0i32
                        } else {
                          data[((pos.wrapping_add(i).wrapping_sub(1usize) & mask) as (usize))] as
                          (i32)
                        }) as (usize);
        let last_c: usize = (if i < 2usize {
                               0i32
                             } else {
                               data[((pos.wrapping_add(i).wrapping_sub(2usize) & mask) as
                                (usize))] as (i32)
                             }) as (usize);
        let utf8_pos: usize = UTF8Position(last_c, c, max_utf8);
        let masked_pos: usize = pos.wrapping_add(i) & mask;
        let mut histo: usize = histogram[utf8_pos][data[(masked_pos as (usize))] as (usize)];
        let mut lit_cost: super::util::floatX;
        if histo == 0usize {
          histo = 1usize;
        }
        lit_cost = FastLog2(in_window_utf8[utf8_pos] as u64) - FastLog2(histo as u64);
        lit_cost = lit_cost + 0.02905 as super::util::floatX;
        if lit_cost < 1.0 as super::util::floatX {
          lit_cost = lit_cost * 0.5 as super::util::floatX;
          lit_cost = lit_cost + 0.5 as super::util::floatX;
        }
        if i < 2000usize {
          lit_cost = lit_cost +
                     (0.7 as super::util::floatX - (2000usize).wrapping_sub(i) as (super::util::floatX) / 2000.0 as super::util::floatX * 0.35 as super::util::floatX);
        }
        cost[(i as (usize))] = lit_cost as (f32);
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
}

pub fn BrotliEstimateBitCostsForLiterals(pos: usize,
                                         len: usize,
                                         mask: usize,
                                         data: &[u8],
                                         cost: &mut [f32]) {
  if BrotliIsMostlyUTF8(data, pos, mask, len, kMinUTF8Ratio) != 0 {
    EstimateBitCostsForLiteralsUTF8(pos, len, mask, data, cost);
  } else {
    let mut histogram: [usize; 256] = [0; 256];

    let window_half: usize = 2000usize;
    let mut in_window: usize = brotli_min_size_t(window_half, len);
    let mut i: usize;
    i = 0usize;
    while i < in_window {
      {
        let _rhs = 1;
        let _lhs = &mut histogram[data[((pos.wrapping_add(i) & mask) as (usize))] as (usize)];
        *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
      }
      i = i.wrapping_add(1 as (usize));
    }
    i = 0usize;
    while i < len {
      {
        let mut histo: usize;
        if i >= window_half {
          {
            let _rhs = 1;
            let _lhs = &mut histogram[data[((pos.wrapping_add(i).wrapping_sub(window_half) &
                              mask) as (usize))] as (usize)];
            *_lhs = (*_lhs).wrapping_sub(_rhs as (usize));
          }
          in_window = in_window.wrapping_sub(1 as (usize));
        }
        if i.wrapping_add(window_half) < len {
          {
            let _rhs = 1;
            let _lhs = &mut histogram[data[((pos.wrapping_add(i).wrapping_add(window_half) &
                              mask) as (usize))] as (usize)];
            *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
          }
          in_window = in_window.wrapping_add(1 as (usize));
        }
        histo = histogram[data[((pos.wrapping_add(i) & mask) as (usize))] as (usize)];
        if histo == 0usize {
          histo = 1usize;
        }
        {
          let mut lit_cost: super::util::floatX = FastLog2(in_window as u64) - FastLog2(histo as u64);
          lit_cost = lit_cost + 0.029 as super::util::floatX;
          if lit_cost < 1.0 as super::util::floatX {
            lit_cost = lit_cost * 0.5 as super::util::floatX;
            lit_cost = lit_cost + 0.5 as super::util::floatX;
          }
          cost[(i as (usize))] = lit_cost as (f32);
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
  }
}
