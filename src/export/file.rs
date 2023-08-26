use std::path::PathBuf;
use crate::ir::{IrExt, IrItem, ModExt};
use crate::prelude::IrMod;
use inflector::Inflector;
use crate::export::BurrMod;
use super::item::Item;

/// A collection of items to export
/// Optionally configured for direct export
#[derive(Clone, Debug)]
pub struct BurrFile {
    pub(crate) target: PathBuf,
    pub(crate) items: Vec<Item>,
}

impl BurrFile {
    pub fn new(target: PathBuf) -> Self {
        BurrFile {
            target,
            items: Vec::new(),
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
        BurrFile {
            target: PathBuf::from(value.name).with_extension("ts"),
            items: value.items,
        }
    }
}