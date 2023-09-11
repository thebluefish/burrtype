use crate::ir::{IrExt, IrItem, ModExt};
use crate::prelude::IrMod;
use inflector::Inflector;
use super::item::Item;

/// A collection of items to export
/// Optionally configured for direct export
#[derive(Clone, Debug)]
pub struct BurrMod {
    pub name: String,
    pub items: Vec<Item>,
}

impl BurrMod {
    pub fn new<S: Into<String>>(target: S) -> Self {
        BurrMod {
            name: target.into(),
            items: Vec::new(),
        }
    }

    /// Sets the target this module will be exported to
    pub fn target(&mut self, to: String) {
        self.name = to;
    }

    pub fn with_item<IR: Into<Item>>(mut self, item: IR) -> Self {
        self.items.push(item.into());
        self
    }
}

impl From<BurrMod> for Item {
    fn from(value: BurrMod) -> Self {
        Item::Mod(value)
    }
}