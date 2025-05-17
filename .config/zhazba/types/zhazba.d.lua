--- @class Editor
--- @field config fun(self: self): Config
--- @field mode fun(self: self): string
--- @field render fun(self: self): Render
---
--- @field create_register fun(self: self, register: string): nil
--- @field read_register fun(self: self, register: string): string
--- @field current_register fun(self: self): string
---
--- @field event_callback fun(self: self, event_name: string, event_callback: fun()): nil
Editor = {}

--- @class Config
--- @field add_keymap fun(self: self, key_code: string, mode: string, ka: KeyAction): self | nil
--- @field add_command fun(self: self, key_code: string, ka: KeyAction): self | nil

--- @class Render
--- Returns root UiNode::Block
--- @field window fun(self: self): UiNode

--- @class UiNode
--- This must be used only for UiNode::Block or UiNode::Tabs
--- @field paragraph fun(self: self, text: string): self
--- This must be used only for UiNode::Paragraph
--- @field alter fun(self: self, text: string): nil


--- @class KeyAction
--- @field Single fun(self: self, action: Action): self
--- @field Multiple fun(self: self, actions: Action[]): self
KeyAction = {}

--- @class Action
--- @field Quit fun(self: self, force: boolean): self
--- @field Save fun(self: self): self
--- @field ChangeMode fun(self: self, mode: string): self
---
--- @field EnterRegister fun(self: self, register: string): self
--- @field LeaveRegister fun(self: self): self
---
--- @field ExecuteCommand fun(self: self): self
---
--- @field MoveTo fun(self: self, cx: integer, cy: integer): self
--- @field MoveLeft fun(self: self): self
--- @field MoveRight fun(self: self): self
--- @field MoveUp fun(self: self): self
--- @field MoveDown fun(self: self): self
---
--- @field InsertIntoRegisterAtPos fun(self: self, name: string, ch: string, cx: integer, cy: integer): self
--- @field InsertIntoRegister fun(self: self, name: string, ch: string): self
--- @field InsertIntoCurrentRegister fun(self: self, ch: string): self
--- @field DeletePrevFromRegister fun(self: self, name: string): self
--- @field DeletePrevFromCurrentRegister fun(self: self): self
---
--- @field EventCallback fun(self: self, event_name: string): self
Action = {}

-- - @param message any
--- @return nil
--- @diagnostic disable-next-line: lowercase-global
function info(...)
end
