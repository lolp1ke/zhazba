use std::sync::Arc;

use anyhow::Result;
use tracing::{debug, error, info};

use zhazba_args::Args;
use zhazba_editor::Editor;
use zhazba_logger::init_logger;
use zhazba_lua::{LuaObjectLike, LuaValue, LuaVariadic, with_global_lua};
use zhazba_plugin::Plugin;
use zhazba_render::TermRender;


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  init_logger();
  let default_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |info| {
    TermRender::cleanup().unwrap();


    // FIXME: implement downcast for payload
    //        mb move to something like src/tools/panic?
    // info.
    error!(
      "{:?}\n{:#?}",
      info.payload().downcast_ref::<&'static str>(),
      std::backtrace::Backtrace::capture(),
    );
    default_hook(info);
  }));
  let args = Args::new();


  let render = TermRender::new()?;
  let plugin = Plugin::new();
  let editor =
    Editor::new(args.workspace, render, Plugin::raw(Arc::clone(&plugin)))?;


  let config_source = match std::fs::read_to_string(format!(
    "{}.config/zhazba/init.lua",
    if env!("ENV") == "DEBUG" { "./" } else { "~/" },
  )) {
    Ok(source) => source,
    Err(_) => panic!(),
  };
  with_global_lua(|lua| {
    use zhazba_action::{ActionUserDataFactory, KeyActionUserDataFactory};
    lua
      .load(format!(
        "package.path = package.path .. \";{0}.config/zhazba/?.lua;{0}.config/zhazba/?/init.lua\"",
        if env!("ENV") == "DEBUG" { "./" } else { "~/" },
      ))
      .exec()?;

    let editor = lua.create_userdata(editor.clone())?;
    lua.globals().set("Editor", editor)?;

    let action = lua.create_userdata(ActionUserDataFactory)?;
    lua.globals().set("Action", action)?;

    let key_action = lua.create_userdata(KeyActionUserDataFactory)?;
    lua.globals().set("KeyAction", key_action)?;

    let info = lua.create_function(|_, args: LuaVariadic<LuaValue>| {
      let msg = args
        .iter()
        .map(|arg| match arg {
          LuaValue::Nil => "nil".to_string(),
          LuaValue::Boolean(b) => format!("{}", b),
          LuaValue::Integer(i) => format!("{}", i),
          LuaValue::Number(n) => format!("{}", n),
          LuaValue::String(s) => format!("{}", s.display()),
          LuaValue::Table(t) => format!(
            "{}",
            t.to_string().unwrap_or_else(|_| "<table>".to_string()),
          ),
          LuaValue::UserData(t) => format!(
            "{}",
            t.to_string().unwrap_or_else(|_| "<user_data>".to_string()),
          ),

          _ => "<?>".to_string(),
        })
        .collect::<Vec<_>>();
      info!("{}", msg.join(" "));
      return Ok(());
    })?;
    lua.globals().set("info", info)?;

    lua.load(&config_source).exec()?;
    debug!("Config loaded just fine");
    return Ok::<(), anyhow::Error>(());
  })?;


  plugin.write_arc().init().await?;
  editor
    .write_arc()
    .run(|| unsafe {
      // FIXME: idk if it is the good way to resolve the problem but for now it works just fines
      editor.force_unlock_read_fair();
      editor.force_unlock_write_fair();
    })
    .await?;
  // debug!("{:#?}", editor.read_arc());


  TermRender::cleanup()?;
  return Ok(());
}
