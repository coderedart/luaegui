use crate::{
    lua_registry_scoped_ui_extract, Color32, Context, Id, IntoIdSource, IntoRichText,
    IntoTextureId, IntoWidgetText, LayerId, Layout, LuaEguiWidget, Painter, Rect, Response, Sense,
    Spacing, Style, TextStyle, Vec2, Visuals,
};
use derive_more::{AsMut, AsRef, Deref, DerefMut, From};

use luaegui_derive::wrap_method;
use tealr::{
    mlu::{
        mlua::{Function, Lua, MultiValue, Result, Table, UserData, UserDataMethods, Value},
        *,
    },
    *,
};

#[derive(From, Deref, DerefMut, AsMut, AsRef)]
pub struct Ui<'ui>(&'ui mut egui::Ui);

impl<'a> UserData for Ui<'a> {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut x = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut x);
    }
    fn add_fields<'lua, F: ::tealr::mlu::mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl<'a> TypeName for Ui<'a> {
    fn get_type_parts() -> ::std::borrow::Cow<'static, [::tealr::NamePart]> {
        std::borrow::Cow::Borrowed(&[NamePart::Type(::tealr::TealType {
            name: std::borrow::Cow::Borrowed("Ui"),
            generics: None,
            type_kind: KindOfType::External,
        })])
    }
}

impl TypeBody for Ui<'static> {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}

impl<'a> TealData for Ui<'a> {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("This is the egui::Ui wrapper type");

        // Ui Creation functions
        // TODO: wrap Layout, Rect, ClipRect for creation functions
        // getter / setters
        wrap_method!(m; id;; Id);
        wrap_method!(m; style;; Style);
        methods.add_method_mut("set_style", |_, ui, a0: Style| {
            ui.set_style(a0);
            Ok(())
        });
        // TODO: style_mut
        wrap_method!(mm; reset_style);
        wrap_method!(m; spacing;; Spacing);
        wrap_method!(m; visuals;; Visuals);
        // TODO: spacing mut
        // TODO: visuals mut
        wrap_method!(m; ctx;; Context);
        wrap_method!(m; painter;; Painter);

        wrap_method!(m; is_enabled;; bool);
        wrap_method!(mm; set_enabled;  bool);

        wrap_method!(mm; set_visible; bool);
        wrap_method!(m; is_visible;; bool);
        wrap_method!(m; layout;; Layout);
        wrap_method!(m; wrap_text;; bool);
        wrap_method!(m; painter_at; Rect; Painter);
        wrap_method!(m; layer_id;; LayerId);

        // TODO all RWLock Guards functions
        methods.add_method("text_style_height", |_, ui, style: TextStyle| {
            Ok(ui.text_style_height(style.as_ref()))
        });
        wrap_method!(m; clip_rect;; Rect);
        wrap_method!(mm; set_clip_rect; Rect);
        wrap_method!(mm; is_rect_visible; Rect; bool);

        // Size related functions
        wrap_method!(m; min_rect;; Rect);
        wrap_method!(m; min_size;; Vec2);
        wrap_method!(m; max_rect;; Rect);
        wrap_method!(mm; set_max_size; Vec2);
        wrap_method!(mm; set_max_width; f32);
        wrap_method!(mm; set_max_height; f32);
        wrap_method!(mm; set_min_size; Vec2);
        wrap_method!(mm; set_min_width; f32);
        wrap_method!(mm; set_min_height; f32);
        wrap_method!(mm; shrink_width_to_current);
        wrap_method!(mm; shrink_height_to_current);
        wrap_method!(mm; expand_to_include_rect; Rect);

        methods.add_method_mut("set_width_range", |_, ui, (min, max): (f32, f32)| {
            ui.set_width_range(min..=max);
            Ok(())
        });
        methods.add_method_mut("set_height_range", |_, ui, (min, max): (f32, f32)| {
            ui.set_height_range(min..=max);
            Ok(())
        });

        wrap_method!(mm; set_width; f32);
        wrap_method!(mm; set_height; f32);

        wrap_method!(mm; expand_to_include_x; f32);
        wrap_method!(mm; expand_to_include_y; f32);

