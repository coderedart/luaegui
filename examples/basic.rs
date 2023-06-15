use egui_backend::{BackendConfig, GfxBackend, UserApp, WindowBackend};
use egui_render_glow::GlowBackend;
use egui_window_glfw_passthrough::GlfwBackend;
use mlua::Function;

fn main() {
    fake_main();
}

const LUA_CODE: &str = r#"
my_data = {
    text = "my text"
}
function show_fn(ui)
    ui:label(my_data.text);
    ui:text_edit_singleline(my_data);
    if ui:button("cute button"):clicked() then
        print("cute button pressed. printing my_data.text");
        print(my_data.text);
    end
end
function gui_run(ctx)
    local top_panel = egui.top_bottom_panel.top("top panel");
    top_panel:show(ctx, 
        function (ui) 
            ui:menu_button("my menu",
                function (ui) 
                    ui:label("empty :(");
                end
            );
        end
    );
    local new_window = egui.window.new("my lua window");
    new_window:show(ctx, show_fn);
end
"#;

struct AppData<W: WindowBackend, G: GfxBackend> {
    pub script_time: std::time::Duration,
    pub lua: mlua::Lua,
    pub code: String,
    pub markdown_cache: egui_commonmark::CommonMarkCache,
    pub egui_context: egui::Context,
    pub gfx_backend: G,
    pub window_backend: W,
}

impl<W: WindowBackend, G: GfxBackend> UserApp for AppData<W, G> {
    fn gui_run(&mut self) {
        use egui::*;
        let ctx = self.egui_context.clone();
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
    type UserGfxBackend = G;
    type UserWindowBackend = W;
    fn get_all(
        &mut self,
    ) -> (
        &mut Self::UserWindowBackend,
        &mut Self::UserGfxBackend,
        &egui::Context,
    ) {
        (
            &mut self.window_backend,
            &mut self.gfx_backend,
            &mut self.egui_context,
        )
    }
}

fn fake_main() {
    let mut window_backend = GlfwBackend::new(
        Default::default(),
        BackendConfig {
            is_opengl: true,
            ..Default::default()
        },
    );
    let gfx_backend = GlowBackend::new(&mut window_backend, Default::default());
    let lua = mlua::Lua::new();
    luaegui::register_egui_bindings(&lua).unwrap();
    let app = AppData {
        egui_context: Default::default(),
        gfx_backend,
        window_backend,
        lua,
        code: LUA_CODE.to_string(),
        script_time: std::time::Duration::ZERO,
        markdown_cache: Default::default(),
    };
    GlfwBackend::run_event_loop(app);
}

const README: &str = include_str!("../README.md");
