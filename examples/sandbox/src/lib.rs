use burrtype::prelude::*;

#[derive(Burr, serde::Serialize, serde::Deserialize, Debug)]
/// A named struct is defined by braces and fields with named
pub struct Foo {
    /// comments work at all levels
    pub one: u32,
    pub two: String,
}

#[derive(Burr, serde::Serialize, serde::Deserialize, Debug)]
pub struct Bar(pub Foo);

/// Aim soon to support #[burrmod] for organizing a set of items within an inline module
/// Note this will not work for file-based modules, only inline ones
pub mod inner {
    pub mod bar {
        #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
        /// strike the earth!
        pub struct DeepTupleStruct(
            /// Why do we care about such things
            pub u64,
        );
    }

    /// This struct doesn't derive Burr and can't be exported directly
    /// It can be serialized/deserialized, so we should support some way to describe it
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct PhantomType(pub u64);

    #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
    pub struct NamedStruct {
        // todo: should this support some sort of type mocking?
        /// Type alias allows us to treat one type like another
        /// Here we treat a newtype like its known inner type
        #[burr(type = u64)]
        pub foo: PhantomType,
        /// Here we treat a third-party type by its known serde representation
        #[burr(type = f64)]
        pub bar: rust_decimal::Decimal,
        /// We need to support optional fields, too
        pub opt: Option<super::Foo>,
    }

    /// A tuple struct is defined by parenthesis and only types
    #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
    pub struct TupleStruct(pub u32, pub super::Foo);

    #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
    /// A unit struct has no shape nor fields
    pub struct UnitStruct;

    #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
    /// An enum's variants correlate with struct variants
    pub enum Enum {
        /// A struct variant is defined by braces and fields with named
        Struct {
            /// An inline comment
            foo: super::Foo,
            bar: String,
        },
        TinyTuple(String),
        /// A tuple variant is defined by parenthesis and only types
        Tuple(
            /// Give some meaning to these nameless types
            u32,
            u64,
        ),
        /// A unit variant has no shape nor fields
        Unit,
        /// Bigger structs can expand to a better format
        BigStruct {
            /// It doesn't matter where types are, we can reference them
            one: bar::DeepTupleStruct,
            two: Option<NamedStruct>,
            three: TupleStruct,
            four: super::Foo,
        },
    }
}