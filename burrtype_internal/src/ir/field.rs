use std::any::TypeId;
use proc_macro2::Ident;
use syn::TypePath;

#[derive(Clone, Debug)]
pub struct IrType {
    pub path: TypePath,
    pub id: TypeId,
    pub optional: bool,
}

/// A `name: type,` field
#[derive(Clone, Debug)]
pub struct IrNamedField {
    pub ident: Ident,
    pub ty: IrType,
    #[cfg(feature = "docs")]
    pub docs: Option<&'static str>,
}

/// A `type,` field
#[derive(Clone, Debug)]
pub struct IrUnnamedField {
    pub ty: IrType,
    #[cfg(feature = "docs")]
    pub docs: Option<&'static str>,
}

impl IrNamedField {
    pub fn name(&self) -> String {
        self.ident.to_string()
    }
}