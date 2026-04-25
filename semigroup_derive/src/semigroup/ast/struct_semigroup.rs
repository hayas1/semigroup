use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident};
use syn::{
    Block, DataStruct, DeriveInput, Expr, FieldValue, Fields, GenericParam, Ident, ItemImpl,
    ItemStruct, parse_quote,
};

use crate::{
    constant::Constant,
    semigroup::{
        ast::field_semigroup::{FieldAnnotated, FieldSemigroupOp},
        attr::ContainerAttr,
    },
};

#[derive(Debug, Clone)]
pub struct StructSemigroup<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,
    attr: &'a ContainerAttr,
    field_ops: Vec<FieldSemigroupOp<'a>>,
}
impl ToTokens for StructSemigroup<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_semigroup().to_tokens(tokens);
        self.impl_monoid().to_tokens(tokens);
        self.impl_commutative().to_tokens(tokens);
    }
}
impl<'a> StructSemigroup<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
        data_struct: &'a DataStruct,
    ) -> syn::Result<Self> {
        let field_ops = FieldSemigroupOp::new_fields(constant, derive, attr, &data_struct.fields)?;
        Ok(Self {
            constant,
            derive,
            attr,
            field_ops,
        })
    }
    pub fn impl_semigroup(&self) -> ItemImpl {
        let Self {
            constant,
            derive,
            field_ops,
            ..
        } = self;
        let Constant {
            path_semigroup,
            path_semigroup_op,
            ..
        } = constant;
        let DeriveInput {
            ident, generics, ..
        } = derive;
        let mut g = generics.clone();
        let where_clause = g.make_where_clause();
        field_ops
            .iter()
            .flat_map(|op| {
                op.where_ty()
                    .map(|ty| parse_quote! { #ty: #path_semigroup })
            })
            .for_each(|w| where_clause.predicates.push(w));
        let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
        let fields_op_assign = field_ops
            .iter()
            .map(|op| op.impl_field_semigroup_op_assign());
        parse_quote! {
            #[automatically_derived]
            impl #impl_generics #path_semigroup for #ident #ty_generics #where_clause {
                fn op_assign(base: &mut Self, other: Self) {
                    use #path_semigroup_op;
                    #(#fields_op_assign)*
                }
            }
        }
    }
    pub fn impl_monoid(&self) -> Option<ItemImpl> {
        let Self {
            constant,
            derive,
            field_ops,
            attr,
            ..
        } = self;
        let Constant {
            path_monoid,
            path_monoid_op,
            attr_feature_monoid,
            ..
        } = constant;
        let DeriveInput {
            ident, generics, ..
        } = derive;
        (attr.is_monoid() && attr.with_monoid_impl()).then(|| {
            let mut g = generics.clone();
            let where_clause = g.make_where_clause();
            field_ops
                .iter()
                .flat_map(|op| op.where_ty().map(|ty| parse_quote! { #ty: #path_monoid }))
                .for_each(|w| where_clause.predicates.push(w));
            attr.monoid_where()
                .into_iter()
                .for_each(|w| where_clause.predicates.push(w));

            let (impl_generics, ty_generics, where_clause) = g.split_for_impl();

            attr.identity()
                .map(|expr| {
                    parse_quote! {
                        #[automatically_derived]
                        #attr_feature_monoid
                        impl #impl_generics #path_monoid for #ident #ty_generics #where_clause {
                            fn identity() -> Self {
                                #expr
                            }
                        }
                    }
                })
                .unwrap_or_else(|| {
                    let fields_op = field_ops.iter().map(|op| {
                        op.impl_field_monoid_identity()
                            .unwrap_or_else(|e| todo!("{e}"))
                    });
                    parse_quote! {
                        #[automatically_derived]
                        #attr_feature_monoid
                        impl #impl_generics #path_monoid for #ident #ty_generics #where_clause {
                            fn identity() -> Self {
                                use #path_monoid_op;
                                Self {
                                    #(#fields_op),*
                                }
                            }
                        }
                    }
                })
        })
    }
    pub fn impl_commutative(&self) -> Option<ItemImpl> {
        let Self {
            constant,
            derive,
            field_ops,
            attr,
            ..
        } = self;
        let Constant {
            path_commutative, ..
        } = constant;
        let DeriveInput {
            ident, generics, ..
        } = derive;
        let mut g = generics.clone();
        let where_clause = g.make_where_clause();
        field_ops
            .iter()
            .flat_map(|op| {
                op.where_ty()
                    .map(|ty| parse_quote! { #ty: #path_commutative })
            })
            .for_each(|w| where_clause.predicates.push(w));
        attr.commutative_where()
            .into_iter()
            .for_each(|w| where_clause.predicates.push(w));

        let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
        attr.is_commutative().then(|| {
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_commutative for #ident #ty_generics #where_clause {}
            }
        })
    }
}

/// Generates the `XxxAnnotated<A>` struct and related impls for `#[semigroup(annotated)]`.
///
/// For each field in the original struct, the annotated struct has a corresponding field of type
/// `Annotated<OpWrapper<FieldType>, A>`.  The `Semigroup` impl is generated via the blanket
/// `impl<T: Semigroup + Idempotent, A> Semigroup for Annotated<T, A>`.
#[derive(Debug, Clone)]
pub struct StructAnnotated<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,
    data_struct: &'a DataStruct,
    annotated_ident: Ident,
    field_annotated: Vec<FieldAnnotated<'a>>,
}
impl ToTokens for StructAnnotated<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.def_annotated_struct().to_tokens(tokens);
        self.impl_semigroup_for_annotated().to_tokens(tokens);
        self.impl_annotated_method().to_tokens(tokens);
        self.impl_from_for_original().to_tokens(tokens);
    }
}
impl<'a> StructAnnotated<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
        data_struct: &'a DataStruct,
    ) -> syn::Result<Self> {
        let annotated_ident = Self::annotated_ident(&derive.ident);
        let field_annotated =
            FieldAnnotated::new_fields(constant, derive, attr, &data_struct.fields)?;
        Ok(Self {
            constant,
            derive,
            data_struct,
            annotated_ident,
            field_annotated,
        })
    }

    pub fn annotated_ident(ident: &Ident) -> Ident {
        format_ident!("{}Annotated", ident)
    }

    /// Returns a clone of the original generics extended with `A: Clone` type parameter.
    fn generics_with_a(&self) -> syn::Generics {
        let mut g = self.derive.generics.clone();
        g.params.push(GenericParam::Type(parse_quote! { A: Clone }));
        g
    }

    /// Generates the `XxxAnnotated<A>` struct definition with per-field `Annotated<Op, A>` fields.
    pub fn def_annotated_struct(&self) -> ItemStruct {
        let Self {
            constant: Constant { path_annotated, .. },
            derive: DeriveInput { vis, generics, .. },
            data_struct,
            annotated_ident,
            field_annotated,
            ..
        } = self;

        let a_param: GenericParam = parse_quote! { A };
        let a_ident: Ident = parse_quote! { A };

        let mut g = generics.clone();
        g.params.push(a_param);
        let (_, ty_generics, where_clause) = g.split_for_impl();

        let field_defs: Vec<syn::Field> = field_annotated
            .iter()
            .map(|f| f.struct_field_tokens(&a_ident, path_annotated))
            .collect();

        match &data_struct.fields {
            Fields::Named(_) => {
                parse_quote! {
                    #[derive(Debug, Clone, PartialEq)]
                    #vis struct #annotated_ident #ty_generics #where_clause {
                        #( #field_defs ),*
                    }
                }
            }
            Fields::Unnamed(_) => {
                parse_quote! {
                    #[derive(Debug, Clone, PartialEq)]
                    #vis struct #annotated_ident #ty_generics( #( #field_defs ),* ) #where_clause;
                }
            }
            Fields::Unit => todo!(),
        }
    }

    /// Generates `impl<..., A> Semigroup for XxxAnnotated<..., A>`.
    pub fn impl_semigroup_for_annotated(&self) -> ItemImpl {
        let Self {
            constant:
                Constant {
                    path_semigroup,
                    path_semigroup_op,
                    path_idempotent_op,
                    path_selected,
                    ..
                },
            annotated_ident,
            field_annotated,
            ..
        } = self;

        let g = self.generics_with_a();
        let (impl_generics, ty_generics, where_clause) = g.split_for_impl();

        let fields_op_assign: Vec<Block> = field_annotated
            .iter()
            .map(|f| {
                f.annotated_op_assign_stmts(path_selected)
                    .unwrap_or_else(|e| todo!("{e}"))
            })
            .collect();

        parse_quote! {
            #[automatically_derived]
            impl #impl_generics #path_semigroup for #annotated_ident #ty_generics #where_clause {
                fn op_assign(base: &mut Self, other: Self) {
                    use #path_idempotent_op;
                    use #path_semigroup_op;
                    #( #fields_op_assign )*
                }
            }
        }
    }

    /// Generates `impl<A: Clone> AnnotateFields<A> for Xxx { type Annotated = XxxAnnotated<..., A>; fn annotated(...) }`.
    pub fn impl_annotated_method(&self) -> ItemImpl {
        let Self {
            constant:
                Constant {
                    path_annotated,
                    path_annotate_fields,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            data_struct,
            annotated_ident,
            field_annotated,
            ..
        } = self;

        let g_with_a = self.generics_with_a();
        let (impl_generics, ret_ty_generics, where_clause) = g_with_a.split_for_impl();
        let (_, ty_generics, _) = generics.split_for_impl();

        let n = field_annotated.len();

        let struct_init: Expr = match &data_struct.fields {
            Fields::Named(_) => {
                let field_inits: Vec<FieldValue> = field_annotated
                    .iter()
                    .enumerate()
                    .map(|(i, f)| f.annotated_init_named(path_annotated, i + 1 == n))
                    .collect();
                parse_quote! {
                    #annotated_ident {
                        #( #field_inits ),*
                    }
                }
            }
            Fields::Unnamed(_) => {
                let field_inits: Vec<Expr> = field_annotated
                    .iter()
                    .enumerate()
                    .map(|(i, f)| f.annotated_init_value_expr(path_annotated, i + 1 == n))
                    .collect();
                parse_quote! {
                    #annotated_ident(
                        #( #field_inits ),*
                    )
                }
            }
            Fields::Unit => todo!(),
        };

        parse_quote! {
            #[automatically_derived]
            impl #impl_generics #path_annotate_fields<A> for #ident #ty_generics #where_clause {
                type Annotated = #annotated_ident #ret_ty_generics;
                fn annotated(self, annotation: A) -> Self::Annotated {
                    #struct_init
                }
            }
        }
    }

    /// Generates `impl<A> From<XxxAnnotated<..., A>> for Xxx<...>`.
    pub fn impl_from_for_original(&self) -> ItemImpl {
        let Self {
            derive: DeriveInput {
                ident, generics, ..
            },
            data_struct,
            annotated_ident,
            field_annotated,
            ..
        } = self;

        let mut g_with_a = generics.clone();
        g_with_a.params.push(GenericParam::Type(parse_quote! { A }));
        let (impl_generics, ret_ty_generics, where_clause) = g_with_a.split_for_impl();
        let (_, ty_generics, _) = generics.split_for_impl();

        let struct_init: Expr = match &data_struct.fields {
            Fields::Named(_) => {
                let field_inits: Vec<FieldValue> =
                    field_annotated.iter().map(|f| f.from_named()).collect();
                parse_quote! { Self { #(#field_inits),* } }
            }
            Fields::Unnamed(_) => {
                let field_inits: Vec<Expr> = field_annotated
                    .iter()
                    .map(|f| f.from_value_expr())
                    .collect();
                parse_quote! { Self(#(#field_inits),*) }
            }
            Fields::Unit => todo!(),
        };

        parse_quote! {
            #[automatically_derived]
            impl #impl_generics From<#annotated_ident #ret_ty_generics> for #ident #ty_generics #where_clause {
                fn from(annotated: #annotated_ident #ret_ty_generics) -> Self {
                    #struct_init
                }
            }
        }
    }
}
