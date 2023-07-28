mod burrtribute;

use burrtribute::*;
use burrtype::prelude::*;
use proc_macro::{TokenStream as ProcTokenStream};
use proc_macro2::{Span, Ident, Literal, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use std::collections::HashMap;
use std::process::id;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Lit, LitByteStr, LitStr, Meta, MetaNameValue, Token, TypePath, Item, ItemMod, punctuated::Punctuated, spanned::Spanned, parse::{Parse, ParseStream}, Attribute, Expr, ExprLit, parse_quote};
use syn::parse::Parser;
use syn::token::Const;
use inflector::*;

/// Implements #[derive(Burr)]
#[proc_macro_derive(Burr, attributes(burr))]
pub fn burr_macro(input: ProcTokenStream) -> ProcTokenStream {
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
                                        quote! {fields.extend(<#ty as burrtype::ir::NamedStructExt>::fields());}
                                    }
                                    else {
                                        quote! {fields.push(burrtype::ir::IrNamedField {
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
                        let mut fields = Vec::<burrtype::ir::IrNamedField>::new();
                        #( #a )*
                        fields
                    };

                    let impl_frag = quote! {
                        impl burrtype::ir::NamedStructExt for #name {
                            fn fields() -> Vec<burrtype::ir::IrNamedField> {
                                #field_map_frag
                            }
                        }
                    };

                    let irext_impl: Item = parse_quote!(
                        impl burrtype::ir::IrExt for #name {
                            fn get_ir() -> burrtype::ir::IrItem {
                                burrtype::ir::IrUnitStruct {
                                    name: syn::parse_quote!(#name)
                                }.into()
                            }
                        }
                    );
                    println!("ir:\n{}", impl_frag);

                    return quote!(#impl_frag #irext_impl).into()
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
                        impl burrtype::ir::IrExt for #name {
                            fn get_ir() -> burrtype::ir::IrItem {
                                burrtype::ir::IrUnitStruct {
                                    name: syn::parse_quote!(#name)
                                }.into()
                            }
                        }
                    }.into();
                }
                Fields::Unit => {
                    println!("struct {name:?};");
                    return quote! {
                        impl burrtype::ir::IrExt for #name {
                            fn get_ir() -> burrtype::ir::IrItem {
                                burrtype::ir::IrUnitStruct {
                                    name: syn::parse_quote!(#name)
                                }.into()
                            }
                        }
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

/// #[burrmod] amd its variants should produce a `struct <Name>Mod` that exposes information about this module and its items
/// #[burrmod(ir = "T")] will instead produce a `struct T`
/// #[burrmod(flatten)] should produce its inner items without the module declaration
/// #[burrmod(inline)] should produce its `BurrMod` and related impls inside the module
#[proc_macro_attribute]
pub fn burrmod(args: ProcTokenStream, input: ProcTokenStream) -> ProcTokenStream {
    let args = TokenStream::from(args);
    let ir_input = TokenStream::from(input.clone());

    // slap attribute back on to be included with syn ir
    let item_with_attr: ProcTokenStream = quote_spanned!(Span::call_site()=>
        #[burrmod(#args)]
        #ir_input
    ).into();

    let mut item = parse_macro_input!(input as ItemMod);
    let ir_item = parse_macro_input!(item_with_attr as ItemMod);

    let mut ir = match parse_mod(ir_item.clone()) {
        Ok(ir) => ir,
        Err(err) => return err.into()
    };

    // process content into ir representation
    let (_, items) = ir_item.content.expect("unsupported opaque module");
    for item in items {
        let name = item.get_ident().unwrap().clone();
        if let Ok(item) = parse_item(item) {
            ir.items.push(item);
        }
        else {
            println!("skipping {name}");
        }
    }

    let IrMod { name, ir_name, flatten, inline, items} = ir;

    // process inner ir into ir output
    let item_tokens = items.iter().map(|item| {
        match item {
            IrItem::Mod(inner) => {
                let IrMod { name: mod_name, ir_name, flatten, inline: mod_inline, .. } = inner;
                // Resolve path from child to impl
                let mut ir_path = quote!(#ir_name);
                if *mod_inline {
                    ir_path = quote!(#mod_name :: #ir_path);
                }
                if !inline {
                    ir_path = quote!(#name :: #ir_path);
                }

                println!("path {ir_path}");

                if *flatten {
                    quote! { items.extend(<#ir_path as burrtype::ir::ModExt>::items()); }
                }
                else {
                    quote! { items.push(<#ir_path as burrtype::ir::IrExt>::get_ir()); }
                }
            }
            IrItem::NamedStruct(inner) => {
                let IrNamedStruct { name: struct_name, .. } = inner;
                let ir_path = if inline { quote!(#struct_name) } else { quote!(#name :: #struct_name) };
                quote! { items.push(<#ir_path as burrtype::ir::IrExt>::get_ir()); }
            }
            IrItem::UnnamedStruct(inner) => {
                let IrUnnamedStruct { name: struct_name, .. } = inner;
                let ir_path = if inline { quote!(#struct_name) } else { quote!(#name :: #struct_name) };
                quote! { items.push(<#ir_path as burrtype::ir::IrExt>::get_ir()); }
            }
            IrItem::UnitStruct(inner) => {
                let IrUnitStruct { name: struct_name } = inner;
                let ir_path = if inline { quote!(#struct_name) } else { quote!(#name :: #struct_name) };
                quote! { items.push(<#ir_path as burrtype::ir::IrExt>::get_ir()); }
            }
        }
    }).collect::<Vec<_>>();
    let out_struct: Item = parse_quote!(pub struct #ir_name;);
    let out_modext_impl: Item = parse_quote!(
        impl burrtype::ir::ModExt for #ir_name {
            fn name() -> syn::Ident {
                syn::parse_quote!(#name)
            }
            fn flatten() -> bool {
                #flatten
            }
            fn items() -> Vec<burrtype::ir::IrItem> {
                let mut items = vec![];
                #(#item_tokens)*
                items
            }
        }
    );
    let out_irext_impl: Item = parse_quote!(
        impl burrtype::ir::IrExt for #ir_name {
            fn get_ir() -> burrtype::ir::IrItem {
                burrtype::ir::IrUnitStruct {
                    name: syn::parse_quote!(#name)
                }.into()
            }
        }
    );
    if inline {
        let (_, content) = item.content.as_mut().expect("unsupported opaque module");
        content.insert(0, out_struct);
        content.insert(1, out_modext_impl);
        content.insert(2, out_irext_impl);
        item.to_token_stream().into()
    }
    else {
        quote_spanned!(Span::call_site()=> #item #out_struct #out_modext_impl #out_irext_impl).into()
    }
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