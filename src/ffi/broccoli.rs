use core;
use core::slice;
pub use brotli_decompressor::ffi::interface::{
  c_void,
};
use concat::BroCatli;
pub use concat::BroCatliResult;
pub type BroccoliResult = BroCatliResult;
// a tool to concatenate brotli files together

#[repr(C)]
#[no_mangle]
pub struct BroccoliState {
    more_data: *mut c_void,
    current_data: [u8;120],
}

impl Clone for BroccoliState {
  fn clone(&self) -> BroccoliState {
    let mut cd = [0u8; 120];
    cd.clone_from_slice(&self.current_data[..]);
    BroccoliState{
      more_data:self.more_data,
      current_data:cd,
    }
  }
}

impl Copy for BroccoliState{}

impl Default for BroccoliState {
    fn default() -> BroccoliState {
        BroCatli::new().into()
    }
}
impl From<BroCatli> for BroccoliState {
    fn from(data: BroCatli) -> BroccoliState {
        let mut buffer = [0u8; 120];
        data.serialize_to_buffer(&mut buffer[..]).unwrap();
        BroccoliState{
            more_data: core::ptr::null_mut(),
            current_data:buffer,
        }
    }
}
impl Into<BroCatli> for BroccoliState {
    fn into(self) -> BroCatli {
        BroCatli::deserialize_from_buffer(&self.current_data[..]).unwrap()
    }
}

#[no_mangle]
pub extern fn BroccoliCreateInstance() -> BroccoliState {
    BroCatli::new().into()
}
#[no_mangle]
pub extern fn BroccoliCreateInstanceWithWindowSize(window_size: u8) -> BroccoliState {
    BroCatli::new_with_window_size(window_size).into()
}
#[no_mangle]
pub extern fn BroccoliDestroyInstance(_state: BroccoliState) {
}

#[no_mangle]
pub unsafe extern fn BroccoliNewBrotliFile(state: *mut BroccoliState) {
    let mut bro_catli: BroCatli = (*state).into();
    bro_catli.new_brotli_file();
    *state = BroccoliState::from(bro_catli);
}

#[no_mangle]
pub unsafe extern fn BroccoliConcatStream(
  state: *mut BroccoliState,
  available_in: *mut usize,
  input_buf_ptr: *mut*const u8,
  available_out: *mut usize,
  output_buf_ptr: *mut*mut u8) -> BroccoliResult {
  let input_buf = slice::from_raw_parts(*input_buf_ptr, *available_in);
  let output_buf = slice::from_raw_parts_mut(*output_buf_ptr, *available_out);
  let mut input_offset = 0usize;
  let mut output_offset = 0usize;
  let mut bro_catli: BroCatli = (*state).into();
  let ret = bro_catli.stream(input_buf, &mut input_offset, output_buf, &mut output_offset);
  *input_buf_ptr = (*input_buf_ptr).offset(input_offset as isize);
  *output_buf_ptr = (*output_buf_ptr).offset(output_offset as isize);
  *available_in -= input_offset;
  *available_out -= output_offset;
  *state = BroccoliState::from(bro_catli);
  ret
}

#[no_mangle]
pub unsafe extern fn BroccoliConcatFinish(
  state: *mut BroccoliState,
  available_out: *mut usize,
  output_buf_ptr: *mut*mut u8) -> BroCatliResult {
  let output_buf = slice::from_raw_parts_mut(*output_buf_ptr, *available_out);
  let mut output_offset = 0usize;
  let mut bro_catli: BroCatli = (*state).into();
  let ret = bro_catli.finish(output_buf, &mut output_offset);
  *output_buf_ptr = (*output_buf_ptr).offset(output_offset as isize);
  *available_out -= output_offset;
  *state = BroccoliState::from(bro_catli);
  ret
}
