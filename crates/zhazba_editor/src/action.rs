use std::sync::Arc;

use anyhow::Result;
use tracing::{debug, error};

use zhazba_action::Action;

use crate::{Editor, EditorInner};


impl EditorInner {
  pub(crate) fn execute_actions(&mut self) -> Result<bool> {
    let mut quit = false;
    while let Some(action) = self.actions_queqe.pop_front() {
      quit |= self.execute_action(action)?;
    }

    return Ok(quit);
  }
  fn execute_action(&mut self, action: Action) -> Result<bool> {
    use Action::*;

    debug!("Action to be executed: {:?}", action);
    match action {
      Quit(force) => {
        if force {
          return Ok(true);
        };

        return Ok(true);
      }
      Save => {}
      ChangeMode(mode) => {
        self.mode = mode.chars().next().unwrap_or_else(|| Editor::DEFAULT_MODE);
      }

      EnterRegister(register) => {
        self.current_register = Some(Arc::from(register));
      }
      LeaveRegister => self.current_register = None,

      ExecuteCommand => return self.execute_command(),

      MoveTo(cx, cy) => self.pos = (cx, cy),
      MoveLeft => todo!(),
      MoveRight => self.pos.0 += 1,
      MoveUp => todo!(),
      MoveDown => todo!(),

      InsertIntoRegister(register, append_char) => {
        if let Some(content) = self.register_map.get_mut(&*register) {
          content.push_str(&append_char);
        };
      }
      InsertIntoCurrentRegister(append_char) => {
        if let Some(register) = &self.current_register {
          if let Some(content) = self.register_map.get_mut(register) {
            content.push_str(&append_char);
          };
        };
      }
      DeletePrevFromRegister(register) => {
        if let Some(content) = self.register_map.get_mut(&*register) {
          content.pop();
        };
      }
      DeletePrevFromCurrentRegister => {
        if let Some(register) = &self.current_register {
          if let Some(content) = self.register_map.get_mut(register) {
            content.pop();
          };
        };
      }
      ClearRegister(register) => {
        if let Some(content) = self.register_map.get_mut(&*register) {
          content.clear();
        };
      }
      InsertIntoBufferAt(append_char, cx, cy) => {
        self.insert_into_buffer((cx, cy), &append_char);
      }

      EventCallback(event_name) => {
        if let Some(lua_callbacks) = self.event_callbacks.get(&*event_name) {
          for lua_callback in lua_callbacks.iter() {
            lua_callback.call::<()>(()).unwrap_or_else(|err| {
              error!(
                "Failed to call callback function on event: {}\nError: {}",
                event_name, err
              );
            });
          }
        };

        // NOTE: Maybe add rerender action
        self.render.write_arc().draw_frame()?;
      }

      _ => error!("Action: {:?} is not implemented yet", action),
    };

    return Ok(false);
  }
}
