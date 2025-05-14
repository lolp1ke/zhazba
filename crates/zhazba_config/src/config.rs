use std::{collections::HashMap, ops::Deref, sync::Arc};

use parking_lot::RwLock;
use zhazba_action::KeyAction;


#[derive(Clone, Debug)]
pub struct Config(Arc<RwLock<ConfigInner>>);
impl Deref for Config {
  type Target = Arc<RwLock<ConfigInner>>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}
impl Default for Config {
  fn default() -> Self {
    Self(Arc::new(RwLock::new(ConfigInner {
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
