use crossterm::event;

use zhazba_action::{Action, KeyAction};

use crate::{Editor, EditorInner};


impl EditorInner {
  // FIXME: Wrong approach
  //        (i just feel it is wrong)
  pub(crate) fn handle_key_code(
    &self,
    code: &event::KeyCode,
  ) -> Option<KeyAction> {
    use event::KeyCode::*;
    if let Some(r) = &self.current_register {
      if **r != *Editor::COMMAND_REGISTER {
        return None;
      };

      match code {
        Enter => {
          return Some(KeyAction::Multiple(vec![
            Action::ExecuteCommand,
            Action::ClearRegister(r.to_string()),
            Action::LeaveRegister,
            Action::ChangeMode(Editor::DEFAULT_MODE.to_string()),
          ]));
        }
        Backspace => {
          return Some(KeyAction::Single(Action::DeletePrevFromRegister(
            Editor::COMMAND_REGISTER.to_string(),
          )));
        }

        _ => {}
      };
    };


    return None;
  }
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
  pub(crate) fn handle_event(&self, event: event::Event) -> Option<KeyAction> {
    match event {
      event::Event::Key(event::KeyEvent {
        code,
        modifiers,
        kind: event::KeyEventKind::Press,
        ..
      }) => {
        if let Some(ka) = self.handle_key_code(&code) {
          return Some(ka);
        };


        let key_code = self.format_key_event(code, modifiers);
        let ka = self
          .config
          .borrow()
          .keymaps
          .get(&(key_code.clone(), self.mode))
          .cloned();
        if let Some(ka) = ka {
          if let None = self.current_register {
            return Some(ka);
          };
        };

        if let Some(register) = &self.current_register {
          if let Some(_) = self.register_map.get(register) {
            return Some(KeyAction::Single(Action::InsertIntoRegister(
              register.to_string(),
              format!("{}", code.as_char().unwrap_or_else(|| { '\0' })),
            )));
          };
        };
        if self.mode == Editor::BUFFER_MODE {
          let (cx, cy) = self.pos;


          return Some(KeyAction::Multiple(vec![
            Action::InsertIntoBufferAt(cx, cy, code.to_string()),
            Action::MoveTo(cx + 1, cy),
          ]));
        };


        return None;
      }

      _ => return None,
    };
  }

  pub(crate) fn handle_key_action(&mut self, key_action: KeyAction) {
    use KeyAction::*;

    match key_action {
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
}
