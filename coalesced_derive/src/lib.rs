use proc_macro::TokenStream;
use quote::ToTokens;

mod constant;
mod construction;
mod error;
mod semigroup;

#[proc_macro_derive(Construction, attributes(construction))]
pub fn derive_construction(input: TokenStream) -> TokenStream {
    let constants = constant::Constant::new();
    let derive = syn::parse_macro_input!(input);
    let attr = match construction::attr::ConstructionAttr::new(&derive) {
        Ok(attr) => attr,
        Err(e) => return e.into_compile_error().into(),
    };
    let construction =
        match construction::implementor::Construction::new(&constants, &derive, &attr) {
            Ok(construction) => construction,
            Err(e) => return e.into_compile_error().into(),
        };
    construction.into_token_stream().into()
}
