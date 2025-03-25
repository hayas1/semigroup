use std::{fs, path::Path, process::Command};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Type};

const GENERATED_RS: &str = "src/generated.rs";
const RUSTFMT: &str = "rustfmt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let generated_impl = ExtensionImplementor::implement(&ExtensionTargets::default());
    let generated_rs = Path::new(GENERATED_RS);
    fs::write(&generated_rs, generated_impl.to_string())?;
    Command::new(RUSTFMT).arg(&generated_rs).status()?;
    Ok(())
}

enum ExtensionImplementor {}
impl ExtensionImplementor {
    fn implement(targets: &ExtensionTargets) -> TokenStream {
        let impl_primitive = targets.primitive_owned().map(Self::basic_priority);
        let impl_primitive_ref = targets.primitive_ref().map(Self::basic_priority);
        let impl_primitive_mut = targets.primitive_mut().map(Self::basic_priority);
        let impl_slice_ref = targets.slice_ref().map(Self::basic_priority);
        let impl_wrap_owned = targets.wrap_owned().map(Self::basic_generics_priority);
        let impl_wrap_ref = targets.wrap_ref().map(Self::basic_generics_priority);
        let impl_wrap_mut = targets.wrap_mut().map(Self::basic_generics_priority);
        quote! {
            use crate::extension::{Extension, WithExt};
            #(#impl_primitive)*
            #(#impl_primitive_ref)*
            #(#impl_primitive_mut)*
            #(#impl_slice_ref)*
            #(#impl_wrap_owned)*
            #(#impl_wrap_ref)*
            #(#impl_wrap_mut)*
        }
    }
    fn basic_priority<T: ToTokens>(ident: T) -> TokenStream {
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
    fn basic_generics_priority<T: ToTokens>(ident: T) -> TokenStream {
        quote! {
            #[doc = "Generated implementation"]
            impl<T> Extension for #ident<T> {
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

struct ExtensionTargets {
    primitives: Vec<Type>,
    reference: Vec<Type>,
    wrap: Vec<Type>,
}
impl Default for ExtensionTargets {
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
            reference: vec![parse_quote! {str}, parse_quote! {std::path::Path}],
            wrap: vec![
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
            ],
        }
    }
}
impl ExtensionTargets {
    fn primitive_owned<'a>(&'a self) -> impl 'a + Iterator<Item = &Type> {
        self.primitives.iter()
    }
    fn primitive_ref<'a>(&'a self) -> impl 'a + Iterator<Item = Type> {
        self.primitives
            .iter()
            .map(|t| Type::Reference(parse_quote! {&#t}))
    }
    fn primitive_mut<'a>(&'a self) -> impl 'a + Iterator<Item = Type> {
        self.primitives
            .iter()
            .map(|t| Type::Reference(parse_quote! {&mut #t}))
    }
    fn slice_ref<'a>(&'a self) -> impl 'a + Iterator<Item = Type> {
        self.reference
            .iter()
            .map(|t| Type::Reference(parse_quote! {&#t}))
    }
    fn wrap_owned<'a>(&'a self) -> impl 'a + Iterator<Item = &Type> {
        self.wrap.iter()
    }
    fn wrap_ref<'a>(&'a self) -> impl 'a + Iterator<Item = Type> {
        self.wrap
            .iter()
            .map(|t| Type::Reference(parse_quote! {&#t}))
    }
    fn wrap_mut<'a>(&'a self) -> impl 'a + Iterator<Item = Type> {
        self.wrap
            .iter()
            .map(|t| Type::Reference(parse_quote! {&mut #t}))
    }
}
