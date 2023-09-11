use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::ir::{IrExt, IrItem, IrType, ModExt};
use crate::prelude::IrMod;
use inflector::Inflector;
use proc_macro2::Ident;
use crate::export::BurrMod;
use super::item::Item;

/// A collection of items to export + metadata
#[derive(Clone, Debug)]
pub struct BurrFile {
    pub items: Vec<Item>,
    // path to file for export, with optional extension
    pub target: PathBuf,
    // describes what types are being exported by this file
    // todo: figure out of inline modules need to be handled specially for exports too
    pub exports: HashMap<TypeId, Ident>,
}

impl BurrFile {
    pub fn new(target: PathBuf) -> Self {
        BurrFile {
            items: Vec::new(),
            target,
            exports: HashMap::new(),
        }
    }

    /// Sets the target this file will be exported to
    pub fn target(&mut self, to: PathBuf) {
        self.target = to;
    }

    pub fn with_item<IR: Into<Item>>(mut self, item: IR) -> Self {
        self.items.push(item.into());
        self
    }
}

impl From<BurrFile> for Item {
    fn from(value: BurrFile) -> Self {
        Item::File(value)
    }
}

impl From<BurrFile> for BurrMod {
    fn from(value: BurrFile) -> Self {
        BurrMod {
            name: value.target.with_extension("").to_string_lossy().to_string(),
            items: value.items,
        }
    }
}

impl From<BurrMod> for BurrFile {
    fn from(value: BurrMod) -> Self {
        let mut exports = HashMap::new();
        for item in &value.items {
            match item {
                // todo: figure out if this is how we want to deal with nested files
                // if so, figure out how to properly reference exports between different directory levels
                Item::File(inner) => {}
                // todo: figure out of inline modules need to be handled specially for exports too
                Item::Mod(inner) => {}
                // todo: handle cases where an item already exists, at least logging it
                Item::NamedStruct(inner) => {
                    exports.insert(inner.id, inner.name.clone());
                },
                Item::TupleStruct(inner) => {
                    exports.insert(inner.id, inner.name.clone());
                },
                Item::UnitStruct(inner) => {
                    exports.insert(inner.id, inner.name.clone());
                },
            }
        }
        BurrFile {
            items: value.items,
            target: PathBuf::from(value.name).with_extension("ts"),
            exports,
        }
    }
}