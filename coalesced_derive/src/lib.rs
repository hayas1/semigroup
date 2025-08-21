mod constant;
mod construction;
mod error;
mod semigroup;

#[proc_macro_derive(Construction, attributes(construction))]
pub fn derive_construction(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    construction::gen::gen_construction::<constant::Absolute>(&derive).into()
}

#[cfg(feature = "use_scope")]
#[proc_macro_derive(ConstructionUse, attributes(construction))]
pub fn derive_construction_use(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    construction::gen::gen_construction::<constant::Use>(&derive).into()
}
