use crate::gen::BurrMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{quote_spanned, ToTokens};
use syn::{Expr, Field, GenericArgument, MetaList, MetaNameValue, parse_quote, Path, PathArguments, Token, Type};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

pub struct FlaggedField {
    /// prevents this field from appearing in export
    pub ignore: bool,
    /// substitute type's members
    pub flatten: bool,
    /// overrides type
    pub ty: Option<Type>,
    /// original data
    pub field: Field,
}

/// Tries to parse a `T` out of `Option<T>` types
/// `Option<T>` => `(T, true)`
/// `T` => `(T, false)`
pub fn option(ty: &Type) -> (&Type, bool) {
    if let Type::Path(path) = ty {
        // We don't care about the path to `Option`, only that the root type is `Option`
        if let Some(last) = path.path.segments.last() {
            if last.ident == "Option" {
                // `Option<T>` has exactly one parameter, and it's a Type
                if let PathArguments::AngleBracketed(args) = &last.arguments {
                    if let GenericArgument::Type(ty) = args.args.first().expect("Option should contain exactly one argument") {
                        return (ty, true)
                    }
                }
            }
        }
    }

    (ty, false)
}



/// Named fields can have the following attributes:
/// #[burr(flatten)]
/// #[burr(ignore)]
/// #[burr(type = T)]
pub fn named_field_attrs(field: &Field) -> Result<FlaggedField, TokenStream> {
    let mut ignore = false;
    let mut flatten = false;
    let mut ty = None;

    // parse attributes
    for attr in &field.attrs {
        #[cfg(feature = "serde_compat")]
        if attr.path().is_ident("serde") {
            match attr.parse_args_with(Punctuated::<BurrMeta, Token![,]>::parse_terminated) {
                Ok(items) => {
                    for meta in items {
                        match meta {
                            BurrMeta::Path(path) if path.is_ident("skip") => {
                                ignore = true;
                            }
                            BurrMeta::Path(path) if path.is_ident("flatten") => {
                                flatten = true;
                            }
                            _ => {}
                        }
                    }
                }
                Err(err) => return Err(err.into_compile_error()),
            }
        }
        if attr.path().is_ident("burr") {
            match attr.parse_args_with(Punctuated::<BurrMeta, Token![,]>::parse_terminated) {
                Ok(items) => {
                    for meta in items {
                        match meta {
                            BurrMeta::Path(path) if path.is_ident("ignore") => {
                                ignore = true;
                            }
                            BurrMeta::Path(path) if path.is_ident("flatten") => {
                                flatten = true;
                            }
                            BurrMeta::KeywordValue(meta) if meta.path == "type" => {
                                let value = &meta.value;
                                ty = Some(parse_quote!(#value));
                            }
                            _ => {
                                return Err(
                                    quote_spanned! {meta.span() => compile_error!("unknown attribute"); },
                                )
                            }
                        }
                    }
                }
                Err(err) => return Err(err.into_compile_error()),
            }
        }
    }

    Ok(FlaggedField {
        ignore,
        flatten,
        ty,
        field: field.clone(),
    })
}

/// Unnamed fields can have the following attributes:
/// #[burr(ignore)]
/// #[burr(type = T)]
pub fn unnamed_field_attrs(field: &Field) -> Result<FlaggedField, TokenStream> {
    let mut ignore = false;
    let mut ty = None;

    // parse attributes
    for attr in &field.attrs {
        #[cfg(feature = "serde_compat")]
        if attr.path().is_ident("serde") {
            match attr.parse_args_with(Punctuated::<BurrMeta, Token![,]>::parse_terminated) {
                Ok(items) => {
                    for meta in items {
                        match meta {
                            BurrMeta::Path(path) if path.is_ident("skip") => {
                                ignore = true;
                            }
                            _ => {}
                        }
                    }
                }
                Err(err) => return Err(err.into_compile_error()),
            }
        }
        if attr.path().is_ident("burr") {
            match attr.parse_args_with(Punctuated::<BurrMeta, Token![,]>::parse_terminated) {
                Ok(items) => {
                    for meta in items {
                        match meta {
                            BurrMeta::Path(path) if path.is_ident("ignore") => {
                                ignore = true;
                            }
                            BurrMeta::KeywordValue(meta) if meta.path == "type" => {
                                let value = &meta.value;
                                ty = Some(parse_quote!(#value));
                            }
                            _ => {
                                return Err(
                                    quote_spanned! {meta.span() => compile_error!("unknown attribute"); },
                                )
                            }
                        }
                    }
                }
                Err(err) => return Err(err.into_compile_error()),
            }
        }
    }

    Ok(FlaggedField {
        ignore,
        flatten: false,
        ty,
        field: field.clone(),
    })
}