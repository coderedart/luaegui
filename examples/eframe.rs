use egui::Window;

pub struct MyApp {
    pub lua_code: String,
    pub lua_vm: tealr::mlu::mlua::Lua,
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        Window::new("lua editor").show(ctx, |ui| {
            if ui.button("reload code").clicked() {
                self.lua_vm
                    .load(&self.lua_code)
                    .exec()
                    .expect("failed to execute lua code");
            }
            if ui.button("realod sample.lua file").clicked() {
                let code = std::fs::read_to_string("./examples/sample.lua")
                    .expect("failed to read examples/sample.lua file");
                self.lua_code = code;
                self.lua_vm
                    .load(&self.lua_code)
                    .exec()
                    .expect("failed to execute lua code");
            }
            ui.text_edit_multiline(&mut self.lua_code);
        });
        self.lua_vm
            .globals()
            .get::<_, tealr::mlu::mlua::Function>("On_gui")
            .expect("failed to get on_gui function")
            .call::<_, ()>(luaegui::Context::from(ctx.clone()))
            .expect("failed to call On_gui function");
    }
}
pub const LUA_GUI_CODE: &str = include_str!("sample.lua");
pub fn main() {
    let lua_vm = tealr::mlu::mlua::Lua::new();
    let app = Box::new(MyApp {
        lua_code: LUA_GUI_CODE.to_string(),
        lua_vm,
    });
    eframe::run_native(
        "eframe lua example",
        eframe::NativeOptions {
            always_on_top: true,
            ..Default::default()
        },
        Box::new(|_creation_context| {
            luaegui::register_egui_lua_bindings(&app.lua_vm)
                .expect("failed to register egui bindings");
            app.lua_vm
                .load(LUA_GUI_CODE)
                .exec()
                .expect("failed to execute lua code");
            app
        }),
    );
}
