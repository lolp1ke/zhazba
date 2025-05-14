use std::{
  collections::{HashMap, VecDeque},
  fs::DirEntry,
  ops::Deref,
  path::PathBuf,
  sync::Arc,
  time::Duration,
};

use anyhow::Result;
use crossterm::{ExecutableCommand, event, terminal};
use futures::{FutureExt, StreamExt, select};
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
// NOTE: mb remove ratatui from dependencies for this crate
use ratatui::layout::Rect;
use tracing::error;

use zhazba_action::Action;
use zhazba_buffer::{Buffer, BufferInner, BufferManager};
use zhazba_config::{Config, ConfigInner};
use zhazba_lua::Function;
use zhazba_plugin::Plugin;
use zhazba_render::{TermRender, UiNode};


#[derive(Clone, Debug)]
pub struct Editor(Arc<RwLock<EditorInner>>);
impl Editor {
  pub(crate) const DEFAULT_MODE: char = 'n';
  pub(crate) const BUFFER_MODE: char = 'i';
  pub(crate) const COMMAND_REGISTER: &str = "cmd";


  pub fn new(workspace: Option<PathBuf>, render: TermRender) -> Self {
    let size = render.read().stdout.read().size().unwrap();


    return Self(Arc::new(RwLock::new(EditorInner::new(
      workspace,
      render,
      (size.width, size.height),
    ))));
  }
}
impl Deref for Editor {
  type Target = Arc<RwLock<EditorInner>>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}

#[derive(Clone, Debug)]
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
  pub(crate) event_callbacks: HashMap<Arc<str>, Vec<Function>>,
}
impl EditorInner {
  pub fn new(
    workspace: Option<PathBuf>,
    render: TermRender,
    size: (u16, u16),
  ) -> Self {
    let register_map = HashMap::from_iter([(
      Arc::from(Editor::COMMAND_REGISTER),
      String::new(),
    )]);
    let event_callbacks =
      HashMap::from_iter([(Arc::from("on_mode_change"), Vec::new())]);

    let mut editor = Self {
      config: Config::default(),

      workspace,
      buffer_manager: BufferManager::new(),
      mode: Editor::DEFAULT_MODE,

      plugin: Plugin::new(),

      render,
      size,
      pos: (0, 0),
      v_pos: (0, 0),

      register_map,
      current_register: None,

      actions_queqe: VecDeque::new(),
      event_callbacks,
    };
    let _ = editor.load_dir();


    return editor;
  }
  fn load_dir(&mut self) -> Result<()> {
    if let Some(workspace) = self.workspace.as_ref() {
      if workspace.is_file() {
        self.buffer_manager.push_front(Buffer::new(
          BufferInner::load_from_file(workspace.clone()),
        ));

        return Ok(());
      };
      visit_dirs(workspace, &mut |dir_entry: &DirEntry| {
        let buffer: BufferInner = BufferInner::load_from_file(dir_entry.path());
        let buffer: Buffer = Buffer::new(buffer);
        self.buffer_manager.push_front(buffer);
      })?;
    };


    return Ok(());
    // TODO: Move into something like src/tools?
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
  pub(crate) fn cfg(&self) -> MappedRwLockReadGuard<'_, ConfigInner> {
    return RwLockReadGuard::map(self.config.read(), |cfg| return cfg);
  }


  pub async fn run(&mut self) -> Result<()> {
    self.plugin.write_arc().init().await?;

    let mut event_stream = event::EventStream::new();

    let b = self.buffer_manager.get_buffer().clone();
    self
      .render
      .write_arc()
      .node
      .append_child(UiNode::Buffer(Arc::new(b)));
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


          self.render.write_arc().draw_frame()?;
          if self.execute_actions()? {
            break;
          };
        }
      };
    }

    terminal::disable_raw_mode()?;
    self
      .render
      .write_arc()
      .stdout
      .write_arc()
      .backend_mut()
      .execute(terminal::LeaveAlternateScreen)?;
    return Ok(());
  }
}
