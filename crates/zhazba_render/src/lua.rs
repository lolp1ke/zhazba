use std::sync::Arc;

use ratatui::{layout::Constraint, widgets::Paragraph};

use zhazba_lua::{
  FromLua, Lua, LuaError, LuaMetaMethod, LuaResult, LuaUserData,
  LuaUserDataMethods, LuaValue,
};

use crate::{TermRender, UiNode, UiNodeInner};


#[derive(Clone)]
struct LuaWrapper<T>(T);
impl FromLua for LuaWrapper<Constraint> {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    return Ok(match value.as_table() {
      Some(ud) => {
        todo!()
      }

      _ => {
        return Err(LuaError::RuntimeError(format!("Expected table")));
      }
    });
  }
}


impl LuaUserData for UiNode {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(LuaMetaMethod::ToString, |_, this: &Self, ()| {
      return Ok(format!("{:?}", this));
    });


    methods.add_method("paragraph", |_, this, text: String| {
      let node = Self::new(UiNodeInner::Paragraph {
        widget: Paragraph::new(text),
      });
      this
        .write_arc()
        .append_child(Self::raw(Arc::clone(&node)), Constraint::Min(1));


      return Ok(node);
    });
    methods.add_method("alter", |_, this, text: String| {
      this.write_arc().text(text);

      return Ok(());
    });
  }
}

impl LuaUserData for TermRender {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("window", |_, this, ()| {
      let window = UiNode::raw(Arc::clone(&this.read_arc().node));

      return Ok(window);
    });
  }

  // fn add_fields<F: zhazba_lua::UserDataFields<Self>>(fields: &mut F) {}
}
