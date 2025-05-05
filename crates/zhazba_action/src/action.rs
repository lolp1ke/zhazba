use std::collections::HashMap;

use zhazba_lua::lua_userdata_enum;


#[derive(Clone, Debug)]
#[lua_userdata_enum]
pub enum Action {
  Quit(bool),
  Save,
  ChangeMode(String),

  EnterRegister(String),
  LeaveRegister,

  ExecuteCommand,

  MoveTo(usize, usize),
  MoveLeft,
  MoveRight,
  MoveUp,
  MoveDown,

  InsertIntoRegister(String, String),
  DeletePrevFromRegister(String),
  ClearRegister(String),
  InsertIntoBufferAt(usize, usize, String),
  // Callback(Function),
  Dummy,
}

#[derive(Clone, Debug)]
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
