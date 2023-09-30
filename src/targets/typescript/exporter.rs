use super::{TsFile, TsFormatter};
use crate::export::Burrxporter;
use bevy_reflect::{TypeInfo, TypeRegistration, VariantInfo};
use inflector::Inflector;
use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use path_macro::path;
use path_slash::*;
use crate::type_registration::TypeRegistrationExt;

/// An export-friendly version of the Typescript export builder
/// Contains files being exported and computed metadata about files and their types
pub struct TsExporter<'t> {
    pub exporter: &'t Burrxporter,
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
    pub fn export(self) {
        // Export files
        for file in self.files.values() {
            let mut out = String::new();

            // build imports
            // these will be written first and should look like:
            // import * as {name} from {path}
            let mut field_types = HashSet::new();

            for item in &file.items {
                field_types.extend(item.get_fields());
            }

            for item in &file.mods {
                field_types.extend(item.pull_fields());
            }

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
                // resolve relative path from other file to this one
                let mut depth = 0;
                let mut parent = file.target.parent();
                let mut found = None;
                while let Some(path) = parent {
                    if let Ok(p) = import.strip_prefix(path) {
                        found = Some(p.to_path_buf());
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
            for (i, item) in file.items.iter().enumerate() {
                if i > 0 {
                    out.push_str("\n");
                }
                out.push_str(&self.format_type(item));
                out.push_str("\n");
            }

            // export to file
            if out.is_empty() {
                println!("skipping empty file: {}", file.target.to_slash_lossy());
            }
            else {
                println!("writing to: {}", file.target.to_slash_lossy());

                let mut writer = self.exporter.open_writer(&file.target).unwrap();
                writer.write(out.as_bytes()).unwrap();
            }
        }
    }

    fn format_type(&self, registration: &TypeRegistration) -> String {
        let mut out = String::new();
        match registration.type_info() {
            TypeInfo::Struct(info) => {
                // struct header
                #[cfg(feature = "comments")]
                if let Some(docs) = info.docs() {
                    out.push_str(&format!("{}/** {docs} */\n", self.formatter.get_indentation()));
                }
                if info.field_len() == 0 {
                    out.push_str(&format!("{}export interface {} {{}}", self.formatter.get_indentation(), registration.short_name().to_pascal_case()));
                }
                else {
                    out.push_str(&format!("{}export interface {} {{\n", self.formatter.get_indentation(), registration.short_name().to_pascal_case()));
                    self.formatter.depth.fetch_add(1, Ordering::Relaxed);
                    // struct items
                    for n in 0..info.field_len() {
                        let field = info.field_at(n).unwrap();
                        #[cfg(feature = "comments")]
                        if let Some(docs) = field.docs() {
                            out.push_str(&format!("{}/** {docs} */\n", self.formatter.get_indentation()));
                        }
                        out.push_str(&format!("{}{}: {},\n", self.formatter.get_indentation(), field.name(), self.get_field_name(field.type_id())));
                    }
                    // struct tail
                    self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                    out.push_str(&format!("{}}}", self.formatter.get_indentation()));
                }
            }
            TypeInfo::TupleStruct(info) => {
                // struct header
                #[cfg(feature = "comments")]
                if let Some(docs) = info.docs() {
                    out.push_str(&format!("{}/** {docs} */\n", self.formatter.get_indentation()));
                }
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

                    #[cfg(feature = "comments")]
                    if let Some(docs) = field.docs() {
                        out.push_str(&format!("/** {docs} */ "));
                    }
                    out.push_str( &self.get_field_name(field.type_id()));
                }
                // struct tail
                out.push_str(&format!("]"));
            }
            TypeInfo::Enum(info) => {
                // enum header
                #[cfg(feature = "comments")]
                if let Some(docs) = info.docs() {
                    out.push_str(&format!("{}/** {docs} */\n", self.formatter.get_indentation()));
                }
                out.push_str(&format!("{}export type {} =\n", self.formatter.get_indentation(), registration.short_name().to_pascal_case()));
                self.formatter.depth.fetch_add(1, Ordering::Relaxed);

                for var in info.variant_names() {
                    // variant items
                    match info.variant(var) {
                        Some(VariantInfo::Struct(info)) => {
                            #[cfg(feature = "comments")]
                            if let Some(docs) = info.docs() {
                                out.push_str(&format!("{}/** {docs} */\n", self.formatter.get_indentation()));
                            }
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
                                    #[cfg(feature = "comments")]
                                    if let Some(docs) = field.docs() {
                                        out.push_str(&format!("/** {docs} */ "));
                                    }
                                    out.push_str(&format!("\"{}\": {}", field.name(), self.get_field_name(field.type_id())));
                                }
                                else {
                                    #[cfg(feature = "comments")]
                                    if let Some(docs) = field.docs() {
                                        out.push_str(&format!("{}/** {docs} */\n", self.formatter.get_indentation()));
                                    }
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
                            #[cfg(feature = "comments")]
                            if let Some(docs) = info.docs() {
                                out.push_str(&format!("{}/** {docs} */\n", self.formatter.get_indentation()));
                            }
                            out.push_str(&format!("{}| [", self.formatter.get_indentation()));
                            self.formatter.depth.fetch_add(2, Ordering::Relaxed);

                            for n in 0..info.field_len() {
                                let field = info.field_at(n).unwrap();
                                if n > 0 {
                                    out.push_str(&format!(", "));
                                }
                                #[cfg(feature = "comments")]
                                if let Some(docs) = field.docs() {
                                    out.push_str(&format!("/** {docs} */ "));
                                }
                                out.push_str(&format!("{}", self.get_field_name(field.type_id())));
                            }

                            // struct variant tail
                            self.formatter.depth.fetch_sub(2, Ordering::Relaxed);
                            out.push_str(&format!("]\n"));
                        }
                        Some(VariantInfo::Unit(info)) => {
                            #[cfg(feature = "comments")]
                            if let Some(docs) = info.docs() {
                                out.push_str(&format!("{}/** {docs} */\n", self.formatter.get_indentation()));
                            }
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
                .unwrap_or_else(|| self.exporter.type_registry.get(target_id).expect("type not registered"))
                .short_name()
                .to_string()
        }
    }
}