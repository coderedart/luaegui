use std::sync::Arc;

use derive_more::*;
use tealr::mlu::*;

use crate::{add_method, wrapper};


wrapper!(TextShape egui::epaint::TextShape);
impl TealData for TextShape {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("egui docs link: https://docs.rs/epaint/latest/epaint/struct.TextShape.html#");
        methods.add_function("new", |_, (a0, a1) : (Pos2, Galley)| {
            Ok(Self(egui::epaint::TextShape::new(a0.into(), a1.0.clone())))
        });
        add_method!(methods, visual_bounding_rect, (), Rect);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("pos", |_, s| Ok(Pos2::from(s.pos)));
        fields.add_field_method_set("pos", |_, s, a0: Pos2| {
            s.pos = a0.into();
            Ok(())
        });
        fields.add_field_method_get("galley", |_, s| Ok(Galley::from(s.galley.clone())));
        fields.add_field_method_set("galley", |_, s, a0: Galley| {
            s.galley = a0.0.clone();
            Ok(())
        });
        fields.add_field_method_get("underline", |_, s| Ok(Stroke::from(s.underline)));
        fields.add_field_method_set("underline", |_, s, a0: Stroke| {
            s.underline = a0.into();
            Ok(())
        });
        fields.add_field_method_get("override_text_color", |_, s| Ok(s.override_text_color.map(Color32::from)));
        fields.add_field_method_set("override_text_color", |_, s, a0: Option<Color32>| {
            s.override_text_color = a0.map(|a| a.into());
            Ok(())
        });
        fields.add_field_method_get("angle", |_, s| Ok(s.angle));
        fields.add_field_method_set("angle", |_, s, a0: f32| {
            s.angle = a0;
            Ok(())
        });
    }
}


