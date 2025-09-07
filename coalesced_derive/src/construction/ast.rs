use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsUnnamed, Ident,
    ItemImpl, ItemTrait,
};

use crate::{
    annotated::Annotated, constant::Constant, construction::attr::ContainerAttr,
    error::ConstructionError,
};

#[derive(Debug, Clone)]
pub struct Construction<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,
    field: &'a Field,

    attr: &'a ContainerAttr,
    annotated: Annotated<'a>,
    semigroup_trait: ConstructionTrait<'a>,
}
impl ToTokens for Construction<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.semigroup_trait.to_tokens(tokens);
        self.impl_from().to_tokens(tokens);
        self.impl_deref().to_tokens(tokens);
        self.impl_deref_mut().to_tokens(tokens);
        self.impl_construction().to_tokens(tokens);
        self.impl_construction_annotated()
            .into_iter()
            .for_each(|i| i.to_tokens(tokens));
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
                let annotated = Annotated::new(
                    &constant.path_annotated,
                    &derive.ident,
                    &derive.generics,
                    attr.annotation_type_param(),
                    attr.annotation_where(),
                );
                let semigroup_trait = ConstructionTrait::new(constant, derive, attr)?;
                Ok(Self {
                    constant,
                    derive,
                    field,
                    attr,
                    annotated,
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
    pub fn impl_construction(&self) -> ItemImpl {
        let Self {
            constant:
                Constant {
                    path_construction_trait,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            field: Field { ty, .. },
            ..
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        parse_quote! {
            impl #impl_generics #path_construction_trait<#ty> for #ident #ty_generics #where_clause {
                fn new(value: #ty) -> Self {
                    Self(value)
                }
                fn into_inner(self) -> #ty {
                    self.0
                }
            }
        }
    }
    pub fn impl_construction_annotated(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_construction_annotated,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            field: Field { ty, .. },
            attr,
            ..
        } = self;
        attr.is_annotated().then(|| {
            let (_, ty_generics, _) = generics.split_for_impl();
            let (annotated_impl_generics, _, where_clause) = self.annotated.split_for_impl();
            let a = attr.annotation_type_param().ident; // TODO split method
            parse_quote! {
                impl #annotated_impl_generics #path_construction_annotated<#ty, #a> for #ident #ty_generics #where_clause {}
            }
        })
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
    annotated: Annotated<'a>,
}
impl ToTokens for ConstructionTrait<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.def_trait().to_tokens(tokens);
        self.impl_trait().to_tokens(tokens);
        self.impl_trait_reversed().to_tokens(tokens);
        self.impl_trait_annotated()
            .into_iter()
            .for_each(|i| i.to_tokens(tokens));
        self.impl_trait_reversed_annotated()
            .into_iter()
            .for_each(|i| i.to_tokens(tokens));
        self.impl_semigroup_with_unit_annotate()
            .into_iter()
            .for_each(|i| i.to_tokens(tokens));
        self.impl_annotate()
            .into_iter()
            .for_each(|i| i.to_tokens(tokens));
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
        let annotated = Annotated::new(
            &constant.path_annotated,
            &derive.ident,
            &derive.generics,
            attr.annotation_type_param(),
            attr.annotation_where(),
        );

        Ok(Self {
            constant,
            derive,
            trait_ident,
            method_ident,
            attr,
            annotated,
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
            trait_ident, attr, ..
        } = self;
        attr.is_annotated().then(|| {
            let (annotated_impl_generics, annotated_ty, where_clause) =
                self.annotated.split_for_impl();

            parse_quote! {
                impl #annotated_impl_generics #trait_ident for #annotated_ty #where_clause {}
            }
        })
    }
    pub fn impl_trait_reversed_annotated(&self) -> Option<ItemImpl> {
        let Self {
            constant: Constant { path_reversed, .. },
            trait_ident,
            attr,
            ..
        } = self;
        attr.is_annotated().then(|| {
            let (annotated_impl_generics, annotated_ty, where_clause) = self.annotated.split_for_impl();
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

    pub fn impl_annotate(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_annotate,
                    path_annotated,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            ..
        } = self;
        (attr.is_annotated() && !attr.without_annotate_impl).then(|| {
            let (_, ty_generics, _) = generics.split_for_impl();
            let (annotated_impl_generics, annotated_ty, where_clause) = self.annotated.split_for_impl();
            let annotated_self = annotated_ty.ty_self();
            let a = self.annotated.annotation().ident;
            parse_quote! {
                impl #annotated_impl_generics #path_annotate<#a> for #ident #ty_generics #where_clause {
                    type Annotation = #a;
                    fn annotated(self, annotation: Self::Annotation) -> #annotated_self {
                        #path_annotated {
                            value: self,
                            annotation
                        }
                    }
                }
            }
        })
    }
}
