use super::{TsFile, TsFormatter, TypeScript};
use crate::export::{BurrMod, Burrxporter};
use bevy_reflect::prelude::*;
use bevy_reflect::{TypeInfo, TypeRegistration, VariantInfo};
use inflector::Inflector;
use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use path_macro::path;
use path_slash::*;

/// An export-friendly version of the Typescript export builder
/// Contains files being exported and computed metadata about files and their types
pub struct TsExporter<'t> {
    pub formatter: TsFormatter<'t>,
    /// maps file paths to files
    pub files: HashMap<PathBuf, TsFile>,
    /// type information for types being exported
    pub type_registry: HashMap<TypeId, TypeRegistration>,
    /// types being exported to file paths
    pub type_exports: HashMap<TypeId, PathBuf>,
    /// types being mapped to other types
    pub type_overrides: HashMap<TypeId, TypeId>,
    /// types being mapped to string
    pub type_strings: HashMap<TypeId, &'t str>,
}

impl<'t> TsExporter<'t> {
    pub fn export(mut self, exporter: &Burrxporter) {
        // Export files
        for file in self.files.values() {
            println!("to {}", file.target.to_string_lossy());
            let mut out = String::new();

            // build imports
            // these will be written first and should look like:
            // import * as {name} from {path}
            let mut field_types = HashSet::new();

            for item in &file.items {
                field_types.extend(get_fields(item));
            }

            for item in &file.mods {
                field_types.extend(pull_fields(item));
            }

            println!("types: {field_types:?}");

            // get all used imports by target
            let mut import_map: HashMap<PathBuf, HashSet<TypeId>> = HashMap::new();
            for id in &field_types {
                if let Some(target) = self.type_exports.get(id) {
                    import_map
                        .entry(target.clone())
                        .or_default()
                        .insert(id.clone());
                }
            }
            // remove self-references
            import_map.remove(&file.target);

            // todo: disambiguate any duplicate import names here
            // maybe build a HashSet of short_name()s, upon discovering a duplicate, start adding parts of a path to one until it's unique, etc..

            // iterate imports and write them
            for (import, types) in &import_map {
                println!("need to resolve {} from {} with {types:?}", import.to_slash_lossy(), file.target.to_slash_lossy());
                // resolve relative path from other file to this one
                let mut depth = 0;
                let mut parent = file.target.parent();
                let mut found = None;
                while let Some(path) = parent {
                    if let Ok(p) = import.strip_prefix(path) {
                        found = Some(p.to_path_buf());
                        println!("can import from {} at {depth}", p.to_slash_lossy());
                        break
                    }
                    depth += 1;
                    parent = path.parent();
                }
                let mut full_path = found.expect("Failed to find relative path for imports").with_extension("");
                if depth == 0 {
                    full_path = path!("." / full_path);
                }
                else {
                    for _ in 0..depth {
                        full_path = path!(".." / full_path);
                    }
                };
                // write import head
                out.push_str(&format!("import {{ "));
                // write import items
                for (i, ty) in types.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    let tr = self.type_registry.get(ty).expect("type should be known by now");
                    out.push_str(&format!("{}", tr.short_name()));
                }
                // write import tail
                out.push_str(&format!(" }} from '{}'\n", full_path.to_slash_lossy()));
            }

            // separate imports and exports, if any
            if !import_map.is_empty() {
                out.push_str("\n");
            }

            // write exports
            println!("exporting {} items", file.items.len());
            for (i, item) in file.items.iter().enumerate() {
                if i > 0 {
                    out.push_str("\n");
                }
                out.push_str(&self.format_type(item));
            }

            // export to file
            let to = match &exporter.root {
                Some(root) => path!(root / file.target),
                None => file.target.clone(),
            };
            if out.is_empty() {
                println!("skipping empty file: {}", to.to_slash_lossy());
            }
            else {
                println!("writing to: {}", to.to_slash_lossy());

                let mut writer = exporter.open_writer(&to).unwrap();
                writer.write(out.as_bytes()).unwrap();
            }
        }
    }

    fn format_file(&mut self, file: &TsFile) -> String {
        let mut out = String::new();

        for (i, item) in file.items.iter().enumerate() {
            if i > 0 {
                out.push_str("\n");
            }
            out.push_str(&self.format_type(item));
            out.push_str("\n");
        }

        out
    }

    fn format_mod(&mut self, item: &BurrMod) -> String {
        let mut out = String::new();

        // write mod header
        out.push_str(&format!("{}namespace {} {{\n", self.formatter.get_indentation(), item.name.to_pascal_case()));
        self.formatter.depth.fetch_add(1, Ordering::Relaxed);
        // write items

        for (i, (_, item)) in item.types.iter().enumerate() {
            if i > 0 {
                out.push_str("\n");
            }
            out.push_str(&self.format_type(item));
            out.push_str("\n");
        }
        // write mod footer
        self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
        out.push_str(&format!("{}}}", self.formatter.get_indentation()));

        out
    }

    fn format_type(&self, registration: &TypeRegistration) -> String {
        let mut out = String::new();
        match registration.type_info() {
            TypeInfo::Struct(info) => {
                // struct header
                out.push_str(&format!("{}export interface {} {{\n", self.formatter.get_indentation(), registration.short_name().to_pascal_case()));
                self.formatter.depth.fetch_add(1, Ordering::Relaxed);
                // struct items
                for n in 0..info.field_len() {
                    let field = info.field_at(n).unwrap();


                    out.push_str(&format!("{}{}: {},\n", self.formatter.get_indentation(), field.name(), self.get_field_name(field.type_id())));
                }
                // struct tail
                self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                out.push_str(&format!("{}}}", self.formatter.get_indentation()));
            }
            TypeInfo::TupleStruct(info) => {
                // struct header
                out.push_str(&format!("{}export type {} = [", self.formatter.get_indentation(), registration.short_name().to_pascal_case()));
                // struct items
                for n in 0..info.field_len() {
                    let field = info.field_at(n).unwrap();
                    if n > 0 {
                        if n % self.formatter.max_items_per_line == 0 {
                            out.push_str(",\n");
                        }
                        else {
                            out.push_str(", ");
                        }
                    }

                    out.push_str(&format!("{}{}", self.formatter.get_indentation(), self.get_field_name(field.type_id())));
                }
                // struct tail
                out.push_str(&format!("]"));
            }
            TypeInfo::Enum(info) => {
                // enum header
                out.push_str(&format!("{}export type {} =\n", self.formatter.get_indentation(), registration.short_name().to_pascal_case()));
                self.formatter.depth.fetch_add(1, Ordering::Relaxed);

                for var in info.variant_names() {
                    // variant items
                    match info.variant(var) {
                        Some(VariantInfo::Struct(info)) => {
                            let compact = self.formatter.compact_enum.map(|n| info.field_len() <= n).unwrap_or(false);
                            // struct variant head
                            if compact {
                                out.push_str(&format!("{}| {{ ", self.formatter.get_indentation()));
                            }
                            else {
                                out.push_str(&format!("{}| {{\n", self.formatter.get_indentation()));
                            }
                            self.formatter.depth.fetch_add(2, Ordering::Relaxed);

                            for n in 0..info.field_len() {
                                let field = info.field_at(n).unwrap();
                                if compact {
                                    if n > 0 {
                                        out.push_str(&format!(", "));
                                    }
                                    out.push_str(&format!("\"{}\": {}", field.name(), self.get_field_name(field.type_id())));
                                }
                                else {
                                    out.push_str(&format!("{}\"{}\": {},\n", self.formatter.get_indentation(), field.name(), self.get_field_name(field.type_id())));
                                }
                            }

                            // struct variant tail
                            self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                            if compact {
                                out.push_str(&format!(" }}\n"));
                            }
                            else {
                                out.push_str(&format!("{}}}\n", self.formatter.get_indentation()));
                            }
                            self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                        }
                        Some(VariantInfo::Tuple(info)) => {
                            out.push_str(&format!("{}| [", self.formatter.get_indentation()));
                            self.formatter.depth.fetch_add(2, Ordering::Relaxed);

                            for n in 0..info.field_len() {
                                let field = info.field_at(n).unwrap();
                                if n > 0 {
                                    out.push_str(&format!(", "));
                                }
                                out.push_str(&format!("{}", self.get_field_name(field.type_id())));
                            }

                            // struct variant tail
                            self.formatter.depth.fetch_sub(2, Ordering::Relaxed);
                            out.push_str(&format!("]\n"));
                        }
                        Some(VariantInfo::Unit(info)) => {
                            out.push_str(&format!("{}| \"{}\"\n", self.formatter.get_indentation(), info.name()));
                        }
                        None => {}
                    }

                }

                // enum tail
                self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                out.push_str(&format!(";"));
            }
            _ => panic!("attempted to write unsupported type"),
        }
        out
    }

    fn get_field_name(&self, id: TypeId) -> String {
        // get final type to write
        let mut target_id = &id;
        while let Some(id) = self.type_overrides.get(target_id) {
            target_id = id;
        }

        if let Some(name) = self.type_strings.get(target_id) {
            name.to_string()
        }
        else {
            self.type_registry
                .get(target_id)
                .expect("type not registered")
                .short_name()
                .to_string()
        }
    }
}

/// Gets a flat set of all types being used by a module
fn pull_fields(bm: &BurrMod) -> HashSet<TypeId> {
    let mut fields = HashSet::new();
    // iterate fields for each type and add field's TypeId to set
    for (id, tr) in &bm.types {
        fields.extend(get_fields(tr));
    }
    // then repeat this recursively
    for child in &bm.children {
        fields.extend(pull_fields(child));
    }
    fields
}

fn get_fields(registration: &TypeRegistration) -> HashSet<TypeId> {
    let mut fields = HashSet::new();
    match registration.type_info() {
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
                    VariantInfo::Unit(inner) => {}
                }
            }
        }
        TypeInfo::Value(info) => {
            fields.insert(info.type_id());
        }
    }
    fields
}
