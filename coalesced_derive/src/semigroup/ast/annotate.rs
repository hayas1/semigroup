use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{
    parse_quote, DataStruct, DeriveInput, FieldValue, Fields, Ident, ItemImpl, ItemStruct, Member,
    Stmt,
};

use crate::{
    constant::Constant,
    generics::Annotated,
    semigroup::attr::{ContainerAttr, FieldAttr},
};

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
        let (local, value, annotation): (Vec<_>, Vec<_>, Vec<_>) =
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
        let annotated = Annotated::new(path_annotated, ident, generics, parse_quote! { A }, None);
        let a = annotated.type_param().ident;
        let (_, ty_generics, _) = generics.split_for_impl();
        let (impl_generics, _, where_clause) = annotated.split_for_impl();
        Ok(parse_quote! {
            impl #impl_generics #path_annotated_semigroup<#annotation_ident<#a>> for #ident #ty_generics #where_clause {
                fn #ident_annotated_op(base: #path_annotated<Self, #annotation_ident<#a>>, other: #path_annotated<Self, #annotation_ident<#a>>) -> #path_annotated<Self, #annotation_ident<#a>> {
                    #( #local )*
                    #path_annotated {
                        value: #ident {
                            #(#value),*
                        },
                        annotation: #annotation_ident {
                            #(#annotation),*
                        },
                    }
                }
            }
        })
    }
    pub fn impl_annotate(&self) -> ItemImpl {
        let Self {
            constant, derive, ..
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
        let annotated = Annotated::new(
            path_annotated,
            ident,
            generics,
            parse_quote! { A: Clone },
            None,
        );
        let (_, ty_generics, _) = generics.split_for_impl();
        let (impl_generics, _, where_clause) = annotated.split_for_impl();
        let a = annotated.type_param().ident;
        let fields: Vec<FieldValue> = self
            .data_struct
            .fields
            .members()
            .map(|m| parse_quote! { #m: annotation.clone() })
            .collect();
        parse_quote! {
            impl #impl_generics #path_annotate<#annotation_ident<#a>> for #ident #ty_generics #where_clause {
                type Annotation = #a;
                fn annotated(self, annotation: Self::Annotation) -> #path_annotated<Self, #annotation_ident<#a>> {
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

#[derive(Debug, Clone)]
pub struct FieldAnnotatedOp<'a> {
    constant: &'a Constant,
    container_attr: &'a ContainerAttr,
    member: Member,
    field_attr: FieldAttr,
}
impl<'a> FieldAnnotatedOp<'a> {
    pub fn new(
        constant: &'a Constant,
        _derive: &'a DeriveInput,
        container_attr: &'a ContainerAttr,
        member: Member,
        field_attr: FieldAttr,
    ) -> Self {
        Self {
            constant,
            container_attr,
            member,
            field_attr,
        }
    }
    pub fn new_fields(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        container_attr: &'a ContainerAttr,
        fields: &'a Fields,
    ) -> syn::Result<Vec<Self>> {
        fields
            .iter()
            .zip(fields.members())
            .map(|(field, member)| {
                Ok(Self::new(
                    constant,
                    derive,
                    container_attr,
                    member,
                    FieldAttr::new(field)?,
                ))
            })
            .collect()
    }

    pub fn ident_variable(&self) -> Ident {
        match &self.member {
            Member::Named(ident) => ident.clone(),
            Member::Unnamed(index) => format_ident!("_{}", index.index),
        }
    }
    pub fn impl_field_annotated_op(&self) -> Stmt {
        let Self {
            constant,
            container_attr,
            member,
            field_attr,
        } = self;
        let Constant {
            path_annotated_semigroup,
            ident_annotated_op,
            path_construction_trait,
            path_annotated,
            ..
        } = constant;
        let ident_variable = self.ident_variable();
        let with = field_attr.with(container_attr);

        with.map(|path| {
            parse_quote! {
                let #ident_variable = #path_annotated_semigroup::#ident_annotated_op(
                    #path_annotated{ value: base.value.#member, annotation: base.annotation.#member }.map(<#path<_> as #path_construction_trait<_>>::new),
                    #path_annotated{ value: other.value.#member, annotation: other.annotation.#member }.map(<#path<_> as #path_construction_trait<_>>::new),
                );
            }
        })
        .unwrap_or_else(|| {
            parse_quote! {
                let #ident_variable = #path_annotated_semigroup::#ident_annotated_op(
                    #path_annotated{ value: base.value.#member, annotation: base.annotation.#member },
                    #path_annotated{ value: other.value.#member, annotation: other.annotation.#member },
                );
            }
        })
    }
    pub fn impl_field_value(&self) -> FieldValue {
        let Self {
            constant:
                Constant {
                    path_construction_trait,
                    ..
                },
            member,
            ..
        } = self;
        let ident_variable = self.ident_variable();
        let with = self.field_attr.with(self.container_attr);
        with.map(|path| {
            parse_quote! {
                #member: <#path<_> as #path_construction_trait<_>>::into_inner(#ident_variable.value)
            }
        })
        .unwrap_or_else(|| {
            parse_quote! {
                #member: #ident_variable.value
            }
        })
    }
    pub fn impl_field_annotation(&self) -> FieldValue {
        let Self { member, .. } = self;
        let ident_variable = self.ident_variable();
        parse_quote! {
            #member: #ident_variable.annotation
        }
    }
}
