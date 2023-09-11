use burrtype::prelude::*;
use proc_macro::TokenStream as ProcTokenStream;
use proc_macro2::{Ident, Literal, TokenStream, Span};
use quote::{quote, quote_spanned, ToTokens};
use std::collections::HashMap;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Lit, LitByteStr, LitStr, Meta, MetaNameValue, Token, TypePath, punctuated::Punctuated, spanned::Spanned, Item, ItemMod, Expr, ExprLit, ItemStruct, Type};
use inflector::*;
use syn::token::Typeof;

#[derive(Clone, Debug)]
pub enum BurrModItem {
    Mod {
        name: Ident,
    },
    Struct {
        name: Ident,
    },
    Enum {
        name: Ident,
    },
}

#[derive(Clone, Debug)]
pub struct BurrModIr {
    pub ident: Ident,
    pub items: Vec<Item>,
}

impl From<&ItemMod> for BurrModIr {
    fn from(value: &ItemMod) -> Self {
        BurrModIr {
            ident: value.ident.clone(),
            items: vec![],
        }
    }
}

// /// Attempts to parse an item to our IR
// /// This item may or may not be a valid
// pub fn parse_item(item: Item, ir: &mut IrMod) -> Option<IrItem> {
//     match item {
//         Item::Mod(ref inner) => {
//             let mut is_burrmod = false;
//             let mut flatten = false;
//             let mut ignore = false;
//             let mut ir_name_override: Option<Ident> = None;
//             let mut name_override: Option<Ident> = None;
//
//             for attr in &inner.attrs {
//                 if attr.has_ident("burrmod") {
//                     is_burrmod = true;
//
//                     match attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
//                         Ok(items) => {
//                             for meta in items {
//                                 match meta {
//                                     Meta::Path(path) if path.is_ident("flatten") => {
//                                         flatten = true;
//                                     }
//                                     Meta::Path(path) if path.is_ident("ignore") => {
//                                         ignore = true;
//                                     }
//                                     Meta::NameValue(MetaNameValue { path, value, .. }) if path.is_ident("ir") => {
//                                         let ident = match &value {
//                                             Expr::Lit(ExprLit { lit: Lit::Str(lit), ..}) => Ident::new(&lit.value(), value.span()),
//                                             Expr::Path(path) => {
//                                                 assert_eq!(path.path.segments.len(), 1);
//                                                 path.path.segments.last().unwrap().ident.clone()
//                                             }
//                                             value => return Err(syn::Error::new(value.span(), "invalid ir name").to_compile_error())
//                                         };
//                                         ir_name_override = Some(ident);
//
//                                     }
//                                     Meta::NameValue(MetaNameValue { path, value, .. }) if path.is_ident("rename") => {
//                                         let name = match &value {
//                                             Expr::Lit(ExprLit { lit: Lit::Str(lit), ..}) => Ident::new(&lit.value(), value.span()),
//                                             value => return Err(syn::Error::new(value.span(), "invalid name").to_compile_error())
//                                         };
//                                         name_override = Some(name);
//                                     }
//                                     _ => {}
//                                 }
//                             }
//                         }
//                         Err(err) => return Err(err.into_compile_error()),
//                     }
//                 }
//             }
//
//             if is_burrmod && !ignore {
//                 // ir.items.push(item);
//             }
//         }
//         Item::Struct(inner) => {
//             None
//         }
//         Item::Enum(inner) => {
//             None
//         }
//         item => {
//             println!("skipping {:?}", item.get_ident());
//             None
//         },
//     }
// }
//

/// Attempts to parse an item from syn IR to our IR
// pub fn parse_item(item: Item) -> Result<IrItem, TokenStream>  {
//     match item {
//         Item::Mod(inner) => {
//             parse_mod(inner).map(|ir| IrItem::from(ir))
//         }
//         Item::Struct(inner) => {
//             let item = match &inner.fields {
//                 Fields::Named(fields) => {
//                     let mut ir_fields = Vec::with_capacity(fields.named.len());
//                     for field in &fields.named {
//                         if let Type::Path(ty) = &field.ty {
//                             ir_fields.push(IrNamedField {
//                                 name: field.ident.clone().unwrap(),
//                                 ty: IrType {
//                                     id: std::any::TypeId::of::<syn::parse_quote!(ty)>(),
//                                     path: ty.clone(),
//                                 },
//                             });
//                         }
//                         else {
//                             return Err(syn::Error::new(inner.span(), "invalid item type").to_compile_error())
//                         }
//                     }
//
//                     IrNamedStruct {
//                         name: inner.ident.clone(),
//                         fields: ir_fields,
//                     }.into()
//                 }
//                 Fields::Unnamed(fields) => {
//                     let mut ir_fields = Vec::with_capacity(fields.unnamed.len());
//                     for field in &fields.unnamed {
//                         if let Type::Path(ty) = &field.ty {
//                             ir_fields.push(ty.clone());
//                         }
//                         else {
//                             return Err(syn::Error::new(inner.span(), "invalid item type").to_compile_error())
//                         }
//                     }
//                     IrUnnamedStruct {
//                         name: inner.ident.clone(),
//                         fields: ir_fields,
//                     }.into()
//                 }
//                 Fields::Unit => {
//                     IrUnitStruct {
//                         name: inner.ident.clone(),
//                     }.into()
//                 }
//             };
//             Ok(item)
//         }
//         // Item::Enum(inner) => {
//         // }
//         item => return Err(syn::Error::new(item.span(), "invalid item").to_compile_error())
//     }
// }

