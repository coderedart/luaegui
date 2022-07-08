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
        mlua::{self, Lua, UserDataMethods},
        TealData, UserDataWrapper,
    },
    MluaTealDerive, TypeBody, TypeWalker,
};
pub use ui::*;
pub use widget::*;

use derive_more::*;
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
    egui_proxy.set("ui_docs", lua.create_proxy::<UiMutRef>()?)?;
    egui_proxy.set("widget_text", lua.create_proxy::<WidgetText>()?)?;
    egui_proxy.set("vec2", lua.create_proxy::<Vec2>()?)?;
    egui_proxy.set("window", lua.create_proxy::<Window>()?)?;
    lua.globals().set("Egui", egui_proxy)?;
    Ok(())
}

#[macro_export]
macro_rules! lua_registry_scoped_ui {
    ( $lua:expr, $from_ui:expr, |$ui:ident| $code:expr) => {{
        use $crate::ui::Ui;
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
        use $crate::ui::UiMutRef;
        let key = $lua
            .scope(|scope| {
                let $ui = scope
                    .create_nonstatic_userdata(UiMutRef::from($from_ui))
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
        .process_type::<Align>()
        .process_type::<Align2>()
        .process_type::<CircleShape>()
        .process_type::<ClippedPrimitive>()
        .process_type::<ClippedShape>()
        .process_type::<Color32>()
        .process_type::<Context>()
        .process_type::<CubicBezierShape>()
        .process_type::<CursorIcon>()
        .process_type::<FontFamily>()
        .process_type::<FontId>()
        .process_type::<Frame>()
        .process_type::<Galley>()
        .process_type::<Id>()
        .process_type::<Interaction>()
        .process_type::<LayerId>()
        .process_type::<Layout>()
        .process_type::<Margin>()
        .process_type::<Mesh>()
        .process_type::<Painter>()
        .process_type::<PathShape>()
        .process_type::<PointerButton>()
        .process_type::<Pos2>()
        .process_type::<Primitive>()
        .process_type::<Rect>()
        .process_type::<RectShape>()
        .process_type::<RectTransform>()
        .process_type::<Response>()
        .process_type::<RichText>()
        .process_type::<Rounding>()
        .process_type::<Selection>()
        .process_type::<Sense>()
        .process_type::<Shadow>()
        .process_type::<Shape>()
        .process_type::<Spacing>()
        .process_type::<Style>()
        .process_type::<Stroke>()
        .process_type::<TextShape>()
        .process_type::<TextStyle>()
        .process_type::<TextureHandle>()
        .process_type::<TextureId>()
        .process_type::<UiMutRef>()
        .process_type::<Vec2>()
        .process_type::<Visuals>()
        .process_type::<WidgetText>()
        .process_type::<WidgetVisuals>()
        .process_type::<Window>()
}
#[macro_export]
macro_rules! wrapper_from_impl {
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
    };
}

// #[macro_export]
// macro_rules! from_impl {
//     ($name:ident $etype:path) => {
//         impl From<$name> for $etype {
//             fn from(x: $name) -> Self {
//                 x.0
//             }
//         }
//         impl From<&$name> for $etype {
//             fn from(x: &$name) -> Self {
//                 x.clone().0
//             }
//         }
//         impl From<$etype> for $name {
//             fn from(x: $etype) -> Self {
//                 Self(x)
//             }
//         }
//         impl From<&$etype> for $name {
//             fn from(x: &$etype) -> Self {
//                 Self(x.clone())
//             }
//         }
//     };
// }

impl<T> TypeBody for Wrapper<T>
where
    Wrapper<T>: 'static + tealr::TypeName + tealr::mlu::TealData,
{
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as tealr::mlu::TealData>::add_fields(&mut gen);
        <Self as tealr::mlu::TealData>::add_methods(&mut gen);
        gen.into()
    }
}
// impl From<Vec2> for egui::Vec2 {
//     fn from(v: Vec2) -> Self {
//         v.0
//     }
// }
impl<T> tealr::mlu::mlua::UserData for Wrapper<T>
where
    Wrapper<T>: TealData,
{
    fn add_methods<'lua, U: UserDataMethods<'lua, Self>>(methods: &mut U) {
        let mut x = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut x);
    }
    fn add_fields<'lua, F: ::tealr::mlu::mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}
#[derive(Clone, Copy, Default, AsRef, AsMut, Deref, DerefMut)]
pub struct Wrapper<T>(T);

impl<T> Wrapper<T> {
    pub fn into<U>(self) -> U
    where
        T: Into<U>,
    {
        self.0.into()
    }
}
impl<T> From<T> for Wrapper<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

#[macro_export]
macro_rules! wrapper {
    ( $name:ident  $etype:path) => {

        pub type $name = $crate::Wrapper<$etype>;

        impl tealr::TypeName for $name {
            fn get_type_parts() -> std::borrow::Cow<'static, [tealr::NamePart]> {
                tealr::new_type!($name)
            }
        }

        $crate::wrapper_from_impl!($name $etype);
    };

}
#[macro_export]
macro_rules! add_fields {
    ($fields:ident, $($field_name:ident : $field_type:ty),*) => {
        $(
        $fields.add_field_method_get(stringify!($field_name), |_, s| {
            Ok(<$field_type>::from(s.$field_name.clone()))
        });
        $fields.add_field_method_set(stringify!($field_name), |_, s, a0: $field_type| {
            s.$field_name = a0.into();
            Ok(())
        });
    )*
    };
}
