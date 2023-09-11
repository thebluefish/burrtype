use std::any::TypeId;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{format};
use std::io::Write;
use std::path::Path;
use path_macro::path;
use crate::export::{BurrFile, Burrmatter, BurrMod, Burrxporter, Item, Target};
use crate::ir::{IrNamedStruct, IrTupleStruct, IrType, IrUnitStruct};
use crate::syn::SynIdent;
use inflector::Inflector;
use quote::ToTokens;

/// Determines how we want to map modules to files
// todo: consider moving this and related logic to some sort of common writer
pub enum ModFileMap {
    /// Everything will be written to one file
    /// All modules will be inlined
    Inline,
    /// Top-level modules will be written to individual files
    /// Nested modules will be inlined
    DecomposeTop,
    /// Modules will be written to individual files
    /// Nested modules will create directories with appropriate indices
    DecomposeAll,
}

pub struct TypeScript<'t> {
    pub formatter: TsFormatter<'t>,
    pub mod_file_map: ModFileMap,
    /// replaces Rust types with TS types during export
    pub type_map: HashMap<TypeId, &'t str>,
    pub file_type_map: HashMap<TypeId, BurrFile>,
    // pub file_case: FileCase, // move this type to a "targets/common" or such utility mod
}

impl<'t> Default for TypeScript<'t> {
    fn default() -> Self {
        TypeScript {
            formatter: TsFormatter::pretty(),
            mod_file_map: ModFileMap::DecomposeTop,
            type_map: HashMap::default(),
            file_type_map: Default::default(),
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

    pub fn with_map(mut self, mod_file_map: ModFileMap) -> Self {
        self.mod_file_map = mod_file_map;
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

    pub fn id_to_file(&'t self, id: &TypeId) -> Option<&'t BurrFile> {
        self.file_type_map.get(id)
    }

    fn format_item(&mut self, file: &BurrFile, item: &Item) -> String {
        match item {
            Item::Mod(inner) => self.format_mod(file, inner),
            Item::NamedStruct(inner) => self.format_named_struct(file, inner),
            Item::TupleStruct(inner) => self.format_tuple_struct(file, inner),
            Item::UnitStruct(inner) => self.format_unit_struct(file, inner),
            _ => unimplemented!(),
        }
    }

    fn format_file(&mut self, file: &BurrFile) -> String {
        let mut out = String::new();

        if self.formatter.minify {
            for (i, item) in file.items.iter().enumerate() {
                if i > 0 {
                    out.push_str("");
                }
                out.push_str(&self.format_item(file, item));
            }
        }
        else {
            for (i, item) in file.items.iter().enumerate() {
                if i > 0 {
                    out.push_str("\n");
                }
                out.push_str(&self.format_item(file, item));
                out.push_str("\n");
            }
        }

        out
    }

    fn format_mod(&mut self, file: &BurrFile, item: &BurrMod) -> String {
        let mut out = String::new();

        // write mod header
        if self.formatter.minify {
            out.push_str(&format!("namespace {}{{", item.name.to_pascal_case()));
        }
        else {
            out.push_str(&format!("{}namespace {} {{\n", self.formatter.get_indentation(), item.name.to_pascal_case()));
            self.formatter.depth += 1;
        }
        // write items
        if self.formatter.minify {
            for item in item.items.iter() {
                out.push_str(&self.format_item(file, item));
            }
        }
        else {
            for (i, item) in item.items.iter().enumerate() {
                if i > 0 {
                    out.push_str("\n");
                }
                out.push_str(&self.format_item(file, item));
                out.push_str("\n");
            }
        }
        // write mod footer
        if self.formatter.minify {
            out.push_str("}");
        }
        else {
            self.formatter.depth -= 1;
            out.push_str(&format!("{}}}", self.formatter.get_indentation()));
        }

        out
    }

    fn format_named_struct(&mut self, file: &BurrFile, item: &IrNamedStruct) -> String {
        let mut out = String::new();

        if self.formatter.minify {
            out.push_str(&format!("export interface {}{{}};", item.name.to_string().to_pascal_case()));
        }
        else {
            out.push_str(&format!("{}export interface {} {{\n", self.formatter.get_indentation(), item.name.to_string().to_pascal_case()));
            self.formatter.depth += 1;
            for field in &item.fields {
                let field_file = self.id_to_file(&field.ty.id);
                println!("writing {field:?}");
                // Ensure that we know of all types we're writing
                if field_file.is_none() && !self.type_map.contains_key(&field.ty.id) {
                    panic!("type '{}' unknown", field.ty.path.to_token_stream().to_string());
                }
                // We may need to reference types that aren't builtin
                if let Some(ff) = field_file {
                    // Types belonging to the same file can be referenced as-is
                    if ff.target == file.target {
                        out.push_str(&format!("{}{}: {},\n", self.formatter.get_indentation(), field.name, self.get_item_name(&field.ty)));
                    }
                    // Types belonging to other files need to have their references written to this file
                    // todo: figure out how to add references and write them before other items
                    // perhaps the file-specific writer can build items and then organize them before writing any
                    else {

                    }
                }
                // Type is a builtin, so we'll write it out as-is
                else {
                    out.push_str(&format!("{}{}: {},\n", self.formatter.get_indentation(), field.name, self.get_item_name(&field.ty)));
                }
            }
            self.formatter.depth -= 1;
            out.push_str(&format!("{}}}", self.formatter.get_indentation()));
        }

        out
    }

    fn format_tuple_struct(&mut self, file: &BurrFile, item: &IrTupleStruct) -> String {
        let mut out = String::new();

        if self.formatter.minify {
            out.push_str(&format!("export type {} = [", item.name.to_string().to_pascal_case()));
            for (i, field) in item.fields.iter().enumerate() {
                if i > 0 {
                    out.push_str(&format!(", "));
                }
                out.push_str(&self.get_item_name(&field));
            }
            out.push_str(&format!("];"));
        }
        else {
            out.push_str(&format!("{}export type {} = [", self.formatter.get_indentation(), item.name.to_string().to_pascal_case()));
            for (i, field) in item.fields.iter().enumerate() {
                if i > 0 {
                    out.push_str(&format!(", "));
                }
                out.push_str(&self.get_item_name(&field));
            }
            out.push_str(&format!("]"));
        }

        out
    }

    fn format_unit_struct(&mut self, file: &BurrFile, item: &IrUnitStruct) -> String {
        let mut out = String::new();

        if self.formatter.minify {
            out.push_str(&format!("export interface {}{{}};", item.name.to_string().to_pascal_case()));
        }
        else {
            out.push_str(&format!("{}export interface {} {{}}", self.formatter.get_indentation(), item.name.to_string().to_pascal_case()));
        }

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
                let mut file: BurrFile = exporter.mods.iter().map(Clone::clone).reduce(|mut acc,bm| {
                    acc.items.extend(bm.items);
                    acc
                }).unwrap().into();
                file.target = to.with_extension("ts");

                files.push(file);
            }
            ModFileMap::DecomposeTop => {
                // Convert modules into files
                files.extend(exporter.mods.iter()
                    .map(Clone::clone)
                    .map(Into::<BurrFile>::into)
                    .map(|mut f| { f.target = path!(to / f.target); f })
                );
            }
            ModFileMap::DecomposeAll => {
                todo!()
            }
        }
        // todo: first iterate files and build a list of real types contained within
        // then figure out some way to get relative path from one file to another
        // write references for all types external to a file
        // log types that are not covered
        for file in &files {
            for (key, value) in &file.exports {
                if let Some(k) = self.file_type_map.insert(key.clone(), file.clone()) {
                    panic!("duplicate key exists for type: {value}");
                }
            }
        }
        for file in &files {
            println!("file {}", file.target.to_string_lossy());
            let mut writer = exporter.open_writer(&file.target).unwrap();
            writer.write(self.format_file(&file).as_bytes()).unwrap();
        }
    }
}

/// A formatter with options to cover most general cases
pub struct TsFormatter<'t> {
    minify: bool,
    depth: usize,
    tab: Cow<'t, str>,
    max_items_per_line: usize,
}

impl<'t> TsFormatter<'t> {
    pub fn pretty() -> Self {
        TsFormatter {
            minify: false,
            depth: 0,
            tab: "  ".into(),
            max_items_per_line: 12,
        }
    }

    pub fn minify() -> Self {
        TsFormatter {
            minify: true,
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