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

/// Like syn::Meta with an extra variant to support keywords in `name = value` attributes
pub enum Meta {
    Path(Path),
    List(MetaList),
    NameValue(MetaNameValue),
    KeywordValue(TypeValue),
}

impl ToTokens for Meta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Meta::Path(inner) => inner.to_tokens(tokens),
            Meta::List(inner) => inner.to_tokens(tokens),
            Meta::NameValue(inner) => inner.to_tokens(tokens),
            Meta::KeywordValue(inner) => inner.to_tokens(tokens),
        }
    }
}

impl Parse for Meta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(parsed) = input.parse::<Path>() {
            Ok(Meta::Path(parsed))
        }
        else if let Ok(parsed) = input.parse::<MetaList>() {
            Ok(Meta::List(parsed))
        }
        else if let Ok(parsed) = input.parse::<MetaNameValue>() {
            Ok(Meta::NameValue(parsed))
        }
        else {
            input.parse().map(Meta::KeywordValue)
        }
    }
}

pub struct TypeValue {
    pub path: Ident,
    pub eq_token: Token![=],
    pub value: Expr,
}

impl ToTokens for TypeValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

impl Parse for TypeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(TypeValue {
            path: Ident::parse_any(input)?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
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
        if attr.path().is_ident("burr") {
            match attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
                Ok(items) => {
                    for meta in items {
                        match meta {
                            Meta::Path(path) if path.is_ident("ignore") => {
                                ignore = true;
                            }
                            Meta::Path(path) if path.is_ident("flatten") => {
                                flatten = true;
                            }
                            Meta::KeywordValue(meta) if meta.path == "type" => {
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
        if attr.path().is_ident("burr") {
            match attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
                Ok(items) => {
                    for meta in items {
                        match meta {
                            Meta::Path(path) if path.is_ident("ignore") => {
                                ignore = true;
                            }
                            Meta::KeywordValue(meta) if meta.path == "type" => {
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