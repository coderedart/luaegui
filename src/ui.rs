use crate::{lua_registry_scoped_ui, ui_from_table, Response};
use derive_more::{Deref, DerefMut, From};
use tealr::{
    mlu::{
        mlua::{Error, Function, Lua, MultiValue, Table, UserData, UserDataMethods, Value},
        *,
    },
    *,
};

#[derive(From, Deref, DerefMut)]
pub struct Ui<'ui>(&'ui mut egui::Ui);

impl<'a> UserData for Ui<'a> {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method_mut("scope", |lua, ui, ui_function: Function| {
            let inner_response =
                ui.scope(|ui| lua_registry_scoped_ui!(lua, ui, |ui| ui_function.call(ui)));
            let response: Response = inner_response.response.into();
            let key = inner_response.inner?;
            let mut inner: Vec<Value> = lua
                .registry_value(&key)
                .expect("could not get registry value");
            lua.remove_registry_value(key)?;
            inner.insert(0, Value::UserData(lua.create_userdata(response)?));
            let inner = MultiValue::from_vec(inner);
            Ok(inner)
        });
        let mut x = UserDataWrapper::from_user_data_methods(methods);
        // x.add_method_mut("something", |lua, ui, ()|  {
        //     let ctx = crate::Context::from( ui.ctx().clone());
        //     let data =  lua.create_userdata(ctx.clone());

        //     Ok((ctx, data.map(|data| Some(Value::UserData(data)))?))

        // });

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
        // methods.document(
        //     "new scope to make some localized changes without affect the rest of the Ui after this",
        // );

        // methods.document("will create a new scope and add the ui after setting whether it should be enabled or not. won't affect other Ui after this");
        // methods.add_method_mut(
        //     "add_enabled_ui",
        //     |lua, ui, (enabled, ui_function): (bool, Function)| {
        //         let inner_response = ui.add_enabled_ui(enabled, |ui| {
        //             lua_registry_scoped_ui!(lua, ui, |ui| ui_function.call(ui))
        //         });
        //         let response: Response = inner_response.response.into();
        //         let key = inner_response.inner?;
        //         let inner: Value = lua
        //             .registry_value(&key)
        //             .expect("could not get registry value");
        //         lua.remove_registry_value(key)?;

        //         Ok((response, inner))
        //     },
        // );
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
fn add(lua: &Lua, ui: &mut Ui, table: Table) -> Result<Response, Error> {
    ui_from_table(lua, ui, table)
}
