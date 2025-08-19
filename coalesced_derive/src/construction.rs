use darling::{ast::NestedMeta, FromMeta};
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, Attribute, Data, DataStruct, DeriveInput, Field, Fields, FieldsUnnamed, Generics,
    Ident, ItemImpl, ItemTrait, Meta, MetaList, Visibility,
};

use crate::{
    constant::{CONSTRUCTION, IDENT_SEMIGROUP_OP, PATH_ANNOTATED, PATH_REVERSED, PATH_SEMIGROUP},
    error::ConstructionError,
};

#[derive(Debug, Clone)]
pub struct Construction<'a> {
    ident: &'a Ident,
    generics: &'a Generics,
    field: &'a Field,

    attr: ConstructionAttr,
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
                let attr = ConstructionAttr::new(attrs, ident)?;
                let semigroup_trait = TraitAttr::new(&derive.vis, &attr, ident, generics)?;
                Ok(Self {
                    ident,
                    generics,
                    field,
                    attr,
                    semigroup_trait,
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

#[derive(Debug, Clone, FromMeta)]
pub struct ConstructionAttr {
    #[darling(default)]
    pub annotated: bool,
    #[darling(default)]
    pub semigroup: bool,
    pub op: Ident,
}
impl ConstructionAttr {
    pub fn new(attrs: &[Attribute], ident: &Ident) -> syn::Result<Self> {
        let attr = attrs
            .iter()
            .find_map(|Attribute { meta, .. }| match meta {
                Meta::List(MetaList { path, tokens, .. }) if path.is_ident(CONSTRUCTION) => {
                    Some(tokens)
                }
                _ => None,
            })
            .ok_or(syn::Error::new_spanned(
                ident,
                ConstructionError::NoConstructionAttr,
            ))?;

        let this = Self::from_list(&NestedMeta::parse_meta_list(attr.clone())?)?;
        this.validate()
            .map_err(|e| syn::Error::new_spanned(ident, e))?;
        Ok(this)
    }
    pub fn validate(&self) -> Result<&Self, ConstructionError> {
        if self.annotated && self.semigroup {
            Err(ConstructionError::DuplicateConstructionType)
        } else if !self.annotated && !self.semigroup {
            Err(ConstructionError::ConstructionTypeNotFound)
        } else {
            Ok(self)
        }
    }
}

#[derive(Debug, Clone)]
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
        attr: &ConstructionAttr,
        ident: &'a Ident,
        generics: &'a Generics,
    ) -> syn::Result<Self> {
        let newtype_ident = ident;
        let trait_ident = attr.op.clone();
        let method_ident = quote::format_ident!("{}", trait_ident.to_string().to_snake_case());

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
        PATH_SEMIGROUP.with(|ps| {
            IDENT_SEMIGROUP_OP.with(|iso| {
                let (semigroup_trait, semigroup_op) = (&**ps, &**iso);
                parse_quote! {
                    #vis trait #trait_ident: Sized + Semigroup {
                        fn #method_ident(self, other: Self) -> Self {
                            #semigroup_trait::#semigroup_op(self, other)
                        }
                    }
                }
            })
        })
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
        PATH_REVERSED.with(|pr| {
            let reversed = &**pr;
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
            parse_quote! {
                impl #impl_generics #trait_ident for #reversed<#newtype_ident #ty_generics> #where_clause {}
            }
        })
    }
    pub fn impl_trait_annotated(&self) -> ItemImpl {
        let Self {
            newtype_ident,
            trait_ident,
            generics,
            ..
        } = self;
        PATH_ANNOTATED.with(|pa| {
            let annotated = &**pa;
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
            parse_quote! {
                impl<T, A> #trait_ident for #annotated<#newtype_ident #ty_generics, A> #where_clause {}
            }
        })
    }
    pub fn impl_trait_reversed_annotated(&self) -> ItemImpl {
        let Self {
            newtype_ident,
            trait_ident,
            generics,
            ..
        } = self;
        PATH_ANNOTATED.with(|pa| {
            PATH_REVERSED.with(|pr| {
                let (annotated, reversed) = (&**pa, &**pr);
                let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
                parse_quote! {
                    impl<T, A> #trait_ident for #reversed<#annotated<#newtype_ident #ty_generics, A>> #where_clause {}
                }
            })
        })
    }
}