wrapper!(copy default Margin egui::style::Margin);
impl TealData for Margin {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type(
            "egui docs link: https://docs.rs/egui/latest/egui/style/struct.Margin.html",
        );
        methods.add_function("same", |_, a0: f32| {
            Ok(Margin(egui::style::Margin::same(a0)))
        });
        methods.add_function("symmetric", |_, (a0, a1): (f32, f32)| {
            Ok(Margin(egui::style::Margin::symmetric(a0, a1)))
        });
        add_method!(methods, sum, (), Vec2);
        add_method!(methods, left_top, (), Vec2);
        add_method!(methods, right_bottom, (), Vec2);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("left", |_, s| Ok(s.left));
        fields.add_field_method_set("left", |_, s, a0: f32| {
            s.left = a0;
            Ok(())
        });
        fields.add_field_method_get("right", |_, s| Ok(s.right));
        fields.add_field_method_set("right", |_, s, a0: f32| {
            s.right = a0;
            Ok(())
        });
        fields.add_field_method_get("top", |_, s| Ok(s.top));
        fields.add_field_method_set("top", |_, s, a0: f32| {
            s.top = a0;
            Ok(())
        });
        fields.add_field_method_get("bottom", |_, s| Ok(s.bottom));
        fields.add_field_method_set("bottom", |_, s, a0: f32| {
            s.bottom = a0;
            Ok(())
        });
    }
}
wrapper!(copy default Stroke egui::Stroke);
impl TealData for Stroke {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods
            .document_type("egui docs link: https://docs.rs/egui/latest/egui/struct.Stroke.html");
        methods.add_function("none", |_, ()| Ok(Self::from(egui::Stroke::none())));
        methods.add_function("new", |_, (a0, a1): (f32, Color32)| {
            Ok(Self::from(egui::Stroke::new(a0, a1)))
        });
        add_method!(methods, is_empty, (), bool);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("width", |_, s| Ok(s.width));
        fields.add_field_method_set("width", |_, s, a0: f32| {
            s.width = a0;
            Ok(())
        });
        fields.add_field_method_get("color", |_, s| Ok(Color32::from(s.color)));
        fields.add_field_method_set("color", |_, s, a0: Color32| {
            s.color = a0.into();
            Ok(())
        });
    }
}
wrapper!(copy default Rounding egui::Rounding);
impl TealData for Rounding {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("egui docs link: https://docs.rs/egui/latest/egui/struct.Stroke.html");
        methods.add_function("same", |_, a0: f32| Ok(Self::from(egui::Rounding::same(a0))));
        methods.add_function("none", |_, ()| Ok(Self::from(egui::Rounding::none())));
        add_method!(methods, is_same, (), bool);
        add_method!(methods, at_least, f32, Rounding);
        add_method!(methods, at_most, f32, Rounding);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("nw", |_, s| Ok(s.nw));
        fields.add_field_method_set("nw", |_, s, a0: f32| {
            s.nw = a0;
            Ok(())
        });
        fields.add_field_method_get("ne", |_, s| Ok(s.ne));
        fields.add_field_method_set("ne", |_, s, a0: f32| {
            s.ne = a0;
            Ok(())
        });
        fields.add_field_method_get("sw", |_, s| Ok(s.sw));
        fields.add_field_method_set("sw", |_, s, a0: f32| {
            s.sw = a0;
            Ok(())
        });
        fields.add_field_method_get("se", |_, s| Ok(s.se));
        fields.add_field_method_set("se", |_, s, a0: f32| {
            s.se = a0;
            Ok(())
        });
    }
}
wrapper!(Spacing egui::style::Spacing);
impl TealData for Spacing {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type(
            "egui docs link: https://docs.rs/egui/latest/egui/style/struct.Spacing.html",
        );
        add_method!(methods, icon_rectangles, Rect, Rect ; Rect);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("item_spacing", |_, s| Ok(Vec2::from(s.item_spacing)));
        fields.add_field_method_set("item_spacing", |_, s, a0: Vec2| {
            s.item_spacing = a0.into();
            Ok(())
        });
        fields.add_field_method_get("window_margin", |_, s| Ok(Margin::from(s.window_margin)));
        fields.add_field_method_set("window_margin", |_, s, a0: Margin| {
            s.window_margin = a0.into();
            Ok(())
        });
        fields.add_field_method_get("button_padding", |_, s| Ok(Vec2::from(s.button_padding)));
        fields.add_field_method_set("button_padding", |_, s, a0: Vec2| {
            s.button_padding = a0.into();
            Ok(())
        });
        fields.add_field_method_get("indent", |_, s| Ok(s.indent));
        fields.add_field_method_set("indent", |_, s, a0: f32| {
            s.indent = a0;
            Ok(())
        });
        fields.add_field_method_get("interact_size", |_, s| Ok(Vec2::from(s.interact_size)));
        fields.add_field_method_set("interact_size", |_, s, a0: Vec2| {
            s.interact_size = a0.into();
            Ok(())
        });
        fields.add_field_method_get("slider_width", |_, s| Ok(s.slider_width));
        fields.add_field_method_set("slider_width", |_, s, a0: f32| {
            s.slider_width = a0;
            Ok(())
        });
        fields.add_field_method_get("text_edit_width", |_, s| Ok(s.text_edit_width));
        fields.add_field_method_set("text_edit_width", |_, s, a0: f32| {
            s.text_edit_width = a0.into();
            Ok(())
        });
        fields.add_field_method_get("icon_width", |_, s| Ok(s.icon_width));
        fields.add_field_method_set("icon_width", |_, s, a0: f32| {
            s.icon_width = a0.into();
            Ok(())
        });
        fields.add_field_method_get("icon_width_inner", |_, s| Ok(s.icon_width_inner));
        fields.add_field_method_set("icon_width_inner", |_, s, a0: f32| {
            s.icon_width_inner = a0.into();
            Ok(())
        });
        fields.add_field_method_get("icon_spacing", |_, s| Ok(s.icon_spacing));
        fields.add_field_method_set("icon_spacing", |_, s, a0: f32| {
            s.icon_spacing = a0.into();
            Ok(())
        });
        fields.add_field_method_get("tooltip_width", |_, s| Ok(s.tooltip_width));
        fields.add_field_method_set("tooltip_width", |_, s, a0: f32| {
            s.tooltip_width = a0.into();
            Ok(())
        });
        fields.add_field_method_get("indent_ends_with_horizontal_line", |_, s| {
            Ok(s.indent_ends_with_horizontal_line)
        });
        fields.add_field_method_set("indent_ends_with_horizontal_line", |_, s, a0: bool| {
            s.indent_ends_with_horizontal_line = a0.into();
            Ok(())
        });
        fields.add_field_method_get("combo_height", |_, s| Ok(s.combo_height));
        fields.add_field_method_set("combo_height", |_, s, a0: f32| {
            s.combo_height = a0.into();
            Ok(())
        });
        fields.add_field_method_get("scroll_bar_width", |_, s| Ok(s.scroll_bar_width));
        fields.add_field_method_set("scroll_bar_width", |_, s, a0: f32| {
            s.scroll_bar_width = a0.into();
            Ok(())
        });
    }
}

wrapper!(Visuals egui::style::Visuals);
impl TealData for Visuals {}

