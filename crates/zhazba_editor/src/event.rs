use zhazba_action::Action;

use crate::EditorInner;


impl EditorInner {
  // TODO: Make it lua configurable
  pub(crate) fn check_for_native_events(&mut self, action: &Action) {
    if matches!(action, Action::ChangeMode(..)) {
      self
        .actions_queqe
        .push_back(Action::EventCallback("on_mode_change".to_string()));
    };
    if matches!(
      action,
      Action::InsertIntoRegister(..)
        | Action::InsertIntoCurrentRegister(..)
        | Action::InsertIntoRegisterAtPos(..)
        | Action::DeletePrevFromRegister(..)
        | Action::DeletePrevFromCurrentRegister
        | Action::ClearRegister(..)
    ) {
      self
        .actions_queqe
        .push_back(Action::EventCallback("on_register_change".to_string()));
    };
  }
}
