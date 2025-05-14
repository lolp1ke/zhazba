pub use mlua::*;
pub use zhazba_lua_derive::*;

use once_cell::sync::Lazy;


static LUA: Lazy<Lua> = Lazy::new(|| Lua::new());
pub fn with_global_lua<F, T>(f: F) -> T
where
  F: FnOnce(&Lua) -> T, {
  return f(&LUA);
}
