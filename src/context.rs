use crate::{add_container_methods, from_impl};
use derive_more::*;
use tealr::{mlu::*, *};
#[derive(Clone, MluaTealDerive, AsRef, AsMut)]
pub struct Context(egui::Context);

from_impl!(Context egui::Context);

impl tealr::mlu::TealData for Context {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("This is the Egui Context");
        methods.document_type("this will be given to the gui function, and can be used to create windows or other containers");
        methods.document_type("The containers will take a callback which will be given a Ui struct. that can be used by the callback to actually draw the user interface");

        add_container_methods(methods);
        methods.generate_help();
    }
}
