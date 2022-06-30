use egui::Window;

pub struct MyApp {
    pub lua_code: String,
    pub lua_vm: tealr::mlu::mlua::Lua,
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        Window::new("lua editor").show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.lua_code);
            if ui.button("load lua code").clicked() {
                self.lua_vm
                    .load(&self.lua_code)
                    .exec()
                    .expect("failed to execute lua code");
            }
        });
        let _: () = self
            .lua_vm
            .globals()
            .get::<_, tealr::mlu::mlua::Function>("on_gui")
            .expect("failed to get on_gui function")
            .call(luaegui::Context::from(ctx.clone()))
            .unwrap();
    }
}
pub const LUA_GUI_CODE: &str = r#"
    my_plugin = {}
    my_plugin.window_options = {
        title = "my lua window",
        open = true
    }
    on_gui = function (ctx) 
        ctx:new_window(
            my_plugin.window_options,
            function (ui)
                ui:label("hello label from lua")
            end
        );
    end
"#;
pub fn main() {
    let lua_vm = tealr::mlu::mlua::Lua::new();
    let app = Box::new(MyApp {
        lua_code: LUA_GUI_CODE.to_string(),
        lua_vm,
    });
    eframe::run_native(
        "eframe lua example",
        eframe::NativeOptions::default(),
        Box::new(|_creation_context| {
            // app.lua_vm
            //     .globals()
            //     .set::<_, luaegui::Context>("ctx", creation_context.egui_ctx.clone().into())
            //     .unwrap();
            app.lua_vm
                .load(LUA_GUI_CODE)
                .exec()
                .expect("failed to execute lua code");
            app
        }),
    );
}
