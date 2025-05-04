use crate::BufferInner;


impl BufferInner {
  pub(crate) fn pos_to_idx(&self, (cx, cy): (usize, usize)) -> usize {
    if cy >= self.content.len_lines() - 1 {
      return self.content.len_bytes();
    };

    let line_start_idx = self.content.line_to_byte(cy);
    let line = self.content.line(cy);
    let x = std::cmp::min(cx, line.len_bytes());
    return line_start_idx + line.char_to_byte(x);
  }
}
