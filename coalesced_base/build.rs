use proc_macro2::TokenStream;
use quote::quote;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

const GENERATED_RS: &str = "src/generated.rs";
const RUSTFMT: &str = "rustfmt";
const EXTENSIONS_PRIMITIVE: &[&str] = &[
    "()", "bool", "char", "String", "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64",
    "i128", "f32", "f64",
];
const EXTENSIONS_REF: &[&str] = &["&str"];

fn main() {
    let extensions_primitive = EXTENSIONS_PRIMITIVE
        .iter()
        .cloned()
        .map(TokenStream::from_str)
        .map(Result::unwrap)
        .map(coalesce_extension_impl);
    let extensions_primitive_ref = EXTENSIONS_PRIMITIVE
        .iter()
        .map(|s| format!("&{}", s))
        .map(|s| TokenStream::from_str(&s))
        .map(Result::unwrap)
        .map(coalesce_extension_impl);
    let extensions_ref = EXTENSIONS_REF
        .iter()
        .cloned()
        .map(TokenStream::from_str)
        .map(Result::unwrap)
        .map(coalesce_extension_impl);

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

fn coalesce_extension_impl(ident: TokenStream) -> TokenStream {
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
