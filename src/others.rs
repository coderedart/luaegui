use std::sync::Arc;

use derive_more::*;
use luaegui_derive::wrap_method;
use tealr::{
    mlu::{
        mlua::{Function, Lua, Result},
        *,
    },
    new_type,
};

use crate::{add_fields, wrapper};
wrapper!(Shape egui::epaint::Shape);
impl TealData for Shape {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        type InnerType = egui::epaint::Shape;

        methods.add_function("line_segment", |_, (a0, a1): ([Pos2; 2], Stroke)| {
            Ok(Shape::from(InnerType::line_segment(
                [a0[0].into(), a0[1].into()],
                a1,
            )))
        });
        methods.add_function("line", |_, (a0, a1): (Vec<Pos2>, Stroke)| {
            let a0 = a0.into_iter().map(|p| p.0).collect();
            Ok(Shape::from(InnerType::line(a0, a1)))
        });

        methods.add_function("closed_line", |_, (a0, a1): (Vec<Pos2>, Stroke)| {
            let a0 = a0.into_iter().map(|p| p.0).collect();
            Ok(Shape::from(InnerType::closed_line(a0, a1)))
        });
        methods.add_function("dotted_line", |_, (a0, a1): (Vec<Pos2>, Stroke)| {
            let a0 = a0.into_iter().map(|p| p.0).collect();
            Ok(Shape::from(InnerType::line(a0, a1)))
        });
        methods.add_function(
            "dashed_line",
            |_, (a0, a1, a2, a3): (Vec<Pos2>, Stroke, f32, f32)| {
                let a0: Vec<_> = a0.into_iter().map(|p| p.0).collect();
                let result: Vec<_> = InnerType::dashed_line(&a0, a1, a2, a3)
                    .into_iter()
                    .map(Shape::from)
                    .collect();
                Ok(result)
            },
        );
        // TODO: dashed_line_many
        methods.add_function(
            "convex_polygon",
            |_, (a0, a1, a2): (Vec<Pos2>, Color32, Stroke)| {
                let a0 = a0.into_iter().map(|p| p.0).collect();
                Ok(Shape::from(InnerType::convex_polygon(a0, a1, a2)))
            },
        );
        wrap_method!(f; circle_filled; Pos2, f32, Color32 nointo; Shape);
        wrap_method!(f; circle_stroke; Pos2, f32, Stroke nointo; Shape);
        wrap_method!(f; rect_filled; Rect, Rounding nointo, Color32 nointo; Shape);
        wrap_method!(f; rect_stroke; Rect, Rounding nointo, Stroke nointo; Shape);
        // TODO: text can't be done because `Fonts` is not Clone yet
        wrap_method!(f; galley; Pos2, Galley; Shape);
        wrap_method!(f; galley_with_color; Pos2, Galley, Color32; Shape);
        wrap_method!(f; mesh; Mesh; Shape);
        wrap_method!(f; image; TextureId, Rect, Rect, Color32; Shape);
        wrap_method!(m; visual_bounding_rect;;Rect);
        wrap_method!(m; texture_id;; TextureId);
        wrap_method!(mm; translate; Vec2);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(_fields: &mut F) {}
}

wrapper!(Mesh egui::Mesh);
impl TealData for Mesh {}
// wrapper!(Fonts egui::epaint::Fonts);
// impl TealData for Fonts {

// }
wrapper!( CircleShape egui::epaint::CircleShape);
impl TealData for CircleShape {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        type InnerType = egui::epaint::CircleShape;

        wrap_method!(f; filled; Pos2, f32, Color32 nointo; CircleShape);
        wrap_method!(f; stroke; Pos2, f32, Stroke nointo; CircleShape);
        wrap_method!(m; visual_bounding_rect;; Rect);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(
            fields,
            center: Pos2,
            radius: f32,
            fill: Color32,
            stroke: Stroke
        );
    }
}
wrapper!(ClippedPrimitive egui::epaint::ClippedPrimitive);
impl TealData for ClippedPrimitive {
    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(fields, clip_rect: Rect, primitive: Primitive);
    }

    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function("new", |_, args: (Rect, Primitive)| {
            Ok(Self(egui::epaint::ClippedPrimitive {
                clip_rect: args.0.into(),
                primitive: args.1.into(),
            }))
        })
    }
}
wrapper!(ClippedShape egui::epaint::ClippedShape);
impl TealData for ClippedShape {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function("new", |_, args: (Rect, Shape)| {
            Ok(Self(egui::epaint::ClippedShape(
                args.0.into(),
                args.1.into(),
            )))
        })
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("rect", |_, self_ref| Ok(Rect::from(self_ref.0 .0)));
        fields.add_field_method_set("rect", |_, self_ref, a0: Rect| {
            self_ref.0 .0 = a0.into();
            Ok(())
        });

