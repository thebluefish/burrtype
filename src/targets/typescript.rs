mod file;

pub use file::*;

use crate::export::{Burrmatter, BurrMod, Burrxporter, Item, Target};
use burrtype_internal::prelude::*;
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use path_macro::path;
use path_slash::*;
use inflector::Inflector;
use proc_macro2::Ident;
use quote::ToTokens;

/// Determines how we want to map modules to files
// todo: consider moving this and related logic to some sort of common writer
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ModFileMap {
    /// Everything will be written to one file
    /// All modules will be inlined
    /// This implies `IndexGeneratorType::None`
    Inline,
    /// Top-level modules will be written to individual files
    /// Nested modules will be inlined
    DecomposeTop,
    /// Modules will be written to individual files
    /// Nested modules will create directories with appropriate indices
    DecomposeAll,
}

/// Determines how we want to generate indices
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IndexGeneratorType {
    /// Every file belonging to a directory, relative to the root, will have its exports indexed
    Full,
    /// All files will have their exports written to a single index at the root
    Flat,
    /// No indices will be generates for modules
    None,
}

pub struct TypeScript<'t> {
    pub formatter: TsFormatter<'t>,
    pub mod_file_map: ModFileMap,
    pub index_generator: IndexGeneratorType,
    /// replaces Rust types with TS types during export
    pub type_map: HashMap<TypeId, &'t str>,
}

impl<'t> Default for TypeScript<'t> {
    fn default() -> Self {
        TypeScript {
            formatter: TsFormatter::pretty(),
            mod_file_map: ModFileMap::DecomposeAll,
            index_generator: IndexGeneratorType::Full,
            type_map: HashMap::default(),
        }
    }
}

impl<'t> TypeScript<'t> {
    /// Sets up a new TS target with defaults matching common standards
    pub fn new() -> Self {
        TypeScript::default()
            .with_std_remaps()
    }

    pub fn with_std_remaps(mut self) -> Self {
        self.type_map.extend([
            (TypeId::of::<str>(), "string"),
            (TypeId::of::<char>(), "string"),
            (TypeId::of::<String>(), "string"),
            (TypeId::of::<bool>(), "boolean"),
            (TypeId::of::<usize>(), "number"),
            (TypeId::of::<u8>(), "number"),
            (TypeId::of::<u16>(), "number"),
            (TypeId::of::<u32>(), "number"),
            (TypeId::of::<u64>(), "number"),
            (TypeId::of::<u128>(), "number"),
            (TypeId::of::<isize>(), "number"),
            (TypeId::of::<i8>(), "number"),
            (TypeId::of::<i16>(), "number"),
            (TypeId::of::<i32>(), "number"),
            (TypeId::of::<i64>(), "number"),
            (TypeId::of::<i128>(), "number"),
            (TypeId::of::<f32>(), "number"),
            (TypeId::of::<f64>(), "number"),
        ]);
        self
    }

    pub fn with_formatter(mut self, formatter: TsFormatter<'t>) -> Self {
        self.formatter = formatter;
        self
    }

    /// Controls how modules are mapped to files
    pub fn with_file_map(mut self, mod_file_map: ModFileMap) -> Self {
        self.mod_file_map = mod_file_map;
        self
    }

    /// Controls how indices are generated for exported directories
    pub fn with_index_generator(mut self, index_generator: IndexGeneratorType) -> Self {
        self.index_generator = index_generator;
        self
    }

    /// Gets an item's name, remapped if necessary
    pub fn get_item_name(&self, ir: &IrType) -> String {
        if let Some(t) = self.type_map.get(&ir.id) {
            return t.to_string()
        }
        else {
            return ir.get_ident().unwrap().to_string()
        }
    }

    fn format_item(&mut self, item: &Item) -> String {
        match item {
            Item::NamedStruct(inner) => self.format_named_struct(inner),
            Item::TupleStruct(inner) => self.format_tuple_struct(inner),
            Item::UnitStruct(inner) => self.format_unit_struct(inner),
            _ => unimplemented!(),
        }
    }

    fn format_file(&mut self, file: &TsFile) -> String {
        let mut out = String::new();

        for (i, item) in file.items.iter().enumerate() {
            if i > 0 {
                out.push_str("\n");
            }
            out.push_str(&self.format_item(item));
            out.push_str("\n");
        }

        out
    }

    fn format_mod(&mut self, item: &BurrMod) -> String {
        let mut out = String::new();

        // write mod header
        out.push_str(&format!("{}namespace {} {{\n", self.formatter.get_indentation(), item.name.to_pascal_case()));
        self.formatter.depth += 1;
        // write items

        for (i, item) in item.items.iter().enumerate() {
            if i > 0 {
                out.push_str("\n");
            }
            out.push_str(&self.format_item(item));
            out.push_str("\n");
        }
        // write mod footer
        self.formatter.depth -= 1;
        out.push_str(&format!("{}}}", self.formatter.get_indentation()));

        out
    }

