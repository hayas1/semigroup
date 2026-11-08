mod attr;
mod constant;
mod derive;
mod error;
mod op;
mod properties;
mod semigroup;

use quote::ToTokens;

use crate::derive::{OpDerive, SemigroupDerive};

/// Derive `Semigroup` for a struct by combining it field by field.
///
/// ```text
/// #[derive(Semigroup)]
/// #[semigroup(with = "semigroup::op::Coalesce")]  // optional, the op of every field without its own `with`
/// struct Config {
///     #[semigroup(with = "semigroup::op::Last")]
///     name: String,
///     ..
/// }
/// ```
///
/// Without a bound, the impl requires `Semigroup` of every field type combined without `with`.
///
#[doc = include_str!("docs/attr.md")]
#[proc_macro_derive(Semigroup, attributes(semigroup))]
pub fn derive_semigroup(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);
    semigroup::Semigroup::new(&input, SemigroupDerive::Semigroup)
        .map_or_else(syn::Error::into_compile_error, ToTokens::into_token_stream)
        .into()
}

/// Derive `Monoid` for a struct whose fields are monoids, or with a given identity.
///
/// ```text
/// #[derive(Semigroup, Monoid)]
/// #[semigroup(impl(monoid = Self { num: None }))]  // optional, the identity of every field without it
/// ```
///
/// Without `impl(monoid = ..)` or a bound, the impl requires `Monoid` of every field type combined without `with`.
///
#[doc = include_str!("docs/attr.md")]
#[proc_macro_derive(Monoid, attributes(semigroup))]
pub fn derive_monoid(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);
    semigroup::Semigroup::new(&input, SemigroupDerive::Monoid)
        .map_or_else(syn::Error::into_compile_error, ToTokens::into_token_stream)
        .into()
}

/// Derive `Commutative` for a struct whose fields are commutative.
///
/// Without a bound, the impl requires `Commutative` of every field type combined without `with`.
/// An item other than a struct requires `bound(commutative = "..")`.
///
#[doc = include_str!("docs/attr.md")]
#[proc_macro_derive(Commutative, attributes(semigroup))]
pub fn derive_commutative(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);
    semigroup::Semigroup::new(&input, SemigroupDerive::Commutative)
        .map_or_else(syn::Error::into_compile_error, ToTokens::into_token_stream)
        .into()
}

/// Derive `AnnotateFields` and the `XxxAnnotated` struct that keeps an annotation per field.
///
/// ```text
/// #[derive(Semigroup, AnnotateFields)]
/// #[semigroup(annotated(param = B))]  // optional, the type parameter of the annotation, `A` unless given
/// ```
///
#[doc = include_str!("docs/attr.md")]
#[proc_macro_derive(AnnotateFields, attributes(semigroup))]
pub fn derive_annotate_fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);
    semigroup::Semigroup::new(&input, SemigroupDerive::AnnotateFields)
        .map_or_else(syn::Error::into_compile_error, ToTokens::into_token_stream)
        .into()
}

/// Derive `SemigroupOp<T>` for a [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
/// of `T` from a function, with the `Semigroup` and `Construction` impls built on it.
///
/// ```text
/// #[derive(SemigroupOp)]
/// #[semigroup_op(impl(semigroup = |base, other| *base += other), bound = "T: AddAssign")]
/// struct Sum<T>(T);
/// ```
///
/// `impl(semigroup = ..)` takes a closure or a path of `fn(&mut T, T)`.
/// A selective op passes `Self::lift_select_assign` of its `SelectiveOp`.
///
#[doc = include_str!("docs/attr.md")]
#[proc_macro_derive(SemigroupOp, attributes(semigroup_op))]
pub fn derive_semigroup_op(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);
    op::Op::new(&input, OpDerive::Semigroup)
        .map_or_else(syn::Error::into_compile_error, ToTokens::into_token_stream)
        .into()
}

/// Derive `SelectiveOp<T>` for a newtype of `T` from a function, with the `Selective` impl built on it.
///
/// ```text
/// #[derive(SemigroupOp, SelectiveOp)]
/// #[semigroup_op(
///     impl(semigroup = Self::lift_select_assign, selective = |base, other| ..),
///     bound = "T: Ord",
/// )]
/// struct Max<T>(T);
/// ```
///
/// `impl(selective = ..)` takes a closure or a path of `fn(&T, &T) -> Selected`.
///
#[doc = include_str!("docs/attr.md")]
#[proc_macro_derive(SelectiveOp, attributes(semigroup_op))]
pub fn derive_selective_op(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);
    op::Op::new(&input, OpDerive::Selective)
        .map_or_else(syn::Error::into_compile_error, ToTokens::into_token_stream)
        .into()
}

/// Derive `MonoidOp<T>` and `Monoid` for a newtype of `T`.
///
/// ```text
/// #[derive(SemigroupOp, MonoidOp)]
/// #[semigroup_op(impl(semigroup = .., monoid = Self(T::zero())), bound = "T: AddAssign + Zero")]
/// ```
///
/// Without `impl(monoid = ..)`, the identity is `Default::default()`.
///
#[doc = include_str!("docs/attr.md")]
#[proc_macro_derive(MonoidOp, attributes(semigroup_op))]
pub fn derive_monoid_op(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);
    op::Op::new(&input, OpDerive::Monoid)
        .map_or_else(syn::Error::into_compile_error, ToTokens::into_token_stream)
        .into()
}

/// Derive `CommutativeOp<T>` and `Commutative` for a newtype of `T`.
///
/// Commutativity belongs to the op, so no bound is inferred.
///
#[doc = include_str!("docs/attr.md")]
#[proc_macro_derive(CommutativeOp, attributes(semigroup_op))]
pub fn derive_commutative_op(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);
    op::Op::new(&input, OpDerive::Commutative)
        .map_or_else(syn::Error::into_compile_error, ToTokens::into_token_stream)
        .into()
}

/// Embed the properties table into the `<!-- properties -->` placeholder of the doc comment.
///
/// Place it above `#[derive(..)]`: the properties come from the derives of the item,
/// and the bounds from its `#[semigroup(..)]` or `#[semigroup_op(..)]`.
#[proc_macro_attribute]
pub fn semigroup_properties_doc(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let syn::parse::Nothing = syn::parse_macro_input!(attr);
    let item_struct = syn::parse_macro_input!(item);
    properties::Documented::new(&item_struct)
        .map_or_else(syn::Error::into_compile_error, ToTokens::into_token_stream)
        .into()
}
