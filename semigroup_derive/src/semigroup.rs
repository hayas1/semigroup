mod ast;
pub mod attr;

pub use ast::Semigroup;

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use syn::DeriveInput;

    use crate::derive::SemigroupDerive;

    use super::*;

    #[rstest]
    #[case::semigroup_annotated(
        "semigroup_annotated",
        &[SemigroupDerive::Semigroup, SemigroupDerive::AnnotateFields],
        syn::parse_quote! {
            #[semigroup(with = "semigroup::op::Coalesce")]
            pub struct NamedStruct {
                #[semigroup(with = "semigroup::op::Last")]
                pub foo: String,
                pub bar: Option<u32>,
            }
        },
    )]
    #[case::semigroup_not_annotated(
        "semigroup_not_annotated",
        &[SemigroupDerive::Semigroup],
        syn::parse_quote! {
            #[semigroup(with = "semigroup::op::Last", crate = crate)]
            pub struct UnnamedStruct<T: std::ops::Add> (
                #[semigroup(with = "semigroup::op::Added")]
                T,
                u64
            );
        },
    )]
    #[case::semigroup_with_dual(
        "semigroup_with_dual",
        &[SemigroupDerive::Semigroup],
        syn::parse_quote! {
            #[semigroup(crate = crate)]
            pub struct NamedStruct{
                #[semigroup(with = "semigroup::op::Last")]
                pub foo: String,
                #[semigroup(with = "semigroup::op::Last(_)")]
                pub bar: String,
                #[semigroup(with = "semigroup::Dual(semigroup::op::Last(_))")]
                pub baz: String,
            }
        },
    )]
    #[case::semigroup_monoid(
        "semigroup_monoid",
        &[SemigroupDerive::Semigroup, SemigroupDerive::Monoid, SemigroupDerive::Commutative],
        syn::parse_quote! {
            pub struct MonoidStruct{
                #[semigroup(with = "semigroup::op::Sum")]
                pub sum: u32,
            }
        },
    )]
    #[case::semigroup_monoid_identity(
        "semigroup_monoid_identity",
        &[SemigroupDerive::Semigroup, SemigroupDerive::Monoid],
        syn::parse_quote! {
            #[semigroup(impl(monoid = Self { sum: 0 }))]
            pub struct MonoidStruct{
                #[semigroup(with = "semigroup::op::Sum")]
                pub sum: u32,
            }
        },
    )]
    #[case::semigroup_generics(
        "semigroup_generics",
        &[SemigroupDerive::Semigroup],
        syn::parse_quote! {
            #[semigroup(crate = crate)]
            pub struct UnnamedStruct<T, U> (
                T,
                Vec<U>,
            );
        },
    )]
    #[case::semigroup_bound(
        "semigroup_bound",
        &[SemigroupDerive::Semigroup, SemigroupDerive::AnnotateFields, SemigroupDerive::Monoid, SemigroupDerive::Commutative],
        syn::parse_quote! {
            #[semigroup(
                annotated(param = B),
                bound(
                    semigroup = "T: Semigroup + Clone",
                    annotated = "T: Semigroup + Clone",
                    monoid = "T: Semigroup + Clone",
                ),
                crate = crate,
            )]
            pub struct Tagged<T> {
                pub value: T,
                #[semigroup(with = "semigroup::op::Sum")]
                pub count: u32,
            }
        },
    )]
    #[case::semigroup_commutative_enum(
        "semigroup_commutative_enum",
        &[SemigroupDerive::Monoid, SemigroupDerive::Commutative],
        syn::parse_quote! {
            #[semigroup(impl(monoid = Inner::Value(0)), bound(commutative = ""))]
            enum Inner<C> {
                Value(u64),
                Histogram(C),
            }
        },
    )]
    fn test_derive_semigroup_snapshot(
        #[case] case: &str,
        #[case] derives: &[SemigroupDerive],
        #[case] input: DeriveInput,
    ) {
        let generated: TokenStream = derives
            .iter()
            .map(|&derive| Semigroup::new(&input, derive).unwrap().into_token_stream())
            .collect();
        let formatted = prettyplease::unparse(&syn::parse2(generated).unwrap());
        insta::with_settings!({ snapshot_path => "../tests/snapshots", prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!(case, formatted);
        });
    }

    #[rstest]
    #[case::semigroup_enum(
        syn::parse_quote! {
            #[derive(Semigroup)]
            enum Inner { A(u32) }
        },
        SemigroupDerive::Semigroup,
        "derive Semigroup only supports structs",
    )]
    #[case::commutative_enum_without_bound(
        syn::parse_quote! {
            #[derive(Commutative)]
            enum Inner { A(u32) }
        },
        SemigroupDerive::Commutative,
        "derive Commutative infers its bounds from struct fields, so other items require `bound(commutative = ..)`",
    )]
    fn test_derive_semigroup_error(
        #[case] input: DeriveInput,
        #[case] semigroup_derive: SemigroupDerive,
        #[case] expected: &str,
    ) {
        let actual = Semigroup::new(&input, semigroup_derive)
            .map(|_| ())
            .map_err(|e| e.to_string());
        assert_eq!(actual, Err(expected.to_string()));
    }
}
