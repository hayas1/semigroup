use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, Data, DataStruct, DeriveInput, Field, Fields, FieldsUnnamed, Generics, Ident,
    ItemImpl, ItemTrait, Visibility,
};

use crate::{
    constant::{IDENT_SEMIGROUP_OP, PATH_ANNOTATED, PATH_REVERSED, PATH_SEMIGROUP},
    construction::attr::ConstructionAttr,
    error::ConstructionError,
};

#[derive(Debug, Clone)]
pub struct Construction<'a> {
    ident: &'a Ident,
    generics: &'a Generics,
    field: &'a Field,

    semigroup_trait: ConstructionTrait<'a>,
}
impl ToTokens for Construction<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            semigroup_trait, ..
        } = self;
        let from = self.impl_from();
        let into_inner = self.impl_into_inner();
        let deref = self.impl_deref();
        let deref_mut = self.impl_deref_mut();
        tokens.extend(quote::quote! {
            #semigroup_trait
            #from
            #into_inner
            #deref
            #deref_mut
        });
    }
}
impl<'a> Construction<'a> {
    pub fn new(derive: &'a DeriveInput, attr: &'a ConstructionAttr) -> syn::Result<Self> {
        let DeriveInput {
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
                let semigroup_trait = ConstructionTrait::new(&derive.vis, attr, ident, generics)?;
                Ok(Self {
                    ident,
                    generics,
                    field,
                    semigroup_trait,
                })
            }
            Data::Enum(_) | Data::Struct(_) | Data::Union(_) => Err(syn::Error::new_spanned(
                ident,
                ConstructionError::OnlyNewType,
            )),
        }
    }

    pub fn impl_from(&self) -> ItemImpl {
        let Self {
            ident,
            generics,
            field,
            ..
        } = self;
        let field_type = &field.ty;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        parse_quote! {
            impl #impl_generics From<#field_type> for #ident #ty_generics #where_clause {
                fn from(value: #field_type) -> Self {
                    #ident(value)
                }
            }
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
    pub fn impl_deref(&self) -> ItemImpl {
        let Self {
            ident,
            generics,
            field,
            ..
        } = self;
        let field_type = &field.ty;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        parse_quote! {
            impl #impl_generics std::ops::Deref for #ident #ty_generics #where_clause {
                type Target = #field_type;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    }
    pub fn impl_deref_mut(&self) -> ItemImpl {
        let Self {
            ident, generics, ..
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        parse_quote! {
            impl #impl_generics std::ops::DerefMut for #ident #ty_generics #where_clause {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConstructionTrait<'a> {
    pub vis: &'a Visibility,
    pub newtype_ident: &'a Ident,
    pub trait_ident: Ident,
    pub method_ident: Ident,
    pub generics: &'a Generics,

    pub attr: &'a ConstructionAttr,
}
impl ToTokens for ConstructionTrait<'_> {
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
impl<'a> ConstructionTrait<'a> {
    pub fn new(
        vis: &'a Visibility,
        attr: &'a ConstructionAttr,
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
            attr,
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
    pub fn impl_trait_annotated(&self) -> Option<ItemImpl> {
        let Self {
            newtype_ident,
            trait_ident,
            generics,
            attr,
            ..
        } = self;
        attr.annotated.then(|| {
            PATH_ANNOTATED.with(|pa| {
                let annotated = &**pa;
                let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
                parse_quote! {
                    impl<T, A> #trait_ident for #annotated<#newtype_ident #ty_generics, A> #where_clause {}
                }
            })
        })
    }
    pub fn impl_trait_reversed_annotated(&self) -> Option<ItemImpl> {
        let Self {
            newtype_ident,
            trait_ident,
            generics,
            attr,
            ..
        } = self;
        attr.annotated.then(|| {
            PATH_ANNOTATED.with(|pa| {
                PATH_REVERSED.with(|pr| {
                    let (annotated, reversed) = (&**pa, &**pr);
                    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
                    parse_quote! {
                        impl<T, A> #trait_ident for #reversed<#annotated<#newtype_ident #ty_generics, A>> #where_clause {}
                    }
                })
            })
        })
    }
}
