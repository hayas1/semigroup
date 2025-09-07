use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, DeriveInput, Field, Ident, ItemImpl, ItemTrait};

use crate::{annotated::Annotated, constant::Constant, construction::attr::ContainerAttr};

#[derive(Debug, Clone)]
pub struct OpTrait<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,

    trait_ident: &'a Ident,
    method_ident: Ident,

    attr: &'a ContainerAttr,
    annotated: Annotated<'a>,
}
impl ToTokens for OpTrait<'_> {
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
impl<'a> OpTrait<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        _field: &'a Field,
        attr: &'a ContainerAttr,
    ) -> syn::Result<Self> {
        let trait_ident = &attr.op;
        let method_ident = quote::format_ident!("{}", attr.op.to_string().to_snake_case());
        let annotated = Annotated::new(
            &constant.path_annotated,
            &derive.ident,
            &derive.generics,
            attr.annotation_type_param(),
            None,
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
            let (a, annotated_self) = (&self.annotated.annotation_param().ident, annotated_ty.ty_self());
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