pub fn parse_mod(item: ItemMod) -> Result<IrMod, TokenStream> {
    let mut is_burrmod = false;
    let mut flatten = false;
    let mut inline = false;
    let mut ignore = false;
    let mut ir_name_override: Option<Ident> = None;
    let mut name_override: Option<Ident> = None;

    for attr in &item.attrs {
        println!("checking {}: {}\n{}", item.ident, attr.is_ident("burrmod"), attr.get_ident().unwrap());
        if attr.is_ident("burrmod") {
            is_burrmod = true;

            match attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
                Ok(items) => {
                    for meta in items {
                        match meta {
                            Meta::Path(path) if path.is_ident("flatten") => {
                                flatten = true;
                            }
                            Meta::Path(path) if path.is_ident("inline") => {
                                inline = true;
                            }
                            Meta::Path(path) if path.is_ident("ignore") => {
                                ignore = true;
                            }
                            Meta::Path(path) => {
                                println!("path: {path:?}");
                            }
                            Meta::NameValue(MetaNameValue { path, value, .. }) if path.is_ident("ir") => {
                                let ident = match &value {
                                    Expr::Lit(ExprLit { lit: Lit::Str(lit), ..}) => Ident::new(&lit.value(), value.span()),
                                    Expr::Path(path) => {
                                        assert_eq!(path.path.segments.len(), 1);
                                        path.path.segments.last().unwrap().ident.clone()
                                    }
                                    value => return Err(syn::Error::new(value.span(), "invalid ir name").to_compile_error())
                                };
                                ir_name_override = Some(ident);

                            }
                            Meta::NameValue(MetaNameValue { path, value, .. }) if path.is_ident("rename") => {
                                let name = match &value {
                                    Expr::Lit(ExprLit { lit: Lit::Str(lit), ..}) => Ident::new(&lit.value(), value.span()),
                                    value => return Err(syn::Error::new(value.span(), "invalid name").to_compile_error())
                                };
                                name_override = Some(name);
                            }
                            _ => {}
                        }
                    }
                }
                Err(err) => return Err(err.into_compile_error()),
            }
        }
    }

    if !is_burrmod {
        println!("denying {}", item.ident);
        return Err(syn::Error::new(item.span(), "not tagged with #[burrmod]").to_compile_error())
    }

    let name = if let Some(name) = name_override {
        name
    }
    else {
        item.ident
    };

    let ir_name = if let Some(name) = ir_name_override {
        name
    }
    else {
        Ident::new(&format!("{}Mod", name.to_string().to_pascal_case()), Span::call_site())
    };

    Ok(IrMod {
        name: name.clone(),
        ir_name,
        flatten,
        inline,
        items: vec![],
    })
}

pub fn process_burr_item(item: Item, ir: &mut BurrModIr) -> Result<(), TokenStream> {
    match item {
        Item::Mod(ref inner) => {
            let mut is_burrmod = false;
            let mut flatten = false;
            let mut ignore = false;

            for attr in &inner.attrs {
                if attr.is_ident("burrmod") {
                    is_burrmod = true;

                    match attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
                        Ok(items) => {
                            for meta in items {
                                match meta {
                                    Meta::Path(path) if path.is_ident("flatten") => {
                                        flatten = true;
                                    }
                                    Meta::Path(path) if path.is_ident("ignore") => {
                                        ignore = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Err(err) => return Err(err.into_compile_error()),
                    }
                }
            }

            if is_burrmod && !ignore {
                ir.items.push(item);
            }
        }
        item => {
            ir.items.push(item);
        },
    }

    Ok(())
}

/// Returns true if the item has an attribute with the given ident, ignoring its path if any
pub fn item_has_attr<I: ?Sized>(item: &Item, ident: &I) -> bool
    where
        Ident: PartialEq<I>,
{
    if let Some(attrs) = item.clone().get_attrs() {
        for attr in attrs {
            if let Some(segment) = attr.path().segments.last() {
                if &segment.ident == ident {
                    return true
                }
            }
        }
    }

    false
}

