use quote::ToTokens;
use burrtype::*;
use burrtype_derive::*;

#[derive(Burr)]
pub struct Foo {
    pub four: u64,
}

#[derive(Burr)]
pub struct Bar {
    #[burr(ignore)]
    pub one: u32,
    #[burr(ty(String))] // treats `two` as a String type rather than a Foo type
    pub two: Foo,
    #[burr(flatten)] // replaces `three` with fields from `Foo`
    pub three: Foo,
}

fn main() {
    let fields = <Bar as NamedStructExt>::get_fields();
    for IrNamedField { name, ty} in &fields {
        println!("{name}: {},", ty.to_token_stream());
    }
    // println!("fields: {fields:#?}");
}
