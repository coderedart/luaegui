use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::*;
mod input;
use input::InputTokens;
#[proc_macro]
pub fn wrap_method(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InputTokens);

    let method_type = match input.lua_impl_type {
        input::LuaImplType::Function => "add_function",
        input::LuaImplType::MethodRef => "add_method",
        input::LuaImplType::MethodMut => "add_method_mut",
        input::LuaImplType::MethodSelf => "add_method",
    };

    let method_type = format_ident!("{}", method_type);
    let fn_name = input.fn_name;
    let (self_ref, self_ref_usage) = match input.lua_impl_type {
        input::LuaImplType::Function => (quote!(), quote!(InnerType::#fn_name)),
        input::LuaImplType::MethodRef => (quote!(self_ref,), quote!(self_ref.#fn_name)),
        input::LuaImplType::MethodMut => (quote!(self_ref,), quote!(self_ref.#fn_name)),
        input::LuaImplType::MethodSelf => (quote!(self_ref,), quote!(self_ref.clone().#fn_name)),
    };

    let arg_types = input.args.iter().map(|a| &a.outer_type);
    let arg_names = 0..input.args.len();
    let arg_names = Vec::from_iter(
        arg_names
            .into_iter()
            .map(|index| format_ident!("a{}", index)),
    );
    // let inner_arg_names = arg_names.iter();
    // let mut arbitrary_expressions: ArrayVec<[String; 16]> = ArrayVec::new();
    let inner_converter_expressions =
        input
            .args
            .iter()
            .zip(arg_names.iter())
            .map(|(arg, name)| match arg.converter {
                input::Converter::NoInto => {
                    quote!()
                }
                input::Converter::Into => {
                    quote!(let #name = #name.into();)
                }
                input::Converter::Ref => {
                    quote!(let #name = #name.as_ref();)
                }
                input::Converter::Clone => {
                    quote!(let #name = #name.clone();)
                }
                input::Converter::Block(ref e) => {
                    quote!(#e)
                }
            });
    let ret_types: Vec<_> = input.rets.iter().map(|a| &a.outer_type).collect();

    let return_type = match input.rets.len() {
        0 => quote!(()),
        1 => {
            let ret_type = ret_types[0];
            quote!(#ret_type)
        }
        _ => {
            quote!((#(#ret_types),*))
        }
    };
    let last = if let Some(last) = input.last {
        let last = last.stmts;
        quote!(#(#last)*)
    } else {
        match input.rets.len() {
            0 => quote!(),
            1 => quote!(let result = result.clone().into();),
            rest => {
                let index = (0..rest).into_iter().map(Index::from);

                quote!(let result = (#(result.#index.into()),*);)
            }
        }
    };
    let stream = quote!(
        methods.#method_type(stringify!(#fn_name), |lua: &Lua, #self_ref (#(mut #arg_names),*) : (#(#arg_types),*) | -> Result<#return_type> {
            #(#inner_converter_expressions)*
            let mut result = #self_ref_usage(#(#arg_names),*);
            #last
            Ok(result)
        });
    );
    stream.into()
}