        fields.add_field_method_set("rect", |_, self_ref, a0: Shape| {
            self_ref.0 .1 = a0.into();
            Ok(())
        });
        fields.add_field_method_get("shape", |_, self_ref| {
            Ok(Shape::from(self_ref.0 .1.clone()))
        });
    }
}
wrapper!(Primitive egui::epaint::Primitive);
impl TealData for Primitive {}
wrapper!(PathShape egui::epaint::PathShape);
impl TealData for PathShape {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        // closed line, convexpolygon, line
        wrap_method!(m; visual_bounding_rect;; Rect);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(fields, closed: bool, fill: Color32, stroke: Stroke);
        fields.add_field_method_get("points", |_, s| {
            Ok(s.points.iter().copied().map(Pos2::from).collect::<Vec<_>>())
        })
    }
}

wrapper!(RectTransform egui::emath::RectTransform);
impl TealData for RectTransform {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        type InnerType = egui::emath::RectTransform;

        wrap_method!(f; identity; Rect; RectTransform);
        wrap_method!(f; from_to;Rect, Rect; RectTransform);
        wrap_method!(m; from; ; Rect);
        wrap_method!(m; to; ; Rect);
        wrap_method!(m; scale; ; Vec2);
        wrap_method!(m; inverse; ; RectTransform);
        wrap_method!(m; transform_pos; Pos2 ; Pos2);
        wrap_method!(m; transform_rect; Rect; Rect);
        wrap_method!(m; transform_pos_clamped; Pos2 ; Pos2);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(_fields: &mut F) {}
}

wrapper!(CubicBezierShape egui::epaint::CubicBezierShape);
impl TealData for CubicBezierShape {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        type InnerType = egui::epaint::CubicBezierShape;
        methods.add_function(
            "from_points_stroke",
            |_, args: ([Pos2; 4], bool, Color32, Stroke)| {
                let a0 = args.0;
                let a0 = [a0[0].into(), a0[1].into(), a0[2].into(), a0[3].into()];
                let cbs = InnerType::from_points_stroke(a0, args.1, args.2.into(), args.3);
                Ok(CubicBezierShape::from(cbs))
            },
        );
        wrap_method!(m; transform; RectTransform asref; CubicBezierShape);
        methods.add_method(
            "to_path_shapes",
            |_, self_ref, (a0, a1): (Option<f32>, Option<f32>)| {
                let result: Vec<PathShape> = self_ref
                    .to_path_shapes(a0, a1)
                    .into_iter()
                    .map(Into::into)
                    .collect();
                Ok(result)
            },
        );
        wrap_method!(m; visual_bounding_rect; ; Rect);
        wrap_method!(m; logical_bounding_rect; ; Rect);
        methods.add_method("split_range", |_, self_ref, args: (f32, f32)| {
            Ok(CubicBezierShape::from(self_ref.split_range(args.0..args.1)))
        });
        wrap_method!(m; num_quadratics; f32; u32);
        methods.add_method("find_cross_t", |_, self_ref, args: f32| {
            Ok(self_ref.find_cross_t(args))
        });
        wrap_method!(m; sample; f32; Pos2);
        methods.add_method("flatten", |_, self_ref, args: Option<f32>| {
            let result: Vec<Pos2> = self_ref.flatten(args).into_iter().map(Into::into).collect();
            Ok(result)
        });
        methods.add_method(
            "flatten_closed",
            |_, self_ref, args: (Option<f32>, Option<f32>)| {
                let result: Vec<Vec<Pos2>> = self_ref
                    .flatten_closed(args.0, args.1)
                    .into_iter()
                    .map(|v| v.into_iter().map(Into::into).collect())
                    .collect();
                Ok(result)
            },
        );
        methods.add_method(
            "for_each_flattened_with_t",
            |_, self_ref, args: (f32, Function)| {
                self_ref.for_each_flattened_with_t(args.0, &mut |a0, a1| {
                    args.1
                        .call::<_, ()>((Pos2::from(a0), a1))
                        .expect("callback to for_each_flattened_with_t returned error");
                });
                Ok(())
            },
        );
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(fields, closed: bool, fill: Color32, stroke: Stroke);
        fields.add_field_method_set("points", |_, self_ref, a0: [Pos2; 4]| {
            self_ref.points = [a0[0].into(), a0[1].into(), a0[2].into(), a0[3].into()];
            Ok(())
        });
        fields.add_field_method_get("points", |_, self_ref| {
            let a0 = self_ref.points;
            Ok([Pos2::from(a0[0]), a0[1].into(), a0[2].into(), a0[3].into()])
        });
    }
}
wrapper!(FontId egui::FontId);
impl TealData for FontId {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        type InnerType = egui::FontId;
        wrap_method!(f; new; f32, FontFamily; FontId);
        wrap_method!(f; proportional; f32; FontId);
        wrap_method!(f; monospace; f32; FontId);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(fields, size: f32, family: FontFamily);
    }
}
wrapper!(FontFamily egui::FontFamily);
impl TealData for FontFamily {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        type InnerType = egui::FontFamily;
        wrap_method!(f; default; ; FontFamily);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(_fields: &mut F) {}
}
wrapper!( RectShape egui::epaint::RectShape);
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
            Ok(Self(egui::epaint::TextShape::new(a0.into(), a1.0)))
        });
        wrap_method!(m; visual_bounding_rect;; Rect);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(fields, pos: Pos2, underline: Stroke, angle: f32);
        fields.add_field_method_get("galley", |_, s| Ok(Galley(s.galley.clone())));
        fields.add_field_method_set("galley", |_, s, a0: Galley| {
            s.galley = a0.0;
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

wrapper!( Margin egui::style::Margin);
impl TealData for Margin {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        type InnerType = egui::style::Margin;
        methods.document_type(
            "egui docs link: https://docs.rs/egui/latest/egui/style/struct.Margin.html",
        );
        wrap_method!(f; same; f32; Margin);
        // methods.add_function("same", |_, a0: f32| {
        //     Ok(Margin(egui::style::Margin::same(a0)))
        // });
        // methods.add_function("symmetric", |_, (a0, a1): (f32, f32)| {
        //     Ok(Margin(egui::style::Margin::symmetric(a0, a1)))
        // });
        wrap_method!(m; sum;; Vec2);
        wrap_method!(m; left_top;; Vec2);
        wrap_method!(m; right_bottom;; Vec2);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(fields, left: f32, right: f32, top: f32, bottom: f32);
    }
}
wrapper!( Stroke egui::Stroke);
impl TealData for Stroke {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods
            .document_type("egui docs link: https://docs.rs/egui/latest/egui/struct.Stroke.html");

        methods.add_function("none", |_, ()| Ok(Self::from(egui::Stroke::NONE)));
        methods.add_function("new", |_, (a0, a1): (f32, Color32)| {
            Ok(Self::from(egui::Stroke::new(a0, a1)))
        });
        wrap_method!(m; is_empty;; bool);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(fields, width: f32, color: Color32);
    }
}
wrapper!( Rounding egui::Rounding);
impl TealData for Rounding {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods
            .document_type("egui docs link: https://docs.rs/egui/latest/egui/struct.Stroke.html");
        methods.add_function("same", |_, a0: f32| {
            Ok(Self::from(egui::Rounding::same(a0)))
        });
        methods.add_function("none", |_, ()| Ok(Self::from(egui::Rounding::none())));
        wrap_method!(m; is_same;; bool);
        wrap_method!(m; at_least; f32; Rounding);
        wrap_method!(m; at_most; f32; Rounding);
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
        wrap_method!(m; icon_rectangles; Rect; Rect, Rect);
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
wrapper!( Shadow egui::epaint::Shadow);
impl TealData for Shadow {
    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(fields, extrusion: f32, color: Color32);
    }
}

