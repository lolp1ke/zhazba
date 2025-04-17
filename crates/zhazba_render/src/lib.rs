mod term;

pub use term::*;

use std::fmt::Debug;


pub trait Render: Debug + Drop {
  fn draw_frame(&mut self, contnet: String);
}
