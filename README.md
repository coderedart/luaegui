# luaegui
egui bindings for mlua. 

Just look at the example for basic usage. You can play with the web version live at https://coderedart.github.io/luaegui/

There should be a window called `Script Editor` where you can edit the lua code live within egui.
After editing the code, just click the `run` button on top to execute that code in the lua vm.
If there was any error, it will be printed to stdout/console(on web). 
Below the code editor, you can see how long the `gui_run` fn takes every frame. 
### gui_run
Every frame, the example will try to call the `gui_run` fn (if it exists) and gives it egui context as the argument.
If the fn fails for some reason, the error will be printed to stdout/console. 

### egui
We already provide a global table called `egui` which contains most constants + types + functions to be used by lua scripts.
for example, you can create a `Window` using `local window = egui.window.new("my window title");`

