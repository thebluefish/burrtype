use crate::gen::BurrMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, LitStr, Meta, parse_quote, Token};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

/// #[burr(mod = T)]
pub fn mod_override(attrs: &[Attribute]) -> TokenStream {
    for attr in attrs {
        if attr.path().is_ident("burr") {
            match attr.parse_args_with(Punctuated::<BurrMeta, Token![,]>::parse_terminated) {
                Ok(items) => {
                    for meta in items {
                        match meta {
                            BurrMeta::KeywordValue(meta) if meta.path == "mod" => {
                                let value = &meta.value;
                                let ls: LitStr = parse_quote!(#value);
                                return quote!(Some(#ls));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
                // Err(e) => panic!("{}", e.into_compile_error())
            }
        }
    }

    quote!(None)
}

/// `serde_compat` attribute:
/// #[serde(rename = "T")]
pub fn name_override(attrs: &[Attribute], default: &Ident) -> TokenStream {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            println!("serde {}", attr.to_token_stream());
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
                                    println!("rename list");
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

        //
        // #[cfg(feature = "serde_compat")]
        // if attr.path().is_ident("serde") {
        //     match &attr.meta {
        //         Meta::NameValue(meta) if meta.path.is_ident("rename") => {
        //             println!("NameValue: {:?}", meta.path.to_token_stream());
        //             // pull the T from "T"
        //             let value = &meta.value;
        //             let name: LitStr = parse_quote!(#value);
        //             let name: Ident = Ident::new(&name.value(), meta.span());
        //             return quote!(#name);
        //         }
        //         Meta::List(meta) => {
        //             println!("getting em");
        //             if let Err(err) = meta.parse_nested_meta(|meta| {
        //                 if meta.path.is_ident("rename") {
        //                     println!("gottem");
        //                 }
        //                 Ok(())
        //             }) {
        //                 // panic!("{}", err.into_compile_error())
        //             }
        //             // match meta.parse_args_with(Punctuated::<BurrMeta, Token![,]>::parse_terminated) {
        //             //     Ok(items) => {
        //             //         let mut ser = None::<LitStr>;
        //             //         let mut des = None::<LitStr>;
        //             //         for meta in items {
        //             //             match meta {
        //             //                 BurrMeta::KeywordValue(meta) => {
        //             //                     println!("m {}", meta.path);
        //             //                 }
        //             //                 _ => {}
        //             //             }
        //             //         }
        //             //         match (ser, des) {
        //             //             (Some(ser), Some(des)) if ser == des => {
        //             //                 let name: Ident = Ident::new(&ser.value(), meta.span());
        //             //                 return quote!(#name);
        //             //             }
        //             //             (None, None) => {}
        //             //             _ => panic!("`#[serde(rename(serialize = \"T\", deserialize=\"T\")]` is currently unsupported\nconsider `#[serde(rename = \"T\")]` instead")
        //             //         }
        //             //     }
        //             //     Err(e) => panic!("{}", e.into_compile_error())
        //             // }
        //         }
        //         _ => {}
        //     }
        // }
    }

    quote!(#default)
}
