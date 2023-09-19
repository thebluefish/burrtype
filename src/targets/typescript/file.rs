use crate::export::{BurrMod, Item};
use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct TsFile {
    // identifies this module when generating indices and imports
    pub name: String,
    // the directory and file we are exporting to
    pub target: PathBuf,
    // our types being exported
    pub items: Vec<Item>,
    // inline modules that need to be handled separately
    pub mods: Vec<BurrMod>,
}

impl From<BurrMod> for TsFile {
    fn from(value: BurrMod) -> Self {
        TsFile {
            name: value.name.clone(),
            target: PathBuf::from(value.name).with_extension("ts"),
            items: value.items,
            mods: value.children,
        }
    }
}