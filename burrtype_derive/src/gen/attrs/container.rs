use crate::gen::BurrMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, LitStr, Meta, parse_quote, Token};
use syn::punctuated::Punctuated;

/// #[burr(mod = T)]
/// Controls the export module of the output type
pub fn burr_mod(attrs: &[Attribute]) -> TokenStream {
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

/// `serde_compat` attributes:
/// #[serde(tag = "type")]
/// #[serde(tag = "t", content = "c")]
/// #[serde(untagged)]
/// Controls the representation of an enum
pub fn serde_enum_repr(attrs: &[Attribute]) -> TokenStream {
    let mut tag = None::<LitStr>;
    let mut content = None::<LitStr>;

    for attr in attrs {
        if attr.path().is_ident("serde") {
            match attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
                Ok(items) => {
                    for meta in items {
                        match meta {
                            Meta::Path(meta) if meta.is_ident("untagged") => {
                                return quote!(EnumRepr :: Untagged);
                            }
                            Meta::NameValue(meta) if meta.path.is_ident("tag") => {
                                // pull the T from "T"
                                let value = &meta.value;
                                tag = Some(parse_quote!(#value));
                            }
                            Meta::NameValue(meta) if meta.path.is_ident("content") => {
                                // pull the T from "T"
                                let value = &meta.value;
                                content = Some(parse_quote!(#value));
                            }
                            _ => {}
                        }
                    }
                }
                Err(e) => panic!("{}", e.into_compile_error())
            }
        }
    }

    match (&tag, &content) {
        (Some(tag), Some(content)) => {
            quote!(EnumRepr :: Adjacent {
                tag: #tag,
                content: #content,
            })
        }
        (Some(tag), None) => {
            quote!(EnumRepr :: Internal ( #tag ))
        }
        (None, None) => {
            quote!(EnumRepr :: External)
        }
        _ => panic!("invalid #[serde(tag = \"{:?}\", content = \"{:?}\")]", &tag, &content),
    }

}