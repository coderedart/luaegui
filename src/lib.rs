use egui::{
    epaint::Shadow,
    style::{Spacing, WidgetVisuals},
    Align, Align2, Area, CentralPanel, Color32, Context, Direction, Frame, Id, LayerId, Layout,
    Margin, Order, PointerButton, Pos2, Rect, RichText, Rounding, Sense, SidePanel, Stroke, Style,
    TextStyle, TextureHandle, TopBottomPanel, Ui, Vec2, WidgetText, Window,
};
use mlua::{
    AnyUserData, Function, Lua, MultiValue, Result, Table, UserDataFields, UserDataMethods,
    UserDataRef, UserDataRefMut, UserDataRegistrar, Value,
};

trait LuaHelperTrait: Sized {
    fn from_lua(value: Value) -> Result<Self>;
    fn to_lua(value: Self, lua: &Lua) -> Result<Value>;
    fn add_to_lua(lua: &Lua, egui_table: &Table) -> Result<()>;
}

pub fn register_egui_bindings(lua: &Lua) -> mlua::Result<()> {
    let et = lua.create_table()?;
    let egui_table = &et;
    Align::add_to_lua(lua, egui_table)?;
    Align2::add_to_lua(lua, egui_table)?;
    Color32::add_to_lua(lua, egui_table)?;
    Direction::add_to_lua(lua, egui_table)?;
    Id::add_to_lua(lua, egui_table)?;
    Margin::add_to_lua(lua, egui_table)?;
    PointerButton::add_to_lua(lua, egui_table)?;
    Pos2::add_to_lua(lua, egui_table)?;
    Rect::add_to_lua(lua, egui_table)?;
    RichText::add_to_lua(lua, egui_table)?;
    Rounding::add_to_lua(lua, egui_table)?;
    Sense::add_to_lua(lua, egui_table)?;
    Stroke::add_to_lua(lua, egui_table)?;
    TextStyle::add_to_lua(lua, egui_table)?;
    Vec2::add_to_lua(lua, egui_table)?;
    WidgetText::add_to_lua(lua, egui_table)?;

    add_area(lua, egui_table)?;
    add_context(lua, egui_table)?;
    add_frame(lua, egui_table)?;
    add_layer_id(lua, egui_table)?;
    add_layout(lua, egui_table)?;
    add_response(lua)?;
    add_shadow(lua, egui_table)?;
    add_spacing(lua, egui_table)?;
    add_style(lua, egui_table)?;
    add_ui(lua, egui_table)?;
    add_widget_visuals(lua, egui_table)?;
    add_window(lua, egui_table)?;
    add_central_panel(lua, egui_table)?;
    add_side_panel(lua, egui_table)?;
    add_top_bottom_panel(lua, egui_table)?;
    egui_table.set_readonly(true);
    lua.globals().set("egui", et)?;
    Ok(())
}

fn add_context(lua: &Lua, _: &Table) -> mlua::Result<()> {
    lua.register_userdata_type(|reg: &mut UserDataRegistrar<Context>| {
        reg.add_method("request_repaint", |_, this, ()| {
            this.request_repaint();
            Ok(())
        });
        reg.add_method("request_repaint_after", |_, this, duration: f64| {
            this.request_repaint_after(std::time::Duration::from_secs_f64(duration));
            Ok(())
        });
    })?;
    Ok(())
}

impl LuaHelperTrait for Id {
    fn from_lua(value: Value) -> Result<Self> {
        Ok(match value {
            Value::Nil => Id::null(),
            Value::String(s) => Id::null().with(s.to_str().unwrap_or_default()),
            Value::UserData(u) => {
                *u.borrow()
                    .map_err(|_e| mlua::Error::FromLuaConversionError {
                        from: "Value",
                        to: "Id",
                        message: Some(
                            "The variant of value is not suitable for converting to Id".to_string(),
                        ),
                    })?
            }
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "Value",
                    to: "Id",
                    message: Some(
                        "The variant of value is not suitable for converting to Id".to_string(),
                    ),
                })
            }
        })
    }

    fn to_lua(value: Self, lua: &Lua) -> Result<Value> {
        lua.create_any_userdata(value).map(Value::UserData)
    }

    fn add_to_lua(lua: &Lua, egui_table: &Table) -> Result<()> {
        let id: Table<'_> = lua.create_table()?;
        lua.register_userdata_type(|reg: &mut UserDataRegistrar<Id>| {
            reg.add_method("with", |lua, this, value: Value| {
                lua.create_any_userdata(match value {
                    Value::Nil => Id::null(),
                    Value::Boolean(b) => this.with(b),
                    Value::Integer(b) => this.with(b),
                    Value::String(b) => this.with(b),
                    _ => {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: "value",
                            to: "hash_for_egui_id",
                            message: None,
                        })
                    }
                })
            });
            reg.add_method("short_debug_format", |_, this, ()| {
                Ok(this.short_debug_format())
            });
        })?;
        id.set("null", lua.create_any_userdata(Id::null())?)?;
        egui_table.set("id", id)?;
        Ok(())
    }
}

fn add_widget_visuals(lua: &Lua, egui_table: &Table) -> mlua::Result<()> {
    let id = lua.create_table()?;
    lua.register_userdata_type(|reg: &mut UserDataRegistrar<WidgetVisuals>| {
        reg.add_field_method_get("bg_fill", |lua, this| Color32::to_lua(this.bg_fill, lua));
        reg.add_field_method_get("weak_bg_fill", |lua, this| {
            Color32::to_lua(this.weak_bg_fill, lua)
        });
        reg.add_field_method_get("bg_stroke", |lua, this| Stroke::to_lua(this.bg_stroke, lua));
        reg.add_field_method_get("rounding", |lua, this| Rounding::to_lua(this.rounding, lua));
        reg.add_field_method_get("fg_stroke", |lua, this| Stroke::to_lua(this.fg_stroke, lua));
        reg.add_field_method_get("expansion", |_, this| Ok(this.expansion));

        reg.add_field_method_set("bg_fill", |_, this, value: Value| {
            this.bg_fill = Color32::from_lua(value)?;
            Ok(())
        });
        reg.add_field_method_set("weak_bg_fill", |_, this, value: Value| {
            this.weak_bg_fill = Color32::from_lua(value)?;
            Ok(())
        });
        reg.add_field_method_set("bg_stroke", |_, this, value: Value| {
            this.bg_stroke = Stroke::from_lua(value)?;
            Ok(())
        });
        reg.add_field_method_set("rounding", |_, this, value: Value| {
            this.rounding = Rounding::from_lua(value)?;
            Ok(())
        });
        reg.add_field_method_set("fg_stroke", |_, this, value: Value| {
            this.fg_stroke = Stroke::from_lua(value)?;
            Ok(())
        });
        reg.add_field_method_set("expansion", |_, this, value: f32| {
            this.expansion = value;
            Ok(())
        });
    })?;
    id.set("null", lua.create_any_userdata(Id::null())?)?;
    egui_table.set("id", id)?;
    Ok(())
}

