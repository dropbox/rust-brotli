use super::utf8_util::BrotliIsMostlyUTF8;
use super::util::{brotli_max_uint32_t, FastLog2};

static kMinUTF8Ratio: f64 = 0.75f64;

fn brotli_min_size_t(mut a: usize, mut b: usize) -> usize {
  if a < b { a } else { b }
}

fn UTF8Position(mut last: usize, mut c: usize, mut clamp: usize) -> usize {
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

fn DecideMultiByteStatsLevel(mut pos: usize,
                             mut len: usize,
                             mut mask: usize,
                             mut data: &[u8])
                             -> usize {
  let mut counts: [usize; 3] = [0i32 as (usize), 0i32 as (usize), 0i32 as (usize)];
  let mut max_utf8: usize = 1i32 as (usize);
  let mut last_c: usize = 0i32 as (usize);
  let mut i: usize;
  i = 0i32 as (usize);
  'loop1: loop {
    if i < len {
      let mut c: usize = data[(pos.wrapping_add(i) & mask) as (usize)] as (usize);
      {
        let _rhs = 1;
        let _lhs = &mut counts[UTF8Position(last_c, c, 2i32 as (usize))];
        *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
      }
      last_c = c;
      i = i.wrapping_add(1 as (usize));
      continue 'loop1;
    } else {
      break 'loop1;
    }
  }
  if counts[2i32 as (usize)] < 500i32 as (usize) {
    max_utf8 = 1i32 as (usize);
  }
  if counts[1i32 as (usize)].wrapping_add(counts[2i32 as (usize)]) < 25i32 as (usize) {
    max_utf8 = 0i32 as (usize);
  }
  max_utf8
}

fn EstimateBitCostsForLiteralsUTF8(mut pos: usize,
                                   mut len: usize,
                                   mut mask: usize,
                                   mut data: &[u8],
                                   mut cost: &mut [f32]) {
  let max_utf8: usize = DecideMultiByteStatsLevel(pos, len, mask, data);
  let mut histogram: [[usize; 256]; 3] = [[0; 256]; 3];
  let mut window_half: usize = 495i32 as (usize);
  let mut in_window: usize = brotli_min_size_t(window_half, len);
  let mut in_window_utf8: [usize; 3] = [0i32 as (usize), 0i32 as (usize), 0i32 as (usize)];
  let mut i: usize;
  let mut last_c: usize = 0i32 as (usize);
  let mut utf8_pos: usize = 0i32 as (usize);
  i = 0i32 as (usize);
  'loop1: loop {
    if i < in_window {
      let mut c: usize = data[(pos.wrapping_add(i) & mask) as (usize)] as (usize);
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
      i = i.wrapping_add(1 as (usize));
      continue 'loop1;
    } else {
      break 'loop1;
    }
  }
  i = 0i32 as (usize);
  'loop3: loop {
    if i < len {
      if i >= window_half {
        let mut c: usize = (if i < window_half.wrapping_add(1i32 as (usize)) {
                              0i32
                            } else {
                              data[(pos.wrapping_add(i).wrapping_sub(window_half).wrapping_sub(1i32 as
                                                                                          (usize)) &
                               mask) as (usize)] as (i32)
                            }) as (usize);
        let mut last_c: usize =
          (if i < window_half.wrapping_add(2i32 as (usize)) {
             0i32
           } else {
             data[(pos.wrapping_add(i).wrapping_sub(window_half).wrapping_sub(2i32 as (usize)) &
              mask) as (usize)] as (i32)
           }) as (usize);
        let mut utf8_pos2: usize = UTF8Position(last_c, c, max_utf8);
        {
          let _rhs = 1;
          let _lhs = &mut histogram[utf8_pos2][data[(pos.wrapping_add(i).wrapping_sub(window_half) & mask) as
                          (usize)] as (usize)];
          *_lhs = (*_lhs).wrapping_sub(_rhs as (usize));
        }
        {
          let _rhs = 1;
          let _lhs = &mut in_window_utf8[utf8_pos2];
          *_lhs = (*_lhs).wrapping_sub(_rhs as (usize));
        }
      }
      if i.wrapping_add(window_half) < len {
        let mut c: usize = data[(pos.wrapping_add(i)
           .wrapping_add(window_half)
           .wrapping_sub(1i32 as (usize)) & mask) as (usize)] as (usize);
        let mut last_c: usize = data[(pos.wrapping_add(i).wrapping_add(window_half).wrapping_sub(2i32 as (usize)) & mask) as
        (usize)] as (usize);
        let mut utf8_pos2: usize = UTF8Position(last_c, c, max_utf8);
        {
          let _rhs = 1;
          let _lhs = &mut histogram[utf8_pos2][data[(pos.wrapping_add(i).wrapping_add(window_half) & mask) as
                          (usize)] as (usize)];
          *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
        }
        {
          let _rhs = 1;
          let _lhs = &mut in_window_utf8[utf8_pos2];
          *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
        }
      }
      let mut c: usize = (if i < 1i32 as (usize) {
                            0i32
                          } else {
                            data[(pos.wrapping_add(i).wrapping_sub(1i32 as (usize)) & mask) as
                            (usize)] as (i32)
                          }) as (usize);
      let mut last_c: usize = (if i < 2i32 as (usize) {
                                 0i32
                               } else {
                                 data[(pos.wrapping_add(i).wrapping_sub(2i32 as (usize)) &
                                  mask) as (usize)] as (i32)
                               }) as (usize);
      let mut utf8_pos: usize = UTF8Position(last_c, c, max_utf8);
      let mut masked_pos: usize = pos.wrapping_add(i) & mask;
      let mut histo: usize = histogram[utf8_pos][data[masked_pos as (usize)] as (usize)];
      let mut lit_cost: f64;
      if histo == 0i32 as (usize) {
        histo = 1i32 as (usize);
      }
      lit_cost = FastLog2(in_window_utf8[utf8_pos]) - FastLog2(histo);
      lit_cost = lit_cost + 0.02905f64;
      if lit_cost < 1.0f64 {
        lit_cost = lit_cost * 0.5f64;
        lit_cost = lit_cost + 0.5f64;
      }
      if i < 2000i32 as (usize) {
        lit_cost = lit_cost +
                   (0.7f64 - (2000i32 as (usize)).wrapping_sub(i) as (f64) / 2000.0f64 * 0.35f64);
      }
      cost[i as (usize)] = lit_cost as (f32);
      i = i.wrapping_add(1 as (usize));
      continue 'loop3;
    } else {
      break 'loop3;
    }
  }
}

