use crate::EditorInner;


impl EditorInner {
  pub(crate) fn insert_into_buffer(
    &mut self,
    pos: (usize, usize),
    content: &str,
  ) {
    self
      .buffer_manager
      .get_buffer_mut()
      .write_arc()
      .insert(pos, content);
  }
}