fn add_layout(lua: &Lua, egui_table: &Table) -> mlua::Result<()> {
    let layout = lua.create_table()?;
    lua.register_userdata_type(|reg: &mut UserDataRegistrar<Layout>| {
        reg.add_field_method_get("main_dir", |lua, this| {
            Direction::to_lua(this.main_dir, lua)
        });
        reg.add_field_method_get("main_wrap", |_, this| Ok(this.main_wrap));
        reg.add_field_method_get("main_align", |lua, this| {
            Align::to_lua(this.main_align, lua)
        });
        reg.add_field_method_get("main_justify", |_, this| Ok(this.main_justify));
        reg.add_field_method_get("cross_align", |lua, this| {
            Align::to_lua(this.cross_align, lua)
        });
        reg.add_field_method_get("cross_justify", |_, this| Ok(this.cross_justify));

        reg.add_field_method_set("main_dir", |_, this, value: Value| {
            this.main_dir = Direction::from_lua(value)?;
            Ok(())
        });
        reg.add_field_method_set("main_wrap", |_, this, value: bool| {
            this.main_wrap = value;
            Ok(())
        });
        reg.add_field_method_set("main_align", |_, this, value: Value| {
            this.main_align = Align::from_lua(value)?;
            Ok(())
        });
        reg.add_field_method_set("main_justify", |_, this, value: bool| {
            this.main_justify = value;
            Ok(())
        });
        reg.add_field_method_set("cross_align", |_, this, value: Value| {
            this.cross_align = Align::from_lua(value)?;
            Ok(())
        });
        reg.add_field_method_set("cross_justify", |_, this, value: bool| {
            this.cross_justify = value;
            Ok(())
        });
    })?;

    egui_table.set("layout", layout)?;
    Ok(())
}
fn add_spacing(lua: &Lua, _egui_table: &Table) -> mlua::Result<()> {
    lua.register_userdata_type(|reg: &mut UserDataRegistrar<Spacing>| {
        reg.add_field_method_get("item_spacing", |lua, this| {
            Vec2::to_lua(this.item_spacing, lua)
        });
        reg.add_field_method_get("window_margin", |lua, this| {
            Margin::to_lua(this.window_margin, lua)
        });
        reg.add_field_method_get("button_padding", |lua, this| {
            Vec2::to_lua(this.button_padding, lua)
        });
        reg.add_field_method_get("menu_margin", |lua, this| {
            Margin::to_lua(this.menu_margin, lua)
        });
        reg.add_field_method_get("indent", |_, this| Ok(this.indent));

        reg.add_field_method_get("interact_size", |lua, this| {
            Vec2::to_lua(this.interact_size, lua)
        });
        reg.add_field_method_get("slider_width", |_, this| Ok(this.slider_width));
        reg.add_field_method_get("combo_width", |_, this| Ok(this.combo_width));
        reg.add_field_method_get("text_edit_width", |_, this| Ok(this.text_edit_width));
        reg.add_field_method_get("icon_width", |_, this| Ok(this.icon_width));
        reg.add_field_method_get("icon_width_inner", |_, this| Ok(this.icon_width_inner));
        reg.add_field_method_get("icon_spacing", |_, this| Ok(this.icon_spacing));
        reg.add_field_method_get("tooltip_width", |_, this| Ok(this.tooltip_width));
        reg.add_field_method_get("indent_ends_with_horizontal_line", |_, this| {
            Ok(this.indent_ends_with_horizontal_line)
        });
        reg.add_field_method_get("combo_height", |_, this| Ok(this.combo_height));
        reg.add_field_method_get("scroll_bar_width", |_, this| Ok(this.scroll_bar_width));
        reg.add_field_method_get("scroll_handle_min_length", |_, this| {
            Ok(this.scroll_handle_min_length)
        });
        reg.add_field_method_get("scroll_bar_inner_margin", |_, this| {
            Ok(this.scroll_bar_inner_margin)
        });
        reg.add_field_method_get("scroll_bar_outer_margin", |_, this| {
            Ok(this.scroll_bar_outer_margin)
        });

        // reg.add_field_method_set("weak_bg_fill", |_, this, value: Value| {
        //     this.weak_bg_fill = Color32::from_lua(value)?;
        //     Ok(())
        // });
        // reg.add_field_method_set("bg_stroke", |_, this, value: Value| {
        //     this.bg_stroke = Stroke::from_lua(value)?;
        //     Ok(())
        // });
        // reg.add_field_method_set("rounding", |_, this, value: Value| {
        //     this.rounding = Rounding::from_lua(value)?;
        //     Ok(())
        // });
        // reg.add_field_method_set("fg_stroke", |_, this, value: Value| {
        //     this.fg_stroke = Stroke::from_lua(value)?;
        //     Ok(())
        // });
        // reg.add_field_method_set(
        //     "expansion",
        //     |_, this, value: f32| Ok(this.expansion = value),
        // );
    })?;
    Ok(())
}
fn add_response(lua: &Lua) -> mlua::Result<()> {
    lua.register_userdata_type(|reg: &mut UserDataRegistrar<egui::Response>| {
        reg.add_method("changed", |_, this, ()| Ok(this.changed()));
        reg.add_method("clicked", |_, this, ()| Ok(this.clicked()));
        reg.add_method("clicked_by", |_, this, value: Value| {
            Ok(this.clicked_by(PointerButton::from_lua(value)?))
        });
        reg.add_method("clicked_elsewhere", |_, this, ()| {
            Ok(this.clicked_elsewhere())
        });
        // reg.add_method("context_menu", |_, this, ()| Ok(this.changed()));
        reg.add_method("double_clicked", |_, this, ()| Ok(this.double_clicked()));
        reg.add_method("double_clicked_by", |_, this, value: Value| {
            Ok(this.double_clicked_by(PointerButton::from_lua(value)?))
        });
        reg.add_method("drag_delta", |lua, this, ()| {
            Ok(Vec2::to_lua(this.drag_delta(), lua))
        });
        reg.add_method("drag_released", |_, this, ()| Ok(this.drag_released()));
        reg.add_method("drag_released_by", |_, this, value: Value| {
            Ok(this.drag_released_by(PointerButton::from_lua(value)?))
        });
        reg.add_method("drag_started", |_, this, ()| Ok(this.drag_started()));
        reg.add_method("drag_started_by", |_, this, value: Value| {
            Ok(this.drag_started_by(PointerButton::from_lua(value)?))
        });
        reg.add_method("dragged", |_, this, ()| Ok(this.dragged()));
        reg.add_method("dragged_by", |_, this, value: Value| {
            Ok(this.dragged_by(PointerButton::from_lua(value)?))
        });
        reg.add_method("enabled", |_, this, ()| Ok(this.enabled()));
        reg.add_method("gained_focus", |_, this, ()| Ok(this.gained_focus()));
        reg.add_method("has_focus", |_, this, ()| Ok(this.has_focus()));
        reg.add_method("highlight", |lua, this, ()| {
            lua.create_any_userdata(this.clone().highlight())
        });
        reg.add_method("hover_pos", |lua, this, ()| {
            Ok(this.hover_pos().and_then(|p| Pos2::to_lua(p, lua).ok()))
        });
        reg.add_method("hovered", |_, this, ()| Ok(this.hovered()));
    })
}

