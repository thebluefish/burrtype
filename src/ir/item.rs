use super::{IrMod, IrNamedStruct, IrUnnamedStruct, IrUnitStruct};
use proc_macro2::TokenStream;
use syn::Ident;
use quote::ToTokens;

#[derive(Clone, Debug)]
pub enum IrItem {
    Mod(IrMod),
    NamedStruct(IrNamedStruct),
    UnnamedStruct(IrUnnamedStruct),
    UnitStruct(IrUnitStruct),
}

impl IrItem {
    pub fn name(&self) -> &Ident {
        match self {
            IrItem::Mod(inner) => &inner.name,
            IrItem::NamedStruct(inner) => &inner.name,
            IrItem::UnnamedStruct(inner) => &inner.name,
            IrItem::UnitStruct(inner) => &inner.name,
        }
    }
}

impl ToTokens for IrItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
    }
}

impl From<IrMod> for IrItem {
    fn from(value: IrMod) -> Self {
        IrItem::Mod(value)
    }
}

impl From<IrNamedStruct> for IrItem {
    fn from(value: IrNamedStruct) -> Self {
        IrItem::NamedStruct(value)
    }
}

impl From<IrUnnamedStruct> for IrItem {
    fn from(value: IrUnnamedStruct) -> Self {
        IrItem::UnnamedStruct(value)
    }
}

impl From<IrUnitStruct> for IrItem {
    fn from(value: IrUnitStruct) -> Self {
        IrItem::UnitStruct(value)
    }
}