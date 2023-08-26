use std::fmt::{format};
use std::io::Write;
use std::path::Path;
use path_macro::path;
use crate::export::{BurrFile, Burrmatter, BurrMod, Burrxporter, Formatter, Target};

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

pub struct TypeScript {
    pub formatter: Box<dyn Formatter>,
    pub mod_file_map: ModFileMap,
    // pub file_case: FileCase, // move this type to a "targets/common" or such utility mod
}

impl TypeScript {
    /// Sets up a new TS target with defaults matching common industry standards
    pub fn new() -> Self {
        TypeScript {
            formatter: Box::new(Burrmatter::new()),
            mod_file_map: ModFileMap::DecomposeTop,
        }
    }

    /// Creates a new TS ta
    pub fn from_formatter<F: Formatter + 'static>(formatter: F) -> Self {
        TypeScript {
            formatter: Box::new(formatter),
            mod_file_map: ModFileMap::DecomposeTop,
        }
    }

    pub fn with_formatter<F: Formatter + 'static>(mut self, formatter: F) -> Self {
        self.formatter = Box::new(formatter);
        self
    }

    pub fn with_map(mut self, mod_file_map: ModFileMap) -> Self {
        self.mod_file_map = mod_file_map;
        self
    }
}

impl Target for TypeScript {
    fn export(&mut self, to: &Path, exporter: &Burrxporter) {
        let mut files = vec![];
        match self.mod_file_map {
            ModFileMap::Inline => {
                let file = BurrFile {
                    target: to.with_extension("ts"),
                    items: exporter.mods.iter().map(Clone::clone).map(Into::into).collect(),
                };
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
        for BurrFile { target, items } in files {
            println!("mod {}", target.to_string_lossy());
            let mut writer = exporter.open_writer(&target).unwrap();
            writer.write("kek".as_bytes()).unwrap();
        }
    }
}