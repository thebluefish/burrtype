mod r#enum;
mod item;
mod r#struct;

pub use r#enum::*;
pub use item::*;
pub use r#struct::*;

use proc_macro2::{Ident, Literal, Span, TokenStream};
use syn::{Attribute, Data, Item, ItemMod, spanned::Spanned};

pub trait IrExt {
    fn get_ir() -> IrItem;
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