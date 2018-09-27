use core;
use BrotliResult;

const NUM_STREAM_HEADER_BYTES: usize = 5;

#[derive(Clone,Copy)]
struct NewStreamData {
    bytes_so_far: [u8;NUM_STREAM_HEADER_BYTES],
    num_bytes_read: u8,
    num_bytes_written: Option<u8>,
}
impl NewStreamData {
    pub fn new() -> NewStreamData{
        NewStreamData{
          bytes_so_far:[0,0,0,0,0],
          num_bytes_read:0,
          num_bytes_written:None,
        }
    }
    fn sufficient(&self) -> bool {
        if self.num_bytes_read == 4 && (127&self.bytes_so_far[0]) != 17 {
            return true;
        }
        self.num_bytes_read == 5
    }
}

fn parse_window_size(bytes_so_far:&[u8]) -> Result<(u8, usize), ()> {  // returns window_size and offset in stream in bits
  if bytes_so_far[0] & 1 == 0 {
    return Ok((16, 1));
  }
  match bytes_so_far[0] & 15 {
    0x3 => return Ok((18, 4)),
    0x5 => return Ok((19, 4)),
    0x7 => return Ok((20, 4)),
    0x9 => return Ok((21, 4)),
    0xb => return Ok((22, 4)),
    0xd => return Ok((23, 4)),
    0xf => return Ok((24, 4)),
    _ => match bytes_so_far[0] & 127 {
      0x71 => return Ok((15, 7)),
      0x61 => return Ok((14, 7)),
      0x51 => return Ok((13, 7)),
      0x41 => return Ok((12, 7)),
      0x31 => return Ok((11, 7)),
      0x21 => return Ok((10, 7)),
      _ => {},
    }
  }
  if (bytes_so_far[0] & 0x80) != 0 {
    return Err(());
  }
  let ret  = bytes_so_far[1] & 0x3f;
  if ret < 10 || ret > 30 {
    return Err(());
  }
  Ok((ret, 14))
}

fn detect_varlen_offset(bytes_so_far:&[u8]) -> Result<(usize), ()> {  // returns offfset in bits
  let (_, mut offset) = parse_window_size(bytes_so_far)?;
  let mut bytes = 0;
  for (index, item) in bytes_so_far.iter().enumerate() {
    bytes |= *item << (index * 8);
  }
  bytes >>= offset;
  offset += 1;
  if (bytes & 1) != 0 { // ISLAST
    bytes >>= 1;
    offset += 1;
    if (bytes & 1) != 0 { // ISLASTEMPTY
      return Ok(offset);
    }
  }
  bytes >>= 1;
  let mut mnibbles = bytes & 3;
  bytes >>= 2;
  offset += 2;
  if mnibbles == 3 { // metadata block
    if (bytes & 1) != 0 {
      return Err(()); // reserved, must be zero
    }
    bytes >>= 1;
    offset += 1;
    let mskipbytes = bytes & 3;
    offset += 3;
    offset += usize::from(mskipbytes) * 8; // next item is byte aligned
    return Ok(offset);
  }
  mnibbles += 4;
  offset += usize::from(mnibbles) * 4;
  bytes >>= mnibbles * 4;
  offset += 1;
  if (bytes & 1) == 0 { // not UNCOMPRESSED
    Err(()) // not valid bitstream for concatenation
  } else { // UNCOMPRESSED: now things are aligend
    Ok(offset)
  }
}

// eat your vegetables
pub struct BroCatliState {
  last_bytes: [u8; 2],
  last_bytes_len: u8,
  last_byte_sanitized: bool,
  last_byte_bit_offset: u8,
  new_stream_pending: Option<NewStreamData>,
  // need to make sure that window sizes stay similar or get smaller
  window_size: u8,
}

