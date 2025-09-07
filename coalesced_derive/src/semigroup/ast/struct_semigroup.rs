use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{parse_quote, DataStruct, DeriveInput, FieldValue, Fields, Ident, ItemImpl, ItemStruct};

use crate::{
    constant::Constant,
    semigroup::{
        ast::field_semigroup::{FieldAnnotatedOp, FieldSemigroupOp},
        attr::ContainerAttr,
    },
};

#[derive(Debug, Clone)]
pub struct StructSemigroup<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,
    attr: &'a ContainerAttr,
    data_struct: &'a DataStruct,
}
impl ToTokens for StructSemigroup<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_semigroup()
            .as_ref()
            .map(ToTokens::to_token_stream)
            .unwrap_or_else(syn::Error::to_compile_error)
            .to_tokens(tokens);
    }
}
impl<'a> StructSemigroup<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
        data_struct: &'a DataStruct,
    ) -> syn::Result<Self> {
        Ok(Self {
            constant,
            derive,
            data_struct,
            attr,
        })
    }
    pub fn impl_semigroup(&self) -> syn::Result<ItemImpl> {
        let Self {
            constant,
            derive,
            attr,
            data_struct,
            ..
        } = self;
        let Constant {
            path_semigroup,
            ident_semigroup_op,
            ..
        } = constant;
        let DeriveInput {
            ident, generics, ..
        } = derive;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let fields_op = FieldSemigroupOp::new_fields(constant, derive, attr, &data_struct.fields)?
            .into_iter()
            .map(|op| op.impl_field_semigroup_op());
        Ok(parse_quote! {
            impl #impl_generics #path_semigroup for #ident #ty_generics #where_clause {
                fn #ident_semigroup_op(base: Self, other: Self) -> Self {
                    Self {
                        #(#fields_op),*
                    }
                }
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct StructAnnotate<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,
    attr: &'a ContainerAttr,
    data_struct: &'a DataStruct,
}
impl ToTokens for StructAnnotate<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.def_annotation().to_tokens(tokens);
        self.impl_annotated_semigroup()
            .as_ref()
            .map(ToTokens::to_token_stream)
            .unwrap_or_else(syn::Error::to_compile_error)
            .to_tokens(tokens);
        self.impl_annotate().to_tokens(tokens)
    }
}
impl<'a> StructAnnotate<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
        data_struct: &'a DataStruct,
    ) -> Self {
        Self {
            constant,
            derive,
            data_struct,
            attr,
        }
    }

    pub fn annotation_ident(&self) -> Ident {
        format_ident!("{}Annotation", self.derive.ident)
    }

    pub fn def_annotation(&self) -> ItemStruct {
        let Self {
            derive: DeriveInput { vis, .. },
            data_struct,
            ..
        } = self;
        let annotation_ident = self.annotation_ident();
        match &data_struct.fields {
            Fields::Named(fields) => {
                let idents = fields.named.iter().map(|f| &f.ident);
                parse_quote! {
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
                    #vis struct #annotation_ident<A> {
                        #( #idents: A ),*
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let annotation = fields.unnamed.iter().map(|_| format_ident!("A"));
                parse_quote! {
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
                    #vis struct #annotation_ident<A>( #( #annotation ),* );
                }
            }
            Fields::Unit => todo!(),
        }
    }

    pub fn impl_annotated_semigroup(&self) -> syn::Result<ItemImpl> {
        let Self {
            constant,
            derive,
            attr,
            data_struct,
        } = self;
        let Constant {
            path_annotated_semigroup,
            ident_annotated_op,
            path_annotated,
            ..
        } = constant;
        let DeriveInput {
            ident, generics, ..
        } = derive;
        let annotation_ident = self.annotation_ident();
        let (local, value, field_annotation): (Vec<_>, Vec<_>, Vec<_>) =
            FieldAnnotatedOp::new_fields(constant, derive, attr, &data_struct.fields)?
                .into_iter()
                .map(|f| {
                    (
                        f.impl_field_annotated_op(),
                        f.impl_field_value(),
                        f.impl_field_annotation(),
                    )
                })
                .collect();
        let annotation = attr.annotation(&annotation_ident);
        let (_, ty_generics, _) = generics.split_for_impl();
        let (impl_generics, annotation_type, where_clause) = annotation.split_for_impl(generics);
        Ok(parse_quote! {
            impl #impl_generics #path_annotated_semigroup<#annotation_type> for #ident #ty_generics #where_clause {
                fn #ident_annotated_op(base: #path_annotated<Self, #annotation_type>, other: #path_annotated<Self, #annotation_type>) -> #path_annotated<Self, #annotation_type> {
                    #( #local )*
                    #path_annotated {
                        value: #ident {
                            #( #value ),*
                        },
                        annotation: #annotation_ident {
                            #( #field_annotation ),*
                        },
                    }
                }
            }
        })
    }
    pub fn impl_annotate(&self) -> ItemImpl {
        let Self {
            constant,
            derive,
            attr,
            ..
        } = self;
        let Constant {
            path_annotate,
            path_annotated,
            ..
        } = constant;
        let DeriveInput {
            ident, generics, ..
        } = derive;
        let annotation_ident = self.annotation_ident();
        let annotation = attr.annotation(&annotation_ident);
        let (_, ty_generics, _) = generics.split_for_impl();
        let (impl_generics, annotation_type, where_clause) = annotation.split_for_impl(generics);
        let a = &annotation.param().ident; // TODO ?
        let fields: Vec<FieldValue> = self
            .data_struct
            .fields
            .members()
            .map(|m| parse_quote! { #m: annotation.clone() })
            .collect();
        parse_quote! {
            impl #impl_generics #path_annotate<#annotation_type> for #ident #ty_generics #where_clause {
                type Annotation = #a;
                fn annotated(self, annotation: Self::Annotation) -> #path_annotated<Self, #annotation_type> {
                    #path_annotated {
                        value: self,
                        annotation: #annotation_ident {
                            #( #fields ),*
                        },
                    }
                }
            }
        }
    }
}
