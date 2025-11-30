use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, Field, ItemImpl, parse_quote};

use crate::{annotation::Annotation, constant::Constant, construction::attr::ContainerAttr};

#[derive(Debug, Clone)]
pub struct OpTrait<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,
    attr: &'a ContainerAttr,
    annotation: Annotation,

    field: &'a Field,
}

impl ToTokens for OpTrait<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_op_with_unit_annotation().to_tokens(tokens);
    }
}
impl<'a> OpTrait<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
        field: &'a Field,
    ) -> syn::Result<Self> {
        let annotation = attr.annotation(constant);

        Ok(Self {
            constant,
            derive,
            attr,
            annotation,
            field,
        })
    }

    pub fn impl_op_with_unit_annotation(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_annotated,
                    path_construction_semigroup,
                    path_construction_annotated,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            field: Field { ty, .. },
            ..
        } = self;

        attr.is_annotated().then(|| {
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
            let unit_annotation = attr.unit_annotation();
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_construction_semigroup<#ty> for #ident #ty_generics #where_clause {
                    fn lift_op_assign(base: &mut #ty, other: #ty) {
                        let (mut base_unit, other_unit) = (#unit_annotation, #unit_annotation);
                        let (b, o) = (
                            #path_annotated::new(base, &mut base_unit),
                            #path_annotated::new(other, other_unit),
                        );
                        <Self as #path_construction_annotated<_, _>>::lift_annotated_op_assign(b, o);
                    }
                }
            }
        })
    }
}