        // layout related measures
        wrap_method!(m; available_size;; Vec2);
        wrap_method!(m; available_width;; f32);
        wrap_method!(m; available_height;; f32);
        wrap_method!(m; available_size_before_wrap;; Vec2);
        wrap_method!(m; available_rect_before_wrap;; Rect);

        // Id creation
        methods.document("use this function to get a unique ID for your widget. you need to provide something that will remain unique for your widget. maybe its name or its position or whatever. but completely unique to this widget");
        methods.add_method("make_persistent_id", |_, ui, a0: IntoIdSource| {
            Ok(Id::from(ui.make_persistent_id(a0)))
        });

        // Interaction
        wrap_method!(m; interact; Rect , Id , Sense; Response);
        wrap_method!(m; rect_contains_pointer; Rect; bool);
        wrap_method!(m; ui_contains_pointer; ; bool);

        // Allocating space
        wrap_method!(mm; allocate_response; Vec2 , Sense; Response);
        wrap_method!(mm; allocate_exact_size; Vec2 , Sense; Rect , Response);
        wrap_method!(mm; allocate_at_least; Vec2 , Sense; Rect , Response);
        wrap_method!(mm; allocate_space; Vec2 ; Id, Rect);
        wrap_method!(mm; allocate_rect; Rect , Sense; Response );

        wrap_method!(m; cursor;; Rect);

        wrap_method!(m; next_widget_position; ; Vec2);
        // TODO: allocate ui
        // TODO: allocate ui with layout
        // TODO: allocate ui at rect
        wrap_method!(mm; allocate_painter; Vec2 , Sense; Response , Painter; {let result = (result.0.into(), result.1.into());});
        // TODO: wrap_method!(m; scroll_to_rect, Rect ; Option<Align>);
        // TODO: scroll_to_cursor
        wrap_method!(m; scroll_with_delta; Vec2);

