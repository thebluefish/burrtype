use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Expr, Lit, LitStr, Meta, parse_quote, Token};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use burrtype_internal::ir::CaseConvention;

pub mod container;
pub mod variant;
pub mod field;

pub fn docs(attrs: &[Attribute]) -> TokenStream {
    let mut docs = Vec::new();
    for attr in attrs {
        if let Meta::NameValue(attr) = &attr.meta {
            if attr.path.is_ident("doc") {
                if let Expr::Lit(lit) = &attr.value {
                    if let Lit::Str(ls) = &lit.lit {
                        docs.push(ls.value().trim().to_string());
                    }
                }
            }
        }
    }
    #[cfg(feature = "docs")]
    if docs.is_empty() {
        quote! {
            docs: None,
        }
    }
    else {
        let docs = docs.join("\n");

        quote! {
            docs: Some( #docs ),
        }
    }

    #[cfg(not(feature = "docs"))]
    quote!()
}


/// `serde_compat` attribute:
/// #[serde(rename = "T")]
/// #[serde(rename(serialize = "T", deserialize = T")] only for identical Ts
/// Controls the name of the output type
pub fn serde_rename(attrs: &[Attribute], default: &Ident) -> TokenStream {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            match attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
                Ok(items) => {
                    for meta in items {
                        if meta.path().is_ident("rename") {
                            match meta {
                                Meta::NameValue(meta) => {
                                    // pull the T from "T"
                                    let value = &meta.value;
                                    let name: LitStr = parse_quote!(#value);
                                    let name: Ident = Ident::new(&name.value(), meta.span());
                                    return quote!(#name);
                                }
                                Meta::List(meta) => {
                                    let mut ser = None::<LitStr>;
                                    let mut des = None::<LitStr>;
                                    match meta.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
                                        Ok(inner) => {
                                            for im in inner {
                                                match im {
                                                    Meta::NameValue(meta) if meta.path.is_ident("serialize") => {
                                                        let value = &meta.value;
                                                        ser = Some(parse_quote!(#value));
                                                    }
                                                    Meta::NameValue(meta) if meta.path.is_ident("deserialize") => {
                                                        let value = &meta.value;
                                                        des = Some(parse_quote!(#value));
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                        Err(e) => panic!("{}", e.into_compile_error())
                                    }

                                    match (ser, des) {
                                        (Some(ser), Some(des)) if ser == des => {
                                            let name: Ident = Ident::new(&ser.value(), meta.span());
                                            return quote!(#name);
                                        }
                                        (None, None) => {}
                                        _ => panic!("`#[serde(rename(serialize = \"T\", deserialize=\"T\")]` is currently unsupported\nconsider `#[serde(rename = \"T\")]` instead")
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Err(e) => panic!("{}", e.into_compile_error())
            }
        }
    }

    quote!(#default)
}

/// `serde_compat` attribute:
/// #[serde(rename_all = "case")]
/// #[serde(rename_all(serialize = "case", deserialize = "case"))] only if case is identical
/// Controls the case convention for fields or variants
pub fn serde_rename_all(attrs: &[Attribute]) -> CaseConvention {
    let mut ser = None::<CaseConvention>;
    let mut des = None::<CaseConvention>;

    for attr in attrs {
        if attr.path().is_ident("serde") {
            match attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
                Ok(items) => {
                    for meta in items {
                        if meta.path().is_ident("rename_all") {
                            match meta {
                                Meta::NameValue(meta) => {
                                    // pull the T from "T"
                                    let value = &meta.value;
                                    let name: LitStr = parse_quote!(#value);
                                    return CaseConvention::parse(&name.value());
                                }
                                Meta::List(meta) => {
                                    match meta.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
                                        Ok(inner) => {
                                            for im in inner {
                                                match im {
                                                    Meta::NameValue(meta) if meta.path.is_ident("serialize") => {
                                                        let value = &meta.value;
                                                        let name: LitStr = parse_quote!(#value);
                                                        ser = Some(CaseConvention::parse(&name.value()));
                                                    }
                                                    Meta::NameValue(meta) if meta.path.is_ident("deserialize") => {
                                                        let value = &meta.value;
                                                        let name: LitStr = parse_quote!(#value);
                                                        des = Some(CaseConvention::parse(&name.value()));
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                        Err(e) => panic!("{}", e.into_compile_error())
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Err(e) => panic!("{}", e.into_compile_error())
            }
        }
    }

    match (ser, des) {
        (Some(ser), Some(des)) if ser == des => {
            ser
        }
        (None, None) => CaseConvention::Default,
        _ => panic!("`#[serde(rename_all(serialize = \"T\", deserialize=\"T\")]` is currently unsupported\nconsider `#[serde(rename_all = \"T\")]` instead")
    }
}