mod inner;
mod serde_test;

use burrtype::prelude::*;

#[derive(Burr, serde::Serialize, serde::Deserialize, Debug)]
pub struct Foo {
    pub one: u32,
    pub two: String,
}

#[derive(Burr, serde::Serialize, serde::Deserialize, Debug)]
pub struct Bar(pub Foo);