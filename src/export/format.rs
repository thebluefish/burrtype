use super::{BurrMod, Item};
use crate::ir::{IrNamedStruct, IrTupleStruct, IrUnitStruct};
use std::io::Write;
use crate::export::BurrFile;
use inflector::Inflector;

/// Formats items
pub trait Burrmatter {
    /// Formats any kind of item
    fn format_item(&mut self, item: &Item) -> String {
        match item {
            Item::File(inner) => self.format_file(inner),
            Item::Mod(inner) => self.format_mod(inner),
            Item::NamedStruct(inner) => self.format_named_struct(inner),
            Item::TupleStruct(inner) => self.format_tuple_struct(inner),
            Item::UnitStruct(inner) => self.format_unit_struct(inner),
        }
    }

    fn format_file(&mut self, item: &BurrFile) -> String;
    fn format_mod(&mut self, item: &BurrMod) -> String;
    fn format_named_struct(&mut self, item: &IrNamedStruct) -> String;
    fn format_tuple_struct(&mut self, item: &IrTupleStruct) -> String;
    fn format_unit_struct(&mut self, item: &IrUnitStruct) -> String;

}

// /// A formatter with options to cover most general cases
// pub struct Burrmatter {
//     pub pretty_print: bool,
// }
//
// impl Burrmatter {
//     pub fn new() -> Self {
//         Burrmatter {
//             pretty_print: true,
//         }
//     }
//
//     pub fn minify(mut self) -> Self {
//         self.pretty_print = false;
//         self
//     }
// }
//
// impl Formatter for Burrmatter {
//     fn format_file(&mut self, item: &BurrFile) -> String {
//         let mut out = String::new();
//         for (i, item) in item.items.iter().enumerate() {
//             if i > 0 {
//                 out.push_str("\n\n");
//             }
//             out.push_str(&self.format_item(item));
//             out.push_str("\n");
//         }
//
//         out
//     }
//
//     fn format_mod(&mut self, item: &BurrMod) -> String {
//         let mut out = String::new();
//
//         println!("formatting {}: {:#?}", item.name, item.items);
//
//         out.push_str(&format!("namespace {} {{\n", item.name.to_pascal_case()));
//         for (i, item) in item.items.iter().enumerate() {
//             if i > 0 {
//                 out.push_str("\n\n");
//             }
//             out.push_str(&self.format_item(item));
//             out.push_str("\n");
//         }
//         out.push_str("}");
//
//         out
//     }
//
//     fn format_named_struct(&mut self, item: &IrNamedStruct) -> String {
//         let mut out = String::new();
//
//         out.push_str(&format!("export interface {} {{}}", item.name.to_string().to_pascal_case()));
//
//         out
//     }
//
//     fn format_tuple_struct(&mut self, item: &IrTupleStruct) -> String {
//         let mut out = String::new();
//
//         out.push_str(&format!("export interface {} {{}}", item.name.to_string().to_pascal_case()));
//
//         out
//     }
//
//     fn format_unit_struct(&mut self, item: &IrUnitStruct) -> String {
//         let mut out = String::new();
//
//         out.push_str(&format!("export interface {} {{}}", item.name.to_string().to_pascal_case()));
//
//         out
//     }
// }