mod file;
mod exporter;

pub use file::*;

use exporter::*;
use crate::export::{BurrMod, Burrxporter, Target};
use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use bevy_reflect::{TypeInfo, TypeRegistration, VariantInfo};
use path_macro::path;
use path_slash::*;
use inflector::Inflector;

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
    /// todo: fill this during phase 1 of export, then consume it for export?
    /// todo:: consider also converting this struct to something similar with another field for this specific purpose?
    /// todo: also consider that we may simply replace this with <TypeId, TypeRegistration> if the registration can properly boil types down
    pub type_map: HashMap<TypeId, &'t str>,
    /// types being mapped to other types
    pub type_overrides: HashMap<TypeId, TypeId>,
}

impl<'t> Default for TypeScript<'t> {
    fn default() -> Self {
        TypeScript {
            formatter: TsFormatter::pretty(),
            mod_file_map: ModFileMap::DecomposeAll,
            index_generator: IndexGeneratorType::Full,
            type_map: HashMap::default(),
            type_overrides: Default::default(),
        }
    }
}

impl<'t> TypeScript<'t> {
    /// Sets up a new TS target with defaults matching common standards
    pub fn new() -> Self {
        TypeScript::default().with_std_remaps()
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

    /// Sets the exported name of the given type when writing fields
    pub fn with_type_name<T: ?Sized + 'static>(mut self, name: &'t str) -> Self {
        self.type_map.insert(TypeId::of::<T>(), name);
        self
    }

    /// Substitutes F with T when writing fields
    pub fn with_type_remap<F: ?Sized + 'static, T: ?Sized + 'static>(mut self, name: &'t str) -> Self {
        self.type_overrides.insert(TypeId::of::<F>(), TypeId::of::<T>());
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

    pub fn build(self, to: &'t Path, mods: Vec<BurrMod>) -> TsExporter<'t> {
        let TypeScript { formatter, mod_file_map, index_generator, type_map, type_overrides } = self;

        // Builds the set of files to write
        let mut files = HashMap::new();
        match mod_file_map {
            ModFileMap::Inline => {
                // Collect items from all top-level modules into a single top-level file
                let mut file = TsFile {
                    name: to.to_slash_lossy().to_string(),
                    target: to.with_extension("ts"),
                    ..Default::default()
                };
                flatten_all(&mut file, mods);

                files.insert(file.target.clone(), file);
            }
            ModFileMap::DecomposeTop => {
                // Convert modules into files
                files.extend(mods.into_iter()
                    .map(Into::<TsFile>::into)
                    .map(|mut f| { f.target = path!(to / f.target); (f.target.clone(), f) })
                );
            }
            ModFileMap::DecomposeAll => {
                for mut file in mods.into_iter()
                    .map(Into::<TsFile>::into) {
                    let children = decompose_all(&mut file);
                    files.insert(file.target.clone(), file);
                    files.extend(children.into_iter().map(|file| (file.target.clone(), file)));
                }
            }
        }

        // build a map of all types being exported
        let mut type_registry: HashMap<TypeId, TypeRegistration> = HashMap::new();
        let mut type_exports: HashMap<TypeId, PathBuf> = HashMap::new();
        for file in files.values() {
            // Flatten all items in this file
            let mut flat_items = Vec::new();
            flat_items.extend(&file.items);
            for bm in &file.mods {
                flat_items.extend(pull_flat_items(bm));
            }

            for item in flat_items {
                if let Some(old) = type_registry.insert(item.type_id(), item.clone()) {
                    // todo: make this return an error
                    panic!("Type <{}> exported to multiple files:\n  old: {}\n  new: {}",
                           item.short_name(),
                           old.short_name(),
                           file.target.to_string_lossy()
                    );
                }
                if let Some(old) = type_exports.insert(item.type_id(), file.target.clone()) {
                    // todo: make this return an error
                    panic!("Type <{}> exported to multiple files:\n  old: {}\n  new: {}",
                           item.short_name(),
                           old.to_string_lossy(),
                           file.target.to_string_lossy()
                    );
                }
            }
        }

        TsExporter {
            formatter,
            files,
            type_registry,
            type_exports,
            type_overrides,
            type_strings: type_map,
        }
    }
}

impl<'f> Target for TypeScript<'f> {
    fn export(mut self, to: &Path, exporter: &Burrxporter) {
        // build our export-friendly type and export it
        self
            .build(to, exporter.mods.clone())
            .export(exporter);
    }
}

fn flatten_all(target: &mut TsFile, mods: Vec<BurrMod>) {
    println!("flattening {} children into {}", mods.len(), target.name);
    for child in mods {
        target.items.extend(child.types.into_values());
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
fn pull_flat_items(bm: &BurrMod) -> Vec<&TypeRegistration> {
    let mut items = Vec::new();
    items.extend(bm.types.values());
    for child in &bm.children {
        items.extend(pull_flat_items(child));
    }

    items
}

/// A formatter with options to cover most general cases
pub struct TsFormatter<'t> {
    depth: AtomicUsize,
    tab: Cow<'t, str>,
    max_items_per_line: usize,
    /// Controls whether to compact enum variants to a single line each, up to the number of arguments specified
    compact_enum: Option<usize>,
}

impl<'t> TsFormatter<'t> {
    pub fn pretty() -> Self {
        TsFormatter {
            depth: AtomicUsize::new(0),
            tab: "  ".into(),
            // todo: change this to an enum to be based on either number of items per line, max line length, or max combined item length
            max_items_per_line: 12,
            // todo: add a wrap_to field that describes how we should wrap to the next line, such as aligning the first item for each line
            compact_enum: Some(2),
        }
    }

    pub fn with_max_items_per_line(mut self, n: usize) -> Self {
        self.max_items_per_line = n;
        self
    }

    pub fn get_indentation(&self) -> Cow<'static, str> {
        self.tab.repeat(self.depth.load(Ordering::Relaxed)).into()
    }
}