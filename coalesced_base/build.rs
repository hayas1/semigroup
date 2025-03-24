use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::fs;
use std::path::Path;
use std::process::Command;
use syn::{parse_quote, Type};

const GENERATED_RS: &str = "src/generated.rs";
const RUSTFMT: &str = "rustfmt";

fn main() {
    let generated_impl = Implementor::default().gen();
    let generated_rs = Path::new(GENERATED_RS);
    fs::write(&generated_rs, generated_impl.to_string()).unwrap();
    Command::new(RUSTFMT).arg(&generated_rs).status().unwrap();
}

struct Implementor {
    primitives: Vec<Type>,
    reference: Vec<Type>,
}
impl Default for Implementor {
    fn default() -> Self {
        Self {
            primitives: vec![
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
            ],
            reference: vec![parse_quote! {&str}],
        }
    }
}
impl Implementor {
    fn gen(&self) -> TokenStream {
        let impl_primitive = self.gen_primitive_owned();
        let impl_primitive_ref = self.gen_primitive_ref();
        let impl_primitive_mut = self.gen_primitive_mut();
        let impl_ref = self.gen_ref();
        quote! {
            use crate::extension::{Extension, WithExt};
            #(#impl_primitive)*
            #(#impl_primitive_ref)*
            #(#impl_primitive_mut)*
            #(#impl_ref)*
        }
    }
    fn gen_primitive_owned<'a>(&'a self) -> impl 'a + Iterator<Item = TokenStream> {
        self.primitives
            .iter()
            .map(Self::impl_coalesce_extension_primitive)
    }
    fn gen_primitive_ref<'a>(&'a self) -> impl 'a + Iterator<Item = TokenStream> {
        self.primitives
            .iter()
            .map(|t| Type::Reference(parse_quote! {&#t}))
            .map(Self::impl_coalesce_extension_primitive)
    }
    fn gen_primitive_mut<'a>(&'a self) -> impl 'a + Iterator<Item = TokenStream> {
        self.primitives
            .iter()
            .map(|t| Type::Reference(parse_quote! {&mut #t}))
            .map(Self::impl_coalesce_extension_primitive)
    }
    fn gen_ref<'a>(&'a self) -> impl 'a + Iterator<Item = TokenStream> {
        self.reference
            .iter()
            .map(Self::impl_coalesce_extension_primitive)
    }

    fn impl_coalesce_extension_primitive<T: ToTokens>(ident: T) -> TokenStream {
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
}
