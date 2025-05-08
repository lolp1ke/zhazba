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

  InsertIntoRegisterAtPos(String, String, usize, usize),
  InsertIntoRegister(String, String),
  InsertIntoCurrentRegister(String),
  DeletePrevFromRegister(String),
  DeletePrevFromCurrentRegister,
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
