mod burrmod;
mod target;

use std::any::TypeId;
use std::collections::{HashMap, HashSet};
pub use burrmod::*;
pub use target::*;

use std::fs;
use std::fs::File;
use std::io::{BufWriter, Error as IoError, Write};
use std::path::{Path, PathBuf};
use path_macro::path;
use burrtype_internal::ir::IrItem;

#[derive(thiserror::Error, Debug)]
pub enum ExportError {
    #[error(transparent)]
    IoError(#[from] IoError),
    #[error("data store disconnected")]
    InvalidTarget,
}

/// Builds and exports a collection of modules representing your public API
/// Supporting all #[derive(Burr)] items:
/// - Modules
/// - Structs
/// - Enums
/// - Constants
/// - Typedefs
/// Supports exporting to different languages
pub struct Burrxporter {
    pub mods: Vec<BurrMod>,
    pub root: Option<PathBuf>,
    pub type_registry: HashMap<TypeId, IrItem>,
}

impl Burrxporter {
    pub fn new() -> Self {
        let mut type_registry = HashMap::with_capacity(crate::TYPES.len());
        type_registry.extend(crate::TYPES.iter().map(|ty_fn| {
            let tr = ty_fn();
            (tr.type_id(), tr)
        }));

        Burrxporter {
            mods: Vec::new(),
            root: None,
            type_registry,
        }
    }

    pub fn with_mod<M: Into<BurrMod>>(&mut self, r#mod: M) -> &mut Self {
        self.mods.push(r#mod.into());
        self
    }

    /// Sets root path for exports
    pub fn with_root<P: Into<PathBuf>>(&mut self, to: P) -> &mut Self {
        self.root = Some(to.into());
        self
    }

    /// Adds output target with configuration
    pub fn export<P: AsRef<Path>, T: Target>(&mut self, to: P, target: T) -> Result<&mut Self, ExportError> {
        target.export(to.as_ref(), &self);
        Ok(self)
    }

    /// Resolves all modules for export
    /// Collects dependent types and resolves their target modules
    pub fn resolve_exports(&mut self, default: &str) -> &mut Self {
        // Ensure dependencies of dependencies are also included by repeatedly checking until no more have been included
        let mut dirty = true;
        while dirty {
            dirty = false;

            let mut exporting = HashSet::<TypeId>::new();
            let mut importing = HashSet::<TypeId>::new();
            for om in &self.mods {
                exporting.extend(om.pull_exports());
                importing.extend(om.pull_fields());
            }

            let mut touched_mods = Vec::new();
            let mut diff = Vec::new();
            for id in importing.difference(&exporting) {
                // todo: consider handling the None case
                // None usually means we have encountered a builtin such as usize or String, but might not always
                if let Some(ir) = self.type_registry.get(id) {
                    diff.push((ir.clone(), ir.mod_override().unwrap_or(default)));
                }
            }
            diff.sort_by(|(_, a), (_, b)| a.cmp(b));

            for (ir, path) in diff {
                if let Some((bm, auto)) = get_or_create_mod(&mut self.mods, Path::new(path)) {
                    bm.auto_exports.push(ir.type_id());
                    bm.types.insert(ir.type_id(), ir);
                    if auto {
                        touched_mods.push(Path::new(path));
                    }
                }
            }

            for path in touched_mods {
                let (tm, auto) = get_or_create_mod(&mut self.mods, path).unwrap();
                assert!(!auto);

                tm.auto_exports.sort_by(|a, b| {
                    let a = tm.types.get(a).unwrap().name();
                    let b = tm.types.get(b).unwrap().name();
                    a.cmp(&b)
                });
            }
        }
        self
    }

    /// Gets the writer for a file path
    /// Creates the directory and file if it does not exist, truncates if it does
    // todo: options for allowing user to plugin their writer, something like a Box<dyn Writer> or an enum Burrwriter<'t> { Owned, Shared<'t> }
    pub(crate) fn open_writer(&self, to: &Path) -> Result<impl Write, ExportError> {
        let path = self.root.as_ref().map_or_else(|| to.to_path_buf(), |root| path!(root / to));

        // Extract parent and ensure it exists
        let parent = match path.parent() {
            Some(path) => path,
            None => return Err(ExportError::InvalidTarget),
        };
        fs::create_dir_all(parent)?;

        // Creates the file and returns it
        let file = File::create(path)?;
        Ok(BufWriter::new(file))
    }
}

/// Gets a module at the specified path, or creates the necessary module tree as needed
/// todo: convert the return type to a more descriptive error type when we are ready to reorganize things for error handling
fn get_or_create_mod<'m, 'p>(mods: &'m mut Vec<BurrMod>, path: &'p Path) -> Option<(&'m mut BurrMod, bool)> {
    let mut created = false;

    if path.components().count() == 0 {
        return None;
    }

    // We have to handle root mods separately from child mods
    // This feels anti-DRY, but idk a better way
    let mut components = path.components();
    let cname = components.next().unwrap().as_os_str().to_string_lossy();

    let mut search = if let Some(pos) = mods.iter().position(|bm| bm.name == cname) {
        &mut mods[pos]
    }
    else {
        created = true;
        mods.push(BurrMod::new(cname.to_owned()));
        mods.last_mut().unwrap()
    };

    for component in components {
        let cname = component.as_os_str().to_string_lossy();

        search = if let Some(pos) = search.children.iter().position(|bm| bm.name == cname) {
            &mut search.children[pos]
        }
        else {
            created = true;
            search.children.push(BurrMod::new(cname.to_owned()));
            search.children.last_mut().unwrap()
        };
    }

    Some((search, created))
}