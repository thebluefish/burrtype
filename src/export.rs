mod burrmod;
mod target;

use std::any::TypeId;
use std::collections::HashMap;
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
    pub default_mod: Option<String>,
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
            default_mod: None,
            type_registry,
        }
    }

    /// Sets the "default" mod, where unspecified imports will be exported to
    pub fn with_default_mod<S: Into<String>>(&mut self, target: S) -> &mut Self {
        self.default_mod = Some(target.into());
        self
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