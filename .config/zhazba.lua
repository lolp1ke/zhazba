Config = Editor:config()

Config:add_keymap("i", "n", KeyAction:Single(Action:ChangeMode("i")))
Config:add_keymap("a", "n", KeyAction:Single(Action:ChangeMode("i")))
Config:add_keymap("esc", "i", KeyAction:Single(Action:ChangeMode("n")))

Config:add_keymap("ctrl-s", "n", KeyAction:Single(Action:Save()))
Config:add_keymap("ctrl-s", "i", KeyAction:Single(Action:Save()))

Config:add_keymap("q", "n", KeyAction:Single(Action:Quit(false)))
