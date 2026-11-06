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
///     bound = "T: Clone",  // optional, every impl
///     hidden_inner,        // optional, the type is not a newtype: no `Construction` or `SemigroupOp` glue
/// )]
/// ```
///
/// # `properties(..)`
/// Each property groups everything about it, and a bare word enables it with its defaults:
///
/// | property | arguments | generates |
/// | --- | --- | --- |
/// | `semigroup` | `bound = ".."`, `manual_impl` | `Semigroup` |
/// | `annotated` | `param = A` | `XxxAnnotated<.., A>` and `AnnotateFields<A>` |
/// | `selective` | `bound = ".."`, `manual_impl` | `Selective` |
/// | `monoid` | `identity = <expr>`, `bound = ".."`, `manual_impl` | `Monoid` |
/// | `commutative` | `bound = ".."` | `Commutative` |
///
/// - `bound` takes where predicates as a string, such as `bound = "T: Default, T: Copy"`.
///   As in serde, a bound *replaces* the bounds the derive would infer for that impl.
///   A property's own `bound` takes precedence over the container's `bound`, which applies to every impl.
/// - `manual_impl` skips generating the impl, so that it can be written by hand.
///
/// Specific to `#[semigroup_op]`: `semigroup(manual_impl)` skips the `SemigroupOp` impl derived from
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
///     bound = "T: Clone",                // optional, every impl
/// )]
/// struct Config {
///     #[semigroup(with = "semigroup::op::Last")]
///     name: String,
///     ..
/// }
/// ```
///
/// # `properties(..)`
/// Each property groups everything about it, and a bare word enables it with its defaults:
///
/// | property | arguments | generates |
/// | --- | --- | --- |
/// | `semigroup` | `bound = ".."`, `manual_impl` | `Semigroup` |
/// | `annotated` | `param = A` | `XxxAnnotated<.., A>` and `AnnotateFields<A>` |
/// | `selective` | `bound = ".."`, `manual_impl` | `Selective` |
/// | `monoid` | `identity = <expr>`, `bound = ".."`, `manual_impl` | `Monoid` |
/// | `commutative` | `bound = ".."` | `Commutative` |
///
/// - `bound` takes where predicates as a string, such as `bound = "T: Default, T: Copy"`.
///   As in serde, a bound *replaces* the bounds the derive would infer for that impl.
///   A property's own `bound` takes precedence over the container's `bound`, which applies to every impl.
/// - `manual_impl` skips generating the impl, so that it can be written by hand.
///
/// Specific to `#[semigroup]`: without `bound`, each impl requires its trait of every field type
/// combined without `with` (neither its own nor the container's);
/// `selective` and `semigroup(manual_impl)` are not supported.
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
/// It takes the content of `properties(..)` of the derives, such as
/// `#[properties(selective, monoid(bound = "T: num::Bounded"), commutative)]`,
/// so the same arguments can be copied from `#[semigroup_op(properties(..))]`.
#[proc_macro_attribute]
pub fn properties(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let meta = syn::parse_macro_input!(attr);
    let item_struct = syn::parse_macro_input!(item);
    properties::impl_properties::<constant::External>(&meta, &item_struct)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn properties_priv(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let meta = syn::parse_macro_input!(attr);
    let item_struct = syn::parse_macro_input!(item);
    properties::impl_properties::<constant::Internal>(&meta, &item_struct)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
