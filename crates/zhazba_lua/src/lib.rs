pub use mlua::*;
pub use zhazba_lua_derive::*;


// pub struct Shared<T>(pub T);
// impl<T: UserData + 'static> IntoLua for Shared<T> {
//   fn into_lua(self, lua: &Lua) -> Result<Value> {
//     return Ok(Value::UserData(lua.create_userdata(self.0)?));
//   }
// }
