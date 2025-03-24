use proc_macro2::TokenStream;
use quote::quote;
use std::fs;
use std::path::Path;
use std::process::Command;
use syn::{parse_quote, Type};

const GENERATED_RS: &str = "src/generated.rs";
const RUSTFMT: &str = "rustfmt";

fn main() {
    let primitives = vec![
        parse_quote! {()},
        parse_quote! {bool},
        parse_quote! {char},
        parse_quote! {String},
        parse_quote! {u8},
        parse_quote! {u16},
        parse_quote! {u32},
        parse_quote! {u64},
        parse_quote! {u128},
        parse_quote! {i8},
        parse_quote! {i16},
        parse_quote! {i32},
        parse_quote! {i64},
        parse_quote! {i128},
        parse_quote! {f32},
        parse_quote! {f64},
    ];
    let extensions_primitive = primitives.iter().map(coalesce_extension_impl_type);

    let extensions_primitive_ref = primitives
        .iter()
        .map(|t| Type::Reference(parse_quote! {&#t}))
        .map(coalesce_extension_impl_type_owned);

    let extensions_ref = vec![parse_quote!(&str)]
        .into_iter()
        .map(coalesce_extension_impl_type_owned);

    let generated_impl = quote! {
        use crate::extension::{Extension, WithExt};
        #(#extensions_primitive)*
        #(#extensions_primitive_ref)*
        #(#extensions_ref)*
    };

    let generated_rs = Path::new(GENERATED_RS);
    fs::write(&generated_rs, generated_impl.to_string()).unwrap();
    Command::new(RUSTFMT).arg(&generated_rs).status().unwrap();
}

fn coalesce_extension_impl_type(ident: &Type) -> TokenStream {
    quote! {
        #[doc = "Generated implementation"]
        impl Extension for #ident {
            fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
                other
            }
            fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
                base
            }
        }
    }
}
fn coalesce_extension_impl_type_owned(ident: Type) -> TokenStream {
    coalesce_extension_impl_type(&ident)
}