wrapper!( WidgetVisuals egui::style::WidgetVisuals);
impl TealData for WidgetVisuals {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        wrap_method!(m; text_color;; Color32);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(
            fields,
            bg_fill: Color32,
            bg_stroke: Stroke,
            rounding: Rounding,
            fg_stroke: Stroke,
            expansion: f32
        );
    }
}
wrapper!(Widgets egui::style::Widgets);
impl TealData for Widgets {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(_methods: &mut T) {}

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(
            fields,
            noninteractive: WidgetVisuals,
            inactive: WidgetVisuals,
            hovered: WidgetVisuals,
            active: WidgetVisuals,
            open: WidgetVisuals
        );
    }
}
wrapper!( Selection egui::style::Selection);
impl TealData for Selection {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(_methods: &mut T) {}

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(fields, bg_fill: Color32, stroke: Stroke);
    }
}
wrapper!(Interaction egui::style::Interaction);
impl TealData for Interaction {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(_methods: &mut T) {}

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(
            fields,
            resize_grab_radius_side: f32,
            resize_grab_radius_corner: f32,
            show_tooltips_only_when_still: bool
        );
    }
}
wrapper!(Visuals egui::style::Visuals);
impl TealData for Visuals {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("noninteractive", |_, s, ()| {
            Ok(WidgetVisuals::from(*s.noninteractive()))
        });
        wrap_method!(m; text_color;; Color32);
        wrap_method!(m; weak_text_color;; Color32);
        wrap_method!(m; strong_text_color;; Color32);
        wrap_method!(m; window_fill;; Color32);
        wrap_method!(m; window_stroke;; Stroke);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("override_text_color", |_, s| {
            Ok(s.override_text_color.map(Color32::from))
        });
        fields.add_field_method_set("override_text_color", |_, s, a0: Option<Color32>| {
            s.override_text_color = a0.map(|a| a.into());
            Ok(())
        });
        fields.add_field_method_get("widgets", |_, s| Ok(Widgets::from(s.widgets.clone())));
        fields.add_field_method_set("widgets", |_, s, a0: Widgets| {
            s.widgets = a0.into();
            Ok(())
        });
        add_fields!(
            fields,
            dark_mode: bool,
            selection: Selection,
            hyperlink_color: Color32,
            faint_bg_color: Color32,
            extreme_bg_color: Color32,
            code_bg_color: Color32,
            window_rounding: Rounding,
            window_shadow: Shadow,
            popup_shadow: Shadow,
            resize_corner_size: f32,
            text_cursor_width: f32,
            text_cursor_preview: bool,
            clip_rect_margin: f32,
            button_frame: bool,
            collapsing_header_frame: bool
        );
    }
}

