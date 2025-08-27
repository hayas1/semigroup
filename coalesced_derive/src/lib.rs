mod constant;
mod construction;
mod error;
mod semigroup;

#[proc_macro_derive(Construction, attributes(construction))]
pub fn derive_construction(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    construction::gen_construction::<constant::Absolute>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[cfg(feature = "use_scope")]
#[proc_macro_derive(ConstructionUse, attributes(construction))]
pub fn derive_construction_use(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    construction::gen_construction::<constant::Use>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Semigroup, attributes(semigroup))]
pub fn derive_semigroup(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    semigroup::gen_semigroup::<constant::Absolute>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[cfg(feature = "use_scope")]
#[proc_macro_derive(SemigroupUse, attributes(semigroup))]
pub fn derive_semigroup_use(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    semigroup::gen_semigroup::<constant::Use>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
