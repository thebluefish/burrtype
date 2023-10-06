use burrtype::prelude::*;

#[derive(Burr, serde::Serialize, serde::Deserialize, Debug)]
/// A named struct is defined by braces and fields with named
pub struct Foo {
    /// comments work at all levels
    /// Even below when this field is substituted in using #[serde(flatten)]
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
        #[burr(mod = "deep/types")]
        /// We can assign a module at the type level
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
    #[burr(mod = "types")]
    #[serde(rename = "RenamedStruct")]
    // #[serde(rename(serialize = "SameNameAlsoWorks", deserialize = "SameNameAlsoWorks"))]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub struct NamedStruct {
        /// Type alias allows us to treat one type like another
        /// Here we treat a newtype like its known inner type
        #[burr(type = u64)]
        pub foo: PhantomType,
        /// Rust reserved keywords should resolve properly for other languages
        pub r#type: rust_decimal::Decimal,
        /// We need to support optional fields, too
        #[serde(rename = "optional")]
        pub opt: Option<super::Foo>,
        #[serde(flatten)]
        pub more: super::Foo,
    }

    /// A tuple struct is defined by parenthesis and only types
    #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
    #[burr(mod = "types")]
    pub struct TupleStruct(pub u32, pub super::Foo);

    #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
    #[burr(mod = "types")]
    /// A unit struct has no shape nor fields
    pub struct UnitStruct;

    /// The simplest enum of all unit types
    #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
    pub enum Things {
        One,
        Two,
    }

    #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
    #[burr(mod = "types")]
    /// An enum's variants correlate with struct variants
    pub enum Enum {
        /// A struct variant is defined by braces and fields with named
        Struct {
            /// An inline comment
            foo: super::Foo,
            bar: String,
        },
        #[serde(skip)]
        HiddenTuple(String, String, String),
        TinyTuple(String),
        /// A tuple variant is defined by parenthesis and only types
        Tuple(
            /// Give some meaning to these nameless types
            Things,
            Things,
        ),
        /// A unit variant has no shape nor fields
        Unit,
        /// Bigger structs can expand to a better format
        BigStruct {
            // This works in theory, but needs <https://github.com/serde-rs/serde/pull/2567>
            // #[serde(flatten)]
            // more: super::Foo,
            /// It doesn't matter where types are, we can reference them
            three: bar::DeepTupleStruct,
            four: Option<NamedStruct>,
            #[serde(rename = "six")]
            five: TupleStruct,
        },
    }
}