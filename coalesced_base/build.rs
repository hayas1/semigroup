use std::{fs, path::Path, process::Command};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Type};

const GENERATED_RS: &str = "src/generated.rs";
const RUSTFMT: &str = "rustfmt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let targets = Target::targets().map(Target::implement);
    let generated_impl = Implementor::generate(targets);
    let generated_rs = Path::new(GENERATED_RS);
    fs::write(generated_rs, generated_impl.to_string())?;
    Command::new(RUSTFMT).arg(generated_rs).status()?;
    Ok(())
}

enum Implementor {
    Extension,
}
impl Implementor {
    fn generate<I: IntoIterator<Item = TokenStream>>(implementations: I) -> TokenStream {
        let impl_iter = implementations.into_iter();
        quote! {
            use crate::extension::{Extension, WithExt};
            #(#impl_iter)*
        }
    }

    fn basic_implement<I: ToTokens, G: ToTokens>(&self, ident: I, generics: G) -> TokenStream {
        match self {
            Self::Extension => quote! {
                #[doc = "Generated implementation"]
                impl #generics Extension for #ident #generics {
                    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
                        other
                    }
                    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
                        base
                    }
                }
            },
        }
    }
}
struct Target {
    ident: Type,
    generics: TokenStream,
    owned: bool,
    reference: bool,
    mutable: bool,
    implementor: Implementor,
}
impl Target {
    fn implement(self) -> TokenStream {
        let Self {
            ident,
            generics,
            owned,
            reference,
            mutable,
            implementor,
        } = self;

        let owned_impl = owned
            .then(|| implementor.basic_implement(&ident, &generics))
            .unwrap_or_default();

        let ref_impl = reference
            .then(|| {
                let ref_ident = Type::Reference(parse_quote! {&#ident});
                implementor.basic_implement(&ref_ident, &generics)
            })
            .unwrap_or_default();

        let mut_impl = mutable
            .then(|| {
                let mut_ident = Type::Reference(parse_quote! {&mut #ident});
                implementor.basic_implement(&mut_ident, &generics)
            })
            .unwrap_or_default();

        quote! {
            #owned_impl
            #ref_impl
            #mut_impl
        }
    }
    fn targets() -> impl Iterator<Item = Self> {
        let primitives = Self::primitives().into_iter().map(Self::extension);
        let reference = Self::reference().into_iter().map(Self::extension_ref_only);
        let wrap = Self::wrap().into_iter().map(Self::extension_with_generics);
        primitives.chain(reference).chain(wrap)
    }
    fn extension(ty: Type) -> Self {
        Self {
            ident: ty,
            generics: TokenStream::new(),
            owned: true,
            reference: true,
            mutable: true,
            implementor: Implementor::Extension,
        }
    }
    fn extension_ref_only(ty: Type) -> Self {
        Self {
            ident: ty,
            generics: TokenStream::new(),
            owned: false,
            reference: true,
            mutable: false,
            implementor: Implementor::Extension,
        }
    }
    fn extension_with_generics(ty: Type) -> Self {
        Self {
            ident: ty,
            generics: parse_quote! {<T>},
            owned: true,
            reference: true,
            mutable: true,
            implementor: Implementor::Extension,
        }
    }
    fn primitives() -> Vec<Type> {
        vec![
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
        ]
    }
    fn reference() -> Vec<Type> {
        vec![parse_quote! {str}, parse_quote! {std::path::Path}]
    }
    fn wrap() -> Vec<Type> {
        vec![
            parse_quote! {std::marker::PhantomData},
            parse_quote! {Box},
            parse_quote! {std::rc::Rc},
            parse_quote! {std::rc::Weak},
            parse_quote! {std::sync::Arc},
            parse_quote! {std::sync::Weak},
            parse_quote! {std::sync::Mutex},
            parse_quote! {std::sync::RwLock},
            parse_quote! {std::cell::Cell},
            parse_quote! {std::cell::RefCell},
            parse_quote! {std::num::Saturating},
            parse_quote! {std::num::Wrapping},
            parse_quote! {std::cmp::Reverse},
        ]
    }
}