impl BroCatliState {
  pub fn new() -> BroCatliState {
    BroCatliState {
      last_bytes: [0,0],
      last_bytes_len: 0,
      last_byte_bit_offset: 0,
      last_byte_sanitized: false,
      new_stream_pending: None,
      window_size:0,
    }
  }
  pub fn new_brotli_file(&mut self) {
    self.new_stream_pending = Some(NewStreamData::new());
  }
  fn flush_previous_stream(&mut self, out_bytes: &mut [u8], out_offset: &mut usize) -> BrotliResult {
    if !self.last_byte_sanitized { // if the previous stream hasn't had the last metablock (bit 1,1) sanitized
      if self.last_bytes_len == 0 { // first stream or otherwise sanitized
        self.last_byte_sanitized = true;
        return BrotliResult::ResultSuccess;
      }
      // create a 16 bit integer with the last 2 bytes of data
      let mut last_bytes = self.last_bytes[0] as u16 + ((self.last_bytes[1] as u16) << 8);
      let max = self.last_bytes_len * 8;
      let mut index = max - 1;
      for i in 0..max {
        index = max - 1 - i;
        if ((1<<index) & last_bytes) == 0 {
          break; // find the highest set bit
        }
      }
      if index < 2 { // if the bit is too low, return failure, since both bits could not possibly have been set
        return BrotliResult::ResultFailure
      }
      if (last_bytes >> (index - 2)) != 3 { // last two bits need to be set for the final metablock
        return BrotliResult::ResultFailure
      }
      index -= 2; // discard the final two bits
      last_bytes &= (1 << (index + 1)) - 1; // mask them out
      self.last_bytes[0] = last_bytes as u8 & 0xff; // reset the last_bytes pair
      self.last_bytes[1] = (last_bytes >> 8) as u8 & 0xff;
      if index >= 8 { // if both bits and one useful bit were in the second block, then write that
        out_bytes[*out_offset] = self.last_bytes[0];
        self.last_bytes[0] = self.last_bytes[1];
        index -= 8;
      }
      self.last_byte_bit_offset = index;
      assert!(index < 8);
      self.last_byte_sanitized = true;
    }
    BrotliResult::ResultSuccess
  }

