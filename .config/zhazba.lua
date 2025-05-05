local config = Editor:config()

config:add_keymap("i", "n", KeyAction:Single(Action:ChangeMode("i")))
config:add_keymap("a", "n", KeyAction:Single(Action:ChangeMode("i")))

config:add_keymap("<esc>", "i", KeyAction:Single(Action:ChangeMode("n")))

config:add_keymap(":", "n", KeyAction:Single(Action:EnterRegister("cmd")))
config:add_keymap("<esc>", "n", KeyAction:Single(Action:LeaveRegister()))

config:add_keymap("<c-s>", "n", KeyAction:Single(Action:Save()))
config:add_keymap("<c-s>", "i", KeyAction:Single(Action:Save()))

config:add_keymap("<c-q>", "n", KeyAction:Single(Action:Quit(false)))
config:add_keymap("<c-q>", "i", KeyAction:Single(Action:Quit(false)))
