use tracing::info;

use crate::{Editor, EditorInner};


impl EditorInner {
  pub(crate) fn execute_command(&mut self) {
    if let Some(cmd) = self.register_map.get(Editor::COMMAND_REGISTER) {
      if let Some((cmd, args)) = cmd.split_once(' ') {
        let args = args.split(" ");
      };


      info!("Command: {:?}", cmd);
    };
  }
}
