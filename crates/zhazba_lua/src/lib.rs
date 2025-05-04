pub use mlua::*;
pub use zhazba_lua_derive::*;


thread_local! {
  pub static LUA: Lua = Lua::new();
}