wrapper!(TextStyle egui::TextStyle);
impl TealData for TextStyle {}

wrapper!(Painter egui::Painter);
impl TealData for Painter {}

wrapper!(Layout egui::Layout);
impl TealData for Layout {}

wrapper!(copy Rect egui::Rect);
impl TealData for Rect {}

wrapper!(copy LayerId egui::LayerId);
impl TealData for LayerId {}

wrapper!(copy default Color32 egui::Color32);

wrapper!(copy Id egui::Id);
impl TealData for Id {}

wrapper!(RichText egui::RichText);
impl TealData for RichText {}

wrapper!(WidgetText egui::WidgetText);
impl TealData for WidgetText {}

wrapper!(TextureId egui::TextureId);
impl TealData for TextureId {}

wrapper!(copy default Vec2 egui::Vec2);

wrapper!(copy default Pos2 egui::Pos2);
impl TealData for Pos2 {}
wrapper!(copy Sense egui::Sense);
impl TealData for Sense {}

wrapper!(copy default Align egui::Align);
impl TealData for Align {}

wrapper!(copy PointerButton egui::PointerButton);
impl TealData for PointerButton {}

wrapper!(copy default CursorIcon egui::CursorIcon);
impl TealData for CursorIcon {}

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

impl From<IntoWidgetText> for egui::WidgetText {
    fn from(into_widget_text: IntoWidgetText) -> Self {
        match into_widget_text {
            IntoWidgetText::String(s) => s.into(),
            IntoWidgetText::RichText(r) => r.into(),
            IntoWidgetText::WidgetText(wt) => wt.into(),
            IntoWidgetText::Galley(g) => g.into(),
        }
    }
}
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
impl From<RichText> for WidgetText {
    fn from(rt: RichText) -> Self {
        Self(rt.0.into())
    }
}
impl From<RichText> for egui::WidgetText {
    fn from(rt: RichText) -> Self {
        rt.0.into()
    }
}
impl From<egui::RichText> for WidgetText {
    fn from(rt: egui::RichText) -> Self {
        Self(rt.into())
    }
}
impl From<String> for WidgetText {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

impl From<Galley> for WidgetText {
    fn from(g: Galley) -> Self {
        Self(g.0.into())
    }
}
impl From<Galley> for egui::WidgetText {
    fn from(g: Galley) -> Self {
        g.0.into()
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
impl From<IntoRichText> for egui::RichText {
    fn from(into_rich_text: IntoRichText) -> Self {
        match into_rich_text {
            IntoRichText::String(s) => s.into(),
            IntoRichText::RichText(r) => r.into(),
        }
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

impl From<TextureHandle> for TextureId {
    fn from(th: TextureHandle) -> Self {
        th.id().into()
    }
}
tealr::create_union_mlua!(pub enum IntoTextureId = TextureId | TextureHandle);

impl From<IntoTextureId> for TextureId {
    fn from(into_texture_id: IntoTextureId) -> Self {
        match into_texture_id {
            IntoTextureId::TextureId(tid) => tid,
            IntoTextureId::TextureHandle(th) => th.into(),
        }
    }
}
impl From<IntoTextureId> for egui::TextureId {
    fn from(into_texture_id: IntoTextureId) -> Self {
        match into_texture_id {
            IntoTextureId::TextureId(tid) => tid.into(),
            IntoTextureId::TextureHandle(th) => th.into(),
        }
    }
}
impl From<TextureHandle> for egui::TextureId {
    fn from(t: TextureHandle) -> Self {
        t.into()
    }
}

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
impl From<egui::Pos2> for Vec2 {
    #[inline(always)]
    fn from(v: egui::Pos2) -> Self {
        Self(egui::vec2(v.x, v.y))
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

#[derive(Clone, AsRef, AsMut, Deref, DerefMut, tealr::MluaTealDerive)]
pub struct Style(pub Arc<egui::Style>);
impl TealData for Style {}

impl From<Style> for egui::Style {
    fn from(s: Style) -> Self {
        (*s.0).clone()
    }
}
impl From<egui::Style> for Style {
    fn from(s: egui::Style) -> Self {
        Self(Arc::new(s))
    }
}
impl From<&Arc<egui::Style>> for Style {
    fn from(s: &Arc<egui::Style>) -> Self {
        Self(s.clone())
    }
}
impl From<Style> for Arc<egui::Style> {
    fn from(s: Style) -> Self {
        s.0
    }
}

tealr::create_union_mlua!(pub Derives(Debug, Hash) enum IntoIdSource = String | i64 | bool);
