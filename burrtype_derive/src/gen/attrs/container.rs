use std::borrow::Cow;
use crate::gen::BurrMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, LitStr, Meta, parse_quote, Token};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use burrtype_internal::ir::CaseConvention;

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
