pub mod r#struct;
pub mod r#enum;

pub use r#struct::*;
pub use r#enum::*;

use proc_macro2::{Ident, Literal, TokenStream};

pub enum IrItem {
    Mod(IrMod),
    NamedStruct(IrNamedStruct),
    UnnamedStruct(IrUnnamedStruct),
}

pub trait ModExt {
    fn get_ir() -> IrNamedStruct;
}

pub struct IrMod {
    pub name: Ident,
    pub items: Vec<IrItem>,
}