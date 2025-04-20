use std::collections::HashMap;

use zhazba_lua::lua_userdata_enum;

#[derive(Debug, Clone)]
#[lua_userdata_enum]
pub enum Action {
  Quit(bool),
  Save,
  ChangeMode(String),
}
// impl zhazba_lua::IntoLua for Action {
//   fn into_lua(
//     self,
//     lua: &zhazba_lua::Lua,
//   ) -> zhazba_lua::Result<zhazba_lua::Value> {
//     todo!()
//   }
// }


#[derive(Debug, Clone)]
#[lua_userdata_enum]
pub enum KeyAction {
  Single(Action),
  Multiple(Vec<Action>),
  Nested(HashMap<String, KeyAction>),
}
// impl zhazba_lua::IntoLua for KeyAction {
//   fn into_lua(
//     self,
//     lua: &zhazba_lua::Lua,
//   ) -> zhazba_lua::Result<zhazba_lua::Value> {
//     todo!()
//   }
// }
