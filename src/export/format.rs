use super::{BurrMod, Item};
use crate::ir::{IrNamedStruct, IrTupleStruct, IrUnitStruct};
use std::io::Write;
use crate::export::BurrFile;

/// Formats items
pub trait Formatter {
    /// Formats any kind of item
    fn format_item(&mut self, item: Item) -> String {
        match item {
            Item::File(inner) => self.format_file(inner),
            Item::Mod(inner) => self.format_mod(inner),
            Item::NamedStruct(inner) => self.format_named_struct(inner),
            Item::TupleStruct(inner) => self.format_tuple_struct(inner),
            Item::UnitStruct(inner) => self.format_unit_struct(inner),
        }
    }

    fn format_file(&mut self, item: BurrFile) -> String;
    fn format_mod(&mut self, item: BurrMod) -> String;
    fn format_named_struct(&mut self, item: IrNamedStruct) -> String;
    fn format_tuple_struct(&mut self, item: IrTupleStruct) -> String;
    fn format_unit_struct(&mut self, item: IrUnitStruct) -> String;

}

/// A formatter with options to cover most general cases
pub struct Burrmatter {
    pub pretty_print: bool,
}

impl Burrmatter {
    pub fn new() -> Self {
        Burrmatter {
            pretty_print: true,
        }
    }

    pub fn minify(mut self) -> Self {
        self.pretty_print = false;
        self
    }
}

impl Formatter for Burrmatter {
    fn format_file(&mut self, item: BurrFile) -> String {
        todo!()
    }

    fn format_mod(&mut self, item: BurrMod) -> String {
        todo!()
    }

    fn format_named_struct(&mut self, item: IrNamedStruct) -> String {
        todo!()
    }

    fn format_tuple_struct(&mut self, item: IrTupleStruct) -> String {
        todo!()
    }

    fn format_unit_struct(&mut self, item: IrUnitStruct) -> String {
        todo!()
    }
}