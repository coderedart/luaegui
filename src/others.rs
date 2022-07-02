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
pub struct WidgetText(pub egui::WidgetText);
impl TealData for WidgetText {}
impl From<RichText> for WidgetText {
    fn from(rt: RichText) -> Self {
        Self(rt.0.into())
    }
}
impl From<String> for WidgetText {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}
impl From<WidgetText> for egui::WidgetText {
    fn from(wt: WidgetText) -> Self {
        wt.0
    }
}
impl From<Galley> for WidgetText {
    fn from(g: Galley) -> Self {
        Self(g.0.into())
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
            IntoRichText::RichText(r) => r,
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

#[derive(Clone, AsRef, AsMut, Deref, tealr::MluaTealDerive)]
pub struct TextureHandle(egui::TextureHandle);
impl TealData for TextureHandle {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("name", |_, th, ()| Ok(th.name()));
        methods.add_method("aspect_ratio", |_, th, ()| Ok(th.aspect_ratio()));
        methods.add_method("size", |_, th, ()| Ok(th.size()));
        methods.add_method("id", |_, th, ()| Ok(TextureId(th.id())));
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(_fields: &mut F) {}
}

#[derive(Clone, AsRef, AsMut, Deref, tealr::MluaTealDerive)]
pub struct TextureId(egui::TextureId);
impl From<egui::TextureId> for TextureId {
    fn from(id: egui::TextureId) -> Self {
        Self(id)
    }
}
impl From<TextureId> for egui::TextureId {
    fn from(id: TextureId) -> Self {
        id.0
    }
}
impl From<TextureHandle> for TextureId {
    fn from(th: TextureHandle) -> Self {
        th.id().into()
    }
}
impl TealData for TextureId {}
tealr::create_union_mlua!(pub enum IntoTextureId = TextureId | TextureHandle);

impl From<IntoTextureId> for TextureId {
    fn from(into_texture_id: IntoTextureId) -> Self {
        match into_texture_id {
            IntoTextureId::TextureId(tid) => tid,
            IntoTextureId::TextureHandle(th) => th.into(),
        }
    }
}

#[derive(Clone, Copy, AsRef, Default, AsMut, Deref, DerefMut, tealr::MluaTealDerive)]
pub struct Vec2(egui::Vec2);
impl TealData for Vec2 {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function("default", |_, ()| Ok(Self::default()));
        methods.add_function("new", |_, (x, y): (f32, f32)| Ok(Self::from((x, y))));
        methods.add_function("splat", |_, f: f32| Ok(Self(egui::Vec2::splat(f))));
        methods.add_method("normalized", |_, v, ()| Ok(Self(v.normalized())));
        methods.add_method("rot90", |_, v, ()| Ok(Self(v.rot90())));
        methods.add_method("length", |_, v, ()| Ok(v.length()));
        methods.add_method("length_sq", |_, v, ()| Ok(v.length_sq()));
        methods.add_method("angle", |_, v, ()| Ok(v.angle()));
        methods.add_function("angled", |_, a: f32| Ok(Self(egui::Vec2::angled(a))));
        methods.add_method("floor", |_, v, ()| Ok(Self(v.floor())));
        methods.add_method("round", |_, v, ()| Ok(Self(v.round())));
        methods.add_method("ceil", |_, v, ()| Ok(Self(v.ceil())));
        methods.add_method("abs", |_, v, ()| Ok(Self(v.abs())));
        methods.add_method("is_finite", |_, v, ()| Ok(v.is_finite()));
        methods.add_method("any_nan", |_, v, ()| Ok(v.any_nan()));
        methods.add_method("min", |_, v, o: Self| Ok(Self(v.min(*o))));
        methods.add_method("max", |_, v, o: Self| Ok(Self(v.max(*o))));
        methods.add_method("dot", |_, v, o: Self| Ok(v.dot(*o)));
        methods.add_method("min_elem", |_, v, ()| Ok(v.min_elem()));
        methods.add_method("max_elem", |_, v, ()| Ok(v.max_elem()));
        methods.add_method("clamp", |_, v, (min, max): (Self, Self)| {
            Ok(Self(v.clamp(*min, *max)))
        });
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, v| Ok(v.x));
        fields.add_field_method_get("y", |_, v| Ok(v.y));
        fields.add_field_method_set("x", |_, v, x: f32| {
            v.x = x;
            Ok(())
        });
        fields.add_field_method_set("y", |_, v, y: f32| {
            v.y = y;
            Ok(())
        });
    }
}

impl From<[f32; 2]> for Vec2 {
    #[inline(always)]
    fn from(v: [f32; 2]) -> Self {
        Self(v.into())
    }
}

impl From<&Vec2> for [f32; 2] {
    #[inline(always)]
    fn from(v: &Vec2) -> Self {
        [v.x, v.y]
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline(always)]
    fn from(v: (f32, f32)) -> Self {
        Self(v.into())
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline(always)]
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}

impl From<&Vec2> for (f32, f32) {
    #[inline(always)]
    fn from(v: &Vec2) -> Self {
        (v.x, v.y)
    }
}
impl From<Vec2> for egui::Vec2 {
    fn from(v: Vec2) -> Self {
        v.0
    }
}
