#[derive(Clone)]
pub struct Context(egui::Context);

impl AsRef<egui::Context> for Context {
    fn as_ref(&self) -> &egui::Context {
        &self.0
    }
}
impl AsMut<egui::Context> for Context {
    fn as_mut(&mut self) -> &mut egui::Context {
        &mut self.0
    }
}

impl From<egui::Context> for Context {
    fn from(e: egui::Context) -> Self {
        Self(e)
    }
}

impl Into<egui::Context> for Context {
    fn into(self) -> egui::Context {
        self.0
    }
}

impl tealr::mlu::mlua::UserData for Context {
    fn add_fields<'lua, F: tealr::mlu::mlua::UserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: tealr::mlu::mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method(
            "new_window",
            |lua, ctx, args: (String, tealr::mlu::mlua::Table, tealr::mlu::mlua::Function)| {
                let name = args.0;
                let _window_options = args.1;
                let ui_callback = args.2;
                let window = egui::Window::new(&name);
                window.show(ctx.as_ref(), |ui| {
                    lua.scope(|scope| {
                        let data = scope
                            .create_nonstatic_userdata::<super::Ui>(ui.into())
                            .unwrap();
                        let _: () = ui_callback.call(data).unwrap();
                        Ok(())
                    })
                    .unwrap();
                });
                Ok(())
            },
        );
    }
}
