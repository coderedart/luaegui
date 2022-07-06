use derive_more::*;
use luaegui_derive::wrap_method;
use tealr::{
    mlu::{
        mlua::{self, Function, Lua, Result},
        TealDataMethods,
    },
    *,
};

use crate::{Align, CursorIcon, IntoWidgetText, PointerButton, Pos2, Sense, Ui, Vec2};

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
        wrap_method!(m; clicked;; bool);
        wrap_method!(m; clicked_by; PointerButton; bool);
        wrap_method!(m; secondary_clicked;; bool);
        wrap_method!(m; middle_clicked;; bool);
        wrap_method!(m; double_clicked;; bool);
        wrap_method!(m; triple_clicked;; bool);
        wrap_method!(m; double_clicked_by; PointerButton; bool);
        wrap_method!(m; triple_clicked_by; PointerButton; bool);
        wrap_method!(m; clicked_elsewhere;; bool);
        wrap_method!(m; enabled;; bool);
        wrap_method!(m; hovered;; bool);
        wrap_method!(m; has_focus;; bool);
        wrap_method!(m; gained_focus;; bool);
        wrap_method!(m; lost_focus;; bool);
        wrap_method!(m; request_focus);
        wrap_method!(m; surrender_focus);
        wrap_method!(m; dragged;;bool);
        wrap_method!(m; dragged_by; PointerButton; bool);
        wrap_method!(m; drag_started;; bool);
        wrap_method!(m; drag_released;; bool);
        wrap_method!(m; drag_delta;; Vec2);
        methods.add_method("interact_pointer_pos", |_, resp, ()| {
            Ok(resp.interact_pointer_pos().map(Pos2::from))
        });
        methods.add_method("hover_pos", |_, resp, ()| {
            Ok(resp.hover_pos().map(Pos2::from))
        });
        wrap_method!(m; is_pointer_button_down_on;; bool);
        wrap_method!(m; changed;; bool);
        wrap_method!(mm; mark_changed);
        methods.add_method("on_hover_ui", |lua, resp, a0: Function| {
            Ok(Response::from(resp.clone().0.on_hover_ui(|ui| {
                lua.scope(|scope| {
                    let ui = scope.create_nonstatic_userdata(Ui::from(ui))?;
                    a0.call(ui)?;
                    Ok(())
                })
                .expect("failed to call ui function in hover ui");
            })))
        });
        methods.add_method("on_disabled_hover_ui", |lua, resp, a0: Function| {
            Ok(Response::from(resp.clone().0.on_disabled_hover_ui(|ui| {
                lua.scope(|scope| {
                    let ui = scope.create_nonstatic_userdata(Ui::from(ui))?;
                    a0.call(ui)?;
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
                        a0.call(ui)?;
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
        wrap_method!(m; interact; Sense; Response);
        methods.add_method("scroll_to_me", |_, resp, a0: Option<Align>| {
            resp.scroll_to_me(a0.map(|a| a.into()));
            Ok(())
        });
        // wrap_method!(m; widget_info, (), bool);
        methods.add_method("context_menu", |lua, resp, a0: Function| {
            Ok(Response::from(resp.clone().0.context_menu(|ui| {
                lua.scope(|scope| {
                    let ui = scope.create_nonstatic_userdata(Ui::from(ui))?;
                    a0.call(ui)?;
                    Ok(())
                })
                .expect("failed to call ui function in hover ui");
            })))
        });
        wrap_method!(m; union; Response; Response);
    }
}
