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

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_type<IR: IrExt>(mut self) -> Self {
        self.items.push(IR::get_ir().into());
        self
    }

    pub fn with_item<IR: Into<Item>>(mut self, item: IR) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn extend<IR: Into<Item>>(mut self, other: IR) -> Self {
        match other.into() {
            Item::File(inner) => self.items.extend(inner.items),
            Item::Mod(inner) => self.items.extend(inner.items),
            _ => panic!("attempting to extend unsupported item"),
        }
        self
    }
}

pub trait BurrModExt: ModExt {
    fn to_mod() -> BurrMod;
}

impl<T> BurrModExt for T where T: ModExt {
    fn to_mod() -> BurrMod {
        BurrMod {
            name: T::name().to_string(),
            items: T::items().into_iter().map(Into::into).collect(),
        }
    }
}

impl From<BurrMod> for Item {
    fn from(value: BurrMod) -> Self {
        Item::Mod(value)
    }
}