  fn shift_and_check_new_stream_header(&mut self, mut new_stream_pending: NewStreamData, out_bytes: &mut [u8], out_offset: &mut usize) -> BrotliResult {
    if new_stream_pending.num_bytes_written.is_none() {
      let (window_size, window_offset) = if let Ok(results) = parse_window_size(
        &new_stream_pending.bytes_so_far[..usize::from(new_stream_pending.num_bytes_read)],
      ) {
        results
      } else {
        return BrotliResult::ResultFailure;
      };
      if self.window_size == 0 { // parse window size and just copy everything
        self.window_size = window_size;
        assert_eq!(self.last_byte_bit_offset, 0); // we are first stream
        out_bytes[*out_offset] = new_stream_pending.bytes_so_far[0];
        new_stream_pending.num_bytes_written = Some(1);
        *out_offset += 1;
      } else {
        if window_size > self.window_size {
          return BrotliResult::ResultFailure;
        }
        let mut realigned_header:[u8;NUM_STREAM_HEADER_BYTES + 1] = [self.last_bytes[0],
                                                                    0,0,0,0,0,
        ];
        let varlen_offset = if let Ok(voffset) = detect_varlen_offset(
          &new_stream_pending.bytes_so_far[..usize::from(new_stream_pending.num_bytes_read)],
        ) {
          voffset
        } else {
          return BrotliResult::ResultFailure;
        };
        let mut bytes_so_far = 0u64;
        for index in 0..usize::from(new_stream_pending.num_bytes_read) {
          bytes_so_far |= u64::from(new_stream_pending.bytes_so_far[index] << (index * 8));
        }
        bytes_so_far >>= window_offset; // mask out the window size
        bytes_so_far &= (1u64 << (varlen_offset - window_offset)) - 1;
        let var_len_bytes = ((usize::from(varlen_offset - window_offset) + 7) / 8);
        for byte_index in 0..var_len_bytes {
          let cur_byte = (bytes_so_far >> (byte_index *8)) as u8;
          realigned_header[byte_index] |= (cur_byte & ((1 << (8 - self.last_byte_bit_offset)) - 1)) << self.last_byte_bit_offset;
          realigned_header[byte_index + 1] = cur_byte >> (8 - self.last_byte_bit_offset);
        }
        let whole_byte_destination = var_len_bytes + (self.last_byte_bit_offset != 0) as usize;
        let whole_byte_source = (varlen_offset + 7) / 8;
        let num_whole_bytes_to_copy = usize::from(new_stream_pending.num_bytes_read) - whole_byte_source;
        for aligned_index in 0..num_whole_bytes_to_copy {
          realigned_header[whole_byte_destination] = new_stream_pending.bytes_so_far[whole_byte_source];
        }
        out_bytes[*out_offset] = realigned_header[0];
        *out_offset += 1;
        new_stream_pending.num_bytes_read = (whole_byte_destination + num_whole_bytes_to_copy) as u8;
        new_stream_pending.num_bytes_written = Some(0);
        new_stream_pending.bytes_so_far.clone_from_slice(&realigned_header[1..]);
      }
    } else {
      assert!(self.window_size != 0);
    }
    let to_copy = core::cmp::min(out_bytes.len() - *out_offset,
                                 usize::from(new_stream_pending.num_bytes_read - new_stream_pending.num_bytes_written.unwrap()));
    out_bytes.split_at_mut(*out_offset).1.split_at_mut(to_copy).0.clone_from_slice(
      &new_stream_pending.bytes_so_far[usize::from(new_stream_pending.num_bytes_written.unwrap())..usize::from(new_stream_pending.num_bytes_read)]);
    new_stream_pending.num_bytes_written = Some((new_stream_pending.num_bytes_written.unwrap() + to_copy as u8));
    if new_stream_pending.num_bytes_written.unwrap() != new_stream_pending.num_bytes_read {
      self.new_stream_pending = Some(new_stream_pending);
      return BrotliResult::NeedsMoreOutput;
    }
    self.new_stream_pending = None;
    self.last_byte_sanitized = false;
    self.last_byte_bit_offset = 0;
    self.last_bytes_len = 0;
    self.last_bytes = [0,0];
    BrotliResult::ResultSuccess
  }
  pub fn stream(&mut self, mut in_bytes: &[u8], in_offset: &mut usize, out_bytes: &mut [u8], out_offset: &mut usize) -> BrotliResult {
    if let Some(mut new_stream_pending) = self.new_stream_pending.clone() {
      let flush_result = self.flush_previous_stream(out_bytes, out_offset);
      if let BrotliResult::ResultSuccess = flush_result {
        if usize::from(new_stream_pending.num_bytes_read) < new_stream_pending.bytes_so_far.len() {
          {
            let dst = &mut new_stream_pending.bytes_so_far[usize::from(new_stream_pending.num_bytes_read)..];
            let to_copy = core::cmp::min(dst.len(), in_bytes.len() - *in_offset);
            dst.clone_from_slice(in_bytes.split_at(*in_offset).1.split_at(to_copy).0);
            *in_offset += to_copy;
            new_stream_pending.num_bytes_read += to_copy as u8;
          }
          self.new_stream_pending = Some(new_stream_pending); // write back changes
        }
        if !new_stream_pending.sufficient() {
          return BrotliResult::NeedsMoreInput;
        }
        if out_bytes.len() == *out_offset {
          return BrotliResult::NeedsMoreOutput;
        }
        let shift_result = self.shift_and_check_new_stream_header(new_stream_pending, out_bytes, out_offset);
        if let BrotliResult::ResultSuccess = shift_result {
        } else {
          return shift_result;
        }
      } else {
        return flush_result;
      }
      if *out_offset == out_bytes.len() {
        return BrotliResult::NeedsMoreOutput; // need to be able to write at least one byte of data to make progress
      }
    }
    assert!(self.new_stream_pending.is_none());// this should have been handled above
    if self.last_bytes_len != 2 {
      if in_bytes.len() == *in_offset {
        return BrotliResult::NeedsMoreInput;
      }
      self.last_bytes[usize::from(self.last_bytes_len)] = in_bytes[*in_offset];
      *in_offset += 1;
      if self.last_bytes_len != 2 {
        if in_bytes.len() == *in_offset {
          return BrotliResult::NeedsMoreInput;
        }
        self.last_bytes[usize::from(self.last_bytes_len)] = in_bytes[*in_offset];
        *in_offset += 1;
      }
    }
    if in_bytes.len() == *in_offset{
      return BrotliResult::NeedsMoreInput;
    }
    if out_bytes.len() == *out_offset{
      return BrotliResult::NeedsMoreOutput;
    }
    let mut to_copy = core::cmp::min(out_bytes.len() - *out_offset,
                                     in_bytes.len() - *in_offset);
    assert!(to_copy != 0);
    if to_copy == 1 {
      out_bytes[*out_offset] = self.last_bytes[0];
      self.last_bytes[0] = self.last_bytes[1];
      self.last_bytes[1] = in_bytes[*in_offset];
      *in_offset += 1;
      *out_offset += 1;
      if out_bytes.len() < in_bytes.len(){
        return BrotliResult::NeedsMoreOutput;
      } else {
        return BrotliResult::NeedsMoreInput;
      }
    }
    out_bytes.split_at_mut(*out_offset).1.split_at_mut(2).0.clone_from_slice(&self.last_bytes[..]);
    *out_offset += 2;
    let (new_in_offset, last_two) = in_bytes.split_at(in_bytes.len()-2);
    self.last_bytes.clone_from_slice(last_two);
    to_copy -= 2;
    out_bytes.split_at_mut(*out_offset).1.split_at_mut(to_copy).0.clone_from_slice(
      in_bytes.split_at(*in_offset).1.split_at(to_copy).0);
    if out_bytes.len() < in_bytes.len(){
      return BrotliResult::NeedsMoreOutput;
    } else {
      return BrotliResult::NeedsMoreInput;
    }
  }
   pub fn finish(&mut self, out_bytes: &mut [u8], out_offset: &mut usize) -> BrotliResult {
       while self.last_bytes_len != 0 {
           if *out_offset == out_bytes.len() {
               return BrotliResult::NeedsMoreOutput;
           }
           out_bytes[*out_offset] = self.last_bytes[0];
           *out_offset += 1;
           self.last_bytes_len -= 1;
           self.last_bytes[0] = self.last_bytes[1];
       }
       BrotliResult::ResultSuccess
   }
}

