use std::{collections::HashMap, fs::DirEntry, path::PathBuf, sync::Arc};

use anyhow::Result;
use tracing::debug;


#[derive(Debug, Default)]
pub struct Registry {
  pub(crate) plugins: HashMap<Arc<str>, Arc<str>>,
}
impl Registry {
  pub(crate) fn new() -> Self {
    let mut registry = Self {
      plugins: HashMap::new(),
    };
    _ = registry.load();

    return registry;
  }

  fn add_plugin(&mut self, name: &str, path: &str) -> Option<Arc<str>> {
    return self.plugins.insert(Arc::from(name), Arc::from(path));
  }

  pub(crate) fn load(&mut self) -> Result<()> {
    let plugins_dir = PathBuf::from(format!(
      "{}.config/zhazba/plugins/",
      if env!("ENV") == "DEBUG" { "./" } else { "~/" },
    ));
    visit_dirs(&plugins_dir, &mut |dir| {
      debug!("Plugin path: {}", dir.path().display());

      self.add_plugin(
        &format!(
          "{}",
          dir
            .path()
            .to_string_lossy()
            .trim_start_matches(&plugins_dir.to_string_lossy().to_string())
            .trim_end_matches(".lua")
            .replace("/", "."),
        ),
        &format!("{}", dir.path().to_string_lossy()),
      );
    })?;


    debug!("Plugins loaded");
    return Ok(());
    fn visit_dirs(dir: &PathBuf, cb: &mut dyn FnMut(&DirEntry)) -> Result<()> {
      if !dir.is_dir() {
        return Ok(());
      };
      for entry in std::fs::read_dir(dir)? {
        let entry: DirEntry = entry?;
        let path: PathBuf = entry.path();
        if path.is_dir() {
          visit_dirs(&path, cb)?;
        } else {
          cb(&entry);
        };
      }


      return Ok(());
    }
  }
}
