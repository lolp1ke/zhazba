use std::{ops::Deref, sync::Arc};

use ratatui::{layout::Constraint, widgets::Paragraph};

use zhazba_lua::{
  FromLua, Lua, LuaError, LuaMetaMethod, LuaResult, LuaUserData,
  LuaUserDataMethods, LuaValue,
};

use crate::{TermRender, UiNode, UiNodeInner};


// TODO: Create proc_macro_attribute
struct LuaWrapper<T>(T);
impl<T> Deref for LuaWrapper<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}
impl FromLua for LuaWrapper<Constraint> {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    match value.as_table() {
      Some(t) => {
        let value = t.get::<u16>("value")?;
        let variant = t.get::<String>("variant")?;


        match variant.to_lowercase().as_str() {
          "fill" => {
            return Ok(Self(Constraint::Fill(value)));
          }
          "lenght" => {
            return Ok(Self(Constraint::Length(value)));
          }
          "max" => {
            return Ok(Self(Constraint::Max(value)));
          }
          "min" => {
            return Ok(Self(Constraint::Min(value)));
          }
          "percentage" => {
            return Ok(Self(Constraint::Percentage(value)));
          }

          _ => {
            return Err(LuaError::RuntimeError(format!(
              "Unexpected key value for a table"
            )));
          }
        };
      }

      _ => {
        return Err(LuaError::RuntimeError(format!("Expected table")));
      }
    };
  }
}


impl LuaUserData for UiNode {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(LuaMetaMethod::ToString, |_, this: &Self, ()| {
      return Ok(format!("{:?}", this));
    });


    methods.add_method(
      "paragraph",
      |_, this, (text, constraint): (String, LuaWrapper<Constraint>)| {
        let node = Self::new(UiNodeInner::Paragraph {
          widget: Paragraph::new(text),
        });
        this
          .write_arc()
          .append_child(Self::raw(Arc::clone(&node)), *constraint);


        return Ok(node);
      },
    );
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
