use proc_macro2::TokenStream;
use syn::{Attribute, Token};
use syn::punctuated::Punctuated;
use crate::gen::BurrMeta;

pub fn serde_skip(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            match attr.parse_args_with(Punctuated::<BurrMeta, Token![,]>::parse_terminated) {
                Ok(items) => {
                    for meta in items {
                        match meta {
                            BurrMeta::Path(path) if path.is_ident("skip") => {
                                return true
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    false
}
