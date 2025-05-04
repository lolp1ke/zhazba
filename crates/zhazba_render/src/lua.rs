use zhazba_lua::MetaMethod;

use crate::UiNode;


impl zhazba_lua::UserData for UiNode {
  fn add_methods<M: zhazba_lua::UserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(MetaMethod::ToString, |_, this: &Self, ()| {
      return Ok(format!("{:?}", this));
    });

    // methods.add_method("make_window", |_, _, ()| {
    // return Ok(Self::make_block());
    // });
  }
}
