use burrtype::*;

use proc_macro::TokenStream as ProcTokenStream;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use std::collections::HashMap;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, Lit, LitByteStr, LitStr, Meta, Token,
    TypePath,
};

#[proc_macro_derive(Burr, attributes(burr))]
pub fn my_macro(input: ProcTokenStream) -> ProcTokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name = input.ident.clone();

    match input.data {
        Data::Struct(inner) => {
            match &inner.fields {
                Fields::Named(fields) => {
                    let a = fields.named.iter()
                        .map(|field| {
                            let name = field.ident.clone().unwrap();
                            let ty = &field.ty;
                            match parse_named_field_attrs(field) {
                                Ok(ir) => {
                                    let st = if ir.ignore {
                                        quote! { }
                                    }
                                    else if ir.flatten {
                                        quote! {fields.extend(<#ty as burrtype::NamedStructExt>::get_fields());}
                                    }
                                    else {
                                        quote! {fields.push(burrtype::IrNamedField {
                                            name: syn::parse_quote!(#name),
                                            ty: syn::parse_quote!(#ty),
                                        });}
                                    };
                                    return st
                                }
                                Err(err) => return err.into(),
                            }
                        })
                        .map(|field| {
                            println!("field => {field}");
                            field
                        })
                        .collect::<Vec<_>>();
                    let field_map_frag = quote! {
                        let mut fields = Vec::<burrtype::IrNamedField>::new();
                        #( #a )*
                        fields
                    };

                    let impl_frag = quote! {
                        impl NamedStructExt for #name {
                            // fn get_ir() -> IrNamedStruct {
                            // }
                            fn get_fields() -> Vec<burrtype::IrNamedField> {
                                #field_map_frag
                            }
                        }
                    };

                    println!("ir:\n{}", impl_frag);

                    return impl_frag.into()
                }
                Fields::Unnamed(fields) => {
                    println!(
                        "struct {name} ({})",
                        fields
                            .unnamed
                            .iter()
                            .map(|f| f.ty.to_token_stream().to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                    return quote! {
                    }.into();
                }
                Fields::Unit => {
                    println!("struct {name:?};");
                    return quote! {
                    }.into();
                }
            }
        }
        Data::Enum(inner) => {
            println!("enum {name:?}");
        }
        Data::Union(_) => unimplemented!(),
    }

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // ...
    };

    // Hand the output tokens back to the compiler
    ProcTokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn burr(args: ProcTokenStream, input: ProcTokenStream) -> ProcTokenStream {
    let _ = args;
    let _ = input;

    input
}

struct FlaggedField {
    /// prevents this field from appearing in export
    pub ignore: bool,
    /// substitute type's members
    pub flatten: bool,
    /// overrides type
    pub ty: Option<TypePath>,
    /// original data
    pub field: Field,
}

/// Named fields can have the following attributes:
/// #[burr(flatten)] or #[serde(flatten)]
/// #[burr(ignore)]
/// #[burr(ty(T)]
fn parse_named_field_attrs(field: &Field) -> Result<FlaggedField, TokenStream> {
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
                            Meta::List(meta) if meta.path.is_ident("ty") => {
                                if let Ok(path) = meta.parse_args::<TypePath>() {
                                    ty = Some(path);
                                } else {
                                    return Err(
                                        quote_spanned! {meta.tokens.span() => compile_error!("invalid args"); },
                                    );
                                }
                            }
                            _ => {
                                return Err(
                                    quote_spanned! {meta.span() => compile_error!("unkown attribute"); },
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
/// #[burr(ty(T)]
fn parse_unnamed_field_attrs(field: &Field) -> Result<FlaggedField, TokenStream> {
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
                            Meta::List(meta) if meta.path.is_ident("ty") => {
                                if let Ok(path) = meta.parse_args::<TypePath>() {
                                    ty = Some(path);
                                } else {
                                    return Err(
                                        quote_spanned! {meta.tokens.span() => compile_error!("invalid args"); },
                                    );
                                }
                            }
                            _ => {
                                return Err(
                                    quote_spanned! {meta.span() => compile_error!("unkown attribute"); },
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

//
// let recurse = ir_struct.fields.iter().map(|(name, field)| {
// let ty = &field.field.ty;
// quote_spanned! {field.field.span()=>
// <#ty as burrtype::Ts>::get_ir()
// }
// }).collect::<Vec<_>>();
// let sp = quote! {
//                         #( #recurse; )*
//                     };
// println!("ir: {}", sp);
