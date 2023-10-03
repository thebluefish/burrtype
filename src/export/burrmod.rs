use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use burrtype_internal::ir::{IrEnumVariant, IrItem};
use burrtype_internal::prelude::IrExt;

/// A collection of items to export
#[derive(Clone, Debug)]
pub struct BurrMod {
    pub name: String,
    pub types: HashMap<TypeId, IrItem>,
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
        for (_, item) in &self.types {
            match item {
                IrItem::UnitStruct(_) => {}
                IrItem::NamedStruct(ir) => fields.extend(ir.fields.iter().map(|f| f.ty.id)),
                IrItem::TupleStruct(ir) => fields.extend(ir.fields.iter().map(|ty| ty.ty.id)),
                IrItem::Enum(ir) => {
                    for var in &ir.variants {
                        match var {
                            IrEnumVariant::Struct(ir) => fields.extend(ir.fields.iter().map(|f| f.ty.id)),
                            IrEnumVariant::Tuple(ir) => fields.extend(ir.fields.iter().map(|f| f.ty.id)),
                            _ => {}
                        }
                    }
                }
            }
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

    pub fn with_type<T: IrExt>(mut self) -> Self {
        let item = T::get_ir();
        self.types.insert(item.type_id(), item);
        self
    }

    pub fn with_mod<M: Into<BurrMod>>(mut self, r#mod: M) -> Self {
        self.children.push(r#mod.into());
        self
    }
}