use std::rc::Rc;

use zhazba_config::Config;
use zhazba_lua::{lua_method, lua_userdata};
use zhazba_render::TermRender;

use crate::Editor;


#[lua_userdata]
impl Editor {
  #[lua_method]
  fn config(&self) -> Config {
    return self.borrow().config.clone();
  }
  #[lua_method]
  fn render(&self) -> TermRender {
    return self.borrow().render.clone();
  }
  #[lua_method]
  fn create_register(&self, mode: String) {
    self
      .borrow_mut()
      .register_map
      // NOTE: Maybe use String::with_capacity(...);
      .insert(Rc::from(mode), String::new());
  }
}
