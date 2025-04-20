use std::{
  collections::VecDeque,
  ops::{Deref, DerefMut},
};

use crate::Buffer;


#[derive(Debug, Clone)]
pub struct BufferManager {
  buffers: VecDeque<Buffer>,
  current_idx: usize,
}
impl BufferManager {
  pub fn new() -> Self {
    return Self {
      buffers: VecDeque::new(),
      current_idx: usize::MAX,
    };
  }
  pub fn set_buffer_idx(&mut self, idx: usize) {
    if !(idx > self.current_idx || idx < self.current_idx) {
      return;
    };
    self.current_idx = idx;
  }
}
impl Deref for BufferManager {
  type Target = VecDeque<Buffer>;

  fn deref(&self) -> &Self::Target {
    return &self.buffers;
  }
}
impl DerefMut for BufferManager {
  fn deref_mut(&mut self) -> &mut Self::Target {
    return &mut self.buffers;
  }
}