    fn format_named_struct(&mut self, item: &IrNamedStruct) -> String {
        let mut out = String::new();

        // write struct header
        out.push_str(&format!("{}export interface {} {{\n", self.formatter.get_indentation(), item.name.to_string().to_pascal_case()));
        self.formatter.depth += 1;
        // write items
        for field in &item.fields {
            out.push_str(&format!("{}{}: {},\n", self.formatter.get_indentation(), field.name, self.get_item_name(&field.ty)));
        }
        // write struct tail
        self.formatter.depth -= 1;
        out.push_str(&format!("{}}}", self.formatter.get_indentation()));

        out
    }

    fn format_tuple_struct(&mut self, item: &IrTupleStruct) -> String {
        let mut out = String::new();

        // write struct header
        out.push_str(&format!("{}export type {} = [", self.formatter.get_indentation(), item.name.to_string().to_pascal_case()));
        // write items
        for (i, field) in item.fields.iter().enumerate() {
            if i > 0 {
                out.push_str(&format!(", "));
            }
            out.push_str(&self.get_item_name(&field));
        }
        // write struct tail
        out.push_str(&format!("]"));

        out
    }

    fn format_unit_struct(&mut self, item: &IrUnitStruct) -> String {
        let mut out = String::new();

        out.push_str(&format!("{}export interface {} {{}}", self.formatter.get_indentation(), item.name.to_string().to_pascal_case()));

        out
    }
}

impl<'f> Target for TypeScript<'f> {
    fn export(&mut self, to: &Path, exporter: &Burrxporter) {
        // Builds the set of files to write
        let mut files = vec![];
        match self.mod_file_map {
            ModFileMap::Inline => {
                // Collect items from all top-level modules into a single top-level file
                let mut file = TsFile {
                    name: to.to_slash_lossy().to_string(),
                    target: to.with_extension("ts"),
                    ..Default::default()
                };
                flatten_all(&mut file, exporter.mods.clone());

                files.push(file);
            }
            ModFileMap::DecomposeTop => {
                // Convert modules into files
                files.extend(exporter.mods.clone().into_iter()
                    .map(Into::<TsFile>::into)
                    .map(|mut f| { f.target = path!(to / f.target); f })
                );
            }
            ModFileMap::DecomposeAll => {
                for mut file in exporter.mods.clone().into_iter()
                    .map(Into::<TsFile>::into) {
                    let children = decompose_all(&mut file);
                    files.push(file);
                    files.extend(children);
                }
            }
        }
        for file in &files {
            println!("made {} items", file.items.len());
        }

        // build a map of all types being exported
        let mut type_map: HashMap<TypeId, (Ident, PathBuf)> = HashMap::new();
        for file in &files {
            // Flatten all items in this file
            let mut flat_items = Vec::new();
            flat_items.extend(&file.items);
            for bm in &file.mods {
                flat_items.extend(pull_flat_items(bm));
            }

            for item in flat_items {
                if let Some((_, old)) = type_map.insert(item.get_id(), (item.get_name().clone(), file.target.clone())) {
                    // todo: make this return an error
                    panic!("Type <{}> exported to multiple files:\n  old: {}\n  new: {}",
                           item.get_name(),
                           old.to_string_lossy(),
                           file.target.to_string_lossy()
                    );
                }
            }
        }

        // build indices
        let mut indices: HashMap<PathBuf, Vec<&TsFile>> = HashMap::new();
        if self.mod_file_map != ModFileMap::Inline && self.index_generator != IndexGeneratorType::None {
            for file in &files {
                if let Some(parent) = file.target.parent() {
                    indices.entry(parent.into()).or_default().push(file);
                }
            }
        }

        // // write indices
        // for (index, children) in indices {
        //     let mut out = String::new();
        //     // build imports while collecting names for export
        //     let mut names = children.into_iter().enumerate().map(|(i, child)| {
        //         if i > 0 {
        //             out.push_str("\n");
        //         }
        //         let name = child.name.to_pascal_case();
        //         let path = path!("." / child.target.strip_prefix(&index).unwrap()).with_extension("");
        //         out.push_str(&format!("import * as {} from '{}'", name, path.to_slash_lossy()));
        //         name
        //     }).collect::<Vec<_>>();
        //
        //     // build exports
        //     if !names.is_empty() {
        //         out.push_str("\n\nexport { ");
        //         for (i, name) in names.into_iter().enumerate() {
        //             if i > 0 {
        //                 out.push_str(", ");
        //             }
        //             out.push_str(&format!("{name}"));
        //         }
        //         out.push_str(" }");
        //     }
        //
        //     let mut writer = exporter.open_writer(&path!(index / "index.ts")).unwrap();
        //     writer.write(out.as_bytes()).unwrap();
        // }

        // Export files
        for file in &mut files {
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

            let mut import_map: HashMap<PathBuf, HashSet<TypeId>> = HashMap::new();
            for id in &field_types {
                if let Some((_, file)) = type_map.get(id) {
                    import_map.entry(file.clone()).or_default().insert(id.clone());
                }
            }
            // remove self-references
            import_map.remove(&file.target);

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
                    out.push_str(&format!("{}", type_map.get(ty).expect("type should be known by now").0));
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
                out.push_str(&self.format_item(item));
            }

            // create and/or write file
            let to = match &exporter.root {
                Some(root) => path!(root / file.target),
                None => file.target.clone(),
            };
            println!("writing to {}", to.to_slash_lossy());
            let mut writer = exporter.open_writer(&to).unwrap();
            writer.write(out.as_bytes()).unwrap();
        }

