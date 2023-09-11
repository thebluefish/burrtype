use std::any::TypeId;
use proc_macro2::{Ident, Literal, TokenStream};
use std::collections::HashMap;
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, Lit, LitByteStr, LitStr, Meta, Token,
    TypePath,
};
use crate::syn::SynIdent;

pub trait NamedStructExt {
    fn fields() -> Vec<IrNamedField> {
        Vec::new()
    }
}

#[derive(Clone, Debug)]
pub struct IrType {
    pub path: TypePath,
    pub id: TypeId,
}

impl SynIdent for IrType {
    fn is_ident<I: ?Sized>(&self, ident: &I) -> bool
        where Ident: PartialEq<I>
    {
        if let Some(segment) = self.path.path.segments.last() {
            return &segment.ident == ident
        }
        false
    }

    fn get_ident(&self) ->  Option<&Ident> {
        if let Some(segment) = self.path.path.segments.last() {
            return Some(&segment.ident)
        }
        None
    }
}

pub trait TupleStructExt {
    fn fields() -> Vec<IrType> {
        Vec::new()
    }
}

pub trait UnionStructExt {
}

/// A field of an `IrNamedStruct`
#[derive(Clone, Debug)]
pub struct IrNamedField {
    pub name: Ident,
    pub ty: IrType,
}

/// A struct with the format:
/// ```
/// struct T {
///     name: type,
///     ...
/// }
/// ```
#[derive(Clone, Debug)]
pub struct IrNamedStruct {
    pub name: Ident,
    pub id: TypeId,
    pub fields: Vec<IrNamedField>,
}

/// A struct with the format:
/// ```
/// struct T (type, ...);
/// ```
#[derive(Clone, Debug)]
pub struct IrTupleStruct {
    pub name: Ident,
    pub id: TypeId,
    pub fields: Vec<IrType>,
}

/// A struct with the format:
/// ```
/// struct T;
/// ```
#[derive(Clone, Debug)]
pub struct IrUnitStruct {
    pub name: Ident,
    pub id: TypeId,
}