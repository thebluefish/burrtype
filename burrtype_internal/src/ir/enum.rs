use super::{ IrNamedField, IrUnnamedField};
use std::any::TypeId;
use std::collections::HashSet;
use proc_macro2::Ident;

#[derive(Clone, Copy, Debug)]
pub enum EnumRepr {
    External,
    Untagged,
    Internal(&'static str),
    Adjacent {
        tag: &'static str,
        content: &'static str,
    },
}

#[derive(Clone, Debug)]
pub struct IrEnum {
    pub ident: Ident,
    pub id: TypeId,
    pub variants: Vec<IrEnumVariant>,
    pub repr: EnumRepr,
    #[cfg(feature = "docs")]
    pub docs: Option<&'static str>,
    pub r#mod: Option<&'static str>,
}

impl IrEnum {
    pub fn name(&self) -> String {
        self.ident.to_string()
    }
}

#[derive(Clone, Debug)]
pub enum IrEnumVariant {
    Struct(IrEnumStructVariant),
    Tuple(IrEnumTupleVariant),
    Unit(IrEnumUnitVariant),
}

impl IrEnumVariant {
    /// Gets a flat set of all types being used by a variant
    pub fn all_field_types(&self) -> HashSet<TypeId> {
        match self {
            IrEnumVariant::Struct(inner) => {
                HashSet::from_iter(inner.fields.iter().map(|field| field.ty.id))
            },
            IrEnumVariant::Tuple(inner) => {
                HashSet::from_iter(inner.fields.iter().map(|field| field.ty.id))
            },
            _ => HashSet::default(),
        }
    }

    pub fn ident(&self) -> &Ident {
        match self {
            IrEnumVariant::Struct(inner) => &inner.ident,
            IrEnumVariant::Tuple(inner) => &inner.ident,
            IrEnumVariant::Unit(inner) => &inner.ident,
        }
    }

    pub fn name(&self) -> String {
        self.ident().to_string()
    }
}

/// A variant with the format:
/// ```
/// T {
///     name: type,
///     ...
/// },
/// ```
#[derive(Clone, Debug)]
pub struct IrEnumStructVariant {
    pub ident: Ident,
    pub fields: Vec<IrNamedField>,
    #[cfg(feature = "docs")]
    pub docs: Option<&'static str>,
}

impl IrEnumStructVariant {
    pub fn name(&self) -> String {
        self.ident.to_string()
    }
}

impl From<IrEnumStructVariant> for IrEnumVariant {
    fn from(value: IrEnumStructVariant) -> Self {
        IrEnumVariant::Struct(value)
    }
}

/// A variant with the format:
/// ```
/// T (U, ...),
/// ```
#[derive(Clone, Debug)]
pub struct IrEnumTupleVariant {
    pub ident: Ident,
    pub fields: Vec<IrUnnamedField>,
    #[cfg(feature = "docs")]
    pub docs: Option<&'static str>,
}

impl IrEnumTupleVariant {
    pub fn name(&self) -> String {
        self.ident.to_string()
    }
}

impl From<IrEnumTupleVariant> for IrEnumVariant {
    fn from(value: IrEnumTupleVariant) -> Self {
        IrEnumVariant::Tuple(value)
    }
}

/// A variant with the format:
/// ```
/// T,
/// ```
#[derive(Clone, Debug)]
pub struct IrEnumUnitVariant {
    pub ident: Ident,
    #[cfg(feature = "docs")]
    pub docs: Option<&'static str>,
}

impl IrEnumUnitVariant {
    pub fn name(&self) -> String {
        self.ident.to_string()
    }
}

impl From<IrEnumUnitVariant> for IrEnumVariant {
    fn from(value: IrEnumUnitVariant) -> Self {
        IrEnumVariant::Unit(value)
    }
}