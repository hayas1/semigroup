use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{parse_quote, DataStruct, DeriveInput, ItemStruct};

use crate::{constant::Constant, semigroup::attr::ContainerAttr};

#[derive(Debug, Clone)]
pub struct StructAnnotate<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,
    attr: &'a ContainerAttr,
    data_struct: &'a DataStruct,
}
impl ToTokens for StructAnnotate<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let _ = tokens;
        todo!()
    }
}
impl<'a> StructAnnotate<'a> {
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

    pub fn def_annotation(&self) -> ItemStruct {
        let Self {
            derive: DeriveInput { vis, ident, .. },
            data_struct,
            ..
        } = self;
        let annotation_ident = format_ident!("{}Annotation", ident);
        match &data_struct.fields {
            syn::Fields::Named(fields) => {
                let idents = fields.named.iter().map(|f| &f.ident);
                parse_quote! {
                    #vis struct #annotation_ident<A> {
                        #( #idents: A ),*
                    }
                }
            }
            syn::Fields::Unnamed(fields) => {
                let annotation = fields.unnamed.iter().map(|_| format_ident!("A"));
                parse_quote! {
                    #vis struct #annotation_ident<A>( #( #annotation ),* );
                }
            }
            syn::Fields::Unit => todo!(),
        }
    }
}
