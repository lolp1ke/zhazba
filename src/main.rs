use anyhow::Result;
use zhazba_args::Args;
use zhazba_editor::Editor;
use zhazba_logger::init_logger;
use zhazba_render::{TermRender, terminal_size};


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let args = Args::new();
  init_logger();

  let render = TermRender::new()?;

  let mut editor = Editor::new(args.workspace, render, terminal_size()?);
  editor.run().await?;

  return Ok(());
}
