use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsUnnamed, Ident,
    ItemFn, ItemImpl, ItemTrait,
};

use crate::{
    constant::Constant,
    construction::{attr::ContainerAttr, generics::Annotated},
    error::ConstructionError,
};

#[derive(Debug, Clone)]
pub struct Construction<'a> {
    derive: &'a DeriveInput,
    field: &'a Field,

    semigroup_trait: ConstructionTrait<'a>,
}
impl ToTokens for Construction<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            semigroup_trait, ..
        } = self;
        let from = self.impl_from();
        let deref = self.impl_deref();
        let deref_mut = self.impl_deref_mut();
        let impl_block = self.impl_block();
        tokens.extend(quote::quote! {
            #semigroup_trait
            #from
            #deref
            #deref_mut
            #impl_block
        });
    }
}
impl<'a> Construction<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
    ) -> syn::Result<Self> {
        match &derive.data {
            Data::Struct(DataStruct {
                fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }),
                ..
            }) if unnamed.len() == 1 => {
                // if let [field] = unnamed => // match: `if let` guards are experimental see issue #51114 https://github.com/rust-lang/rust/issues/51114
                let &[field] = unnamed.iter().collect::<Vec<_>>().as_slice() else {
                    unreachable!()
                };
                let semigroup_trait = ConstructionTrait::new(constant, derive, attr)?;
                Ok(Self {
                    derive,
                    field,
                    semigroup_trait,
                })
            }
            Data::Enum(DataEnum { enum_token, .. }) => Err(syn::Error::new_spanned(
                enum_token,
                ConstructionError::OnlyNewType,
            )),
            Data::Struct(DataStruct { struct_token, .. }) => Err(syn::Error::new_spanned(
                struct_token,
                ConstructionError::OnlyNewType,
            )),
            Data::Union(union) => Err(syn::Error::new_spanned(
                union.union_token,
                ConstructionError::OnlyNewType,
            )),
        }
    }

    pub fn impl_from(&self) -> ItemImpl {
        let Self {
            derive: DeriveInput {
                ident, generics, ..
            },
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
    pub fn impl_block(&self) -> ItemImpl {
        let Self {
            derive: DeriveInput {
                ident, generics, ..
            },
            ..
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let into_inner = self.impl_into_inner();
        parse_quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                #into_inner
            }
        }
    }
    pub fn impl_into_inner(&self) -> ItemFn {
        let Self {
            field: Field { ty, .. },
            ..
        } = self;
        parse_quote! {
            pub fn into_inner(self) -> #ty {
                self.0
            }
        }
    }
    pub fn impl_deref(&self) -> ItemImpl {
        let Self {
            derive: DeriveInput {
                ident, generics, ..
            },
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
            derive: DeriveInput {
                ident, generics, ..
            },
            ..
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
    constant: &'a Constant,
    derive: &'a DeriveInput,

    trait_ident: Ident,
    method_ident: Ident,

    attr: &'a ContainerAttr,
}
impl ToTokens for ConstructionTrait<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let def_trait = self.def_trait();
        let impl_trait = self.impl_trait();
        let impl_trait_reversed = self.impl_trait_reversed();
        let impl_trait_annotated = self.impl_trait_annotated();
        let impl_trait_reversed_annotated = self.impl_trait_reversed_annotated();
        let impl_semigroup_with_unit_annotate = self.impl_semigroup_with_unit_annotate();

        tokens.extend(quote::quote! {
            #def_trait
            #impl_trait
            #impl_trait_reversed
            #impl_trait_annotated
            #impl_trait_reversed_annotated
            #impl_semigroup_with_unit_annotate
        });
    }
}
impl<'a> ConstructionTrait<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
    ) -> syn::Result<Self> {
        let trait_ident = attr.op.clone();
        let method_ident = quote::format_ident!("{}", trait_ident.to_string().to_snake_case());

        Ok(Self {
            constant,
            derive,
            trait_ident,
            method_ident,
            attr,
        })
    }

    pub fn def_trait(&self) -> ItemTrait {
        let Self {
            constant:
                Constant {
                    path_semigroup,
                    ident_semigroup_op,
                    ..
                },
            derive: DeriveInput { vis, .. },
            trait_ident,
            method_ident,
            ..
        } = self;

        parse_quote! {
            #vis trait #trait_ident: Sized + Semigroup {
                fn #method_ident(self, other: Self) -> Self {
                    #path_semigroup::#ident_semigroup_op(self, other)
                }
            }
        }
    }
    pub fn impl_trait(&self) -> ItemImpl {
        let Self {
            derive: DeriveInput {
                ident, generics, ..
            },
            trait_ident,
            ..
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        parse_quote! {
            impl #impl_generics #trait_ident for #ident #ty_generics #where_clause {}
        }
    }
    pub fn impl_trait_reversed(&self) -> ItemImpl {
        let Self {
            constant: Constant { path_reversed, .. },
            derive: DeriveInput {
                ident, generics, ..
            },
            trait_ident,
            ..
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        parse_quote! {
            impl #impl_generics #trait_ident for #path_reversed<#ident #ty_generics> #where_clause {}
        }
    }
    pub fn impl_trait_annotated(&self) -> Option<ItemImpl> {
        let Self {
            constant: Constant { path_annotated, .. },
            derive: DeriveInput {
                ident, generics, ..
            },
            trait_ident,
            attr,
            ..
        } = self;
        attr.is_annotated().then(|| {
            let annotated = Annotated::new(path_annotated, ident, generics, attr);
            let (annotated_impl_generics, annotated_ty, where_clause) = annotated.split_for_impl();

            parse_quote! {
                impl #annotated_impl_generics #trait_ident for #annotated_ty #where_clause {}
            }
        })
    }
    pub fn impl_trait_reversed_annotated(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_annotated,
                    path_reversed,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            trait_ident,
            attr,
            ..
        } = self;
        attr.is_annotated().then(|| {
            let annotated = Annotated::new(path_annotated, ident, generics, attr);
            let (annotated_impl_generics, annotated_ty, where_clause) = annotated.split_for_impl();
            parse_quote! {
                impl #annotated_impl_generics #trait_ident for #path_reversed<#annotated_ty> #where_clause {}
            }
        })
    }
    pub fn impl_semigroup_with_unit_annotate(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_semigroup,
                    ident_semigroup_op,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            ..
        } = self;
        attr.is_annotated().then(|| {
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
            let unit = attr.unit_annotate();
            parse_quote! {
                impl #impl_generics #path_semigroup for #ident #ty_generics #where_clause {
                    fn #ident_semigroup_op(base: Self, other: Self) -> Self {
                        Self::default_semigroup_op(base, other, #unit, #unit)
                    }
                }
            }
        })
    }
}
