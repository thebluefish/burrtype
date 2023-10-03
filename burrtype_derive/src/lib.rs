mod gen;

use proc_macro::{TokenStream as ProcTokenStream};
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Meta, Expr, Lit};
use inflector::*;

#[proc_macro_derive(Burr, attributes(burr))]
pub fn burr_macro(input: ProcTokenStream) -> ProcTokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let DeriveInput { attrs, ident, data, .. } = input;

    let ir_ar = gen::auto_registration_fn(ident.clone());

    let ir_impl = match data {
        Data::Struct(inner) => {
            match inner.fields {
                Fields::Named(inner) => gen::named_struct_ir(attrs, ident, inner),
                Fields::Unnamed(inner) => gen::tuple_struct_ir(attrs, ident, inner),
                Fields::Unit => gen::unit_struct_ir(attrs, ident),
            }
        }
        Data::Enum(inner) => gen::enum_ir(attrs, ident, inner),
        Data::Union(_) => panic!("unions are unsupported"),
    };

    quote!(#ir_impl #ir_ar).into()
}

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