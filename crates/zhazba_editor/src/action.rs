use std::rc::Rc;

use anyhow::Result;
use tracing::error;

use zhazba_action::Action;

use crate::{Editor, EditorInner};


impl EditorInner {
  pub(crate) fn execute_actions(&mut self) -> Result<bool> {
    let mut quit = false;
    while let Some(action) = self.actions_queqe.pop_front() {
      quit = self.execute_action(action)?;
    }

    return Ok(quit);
  }
  fn execute_action(&mut self, action: Action) -> Result<bool> {
    use Action::*;

    match action {
      Quit(force) => {
        if force {
          return Ok(true);
        };

        return Ok(true);
      }
      Save => todo!(),
      ChangeMode(mode) => {
        self.mode = mode.chars().next().unwrap_or_else(|| Editor::DEFAULT_MODE);
      }

      EnterRegister(register) => {
        self.current_register = Some(Rc::from(register));
      }
      LeaveRegister => self.current_register = None,

      ExecuteCommand => self.execute_command(),

      MoveTo(cx, cy) => self.pos = (cx, cy),
      MoveLeft => todo!(),
      MoveRight => self.pos.0 += 1,
      MoveUp => todo!(),
      MoveDown => todo!(),

      InsertIntoRegister(register, append_char) => {
        if let Some(content) = self.register_map.get_mut(&Rc::from(register)) {
          content.push_str(&append_char);
        };
      }
      DeletePrevFromRegister(register) => {
        if let Some(content) = self.register_map.get_mut(&Rc::from(register)) {
          content.pop();
        };
      }
      ClearRegister(register) => {
        if let Some(content) = self.register_map.get_mut(&Rc::from(register)) {
          content.clear();
        };
      }
      InsertIntoBufferAt(cx, cy, append_char) => {
        self.insert_into_buffer((cx, cy), &append_char);
      }

      _ => error!("Action: {:?} is not implemented yet", action),
    };

    return Ok(false);
  }
}
