use std::collections::HashMap;

use zhazba_lua::lua_userdata_enum;


#[derive(Clone, Debug)]
#[lua_userdata_enum]
pub enum Action {
  // force
  Quit(bool),
  Save,
  // mode
  ChangeMode(String),

  // register
  EnterRegister(String),
  LeaveRegister,

  ExecuteCommand,

  // cx, cy
  MoveTo(usize, usize),
  MoveLeft,
  MoveRight,
  MoveUp,
  MoveDown,

  // register, append_char, cx, cy
  InsertIntoRegisterAtPos(String, String, usize, usize),
  // register, append_char
  InsertIntoRegister(String, String),
  // append_char
  InsertIntoCurrentRegister(String),
  // register
  DeletePrevFromRegister(String),
  DeletePrevFromCurrentRegister,
  // register
  ClearRegister(String),
  // append_char, cx, cy
  InsertIntoBufferAt(String, usize, usize),
  // Callback(Function),
  // Event_name
  EventCallback(String),
  Dummy,
}

#[derive(Clone, Debug)]
#[lua_userdata_enum]
pub enum KeyAction {
  Single(Action),
  Multiple(Vec<Action>),
  Nested(HashMap<String, KeyAction>),
}
