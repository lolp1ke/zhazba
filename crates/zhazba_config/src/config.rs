use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use zhazba_action::KeyAction;
use zhazba_lua::*;
// use zhazba_lua::LuaUserData;


#[derive(Debug)]
pub struct Config {
  inner: Rc<RefCell<ConfigInner>>,
}
impl Default for Config {
  fn default() -> Self {
    Self {
      inner: Rc::new(RefCell::new(ConfigInner {
        theme: String::new(),

        keymaps: HashMap::new(),

        tab_width: 2,
        use_tabs: false,
      })),
    }
  }
}
impl Deref for Config {
  type Target = Rc<RefCell<ConfigInner>>;

  fn deref(&self) -> &Self::Target {
    return &self.inner;
  }
}
#[lua_userdata]
impl Config {
  #[lua_method]
  pub fn add_keymap(
    &self,
    key: (String, String),
    key_action: KeyAction,
  ) -> Option<KeyAction> {
    let (key_code, mode) = key;


    return self.borrow_mut().keymaps.insert(
      (
        key_code.to_lowercase(),
        mode.chars().last().unwrap_or_else(|| '\0'),
      ),
      key_action,
    );
  }
}
#[derive(Debug)]
pub struct ConfigInner {
  theme: String,

  keymaps: HashMap<(String, char), KeyAction>,

  tab_width: u16,
  use_tabs: bool,
}