pub fn BrotliEstimateBitCostsForLiterals(mut pos: usize,
                                         mut len: usize,
                                         mut mask: usize,
                                         mut data: &[u8],
                                         mut cost: &mut [f32]) {
  if BrotliIsMostlyUTF8(data, pos, mask, len, kMinUTF8Ratio) != 0 {
    EstimateBitCostsForLiteralsUTF8(pos, len, mask, data, cost);
  } else {
    let mut histogram: [usize; 256] = [0; 256];
    let mut window_half: usize = 2000i32 as (usize);
    let mut in_window: usize = brotli_min_size_t(window_half, len);
    let mut i: usize;
    i = 0i32 as (usize);
    'loop2: loop {
      if i < in_window {
        {
          let _rhs = 1;
          let _lhs = &mut histogram[data[(pos.wrapping_add(i) & mask) as (usize)] as (usize)];
          *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
        }
        i = i.wrapping_add(1 as (usize));
        continue 'loop2;
      } else {
        break 'loop2;
      }
    }
    i = 0i32 as (usize);
    'loop4: loop {
      if i < len {
        let mut histo: usize;
        if i >= window_half {
          {
            let _rhs = 1;
            let _lhs = &mut histogram[data[(pos.wrapping_add(i).wrapping_sub(window_half) &
                             mask) as (usize)] as (usize)];
            *_lhs = (*_lhs).wrapping_sub(_rhs as (usize));
          }
          in_window = in_window.wrapping_sub(1 as (usize));
        }
        if i.wrapping_add(window_half) < len {
          {
            let _rhs = 1;
            let _lhs = &mut histogram[data[(pos.wrapping_add(i).wrapping_add(window_half) &
                             mask) as (usize)] as (usize)];
            *_lhs = (*_lhs).wrapping_add(_rhs as (usize));
          }
          in_window = in_window.wrapping_add(1 as (usize));
        }
        histo = histogram[data[(pos.wrapping_add(i) & mask) as (usize)] as (usize)];
        if histo == 0i32 as (usize) {
          histo = 1i32 as (usize);
        }
        let mut lit_cost: f64 = FastLog2(in_window) - FastLog2(histo);
        lit_cost = lit_cost + 0.029f64;
        if lit_cost < 1.0f64 {
          lit_cost = lit_cost * 0.5f64;
          lit_cost = lit_cost + 0.5f64;
        }
        cost[i as (usize)] = lit_cost as (f32);
        i = i.wrapping_add(1 as (usize));
        continue 'loop4;
      } else {
        break 'loop4;
      }
    }
  }
}
