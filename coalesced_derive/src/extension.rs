use proc_macro2::TokenStream;
use syn::{parse_quote, Data, DataStruct, DeriveInput, ItemImpl, ItemStruct};

use crate::error::DeriveError;

pub struct Implementor {
    input: DeriveInput,
}
impl Implementor {
    pub fn new(input: DeriveInput) -> Self {
        Self { input }
    }

    pub fn implement(&self) -> TokenStream {
        let DeriveInput { ident, data, .. } = &self.input;
        match &data {
            Data::Enum(_e) => todo!(),
            Data::Struct(s) => self.implement_struct(s),
            Data::Union(_) => {
                syn::Error::new_spanned(ident, DeriveError::UnsupportedUnion).to_compile_error()
            }
        }
    }

    fn implement_struct(&self, s: &DataStruct) -> TokenStream {
        let DeriveInput {
            ident, generics, ..
        } = &self.input;
        let (g_impl, g_type, g_where) = generics.split_for_impl();

        let extension = self.implement_struct_extension();
        let with_ext_def = self.definition_struct_with_ext();
        let with_ext_impl = self.implement_struct_with_ext();
        let from_with_ext = self.implement_struct_from_with_ext();
        parse_quote! {
            #extension
            #with_ext_def
            #with_ext_impl
            #from_with_ext
        }
    }

    fn implement_struct_extension(&self) -> ItemImpl {
        todo!()
    }
    fn definition_struct_with_ext(&self) -> ItemStruct {
        todo!()
    }
    fn implement_struct_with_ext(&self) -> ItemImpl {
        todo!()
    }
    fn implement_struct_from_with_ext(&self) -> ItemImpl {
        todo!()
    }
}
