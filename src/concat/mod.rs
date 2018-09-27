use BrotliResult;

#[derive(Clone,Copy)]
struct NewStreamData {
    bytes_so_far: [u8;4],
    num_bytes_read: u8,
}
impl NewStreamData {
    pub fn new() -> NewStreamData{
        NewStreamData{
            bytes_so_far:[0,0,0,0],
            num_bytes_read:0,
        }
    }
    fn sufficient(&self) -> bool {
        if self.num_bytes_read == 3 && (127&self.bytes_so_far[0]) != 17 {
            return true;
        }
        self.num_bytes_read == 4
    }
}

// eat your vegetables
pub struct BrocatliState {
    last_bytes: [u8; 2],
    last_bytes_len: u8,
    new_stream_pending: Option<NewStreamData>,
    // need to make sure that window sizes stay similar or get smaller
    window_size: u8,
}

impl BrocatliState {
   pub fn new() -> BrocatliState {
      BrocatliState {
         last_bytes: [0,0],
         last_bytes_len: 0,
         new_stream_pending: None,
         window_size:0,
      }
   }
   pub fn new_brotli_file(&mut self) {
       self.new_stream_pending = Some(NewStreamData::new());
   }
    fn flush_previous_stream(&self, out_bytes: &mut [u8], out_offset: &mut usize) -> BrotliResult {
        //FIXME
        BrotliResult::ResultSuccess
    }
    pub fn stream(&mut self, in_bytes: &[u8], in_offset: &mut usize, out_bytes: &mut [u8], out_offset: &mut usize) -> BrotliResult {
        if let BrotliResult::NeedsMoreOutput = self.flush_previous_stream(out_bytes, out_offset) {
            return BrotliResult::NeedsMoreOutput
        }
        if let Some(new_stream_pending) = self.new_stream_pending {
            
        }
        BrotliResult::ResultSuccess
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