fn add_ui(lua: &Lua, _egui_table: &Table) -> mlua::Result<()> {
    lua.register_userdata_type(|reg: &mut UserDataRegistrar<Ui>| {
        reg.add_method_mut(
            "add_enabled_ui",
            |lua, this, (enabled, add_contents): (bool, Function)| {
                let ir = this.add_enabled_ui(enabled, |ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                let r = lua.create_any_userdata(ir.response)?;
                let mut i = ir.inner?;
                i.push_front(Value::UserData(r));
                Ok(i)
            },
        );

        reg.add_method_mut("add_space", |_, this, amount: f32| {
            this.add_space(amount);
            Ok(())
        });
        reg.add_method_mut(
            "add_visible_ui",
            |lua, this, (visible, add_contents): (bool, Function)| {
                let ir = this.add_visible_ui(visible, |ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                let r = lua.create_any_userdata(ir.response)?;
                let mut i = ir.inner?;
                i.push_front(Value::UserData(r));
                Ok(i)
            },
        );
        reg.add_method_mut(
            "allocate_at_least",
            |lua, this, (desired_size, sense): (Value, Value)| {
                let (rect, resp) = this.allocate_at_least(
                    LuaHelperTrait::from_lua(desired_size)?,
                    LuaHelperTrait::from_lua(sense)?,
                );

                Ok((Rect::to_lua(rect, lua)?, lua.create_any_userdata(resp)?))
            },
        );
        reg.add_method_mut(
            "allocate_exact_size",
            |lua, this, (desired_size, sense): (Value, Value)| {
                let (rect, resp) = this.allocate_exact_size(
                    LuaHelperTrait::from_lua(desired_size)?,
                    LuaHelperTrait::from_lua(sense)?,
                );

                Ok((Rect::to_lua(rect, lua)?, lua.create_any_userdata(resp)?))
            },
        );
        reg.add_method_mut(
            "allocate_rect",
            |lua, this, (rect, sense): (Value, Value)| {
                lua.create_any_userdata(this.allocate_rect(
                    LuaHelperTrait::from_lua(rect)?,
                    LuaHelperTrait::from_lua(sense)?,
                ))
            },
        );
        reg.add_method_mut(
            "allocate_response",
            |lua, this, (desired_size, sense): (Value, Value)| {
                lua.create_any_userdata(this.allocate_response(
                    LuaHelperTrait::from_lua(desired_size)?,
                    LuaHelperTrait::from_lua(sense)?,
                ))
            },
        );

        reg.add_method_mut("allocate_space", |lua, this, desired_size: Value| {
            let (id, rect) = this.allocate_space(LuaHelperTrait::from_lua(desired_size)?);
            Ok((lua.create_any_userdata(id)?, Rect::to_lua(rect, lua)?))
        });
        reg.add_method_mut(
            "allocate_ui",
            |lua, this, (desired_size, add_contents): (Value, Function)| {
                let ir = this.allocate_ui(Vec2::from_lua(desired_size)?, |ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                let r = lua.create_any_userdata(ir.response)?;
                let mut i = ir.inner?;
                i.push_front(Value::UserData(r));
                Ok(i)
            },
        );
        reg.add_method_mut(
            "allocate_ui_at_rect",
            |lua, this, (max_rect, add_contents): (Value, Function)| {
                let ir = this.allocate_ui_at_rect(Rect::from_lua(max_rect)?, |ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                let r = lua.create_any_userdata(ir.response)?;
                let mut i = ir.inner?;
                i.push_front(Value::UserData(r));
                Ok(i)
            },
        );
        reg.add_method_mut(
            "allocate_ui_with_layout",
            |lua, this, (desired_size, layout, add_contents): (Value, UserDataRef<Layout>, Function)| {
                let ir = this.allocate_ui_with_layout(Vec2::from_lua(desired_size)?, *layout,|ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                let r = lua.create_any_userdata(ir.response)?;
                let mut i = ir.inner?;
                i.push_front(Value::UserData(r));
                Ok(i)
            },
        );
        reg.add_method("auto_id_with", |lua, this, value: Value| {
            lua.create_any_userdata(match value {
              Value::Boolean(b) => {
                this.auto_id_with(b)
                },
                Value::Integer(b) => this.auto_id_with(b),
                Value::String(b) => this.auto_id_with(b),
                _ => return Err(mlua::Error::external("value type cannot be hashed to get new id"))
            })
        });
        reg.add_method("available_height", |_, this, ()|Ok(this.available_height()));
        reg.add_method("available_rect_before_wrap", |lua, this, ()| Rect::to_lua(this.available_rect_before_wrap(), lua));
        reg.add_method("avaialble_size", |lua, this, ()| Vec2::to_lua(this.available_size(), lua));
        reg.add_method("avaialble_size_before_wrap", |lua, this, ()| Vec2::to_lua(this.available_size_before_wrap(), lua));
        reg.add_method("available_width", |_, this, ()|Ok(this.available_width()));
        reg.add_method_mut("button", |lua, this, value: Value| {
            lua.create_any_userdata(this.button(WidgetText::from_lua(value)?))
        });
        reg.add_method_mut(
            "centered_and_justified",
            |lua, this, add_contents: Function| {
                let ir = this.centered_and_justified(|ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                let r = lua.create_any_userdata(ir.response)?;
                let mut i = ir.inner?;
                i.push_front(Value::UserData(r));
                Ok(i)
            },
        );
        reg.add_method_mut("checkbox", |lua, this, value: Table| {
            let mut b: bool = value.get("checked")?;
            let result = lua.create_any_userdata(this.checkbox(&mut b, WidgetText::from_lua(value.get("text")?)?));
            value.set("checked", b)?;
            result
        });
        reg.add_method_mut("child_ui", |lua, this, (max_rect, layout): (Value, UserDataRef<Layout>)| {
            let ui =  this.child_ui(LuaHelperTrait::from_lua(max_rect)?, *layout);
            lua.create_any_userdata(ui)
        });
        // requires impl Hash for Value smh
        // reg.add_method_mut("child_ui_with_id_source", |lua, this, (max_rect, layout): (Value, Value)| {
        //     let ui =  this.child_ui(LuaHelperTrait::from_lua(max_rect)?, LuaHelperTrait::from_lua(layout)?);
        //     lua.create_any_userdata(ui)
        // });
        reg.add_method("clip_rect", |lua, this, ()| {
            Rect::to_lua(this.clip_rect(), lua)
        });
        reg.add_method_mut("close_menu", |_, this, ()| {
            this.close_menu();
            Ok(())
        });
        reg.add_method_mut("code", |lua, this, value: Value| {
            lua.create_any_userdata(this.code(RichText::from_lua(value)?))
        });
        reg.add_method_mut("code_editor", |lua, this, value: Table| {
            let mut b: String = value.get("text")?;
            let result = lua.create_any_userdata(this.code_editor(&mut b));
            value.set("text", b)?;
            result
        });
        // reg.add_method_mut(
        //     "collapsing",
        //     |lua, this, (heading, add_contents): (Value, Function)| {
        //         let result = lua.create_table()?;
        //         let ir = this.collapsing( WidgetText::from_lua(heading)?,|ui| {
        //             lua.scope(|scope| {
        //                 let ui = scope.create_any_userdata_ref_mut(ui)?;
        //                 let _result: Result<MultiValue> = add_contents.call(ui);
        //                 // some lifetime error...
        //                 Ok(())
        //             })
        //         });
        //         result.set("header_response", lua.create_any_userdata(ir.header_response)?)?;
        //         result.set("body_response", lua.create_any_userdata(ir.body_response)?)?;
        //         result.set("body_returned", lua.create_any_userdata(ir.body_returned)?)?;
        //         result.set("openness", lua.create_any_userdata(ir.openness)?)?;
        //         Ok(Value::Table(result))
        //     },
        // );
        reg.add_method_mut(
            "columns",
            |lua, this, (num, add_contents): (usize, Function)| {
                let ir = this.columns( num, |cols| {
                    lua.scope(|scope| {
                        let cols: Vec<AnyUserData> = cols.into_iter().map(|ui| scope.create_any_userdata_ref_mut(ui)).collect::<Result<Vec<AnyUserData>>>()?;
                        let result: Result<MultiValue> = add_contents.call(cols);
                        result
                    })
                });
                ir
            },
        );
        reg.add_method(
            "ctx",
            |lua, this, ()| {
                lua.create_any_userdata(this.ctx().clone())
            },
        );
        reg.add_method(
            "cursor",
            |lua, this, ()| {
                Rect::to_lua(this.cursor(), lua)
            },
        );
        reg.add_method_mut(
            "data",
            |lua, this, add_contents:  Function| {
                let ir = this.data(  |reader| {
                    lua.scope(|scope| {
                        let reader = scope.create_any_userdata_ref(reader)?;
                        let result: Result<MultiValue> = add_contents.call(reader);
                        result
                    })
                });
                ir
            },
        );
        reg.add_method_mut(
            "data_mut",
            |lua, this, add_contents:  Function| {
                let ir = this.data_mut(  |reader| {
                    lua.scope(|scope| {
                        let reader = scope.create_any_userdata_ref_mut(reader)?;
                        let result: Result<MultiValue> = add_contents.call(reader);
                        result
                    })
                });
                ir
            },
        );

        reg.add_method(
            "debug_paint_cursor",
            |_, this, ()| {
                Ok(this.debug_paint_cursor())
            },
        );
        reg.add_method_mut("drag_angle", |lua, this, value: Table| {
            let mut b: f32 = value.get("value")?;
            let result = lua.create_any_userdata(this.drag_angle(&mut b));
            value.set("value", b)?;
            result
        });

        reg.add_method_mut("drag_angle_tau", |lua, this, value: Table| {
            let mut b: f32 = value.get("value")?;
            let result = lua.create_any_userdata(this.drag_angle_tau(&mut b));
            value.set("value", b)?;
            result
        });
        reg.add_method_mut(
            "end_row",
            |_, this, ()| {
                Ok(this.end_row())
            },
        );

        reg.add_method_mut(
            "expand_to_include_rect",
            |_, this, rect: Value| {
                Ok(this.expand_to_include_rect(Rect::from_lua(rect)?))
            },
        );

        reg.add_method_mut(
            "expand_to_include_x",
            |_, this, x: f32| {
                Ok(this.expand_to_include_x(x))
            },
        );
        reg.add_method_mut(
            "expand_to_include_y",
            |_, this, x: f32| {
                Ok(this.expand_to_include_y(x))
            },
        );

        reg.add_method_mut(
            "fonts",
            |lua, this, add_contents:  Function| {
                let ir = this.fonts(  |reader| {
                    lua.scope(|scope| {
                        let reader = scope.create_any_userdata_ref(reader)?;
                        let result: Result<MultiValue> = add_contents.call(reader);
                        result
                    })
                });
                ir
            },
        );

        reg.add_method_mut(
            "group",
            |lua, this, add_contents: Function| {
                let ir = this.group(|ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                let r = lua.create_any_userdata(ir.response)?;
                let mut i = ir.inner?;
                i.push_front(Value::UserData(r));
                Ok(i)
            },
        );
        reg.add_method_mut("heading", |lua, this, value: Value| {
            lua.create_any_userdata(this.heading(RichText::from_lua(value)?))
        });
        reg.add_method_mut(
            "horizontal",
            |lua, this, add_contents: Function| {
                let ir = this.horizontal(|ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                let r = lua.create_any_userdata(ir.response)?;
                let mut i = ir.inner?;
                i.push_front(Value::UserData(r));
                Ok(i)
            },
        );

        reg.add_method_mut(
            "horizontal_centered",
            |lua, this, add_contents: Function| {
                let ir = this.horizontal_centered(|ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                let r = lua.create_any_userdata(ir.response)?;
                let mut i = ir.inner?;
                i.push_front(Value::UserData(r));
                Ok(i)
            },
        );

        reg.add_method_mut(
            "horizontal_top",
            |lua, this, add_contents: Function| {
                let ir = this.horizontal_top(|ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                let r = lua.create_any_userdata(ir.response)?;
                let mut i = ir.inner?;
                i.push_front(Value::UserData(r));
                Ok(i)
            },
        );

        reg.add_method_mut(
            "horizontal_wrapped",
            |lua, this, add_contents: Function| {
                let ir = this.horizontal_wrapped(|ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                let r = lua.create_any_userdata(ir.response)?;
                let mut i = ir.inner?;
                i.push_front(Value::UserData(r));
                Ok(i)
            },
        );

        reg.add_method_mut("hyperlink", |lua, this, value: String| {
            lua.create_any_userdata(this.hyperlink(value))
        });

        reg.add_method_mut("hyperlink_to", |lua, this, (label, url): (Value,  String)| {
            lua.create_any_userdata(this.hyperlink_to(WidgetText::from_lua(label)?, url ))
        });

        reg.add_method("id", |lua, this, ()| lua.create_any_userdata(this.id()));

        reg.add_method_mut("image", |lua, this, (texture, size): (UserDataRef<TextureHandle>,  Value)| {
            lua.create_any_userdata(this.image(texture.id(), Vec2::from_lua(size)? ))
        });
        reg.add_method_mut(
            "indent",
            |lua, this, (hashable, add_contents): (Value, Function)| {
                let ir = this.indent( LuaHashable::from_lua(hashable)?, |ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                let r = lua.create_any_userdata(ir.response)?;
                let mut i = ir.inner?;
                i.push_front(Value::UserData(r));
                Ok(i)
            },
        );

        reg.add_method_mut(
            "input",
            |lua, this, add_contents:  Function| {
                let ir = this.input(  |reader| {
                    lua.scope(|scope| {
                        let reader = scope.create_any_userdata_ref(reader)?;
                        let result: Result<MultiValue> = add_contents.call(reader);
                        result
                    })
                });
                ir
            },
        );
        reg.add_method_mut(
            "input_mut",
            |lua, this, add_contents:  Function| {
                let ir = this.input_mut(  |reader| {
                    lua.scope(|scope| {
                        let reader = scope.create_any_userdata_ref_mut(reader)?;
                        let result: Result<MultiValue> = add_contents.call(reader);
                        result
                    })
                });
                ir
            },
        );

        reg.add_method("interact", |lua, this, (rect, id, sense): (Value, UserDataRef<Id>, Value)| {
            lua.create_any_userdata( this.interact(Rect::from_lua(rect)?, *id, Sense::from_lua(sense)?))
        });

        reg.add_method("interact_with_hovered", |lua, this, (rect, hovered, id, sense): (Value, bool, UserDataRef<Id>, Value)| {
            lua.create_any_userdata( this.interact_with_hovered(Rect::from_lua(rect)?, hovered, *id, Sense::from_lua(sense)?))
        });
        reg.add_method("is_enabled", |_, this, ()| Ok(this.is_enabled()));        
        reg.add_method_mut("is_rect_visible", |_, this, clip_rect: Value| {
            Ok(this.is_rect_visible(Rect::from_lua(clip_rect)?))
        });
        reg.add_method("is_visible", |_, this, ()| Ok(this.is_visible()));
        reg.add_method_mut("label", |lua, this, value: Value| {
            lua.create_any_userdata(this.label(WidgetText::from_lua(value)?))
        });
        reg.add_method("layer_id", |lua, this, () | {
            lua.create_any_userdata(this.layer_id())
        });

        reg.add_method("layout", |lua, this, () | {
            lua.create_any_userdata(this.layout().clone())
        });

        reg.add_method_mut("link", |lua, this, value: Value| {
            lua.create_any_userdata(this.link(WidgetText::from_lua(value)?))
        });

        reg.add_method_mut("make_persistent_id", |lua, this, value: Value| {
            lua.create_any_userdata(this.make_persistent_id(LuaHashable::from_lua(value)?))
        });

        reg.add_method("max_rect", |lua, this, ()| {
            Rect::to_lua(this.max_rect(), lua)
        });

        reg.add_method(
            "memory",
            |lua, this, add_contents:  Function| {
                let ir = this.memory(  |reader| {
                    lua.scope(|scope| {
                        let reader = scope.create_any_userdata_ref(reader)?;
                        let result: Result<MultiValue> = add_contents.call(reader);
                        result
                    })
                });
                ir
            },
        );
        reg.add_method_mut(
            "memory_mut",
            |lua, this, add_contents:  Function| {
                let ir = this.memory_mut(  |reader| {
                    lua.scope(|scope| {
                        let reader = scope.create_any_userdata_ref_mut(reader)?;
                        let result: Result<MultiValue> = add_contents.call(reader);
                        result
                    })
                });
                ir
            },
        );
        reg.add_method_mut(
            "menu_button",
            |lua, this, (title, add_contents): (Value, Function)| {
                let ir = this.menu_button(WidgetText::from_lua(title)?, |ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });

                let mut result = MultiValue::new();
                let response = lua.create_any_userdata(ir.response)?;
                result.push_front(Value::UserData(response));
                if let Some(inner) = ir.inner {
                    let inner = inner?;
                    for v in inner {
                        result.push_front(v);
                    }
                }
                Ok(result)
            },
        );
        reg.add_method_mut("selectable_label", |lua, ui, (selected, text): (bool, Value)| {
            lua.create_any_userdata(ui.selectable_label(selected, WidgetText::from_lua(text)?))
        });
        reg.add_method_mut(
            "set_row_height",
            |_, this, height: f32| {
                Ok(this.set_row_height(height))
            },
        );
        reg.add_method_mut(
            "output",
            |lua, this, add_contents:  Function| {
                let ir = this.output(  |reader| {
                    lua.scope(|scope| {
                        let reader = scope.create_any_userdata_ref(reader)?;
                        let result: Result<MultiValue> = add_contents.call(reader);
                        result
                    })
                });
                ir
            },
        );
        reg.add_method_mut(
            "output_mut",
            |lua, this, add_contents:  Function| {
                let ir = this.output_mut(  |reader| {
                    lua.scope(|scope| {
                        let reader = scope.create_any_userdata_ref_mut(reader)?;
                        let result: Result<MultiValue> = add_contents.call(reader);
                        result
                    })
                });
                ir
            },
        );
        reg.add_method(
            "next_widget_position",
            |lua, this, ()| {
                Pos2::to_lua(this.next_widget_position(), lua)
            },
        );
        reg.add_method(
            "painter",
            |lua, this, ()| {
                lua.create_any_userdata(this.painter().clone())
            },
        );
        reg.add_method_mut("text_edit_multiline", |lua, this, value: Table| {
            let mut b: String = value.get("text")?;
            let result = lua.create_any_userdata(this.text_edit_multiline(&mut b));
            value.set("text", b)?;
            result
        });
        reg.add_method_mut("text_edit_singleline", |lua, this, value: Table| {
            let mut b: String = value.get("text")?;
            let result = lua.create_any_userdata(this.text_edit_singleline(&mut b));
            value.set("text", b)?;
            result
        });
        reg.add_method_mut("set_enabled", |_, this, enabled: bool| {
            this.set_enabled(enabled);
            Ok(())
        });

        reg.add_method("wrap_text", |_, this, ()| Ok(this.wrap_text()));

        reg.add_method_mut("set_clip_rect", |_, this, clip_rect: Value| {
            this.set_clip_rect(Rect::from_lua(clip_rect)?);
            Ok(())
        });
    })?;

    Ok(())
}
impl LuaHelperTrait for Sense {
    fn from_lua(value: Value) -> Result<Self> {
        match value {
            Value::Integer(i) => {
                let i = i as u8;
                let click = 0 != (i & 1);
                let drag = 0 != (i & (1 << 1));
                let focusable = 0 != (i & (1 << 2));
                Ok(Self {
                    click,
                    drag,
                    focusable,
                })
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "luavalue",
                to: "pointerbutton",
                message: None,
            }),
        }
    }

    fn to_lua(value: Self, _lua: &Lua) -> Result<Value> {
        let mut u = 0u8;
        if value.click {
            u &= 1;
        }
        if value.drag {
            u &= 1 << 1;
        }
        if value.focusable {
            u &= 1 << 2;
        }
        Ok(Value::Integer(u as _))
    }

    fn add_to_lua(lua: &Lua, egui_table: &Table) -> Result<()> {
        let sense = lua.create_table()?;
        sense.set("hover", Sense::to_lua(Sense::hover(), lua)?)?;
        sense.set(
            "focusable_noninteractive",
            Sense::to_lua(Sense::focusable_noninteractive(), lua)?,
        )?;
        sense.set("click", Sense::to_lua(Sense::click(), lua)?)?;
        sense.set("drag", Sense::to_lua(Sense::drag(), lua)?)?;
        sense.set(
            "union",
            lua.create_function(|lua, (first, second): (Value, Value)| {
                let first = Sense::from_lua(first)?;
                let second = Sense::from_lua(second)?;
                Sense::to_lua(first.union(second), lua)
            })?,
        )?;
        sense.set(
            "union",
            lua.create_function(|_, value: Value| Ok(Sense::from_lua(value)?.interactive()))?,
        )?;

        egui_table.set("sense", sense)?;
        Ok(())
    }
}
impl LuaHelperTrait for Margin {
    fn add_to_lua(lua: &Lua, egui_table: &Table) -> mlua::Result<()> {
        let margin = lua.create_table()?;
        margin.set(
            "same",
            lua.create_function(|lua, margin: f32| Margin::to_lua(Margin::same(margin), lua))?,
        )?;
        margin.set(
            "symmetric",
            lua.create_function(|lua, (x, y): (f32, f32)| {
                Margin::to_lua(Margin::symmetric(x, y), lua)
            })?,
        )?;
        margin.set(
            "sum",
            lua.create_function(|lua, value: Value| {
                Vec2::to_lua(Margin::from_lua(value)?.sum(), lua)
            })?,
        )?;
        margin.set(
            "left_top",
            lua.create_function(|lua, value: Value| {
                Vec2::to_lua(Margin::from_lua(value)?.left_top(), lua)
            })?,
        )?;
        margin.set(
            "right_bottom",
            lua.create_function(|lua, value: Value| {
                Vec2::to_lua(Margin::from_lua(value)?.right_bottom(), lua)
            })?,
        )?;
        margin.set(
            "is_same",
            lua.create_function(|_, value: Value| Ok(Margin::from_lua(value)?.is_same()))?,
        )?;

        egui_table.set("rounding", margin)?;
        Ok(())
    }
    fn from_lua(value: Value) -> Result<Self> {
        Ok(match value {
            Value::Table(t) => {
                let left: f32 = t.get("left")?;
                let right: f32 = t.get("right")?;
                let top: f32 = t.get("top")?;
                let bottom: f32 = t.get("bottom")?;
                Self {
                    left,
                    right,
                    top,
                    bottom,
                }
            }
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "luavalue",
                    to: "pointerbutton",
                    message: None,
                })
            }
        })
    }

    fn to_lua(value: Self, lua: &Lua) -> Result<Value> {
        let margin = lua.create_table()?;
        margin.set("left", value.left)?;
        margin.set("right", value.right)?;
        margin.set("top", value.top)?;
        margin.set("bottom", value.bottom)?;
        Ok(Value::Table(margin))
    }
}
impl LuaHelperTrait for TextStyle {
    fn from_lua(value: Value) -> Result<Self> {
        match value {
            Value::Integer(i) => Ok(match i {
                0 => Self::Small,
                1 => Self::Body,
                2 => Self::Monospace,
                3 => Self::Button,
                4 => Self::Heading,
                _ => {
                    return Err(mlua::Error::RuntimeError(format!(
                        "the value {i} doesn't match any TextStyle enum variants"
                    )))
                }
            }),
            Value::String(s) => Ok(Self::Name(s.to_str().unwrap_or_default().into())),
            _ => {
                return Err(mlua::Error::RuntimeError(format!(
                    "invalid type to convert to TextStyle enum variants"
                )))
            }
        }
    }

    fn to_lua(value: Self, lua: &Lua) -> Result<Value> {
        Ok(match value {
            TextStyle::Small => Value::Integer(0),
            TextStyle::Body => Value::Integer(1),
            TextStyle::Monospace => Value::Integer(2),
            TextStyle::Button => Value::Integer(3),
            TextStyle::Heading => Value::Integer(4),
            TextStyle::Name(n) => Value::String(lua.create_string(n.as_bytes())?),
        })
    }

    fn add_to_lua(lua: &Lua, egui_table: &Table) -> Result<()> {
        let text_style = lua.create_table()?;
        text_style.set("small", Value::Integer(0))?;
        text_style.set("body", Value::Integer(1))?;
        text_style.set("monospace", Value::Integer(2))?;
        text_style.set("button", Value::Integer(3))?;
        text_style.set("heading", Value::Integer(4))?;
        text_style.set_readonly(true);
        egui_table.set("text_style", text_style)?;
        Ok(())
    }
}
impl LuaHelperTrait for Rounding {
    fn add_to_lua(lua: &Lua, egui_table: &Table) -> mlua::Result<()> {
        let rounding = lua.create_table()?;
        rounding.set(
            "same",
            lua.create_function(|lua, radius: f32| Rounding::to_lua(Rounding::same(radius), lua))?,
        )?;
        rounding.set(
            "none",
            lua.create_function(|lua, ()| Rounding::to_lua(Rounding::none(), lua))?,
        )?;
        rounding.set(
            "is_same",
            lua.create_function(|_, rounding: Value| -> mlua::Result<bool> {
                Ok(Rounding::from_lua(rounding)?.is_same())
            })?,
        )?;
        rounding.set(
            "atleast",
            lua.create_function(|lua, (value, min): (Value, f32)| {
                Rounding::to_lua(Rounding::from_lua(value)?.at_least(min), lua)
            })?,
        )?;
        rounding.set(
            "atmost",
            lua.create_function(|lua, (value, max): (Value, f32)| {
                Rounding::to_lua(Rounding::from_lua(value)?.at_most(max), lua)
            })?,
        )?;
        egui_table.set("rounding", rounding)?;
        Ok(())
    }
    fn from_lua(value: Value) -> Result<Self> {
        Ok(match value {
            Value::Table(t) => {
                let nw: f32 = t.get("nw")?;
                let ne: f32 = t.get("ne")?;
                let sw: f32 = t.get("sw")?;
                let se: f32 = t.get("se")?;
                Self { nw, ne, sw, se }
            }
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "luavalue",
                    to: "pointerbutton",
                    message: None,
                })
            }
        })
    }

    fn to_lua(value: Self, lua: &Lua) -> Result<Value> {
        let rounding = lua.create_table()?;
        rounding.set("nw", value.nw)?;
        rounding.set("ne", value.ne)?;
        rounding.set("sw", value.sw)?;
        rounding.set("se", value.se)?;
        Ok(Value::Table(rounding))
    }
}
impl LuaHelperTrait for Rect {
    fn add_to_lua(lua: &Lua, egui_table: &Table) -> mlua::Result<()> {
        let rect = lua.create_table()?;
        rect.set("everything", Rect::to_lua(Rect::EVERYTHING, lua)?)?;
        rect.set("nothing", Rect::to_lua(Rect::NOTHING, lua)?)?;
        rect.set("nan", Rect::to_lua(Rect::NAN, lua)?)?;
        rect.set(
            "from_min_max",
            lua.create_function(|lua, (min, max): (Value, Value)| {
                let min = Pos2::from_lua(min)?;
                let max = Pos2::from_lua(max)?;
                Rect::to_lua(Rect { min, max }, lua)
            })?,
        )?;
        egui_table.set("stroke", rect)?;
        Ok(())
    }
    fn from_lua(value: Value) -> Result<Self> {
        Ok(match value {
            Value::Table(t) => {
                let min = Pos2::from_lua(t.get("min")?)?;
                let max = Pos2::from_lua(t.get("max")?)?;
                Rect { min, max }
            }
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "luavalue",
                    to: "pointerbutton",
                    message: None,
                })
            }
        })
    }

    fn to_lua(value: Self, lua: &Lua) -> Result<Value> {
        let rect = lua.create_table()?;
        rect.set("min", Pos2::to_lua(value.min, lua)?)?;
        rect.set("max", Pos2::to_lua(value.max, lua)?)?;
        Ok(Value::Table(rect))
    }
}
impl LuaHelperTrait for Color32 {
    fn add_to_lua(lua: &Lua, egui_table: &Table) -> mlua::Result<()> {
        let color32 = lua.create_table()?;
        // multiply first align by 4 to push its bits to left.
        // second align will fit in the 2 bits
        color32.set("transparent", Color32::to_lua(Color32::TRANSPARENT, lua)?)?;
        color32.set("black", Color32::to_lua(Color32::BLACK, lua)?)?;
        color32.set("dark_gray", Color32::to_lua(Color32::DARK_GRAY, lua)?)?;
        color32.set("gray", Color32::to_lua(Color32::GRAY, lua)?)?;
        color32.set("light_gray", Color32::to_lua(Color32::LIGHT_GRAY, lua)?)?;
        color32.set("white", Color32::to_lua(Color32::WHITE, lua)?)?;
        color32.set("brown", Color32::to_lua(Color32::BROWN, lua)?)?;
        color32.set("dark_red", Color32::to_lua(Color32::DARK_RED, lua)?)?;
        color32.set("red", Color32::to_lua(Color32::RED, lua)?)?;
        color32.set("light_red", Color32::to_lua(Color32::LIGHT_RED, lua)?)?;
        color32.set("yellow", Color32::to_lua(Color32::YELLOW, lua)?)?;
        color32.set("light_yellow", Color32::to_lua(Color32::LIGHT_YELLOW, lua)?)?;
        color32.set("khaki", Color32::to_lua(Color32::KHAKI, lua)?)?;
        color32.set("dark_green", Color32::to_lua(Color32::DARK_GREEN, lua)?)?;
        color32.set("green", Color32::to_lua(Color32::GREEN, lua)?)?;
        color32.set("light_green", Color32::to_lua(Color32::LIGHT_GREEN, lua)?)?;
        color32.set("dark_blue", Color32::to_lua(Color32::DARK_BLUE, lua)?)?;
        color32.set("blue", Color32::to_lua(Color32::BLUE, lua)?)?;
        color32.set("light_blue", Color32::to_lua(Color32::LIGHT_BLUE, lua)?)?;
        color32.set("gold", Color32::to_lua(Color32::GOLD, lua)?)?;
        color32.set("debug_color", Color32::to_lua(Color32::DEBUG_COLOR, lua)?)?;
        color32.set(
            "temporary_color",
            Color32::to_lua(Color32::TEMPORARY_COLOR, lua)?,
        )?;

        color32.set(
            "from_rgba_premultiplied",
            lua.create_function(|lua, (r, g, b, a): (u8, u8, u8, u8)| {
                Color32::to_lua(Color32::from_rgba_premultiplied(r, g, b, a), lua)
            })?,
        )?;
        egui_table.set("color32", color32)?;
        Ok(())
    }

    fn from_lua(value: Value) -> Result<Self> {
        Ok(match value {
            Value::Integer(i) => {
                let c = i.to_le_bytes();

                Color32::from_rgba_premultiplied(c[0], c[1], c[2], c[3])
            }
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "luavalue",
                    to: "pointerbutton",
                    message: None,
                })
            }
        })
    }

    fn to_lua(value: Self, _lua: &Lua) -> Result<Value> {
        Ok(Value::Integer({
            let a = value.to_array();
            i32::from_le_bytes(a)
        }))
    }
}
impl LuaHelperTrait for Pos2 {
    fn from_lua(value: Value) -> Result<Self> {
        match value {
            Value::Vector(x, y, _) => Ok(Self { x, y }),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "luavalue",
                to: "pos2",
                message: None,
            }),
        }
    }

    fn to_lua(value: Self, _lua: &Lua) -> Result<Value> {
        Ok(Value::Vector(value.x, value.y, 0.0))
    }

    fn add_to_lua(_lua: &Lua, _egui_table: &Table) -> Result<()> {
        Ok(())
    }
}
impl LuaHelperTrait for Stroke {
    fn add_to_lua(lua: &Lua, egui_table: &Table) -> mlua::Result<()> {
        let stroke = lua.create_table()?;
        stroke.set("none", Stroke::to_lua(Stroke::NONE, lua)?)?;
        stroke.set(
            "new",
            lua.create_function(|lua, (width, color): (f32, Value)| {
                let color = Color32::from_lua(color)?;
                Stroke::to_lua(Stroke { width, color }, lua)
            })?,
        )?;
        egui_table.set("stroke", stroke)?;
        Ok(())
    }
    fn from_lua(value: Value) -> Result<Self> {
        match value {
            Value::Vector(width, color, _) => {
                let color = color.to_le_bytes();
                Ok(Self {
                    width,
                    color: Color32::from_rgba_premultiplied(color[0], color[1], color[2], color[3]),
                })
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "luavalue",
                to: "pos2",
                message: None,
            }),
        }
    }

    fn to_lua(value: Self, _lua: &Lua) -> Result<Value> {
        let width = value.width;
        let color = value.color.to_array();
        let color = f32::from_le_bytes(color);
        Ok(Value::Vector(width, color, 0.0))
    }
}
impl LuaHelperTrait for Vec2 {
    fn from_lua(value: Value) -> Result<Self> {
        match value {
            Value::Vector(x, y, _) => Ok(Self { x, y }),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "luavalue",
                to: "pos2",
                message: None,
            }),
        }
    }

    fn to_lua(value: Self, _lua: &Lua) -> Result<Value> {
        Ok(Value::Vector(value.x, value.y, 0.0))
    }

    fn add_to_lua(_lua: &Lua, _egui_table: &Table) -> Result<()> {
        Ok(())
    }
}
impl LuaHelperTrait for Align2 {
    fn add_to_lua(lua: &Lua, egui_table: &Table) -> mlua::Result<()> {
        let align = lua.create_table()?;
        // multiply first align by 4 to push its bits to left.
        // second align will fit in the 2 bits
        align.set("left_bottom", Value::Integer(2))?;
        align.set("left_center", Value::Integer(1))?;
        align.set("left_top", Value::Integer(0))?;
        align.set("center_bottom", Value::Integer(6))?;
        align.set("center_center", Value::Integer(5))?;
        align.set("center_top", Value::Integer(4))?;
        align.set("right_bottom", Value::Integer(10))?;
        align.set("right_center", Value::Integer(9))?;
        align.set("right_top", Value::Integer(8))?;
        // align.set("center", Value::Integer(1))?;
        // align.set("max", Value::Integer(2))?;
        egui_table.set("align2", align)?;
        Ok(())
    }
    fn from_lua(value: Value) -> Result<Self> {
        Ok(match value {
            Value::Integer(i) => match i {
                0 => Align2::LEFT_TOP,
                1 => Align2::LEFT_CENTER,
                2 => Align2::LEFT_BOTTOM,
                4 => Align2::CENTER_TOP,
                5 => Align2::CENTER_CENTER,
                6 => Align2::CENTER_BOTTOM,
                8 => Align2::RIGHT_TOP,
                9 => Align2::RIGHT_CENTER,
                10 => Align2::RIGHT_BOTTOM,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "luavalue",
                        to: "pointerbutton",
                        message: Some("integer value out of range".to_string()),
                    })
                }
            },
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "luavalue",
                    to: "pointerbutton",
                    message: None,
                })
            }
        })
    }

    fn to_lua(value: Self, _lua: &Lua) -> Result<Value> {
        Ok(Value::Integer(match value {
            Align2::LEFT_TOP => 0,
            Align2::LEFT_CENTER => 1,
            Align2::LEFT_BOTTOM => 2,
            Align2::CENTER_TOP => 4,
            Align2::CENTER_CENTER => 5,
            Align2::CENTER_BOTTOM => 6,
            Align2::RIGHT_TOP => 8,
            Align2::RIGHT_CENTER => 9,
            Align2::RIGHT_BOTTOM => 10,
        }))
    }
}

