use super::Item;
use crate::ir::{IrNamedStruct, IrTupleStruct, IrUnitStruct};

/// Formats items
pub trait Burrmatter {
    /// Formats any kind of item
    fn format_item(&mut self, item: &Item) -> String {
        match item {
            Item::NamedStruct(inner) => self.format_named_struct(inner),
            Item::TupleStruct(inner) => self.format_tuple_struct(inner),
            Item::UnitStruct(inner) => self.format_unit_struct(inner),
        }
    }

    fn format_named_struct(&mut self, item: &IrNamedStruct) -> String;
    fn format_tuple_struct(&mut self, item: &IrTupleStruct) -> String;
    fn format_unit_struct(&mut self, item: &IrUnitStruct) -> String;

}