use proc_macro2::{Ident, Literal, TokenStream};
use std::collections::HashMap;
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, Lit, LitByteStr, LitStr, Meta, Token,
    TypePath,
};

pub trait NamedStructExt {
    // fn get_ir() -> IrNamedStruct;
    fn get_fields() -> Vec<IrNamedField> {
        Vec::new()
    }
}

pub trait UnnamedStructExt {
    // fn get_ir() -> IrUnnamedStruct;
    fn get_fields() -> Vec<TypePath> {
        Vec::new()
    }
}

pub trait StructUnionExt {
}

pub struct IrNamedField {
    pub name: Ident,
    pub ty: TypePath,
}

pub struct IrNamedStruct {
    pub name: Ident,
    pub fields: Vec<IrNamedField>,
}

pub struct IrUnnamedStruct {
    pub name: Ident,
    pub fields: Vec<TypePath>,
}

pub struct StructField {
    /// substitute type's members
    pub flatten: bool,
    /// type
    pub ty: TypePath,
}
