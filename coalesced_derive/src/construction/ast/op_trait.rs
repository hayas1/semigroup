use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, DeriveInput, Field, Ident, ItemImpl, ItemTrait};

use crate::{annotation::Annotation, constant::Constant, construction::attr::ContainerAttr};

#[derive(Debug, Clone)]
pub struct OpTrait<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,

    trait_ident: &'a Ident,
    method_ident: Ident,

    attr: &'a ContainerAttr,
    annotation: Annotation,
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
        attr: &'a ContainerAttr,
        _field: &'a Field,
    ) -> syn::Result<Self> {
        let trait_ident = &attr.op;
        let method_ident = quote::format_ident!("{}", attr.op.to_string().to_snake_case());
        let annotation = attr.annotation();

        Ok(Self {
            constant,
            derive,
            trait_ident,
            method_ident,
            attr,
            annotation,
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
            annotation,
            ..
        } = self;

        attr.is_annotated().then(|| {
            let (_, ty_generics, _) = generics.split_for_impl();
            let (impl_generics, annotated_type, where_clause) =
                annotation.split_for_impl(generics);

            parse_quote! {
                impl #impl_generics #trait_ident for #path_annotated<#ident #ty_generics, #annotated_type> #where_clause {}
            }
        })
    }
    pub fn impl_trait_reversed_annotated(&self) -> Option<ItemImpl> {
        let Self {
            derive: DeriveInput {
                ident, generics, ..
            },
            constant:
                Constant {
                    path_annotated,
                    path_reversed,
                    ..
                },
            trait_ident,
            attr,
            annotation,
            ..
        } = self;

        attr.is_annotated().then(|| {
            let (_, ty_generics, _) = generics.split_for_impl();
            let (impl_generics, annotated_type, where_clause) = annotation.split_for_impl(generics);
            parse_quote! {
                impl #impl_generics #trait_ident for #path_reversed<#path_annotated<#ident #ty_generics, #annotated_type>> #where_clause {}
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
            annotation,
            ..
        } = self;

        (attr.is_annotated() && !attr.without_annotate_impl).then(|| {
            let (_, ty_generics, _) = generics.split_for_impl();
            let (impl_generics, annotated_type, where_clause) = annotation.split_for_impl(generics);
            parse_quote! {
                impl #impl_generics #path_annotate<#annotated_type> for #ident #ty_generics #where_clause {
                    type Annotation = #annotated_type;
                    fn annotated(self, annotation: Self::Annotation) -> #path_annotated<Self, #annotated_type> {
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
