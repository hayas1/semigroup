use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, ItemStruct};

use crate::properties::{attr::PropertiesAttr, documented::content::Content};

mod content;
mod table;

#[derive(Debug, Clone)]
pub struct Documented<'a> {
    doc: Vec<Attribute>,
    item: &'a ItemStruct,
}
impl ToTokens for Documented<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.documented_item().to_tokens(tokens);
    }
}
impl<'a> Documented<'a> {
    pub fn new(item: &'a ItemStruct) -> syn::Result<Self> {
        let attr = PropertiesAttr::try_from(item)?;
        let constant = attr.constant();
        let mut content = Content::new(&constant, &attr, item)?;
        content.embed_properties();
        Ok(Self {
            doc: content.to_attributes(),
            item,
        })
    }
    pub fn documented_item(&self) -> ItemStruct {
        let others = self
            .item
            .attrs
            .iter()
            .filter(|attr| !attr.path().is_ident("doc"))
            .cloned();
        ItemStruct {
            attrs: self.doc.iter().cloned().chain(others).collect(),
            ..self.item.clone()
        }
    }
}
