use core;
pub const MAX_THREADS: usize = 16;


pub struct FixedQueue<T:Sized>{
  data: [Option<T>;MAX_THREADS],
  size: usize,
  start: usize,
}
impl<T:Sized> Default for FixedQueue<T> {
  fn default() -> Self {
    Self::new()
  }
}
impl<T:Sized> FixedQueue<T> {
  pub fn new() -> Self {
    FixedQueue{
      data:[None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,
      ],
      size:0,
      start:0,
    }
  }
  pub fn can_push(&self) -> bool {
    self.size < self.data.len()
  }
  pub fn size(&self) -> usize {
    self.size
  }
  pub fn push(&mut self, item: T) -> Result<(), ()> {
    if self.size == self.data.len() {
      return Err(());
    }
    let index = (self.start + self.size) % self.data.len();
    self.data[index] = Some(item);
    self.size += 1;
    Ok(())
  }
  pub fn pop(&mut self) -> Option<T> {
    if self.size == 0 {
      return None;
    }
    let ret = core::mem::replace(&mut self.data[self.start], None);
    self.start += 1;
    self.size -= 1;
    ret
  }
}