wrapper!(TextStyle egui::TextStyle);
impl TealData for TextStyle {}

wrapper!(Painter egui::Painter);
impl TealData for Painter {}

wrapper!(Layout egui::Layout);
impl TealData for Layout {}

wrapper!(Align2 egui::Align2);
impl TealData for Align2 {}
wrapper!( Rect egui::Rect);
impl TealData for Rect {}

wrapper!( LayerId egui::LayerId);
impl TealData for LayerId {}

wrapper!( Color32 egui::Color32);

wrapper!( Id egui::Id);
impl TealData for Id {}

wrapper!(RichText egui::RichText);
impl TealData for RichText {}

wrapper!(WidgetText egui::WidgetText);
impl TealData for WidgetText {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        wrap_method!(m; is_empty;; bool);
        wrap_method!(m; text;; String);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(_fields: &mut F) {}
}

wrapper!( TextureId egui::TextureId);
impl TealData for TextureId {}

wrapper!( Vec2 egui::Vec2);
// pub type Vec2 = Wrapper<egui::Vec2>;

// impl TypeName for Vec2 {
//     fn get_type_parts() -> std::borrow::Cow<'static, [tealr::NamePart]> {
//         new_type!(Vec2)
//     }
// }

wrapper!( Pos2 egui::Pos2);
impl TealData for Pos2 {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        wrap_method!(m; to_vec2;; Vec2);
        wrap_method!(m; distance; Pos2; f32);
        wrap_method!(m; distance_sq; Pos2; f32);
        wrap_method!(m; floor;; Pos2);
        wrap_method!(m; round;; Pos2);
        wrap_method!(m; ceil;; Pos2);
        wrap_method!(m; is_finite;; bool);
        wrap_method!(m; any_nan;; bool);
        wrap_method!(m; min; Pos2; Pos2);
        wrap_method!(m; max; Pos2; Pos2);
        wrap_method!(m; clamp; Pos2, Pos2; Pos2);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(_fields: &mut F) {}
}
wrapper!( Sense egui::Sense);
impl TealData for Sense {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        wrap_method!(m; interactive;; bool);
    }

    fn add_fields<'lua, F: TealDataFields<'lua, Self>>(fields: &mut F) {
        add_fields!(fields, click: bool, drag: bool, focusable: bool);
    }
}

wrapper!( Align egui::Align);
impl TealData for Align {}

wrapper!( PointerButton egui::PointerButton);
impl TealData for PointerButton {}

wrapper!( CursorIcon egui::CursorIcon);
impl TealData for CursorIcon {}

impl TealData for Color32 {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        type InnerType = egui::Color32;
        wrap_method!(f; default;; Color32);
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
impl From<Galley> for Arc<egui::Galley> {
    fn from(a: Galley) -> Self {
        a.0
    }
}

#[derive(Clone, AsRef, AsMut, Deref, tealr::MluaTealDerive)]
pub struct Galley(Arc<egui::Galley>);
impl TealData for Galley {}

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
        wrap_method!(m; id; ; TextureId);
        // methods.add_method("id", |_, th, ()| Ok(TextureId(th.id().into())));
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

wrapper!(Style egui::Style);
impl TealData for Style {}

impl From<&Arc<egui::Style>> for Style {
    fn from(s: &Arc<egui::Style>) -> Self {
        Self(egui::Style::clone(s))
    }
}
impl From<Arc<egui::Style>> for Style {
    fn from(s: Arc<egui::Style>) -> Self {
        Self(egui::Style::clone(s.as_ref()))
    }
}
impl From<Style> for Arc<egui::Style> {
    fn from(s: Style) -> Self {
        Arc::new(s.0)
    }
}

tealr::create_union_mlua!(pub Derives(Debug, Hash) enum IntoIdSource = String | i64 | bool);
