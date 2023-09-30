use crate::export::Burrxporter;
use std::path::Path;

pub trait Target {
    fn export(self, to: &Path, exporter: &Burrxporter);
}

