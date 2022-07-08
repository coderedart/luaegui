use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::*;
mod input;
use input::InputTokens;
#[proc_macro]
pub fn wrap_method(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InputTokens);

    // the type of method to add
    let method_type = match input.lua_impl_type {
        input::LuaImplType::Function => "add_function",
        input::LuaImplType::MethodRef => "add_method",
        input::LuaImplType::MethodMut => "add_method_mut",
        input::LuaImplType::MethodSelf => "add_method",
    };

    let method_type = format_ident!("{}", method_type);
    // the name of the inner function to call
    let fn_name = input.fn_name;
    // if its a function, we need to call it using the namespace of the inner type. so, we need to do a `type InnerType = egui::Type` inside the tealdata impl so that this macro would work
    // if its a &self or &mut self, we can simply call it directly
    // if its a self taking method, then we need to clone it first.
    let (self_ref, self_ref_usage) = match input.lua_impl_type {
        input::LuaImplType::Function => (quote!(), quote!(InnerType::#fn_name)),
        input::LuaImplType::MethodRef => (quote!(self_ref,), quote!(self_ref.#fn_name)),
        input::LuaImplType::MethodMut => (quote!(self_ref,), quote!(self_ref.#fn_name)),
        input::LuaImplType::MethodSelf => (quote!(self_ref,), quote!(self_ref.clone().#fn_name)),
    };

    // gather argument types to use inside the tuple for the wrapping closure
    let arg_types = input.args.iter().map(|a| &a.outer_type);
    let arg_names = 0..input.args.len();
    // create arg names like a0, a1 etc.. based on how many arguments there are.
    let arg_names = Vec::from_iter(
        arg_names
            .into_iter()
            .map(|index| format_ident!("a{}", index)),
    );
    // here we will create the quotes to be placed inside the closure before calling the inner fn. this will help convert the arguments to their appropriate types
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
    // return types
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

    // quotes that will be placed after calling the inner fn, but before returning the result. to convert the results to appropriate types
    // if there's already a last block provided, we will let it handle all conversions.
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