impl LuaHelperTrait for Align {
    fn from_lua(value: Value) -> Result<Self> {
        Ok(match value {
            Value::Integer(i) => match i {
                0 => Align::Min,
                1 => Align::Center,
                2 => Align::Max,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "luavalue",
                        to: "pointerbutton",
                        message: Some("integer value out of range".to_string()),
                    })
                }
            },
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "luavalue",
                    to: "pointerbutton",
                    message: None,
                })
            }
        })
    }

    fn to_lua(value: Self, _lua: &Lua) -> Result<Value> {
        Ok(Value::Integer(match value {
            Align::Min => 0,
            Align::Center => 1,
            Align::Max => 2,
        }))
    }

    fn add_to_lua(lua: &Lua, egui_table: &Table) -> Result<()> {
        let align = lua.create_table()?;
        align.set("min", Value::Integer(0))?;
        align.set("center", Value::Integer(1))?;
        align.set("max", Value::Integer(2))?;
        egui_table.set("align", align)?;
        Ok(())
    }
}
impl LuaHelperTrait for PointerButton {
    fn add_to_lua(lua: &Lua, egui_table: &Table) -> Result<()> {
        let pointer_button = lua.create_table()?;
        pointer_button.set("primary", Value::Integer(0))?;
        pointer_button.set("secondary", Value::Integer(1))?;
        pointer_button.set("middle", Value::Integer(2))?;
        pointer_button.set("extra1", Value::Integer(3))?;
        pointer_button.set("extra2", Value::Integer(4))?;
        egui_table.set("pointer_button", pointer_button)?;
        Ok(())
    }
    fn from_lua(value: Value) -> Result<egui::PointerButton> {
        Ok(match value {
            Value::Integer(i) => match i {
                0 => PointerButton::Primary,
                1 => PointerButton::Secondary,
                2 => PointerButton::Middle,
                3 => PointerButton::Extra1,
                4 => PointerButton::Extra2,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "luavalue",
                        to: "pointerbutton",
                        message: Some("integer value out of range".to_string()),
                    })
                }
            },
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "luavalue",
                    to: "pointerbutton",
                    message: None,
                })
            }
        })
    }

    fn to_lua(value: Self, _lua: &Lua) -> Result<Value> {
        Ok(Value::Integer(match value {
            PointerButton::Primary => 0,
            PointerButton::Secondary => 1,
            PointerButton::Middle => 2,
            PointerButton::Extra1 => 3,
            PointerButton::Extra2 => 4,
        }))
    }
}

