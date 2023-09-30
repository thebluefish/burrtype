use proc_macro::{TokenStream as ProcTokenStream};
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput};
use inflector::*;

/// Cursed stub necessary to re-export `linkme::distributed_slice`
/// Otherwise the user will need to include `linkme` in their crate's dependencies
#[proc_macro_attribute]
pub fn linkme(_args: ProcTokenStream, input: ProcTokenStream) -> ProcTokenStream {
    input
}

/// Auto-registers type with `burrtype::TYPES` using optional configuration
#[proc_macro_attribute]
pub fn burr(_args: ProcTokenStream, input: ProcTokenStream) -> ProcTokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let name = &item.ident;
    let fn_name = quote::format_ident!("burr_add_{}_type_registration", name.to_string().to_snake_case());

    let ir_auto_registration = quote! {
        #[burrtype::linkme::distributed_slice(burrtype::TYPES)]
        #[linkme(crate = burrtype::linkme)]
        #[doc(hidden)]
        fn #fn_name() -> burrtype::TypeRegistration {
            <#name as burrtype::GetTypeRegistration>::get_type_registration()
        }
    };

    quote_spanned!(Span::call_site()=> #item #ir_auto_registration).into()
}