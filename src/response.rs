use derive_more::*;
use tealr::{
    mlu::{
        mlua::{self, Function},
        TealDataMethods,
    },
    *,
};

use crate::{
    add_method, add_method_mut, Align, CursorIcon, IntoWidgetText, PointerButton, Pos2, Sense, Ui,
    Vec2,
};

#[derive(Clone, MluaTealDerive, AsRef, AsMut, From, Deref, DerefMut)]
pub struct Response(pub egui::Response);

impl From<Response> for egui::Response {
    fn from(r: Response) -> Self {
        r.0
    }
}

impl tealr::mlu::TealData for Response {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("This is the Egui Response");
        add_method!(methods, clicked, (), bool);
        add_method!(methods, clicked_by, PointerButton, bool);
        add_method!(methods, secondary_clicked, (), bool);
        add_method!(methods, middle_clicked, (), bool);
        add_method!(methods, double_clicked, (), bool);
        add_method!(methods, triple_clicked, (), bool);
        add_method!(methods, double_clicked_by, PointerButton, bool);
        add_method!(methods, triple_clicked_by, PointerButton, bool);
        add_method!(methods, clicked_elsewhere, (), bool);
        add_method!(methods, enabled, (), bool);
        add_method!(methods, hovered, (), bool);
        add_method!(methods, has_focus, (), bool);
        add_method!(methods, gained_focus, (), bool);
        add_method!(methods, lost_focus, (), bool);
        add_method!(methods, request_focus);
        add_method!(methods, surrender_focus);
        add_method!(methods, dragged, (), bool);
        add_method!(methods, dragged_by, PointerButton, bool);
        add_method!(methods, drag_started, (), bool);
        add_method!(methods, drag_released, (), bool);
        add_method!(methods, drag_delta, (), Vec2);
        methods.add_method("interact_pointer_pos", |_, resp, ()| {
            Ok(resp.interact_pointer_pos().map(Pos2::from))
        });
        methods.add_method("hover_pos", |_, resp, ()| {
            Ok(resp.hover_pos().map(Pos2::from))
        });
        add_method!(methods, is_pointer_button_down_on, (), bool);
        add_method!(methods, changed, (), bool);
        add_method_mut!(methods, mark_changed);
        methods.add_method("on_hover_ui", |lua, resp, a0: Function| {
            Ok(Response::from(resp.clone().0.on_hover_ui(|ui| {
                lua.scope(|scope| {
                    let ui = scope.create_nonstatic_userdata(Ui::from(ui))?;
                    let _: () = a0.call(ui)?;
                    Ok(())
                })
                .expect("failed to call ui function in hover ui");
            })))
        });
        methods.add_method("on_disabled_hover_ui", |lua, resp, a0: Function| {
            Ok(Response::from(resp.clone().0.on_disabled_hover_ui(|ui| {
                lua.scope(|scope| {
                    let ui = scope.create_nonstatic_userdata(Ui::from(ui))?;
                    let _: () = a0.call(ui)?;
                    Ok(())
                })
                .expect("failed to call ui function in hover ui");
            })))
        });
        methods.add_method("on_hover_ui_at_pointer", |lua, resp, a0: Function| {
            Ok(Response::from(resp.clone().0.on_hover_ui_at_pointer(
                |ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_nonstatic_userdata(Ui::from(ui))?;
                        let _: () = a0.call(ui)?;
                        Ok(())
                    })
                    .expect("failed to call ui function in hover ui");
                },
            )))
        });
        methods.add_method("on_hover_text_at_pointer", |_, resp, a0: IntoWidgetText| {
            Ok(Response::from(resp.clone().0.on_hover_text_at_pointer(a0)))
        });
        methods.add_method("on_hover_text", |_, resp, a0: IntoWidgetText| {
            Ok(Response::from(resp.clone().0.on_hover_text(a0)))
        });
        methods.add_method("on_disabled_hover_text", |_, resp, a0: IntoWidgetText| {
            Ok(Response::from(resp.clone().0.on_disabled_hover_text(a0)))
        });

        methods.add_method("on_hover_cursor", |_, resp, a0: CursorIcon| {
            Ok(Response::from(resp.clone().0.on_hover_cursor(a0.into())))
        });
        add_method!(methods, interact, Sense, Response);
        methods.add_method("scroll_to_me", |_, resp, a0: Option<Align>| {
            Ok(resp.scroll_to_me(a0.map(|a| a.into())))
        });
        // add_method!(methods, widget_info, (), bool);
        methods.add_method("context_menu", |lua, resp, a0: Function| {
            Ok(Response::from(resp.clone().0.context_menu(|ui| {
                lua.scope(|scope| {
                    let ui = scope.create_nonstatic_userdata(Ui::from(ui))?;
                    let _: () = a0.call(ui)?;
                    Ok(())
                })
                .expect("failed to call ui function in hover ui");
            })))
        });
        add_method!(methods, union, Response, Response);
    }
}