impl LuaHelperTrait for Direction {
    fn add_to_lua(lua: &Lua, egui_table: &Table) -> Result<()> {
        let direction = lua.create_table()?;
        direction.set("left_to_right", Value::Integer(0))?;
        direction.set("right_to_left", Value::Integer(1))?;
        direction.set("top_down", Value::Integer(2))?;
        direction.set("bottom_up", Value::Integer(3))?;
        egui_table.set("pointer_button", direction)?;
        Ok(())
    }
    fn from_lua(value: Value) -> Result<Self> {
        Ok(match value {
            Value::Integer(i) => match i {
                0 => Direction::LeftToRight,
                1 => Direction::RightToLeft,
                2 => Direction::TopDown,
                3 => Direction::BottomUp,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "luavalue",
                        to: "pointerbutton",
                        message: Some("integer value out of range".to_string()),
                    })
                }
            },
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "luavalue",
                    to: "pointerbutton",
                    message: None,
                })
            }
        })
    }

    fn to_lua(value: Self, _lua: &Lua) -> Result<Value> {
        Ok(Value::Integer(match value {
            Self::LeftToRight => 0,
            Self::RightToLeft => 1,
            Self::TopDown => 2,
            Self::BottomUp => 3,
        }))
    }
}
impl LuaHelperTrait for WidgetText {
    fn from_lua(value: Value) -> Result<Self> {
        match value {
            Value::String(s) => Ok(s.to_str().unwrap_or_default().into()),
            Value::UserData(u) => {
                if let Ok(u) = u.borrow::<WidgetText>() {
                    Ok(u.clone())
                } else if let Ok(u) = u.borrow::<RichText>() {
                    Ok(u.clone().into())
                } else {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "userdata",
                        to: "widgettext",
                        message: None,
                    });
                }
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "luavalue",
                to: "widgettext",
                message: None,
            }),
        }
    }

    fn to_lua(value: Self, lua: &Lua) -> Result<Value> {
        Ok(mlua::Value::UserData(lua.create_any_userdata(value)?))
    }

    fn add_to_lua(lua: &Lua, _egui_table: &Table) -> Result<()> {
        lua.register_userdata_type(|_reg: &mut UserDataRegistrar<WidgetText>| {})?;
        Ok(())
    }
}

