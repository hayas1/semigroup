use std::{fs, path::Path, process::Command};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Generics, Type};

const GENERATED_RS: &str = "src/generated.rs";
const RUSTFMT: &str = "rustfmt";

// TODO specialization https://rust-lang.github.io/rfcs/1210-impl-specialization.html
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let targets = Target::targets().into_iter().map(Target::implement);
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

    fn basic_implement(&self, ident: impl ToTokens, generics: &Option<Generics>) -> TokenStream {
        let impl_generics = generics
            .as_ref()
            .map(|g| g.params.clone())
            .unwrap_or_default()
            .into_iter();
        match self {
            Self::Extension => quote! {
                #[doc = "Generated implementation"]
                impl <X, #(#impl_generics),*> Extension<X> for #ident #generics {
                    type WithExt = WithExt<Self, X>;
                    fn with_extension(self, extension: X) -> Self::WithExt {
                        WithExt {
                            value: self,
                            extension,
                        }
                    }
                    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
                        with_ext.value
                    }
                    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
                        other
                    }
                    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
                        base
                    }
                }
            },
        }
    }
}
struct Target {
    ident: Type,
    generics: Option<Generics>,
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

    fn new_extension(
        ident: Type,
        generics: Option<Generics>,
        (owned, reference, mutable): (bool, bool, bool),
    ) -> Self {
        let implementor = Implementor::Extension;
        Self {
            ident,
            generics,
            owned,
            reference,
            mutable,
            implementor,
        }
    }

    fn targets() -> Vec<Self> {
        vec![
            Self::new_extension(parse_quote! {()}, None, (true, true, true)),
            Self::new_extension(parse_quote! {bool}, None, (true, true, true)),
            Self::new_extension(parse_quote! {char}, None, (true, true, true)),
            Self::new_extension(parse_quote! {String}, None, (true, true, true)),
            Self::new_extension(parse_quote! {u8}, None, (true, true, true)),
            Self::new_extension(parse_quote! {u16}, None, (true, true, true)),
            Self::new_extension(parse_quote! {u32}, None, (true, true, true)),
            Self::new_extension(parse_quote! {u64}, None, (true, true, true)),
            Self::new_extension(parse_quote! {u128}, None, (true, true, true)),
            Self::new_extension(parse_quote! {i8}, None, (true, true, true)),
            Self::new_extension(parse_quote! {i16}, None, (true, true, true)),
            Self::new_extension(parse_quote! {i32}, None, (true, true, true)),
            Self::new_extension(parse_quote! {i64}, None, (true, true, true)),
            Self::new_extension(parse_quote! {i128}, None, (true, true, true)),
            Self::new_extension(parse_quote! {f32}, None, (true, true, true)),
            Self::new_extension(parse_quote! {f64}, None, (true, true, true)),
            Self::new_extension(parse_quote! {str}, None, (false, true, false)),
            Self::new_extension(parse_quote! {std::path::Path}, None, (false, true, false)),
            Self::new_extension(parse_quote! {std::path::PathBuf}, None, (true, true, true)),
            Self::new_extension(parse_quote! {std::time::Duration}, None, (true, true, true)),
            Self::new_extension(parse_quote! {std::time::Instant}, None, (true, true, true)),
            Self::new_extension(
                parse_quote! {std::time::SystemTime},
                None,
                (true, true, true),
            ),
            Self::new_extension(parse_quote! {std::net::IpAddr}, None, (true, true, true)),
            Self::new_extension(parse_quote! {std::net::Ipv4Addr}, None, (true, true, true)),
            Self::new_extension(parse_quote! {std::net::Ipv6Addr}, None, (true, true, true)),
            Self::new_extension(
                parse_quote! {std::net::SocketAddr},
                None,
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::net::SocketAddrV4},
                None,
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::net::SocketAddrV6},
                None,
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::atomic::AtomicBool},
                None,
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::atomic::AtomicIsize},
                None,
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::atomic::AtomicI8},
                None,
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::atomic::AtomicI16},
                None,
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::atomic::AtomicI32},
                None,
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::atomic::AtomicI64},
                None,
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::atomic::AtomicUsize},
                None,
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::atomic::AtomicU8},
                None,
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::atomic::AtomicU16},
                None,
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::atomic::AtomicU32},
                None,
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::atomic::AtomicU64},
                None,
                (true, true, true),
            ),
            Self::new_extension(parse_quote! {std::ffi::OsStr}, None, (false, true, false)),
            Self::new_extension(parse_quote! {std::ffi::OsString}, None, (true, true, true)),
            Self::new_extension(parse_quote! {std::ffi::CStr}, None, (false, true, false)),
            Self::new_extension(parse_quote! {std::ffi::CString}, None, (true, true, true)),
            Self::new_extension(
                parse_quote! {std::marker::PhantomData},
                Some(parse_quote! {<T>}),
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {Box},
                Some(parse_quote! {<T>}),
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::rc::Rc},
                Some(parse_quote! {<T>}),
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::rc::Weak},
                Some(parse_quote! {<T>}),
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::Arc},
                Some(parse_quote! {<T>}),
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::Weak},
                Some(parse_quote! {<T>}),
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::Mutex},
                Some(parse_quote! {<T>}),
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::sync::RwLock},
                Some(parse_quote! {<T>}),
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::cell::Cell},
                Some(parse_quote! {<T>}),
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::cell::RefCell},
                Some(parse_quote! {<T>}),
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::num::Saturating},
                Some(parse_quote! {<T>}),
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::num::Wrapping},
                Some(parse_quote! {<T>}),
                (true, true, true),
            ),
            Self::new_extension(
                parse_quote! {std::cmp::Reverse},
                Some(parse_quote! {<T>}),
                (true, true, true),
            ),
        ]
    }
}
