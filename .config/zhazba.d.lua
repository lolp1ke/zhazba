--- @class Editor
--- @field config fun(self: self): Config
--- @field render fun(self: self): Render
---
--- @field create_register fun(self: self, name: string): nil
Editor = {}

--- @class Config
--- @field add_keymap fun(self: self, key_map: string, mode: string, ka: KeyAction): KeyAction | nil

--- @class Render


--- @class KeyAction
--- @field Single fun(self: self, action: Action): KeyAction
KeyAction = {}

--- @class Action
--- @field Quit fun(self: self, force: boolean): Action
--- @field Save fun(self: self): Action
--- @field ChangeMode fun(self: self, mode: string): Action
---
--- @field EnterRegister fun(self: self, register: string): Action
--- @field LeaveRegister fun(self: self): Action
---
--- @field ExecuteCommand fun(self: self): Action
---
--- @field MoveTo fun(self: self, cx: integer, cy: integer): Action
--- @field MoveLeft fun(self: self): Action
--- @field MoveRight fun(self: self): Action
--- @field MoveUp fun(self: self): Action
--- @field MoveDown fun(self: self): Action
Action = {}

--- @param message any
--- @return nil
--- @diagnostic disable-next-line: lowercase-global
function info(message)
end
