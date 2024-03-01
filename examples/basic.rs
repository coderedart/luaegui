use egui_overlay::*;
use mlua::Function;

fn main() {
    fake_main();
}

const LUA_CODE: &str = include_str!("script.lua");

struct AppData {
    pub script_time: std::time::Duration,
    pub lua: mlua::Lua,
    pub code: String,
    pub markdown_cache: egui_commonmark::CommonMarkCache,
}

impl EguiOverlay for AppData {
    fn gui_run(
        &mut self,
        egui_context: &egui::Context,
        _default_gfx_backend: &mut egui_render_three_d::ThreeDBackend,
        _glfw_backend: &mut egui_window_glfw_passthrough::GlfwBackend,
    ) {
        use egui::*;
        let ctx = egui_context.clone();
        Window::new("README").show(&ctx, |ui| {
            egui_commonmark::CommonMarkViewer::new("readme renderer").show(
                ui,
                &mut self.markdown_cache,
                README,
            );
        });
        Window::new("Script Editor")
            .min_width(400.0)
            .show(&ctx, |ui| {
                if ui.button("run").clicked() {
                    if let Err(e) = self.lua.load(&self.code).exec() {
                        eprintln!("lua load error: {e:?}");
                    }
                }
                if !self.lua.globals().contains_key("gui_run").unwrap() {
                    ui.colored_label(Color32::RED, "gui_run fn is not defined");
                }
                ui.add(
                    egui::TextEdit::multiline(&mut self.code)
                        .code_editor()
                        .desired_width(400.0),
                );
                ui.horizontal(|ui| {
                    ui.label("script execution time (micros): ");
                    ui.label(format!("{}", self.script_time.as_micros()));
                });
            });
        let start = std::time::Instant::now();
        if let Ok(f) = self.lua.globals().get::<_, Function>("gui_run") {
            let c = self.lua.create_any_userdata(ctx).unwrap();
            let _: () = f.call(c).unwrap();
        }
        self.script_time = start.elapsed();
    }
}

fn fake_main() {
    let lua = mlua::Lua::new();
    luaegui::register_egui_bindings(&lua).unwrap();
    let app = AppData {
        lua,
        code: LUA_CODE.to_string(),
        script_time: std::time::Duration::ZERO,
        markdown_cache: Default::default(),
    };
    start(app)
}

const README: &str = include_str!("../README.md");
