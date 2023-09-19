use std::any::TypeId;
use std::collections::HashMap;
#[cfg(feature = "bevy_reflect")]
use bevy_reflect::*;
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
    #[cfg(feature = "bevy_reflect")]
    pub registry: HashMap<TypeId, TypeRegistration>,
    pub children: Vec<BurrMod>,
}

impl BurrMod {
    pub fn new<S: Into<String>>(target: S) -> Self {
        BurrMod {
            name: target.into(),
            items: Vec::new(),
            #[cfg(feature = "bevy_reflect")]
            registry: HashMap::new(),
            children: Vec::new(),
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

    #[cfg(feature = "bevy_reflect")]
    pub fn with_reflection<T: GetTypeRegistration>(mut self) -> Self {
        let registration = T::get_type_registration();
        self.registry.insert(registration.type_id(), registration);
        self
    }

    pub fn with_item<IR: Into<Item>>(mut self, item: IR) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn with_mod<M: Into<BurrMod>>(mut self, r#mod: M) -> Self {
        self.children.push(r#mod.into());
        self
    }
}

// todo: fix this
// pub trait BurrModExt: ModExt {
//     fn to_mod() -> BurrMod;
// }
// 
// impl<T> BurrModExt for T where T: ModExt {
//     fn to_mod() -> BurrMod {
//         BurrMod {
//             name: T::name().to_string(),
//             items: T::items().into_iter().map(Into::into).collect(),
//         }
//     }
// }