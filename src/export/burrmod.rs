use bevy_reflect::*;
use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use crate::type_registration::TypeRegistrationExt;

/// A collection of items to export
#[derive(Clone, Debug)]
pub struct BurrMod {
    pub name: String,
    pub types: HashMap<TypeId, TypeRegistration>,
    pub children: Vec<BurrMod>,
}

impl BurrMod {
    pub fn new<S: Into<String>>(target: S) -> Self {
        BurrMod {
            name: target.into(),
            types: HashMap::new(),
            children: Vec::new(),
        }
    }

    /// Gets a flat set of all types being used by a module
    pub fn pull_fields(&self) -> HashSet<TypeId> {
        let mut fields = HashSet::new();
        // iterate fields for each type and add field's TypeId to set
        for (_, tr) in &self.types {
            fields.extend(tr.get_fields());
        }
        // then repeat this recursively
        for child in &self.children {
            fields.extend(child.pull_fields());
        }
        fields
    }

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_type<T: GetTypeRegistration>(mut self) -> Self {
        let registration = T::get_type_registration();
        self.types.insert(registration.type_id(), registration);
        self
    }

    pub fn with_mod<M: Into<BurrMod>>(mut self, r#mod: M) -> Self {
        self.children.push(r#mod.into());
        self
    }
}