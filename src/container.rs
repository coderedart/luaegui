use egui::Window;
use tealr::mlu::{
    mlua::{Error, Function, Lua, String as LuaString, Table, Value},
    TealDataMethods,
};

use crate::{Context, Ui};

pub fn add_container_methods<'lua, T: TealDataMethods<'lua, Context>>(methods: &mut T) {
    methods.document(NEW_WINDOW_DOCS);
    methods.add_method("new_window", new_window);
}
const NEW_WINDOW_DOCS: &str = r#"
Creates a new Window.
Args:
1. Table with options for the Window
2. A function which takes a Ui as the argument and adds whatever it wants to the Window's ui. The function is only run IF Window is not collapsed.
the following options can be set in the first argument table.
    title : string. the only required argument. rest are optional.
    open: bool. only mutable field. if close button on window is clicked, lua will set this field to false. 

"#;
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
