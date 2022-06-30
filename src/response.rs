use derive_more::*;
use tealr::{
    mlu::{mlua, TealDataMethods},
    *,
};

#[derive(Clone, MluaTealDerive, AsRef, AsMut, From, Deref)]
pub struct Response(egui::Response);

impl tealr::mlu::TealData for Response {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("This is the Egui Response");
        methods.add_method("clicked", |_, response, ()| Ok(response.clicked()));
    }
}
