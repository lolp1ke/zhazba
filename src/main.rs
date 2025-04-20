use anyhow::Result;

use tracing::{error, info};
use zhazba_args::Args;
use zhazba_editor::Editor;
use zhazba_logger::init_logger;
use zhazba_render::{TermRender, terminal_size};


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let args = Args::new();
  init_logger();


  let render = TermRender::new()?;
  let editor = Editor::new(args.workspace, render, terminal_size()?);
  editor.borrow_mut().load_dir()?;


  let config_source = match std::fs::read_to_string(".config/zhazba.lua") {
    Ok(source) => source,
    Err(err) => {
      error!("{}", err);
      panic!();
    }
  };
  load_lua(editor.clone(), &config_source).unwrap_or_else(|err| {
    error!("{}", err);
  });
  info!("{:#?}", editor.borrow().config);


  editor.borrow_mut().run().await?;
  return Ok(());
}


// TODO: migrate fn into zhazba_lua crate
fn load_lua(editor: Editor, src: &str) -> zhazba_lua::Result<()> {
  use zhazba_action::{ActionUserDataFactory, KeyActionUserDataFactory};
  let lua = zhazba_lua::Lua::new();
  lua
    .load("package.path = package.path .. \";.config/?.lua\"")
    .exec()?;

  let editor = lua.create_userdata(editor.clone())?;
  lua.globals().set("Editor", editor)?;

  let action = lua.create_userdata(ActionUserDataFactory)?;
  lua.globals().set("Action", action)?;

  let key_action = lua.create_userdata(KeyActionUserDataFactory)?;
  lua.globals().set("KeyAction", key_action)?;

  let info = lua.create_function(|_, lua_value: zhazba_lua::Value| {
    info!("{:?}", lua_value);
    return Ok(());
  })?;
  lua.globals().set("info", info)?;


  lua.load(src).exec()?;
  info!("Config loaded just fine");
  return Ok(());
}
