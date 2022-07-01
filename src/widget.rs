use crate::{Response, Ui};

use egui::Widget;
use tealr::mlu::mlua::{Error, Function, Lua, Table, Value};

pub trait LuaEguiWidget {
    fn from_table(ui: &mut egui::Ui, table: Table) -> Result<Response, Error>;
}
pub fn ui_from_table(lua: &Lua, ui: &mut egui::Ui, table: Table) -> Result<Response, Error> {
    use tealr::mlu::mlua::String;
    let widget_name: String = table.get("widget_type")?;
    match widget_name.to_str()? {
        "custom" => {
            let ui_function: Function = table.get("ui")?;
            lua.scope(|scope| {
                let ui = scope.create_nonstatic_userdata(Ui::from(ui))?;
                ui_function.call((ui, table))
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
impl LuaEguiWidget for egui::Button {
    fn from_table(ui: &mut egui::Ui, table: Table) -> Result<Response, Error> {
        let text: String = table.get("text")?;
        let mut button = Self::new(text);
        if let Ok(Value::Boolean(wrap)) = table.get("wrap") {
            button = button.wrap(wrap);
        }
        Ok(button.ui(ui).into())
    }
}
