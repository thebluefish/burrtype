use std::path::Path;
use crate::export::Burrxporter;
use super::{BurrMod, Burrmatter};

pub trait Target {
    fn export(&mut self, to: &Path, exporter: &Burrxporter);
}

