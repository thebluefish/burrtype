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
    pub struct NamedStruct {
        /// Type alias allows us to treat one type like another
        /// Here we treat a newtype like its known inner type
        #[burr(type = u64)]
        pub foo: PhantomType,
        /// Rust reserved keywords should resolve properly for other languages
        pub ty: rust_decimal::Decimal,
        /// We need to support optional fields, too
        pub opt: Option<super::Foo>,
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
        TinyTuple(String),
        /// A tuple variant is defined by parenthesis and only types
        Tuple(
            /// Comments give meaning to these nameless types
            Things,
            Things,
        ),
        /// A unit variant has no shape nor fields
        Unit,
        /// Bigger structs can expand to a better format
        BigStruct {
            one: super::Foo,
            /// It doesn't matter where types are, we can reference them
            three: bar::DeepTupleStruct,
            four: Option<NamedStruct>,
            five: TupleStruct,
        },
    }
}

pub mod serde_test {
    use super::inner::*;

    #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    pub enum Stuff {
        Red,
        Two,
    }

    #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
    #[burr(mod = "types")]
    #[serde(rename = "RenamedStruct")]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub struct NamedStruct {
        pub foo: Stuff,
        #[serde(rename = "optional")]
        pub opt: Option<super::Foo>,
        #[serde(flatten)]
        pub more: super::Foo,
    }


    #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
    #[burr(mod = "types")]
    #[serde(rename_all = "snake_case")]
    #[serde(untagged)]
    /// An enum's variants correlate with struct variants
    pub enum UntaggedEnum {
        Struct {
            foo: super::Foo,
            bar: String,
        },
        /// Unit variant will be a string, but the newtype below will also capture a string
        /// In untagged enum representations, serde will attempt them top-to-bottom
        /// So we place more specific cases before general ones
        Unit,
        #[serde(skip)]
        HiddenTuple(String, String, String),
        TinyTuple(String),
        Tuple(
            Stuff,
            Stuff,
        ),
        /// Bigger structs can expand to a better format
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        BigStruct {
            three: bar::DeepTupleStruct,
            four: Option<NamedStruct>,
            #[serde(rename = "six")]
            five: TupleStruct,
        },
    }

    #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
    #[burr(mod = "types")]
    #[serde(rename_all = "PascalCase")]
    #[serde(tag = "t", content = "c")]
    /// An enum's variants correlate with struct variants
    pub enum AdjacentlyTaggedEnum {
        Struct {
            foo: super::Foo,
            bar: String,
        },
        #[serde(skip)]
        HiddenTuple(String, String, String),
        TinyTuple(String),
        Tuple(
            Stuff,
            Stuff,
        ),
        Unit,
        #[serde(rename_all = "UPPERCASE")]
        BigStruct {
            three: bar::DeepTupleStruct,
            four: Option<NamedStruct>,
            #[serde(rename = "six")]
            five: TupleStruct,
        },
    }

    #[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
    #[burr(mod = "types")]
    #[serde(tag = "type")]
    /// An enum's variants correlate with struct variants
    pub enum InternallyTaggedEnum {
        Struct {
            foo: super::Foo,
            bar: String,
        },
        Unit,
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        BigStruct {
            // Currently this only works for enums without tuple variants - the same restriction as internally-tagged enums
            // Awaiting <https://github.com/serde-rs/serde/pull/2567>
            #[serde(flatten)]
            more: super::Foo,
            /// It doesn't matter where types are, we can reference them
            three: bar::DeepTupleStruct,
            four: Option<NamedStruct>,
            #[serde(rename = "six")]
            five: TupleStruct,
        },
    }

}