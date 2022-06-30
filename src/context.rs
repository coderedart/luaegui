use tealr::{mlu::*, *};

use crate::add_container_methods;
#[derive(Clone, MluaTealDerive)]
pub struct Context(egui::Context);

impl AsRef<egui::Context> for Context {
    fn as_ref(&self) -> &egui::Context {
        &self.0
    }
}
impl AsMut<egui::Context> for Context {
    fn as_mut(&mut self) -> &mut egui::Context {
        &mut self.0
    }
}

impl From<egui::Context> for Context {
    fn from(e: egui::Context) -> Self {
        Self(e)
    }
}

impl Into<egui::Context> for Context {
    fn into(self) -> egui::Context {
        self.0
    }
}

impl tealr::mlu::TealData for Context {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("This is the Egui Context");
        methods.document_type("this will be given to the gui function, and can be used to create windows or other containers");
        methods.document_type("The containers will take a callback which will be given a Ui struct. that can be used by the callback to actually draw the user interface");

        add_container_methods(methods);
    }
}
