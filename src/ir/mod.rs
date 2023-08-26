use proc_macro2::{Ident, Literal, Span, TokenStream};
use syn::{Attribute, Data, Item, ItemMod, spanned::Spanned};
use super::IrItem;

pub trait IrExt {
    fn get_ir() -> IrItem;
}

impl<IR: IrExt> From<IR> for IrItem {
    fn from(value: IR) -> Self {
        IR::get_ir()
    }
}

pub trait ModExt {
    fn name() -> Ident;
    fn flatten() -> bool;
    fn items() -> Vec<IrItem>;
}

#[derive(Clone, Debug)]
pub struct IrMod {
    pub name: Ident,
    pub ir_name: Ident,
    pub flatten: bool,
    pub inline: bool,
    pub items: Vec<IrItem>,
}