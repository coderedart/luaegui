use egui::{Window};
use tealr::mlu::{
    mlua::{Error, Function, Lua, String as LuaString, Table, Value},
    TealDataMethods,
};

use crate::{Context, Ui};

pub fn add_container_methods<'lua, T: TealDataMethods<'lua, Context>>(methods: &mut T) {
    methods.document("takes a table which has the relevant options set and a ui callback which will be called if window is not collapsed");
    methods.document(
        "the following fields maybe set in the table. only title is required and rest are optional",
    );
    methods.document("title : string");
    methods.document("open: bool. true is window shown or open. false is window not displayed or closed. we will set this field if user clicks the close button on top right of window");
    methods.add_method("new_window", new_window);
}
pub fn new_window(lua: &Lua, context: &Context, args: (Table, Function)) -> Result<(), Error> {
    let options = args.0;
    let options_clone = options.clone();
    let ui_function = args.1;
    let title = options.get::<_, String>("title")?;
    let mut window = Window::new(title);
    let pairs = options.pairs::<LuaString, Value>();
    let mut open = true;
    for pair in pairs {
        let (k, v) = pair?;
        let k = k.to_str()?;
        match k {
            "open" => {
                open = if let Value::Boolean(b) = v { b } else { true };
            }

            _ => {}
        }
    }
    window = window.open(&mut open);

    let mut result = Ok(());
    window.show(context.as_ref(), |ui| {
        result = lua.scope(|scope| {
            let temp_ui = scope
                .create_nonstatic_userdata(Ui::from(ui))
                .expect("failed to create temporary ui");
            ui_function.call(temp_ui)
        });
    });
    options_clone.set("open", open)?;
    result
}
