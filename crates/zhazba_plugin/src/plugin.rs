use std::{ops::Deref, path::PathBuf, sync::Arc};

use anyhow::Result;
use parking_lot::RwLock;
use tracing::debug;

use crate::{Registry, Runtime};


#[derive(Clone, Debug)]
pub struct Plugin(Arc<RwLock<PluginInner>>);
impl Plugin {
  pub fn new() -> Self {
    return Self(Arc::new(RwLock::new(PluginInner {
      runtime: Runtime::new(),
      registry: Registry::default(),
    })));
  }
  pub fn raw(raw: Arc<RwLock<PluginInner>>) -> Self {
    return Self(raw);
  }
}
impl Deref for Plugin {
  type Target = Arc<RwLock<PluginInner>>;

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
  pub fn new() -> Self {
    return Self {
      runtime: Runtime::new(),
      registry: Registry::default(),
    };
  }
  pub async fn init(&mut self) -> Result<()> {
    self.registry.load()?;
    // self.runtime.init()?;


    for (_, path) in self.registry.plugins.iter() {
      match std::fs::read_to_string(PathBuf::from(&**path)) {
        Ok(code) => {
          self.runtime.load_module(&code).await?;
        }
        Err(_) => todo!(),
      };
    }


    debug!("Plugin manager initted");
    return Ok(());
  }
}
