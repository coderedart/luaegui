use crate::{
    lua_registry_scoped_ui_extract, Color32, IntoRichText, LuaEguiWidget, Response, RichText,
};
use derive_more::{AsMut, AsRef, Deref, DerefMut, From};
use tealr::{
    mlu::{
        mlua::{Function, Lua, MultiValue, Table, UserData, UserDataMethods, Value},
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

        methods.document("this function just shows the text. only argument is string");
        methods.add_method_mut("label", |_, ui, text: std::string::String| {
            ui.label(&text);
            Ok(())
        });
        methods.document(UI_ADD_DOCS);
        methods.add_method_mut("add", add);
        methods.document(
            "makes the Ui unable to be interacted with input. once set, it cannot be unset.",
        );
        methods.add_method_mut("set_enabled", |_, ui, enabled: bool| {
            ui.set_enabled(enabled);
            Ok(())
        });

        methods.document(
            "new scope to make some localized changes without affect the rest of the Ui after this",
        );
        methods.add_method_mut("scope", |lua, ui, ui_function: Function| {
            let inner_response =
                ui.scope(|ui| lua_registry_scoped_ui_extract!(lua, ui, |ui| ui_function.call(ui)));

            Ok((
                Response::from(inner_response.response),
                inner_response.inner,
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
        methods.add_method_mut("add_space", |_, ui, amount: f32| Ok(ui.add_space(amount)));
        methods.add_method_mut(
            "colored_label",
            |_, ui, (color, text): (Color32, IntoRichText)| {
                let text: RichText = text.into();
                Ok(Response::from(ui.colored_label(*color.as_ref(), text)))
            },
        );
    }
}
struct UiTable<'lua> {
    lua: &'lua Lua,
    table: Table<'lua>,
}
impl<'lua> egui::Widget for UiTable<'lua> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        add(self.lua, &mut Ui(ui), self.table)
            .expect("widget failed to render inside Widget trait")
            .0
    }
}
impl TypeName for UiTable<'static> {
    fn get_type_parts() -> std::borrow::Cow<'static, [NamePart]> {
        new_type!(UiTable)
    }
}
const UI_ADD_DOCS: &str = r#"
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
"#;
fn add<'lua>(lua: &'lua Lua, ui: &mut Ui, table: Table) -> Result<Response, mlua::Error> {
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
        rest => {
            let response = match rest {
                "button" => egui::Button::from_table(ui, table),
                _ => {
                    todo!()
                }
            };
            response
        }
    }
}
