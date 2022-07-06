use proc_macro2::Span;
use syn::{parse::Parse, token::Brace, *};
use tinyvec::ArrayVec;

/*
the syntax is roughly split into 4 parts
1. whether we are wrapping a function or method or method_ref or a method_self (takes self by value)
2. the name of the function / method we are wrapping.
3. the arguments
4. the return types

The arguments themselves are divided into 3 parts
1. the outer type (we get from lua)
2. the function to use to convert outer to inner, before passing it to original fn
3. if the arguments need more complex conversions, then provide a block instead that will be run before calling the inner fn

The return types are divided into 3 parts.
1. the outer type that we give to lua
2. the function to use to convert the value we got from original fn into the outer type
3. if the returns need more complex  conversions, provide a block that will be placed after the inner fn is called

The syntax is like this:
1. one of f / m / mm / ms to indicate the fn, method ref, method mut ref or method self wrapping
2. the name of fn
3. the arguments separated .
    1. first ident is the outer type
    2. second is the conversion method to use
*/
pub struct InputTokens {
    pub lua_impl_type: LuaImplType,
    pub fn_name: Ident,
    pub args: ArrayVec<[Arg; 4]>,
    pub rets: ArrayVec<[Arg; 4]>,
    pub last: Option<Block>,
}

impl Parse for InputTokens {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let lua_impl_type = input.parse::<LuaImplType>()?;

        input.parse::<Token!(;)>().map_err(|mut e| {
            e.combine(input.error("semicolon token required after method type"));
            e
        })?;
        let fn_name = input.parse::<Ident>().map_err(|mut e| {
            e.combine(input.error("the function name of the fn we are wrapping is required"));
            e
        })?;
        let mut input_tokens = Self {
            lua_impl_type,
            fn_name,
            args: ArrayVec::new(),
            rets: ArrayVec::new(),
            last: None,
        };
        if input.is_empty() {
            return Ok(input_tokens);
        }
        input.parse::<Token!(;)>().map_err(|mut e| {
            e.combine(
                input.error("semicolon token required after fn name if arguments are to be used"),
            );
            e
        })?;

        for arg_count in 0..8usize {
            if input.peek(Token!(;)) {
                input.parse::<Token!(;)>()?;
                break;
            }
            if input.is_empty() {
                break;
            }
            if input_tokens.args.len() == 4 {
                return Err(input.error("more than 4 argments are NOT supported yet"));
            }
            let outer_type = input.parse::<Ident>().map_err(|mut e| {
                e.combine(input.error("expected Type Name here for the argument"));
                e
            })?;
            let mut arg = Arg {
                outer_type,
                ..Default::default()
            };
            if input.peek(Ident) | input.peek(Brace) {
                arg.converter = input.parse::<Converter>()?;
            }

            input_tokens.args.push(arg);

            assert_eq!(input_tokens.args.len(), arg_count + 1); // check that current arg count is equal to total args count
            if input.peek(Token!(,)) {
                input.parse::<Token!(,)>()?;
            } else if !input.peek(Token!(;)) && !input.is_empty() {
                // if the input is not comma, not semicolon and not empty, then user made a syntax error.
                let mut e = input
                .error("comma or semicolon required after the Argument type / converter before starting the next argument or return types list respectively");

                e.combine(input.error(format!("remaining tokens: {}", &input)));
                return Err(e);
            }
        }
        if input.is_empty() {
            return Ok(input_tokens);
        }
        for arg_count in 0..8usize {
            if input.peek(Token!(;)) {
                input.parse::<Token!(;)>()?;
                break;
            }
            if input.is_empty() {
                break;
            }
            if input_tokens.rets.len() == 4 {
                return Err(input.error("more than 4 argments are NOT supported yet"));
            }
            let outer_type = input.parse::<Ident>().map_err(|mut e| {
                e.combine(input.error("expected Type Name here for the argument"));
                e
            })?;
            let mut arg = Arg {
                outer_type,
                ..Default::default()
            };
            if input.peek(Ident) | input.peek(Brace) {
                arg.converter = input.parse::<Converter>()?;
            }

            input_tokens.rets.push(arg);

            assert_eq!(input_tokens.rets.len(), arg_count + 1); // check that current arg count is equal to total args count
            if input.peek(Token!(,)) {
                input.parse::<Token!(,)>()?;
            } else if !input.peek(Token!(;)) && !input.is_empty() {
                // if the input is not comma, not semicolon and not empty, then user made a syntax error.
                let mut e = input
                .error("comma or semicolon required after the Return type / converter before starting the next Return type list respectively");

                e.combine(input.error(format!("remaining tokens: {}", &input)));
                return Err(e);
            }
        }

        if input.peek(Brace) {
            input_tokens.last = Some(input.parse::<Block>().map_err(| mut e| {
                e.combine(input.error("expected a block here that will be put before the return statement, but after calling the inner fn"));
                e
            })?);
        }
        if input.peek(Token!(;)) {
            input.parse::<Token!(;)>()?;
        }
        if !input.is_empty() {
            return Err(input.error(format!(
                "tokens still remaining after all the processing. tokens: {}",
                &input
            )));
        }
        Ok(input_tokens)
    }
}

#[derive(Debug)]
pub enum LuaImplType {
    Function,
    MethodRef,
    MethodMut,
    MethodSelf,
}
impl Parse for LuaImplType {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let i = input.parse::<Ident>().map_err(|mut e| {
            e.combine(input.error(
                "please provide a valid lua method type. valid idents => [f | m | mm | ms]",
            ));
            e
        })?;
        let lua_impl_type = LuaImplType::convert(i)?;
        Ok(lua_impl_type)
    }
}
impl LuaImplType {
    fn convert(i: Ident) -> Result<Self> {
        if i == "f" {
            Ok(Self::Function)
        } else if i == "m" {
            Ok(Self::MethodRef)
        } else if i == "mm" {
            Ok(Self::MethodMut)
        } else if i == "ms" {
            Ok(Self::MethodSelf)
        } else {
            Err(Error::new(i.span(),"need a valid ident token to indicate whether this is a function, method_ref, method_mut_ref or method self. valid idents => [f | m | mm | ms"))
        }
    }
}

pub struct Arg {
    pub outer_type: Ident,
    pub converter: Converter,
}
impl Default for Arg {
    fn default() -> Self {
        Self {
            outer_type: Ident::new("fake_type_for_default_impl_for_tinyvec", Span::call_site()),
            converter: Default::default(),
        }
    }
}

pub enum Converter {
    NoInto,
    Into,
    Ref,
    Clone,
    Block(Block),
}
impl Default for Converter {
    fn default() -> Self {
        Self::Into
    }
}
impl Parse for Converter {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        if input.peek(Brace) {
            let content = input.parse::<Block>()?;
            return Ok(Self::Block(content));
        }
        let converter = input.parse::<Ident>()?;
        if converter == "clone" {
            Ok(Self::Clone)
        } else if converter == "nointo" {
            Ok(Self::NoInto)
        } else if converter == "ref" {
            Ok(Self::Ref)
        } else {
            Err(input.error("Invalid Converter for Argument. valid converters => [ clone | nointo | ref | { some code in braces } ]"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::InputTokens;
    use quote::quote;

    #[test]
    fn check_input_syntax() {
        let tokens = quote!(m ; interact; f32, Vec clone, String {let a2 = whatever;}; );
        let _ = syn::parse2::<InputTokens>(tokens).expect("failed to parse InputTokens Type");
    }
}
