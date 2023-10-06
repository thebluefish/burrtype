use crate::export::{BurrMod};
use std::path::PathBuf;
use burrtype_internal::ir::IrItem;

#[derive(Default, Debug)]
pub struct TsFile {
    // identifies this module when generating indices and imports
    pub name: String,
    // the directory and file we are exporting to
    pub target: PathBuf,
    // our types being exported
    pub items: Vec<IrItem>,
    // inline modules that need to be handled separately
    pub mods: Vec<BurrMod>,
}

impl From<BurrMod> for TsFile {
    fn from(value: BurrMod) -> Self {
        let mut items = Vec::new();
        items.extend(value.exports.iter().map(|id| value.types.get(id).unwrap().clone()));
        items.extend(value.auto_exports.iter().map(|id| value.types.get(id).unwrap().clone()));

        TsFile {
            name: value.name.clone(),
            target: PathBuf::from(value.name).with_extension("ts"),
            items,
            mods: value.children,
        }
    }
}