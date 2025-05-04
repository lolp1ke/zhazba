use std::{cell::RefCell, ops::Deref, rc::Rc};

use anyhow::Result;

use crate::{Registry, Runtime};


#[derive(Debug, Clone)]
pub struct Plugin(Rc<RefCell<PluginInner>>);
impl Plugin {
  pub fn new() -> Self {
    return Self(Rc::new(RefCell::new(PluginInner {
      runtime: Runtime::new(),
      registry: Registry::default(),
    })));
  }
}
impl Deref for Plugin {
  type Target = Rc<RefCell<PluginInner>>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}

#[derive(Debug)]
pub struct PluginInner {
  runtime: Runtime,
  registry: Registry,
}
impl PluginInner {
  fn load(&mut self) -> Result<()> {
    let plugins_dir = if env!("ENV") == "debug" {
      ".config/plugins/"
    } else {
      "~/.config/zhazba/plugins/"
    };

    self
      .registry
      .add_plugin("test", &format!("{}{}", plugins_dir, "test.lua"));

    return Ok(());
  }

  pub fn init(&mut self) -> Result<()> {
    self.load()?;

    self.runtime.init()?;
    return Ok(());
  }
}
