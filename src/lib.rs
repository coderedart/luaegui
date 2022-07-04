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
    MluaTealDerive, TypeWalker,
};
pub use ui::*;
pub use widget::*;

#[derive(Clone, Default, MluaTealDerive)]
pub struct EguiProxy;
impl TealData for EguiProxy {}
pub fn register_egui_lua_bindings(lua: &Lua) -> Result<(), mlua::Error> {
    let egui_proxy = lua.create_table()?;

    egui_proxy.set("color32", lua.create_proxy::<Color32>()?)?;
    egui_proxy.set("ctx", lua.create_proxy::<Context>()?)?;
    egui_proxy.set("galley", lua.create_proxy::<Galley>()?)?;
    egui_proxy.set("response", lua.create_proxy::<Response>()?)?;
    egui_proxy.set("rich_text", lua.create_proxy::<RichText>()?)?;
    egui_proxy.set("ui_docs", lua.create_proxy::<Ui>()?)?;
    egui_proxy.set("widget_text", lua.create_proxy::<WidgetText>()?)?;
    lua.globals().set("Egui", egui_proxy)?;
    Ok(())
}

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

pub fn get_all_types() -> TypeWalker {
    tealr::TypeWalker::new()
        .process_type::<Ui>()
        .process_type::<Context>()
        .process_type::<Response>()
        .process_type::<Color32>()
        .process_type::<RichText>()
        .process_type::<WidgetText>()
        .process_type::<Galley>()
}

#[macro_export]
macro_rules! from_impl {
    ($name:ident $etype:path) => {
        impl From<$name> for $etype {
            fn from(x: $name) -> Self {
                x.0
            }
        }
        impl From<&$name> for $etype {
            fn from(x: &$name) -> Self {
                x.clone().0
            }
        }
        impl From<$etype> for $name {
            fn from(x: $etype) -> Self {
                Self(x)
            }
        }
        impl From<&$etype> for $name {
            fn from(x: &$etype) -> Self {
                Self(x.clone())
            }
        }
    };
}

#[macro_export]
macro_rules! wrapper {
    ( $name:ident  $etype:path) => {
        #[derive(Clone, AsRef, AsMut, Deref, DerefMut, tealr::MluaTealDerive)]
        pub struct $name(pub $etype);

        $crate::from_impl!($name $etype);
    };
    ( copy $name:ident  $etype:path) => {
        #[derive(Clone, Copy, AsRef, AsMut, Deref, DerefMut, tealr::MluaTealDerive)]
        pub struct $name(pub $etype);

        $crate::from_impl!($name $etype);
    };
    ( default $name:ident  $etype:path) => {
        #[derive(Clone, Default, AsRef, AsMut, Deref, DerefMut, tealr::MluaTealDerive)]
        pub struct $name(pub $etype);

        $crate::from_impl!($name $etype);
    };
    ( copy default $name:ident  $etype:path) => {
        #[derive(Clone, Default, Copy, AsRef, AsMut, Deref, DerefMut, tealr::MluaTealDerive)]
        pub struct $name(pub $etype);

        $crate::from_impl!($name $etype);
    };

}
