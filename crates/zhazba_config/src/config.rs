use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use zhazba_action::KeyAction;
use zhazba_lua::{lua_method, lua_userdata};


#[derive(Debug, Clone)]
pub struct Config(Rc<RefCell<ConfigInner>>);
impl Deref for Config {
  type Target = Rc<RefCell<ConfigInner>>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}
impl Default for Config {
  fn default() -> Self {
    Self(Rc::new(RefCell::new(ConfigInner {
      theme: String::new(),

      keymaps: HashMap::new(),

      tab_width: 2,
      use_tabs: false,
    })))
  }
}
#[lua_userdata]
impl Config {
  #[lua_method]
  pub fn add_keymap(
    &self,
    key_code: String,
    mode: String,
    key_action: KeyAction,
  ) -> Option<KeyAction> {
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

  // TODO: Replace String with Rc<str>
  pub keymaps: HashMap<(String, char), KeyAction>,

  tab_width: u16,
  use_tabs: bool,
}
impl ConfigInner {}
