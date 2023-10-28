use crate::Foo;

pub mod bar {
    #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
    #[burr(mod = "inner/core")]
    pub struct DeepTupleStruct(
        pub u64,
    );
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PhantomType(pub u64);

#[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
#[burr(mod = "inner")]
pub struct NamedStruct {
    #[burr(type = u64)]
    pub foo: PhantomType,
    pub ty: rust_decimal::Decimal,
    pub opt: Option<Foo>,
}

#[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
#[burr(mod = "inner")]
pub struct TupleStruct(pub u32, pub Foo);

#[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
#[burr(mod = "inner")]
pub struct UnitStruct;

#[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
#[burr(mod = "inner/core")]
pub enum Things {
    One,
    Two,
}

#[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
#[burr(mod = "inner")]
pub enum Enum {
    Struct {
        foo: Foo,
        bar: String,
    },
    TinyTuple(String),
    Tuple(
        Things,
        Things,
    ),
    Unit,
    BigStruct {
        one: Foo,
        three: bar::DeepTupleStruct,
        four: Option<NamedStruct>,
        five: TupleStruct,
    },
}

#[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
#[burr(mod = "inner")]
pub struct Vecs {
    pub one: Vec<u32>,
    pub two: Option<Vec<u32>>,
}
