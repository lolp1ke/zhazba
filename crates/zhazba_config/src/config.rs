use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use zhazba_action::KeyAction;


#[derive(Clone, Debug)]
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
      leader: ' ',
      commands: HashMap::new(),

      tab_width: 2,
      use_tabs: false,
    })))
  }
}

#[derive(Debug)]
pub struct ConfigInner {
  theme: String,

  // TODO: Replace String with Rc<str>
  pub keymaps: HashMap<(String, char), KeyAction>,
  pub leader: char,
  pub commands: HashMap<String, KeyAction>,

  tab_width: u16,
  use_tabs: bool,
}
impl ConfigInner {}
