use super::Context;
use super::Ui;
pub struct Window<'a>(egui::Window<'a>);

impl<'a> tealr::mlu::mlua::UserData for Window<'a> {
    fn add_fields<'lua, F: tealr::mlu::mlua::UserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: tealr::mlu::mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "show",
            |lua,
             args: (
                tealr::mlu::mlua::AnyUserData,
                Context,
                tealr::mlu::mlua::Function,
            )| {
                let window = args
                    .0
                    .take::<Window<'_>>()
                    .expect("failed to get Window from anyuserdata");
                window.0.show(args.1.as_ref(), |ui| {
                    lua.scope(|scope| {
                        let data = scope.create_nonstatic_userdata::<Ui>(ui.into()).unwrap();
                        args.2.call::<_, ()>(data).unwrap();
                        Ok(())
                    })
                    .unwrap();
                });
                Ok(())
            },
        )
    }
}
impl<'open> From<egui::Window<'open>> for Window<'open> {
    fn from(window: egui::Window<'open>) -> Self {
        Self(window)
    }
}
