mod container;
mod context;
mod response;
mod ui;
mod widget;
pub use container::*;
pub use context::*;
pub use egui;
pub use response::*;
pub use ui::*;
pub use widget::*;
pub fn register_egui_lua_bindings() {}

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
