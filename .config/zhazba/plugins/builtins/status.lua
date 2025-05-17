local render = Editor:render()
local root = render:window()
local status_line = root:paragraph(Editor:mode(), { variant = "Min", value = 1 })


Editor:event_callback("on_mode_change", function()
  status_line:alter(string.format("%-10s", Editor:mode()))
end)
Editor:event_callback("on_register_change", function()
  if Editor:current_register() == "cmd" then
    status_line:alter(Editor:read_register("cmd"))
  end
end)
