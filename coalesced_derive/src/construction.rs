use std::fmt::{Display, Formatter};

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, Attribute, Data, DataStruct, DeriveInput, Field, Fields, FieldsUnnamed, Generics,
    Ident, ItemImpl, ItemTrait, Visibility,
};

use crate::error::ConstructionError;

pub struct Construction<'a> {
    ident: &'a Ident,
    generics: &'a Generics,
    field: &'a Field,

    construction: ConstructionAttr,
    semigroup_trait: TraitAttr<'a>,
}
impl ToTokens for Construction<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            semigroup_trait, ..
        } = self;
        let into_inner = self.impl_into_inner();
        tokens.extend(quote::quote! {
            #semigroup_trait
            #into_inner
        });
    }
}
impl<'a> Construction<'a> {
    pub fn new(derive: &'a DeriveInput) -> syn::Result<Self> {
        let DeriveInput {
            attrs,
            ident,
            generics,
            data,
            ..
        } = derive;
        match &data {
            Data::Struct(DataStruct {
                fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }),
                ..
            }) if unnamed.len() == 1 => {
                // if let [field] = unnamed => // match: `if let` guards are experimental see issue #51114 https://github.com/rust-lang/rust/issues/51114
                let &[field] = unnamed.iter().collect::<Vec<_>>().as_slice() else {
                    unreachable!()
                };
                Ok(Self {
                    ident,
                    generics,
                    field,
                    construction: ConstructionAttr::new(attrs, ident)?,
                    semigroup_trait: TraitAttr::new(&derive.vis, attrs, ident, generics)?,
                })
            }
            Data::Enum(_) | Data::Struct(_) | Data::Union(_) => Err(syn::Error::new_spanned(
                ident,
                ConstructionError::OnlyNewType,
            )),
        }
    }

    pub fn impl_into_inner(&self) -> ItemImpl {
        let Self {
            ident,
            generics,
            field,
            ..
        } = self;
        let field_type = &field.ty;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        parse_quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                pub fn into_inner(self) -> #field_type {
                    self.0
                }
            }
        }
    }
}
pub enum ConstructionAttr {
    Annotated,
    Semigroup,
}
impl Display for ConstructionAttr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
impl AsRef<str> for ConstructionAttr {
    fn as_ref(&self) -> &str {
        match self {
            Self::Annotated => "annotated",
            Self::Semigroup => "semigroup",
        }
    }
}
impl ConstructionAttr {
    pub fn new(attrs: &[Attribute], ident: &Ident) -> syn::Result<Self> {
        let (annotated, semigroup) =
            attrs
                .iter()
                .fold((false, false), |(annotated, semigroup), attr| {
                    if attr.path().is_ident(&Self::Annotated) {
                        (true, semigroup)
                    } else if attr.path().is_ident(&Self::Semigroup) {
                        (annotated, true)
                    } else {
                        (annotated, semigroup)
                    }
                });
        match (annotated, semigroup) {
            (true, false) => Ok(Self::Annotated),
            (false, true) => Ok(Self::Semigroup),
            (false, false) => Err(syn::Error::new_spanned(
                ident,
                ConstructionError::ConstructionTypeNotFound,
            )),
            (true, true) => Err(syn::Error::new_spanned(
                ident,
                ConstructionError::DuplicateConstructionType,
            )),
        }
    }
}

pub struct TraitAttr<'a> {
    pub vis: &'a Visibility,
    pub newtype_ident: &'a Ident,
    pub trait_ident: Ident,
    pub method_ident: Ident,
    pub generics: &'a Generics,
}
impl ToTokens for TraitAttr<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let def_trait = self.def_trait();
        let impl_trait = self.impl_trait();
        let impl_trait_reversed = self.impl_trait_reversed();
        let impl_trait_annotated = self.impl_trait_annotated();
        let impl_trait_reversed_annotated = self.impl_trait_reversed_annotated();

        tokens.extend(quote::quote! {
            #def_trait
            #impl_trait
            #impl_trait_reversed
            #impl_trait_annotated
            #impl_trait_reversed_annotated
        });
    }
}
impl<'a> TraitAttr<'a> {
    pub fn new(
        vis: &'a Visibility,
        attrs: &[Attribute],
        ident: &'a Ident,
        generics: &'a Generics,
    ) -> syn::Result<Self> {
        let newtype_ident = ident;
        let trait_ident = attrs
            .iter()
            .find_map(|attr| {
                if attr.path().is_ident("trait") {
                    Some(attr.parse_args::<Ident>().unwrap())
                } else {
                    None
                }
            })
            .ok_or_else(|| syn::Error::new_spanned(ident, ConstructionError::TraitNotFound))?;
        let method_ident = quote::format_ident!("{}", trait_ident.to_string().to_lowercase());

        Ok(Self {
            vis,
            newtype_ident,
            trait_ident,
            method_ident,
            generics,
        })
    }

    pub fn def_trait(&self) -> ItemTrait {
        let Self {
            vis,
            trait_ident,
            method_ident,
            ..
        } = self;
        parse_quote! {
            #vis trait #trait_ident: Sized + Semigroup {
                fn #method_ident(self, other: Self) -> Self {
                    Semigroup::semigroup_op(self, other)
                }
            }
        }
    }
    pub fn impl_trait(&self) -> ItemImpl {
        let Self {
            newtype_ident,
            trait_ident,
            generics,
            ..
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        parse_quote! {
            impl #impl_generics #trait_ident for #newtype_ident #ty_generics #where_clause {}
        }
    }
    pub fn impl_trait_reversed(&self) -> ItemImpl {
        let Self {
            newtype_ident,
            trait_ident,
            generics,
            ..
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        parse_quote! {
            impl #impl_generics #trait_ident for Reversed<#newtype_ident #ty_generics> #where_clause {}
        }
    }
    pub fn impl_trait_annotated(&self) -> ItemImpl {
        let Self {
            newtype_ident,
            trait_ident,
            generics,
            ..
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        parse_quote! {
            impl<T, A> #trait_ident for Annotated<#newtype_ident #ty_generics, A> #where_clause {}
        }
    }
    pub fn impl_trait_reversed_annotated(&self) -> ItemImpl {
        let Self {
            newtype_ident,
            trait_ident,
            generics,
            ..
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        parse_quote! {
            impl<T, A> #trait_ident for Reversed<Annotated<#newtype_ident #ty_generics, A>> #where_clause {}
        }
    }
}
