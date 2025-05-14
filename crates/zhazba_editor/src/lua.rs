use std::sync::Arc;

use zhazba_config::Config;
use zhazba_lua::{Function, lua_method, lua_userdata};
use zhazba_render::TermRender;

use crate::Editor;


#[lua_userdata]
impl Editor {
  #[lua_method]
  fn config(&self) -> Config {
    return self.write_arc().config.clone();
  }
  #[lua_method]
  fn render(&self) -> TermRender {
    return self.write_arc().render.clone();
  }
  #[lua_method]
  fn create_register(&self, mode: String) {
    self
      .write_arc()
      .register_map
      // NOTE: Maybe use String::with_capacity(...);
      .insert(Arc::from(mode), String::new());
  }

  #[lua_method]
  fn event_callback(&mut self, event_name: String, lua_cb: Function) {
    unsafe {
      self.force_unlock_write_fair();
    };
    if let Some(lua_callbacks) = self
      .write_arc()
      .event_callbacks
      .get_mut(&Arc::from(event_name))
    {
      lua_callbacks.push(lua_cb);
    };
  }
}
