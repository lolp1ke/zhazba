use anyhow::Result;
use tracing::{error, info};

use zhazba_args::Args;
use zhazba_editor::Editor;
use zhazba_logger::init_logger;
use zhazba_lua::LUA;
use zhazba_render::{TermRender, disable_raw_mode, terminal_size};


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  std::panic::set_hook(Box::new(|info| {
    let _ = disable_raw_mode();
    info!("{:?}", info);
  }));

  let args = Args::new();
  init_logger();


  let render = TermRender::new()?;
  let editor = Editor::new(args.workspace, render, terminal_size()?);
  editor.borrow_mut().load_dir()?;


  let config_source = match std::fs::read_to_string(".config/zhazba.lua") {
    Ok(source) => source,
    Err(err) => {
      error!("{:?}", err);
      panic!();
    }
  };
  LUA
    .with(|lua| {
      use zhazba_action::{ActionUserDataFactory, KeyActionUserDataFactory};

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


      lua.load(&config_source).exec()?;
      info!("Config loaded just fine");
      return Ok::<(), zhazba_lua::Error>(());
    })
    .unwrap_or_else(|err| {
      error!("{:?}", err);
    });
  // info!("{:#?}", editor.borrow().config);
  // info!("{:#?}", editor.borrow().render);


  editor.borrow_mut().run().await?;
  return Ok(());
}
