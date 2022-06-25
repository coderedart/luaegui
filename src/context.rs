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
            "window",
            |lua, ctx, ui_function: tealr::mlu::mlua::Function| {
                egui::Window::new("lua window").show(ctx.as_ref(), |ui| {
                    lua.scope(|scope| {
                        let data = scope.create_nonstatic_userdata::<Ui>(ui.into()).unwrap();
                        lua.globals().set("ui", data).unwrap();
                        let _: () = ui_function.call(()).unwrap();

                        Ok(())
                    })
                    .unwrap();
                });

                Ok(())
            },
        );
    }
}

pub struct Ui<'ui>(&'ui mut egui::Ui);
impl<'ui> From<&'ui mut egui::Ui> for Ui<'ui> {
    fn from(ui: &'ui mut egui::Ui) -> Self {
        Self(ui)
    }
}
impl<'ui> tealr::mlu::mlua::UserData for Ui<'ui> {
    fn add_fields<'lua, F: tealr::mlu::mlua::UserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: tealr::mlu::mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("label", |_, ui, text: String| {
            ui.0.label(&text);
            Ok(())
        });
    }
}
pub struct Window<'a>(egui::Window<'a>);

impl<'a> tealr::mlu::mlua::UserData for Window<'a> {
    fn add_fields<'lua, F: tealr::mlu::mlua::UserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: tealr::mlu::mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
