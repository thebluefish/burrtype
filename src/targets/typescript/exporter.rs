use super::{TsFile, TsFormatter};
use crate::export::Burrxporter;
use inflector::Inflector;
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use path_macro::path;
use path_slash::*;
use burrtype_internal::ir::{EnumRepr, IrEnumVariant, IrItem, IrNamedField, IrUnnamedField};

/// An export-friendly version of the Typescript export builder
/// Contains files being exported and computed metadata about files and their types
pub struct TsExporter<'t> {
    pub exporter: &'t Burrxporter,
    pub formatter: TsFormatter<'t>,
    /// maps file paths to files
    pub files: HashMap<PathBuf, TsFile>,
    // /// type information for types being exported
    pub type_registry: HashMap<TypeId, IrItem>,
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
            let mut field_types = Vec::new();

            for item in &file.items {
                field_types.extend(item.all_field_types());
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
            // let import_map: Vec<(PathBuf, HashSet<TypeId>)> = import_map.into_iter().collect();

            // iterate imports and write them
            // for (import, types) in &import_map {
            let mut import_map: Vec<(PathBuf, String)> = import_map.into_iter().map(|(import, types)| {
                let mut types: Vec<&IrItem> = types.iter()
                    .map(|id| self.type_registry.get(id).expect("type should be known by now"))
                    .collect();

                // sort type imports alphabetically
                types.sort_by(|a, b| {
                    a.name().cmp(&b.name())
                });

                let mut out = String::new();
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
                    out.push_str(&strip_rust_prefix(format!("{}", ty.ident())));
                }
                // write import tail
                out.push_str(&format!(" }} from '{}'\n", full_path.to_slash_lossy()));

                (import, out)
            }).collect();

            // sort imports by path
            // longest to shortest, followed by alphabetical sorting
            import_map.sort_by(|(a, astr), (b, bstr)| {
                if astr.len() != bstr.len() {
                    astr.len().cmp(&bstr.len())
                }
                else if a.as_os_str().len() != b.as_os_str().len() {
                    a.as_os_str().len().cmp(&b.as_os_str().len())
                }
                else {
                    a.as_os_str().cmp(b.as_os_str())
                }
            });
            import_map.reverse();

            for (_, import) in &import_map {
                out.push_str(import);
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

    fn format_type(&self, item: &IrItem) -> String {
        let mut out = String::new();
        match item {
            IrItem::NamedStruct(ir) => {
                // struct header
                #[cfg(feature = "comments")]
                if let Some(doc) = ir.docs {
                    out.push_str(&format!("{}/** {doc} */\n", self.formatter.get_indentation()));
                }
                out.push_str(&format!("{}export interface {} {{\n", self.formatter.get_indentation(), strip_rust_prefix(ir.name()).to_pascal_case()));
                self.formatter.depth.fetch_add(1, Ordering::Relaxed);

                // struct items
                for field in &ir.fields {
                    #[cfg(feature = "comments")]
                    if let Some(doc) = field.docs {
                        out.push_str(&format!("{}/** {doc} */\n", self.formatter.get_indentation()));
                    }

                    out.push_str(&format!("{}{}{}: {}{},\n",
                                          self.formatter.get_indentation(),
                                          strip_rust_prefix(field.ident.to_string()),
                                          if field.ty.optional { "?" } else { "" },
                                          self.get_field_name(field.ty.id),
                                          if field.ty.array { "[]" } else { "" },
                    ));
                }

                // struct tail
                self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                out.push_str(&format!("{}}}", self.formatter.get_indentation()));
            }
            IrItem::TupleStruct(ir) => {
                // struct header
                #[cfg(feature = "comments")]
                if let Some(doc) = ir.docs {
                    out.push_str(&format!("{}/** {doc} */\n", self.formatter.get_indentation()));
                }
                // tuples with exactly one field are colloquially considered "newtypes" and often treated as such
                // `serde_json` appears to consider these as newtypes by default, so we treat them as such
                // but this may be incorrect behavior for other serialization frameworks
                if ir.fields.len() == 1 {
                    let field = ir.fields.first().unwrap();
                    #[cfg(feature = "comments")]
                    if let Some(doc) = field.docs {
                        out.push_str(&format!("{}/** {doc} */\n", self.formatter.get_indentation()));
                    }
                    out.push_str(&format!("{}export type {} = {}",
                                          self.formatter.get_indentation(),
                                          strip_rust_prefix(ir.name()).to_pascal_case(),
                                          self.get_field_name(field.ty.id),
                    ));
                }
                else {
                    out.push_str(&format!("{}export type {} = [", self.formatter.get_indentation(), strip_rust_prefix(ir.name()).to_pascal_case()));
                    // struct items
                    for (n, field) in ir.fields.iter().enumerate() {
                        if n > 0 {
                            if n % self.formatter.max_items_per_line == 0 {
                                out.push_str(",\n");
                            }
                            else {
                                out.push_str(", ");
                            }
                        }

                        #[cfg(feature = "comments")]
                        if let Some(doc) = field.docs {
                            out.push_str(&format!("/** {doc} */ "));
                        }
                        out.push_str( &self.get_field_name(field.ty.id));
                    }
                    // struct tail
                    out.push_str(&format!("]"));
                }
            }
            IrItem::UnitStruct(ir) => {
                #[cfg(feature = "comments")]
                if let Some(doc) = ir.docs {
                    out.push_str(&format!("{}/** {doc} */\n", self.formatter.get_indentation()));
                }
                out.push_str(&format!("{}export type {} = null", self.formatter.get_indentation(), strip_rust_prefix(ir.ident.to_string()).to_pascal_case()));
            }
            IrItem::Enum(ir) => {
                // enum header
                #[cfg(feature = "comments")]
                if let Some(doc) = ir.docs {
                    out.push_str(&format!("{}/** {doc} */\n", self.formatter.get_indentation()));
                }

                out.push_str(&format!("{}export type {} =\n", self.formatter.get_indentation(), strip_rust_prefix(ir.name()).to_pascal_case()));
                self.formatter.depth.fetch_add(1, Ordering::Relaxed);

                self.format_enum_variants(&mut out, ir.repr, &ir.variants);

                // enum tail
                self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                out.push_str(&format!(";"));
            }
        }
        out
    }

    fn format_enum_variants(&self, out: &mut String, repr: EnumRepr, variants: &[IrEnumVariant]) {
        for var in variants {
            match var {
                IrEnumVariant::Struct(vir) => {
                    #[cfg(feature = "comments")]
                    if let Some(doc) = vir.docs {
                        out.push_str(&format!("{}/** {doc} */\n", self.formatter.get_indentation()));
                    }
                    let compact = self.formatter.compact_enum.map(|n| vir.fields.len() <= n).unwrap_or(false);

                    match repr {
                        EnumRepr::External => {
                            out.push_str(&format!("{}| {{ {}: {{{}",
                                                  self.formatter.get_indentation(),
                                                  strip_rust_prefix(var.name()),
                                                  if compact { " " } else { "\n" },
                            ));
                            self.formatter.depth.fetch_add(2, Ordering::Relaxed);

                            self.format_enum_struct_fields(out, compact, &vir.fields);

                            self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                            out.push_str(&format!("{}}}}}\n",
                                                  if compact { Cow::from(" ") } else { self.formatter.get_indentation() },
                            ));
                            self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                        }
                        EnumRepr::Untagged => {
                            out.push_str(&format!("{}| {{{}",
                                                  self.formatter.get_indentation(),
                                                  if compact { " " } else { "\n" },
                            ));
                            self.formatter.depth.fetch_add(2, Ordering::Relaxed);

                            self.format_enum_struct_fields(out, compact, &vir.fields);

                            self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                            out.push_str(&format!("{}}}\n",
                                                  if compact { Cow::from(" ") } else { self.formatter.get_indentation() },
                            ));
                            self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                        }
                        EnumRepr::Internal(tag) => {
                            out.push_str(&format!("{}| {{{}",
                                                  self.formatter.get_indentation(),
                                                  if compact { " " } else { "\n" },
                            ));
                            self.formatter.depth.fetch_add(2, Ordering::Relaxed);

                            out.push_str(&format!("{}{}: \"{}\",{}",
                                                  if compact { Cow::from("") } else { self.formatter.get_indentation() },
                                                  tag,
                                                  strip_rust_prefix(var.name()),
                                                  if compact { " " } else { "\n" },
                            ));

                            self.format_enum_struct_fields(out, compact, &vir.fields);

                            self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                            out.push_str(&format!("{}}}\n",
                                                  if compact { Cow::from(" ") } else { self.formatter.get_indentation() },
                            ));
                            self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                        }
                        EnumRepr::Adjacent { tag, content } => {
                            out.push_str(&format!("{}| {{{}",
                                                  self.formatter.get_indentation(),
                                                  if compact { " " } else { "\n" },
                            ));
                            self.formatter.depth.fetch_add(2, Ordering::Relaxed);

                            out.push_str(&format!("{}{}: \"{}\",{}",
                                                  if compact { Cow::from("") } else { self.formatter.get_indentation() },
                                                  tag,
                                                  strip_rust_prefix(var.name()),
                                                  if compact { " " } else { "\n" },
                            ));

                            out.push_str(&format!("{}{}: {{{}",
                                                  if compact { Cow::from("") } else { self.formatter.get_indentation() },
                                                  content,
                                                  if compact { " " } else { "\n" },
                            ));
                            self.formatter.depth.fetch_add(1, Ordering::Relaxed);

                            self.format_enum_struct_fields(out, compact, &vir.fields);

                            self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                            out.push_str(&format!("{}}}{}",
                                                  if compact { Cow::from(" ") } else { self.formatter.get_indentation() },
                                                  if compact { "" } else { "\n" },
                            ));

                            self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                            out.push_str(&format!("{}}}\n",
                                                  if compact { Cow::from(" ") } else { self.formatter.get_indentation() },
                            ));

                            self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                        }
                    }
                }
                IrEnumVariant::Tuple(vir) => {
                    #[cfg(feature = "comments")]
                    if let Some(doc) = vir.docs {
                        out.push_str(&format!("{}/** {doc} */\n", self.formatter.get_indentation()));
                    }

                    // "Newtypes" - tuples with exactly one field - are written without the surrounding tuple representation
                    // This unwrapping of the inner type means we need to flatten any docs too
                    let compact = if vir.fields.len() == 1 {
                        let field = vir.fields.first().unwrap();
                        #[cfg(feature = "comments")]
                        if let Some(doc) = field.docs {
                            out.push_str(&format!("/** {doc} */ "));
                        }
                        true
                    }
                    else { false };

                    match repr {
                        EnumRepr::External => {
                            if compact {
                                let field = vir.fields.first().unwrap();
                                out.push_str(&format!("{}| {{ {}: {} }}\n",
                                                      self.formatter.get_indentation(),
                                                      strip_rust_prefix(var.name()),
                                                      self.get_field_name(field.ty.id),
                                ));
                            }
                            else {
                                out.push_str(&format!("{}| {{ {}: [", self.formatter.get_indentation(), strip_rust_prefix(var.name())));

                                self.format_enum_tuple_fields(out, &vir.fields);

                                out.push_str(&format!("] }}\n"));
                            }

                        }
                        EnumRepr::Untagged => {
                            out.push_str(&format!("{}| {}",
                                                  self.formatter.get_indentation(),
                                                  if compact { "" } else { "[" },
                            ));

                            self.format_enum_tuple_fields(out, &vir.fields);

                            out.push_str(&format!("{}\n", if compact { "" } else { "]" }));
                        }
                        EnumRepr::Adjacent { tag, content } => {
                            out.push_str(&format!("{}| {{{}",
                                                  self.formatter.get_indentation(),
                                                  if compact { "" } else { "\n" },
                            ));
                            self.formatter.depth.fetch_add(2, Ordering::Relaxed);

                            out.push_str(&format!("{}{}: \"{}\",{}",
                                                  if compact { Cow::from(" ") } else { self.formatter.get_indentation() },
                                                  strip_rust_prefix(tag),
                                                  strip_rust_prefix(var.name()),
                                                  if compact { "" } else { "\n" },
                            ));

                            out.push_str(&format!("{}{}: {}",
                                                  if compact { Cow::from(" ") } else { self.formatter.get_indentation() },
                                                  strip_rust_prefix(content),
                                                  if compact { "" } else { "[" },
                            ));

                            self.formatter.depth.fetch_add(1, Ordering::Relaxed);
                            self.format_enum_tuple_fields(out, &vir.fields);
                            self.formatter.depth.fetch_sub(1, Ordering::Relaxed);

                            if !compact {
                                out.push_str("],\n");

                            }

                            self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                            out.push_str(&format!("{}}}\n",
                                                  if compact { Cow::from(" ") } else { self.formatter.get_indentation() }
                            ));
                            self.formatter.depth.fetch_sub(1, Ordering::Relaxed);
                        }
                        // Possible through user-crafted IR, but will never be generated by the derive macro we expect you to use
                        EnumRepr::Internal(_) => unreachable!(),
                    }
                }
                IrEnumVariant::Unit(vir) => {
                    #[cfg(feature = "comments")]
                    if let Some(doc) = vir.docs {
                        out.push_str(&format!("{}/** {doc} */\n", self.formatter.get_indentation()));
                    }

                    match repr {
                        EnumRepr::External => {
                            out.push_str(&format!("{}| \"{}\"\n", self.formatter.get_indentation(), strip_rust_prefix(var.name())));
                        }
                        EnumRepr::Untagged => {
                            // todo: fix this, it is probably incorrect
                            out.push_str(&format!("{}| \"{}\"\n", self.formatter.get_indentation(), strip_rust_prefix(var.name())));
                        }
                        EnumRepr::Internal(tag) => {
                            out.push_str(&format!("{}| {{ {}: \"{}\" }}\n", self.formatter.get_indentation(), tag, strip_rust_prefix(var.name())));
                        }
                        EnumRepr::Adjacent { tag, .. } => {
                            // todo: fix this, it may be incorrect
                            out.push_str(&format!("{}| {{ {}: \"{}\" }}\n", self.formatter.get_indentation(), tag, strip_rust_prefix(var.name())));
                        }
                    }
                }
            }
        }
    }

    fn format_enum_struct_fields(&self, out: &mut String, compact: bool, fields: &[IrNamedField]) {
        for (n, field) in fields.iter().enumerate() {
            if compact {
                if n > 0 {
                    out.push_str(&format!(", "));
                }
                #[cfg(feature = "comments")]
                if let Some(doc) = field.docs {
                    out.push_str(&format!("/** {doc} */ "));
                }
                out.push_str(&format!("{}{}: {}{}",
                                      strip_rust_prefix(field.name()),
                                      if field.ty.optional { "?" } else { "" },
                                      self.get_field_name(field.ty.id),
                                      if field.ty.array { "[]" } else { "" },
                ));
            }
            else {
                #[cfg(feature = "comments")]
                if let Some(doc) = field.docs {
                    out.push_str(&format!("{}/** {doc} */\n", self.formatter.get_indentation()));
                }
                out.push_str(&format!("{}{}{}: {}{},\n",
                                      self.formatter.get_indentation(),
                                      strip_rust_prefix(field.name()),
                                      if field.ty.optional { "?" } else { "" },
                                      self.get_field_name(field.ty.id),
                                      if field.ty.array { "[]" } else { "" },
                ));
            }
        }
    }

    fn format_enum_tuple_fields(&self, out: &mut String, fields: &[IrUnnamedField]) {
        for (n, field) in fields.iter().enumerate() {
            if n > 0 {
                out.push_str(&format!(", "));
            }
            #[cfg(feature = "comments")]
            if let Some(doc) = field.docs {
                out.push_str(&format!("/** {doc} */ "));
            }
            out.push_str(&format!("{}", self.get_field_name(field.ty.id)));
        }
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
                .ident()
                .to_string()
        }
    }
}

fn strip_rust_prefix<'s, S: Into<Cow<'s, str>>>(name: S) -> String {
    let name = name.into();
    match name.strip_prefix("r#") {
        None => name.into(),
        Some(name) => name.into()
    }
}