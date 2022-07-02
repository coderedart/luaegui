mod container;
mod context;
mod others;
mod response;
mod ui;
mod widget;
pub use container::*;
pub use context::*;
pub use egui;
pub use others::*;
pub use response::*;
use tealr::{
    mlu::{
        mlua::{self, Lua},
        TealData,
    },
    MluaTealDerive,
};
pub use ui::*;
pub use widget::*;

#[derive(Clone, Default, MluaTealDerive)]
pub struct EguiProxy;
impl TealData for EguiProxy {
    fn add_methods<'lua, T: tealr::mlu::TealDataMethods<'lua, Self>>(_methods: &mut T) {}

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(_fields: &mut F) {}
}
pub fn register_egui_lua_bindings(_lua: &Lua) {}

#[macro_export]
macro_rules! lua_registry_scoped_ui {
    ( $lua:expr, $from_ui:expr, |$ui:ident| $code:expr) => {{
        use crate::ui::Ui;
        $lua.scope(|scope| {
            let $ui = scope.create_nonstatic_userdata(Ui::from($from_ui))?;
            let response: MultiValue = $code?;
            $lua.create_registry_value(response.into_vec())
        })
    }};
}

#[macro_export]
macro_rules! lua_registry_scoped_ui_extract {
    ( $lua:expr, $from_ui:expr, |$ui:ident| $code:expr) => {{
        use crate::ui::Ui;
        let key = $lua
            .scope(|scope| {
                let $ui = scope
                    .create_nonstatic_userdata(Ui::from($from_ui))
                    .expect("failed to create non static userdata with Ui");
                let response: MultiValue = $code.expect("ui function returned error");
                $lua.create_registry_value(response.into_vec())
            })
            .expect("failed to get registry key");

        let value: Vec<Value> = $lua
            .registry_value(&key)
            .expect("failed to get registry value");
        $lua.remove_registry_value(key)
            .expect("failed to remove registry value");
        MultiValue::from_vec(value)
    }};
}
