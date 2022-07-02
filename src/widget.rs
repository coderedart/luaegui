use crate::Response;

use egui::Widget;
use tealr::mlu::mlua::{Error, Table, Value};

pub trait LuaEguiWidget {
    fn from_table(ui: &mut egui::Ui, table: Table) -> Result<Response, Error>;
}

impl LuaEguiWidget for egui::Button {
    fn from_table(ui: &mut egui::Ui, table: Table) -> Result<Response, Error> {
        let text: String = table.get("text")?;
        let mut button = Self::new(text);
        if let Ok(Value::Boolean(wrap)) = table.get("wrap") {
            button = button.wrap(wrap);
        }
        Ok(Response::from(button.ui(ui)))
    }
}
