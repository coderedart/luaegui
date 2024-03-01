# Unmaintained
### This repo is abandoned and you are welcome to fork it.

## Deprecation Reason: Performance
1. Due to the immediate mode nature of `egui`, the scripts need to run *every* frame (average of 60 fps atleast).
2. Ui code creates *lots* of temporary `use once` objects, which means lots of garbage collectable objects for a reasonably complex ui script. It is also expensive to create/recreate these objects every frame.
    1. Builder pattern creates a struct for every container/widget. eg: `Window::new("hello")` or `RichText::new("text").color(egui.blue)` etc..
    2. return values of widget `ui` fn like `Response` also create userdata objects.
    3. each `&mut Ui` needs to create a new userdata object to bind that reference to.
    4. Even value types like `Rect` will need to be userdata.
3. Due to a large amount of function calls between native host egui and the script, JIT also sucks at optimizing this. 
4. All the closures also require jumping between host and guest scopes which have some "setup"/"teardown" costs 

This project might still work for some people, and I would encourage them to fork this repo.
But I would like to experiment with retained mode toolkits now, where you only scripts on events.



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
We provide a global table called `egui` which contains most constants + types + functions to be used by lua scripts.
for example, you can create a `Window` using `local window = egui.window.new("my window title");`

### Developer Experience 
Because we don't really have a way to properly document host api in mlua yet, we will do this manually. For now, we provide a type definition file (WIP) `egui.d.lua`. 

1. Install `Luau Language Server` extension by `JohnnyMorganz` in vscode
2. copy `egui.d.lua` file from thi repo to your lua project folder
3. In the settings Ui `luau-lsp.types.definitionFiles`, add the file `egui.d.lua`
4. Now, you have autocompletion, as well as linting (to a reasonable extent) when you want to use egui. 



