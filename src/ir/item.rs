use super::{IrMod, IrNamedStruct, IrTupleStruct, IrUnitStruct};
use proc_macro2::TokenStream;
use syn::Ident;
use quote::ToTokens;

#[derive(Clone, Debug)]
pub enum IrItem {
    NamedStruct(IrNamedStruct),
    TupleStruct(IrTupleStruct),
    UnitStruct(IrUnitStruct),
}

impl IrItem {
    pub fn name(&self) -> &Ident {
        match self {
            IrItem::NamedStruct(inner) => &inner.name,
            IrItem::TupleStruct(inner) => &inner.name,
            IrItem::UnitStruct(inner) => &inner.name,
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