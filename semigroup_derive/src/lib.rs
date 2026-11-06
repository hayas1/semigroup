mod attr;
mod constant;
mod error;
mod name;
mod op;
mod properties;
mod semigroup;

/// Derive `Semigroup` and its properties for a [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
/// that implements `SemigroupOp<T>` (and `SelectiveOp<T>` with `selective`).
///
/// ```text
/// #[semigroup_op(
///     properties(selective, monoid(identity = Self(None)), commutative),
///     manual_impl(selective),           // optional
///     bound(monoid = "T: num::Zero"),   // optional
///     hidden_inner,                     // optional, the type is not a newtype: no `Construction` or `SemigroupOp` glue
/// )]
/// ```
///
/// | attribute | content |
/// | --- | --- |
/// | `properties(..)` | the properties to derive besides `Semigroup`: `annotated(param = A)`, `selective`, `monoid(identity = <expr>)`, `commutative`; a bare word takes the defaults |
/// | `manual_impl(..)` | the impls to leave out so that they can be written by hand: `semigroup`, `selective`, `monoid` |
/// | `bound = ".."` | where predicates for every impl, such as `bound = "T: Default, T: Copy"` |
/// | `bound(..)` | where predicates for each impl: `semigroup`, `annotated`, `selective`, `monoid`, `commutative` |
///
/// As in serde, a bound *replaces* the bounds the derive would infer for that impl,
/// and `bound` takes one of its two forms.
/// `manual_impl(..)` and `bound(..)` may name only properties listed in `properties(..)`; `semigroup` is always derived.
///
/// Specific to `#[semigroup_op]`: `manual_impl(semigroup)` skips the `SemigroupOp` impl derived from
/// `SelectiveOp`, so it requires `selective`; `annotated` is not supported.
#[proc_macro_derive(SemigroupOp, attributes(semigroup_op))]
pub fn derive_semigroup_op(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    op::impl_op::<constant::External>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(SemigroupOpPriv, attributes(semigroup_op))]
pub fn derive_semigroup_op_internal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    op::impl_op::<constant::Internal>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derive `Semigroup` and its properties for a struct by combining it field by field.
///
/// ```text
/// #[semigroup(
///     properties(annotated, monoid, commutative),
///     with = "semigroup::op::Coalesce",  // optional, the op of every field without its own `with`
///     bound = "T: Clone",                // optional
/// )]
/// struct Config {
///     #[semigroup(with = "semigroup::op::Last")]
///     name: String,
///     ..
/// }
/// ```
///
/// | attribute | content |
/// | --- | --- |
/// | `properties(..)` | the properties to derive besides `Semigroup`: `annotated(param = A)`, `selective`, `monoid(identity = <expr>)`, `commutative`; a bare word takes the defaults |
/// | `manual_impl(..)` | the impls to leave out so that they can be written by hand: `semigroup`, `selective`, `monoid` |
/// | `bound = ".."` | where predicates for every impl, such as `bound = "T: Default, T: Copy"` |
/// | `bound(..)` | where predicates for each impl: `semigroup`, `annotated`, `selective`, `monoid`, `commutative` |
///
/// As in serde, a bound *replaces* the bounds the derive would infer for that impl,
/// and `bound` takes one of its two forms.
/// `manual_impl(..)` and `bound(..)` may name only properties listed in `properties(..)`; `semigroup` is always derived.
///
/// Specific to `#[semigroup]`: without a bound, each impl requires its trait of every field type
/// combined without `with` (neither its own nor the container's);
/// `selective` and `manual_impl(semigroup)` are not supported.
#[proc_macro_derive(Semigroup, attributes(semigroup))]
pub fn derive_semigroup(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    semigroup::impl_semigroup::<constant::External>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(SemigroupPriv, attributes(semigroup))]
pub fn derive_semigroup_internal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    semigroup::impl_semigroup::<constant::Internal>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Embed the properties table into the `<!-- properties -->` placeholder of the doc comment.
///
/// It takes no arguments: the properties and bounds come from the `#[semigroup_op(..)]` or
/// `#[semigroup(..)]` of the same item, so nothing is written twice. Without either, the table
/// lists no property beyond `Semigroup`.
#[proc_macro_attribute]
pub fn semigroup_properties_doc(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let syn::parse::Nothing = syn::parse_macro_input!(attr);
    let item_struct = syn::parse_macro_input!(item);
    properties::impl_properties::<constant::External>(&item_struct)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn semigroup_properties_doc_priv(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let syn::parse::Nothing = syn::parse_macro_input!(attr);
    let item_struct = syn::parse_macro_input!(item);
    properties::impl_properties::<constant::Internal>(&item_struct)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