        // adding Widgets
        methods.document(r#"
        This is a generic function that takes and adds a specific widget to the Ui.
        This takes a table as argument. below, you can see how the table will be used.
        The table represents a generic widget and what the fields mean will be decided by the widget itself. 
        The table must have a field called "widget_type" representing the type of widget with any of the following values:
            button, custom
        custom is a widget which is created inside lua itself to help addon makers reuse widgets. 
        all widgets will basically use this table and Ui to draw themselves. different widgets need different data.
        
        Button:
            text: string. the text to show inside the button.
            wrap: bool.   whether the button should wrap the inside text.
        "#);
        methods.add_method_mut("add", ui_add_fn);

        methods.add_method_mut("add_sized", |lua, ui, (a0, a1): (Vec2, Table)| {
            Ok(Response::from(ui.add_sized(a0, UiTable { lua, table: a1 })))
        });
        methods.add_method_mut("put", |lua, ui, (a0, a1): (Rect, Table)| {
            Ok(Response::from(
                ui.put(a0.into(), UiTable { lua, table: a1 }),
            ))
        });
        methods.add_method_mut("add_enabled", |lua, ui, (a0, a1): (bool, Table)| {
            Ok(Response::from(
                ui.add_enabled(a0, UiTable { lua, table: a1 }),
            ))
        });
        methods.document("will create a new scope and add the ui after setting whether it should be enabled or not. won't affect other Ui after this");
        methods.add_method_mut(
            "add_enabled_ui",
            |lua, ui, (enabled, ui_function): (bool, Function)| {
                let inner_response = ui.add_enabled_ui(enabled, |ui| {
                    lua_registry_scoped_ui_extract!(lua, ui, |ui| ui_function.call(ui))
                });

                Ok((
                    Response::from(inner_response.response),
                    inner_response.inner,
                ))
            },
        );

        methods.add_method_mut(
            "add_visible",
            |lua, ui, (visible, widget): (bool, Table)| {
                let response = ui.add_visible(visible, UiTable { lua, table: widget });
                Ok(Response::from(response))
            },
        );
        methods.add_method_mut(
            "add_visible_ui",
            |lua, ui, (visible, ui_function): (bool, Function)| {
                let inner_response = ui.add_visible_ui(visible, |ui| {
                    lua_registry_scoped_ui_extract!(lua, ui, |ui| ui_function.call(ui))
                });

                Ok((
                    Response::from(inner_response.response),
                    inner_response.inner,
                ))
            },
        );
        wrap_method!(mm; add_space; f32);
        methods.add_method_mut("button", |_, ui, text: IntoWidgetText| {
            Ok(Response::from(ui.button(text)))
        });
        methods.add_method_mut(
            "checkbox",
            |_, ui, (mut selected, text): (bool, IntoWidgetText)| {
                let response = Response::from(ui.checkbox(&mut selected, text));
                Ok((response, selected))
            },
        );
        methods.add_method_mut("code", |_, ui, rich_text: IntoRichText| {
            Ok(Response::from(ui.code(rich_text)))
        });
        methods.add_method_mut("code_editor", |_, ui, mut text: String| {
            let response = Response::from(ui.code_editor(&mut text));
            Ok((response, text))
        });

        methods.add_method_mut(
            "colored_label",
            |_, ui, (color, text): (Color32, IntoRichText)| {
                Ok(Response::from(ui.colored_label(color, text)))
            },
        );

        methods.add_method_mut("drag_angle", |_, ui, mut radians: f32| {
            let response = Response::from(ui.drag_angle(&mut radians));
            Ok((response, radians))
        });
        methods.add_method_mut("drag_angle_tau", |_, ui, mut radians: f32| {
            let response = Response::from(ui.drag_angle_tau(&mut radians));
            Ok((response, radians))
        });
        methods.add_method_mut("heading", |_, ui, rich_text: IntoRichText| {
            Ok(Response::from(ui.heading(rich_text)))
        });
        methods.add_method_mut("hyperlink", |_, ui, text: String| {
            Ok(Response::from(ui.hyperlink(text)))
        });

        methods.add_method_mut(
            "hyperlink_to",
            |_, ui, (text, url): (IntoWidgetText, String)| {
                Ok(Response::from(ui.hyperlink_to(text, url)))
            },
        );
        methods.add_method_mut(
            "image",
            |_, ui, (texture_id, size): (IntoTextureId, Vec2)| {
                Ok(Response::from(ui.image(texture_id, size)))
            },
        );
        methods.add_method_mut("label", |_, ui, text: IntoWidgetText| {
            Ok(Response::from(ui.label(text)))
        });
        methods.add_method_mut("link", |_, ui, text: IntoWidgetText| {
            Ok(Response::from(ui.link(text)))
        });
        methods.add_method_mut("monospace", |_, ui, rich_text: IntoRichText| {
            Ok(Response::from(ui.monospace(rich_text)))
        });
        methods.add_method_mut(
            "radio",
            |_, ui, (selected, text): (bool, IntoWidgetText)| {
                Ok(Response::from(ui.radio(selected, text)))
            },
        );
        // TODO: radio value and selectable value
        methods.add_method_mut(
            "selectable_label",
            |_, ui, (selected, text): (bool, IntoWidgetText)| {
                Ok(Response::from(ui.selectable_label(selected, text)))
            },
        );
        wrap_method!(mm; separator;; Response);
        methods.add_method_mut("small", |_, ui, rich_text: IntoRichText| {
            Ok(Response::from(ui.small(rich_text)))
        });
        methods.add_method_mut("small_button", |_, ui, text: IntoWidgetText| {
            Ok(Response::from(ui.small_button(text)))
        });

        wrap_method!(mm; spinner;; Response);
        methods.add_method_mut("strong", |_, ui, rich_text: IntoRichText| {
            Ok(Response::from(ui.strong(rich_text)))
        });

        methods.add_method_mut("text_edit_singleline", |_, ui, mut text: String| {
            let response = Response::from(ui.text_edit_singleline(&mut text));
            Ok((response, text))
        });
        methods.add_method_mut("text_edit_multiline", |_, ui, mut text: String| {
            let response = Response::from(ui.text_edit_multiline(&mut text));
            Ok((response, text))
        });
        methods.add_method_mut(
            "toggle_value",
            |_, ui, (mut selected, text): (bool, IntoWidgetText)| {
                let response = Response::from(ui.toggle_value(&mut selected, text));
                Ok((response, selected))
            },
        );
        methods.add_method_mut("weak", |_, ui, rich_text: IntoRichText| {
            Ok(Response::from(ui.weak(rich_text)))
        });

        // Colors
        methods.add_method_mut("color_edit_button_srgba", |_, ui, mut color: Color32| {
            let response = Response::from(ui.color_edit_button_srgba(&mut color));
            Ok((response, color))
        });
        // TODO: other color editing functions that take mut arrays

        // adding containers / sub uis
        methods.add_method_mut("group", |lua, ui, ui_function: Function| {
            let inner_response =
                ui.group(|ui| lua_registry_scoped_ui_extract!(lua, ui, |ui| ui_function.call(ui)));
            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
            ))
        });
        methods.add_method_mut("push_id", |lua, ui, (a0, a1): (IntoIdSource, Function)| {
            let inner_response = ui.push_id(a0, |ui| {
                lua_registry_scoped_ui_extract!(lua, ui, |ui| a1.call(ui))
            });
            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
            ))
        });
        methods.document(
            "new scope to make some localized changes without affect the rest of the Ui after this",
        );
        methods.add_method_mut("scope", |lua, ui, a0: Function| {
            let inner_response =
                ui.scope(|ui| lua_registry_scoped_ui_extract!(lua, ui, |ui| a0.call(ui)));

            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
            ))
        });
        methods.add_method_mut("with_layer_id", |lua, ui, (a0, a1): (LayerId, Function)| {
            let inner_response = ui.with_layer_id(a0.into(), |ui| {
                lua_registry_scoped_ui_extract!(lua, ui, |ui| a1.call(ui))
            });

            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
            ))
        });
        methods.document("this is one of the functions with a complicated return type");
        methods.document("this returns multiple values in the order of: Response of header, Response of body or nil, inner returns of body, openness (float)");
        methods.add_method_mut(
            "collapsing",
            |lua, ui, (a0, a1): (IntoWidgetText, Function)| {
                let inner_response = ui.collapsing(a0, |ui| {
                    lua_registry_scoped_ui_extract!(lua, ui, |ui| a1.call(ui))
                });
                let header_response =
                    lua.create_userdata(Response::from(inner_response.header_response))?;
                let body_response = match inner_response
                    .body_response
                    .map(Response::from)
                    .map(|r| lua.create_userdata(r))
                {
                    Some(r) => Value::UserData(r?),
                    None => Value::Nil,
                };
                let inner_values = inner_response
                    .body_returned
                    .map(MultiValue::into_vec)
                    .unwrap_or_default();
                let openness = inner_response.openness;
                let mut values = vec![Value::UserData(header_response), body_response];
                values.extend(inner_values.into_iter());
                values.push(Value::Number(openness as f64));
                Ok(MultiValue::from_vec(values))
            },
        );
        methods.add_method_mut("indent", |lua, ui, (a0, a1): (IntoIdSource, Function)| {
            let inner_response = ui.indent(a0, |ui| {
                lua_registry_scoped_ui_extract!(lua, ui, |ui| a1.call(ui))
            });
            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
            ))
        });
        methods.add_method_mut("horizontal", |lua, ui, a0: Function| {
            let inner_response =
                ui.horizontal(|ui| lua_registry_scoped_ui_extract!(lua, ui, |ui| a0.call(ui)));
            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
            ))
        });
        methods.add_method_mut("horizontal_centered", |lua, ui, a0: Function| {
            let inner_response = ui.horizontal_centered(|ui| {
                lua_registry_scoped_ui_extract!(lua, ui, |ui| a0.call(ui))
            });
            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
            ))
        });
        methods.add_method_mut("horizontal_top", |lua, ui, a0: Function| {
            let inner_response =
                ui.horizontal_top(|ui| lua_registry_scoped_ui_extract!(lua, ui, |ui| a0.call(ui)));
            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
            ))
        });
        methods.add_method_mut("horizontal_wrapped", |lua, ui, a0: Function| {
            let inner_response = ui.horizontal_wrapped(|ui| {
                lua_registry_scoped_ui_extract!(lua, ui, |ui| a0.call(ui))
            });
            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
            ))
        });
        methods.add_method_mut("vertical", |lua, ui, a0: Function| {
            let inner_response =
                ui.vertical(|ui| lua_registry_scoped_ui_extract!(lua, ui, |ui| a0.call(ui)));
            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
            ))
        });
        methods.add_method_mut("vertical_centered", |lua, ui, a0: Function| {
            let inner_response = ui
                .vertical_centered(|ui| lua_registry_scoped_ui_extract!(lua, ui, |ui| a0.call(ui)));
            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
            ))
        });
        methods.add_method_mut("vertical_centered_justified", |lua, ui, a0: Function| {
            let inner_response = ui.vertical_centered_justified(|ui| {
                lua_registry_scoped_ui_extract!(lua, ui, |ui| a0.call(ui))
            });
            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
            ))
        });
        methods.add_method_mut("with_layout", |lua, ui, (a0, a1): (Layout, Function)| {
            let inner_response = ui.with_layout(a0.into(), |ui| {
                lua_registry_scoped_ui_extract!(lua, ui, |ui| a1.call(ui))
            });
            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
            ))
        });
        methods.add_method_mut("centered_and_justified", |lua, ui, a0: Function| {
            let inner_response = ui.centered_and_justified(|ui| {
                lua_registry_scoped_ui_extract!(lua, ui, |ui| a0.call(ui))
            });
            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
            ))
        });
        wrap_method!(mm; end_row);
        wrap_method!(mm; set_row_height; f32;);
        methods.document(
            "unlike other ui callbacks, this callback is given an array (table) of Ui objects",
        );
        methods.document(
            "the ui objects can be indexed by the column number, and used to fill up each column",
        );
        methods.add_method_mut("columns", |lua, ui, (a0, a1): (usize, Function)| {
            let response = ui.columns(a0, |columns| {
                let key = lua
                    .scope(|scope| {
                        let columns_table =
                            lua.create_table_with_capacity(columns.len() as i32, 0)?;
                        for (index, ui) in columns.iter_mut().enumerate() {
                            let ui = scope
                                .create_nonstatic_userdata(Ui::from(ui))
                                .expect("failed to create non static userdata with Ui");
                            columns_table.set(index + 1, ui)?; // lua indexing starts from 1
                        }

                        let response: MultiValue =
                            a1.call(columns_table).expect("ui function returned error");
                        lua.create_registry_value(response.into_vec())
                    })
                    .expect("failed to get registry key");

                let value: Vec<Value> = lua
                    .registry_value(&key)
                    .expect("failed to get registry value");
                lua.remove_registry_value(key)
                    .expect("failed to remove registry value");
                MultiValue::from_vec(value)
            });
            Ok(response)
        });

        wrap_method!(mm; close_menu;);
        methods.add_method_mut(
            "menu_button",
            |lua, ui, (a0, a1): (IntoWidgetText, Function)| {
                let inner_response = ui.menu_button(a0, |ui| {
                    lua_registry_scoped_ui_extract!(lua, ui, |ui| a1.call(ui))
                });
                Ok((
                    Response::from(inner_response.response),
                    inner_response.inner.unwrap_or_default(),
                ))
            },
        );
        // debug stuff
        wrap_method!(m; debug_paint_cursor);

        // luaegui_derive::wrap_method!(m; trace_location; String nointo);
        // wrap_method!(m; trace_location, String);
    }
}

fn ui_add_fn(lua: &Lua, ui: &mut Ui, table: Table) -> Result<Response> {
    use tealr::mlu::mlua::String;
    let widget_name: String = table.get("widget_type")?;
    match widget_name.to_str()? {
        "custom" => {
            let ui_function: Function = table.get("ui")?;
            lua.scope(|scope| {
                let ui = Ui(ui);
                let ui = scope.create_nonstatic_userdata(ui)?;
                let response: Response = ui_function.call((ui, table))?;
                Ok(response)
            })
        }
        rest => match rest {
            "button" => egui::Button::from_table(ui, table),
            _ => {
                todo!()
            }
        },
    }
}
struct UiTable<'lua> {
    lua: &'lua Lua,
    table: Table<'lua>,
}
impl<'lua> egui::Widget for UiTable<'lua> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui_add_fn(self.lua, &mut Ui(ui), self.table)
            .expect("widget failed to render inside Widget trait")
            .0
    }
}
impl TypeName for UiTable<'static> {
    fn get_type_parts() -> std::borrow::Cow<'static, [NamePart]> {
        new_type!(UiTable)
    }
}