impl LuaHelperTrait for RichText {
    fn from_lua(value: Value) -> Result<Self> {
        match value {
            Value::String(s) => Ok(s.to_str().unwrap_or_default().into()),
            Value::UserData(u) => {
                if let Ok(u) = u.borrow::<RichText>() {
                    Ok(u.clone())
                } else {
                    Err(mlua::Error::FromLuaConversionError {
                        from: "userdata",
                        to: "widgettext",
                        message: None,
                    })
                }
            }
            Value::Table(_t) => {
                Err(mlua::Error::FromLuaConversionError {
                    from: "table",
                    to: "widgettext",
                    message: None,
                })
                // if let Ok(text) =  t.get::<_, String>("text")  {
                //     todo!()
                // } else {
                //     return e;
                // }
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "luavalue",
                to: "widgettext",
                message: None,
            }),
        }
    }

    fn to_lua(value: Self, lua: &Lua) -> Result<Value> {
        Ok(Value::UserData(lua.create_any_userdata(value)?))
    }

    fn add_to_lua(lua: &Lua, egui_table: &Table) -> Result<()> {
        lua.register_userdata_type(|reg: &mut UserDataRegistrar<RichText>| {
            reg.add_method("is_empty", |_, this, ()| Ok(this.is_empty()));
            reg.add_method("text", |_, this, ()| Ok(this.text().to_string()));
            reg.add_method("size", |lua, this, size: f32| {
                lua.create_any_userdata(this.clone().size(size))
            });
        })?;
        let rich_text = lua.create_table()?;
        rich_text.set(
            "new",
            lua.create_function(|lua, text: String| lua.create_any_userdata(RichText::new(text)))?,
        )?;

        egui_table.set("rich_text", rich_text)
    }
}

#[derive(Hash, Debug)]
enum LuaHashable<'lua> {
    LuaString(mlua::String<'lua>),
    Integer(i32),
}
impl<'lua> LuaHashable<'lua> {
    fn from_lua(value: Value<'lua>) -> Result<Self> {
        match value {
            Value::Integer(i) => Ok(Self::Integer(i)),
            Value::String(i) => Ok(Self::LuaString(i)),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "value",
                to: "LuaHashable",
                message: None,
            }),
        }
    }
}
fn add_style(lua: &Lua, egui_table: &Table) -> Result<()> {
    lua.register_userdata_type(|style: &mut UserDataRegistrar<Style>| {
        style.add_method_mut("ui", |_, this, mut ui: UserDataRefMut<Ui>| {
            this.ui(&mut ui);
            Ok(())
        });
    })?;
    let style = lua.create_table()?;
    style.set(
        "default",
        lua.create_function(|lua, _: ()| lua.create_any_userdata(Style::default()))?,
    )?;
    egui_table.set("style", style)?;
    Ok(())
}
fn add_frame(lua: &Lua, egui_table: &Table) -> Result<()> {
    let frame = lua.create_table()?;
    frame.set(
        "none",
        lua.create_function(|lua, _: ()| lua.create_any_userdata(Frame::none()))?,
    )?;
    frame.set(
        "group",
        lua.create_function(|lua, style: UserDataRef<Style>| {
            lua.create_any_userdata(Frame::group(&style))
        })?,
    )?;
    frame.set(
        "side_top_panel",
        lua.create_function(|lua, style: UserDataRef<Style>| {
            lua.create_any_userdata(Frame::side_top_panel(&style))
        })?,
    )?;
    frame.set(
        "central_panel",
        lua.create_function(|lua, style: UserDataRef<Style>| {
            lua.create_any_userdata(Frame::central_panel(&style))
        })?,
    )?;
    frame.set(
        "window",
        lua.create_function(|lua, style: UserDataRef<Style>| {
            lua.create_any_userdata(Frame::window(&style))
        })?,
    )?;
    frame.set(
        "menu",
        lua.create_function(|lua, style: UserDataRef<Style>| {
            lua.create_any_userdata(Frame::menu(&style))
        })?,
    )?;
    frame.set(
        "popup",
        lua.create_function(|lua, style: UserDataRef<Style>| {
            lua.create_any_userdata(Frame::popup(&style))
        })?,
    )?;
    frame.set(
        "canvas",
        lua.create_function(|lua, style: UserDataRef<Style>| {
            lua.create_any_userdata(Frame::canvas(&style))
        })?,
    )?;
    frame.set(
        "dark_canvas",
        lua.create_function(|lua, style: UserDataRef<Style>| {
            lua.create_any_userdata(Frame::dark_canvas(&style))
        })?,
    )?;
    egui_table.set("frame", frame)?;
    lua.register_userdata_type(|frame: &mut UserDataRegistrar<Frame>| {
        frame.add_field_method_get("inner_margin", |lua, this| -> Result<Value> {
            Margin::to_lua(this.inner_margin, lua)
        });
        frame.add_field_method_get("outer_margin", |lua, this| -> Result<Value> {
            Margin::to_lua(this.outer_margin, lua)
        });
        frame.add_field_method_get("rounding", |lua, this| -> Result<Value> {
            Rounding::to_lua(this.rounding, lua)
        });
        frame.add_field_method_get("shadow", |lua, this| -> Result<Value> {
            Ok(Value::UserData(lua.create_any_userdata(this.shadow)?))
        });
        frame.add_field_method_get("fill", |lua, this| -> Result<Value> {
            Color32::to_lua(this.fill, lua)
        });
        frame.add_field_method_get("stroke", |lua, this| -> Result<Value> {
            Stroke::to_lua(this.stroke, lua)
        });

        frame.add_method(
            "show",
            |lua, this, (mut ui, add_contents): (UserDataRefMut<Ui>, Function)| {
                let frame = *this;
                frame.show(&mut ui, |ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let _: () = add_contents.call(ui)?;
                        Ok(())
                    })
                    .unwrap();
                });
                Ok(())
            },
        )
    })?;
    Ok(())
}
fn add_shadow(lua: &Lua, _egui_table: &Table) -> Result<()> {
    lua.register_userdata_type(|shadow: &mut UserDataRegistrar<Shadow>| {
        shadow.add_field_method_get("extrusion", |_, this| Ok(this.extrusion));
        shadow.add_field_method_get("color", |lua, this| Color32::to_lua(this.color, lua));
        shadow.add_field_method_set("extrusion", |_, this, extrusion: f32| {
            this.extrusion = extrusion;
            Ok(())
        });
        shadow.add_field_method_set("color", |_lua, this, color: Value| {
            this.color = Color32::from_lua(color)?;
            Ok(())
        });
    })?;
    Ok(())
}
impl LuaHelperTrait for Order {
    fn from_lua(value: Value) -> Result<Self> {
        match value {
            Value::Integer(i) => Ok(match i {
                0 => Self::Background,
                1 => Self::PanelResizeLine,
                2 => Self::Middle,
                3 => Self::Foreground,
                4 => Self::Tooltip,
                5 => Self::Debug,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "Integer",
                        to: "Order",
                        message: Some("Integer value none of the enum variants".to_string()),
                    })
                }
            }),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "value",
                to: "Order",
                message: None,
            }),
        }
    }

    fn to_lua(value: Self, _lua: &Lua) -> Result<Value> {
        Ok(Value::Integer(match value {
            Order::Background => 0,
            Order::PanelResizeLine => 1,
            Order::Middle => 2,
            Order::Foreground => 3,
            Order::Tooltip => 4,
            Order::Debug => 5,
        }))
    }

    fn add_to_lua(lua: &Lua, egui_table: &Table) -> Result<()> {
        let order = lua.create_table()?;
        order.set("Background", Self::to_lua(Self::Background, lua)?)?;
        order.set("PanelResizeLine", Self::to_lua(Self::PanelResizeLine, lua)?)?;
        order.set("Middle", Self::to_lua(Self::Middle, lua)?)?;
        order.set("Foreground", Self::to_lua(Self::Foreground, lua)?)?;
        order.set("Tooltip", Self::to_lua(Self::Tooltip, lua)?)?;
        order.set("Debug", Self::to_lua(Self::Debug, lua)?)?;
        egui_table.set("order", order)?;
        Ok(())
    }
}

