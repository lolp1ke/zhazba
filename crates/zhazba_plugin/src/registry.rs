use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

// #[derive(Debug, Default)]
// pub struct Registry(Rc<RefCell<RegistryInner>>);
// impl Deref for Registry {
// type Target = Rc<RefCell<RegistryInner>>;
//
// fn deref(&self) -> &Self::Target {
// return &self.0;
// }
// }

#[derive(Debug, Default)]
pub struct Registry {
  plugins: HashMap<Rc<str>, Rc<str>>,
}
impl Registry {
  pub(crate) fn add_plugin(
    &mut self,
    name: &str,
    path: &str,
  ) -> Option<Rc<str>> {
    return self.plugins.insert(Rc::from(name), Rc::from(path));
  }
}