        // build imports for files
        // fix references for types
        // ideally imports should look something like
//         ```ts
//         import * as {name} from {path}
//         export type foos = [name.Foo, name.Bar]
//         ```

        // todo: fix formatting so that it no longer relies on type mapping
        // at this stage the type information has already been resolved and mapped into the relevant IR
        // for file in &files {
        //     for (key, value) in &file.exports {
        //         if let Some(k) = self.file_type_map.insert(key.clone(), file.clone()) {
        //             panic!("duplicate key exists for type: {value}");
        //         }
        //     }
        // }
        // for file in &files {
        //     println!("file {}", file.target.to_string_lossy());
        //     let mut writer = exporter.open_writer(&file.target).unwrap();
        //     writer.write(self.format_file(&file).as_bytes()).unwrap();
        // }
    }
}

fn flatten_all(target: &mut TsFile, mods: Vec<BurrMod>) {
    println!("flattening {} children into {}", mods.len(), target.name);
    for child in mods {
        target.items.extend(child.items);
        flatten_all(target, child.children);
    }
}

/// Recursively convert a tree of modules into files and directories
fn decompose_all(file: &mut TsFile) -> Vec<TsFile> {
    let mut files = Vec::new();
    // correct the file path for directories
    if !file.mods.is_empty() {
        file.target = path!(file.target.with_extension("") / "index.ts");
    }
    for mut child in file.mods.drain(..).map(Into::<TsFile>::into) {
        // add prefix to child
        if let Some(parent) = file.target.parent() {
            if !parent.as_os_str().is_empty() {
                child.target = path!(parent / child.target);
            }
        }
        let children = decompose_all(&mut child);
        files.push(child);
        files.extend(children);
    }
    files
}

/// Gets a flat list of all items
fn pull_flat_items(bm: &BurrMod) -> Vec<&Item> {
    let mut items = Vec::new();
    items.extend(bm.items.iter());
    for child in &bm.children {
        items.extend(pull_flat_items(child));
    }

    items
}

/// Gets a flat set of all types being used by a module
fn pull_fields(bm: &BurrMod) -> HashSet<TypeId> {
    let mut fields = HashSet::new();
    for item in &bm.items {
        fields.extend(get_fields(item));
    }
    for child in &bm.children {
        fields.extend(pull_fields(child));
    }
    fields
}

/// Gets a flat set of all types being used by an item
fn get_fields(item: &Item) -> HashSet<TypeId> {
    let mut fields = HashSet::new();

    match item {
        Item::NamedStruct(inner) => {
            for field in &inner.fields {
                fields.insert(field.ty.id);
            }
        },
        Item::TupleStruct(inner) => {
            for field in &inner.fields {
                fields.insert(field.id);
            }
        },
        Item::UnitStruct(inner) => {}
    }

    fields
}

/// A formatter with options to cover most general cases
pub struct TsFormatter<'t> {
    depth: usize,
    tab: Cow<'t, str>,
    max_items_per_line: usize,
}

impl<'t> TsFormatter<'t> {
    pub fn pretty() -> Self {
        TsFormatter {
            depth: 0,
            tab: "  ".into(),
            max_items_per_line: 12,
        }
    }

    pub fn minify() -> Self {
        TsFormatter {
            tab: "".into(),
            ..Self::pretty()
        }
    }

    pub fn with_max_items_per_line(mut self, n: usize) -> Self {
        self.max_items_per_line = n;
        self
    }

    pub fn get_indentation(&self) -> Cow<'static, str> {
        self.tab.repeat(self.depth).into()
    }
}