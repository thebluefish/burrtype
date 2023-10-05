use super::{CaseConvention, IrNamedField, IrUnnamedField};
use std::any::TypeId;
use proc_macro2::Ident;

/// A struct with the format:
/// ```
/// struct T {
///     name: type,
///     ...
/// }
/// ```
#[derive(Clone, Debug)]
pub struct IrNamedStruct {
    pub ident: Ident,
    pub id: TypeId,
    pub fields: Vec<IrNamedField>,
    #[cfg(feature = "docs")]
    pub docs: Option<&'static str>,
    pub r#mod: Option<&'static str>,
    #[cfg(feature = "serde_compat")]
    pub case: CaseConvention,
}

impl IrNamedStruct {
    pub fn name(&self) -> String {
        self.ident.to_string()
    }
}

/// A struct with the format:
/// ```
/// struct T (type, ...);
/// ```
#[derive(Clone, Debug)]
pub struct IrTupleStruct {
    pub ident: Ident,
    pub id: TypeId,
    pub fields: Vec<IrUnnamedField>,
    #[cfg(feature = "docs")]
    pub docs: Option<&'static str>,
    pub r#mod: Option<&'static str>,
}

impl IrTupleStruct {
    pub fn name(&self) -> String {
        self.ident.to_string()
    }
}

/// A struct with the format:
/// ```
/// struct T;
/// ```
#[derive(Clone, Debug)]
pub struct IrUnitStruct {
    pub ident: Ident,
    pub id: TypeId,
    #[cfg(feature = "docs")]
    pub docs: Option<&'static str>,
    pub r#mod: Option<&'static str>,
}

impl IrUnitStruct {
    pub fn name(&self) -> String {
        self.ident.to_string()
    }
}