fn add_layer_id(lua: &Lua, egui_table: &Table) -> Result<()> {
    lua.register_userdata_type(|layer_id: &mut UserDataRegistrar<LayerId>| {
        layer_id.add_field_method_get("order", |lua, this| Order::to_lua(this.order, lua));
        layer_id.add_field_method_get("id", |lua, this| Id::to_lua(this.id, lua));
        layer_id.add_field_method_set("order", |_, this, order: Value| {
            this.order = Order::from_lua(order)?;
            Ok(())
        });
        layer_id.add_field_method_set("id", |_, this, value: Value| {
            this.id = Id::from_lua(value)?;
            Ok(())
        });
        layer_id.add_method("allow_interaction", |_, this, _: ()| {
            Ok(this.allow_interaction())
        });
        layer_id.add_method("short_debug_format", |_, this, _: ()| {
            Ok(this.short_debug_format())
        });
    })?;
    let layer_id = lua.create_table()?;
    layer_id.set(
        "new",
        lua.create_function(|lua, (order, id): (Value, Value)| {
            lua.create_any_userdata(LayerId::new(Order::from_lua(order)?, Id::from_lua(id)?))
        })?,
    )?;
    layer_id.set(
        "debug",
        lua.create_function(|lua, _: ()| lua.create_any_userdata(LayerId::debug()))?,
    )?;
    layer_id.set(
        "background",
        lua.create_function(|lua, _: ()| lua.create_any_userdata(LayerId::background()))?,
    )?;
    egui_table.set("layer_id", layer_id)?;
    Ok(())
}
fn add_area(lua: &Lua, egui_table: &Table) -> Result<()> {
    lua.register_userdata_type(|area: &mut UserDataRegistrar<Area>| {
        area.add_method_mut("anchor", |_, this, (align, offset): (Value, Value)| {
            *this = this.anchor(Align2::from_lua(align)?, Vec2::from_lua(offset)?);
            Ok(())
        });
        area.add_method_mut("constrain", |_, this, constrain: bool| {
            *this = this.constrain(constrain);
            Ok(())
        });
        area.add_method_mut("current_pos", |_, this, current_pos: Value| {
            *this = this.current_pos(Pos2::from_lua(current_pos)?);
            Ok(())
        });
        area.add_method_mut("default_pos", |_, this, default_pos: Value| {
            *this = this.default_pos(Pos2::from_lua(default_pos)?);
            Ok(())
        });
        area.add_method_mut("drag_bounds", |_, this, bounds: Value| {
            *this = this.drag_bounds(Rect::from_lua(bounds)?);
            Ok(())
        });
        area.add_method_mut("enabled", |_, this, enabled: bool| {
            *this = this.enabled(enabled);
            Ok(())
        });
        area.add_method_mut("movable", |_, this, movable: bool| {
            *this = this.movable(movable);
            Ok(())
        });
        area.add_method_mut("interactable", |_, this, interactable: bool| {
            *this = this.interactable(interactable);
            Ok(())
        });
        area.add_method("is_movable", |_, this, _: ()| Ok(this.is_movable()));
        area.add_method("is_enabled", |_, this, _: ()| Ok(this.is_enabled()));

        area.add_method_mut("fixed_pos", |_, this, value: Value| {
            *this = this.fixed_pos(Pos2::from_lua(value)?);
            Ok(())
        });
        area.add_method_mut("id", |_, this, id: UserDataRef<Id>| {
            *this = this.id(*id);
            Ok(())
        });
        area.add_method("layer", |lua, this, _: ()| {
            lua.create_any_userdata(this.layer())
        });
        area.add_method(
            "show",
            |lua, this, (ctx, add_contents): (UserDataRef<Context>, Function)| {
                let area = *this;
                let ctx = ctx.clone();
                area.show(&ctx, |ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let _: () = add_contents.call(ui)?;
                        Ok(())
                    })
                    .unwrap();
                });
                Ok(())
            },
        )
    })?;
    let area = lua.create_table()?;
    area.set(
        "new",
        lua.create_function(|lua, value: Value| {
            let area = Area::new(Id::from_lua(value)?);
            let ud = lua.create_any_userdata(area)?;
            Ok(Value::UserData(ud))
        })?,
    )?;
    egui_table.set("area", area)?;
    Ok(())
}

fn add_window(lua: &Lua, egui_table: &Table) -> Result<()> {
    lua.register_userdata_type(|window: &mut UserDataRegistrar<Option<Window<'static>>>| {
        window.add_method_mut("anchor", |_, this, (align, offset): (Value, Value)| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .anchor(Align2::from_lua(align)?, Vec2::from_lua(offset)?),
            );

            Ok(())
        });
        window.add_method_mut("auto_sized", |_, this, _: ()| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .auto_sized(),
            );
            Ok(())
        });
        window.add_method_mut("collapsible", |_, this, constrain: bool| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .collapsible(constrain),
            );
            Ok(())
        });
        window.add_method_mut("constrain", |_, this, constrain: bool| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .constrain(constrain),
            );
            Ok(())
        });
        window.add_method_mut("current_pos", |_, this, current_pos: Value| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .current_pos(Pos2::from_lua(current_pos)?),
            );
            Ok(())
        });
        window.add_method_mut("default_height", |_, this, default_width: f32| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .default_height(default_width),
            );
            Ok(())
        });
        window.add_method_mut("default_open", |_, this, constrain: bool| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .default_open(constrain),
            );
            Ok(())
        });
        window.add_method_mut("default_pos", |_, this, default_pos: Value| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .default_pos(Pos2::from_lua(default_pos)?),
            );
            Ok(())
        });
        window.add_method_mut("default_rect", |_, this, default_pos: Value| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .default_rect(Rect::from_lua(default_pos)?),
            );
            Ok(())
        });
        window.add_method_mut("default_size", |_, this, default_pos: Value| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .default_size(Vec2::from_lua(default_pos)?),
            );
            Ok(())
        });
        window.add_method_mut("default_width", |_, this, default_width: f32| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .default_width(default_width),
            );
            Ok(())
        });
        window.add_method_mut("drag_bounds", |_, this, bounds: Value| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .drag_bounds(Rect::from_lua(bounds)?),
            );
            Ok(())
        });
        window.add_method_mut("enabled", |_, this, enabled: bool| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .enabled(enabled),
            );
            Ok(())
        });
        window.add_method_mut("fixed_pos", |_, this, default_pos: Value| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .fixed_pos(Pos2::from_lua(default_pos)?),
            );
            Ok(())
        });
        window.add_method_mut("fixed_rect", |_, this, default_pos: Value| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .fixed_rect(Rect::from_lua(default_pos)?),
            );
            Ok(())
        });
        window.add_method_mut("fixed_size", |_, this, default_pos: Value| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .fixed_size(Vec2::from_lua(default_pos)?),
            );
            Ok(())
        });
        window.add_method_mut("frame", |_, this, frame: UserDataRef<Frame>| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .frame(frame.clone()),
            );
            Ok(())
        });
        window.add_method_mut("hscroll", |_, this, enabled: bool| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .hscroll(enabled),
            );
            Ok(())
        });
        window.add_method_mut("id", |_, this, id: UserDataRef<Id>| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .id(*id),
            );
            Ok(())
        });
        window.add_method_mut("interactable", |_, this, interactable: bool| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .interactable(interactable),
            );
            Ok(())
        });
        window.add_method_mut("min_height", |_, this, width: f32| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .min_height(width),
            );
            Ok(())
        });
        window.add_method_mut("min_width", |_, this, width: f32| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .min_width(width),
            );
            Ok(())
        });
        window.add_method_mut("movable", |_, this, movable: bool| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .movable(movable),
            );
            Ok(())
        });
        window.add_method_mut("pivot", |_, this, align: Value| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .pivot(Align2::from_lua(align)?),
            );
            Ok(())
        });

        window.add_method_mut("resizeable", |_, this, movable: bool| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .resizable(movable),
            );
            Ok(())
        });
        window.add_method_mut("scroll2", |_, this, (hscroll, vscroll): (bool, bool)| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .scroll2([hscroll, vscroll]),
            );
            Ok(())
        });
        window.add_method_mut(
            "show",
            |lua, this, (ctx, add_contents, open_table): (UserDataRef<Context>, Function, Option<Table>)| {
                let ctx = ctx.clone();
                let mut window = this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?;
                let mut open = true;
                let mut window_option_open_exists = false;
                if let Some(open_table) = open_table.as_ref() {
                    if let Ok(o) = open_table.get::<_, bool>("open") {
                        open = o;
                        window_option_open_exists = true;
                        window = window.open(&mut open)
                    }
                }
                let ir = window.show(&ctx, |ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                if window_option_open_exists {
                    open_table.unwrap().set("open", open)?;
                }
                let mut result = MultiValue::new();
                if let Some(ir) = ir {
                    let response = lua.create_any_userdata(ir.response)?;
                    result.push_front(Value::UserData(response));
                    if let Some(inner) = ir.inner {
                        let inner = inner?;
                        for v in inner {
                            result.push_front(v);
                        }
                    }
                }
                Ok(result)
            });
        window.add_method_mut("titlebar", |_, this, movable: bool| {
                *this = Some(
                    this.take()
                        .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                        .title_bar(movable),
                );
                Ok(())
            });
        window.add_method_mut("vscroll", |_, this, movable: bool| {
                *this = Some(
                    this.take()
                        .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                        .vscroll(movable),
                );
                Ok(())
            });

    })?;
    let window = lua.create_table()?;
    window.set(
        "new",
        lua.create_function(|lua, title: Value| {
            let w = Window::<'static>::new(WidgetText::from_lua(title)?);
            lua.create_any_userdata(Some(w))
        })?,
    )?;
    egui_table.set("window", window)?;
    Ok(())
}

