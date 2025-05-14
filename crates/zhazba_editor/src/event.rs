use zhazba_action::Action;

use crate::EditorInner;

impl EditorInner {
  pub(crate) fn check_for_native_events(&mut self, action: &Action) {
    if matches!(action, Action::ChangeMode(_)) {
      self
        .actions_queqe
        .push_front(Action::EventCallback("on_mode_change".to_string()));
    };
  }
}
