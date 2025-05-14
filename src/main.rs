use anyhow::Result;
use tracing::{debug, error, info};

use zhazba_args::Args;
use zhazba_editor::Editor;
use zhazba_logger::init_logger;
use zhazba_lua::{ObjectLike, with_global_lua};
use zhazba_render::{TermRender, disable_raw_mode};


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let default_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |info| {
    _ = disable_raw_mode();


    // FIXME: implement downcast for payload
    //        mb move to something like src/tools/panic?
    error!(
      "{:#?}",
      if let Some(&s) = info.payload().downcast_ref::<&'static str>() {
        s
      } else if let Some(s) = info.payload().downcast_ref::<String>() {
        s.as_str()
      } else {
        "Box<dyn Any>"
      }
    );
    default_hook(info);
  }));

  let args = Args::new();
  init_logger();


  let render = TermRender::new()?;
  let editor = Editor::new(args.workspace, render);


  let config_source = match std::fs::read_to_string(format!(
    "{}.config/zhazba/init.lua",
    if env!("ENV") == "DEBUG" { "./" } else { "~/" }
  )) {
    Ok(source) => source,
    Err(err) => {
      error!("{:?}", err);
      panic!();
    }
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

    let info = lua.create_function(
      |_, args: zhazba_lua::Variadic<zhazba_lua::Value>| {
        let msg: Vec<_> = args
          .iter()
          .map(|arg| match arg {
            zhazba_lua::Value::Nil => "nil".to_string(),
            zhazba_lua::Value::Boolean(b) => format!("{}", b),
            zhazba_lua::Value::Integer(i) => format!("{}", i),
            zhazba_lua::Value::Number(n) => format!("{}", n),
            zhazba_lua::Value::String(s) => format!("{}", s.display()),
            zhazba_lua::Value::Table(t) => format!(
              "{}",
              t.to_string().unwrap_or_else(|_| "<table>".to_string()),
            ),
            zhazba_lua::Value::UserData(t) => format!(
              "{}",
              t.to_string().unwrap_or_else(|_| "<user_data>".to_string()),
            ),

            _ => "<?>".to_string(),
          })
          .collect();
        info!("{}", msg.join(" "));
        return Ok(());
      },
    )?;
    lua.globals().set("info", info)?;

    lua.load(&config_source).exec()?;
    debug!("Config loaded just fine");
    return Ok::<(), anyhow::Error>(());
  })?;


  editor.write_arc().run().await?;
  debug!("{:#?}", editor.read());
  // match with_global_lua(|lua| {
  //   lua.load("").exec()?;

  //   return Ok::<(), anyhow::Error>(());
  // }) {
  //   Ok(_) => {}
  //   Err(err) => error!("{}", err),
  // };
  return Ok(());
}
