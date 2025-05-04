config = Editor:config()

config:add_keymap("i", "n", KeyAction:Single(Action:ChangeMode("i")))
config:add_keymap("a", "n", KeyAction:Single(Action:ChangeMode("i")))

config:add_keymap("esc", "i", KeyAction:Single(Action:ChangeMode("n")))
config:add_keymap("esc", "c", KeyAction:Single(Action:ChangeMode("n")))

config:add_keymap(":", "n", KeyAction:Single(Action:ChangeMode("c")))

config:add_keymap("control-s", "n", KeyAction:Single(Action:Save()))
config:add_keymap("control-s", "i", KeyAction:Single(Action:Save()))

-- config:add_keymap("q", "n", KeyAction:Single(Action:Quit(false)))
config:add_keymap("control-q", "n", KeyAction:Single(Action:Quit(false)))
config:add_keymap("control-q", "i", KeyAction:Single(Action:Quit(false)))


Editor:create_register("c")
