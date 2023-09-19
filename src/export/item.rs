use std::any::TypeId;
use proc_macro2::Ident;
use crate::ir::{IrExt, IrItem, IrNamedStruct, IrUnitStruct, IrTupleStruct};

#[derive(Clone, Debug)]
pub enum Item {
    NamedStruct(IrNamedStruct),
    TupleStruct(IrTupleStruct),
    UnitStruct(IrUnitStruct),
}

impl Item {
    pub fn get_name(&self) -> &Ident {
        match self {
            Item::NamedStruct(inner) => &inner.name,
            Item::TupleStruct(inner) => &inner.name,
            Item::UnitStruct(inner) => &inner.name,
        }
    }

    pub fn get_id(&self) -> TypeId {
        match self {
            Item::NamedStruct(inner) => inner.id,
            Item::TupleStruct(inner) => inner.id,
            Item::UnitStruct(inner) => inner.id,
        }
    }
}

/// TODO: rename IrItem to Item and replace all calls to it with ir::Item, ditto for other IrT structs
impl From<IrItem> for Item {
    fn from(value: IrItem) -> Self {
        match value {
            IrItem::NamedStruct(inner) => Item::NamedStruct(inner),
            IrItem::TupleStruct(inner) => Item::TupleStruct(inner),
            IrItem::UnitStruct(inner) => Item::UnitStruct(inner),
        }
    }
}

impl<T> From<T> for Item where T: IrExt {
    fn from(_: T) -> Self {
        T::get_ir().into()
    }
}