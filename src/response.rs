use derive_more::*;
use tealr::{
    mlu::{mlua, TealDataMethods},
    *,
};

#[derive(Clone, MluaTealDerive, AsRef, AsMut, From, Deref)]
pub struct Response(pub egui::Response);

impl tealr::mlu::TealData for Response {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("This is the Egui Response");
        methods.add_method("clicked", |_, response, ()| Ok(response.clicked()));
    }
}

// #[derive(MluaTealDerive, AsRef, AsMut, From, Deref)]
// pub struct InnerResponse(egui::InnerResponse<Result<Option<Response>, Error>>);

// impl TealData for InnerResponse {
//     fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
//         methods.add_method("inner", |_, inner_response, ()| {
//             Ok(inner_response.inner.clone()?)
//         });
//         methods.add_method("response", |_, inner_response, ()| {
//             Ok(Response::from(inner_response.response.clone()))
//         })
//     }
// }
