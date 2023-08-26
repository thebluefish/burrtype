mod burrmod;
mod file;
mod format;
mod item;
mod output;
mod target;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Error as IoError, Write};
use std::path::{Path, PathBuf};
use path_macro::path;
pub use burrmod::*;
pub use file::*;
pub use format::*;
pub use item::*;
pub use output::*;
pub use target::*;

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
    pub root: Option<PathBuf>
}

impl Burrxporter {
    pub fn new() -> Self {
        Burrxporter {
            mods: Vec::new(),
            root: None,
        }
    }

    pub fn from_mod<M: Into<BurrMod>>(r#mod: M) -> Self {
        Self::new().with_mod(r#mod)
    }

    /// Adds input module to root collection
    pub fn with_mod<M: Into<BurrMod>>(mut self, r#mod: M) -> Self {
        self.mods.push(r#mod.into());
        self
    }

    /// Sets root path for exports
    pub fn root(mut self, to: &Path) -> Result<Self, ExportError> {
        self.root = Some(to.to_path_buf());
        Ok(self)
    }

    /// Adds output target with configuration
    pub fn export<T: Target>(mut self, to: &Path, mut target: T) -> Result<Self, ExportError> {
        target.export(to, &self);
        Ok(self)
    }

    /// Gets the writer for a file path
    /// Creates the directory and file if it does not exist, truncates if it does
    // todo: consider options for allowing user to plugin their writer, something like a Box<dyn Writer>
    pub fn open_writer(&self, to: &Path) -> Result<impl Write, ExportError> {
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