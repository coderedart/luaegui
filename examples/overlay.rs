use egui_backend::{BackendConfig, GfxBackend, UserApp, WindowBackend};
use egui_render_wgpu::WgpuBackend;
use egui_window_glfw_passthrough::GlfwBackend;
use mlua::Function;

fn main() {
    fake_main();
}

fn fake_main() {
    let mut glfw_backend = GlfwBackend::new(
        Default::default(),
        BackendConfig {
            is_opengl: false,
            opengl_config: Default::default(),
            transparent: None,
        },
    );
    let wgpu_backend = WgpuBackend::new(&mut glfw_backend, Default::default());
    let lua = mlua::Lua::new();
    luaegui::register_egui_bindings(&lua).unwrap();
    let app = AppData {
        egui_context: Default::default(),
        gfx_backend: wgpu_backend,
        window_backend: glfw_backend,
        lua,
        code: LUA_CODE.to_string(),
    };
    GlfwBackend::run_event_loop(app);
}

struct AppData<W: WindowBackend, G: GfxBackend> {
    pub lua: mlua::Lua,
    pub code: String,
    pub egui_context: egui::Context,
    pub gfx_backend: G,
    pub window_backend: W,
}

impl<W: WindowBackend, G: GfxBackend> UserApp for AppData<W, G> {
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

    fn gui_run(&mut self) {
        use egui::*;
        let ctx = self.egui_context.clone();
        Window::new("My Window").show(&ctx, |ui| {
            if ui.button("run").clicked() {
                self.lua.load(&self.code).exec().unwrap();
            }
            ui.code_editor(&mut self.code);
        });

        if let Ok(f) = self.lua.globals().get::<_, Function>("gui_run") {
            let c = self.lua.create_any_userdata(ctx).unwrap();
            let _: () = f.call(c).unwrap();
        }
    }
}

const LUA_CODE: &str = r#"
function gui_run(ctx)

end
"#;
