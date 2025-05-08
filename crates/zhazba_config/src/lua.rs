use zhazba_action::KeyAction;
use zhazba_lua::{lua_method, lua_userdata};

use crate::Config;


#[lua_userdata]
impl Config {
  #[lua_method]
  pub fn add_keymap(
    &self,
    key_code: String,
    mode: String,
    ka: KeyAction,
  ) -> Option<KeyAction> {
    return self.borrow_mut().keymaps.insert(
      (
        key_code.to_lowercase(),
        mode.chars().last().unwrap_or_else(|| '\0'),
      ),
      ka,
    );
  }
  #[lua_method]
  pub fn add_command(
    &self,
    key_code: String,
    ka: KeyAction,
  ) -> Option<KeyAction> {
    return self
      .borrow_mut()
      .commands
      .insert(key_code.to_lowercase(), ka);
  }
}
