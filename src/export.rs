mod burrmod;
mod target;

pub use burrmod::*;
pub use target::*;

use std::fs;
use std::fs::File;
use std::io::{BufWriter, Error as IoError, Write};
use std::path::{Path, PathBuf};
use path_macro::path;

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
}

impl Burrxporter {
    pub fn new() -> Self {
        Burrxporter {
            mods: Vec::new(),
            root: None,
        }
    }

    /// Adds input module to root collection
    pub fn with_mod<M: Into<BurrMod>>(&mut self, r#mod: M) -> &mut Self {
        self.mods.push(r#mod.into());
        self
    }

    /// Sets root path for exports
    pub fn with_root(&mut self, to: &Path) -> &mut Self {
        self.root = Some(to.to_path_buf());
        self
    }

    /// Adds output target with configuration
    pub fn export<T: Target>(&mut self, to: &Path, mut target: T) -> Result<&mut Self, ExportError> {
        target.export(to, &self);
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