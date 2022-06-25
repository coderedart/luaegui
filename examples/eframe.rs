use egui::Window;

pub struct MyApp {
    pub lua_code: String,
    pub lua_vm: tealr::mlu::mlua::Lua,
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        Window::new("lua editor").show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.lua_code);
        });
        self.lua_vm
            .globals()
            .set("ctx", luaegui::Context::from(ctx.clone()))
            .unwrap();
        self.lua_vm
            .load(&self.lua_code)
            .exec()
            .expect("failed to execute lua code");
    }
}
pub const LUA_GUI_CODE: &str = r#"
    ctx:window(function () ui:label("hello label from lua") end );
"#;
pub fn main() {
    let app = Box::new(MyApp {
        lua_code: LUA_GUI_CODE.to_string(),
        lua_vm: tealr::mlu::mlua::Lua::new(),
    });
    eframe::run_native(
        "eframe lua example",
        eframe::NativeOptions::default(),
        Box::new(|_| app),
    );
}
