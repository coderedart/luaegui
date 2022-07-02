use std::sync::Arc;

use derive_more::*;
use tealr::mlu::*;
#[derive(Copy, Clone, From, AsRef, AsMut, Deref, Default, Debug, tealr::MluaTealDerive)]
pub struct Color32(pub egui::Color32);

impl TealData for Color32 {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function("default", |_, ()| Ok(Color32::default()));
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("r", |_, color| Ok(color.r()));
        fields.add_field_method_get("g", |_, color| Ok(color.g()));
        fields.add_field_method_get("b", |_, color| Ok(color.b()));
        fields.add_field_method_get("a", |_, color| Ok(color.a()));
    }
}

tealr::create_union_mlua!(pub enum IntoWidgetText =  String | RichText | WidgetText | Galley);

impl From<IntoWidgetText> for WidgetText {
    fn from(into_widget_text: IntoWidgetText) -> Self {
        match into_widget_text {
            IntoWidgetText::String(s) => s.into(),
            IntoWidgetText::RichText(r) => r.into(),
            IntoWidgetText::WidgetText(wt) => wt,
            IntoWidgetText::Galley(g) => g.into(),
        }
    }
}

#[derive(Clone, AsRef, AsMut, Deref, Default, tealr::MluaTealDerive)]
pub struct WidgetText(egui::WidgetText);
impl TealData for WidgetText {}
impl From<RichText> for WidgetText {
    fn from(rt: RichText) -> Self {
        Self(rt.0.into())
    }
}
impl From<Galley> for WidgetText {
    fn from(g: Galley) -> Self {
        Self(g.0.into())
    }
}
impl<T> From<T> for WidgetText
where
    T: Into<egui::WidgetText>,
{
    fn from(t: T) -> Self {
        Self(t.into())
    }
}
#[derive(Clone, AsRef, AsMut, Deref, tealr::MluaTealDerive)]
pub struct Galley(Arc<egui::Galley>);
impl TealData for Galley {}
impl<T> From<T> for Galley
where
    T: Into<Arc<egui::Galley>>,
{
    fn from(t: T) -> Self {
        Self(t.into())
    }
}

tealr::create_union_mlua!(pub enum IntoRichText = String | RichText );
impl From<IntoRichText> for RichText {
    fn from(into_rich_text: IntoRichText) -> Self {
        match into_rich_text {
            IntoRichText::String(s) => s.into(),
            IntoRichText::RichText(r) => r.into(),
        }
    }
}
#[derive(Clone, AsRef, AsMut, Deref, Default, tealr::MluaTealDerive)]
pub struct RichText(egui::RichText);

impl TealData for RichText {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(_methods: &mut T) {
        // methods.add_function("new", function)
    }
}

impl From<RichText> for egui::RichText {
    fn from(rt: RichText) -> Self {
        rt.0
    }
}
impl From<egui::RichText> for RichText {
    fn from(rt: egui::RichText) -> Self {
        Self(rt)
    }
}
impl From<String> for RichText {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}
