use crate::Foo;
use crate::inner::{bar::DeepTupleStruct, TupleStruct};

#[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[burr(mod = "core/serde")]
pub enum Stuff {
    Red,
    Two,
}

#[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
#[burr(mod = "serde")]
#[serde(rename = "RenamedStruct")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct NamedStruct {
    pub foo: Stuff,
    #[serde(rename = "optional")]
    pub opt: Option<Foo>,
    #[serde(flatten)]
    pub more: Foo,
}


#[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
#[burr(mod = "serde")]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
/// An enum's variants correlate with struct variants
pub enum UntaggedEnum {
    Struct {
        foo: Foo,
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
        three: DeepTupleStruct,
        four: Option<NamedStruct>,
        #[serde(rename = "six")]
        five: TupleStruct,
    },
}

#[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
#[burr(mod = "serde")]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "t", content = "c")]
/// An enum's variants correlate with struct variants
pub enum AdjacentlyTaggedEnum {
    Struct {
        foo: Foo,
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
        three: DeepTupleStruct,
        four: Option<NamedStruct>,
        #[serde(rename = "six")]
        five: TupleStruct,
    },
}

#[derive(burrtype::Burr, serde::Serialize, serde::Deserialize, Debug)]
#[burr(mod = "serde")]
#[serde(tag = "type")]
/// An enum's variants correlate with struct variants
pub enum InternallyTaggedEnum {
    Struct {
        foo: Foo,
        bar: String,
    },
    Unit,
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    BigStruct {
        // Currently this only works for enums without tuple variants - the same restriction as internally-tagged enums
        // Awaiting <https://github.com/serde-rs/serde/pull/2567>
        #[serde(flatten)]
        more: Foo,
        /// It doesn't matter where types are, we can reference them
        three: DeepTupleStruct,
        four: Option<NamedStruct>,
        #[serde(rename = "six")]
        five: TupleStruct,
    },
}
