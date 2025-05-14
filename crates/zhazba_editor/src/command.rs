use anyhow::Result;

use crate::{Editor, EditorInner};


impl EditorInner {
  pub(crate) fn is_cmd_register(&self) -> bool {
    return self
      .current_register
      .as_ref()
      .and_then(|r| Some(&**r == Editor::COMMAND_REGISTER))
      .unwrap_or_else(|| false);
  }
  pub(crate) fn execute_command(&mut self) -> Result<bool> {
    if let Some(cmd_content) =
      self.register_map.get(Editor::COMMAND_REGISTER).cloned()
    {
      let mut args = cmd_content.split_whitespace();
      let cmd = args.next();
      let args = args.collect::<Vec<&str>>();
      if let Some(cmd) = cmd {
        let ka = self.cfg().commands.get(cmd).cloned();
        if let Some(ka) = ka {
          self.handle_key_action(ka);
          return self.execute_actions();
        } else {
          todo!("plugin commands with args");
        };
      };
    };

    return Ok(false);
  }
}
