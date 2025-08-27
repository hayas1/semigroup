use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::constant::ConstantExt;

pub fn gen_semigroup<C: ConstantExt>(derive: &DeriveInput) -> TokenStream {
    let constant = C::constant();
    todo!()
}
