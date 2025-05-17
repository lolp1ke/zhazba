use std::sync::Arc;

use zhazba_config::Config;
use zhazba_lua::{LuaFunction, lua_method, lua_userdata};
use zhazba_render::TermRender;

use crate::Editor;


#[lua_userdata]
impl Editor {
  #[lua_method]
  fn config(&self) -> Config {
    return self.read_arc().config.clone();
  }
  #[lua_method]
  fn mode(&self) -> String {
    return self.read_arc().mode.to_string();
  }
  #[lua_method]
  fn render(&self) -> TermRender {
    return self.read_arc().render.clone();
  }
  #[lua_method]
  fn create_register(&self, register: String) {
    self
      .write_arc()
      .register_map
      // NOTE: Maybe use String::with_capacity(...);
      .insert(Arc::from(register), String::new());
  }
  #[lua_method]
  fn read_register(&self, register: String) -> String {
    return self
      .read_arc()
      .register_map
      .get(&*register)
      .cloned()
      .unwrap_or_default();
  }
  #[lua_method]
  fn current_register(&self) -> String {
    return self
      .read_arc()
      .current_register
      .as_ref()
      .cloned()
      .unwrap_or_default()
      .to_string();
  }


  #[lua_method]
  fn event_callback(&self, event_name: String, lua_cb: LuaFunction) {
    self
      .write_arc()
      .event_callbacks
      .entry(Arc::from(event_name))
      .or_insert_with(|| Vec::new())
      .push(lua_cb);
  }
}
