use std::any::TypeId;
use std::collections::HashSet;
use bevy_reflect::{TypeInfo, TypeRegistration, VariantInfo};

pub trait TypeRegistrationExt {
    fn get_fields(&self) -> HashSet<TypeId>;
}

impl TypeRegistrationExt for TypeRegistration {
    fn get_fields(&self) -> HashSet<TypeId> {
        let mut fields = HashSet::new();
        match self.type_info() {
            TypeInfo::Struct(info) => {
                for n in 0..info.field_len() {
                    fields.insert(info.field_at(n).unwrap().type_id());
                }
            }
            TypeInfo::TupleStruct(info) => {
                for n in 0..info.field_len() {
                    fields.insert(info.field_at(n).unwrap().type_id());
                }
            }
            TypeInfo::Tuple(info) => {
                for n in 0..info.field_len() {
                    fields.insert(info.field_at(n).unwrap().type_id());
                }
            }
            TypeInfo::List(info) => {
                fields.insert(info.item_type_id());
            }
            TypeInfo::Array(info) => {
                fields.insert(info.item_type_id());
            }
            TypeInfo::Map(info) => {
                fields.insert(info.key_type_id());
                fields.insert(info.value_type_id());
            }
            TypeInfo::Enum(info) => {
                for variant in info.iter() {
                    match variant {
                        VariantInfo::Struct(inner) => {
                            for n in 0..inner.field_len() {
                                fields.insert(inner.field_at(n).unwrap().type_id());
                            }
                        }
                        VariantInfo::Tuple(inner) => {
                            for n in 0..inner.field_len() {
                                fields.insert(inner.field_at(n).unwrap().type_id());
                            }
                        }
                        VariantInfo::Unit(_) => {}
                    }
                }
            }
            TypeInfo::Value(info) => {
                fields.insert(info.type_id());
            }
        }
        fields
    }
}