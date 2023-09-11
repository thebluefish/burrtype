use super::{BurrFile, BurrMod};
use crate::ir::{IrExt, IrItem, IrNamedStruct, IrUnitStruct, IrTupleStruct};

#[derive(Clone, Debug)]
pub enum Item {
    File(BurrFile),
    Mod(BurrMod),
    NamedStruct(IrNamedStruct),
    TupleStruct(IrTupleStruct),
    UnitStruct(IrUnitStruct),
}

/// TODO: rename IrItem to Item and replace all calls to it with ir::Item, ditto for other IrT structs
impl From<IrItem> for Item {
    fn from(value: IrItem) -> Self {
        match value {
            IrItem::Mod(inner) => {
                Item::Mod(BurrMod {
                    name: inner.name.to_string(),
                    items: inner.items.iter().map(Clone::clone).map(Into::into).collect(),
                })
            }
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