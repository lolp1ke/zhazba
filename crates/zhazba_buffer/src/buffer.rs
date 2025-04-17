use std::{io, path::PathBuf};

use ropey::Rope;
use tracing::error;


#[derive(Debug)]
pub struct Buffer {
  file: PathBuf,
  changed: bool,

  content: Rope,

  pos: (usize, usize),
  v_pos: (usize, usize),
}
impl Buffer {
  pub fn load_from_file(file: PathBuf) -> Self {
    let content = match std::fs::read_to_string(&file) {
      Ok(string) => string,
      Err(err) => {
        error!("{}", err);
        todo!();
      }
    };
    let content = Rope::from_str(&content);

    return Self {
      file,
      changed: false,

      content,

      pos: (0, 0),
      v_pos: (0, 0),
    };
  }

  pub fn save(&mut self) -> io::Result<()> {
    let mut content = self.content.to_string();
    if let Some(ch) = content.chars().last() {
      if ch == '\n' {
        content.push('\n');
      };
    };
    std::fs::write(&self.file, content)?;

    return Ok(());
  }
}
