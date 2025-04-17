use std::collections::HashMap;

use mlua::FromLua;

#[derive(Debug)]
pub enum Action {
  Quit(bool),
  ChangeMode(char),
}


#[derive(Debug)]
pub enum KeyAction {
  Single(Action),
  Multiple(Vec<Action>),
  Nested(HashMap<String, KeyAction>),
}
impl FromLua for KeyAction {
  fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
    todo!()
  }
}
