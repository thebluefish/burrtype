use std::any::TypeId;
use proc_macro2::{Ident, Literal, TokenStream};
use std::collections::HashMap;
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, Lit, LitByteStr, LitStr, Meta, Token,
    TypePath,
};

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
    pub fields: Vec<IrNamedField>,
}

/// A struct with the format:
/// ```
/// struct T (type, ...);
/// ```
#[derive(Clone, Debug)]
pub struct IrTupleStruct {
    pub name: Ident,
    pub fields: Vec<IrType>,
}

/// A struct with the format:
/// ```
/// struct T;
/// ```
#[derive(Clone, Debug)]
pub struct IrUnitStruct {
    pub name: Ident,
    // pub type_id: TypeId,
}