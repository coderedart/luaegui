use std::sync::Arc;

use derive_more::*;
use tealr::mlu::*;

use crate::{add_fields, add_method, wrapper, add_method_mut};
wrapper!(Shape egui::epaint::Shape);
impl TealData for Shape {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function("line_segment", |_, (a0, a1): ([Pos2; 2], Stroke)| {
            Ok(Shape::from(egui::epaint::Shape::line_segment(
                [a0[0].into(), a0[1].into()],
                a1,
            )))
        });
        methods.add_function("line_segment", |_, (a0, a1): ([Pos2; 2], Stroke)| {
            Ok(Shape::from(egui::epaint::Shape::line_segment(
                [a0[0].into(), a0[1].into()],
                a1,
            )))
        });{
            fn line_segment(_: &mlua::Lua, (a0, a1) : ([Pos2; 2],  Stroke)) -> Result<Shape, mlua::Error> {
                Ok(Shape::from(egui::epaint::Shape::line_segment(
                    [a0[0].into(), a0[1].into()],
                    a1,
                )))
            }
            methods.add_function("line_segment", line_segment);
        }
        
        methods.add_function("line_segment", |_, (a0, a1): ([Pos2; 2], Stroke)| {
            Ok(Shape::from(egui::epaint::Shape::line_segment(
                [a0[0].into(), a0[1].into()],
                a1,
            )))
        });
        methods.add_function("line_segment", |_, (a0, a1): ([Pos2; 2], Stroke)| {
            Ok(Shape::from(egui::epaint::Shape::line_segment(
                [a0[0].into(), a0[1].into()],
                a1,
            )))
        });

        add_method!(methods, texture_id, (), TextureId);
        add_method_mut!(methods, translate, Vec2);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(_fields: &mut F) {}
}

wrapper!(copy CircleShape egui::epaint::CircleShape);
impl TealData for CircleShape {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        // filled and stroke
        add_method!(methods, visual_bounding_rect, (), Rect);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(fields,
        center: Pos2,
        radius : f32,
        fill: Color32,
        stroke: Stroke
        );
    }
}

wrapper!(PathShape egui::epaint::PathShape);
impl TealData for PathShape {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        // closed line, convexpolygon, line
        add_method!(methods, visual_bounding_rect, (), Rect);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(fields,
        closed: bool,
        fill: Color32,
        stroke: Stroke
        );
        fields.add_field_method_get("points", |_, s| {
            Ok(s.points.iter().map(Pos2::from).collect::<Vec<_>>())
        })
    }
}


wrapper!(copy RectShape egui::epaint::RectShape);
impl TealData for RectShape {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(_methods: &mut T) {}

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(
            fields,
            rect: Rect,
            rounding: Rounding,
            fill: Color32,
            stroke: Stroke
        );
    }
}

wrapper!(TextShape egui::epaint::TextShape);
impl TealData for TextShape {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type(
            "egui docs link: https://docs.rs/epaint/latest/epaint/struct.TextShape.html#",
        );
        methods.add_function("new", |_, (a0, a1): (Pos2, Galley)| {
            Ok(Self(egui::epaint::TextShape::new(a0.into(), a1.0.clone())))
        });
        add_method!(methods, visual_bounding_rect, (), Rect);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(fields, pos: Pos2, underline: Stroke, angle: f32);
        fields.add_field_method_get("galley", |_, s| Ok(Galley::from(s.galley.clone())));
        fields.add_field_method_set("galley", |_, s, a0: Galley| {
            s.galley = a0.0.clone();
            Ok(())
        });

        fields.add_field_method_get("override_text_color", |_, s| {
            Ok(s.override_text_color.map(Color32::from))
        });
        fields.add_field_method_set("override_text_color", |_, s, a0: Option<Color32>| {
            s.override_text_color = a0.map(|a| a.into());
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
        add_fields!(fields, left: f32, right: f32, top: f32, bottom: f32);
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
        add_fields!(fields, width: f32, color: Color32);
    }
}
wrapper!(copy default Rounding egui::Rounding);
impl TealData for Rounding {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods
            .document_type("egui docs link: https://docs.rs/egui/latest/egui/struct.Stroke.html");
        methods.add_function("same", |_, a0: f32| {
            Ok(Self::from(egui::Rounding::same(a0)))
        });
        methods.add_function("none", |_, ()| Ok(Self::from(egui::Rounding::none())));
        add_method!(methods, is_same, (), bool);
        add_method!(methods, at_least, f32, Rounding);
        add_method!(methods, at_most, f32, Rounding);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(fields, nw: f32, ne: f32, sw: f32, se: f32);
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
        add_fields!(
            fields,
            item_spacing: Vec2,
            window_margin: Margin,
            button_padding: Vec2,
            indent: f32,
            interact_size: Vec2,
            slider_width: f32,
            text_edit_width: f32,
            icon_width: f32,
            icon_width_inner: f32,
            icon_spacing: f32,
            tooltip_width: f32,
            indent_ends_with_horizontal_line: bool,
            combo_height: f32,
            scroll_bar_width: f32
        );
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
        add_fields!(fields, x: f32, y: f32);
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
