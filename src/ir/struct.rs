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

pub trait UnnamedStructExt {
    fn fields() -> Vec<TypePath> {
        Vec::new()
    }
}

pub trait UnionStructExt {
}

/// A field of an `IrNamedStruct`
#[derive(Clone, Debug)]
pub struct IrNamedField {
    pub name: Ident,
    pub ty: TypePath,
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
pub struct IrUnnamedStruct {
    pub name: Ident,
    pub fields: Vec<TypePath>,
}

/// A struct with the format:
/// ```
/// struct T;
/// ```
#[derive(Clone, Debug)]
pub struct IrUnitStruct {
    pub name: Ident,
}