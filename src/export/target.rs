use crate::export::Burrxporter;
use std::path::Path;

pub trait Target {
    fn export(&mut self, to: &Path, exporter: &Burrxporter);
}

