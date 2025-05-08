use crossterm::event;

use zhazba_action::{Action, KeyAction};

use crate::{Editor, EditorInner};


impl EditorInner {
  fn format_key_event(
    &self,
    code: event::KeyCode,
    modifiers: event::KeyModifiers,
  ) -> String {
    use event::{KeyCode, KeyModifiers};
    if modifiers.is_empty() {
      match code {
        KeyCode::Enter => return format!("<cr>"),
        KeyCode::Esc => return format!("<esc>"),
        KeyCode::Tab => return format!("<tab>"),
        KeyCode::Char(ch)
          if self.mode == Editor::DEFAULT_MODE
            && ch == self.config.borrow().leader
            && !self.is_cmd_register() =>
        {
          return format!("<leader>");
        }

        _ => {}
      };
    };
    let mut key = String::new();

    match modifiers {
      KeyModifiers::CONTROL => key.push_str("<c-"),
      KeyModifiers::SHIFT => key.push_str("<s-"),
      KeyModifiers::ALT => key.push_str("<a-"),

      _ => {}
    };
    match code {
      KeyCode::Char(ch) => key.push(ch),
      KeyCode::Enter => key.push_str("cr"),
      KeyCode::Esc => key.push_str("esc"),
      KeyCode::Tab => key.push_str("tab"),

      _ => {}
    };
    if key.starts_with('<') {
      key.push('>');
    };


    return key;
  }

  pub(crate) fn handle_command_event(
    &self,
    event: &event::Event,
  ) -> Option<KeyAction> {
    match event {
      &event::Event::Key(event::KeyEvent {
        code,
        kind: event::KeyEventKind::Press,
        ..
      }) => {
        match code {
          event::KeyCode::Char(ch) => {
            return Some(KeyAction::Single(Action::InsertIntoRegister(
              Editor::COMMAND_REGISTER.to_string(),
              ch.to_string(),
            )));
          }

          event::KeyCode::Enter => {
            return Some(KeyAction::Multiple(vec![
              Action::ExecuteCommand,
              Action::ClearRegister(Editor::COMMAND_REGISTER.to_string()),
              Action::LeaveRegister,
              Action::ChangeMode(Editor::DEFAULT_MODE.to_string()),
            ]));
          }
          event::KeyCode::Backspace => {
            return Some(KeyAction::Single(Action::DeletePrevFromRegister(
              Editor::COMMAND_REGISTER.to_string(),
            )));
          }
          event::KeyCode::Esc => {
            return Some(KeyAction::Multiple(vec![
              Action::ClearRegister(Editor::COMMAND_REGISTER.to_string()),
              Action::LeaveRegister,
              Action::ChangeMode(Editor::DEFAULT_MODE.to_string()),
            ]));
          }

          _ => return None,
        };
      }

      _ => return None,
    };
  }
  pub(crate) fn handle_event(&self, event: &event::Event) -> Option<KeyAction> {
    match event {
      &event::Event::Key(event::KeyEvent {
        code,
        modifiers,
        kind: event::KeyEventKind::Press,
        ..
      }) => {
        let key_code = self.format_key_event(code, modifiers);
        let ka = self.keymaps().get(&(key_code.clone(), self.mode)).cloned();
        if let Some(ka) = ka {
          return Some(ka);
        };
        // if self.mode == Editor::BUFFER_MODE {
        //   let (cx, cy) = self.pos;


        //   return Some(KeyAction::Multiple(vec![
        //     Action::InsertIntoBufferAt(cx, cy, code.to_string()),
        //     Action::MoveTo(cx + 1, cy),
        //   ]));
        // };


        return None;
      }

      _ => return None,
    };
  }
  pub(crate) fn handle_key_action(&mut self, ka: KeyAction) {
    use KeyAction::*;

    match ka {
      Single(action) => {
        self.actions_queqe.push_back(action);
      }
      Multiple(actions) => {
        for action in actions {
          self.actions_queqe.push_back(action);
        }
      }
      Nested(keymap) => {
        todo!("{:?}", keymap);
      }
    };
  }
  pub(crate) fn handle_key_action_front(&mut self, ka: KeyAction) {
    use KeyAction::*;

    match ka {
      Single(action) => {
        self.actions_queqe.push_front(action);
      }
      Multiple(actions) => {
        for action in actions {
          self.actions_queqe.push_front(action);
        }
      }
      Nested(keymap) => {
        todo!("{:?}", keymap);
      }
    };
  }
}