fn add_central_panel(lua: &Lua, egui_table: &Table) -> Result<()> {
    lua.register_userdata_type(
        |central_panel: &mut UserDataRegistrar<Option<CentralPanel>>| {
            central_panel.add_method_mut("frame", |_, this, frame: UserDataRef<Frame>| {
                *this = Some(
                    this.take()
                        .ok_or_else(|| {
                            mlua::Error::RuntimeError("central paenl is null".to_owned())
                        })?
                        .frame(*frame),
                );
                Ok(())
            });
            central_panel.add_method_mut(
                "show",
                |lua, this, (ctx, add_contents): (UserDataRef<Context>, Function)| {
                    let ctx = ctx.clone();
                    let central_panel = this.take().ok_or_else(|| {
                        mlua::Error::RuntimeError("central panel is null".to_owned())
                    })?;

                    let ir = central_panel.show(&ctx, |ui| {
                        lua.scope(|scope| {
                            let ui = scope.create_any_userdata_ref_mut(ui)?;
                            let result: Result<MultiValue> = add_contents.call(ui);
                            result
                        })
                    });
                    let mut result = MultiValue::new();
                    let response = lua.create_any_userdata(ir.response)?;
                    result.push_front(Value::UserData(response));
                    let inner = ir.inner?;
                    for v in inner {
                        result.push_front(v);
                    }

                    Ok(result)
                },
            );
            central_panel.add_method_mut(
                "show_inside",
                |lua, this, (mut ui, add_contents): (UserDataRefMut<Ui>, Function)| {
                    let central_panel = this.take().ok_or_else(|| {
                        mlua::Error::RuntimeError("central panel is null".to_owned())
                    })?;

                    let ir = central_panel.show_inside(&mut ui, |ui| {
                        lua.scope(|scope| {
                            let ui = scope.create_any_userdata_ref_mut(ui)?;
                            let result: Result<MultiValue> = add_contents.call(ui);
                            result
                        })
                    });
                    let mut result = MultiValue::new();
                    let response = lua.create_any_userdata(ir.response)?;
                    result.push_front(Value::UserData(response));
                    let inner = ir.inner?;
                    for v in inner {
                        result.push_front(v);
                    }

                    Ok(result)
                },
            );
        },
    )?;
    let central_panel = lua.create_table()?;
    central_panel.set(
        "default",
        lua.create_function(|lua, _: ()| lua.create_any_userdata(Some(CentralPanel::default())))?,
    )?;
    egui_table.set("central_panel", central_panel)?;
    Ok(())
}
fn add_side_panel(lua: &Lua, egui_table: &Table) -> Result<()> {
    lua.register_userdata_type(|side_panel: &mut UserDataRegistrar<Option<SidePanel>>| {
        side_panel.add_method_mut("default_width", |_, this, default_width: f32| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .default_width(default_width),
            );
            Ok(())
        });
        side_panel.add_method_mut("exact_width", |_, this, default_width: f32| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .exact_width(default_width),
            );
            Ok(())
        });
        side_panel.add_method_mut("frame", |_, this, frame: UserDataRef<Frame>| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .frame(frame.clone()),
            );
            Ok(())
        });
        side_panel.add_method_mut("max_width", |_, this, width: f32| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .max_width(width),
            );
            Ok(())
        });
        side_panel.add_method_mut("min_width", |_, this, width: f32| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .min_width(width),
            );
            Ok(())
        });

        side_panel.add_method_mut("resizeable", |_, this, movable: bool| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .resizable(movable),
            );
            Ok(())
        });
        side_panel.add_method_mut(
            "show",
            |lua, this, (ctx, add_contents): (UserDataRef<Context>, Function)| {
                let ctx = ctx.clone();
                let side_panel = this
                    .take()
                    .ok_or_else(|| mlua::Error::RuntimeError("central panel is null".to_owned()))?;

                let ir = side_panel.show(&ctx, |ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                let mut result = MultiValue::new();
                let response = lua.create_any_userdata(ir.response)?;
                result.push_front(Value::UserData(response));
                let inner = ir.inner?;
                for v in inner {
                    result.push_front(v);
                }

                Ok(result)
            },
        );
        side_panel.add_method_mut(
            "show_inside",
            |lua, this, (mut ui, add_contents): (UserDataRefMut<Ui>, Function)| {
                let side_panel = this
                    .take()
                    .ok_or_else(|| mlua::Error::RuntimeError("central panel is null".to_owned()))?;

                let ir = side_panel.show_inside(&mut ui, |ui| {
                    lua.scope(|scope| {
                        let ui = scope.create_any_userdata_ref_mut(ui)?;
                        let result: Result<MultiValue> = add_contents.call(ui);
                        result
                    })
                });
                let mut result = MultiValue::new();
                let response = lua.create_any_userdata(ir.response)?;
                result.push_front(Value::UserData(response));
                let inner = ir.inner?;
                for v in inner {
                    result.push_front(v);
                }

                Ok(result)
            },
        );

        side_panel.add_method_mut("show_separator_line", |_, this, movable: bool| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .show_separator_line(movable),
            );
            Ok(())
        });

        side_panel.add_method_mut("width_range", |_, this, (start, end): (f32, f32)| {
            *this = Some(
                this.take()
                    .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                    .width_range(start..=end),
            );
            Ok(())
        });
    })?;
    let side_panel = lua.create_table()?;
    side_panel.set(
        "left",
        lua.create_function(|lua, title: Value| {
            let w = SidePanel::left(Id::from_lua(title)?);
            lua.create_any_userdata(Some(w))
        })?,
    )?;
    side_panel.set(
        "right",
        lua.create_function(|lua, title: Value| {
            let w = SidePanel::right(Id::from_lua(title)?);
            lua.create_any_userdata(Some(w))
        })?,
    )?;
    egui_table.set("side_panel", side_panel)?;
    Ok(())
}
fn add_top_bottom_panel(lua: &Lua, egui_table: &Table) -> Result<()> {
    lua.register_userdata_type(
        |top_bottom_panel: &mut UserDataRegistrar<Option<TopBottomPanel>>| {
            top_bottom_panel.add_method_mut("default_height", |_, this, default_width: f32| {
                *this = Some(
                    this.take()
                        .ok_or_else(|| {
                            mlua::Error::RuntimeError("top_bottom_panel is null".to_owned())
                        })?
                        .default_height(default_width),
                );
                Ok(())
            });
            top_bottom_panel.add_method_mut("exact_height", |_, this, default_width: f32| {
                *this = Some(
                    this.take()
                        .ok_or_else(|| {
                            mlua::Error::RuntimeError("top_bottom_panel is null".to_owned())
                        })?
                        .exact_height(default_width),
                );
                Ok(())
            });
            top_bottom_panel.add_method_mut("frame", |_, this, frame: UserDataRef<Frame>| {
                *this = Some(
                    this.take()
                        .ok_or_else(|| {
                            mlua::Error::RuntimeError("top_bottom_panel is null".to_owned())
                        })?
                        .frame(frame.clone()),
                );
                Ok(())
            });

            top_bottom_panel.add_method_mut("height_range", |_, this, (start, end): (f32, f32)| {
                *this = Some(
                    this.take()
                        .ok_or_else(|| {
                            mlua::Error::RuntimeError("top_bottom_panel is null".to_owned())
                        })?
                        .height_range(start..=end),
                );
                Ok(())
            });
            top_bottom_panel.add_method_mut("max_height", |_, this, width: f32| {
                *this = Some(
                    this.take()
                        .ok_or_else(|| {
                            mlua::Error::RuntimeError("top_bottom_panel is null".to_owned())
                        })?
                        .max_height(width),
                );
                Ok(())
            });
            top_bottom_panel.add_method_mut("min_height", |_, this, width: f32| {
                *this = Some(
                    this.take()
                        .ok_or_else(|| {
                            mlua::Error::RuntimeError("top_bottom_panel is null".to_owned())
                        })?
                        .min_height(width),
                );
                Ok(())
            });

            top_bottom_panel.add_method_mut("resizeable", |_, this, movable: bool| {
                *this = Some(
                    this.take()
                        .ok_or_else(|| {
                            mlua::Error::RuntimeError("top_bottom_panel is null".to_owned())
                        })?
                        .resizable(movable),
                );
                Ok(())
            });
            top_bottom_panel.add_method_mut(
                "show",
                |lua, this, (ctx, add_contents): (UserDataRef<Context>, Function)| {
                    let ctx = ctx.clone();
                    let side_panel = this.take().ok_or_else(|| {
                        mlua::Error::RuntimeError("top_bottom_panel is null".to_owned())
                    })?;

                    let ir = side_panel.show(&ctx, |ui| {
                        lua.scope(|scope| {
                            let ui = scope.create_any_userdata_ref_mut(ui)?;
                            let result: Result<MultiValue> = add_contents.call(ui);
                            result
                        })
                    });
                    let mut result = MultiValue::new();
                    let response = lua.create_any_userdata(ir.response)?;
                    result.push_front(Value::UserData(response));
                    let inner = ir.inner?;
                    for v in inner {
                        result.push_front(v);
                    }

                    Ok(result)
                },
            );
            top_bottom_panel.add_method_mut(
                "show_inside",
                |lua, this, (mut ui, add_contents): (UserDataRefMut<Ui>, Function)| {
                    let side_panel = this.take().ok_or_else(|| {
                        mlua::Error::RuntimeError("top_bottom_panel is null".to_owned())
                    })?;

                    let ir = side_panel.show_inside(&mut ui, |ui| {
                        lua.scope(|scope| {
                            let ui = scope.create_any_userdata_ref_mut(ui)?;
                            let result: Result<MultiValue> = add_contents.call(ui);
                            result
                        })
                    });
                    let mut result = MultiValue::new();
                    let response = lua.create_any_userdata(ir.response)?;
                    result.push_front(Value::UserData(response));
                    let inner = ir.inner?;
                    for v in inner {
                        result.push_front(v);
                    }

                    Ok(result)
                },
            );

            top_bottom_panel.add_method_mut("show_separator_line", |_, this, movable: bool| {
                *this = Some(
                    this.take()
                        .ok_or_else(|| mlua::Error::RuntimeError("window is null".to_owned()))?
                        .show_separator_line(movable),
                );
                Ok(())
            });
        },
    )?;
    let top_bottom_panel = lua.create_table()?;
    top_bottom_panel.set(
        "top",
        lua.create_function(|lua, title: Value| {
            let w = TopBottomPanel::top(Id::from_lua(title)?);
            lua.create_any_userdata(Some(w))
        })?,
    )?;
    top_bottom_panel.set(
        "bottom",
        lua.create_function(|lua, title: Value| {
            let w = TopBottomPanel::bottom(Id::from_lua(title)?);
            lua.create_any_userdata(Some(w))
        })?,
    )?;
    egui_table.set("top_bottom_panel", top_bottom_panel)?;
    Ok(())
}
