use std::any::TypeId;
use std::collections::HashSet;
use super::{IrNamedStruct, IrTupleStruct, IrUnitStruct};
use syn::Ident;
use crate::ir::IrEnum;

#[derive(Clone, Debug)]
pub enum IrItem {
    NamedStruct(IrNamedStruct),
    TupleStruct(IrTupleStruct),
    UnitStruct(IrUnitStruct),
    Enum(IrEnum),
}

impl IrItem {
    pub fn ident(&self) -> &Ident {
        match self {
            IrItem::NamedStruct(inner) => &inner.ident,
            IrItem::TupleStruct(inner) => &inner.ident,
            IrItem::UnitStruct(inner) => &inner.ident,
            IrItem::Enum(inner) => &inner.ident,
        }
    }

    pub fn name(&self) -> String {
        self.ident().to_string()
    }

    pub fn type_id(&self) -> TypeId {
        match self {
            IrItem::NamedStruct(inner) => inner.id,
            IrItem::TupleStruct(inner) => inner.id,
            IrItem::UnitStruct(inner) => inner.id,
            IrItem::Enum(inner) => inner.id,
        }
    }

    /// Gets a flat set of all types being used by an item
    pub fn all_field_types(&self) -> HashSet<TypeId> {
        match self {
            IrItem::NamedStruct(inner) => {
                HashSet::from_iter(inner.fields.iter().map(|field| field.ty.id))
            },
            IrItem::TupleStruct(inner) => {
                HashSet::from_iter(inner.fields.iter().map(|field| field.ty.id))
            },
            IrItem::Enum(inner) => {
                let mut types = HashSet::new();
                for var in &inner.variants {
                    types.extend(var.all_field_types());
                }
                types
            },
            _ => HashSet::default(),
        }
    }
}

impl From<IrNamedStruct> for IrItem {
    fn from(value: IrNamedStruct) -> Self {
        IrItem::NamedStruct(value)
    }
}

impl From<IrTupleStruct> for IrItem {
    fn from(value: IrTupleStruct) -> Self {
        IrItem::TupleStruct(value)
    }
}

impl From<IrUnitStruct> for IrItem {
    fn from(value: IrUnitStruct) -> Self {
        IrItem::UnitStruct(value)
    }
}

impl From<IrEnum> for IrItem {
    fn from(value: IrEnum) -> Self {
        IrItem::Enum(value)
    }
}