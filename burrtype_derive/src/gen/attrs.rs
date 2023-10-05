use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, Lit, Meta};

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
            docs: Some(#docs),
        }
    }

    #[cfg(not(feature = "docs"))]
    quote!()
}
