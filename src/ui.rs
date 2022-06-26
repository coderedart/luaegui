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
