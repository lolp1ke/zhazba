use std::{
  collections::{HashMap, VecDeque},
  fs::DirEntry,
  ops::Deref,
  path::PathBuf,
  sync::Arc,
  time::Duration,
};

use anyhow::Result;
use crossterm::event;
use futures::{FutureExt, StreamExt, select};
use parking_lot::RwLock;

use tracing::error;
use zhazba_action::Action;
use zhazba_buffer::{Buffer, BufferInner, BufferManager};
use zhazba_config::Config;
use zhazba_lua::LuaFunction;
use zhazba_plugin::Plugin;
use zhazba_render::TermRender;


#[derive(Clone, Debug)]
pub struct Editor(Arc<RwLock<EditorInner>>);
impl Editor {
  pub(crate) const DEFAULT_MODE: char = 'n';
  pub(crate) const BUFFER_MODE: char = 'i';
  pub(crate) const COMMAND_REGISTER: &str = "cmd";


  pub fn new(
    workspace: Option<PathBuf>,
    render: TermRender,
    plugin: Plugin,
  ) -> Result<Self> {
    let size = render.read_arc().stdout.size()?;
    let register_map = HashMap::from_iter([(
      Arc::from(Editor::COMMAND_REGISTER),
      String::new(),
    )]);
    let event_callbacks =
      HashMap::from_iter([(Arc::from("on_mode_change"), Vec::new())]);


    let editor = Self(Arc::new(RwLock::new(EditorInner {
      config: Config::default(),
      buffer_manager: BufferManager::new(),
      workspace,
      mode: Self::DEFAULT_MODE,
      render,
      plugin,
      size: (size.width, size.height),
      pos: (0, 0),
      v_pos: (0, 0),
      register_map,
      current_register: None,
      actions_queqe: VecDeque::new(),
      event_callbacks,
    })));
    editor.write_arc().load_dir()?;


    return Ok(editor);
  }
}
impl Deref for Editor {
  type Target = Arc<RwLock<EditorInner>>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}

#[derive(Debug)]
pub struct EditorInner {
  pub(crate) config: Config,
  pub(crate) buffer_manager: BufferManager,
  workspace: Option<PathBuf>,

  pub(crate) mode: char,
  pub(crate) render: TermRender,

  plugin: Plugin,

  size: (u16, u16),
  pub(crate) pos: (usize, usize),
  v_pos: (usize, usize),

  pub(crate) register_map: HashMap<Arc<str>, String>,
  pub(crate) current_register: Option<Arc<str>>,

  pub(crate) actions_queqe: VecDeque<Action>,
  pub(crate) event_callbacks: HashMap<Arc<str>, Vec<LuaFunction>>,
}
impl EditorInner {
  fn load_dir(&mut self) -> Result<()> {
    if let Some(workspace) = self.workspace.as_ref() {
      visit_dirs(workspace, &mut |dir_entry: &DirEntry| {
        let buffer: BufferInner = BufferInner::load_from_file(dir_entry.path());
        let buffer: Buffer = Buffer::new(buffer);
        self.buffer_manager.push_front(buffer);
      })?;
    };


    return Ok(());
    fn visit_dirs(dir: &PathBuf, cb: &mut dyn FnMut(&DirEntry)) -> Result<()> {
      if !dir.is_dir() {
        return Ok(());
      };
      for entry in std::fs::read_dir(dir)? {
        let entry: DirEntry = entry?;
        let path: PathBuf = entry.path();
        if path.is_dir() {
          visit_dirs(&path, cb)?;
        } else {
          cb(&entry);
        };
      }


      return Ok(());
    }
  }
  pub async fn run(&mut self, unlock: impl FnOnce()) -> Result<()> {
    unlock();
    let mut event_stream = event::EventStream::new();

    self.render.write_arc().draw_frame()?;
    loop {
      let mut delay =
        futures_timer::Delay::new(Duration::from_millis(100)).fuse();
      let mut event = event_stream.next().fuse();

      select! {
        _ = delay => {
        }

        event = event => {
          match event {
            Some(Ok(event)) => {
              if self.is_cmd_register() {
                if let Some(ka) = self.handle_command_event(&event) {
                  self.handle_key_action(ka);
                  if self.execute_actions()? {
                    break;
                  };
                };

                continue;
              };

              if let Some(ka) = self.handle_event(&event) {
                self.handle_key_action(ka);
              };
            }

            Some(Err(err)) => {
              error!("{err}");
            }
            None => {}
          };


          if self.execute_actions()? {
            break;
          };
          self.render.write_arc().draw_frame()?;
        }
      };
    }


    return Ok(());
  }
}
