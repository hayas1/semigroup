use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{
    parse_quote, Arm, Data, DataEnum, DataStruct, DeriveInput, Expr, Fields, FieldsNamed,
    FieldsUnnamed, GenericParam, Ident, ItemEnum, ItemFn, ItemImpl, ItemStruct, Path,
    PathArguments, PathSegment, Type, TypeGenerics, TypeParam, TypeParamBound, Variant,
    WhereClause,
};

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
            Data::Enum(e) => self.implement_enum(e),
            Data::Struct(s) => self.implement_struct(s),
            Data::Union(_) => {
                syn::Error::new_spanned(ident, DeriveError::UnsupportedUnion).to_compile_error()
            }
        }
    }

    fn with_ext_path(&self) -> Path {
        match self.input.data {
            Data::Enum(DataEnum { .. })
            | Data::Struct(DataStruct {
                fields: Fields::Named(_) | Fields::Unnamed(_),
                ..
            }) => {
                let ident = format_ident!("{}WithExt", self.input.ident);
                let ex_type = ExTypeGenerics(self);
                parse_quote! { #ident #ex_type }
            }
            Data::Struct(DataStruct {
                fields: Fields::Unit,
                ..
            }) => {
                let ident = &self.input.ident;
                let x_param = self.x_param();
                parse_quote! { ::coalesced::WithExt<#ident, #x_param> }
            }
            Data::Union(_) => unreachable!(),
        }
    }
    fn strip_path_argument(path: &Path) -> Path {
        let segments = path.segments.iter().map(|seg| PathSegment {
            ident: seg.ident.clone(),
            arguments: PathArguments::None,
        });
        Path {
            leading_colon: path.leading_colon,
            segments: segments.collect(),
        }
    }

    fn x_param(&self) -> Ident {
        parse_quote! { X }
    }
    fn extension_generic(&self) -> GenericParam {
        let clone_bound = TypeParamBound::Trait(parse_quote! {Clone});
        GenericParam::Type(TypeParam {
            attrs: Vec::new(),
            ident: self.x_param(),
            colon_token: Some(parse_quote! { : }),
            bounds: vec![clone_bound].into_iter().collect(),
            eq_token: None,
            default: None,
        })
    }
    fn split_with_extension_generics(
        &self,
    ) -> (ExImplGenerics, TypeGenerics, Option<&WhereClause>) {
        let ex_impl = ExImplGenerics(self);
        let (_, g_type, g_where) = self.input.generics.split_for_impl();
        (ex_impl, g_type, g_where)
    }

    fn implement_struct(&self, s: &DataStruct) -> TokenStream {
        let extension = self.implement_struct_extension(s);
        let with_ext_def = self.definition_struct_with_ext(s);
        let coalesce_with_ext = self.implement_struct_coalesce_with_ext(s);
        let from_with_ext = self.implement_struct_from_with_ext();
        parse_quote! {
            #extension
            #with_ext_def
            #coalesce_with_ext
            #from_with_ext
        }
    }
    fn implement_struct_extension(&self, s: &DataStruct) -> ItemImpl {
        let DeriveInput { ident, .. } = &self.input;
        let (g_impl, g_type, g_where) = self.split_with_extension_generics();
        let x_param = self.x_param();

        let with_ext = self.with_ext_path();
        let (ex, we) = (parse_quote! { extension }, parse_quote! { with_ext });
        let (block_with_extension, block_unwrap_extension) = (
            self.implement_struct_extension_with_extension(&s.fields, &ex),
            self.implement_struct_extension_unwrap_extension(&s.fields, &we),
        );
        let (fn_ex_prior, fn_ex_posterior) = (
            self.implement_struct_extension_ex_prior(&s.fields),
            self.implement_struct_extension_ex_posterior(&s.fields),
        );
        parse_quote! {
            impl #g_impl ::coalesced::Extension<#x_param> for #ident #g_type #g_where {
                type WithExt = #with_ext;
                fn with_extension(self, #ex: #x_param) -> Self::WithExt {
                    #block_with_extension
                }
                fn unwrap_extension(#we: Self::WithExt) -> Self {
                    #block_unwrap_extension
                }
                #fn_ex_prior
                #fn_ex_posterior
            }
        }
    }
    fn implement_struct_extension_with_extension(&self, f: &Fields, ex: &Ident) -> Expr {
        let with_ext = Self::strip_path_argument(&self.with_ext_path());
        match f {
            Fields::Named(n) => {
                let (fields, _types) = Self::fields_types(n);
                parse_quote! {
                    #with_ext { #(#fields: self.#fields.with_extension(#ex.clone())),* }
                }
            }
            Fields::Unnamed(u) => {
                let (indices, _types) = Self::indices_types(u);
                parse_quote! {
                    #with_ext( #(self.#indices.with_extension(#ex.clone())),* )
                }
            }
            Fields::Unit => parse_quote! {
                ::coalesced::WithExt {
                    value: self,
                    extension: #ex
                }
            },
        }
    }
    fn implement_struct_extension_unwrap_extension(&self, f: &Fields, we: &Ident) -> Expr {
        match f {
            Fields::Named(n) => {
                let (fields, _types) = Self::fields_types(n);
                parse_quote! {
                    Self { #(#fields: ::coalesced::Extension::unwrap_extension(#we.#fields)),* }
                }
            }
            Fields::Unnamed(u) => {
                let (indices, _types) = Self::indices_types(u);
                parse_quote! {
                    Self( #(::coalesced::Extension::unwrap_extension(#we.#indices)),* )
                }
            }
            Fields::Unit => parse_quote! {
                #we.value
            },
        }
    }
    fn implement_struct_extension_ex_prior(&self, f: &Fields) -> ItemFn {
        match f {
            Fields::Named(_) | Fields::Unnamed(_) => parse_quote! {
                fn ex_prior(base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
                    base.prior(other)
                }
            },
            Fields::Unit => parse_quote! {
                fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
                    other
                }
            },
        }
    }
    fn implement_struct_extension_ex_posterior(&self, f: &Fields) -> ItemFn {
        match f {
            Fields::Named(_) | Fields::Unnamed(_) => parse_quote! {
                fn ex_posterior(base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
                    base.posterior(other)
                }
            },
            Fields::Unit => parse_quote! {
                fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
                    base
                }
            },
        }
    }
    fn definition_struct_with_ext(&self, s: &DataStruct) -> Option<ItemStruct> {
        let DeriveInput { vis, .. } = &self.input;
        let (_, _, g_where) = self.split_with_extension_generics();
        let x_param = self.x_param();
        let with_ext = self.with_ext_path();
        match &s.fields {
            Fields::Named(n) => {
                let (fields, types) = Self::fields_types(n);
                Some(parse_quote! {
                    #[doc(hidden)]
                    #vis struct #with_ext #g_where {
                        #(#fields: ::coalesced::WithExt<#types, #x_param>),*
                    }
                })
            }
            Fields::Unnamed(u) => {
                let (_indices, types) = Self::indices_types(u);
                Some(parse_quote! {
                    #[doc(hidden)]
                    #vis struct #with_ext (
                        #(::coalesced::WithExt<#types, #x_param>),*
                    ) #g_where;
                })
            }
            Fields::Unit => None,
        }
    }
    fn implement_struct_coalesce_with_ext(&self, s: &DataStruct) -> Option<ItemImpl> {
        let (g_impl, _, g_where) = self.split_with_extension_generics();
        let with_ext = self.with_ext_path();

        match &s.fields {
            Fields::Named(n) => {
                let (fields, _types) = Self::fields_types(n);
                Some(parse_quote! {
                    impl #g_impl ::coalesced::Coalesce for #with_ext #g_where {
                        fn prior(self, other: Self) -> Self {
                            Self {
                                #(#fields: self.#fields.prior(other.#fields)),*
                            }
                        }
                        fn posterior(self, other: Self) -> Self {
                            Self {
                                #(#fields: self.#fields.posterior(other.#fields)),*
                            }
                        }
                    }
                })
            }
            Fields::Unnamed(u) => {
                let (indices, _types) = Self::indices_types(u);
                Some(parse_quote! {
                    impl #g_impl ::coalesced::Coalesce for #with_ext #g_where {
                        fn prior(self, other: Self) -> Self {
                            Self( #(self.#indices.prior(other.#indices)),* )
                        }
                        fn posterior(self, other: Self) -> Self {
                            Self( #(self.#indices.posterior(other.#indices)),* )
                        }
                    }
                })
            }
            Fields::Unit => None,
        }
    }
    fn implement_struct_from_with_ext(&self) -> ItemImpl {
        let DeriveInput { ident, .. } = &self.input;
        let (g_impl, g_type, g_where) = self.split_with_extension_generics();
        let with_ext = self.with_ext_path();
        parse_quote! {
            impl #g_impl From<#with_ext> for #ident #g_type #g_where {
                fn from(with_ext: #with_ext) -> Self {
                    ::coalesced::Extension::unwrap_extension(with_ext)
                }
            }
        }
    }

    fn implement_enum(&self, e: &DataEnum) -> TokenStream {
        let extension = self.implement_enum_extension(e);
        let with_ext_def = self.definition_enum_with_ext(e);
        let coalesce_with_ext = self.implement_enum_coalesce_with_ext(e);
        let from_with_ext = self.implement_enum_from_with_ext();
        parse_quote! {
            #extension
            #with_ext_def
            #coalesce_with_ext
            #from_with_ext
        }
    }
    fn implement_enum_extension(&self, e: &DataEnum) -> ItemImpl {
        let DeriveInput { ident, .. } = &self.input;
        let (g_impl, g_type, g_where) = self.split_with_extension_generics();
        let x_param = self.x_param();

        let with_ext = self.with_ext_path();
        let (ex, we) = (parse_quote! { extension }, parse_quote! { with_ext });
        let (arms_with_extension, arms_unwrap_extension) = (
            self.implement_enum_extension_with_extension(e.variants.iter(), &ex),
            self.implement_enum_extension_unwrap_extension(e.variants.iter(), &we),
        );
        parse_quote! {
            impl #g_impl ::coalesced::Extension<#x_param> for #ident #g_type #g_where {
                type WithExt = #with_ext;
                fn with_extension(self, #ex: #x_param) -> Self::WithExt {
                    match self {
                        #(#arms_with_extension),*
                    }
                }
                fn unwrap_extension(#we: Self::WithExt) -> Self {
                    match #we {
                        #(#arms_unwrap_extension),*
                    }
                }
                fn ex_prior(base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
                    base.prior(other)
                }
                fn ex_posterior(base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
                    base.posterior(other)
                }
            }
        }
    }
    fn implement_enum_extension_with_extension<'a>(
        &'a self,
        v: impl 'a + IntoIterator<Item = &'a Variant>,
        ex: &'a Ident,
    ) -> impl 'a + Iterator<Item = Arm> {
        let enum_ident = &self.input.ident;
        let with_ext = Self::strip_path_argument(&self.with_ext_path());
        v.into_iter()
            .map(move |Variant { ident, fields, .. }| match fields {
                Fields::Named(n) => {
                    let (fields, _types) = Self::fields_types(n);
                    parse_quote! {
                        #enum_ident::#ident { #(#fields),* } => #with_ext::#ident {
                            #(#fields: #fields.with_extension(#ex.clone())),*
                        }
                    }
                }
                Fields::Unnamed(u) => {
                    let prefixed_indices = Self::prefixed_indices(u, "base");
                    parse_quote! {
                        #enum_ident::#ident ( #(#prefixed_indices),* ) => #with_ext::#ident (
                            #(#prefixed_indices.with_extension(#ex.clone())),*
                        )
                    }
                }
                Fields::Unit => parse_quote! {
                    #enum_ident::#ident => #with_ext::#ident(().with_extension(#ex))
                },
            })
    }
    fn implement_enum_extension_unwrap_extension<'a>(
        &'a self,
        v: impl 'a + IntoIterator<Item = &'a Variant>,
        _we: &'a Ident,
    ) -> impl 'a + Iterator<Item = Arm> {
        let enum_ident = &self.input.ident;
        let with_ext = Self::strip_path_argument(&self.with_ext_path());
        v.into_iter()
            .map(move |Variant { ident, fields, .. }| match fields {
                Fields::Named(n) => {
                    let (fields, _types) = Self::fields_types(n);
                    parse_quote! {
                        #with_ext::#ident { #(#fields),* } => #enum_ident::#ident {
                            #(#fields: ::coalesced::Extension::unwrap_extension(#fields)),*
                        }
                    }
                }
                Fields::Unnamed(u) => {
                    let prefixed_indices = Self::prefixed_indices(u, "base");
                    parse_quote! {
                        #with_ext::#ident ( #(#prefixed_indices),* ) => #enum_ident::#ident (
                            #(::coalesced::Extension::unwrap_extension(#prefixed_indices)),*
                        )
                    }
                }
                Fields::Unit => parse_quote! {
                    #with_ext::#ident(_) => #enum_ident::#ident
                },
            })
    }
    fn definition_enum_with_ext(&self, e: &DataEnum) -> ItemEnum {
        let DeriveInput { vis, .. } = &self.input;
        let (_, _, g_where) = self.split_with_extension_generics();
        let with_ext = self.with_ext_path();
        let variants = e
            .variants
            .iter()
            .map(|variant| self.definition_enum_with_ext_variant(variant));
        parse_quote! {
            #[doc(hidden)]
            #vis enum #with_ext #g_where {
                #(#variants),*
            }
        }
    }
    fn definition_enum_with_ext_variant(&self, v: &Variant) -> Variant {
        let Variant { ident, fields, .. } = v;
        let x_param = self.x_param();
        match fields {
            Fields::Named(n) => {
                let (fields, types) = Self::fields_types(n);
                parse_quote! {
                    #ident { #(#fields: ::coalesced::WithExt<#types, #x_param>),* }
                }
            }
            Fields::Unnamed(u) => {
                let (_indices, types) = Self::indices_types(u);
                parse_quote! {
                    #ident ( #(::coalesced::WithExt<#types, #x_param>),* )
                }
            }
            Fields::Unit => parse_quote! {
                #ident ( ::coalesced::WithExt<(), #x_param> )
            }, // TODO
        }
    }
    fn implement_enum_coalesce_with_ext(&self, e: &DataEnum) -> ItemImpl {
        let (g_impl, _, g_where) = self.split_with_extension_generics();
        let with_ext = self.with_ext_path();

        let (prior_arms, posterior_arms): (Vec<_>, Vec<_>) = e
            .variants
            .iter()
            .map(|variant| self.implement_enum_coalesce_with_ext_arm(variant))
            .unzip();
        parse_quote! {
            impl #g_impl ::coalesced::Coalesce for #with_ext #g_where {
                fn prior(self, other: Self) -> Self {
                    match (self, other) {
                        #(#prior_arms),*
                    }
                }
                fn posterior(self, other: Self) -> Self {
                    match (self, other) {
                        #(#posterior_arms),*
                    }
                }
            }
        }
    }
    fn implement_enum_coalesce_with_ext_arm(&self, v: &Variant) -> (Arm, Arm) {
        let Variant { ident, fields, .. } = v;
        match fields {
            Fields::Named(n) => {
                let (fields, _types) = Self::fields_types(n);
                let (base_fields, other_fields) = (
                    Self::prefixed_fields(n, "base"),
                    Self::prefixed_fields(n, "other"),
                );
                (
                    parse_quote! {
                        (
                            Self::#ident { #(#fields: #base_fields),* },
                            Self::#ident { #(#fields: #other_fields),* },
                        ) => Self::#ident {
                            #(#fields: #base_fields.prior(#other_fields)),*
                        }
                    },
                    parse_quote! {
                        (
                            Self::#ident { #(#fields: #base_fields),* },
                            Self::#ident { #(#fields: #other_fields),* },
                        ) => Self::#ident {
                            #(#fields: #base_fields.posterior(#other_fields)),*
                        }
                    },
                )
            }
            Fields::Unnamed(u) => {
                let (base_indices, other_indices) = (
                    Self::prefixed_indices(u, "base"),
                    Self::prefixed_indices(u, "other"),
                );
                (
                    parse_quote! {
                        (
                            Self::#ident ( #(#base_indices),* ),
                            Self::#ident ( #(#other_indices),* ),
                        ) => Self::#ident (
                            #(#base_indices.prior(#other_indices)),*
                        )
                    },
                    parse_quote! {
                        (
                            Self::#ident ( #(#base_indices),* ),
                            Self::#ident ( #(#other_indices),* ),
                        ) => Self::#ident (
                            #(#base_indices.posterior(#other_indices)),*
                        )
                    },
                )
            }
            Fields::Unit => (parse_quote! { (_, o) => o }, parse_quote! { (b, _) => b }),
        }
    }
    fn implement_enum_from_with_ext(&self) -> ItemImpl {
        let DeriveInput { ident, .. } = &self.input;
        let (g_impl, g_type, g_where) = self.split_with_extension_generics();
        let with_ext = self.with_ext_path();
        parse_quote! {
            impl #g_impl From<#with_ext> for #ident #g_type #g_where {
                fn from(with_ext: #with_ext) -> Self {
                    ::coalesced::Extension::unwrap_extension(with_ext)
                }
            }
        }
    }

    fn fields_types<'a>(f: &'a FieldsNamed) -> (Vec<&'a Option<Ident>>, Vec<&'a Type>) {
        f.named.iter().map(|f| (&f.ident, &f.ty)).unzip()
    }
    fn indices_types<'a>(f: &'a FieldsUnnamed) -> (Vec<syn::Index>, Vec<&'a Type>) {
        f.unnamed
            .iter()
            .enumerate()
            .map(|(i, f)| (i.into(), &f.ty))
            .unzip()
    }
    fn prefixed_fields(f: &FieldsNamed, prefix: &str) -> Vec<Option<Ident>> {
        f.named
            .iter()
            .map(|f| f.ident.as_ref().map(|i| format_ident!("{}_{}", prefix, i)))
            .collect()
    }
    fn prefixed_indices(f: &FieldsUnnamed, prefix: &str) -> Vec<Ident> {
        f.unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| format_ident!("{}_{}", prefix, i))
            .collect()
    }
}

struct ExImplGenerics<'a>(&'a Implementor);
impl ToTokens for ExImplGenerics<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let x = self.0.extension_generic();
        let mut generics = self.0.input.generics.clone();
        generics.params.push(x);
        let (g_impl, _, _) = generics.split_for_impl();
        g_impl.to_tokens(tokens);
    }
}

struct ExTypeGenerics<'a>(&'a Implementor);
impl ToTokens for ExTypeGenerics<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let x = self.0.extension_generic();
        let mut generics = self.0.input.generics.clone();
        generics.params.push(x);
        let (_, g_type, _) = generics.split_for_impl();
        g_type.to_tokens(tokens);
    }
}
