use burrtype::prelude::*;

#[derive(Reflect)]
/// The #[burr] attribute allows us to auto-include this type
/// Later, we may support configuring target modules and the like via this attribute
#[burr]
pub struct Foo {
    pub one: u64,
    pub two: String,
}

#[derive(Reflect)]
pub struct Bar(Foo);

/// Aim soon to support #[burrmod] for organizing a set of items within an inline module
/// Note this will never work for file-based modules, only inline ones
pub mod inner {
    pub mod bar {
        #[derive(burrtype::Reflect)]
        #[burrtype::burr]
        /// strike the earth!
        pub struct DeepTupleStruct(
            /// Why do we care about such things
            u64
        );
    }

    #[derive(burrtype::Reflect)]
    #[burrtype::burr]
    /// A named struct is defined by braces and fields with named
    pub struct NamedStruct {
        /// Builtin types are supported and usually converted to primitives
        foo: u64,
        /// Types can be referenced from anywhere, so long as they're Reflect
        /// Type overrides can bypass the requirement for Reflect, but are per-language features
        bar: super::Foo,
    }

    /// A tuple struct is defined by parenthesis and only types
    #[derive(burrtype::Reflect)]
    #[burrtype::burr]
    pub struct TupleStruct(u32, super::Foo);

    #[derive(burrtype::Reflect)]
    #[burrtype::burr]
    /// A unit struct has no shape nor fields
    pub struct UnitStruct;

    #[derive(burrtype::Reflect)]
    #[burrtype::burr]
    /// An enum's variants correlate with struct variants
    pub enum Enum {
        /// A struct variant is defined by braces and fields with named
        Struct {
            /// An inline comment
            foo: super::Foo,
            bar: String,
        },
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
            two: NamedStruct,
            three: TupleStruct,
            four: super::Foo,
        },
    }
}

/// This example attempts to demonstrate the various features for exporting to a TypeScript target
fn main() -> anyhow::Result<()> {
    println!("--- export ---\n");

    Burrxporter::new()
        // Build inputs
        // The options associated with inputs should correspond to common idioms found in most languages
        // set the module where unspecified dependent types are exported to
        .with_default_mod("common")
        // organize types into a tree of modules
        // these will typically (but not always) correspond to the exported file tree
        .with_mod(BurrMod::new("inner")
            .with_mod(BurrMod::new("deep2")
                .with_mod(BurrMod::new("bar")
                    // todo: strongly order these
                    // todo: currently the order of their output is random due to hashmap
                    // We only need to include the root types we are trying to export
                    .with_type::<inner::Enum>()
                    .with_type::<inner::UnitStruct>()
                )
            )
        )
        // Builds and writes outputs
        // The options associated with outputs should correspond to features specific to a language
        .with_root("out/api")
        // exports each root-level mod to root/ts/{target}[.ts]
        .export("ts", TypeScript::new())?
        // exports all modules together to root/{target}[.ts]
        .export("bundled.ts", TypeScript::new()
            .with_file_map(ModFileMap::Inline)
        )?
    ;

    println!("\n--- done! ---");
    Ok(())
}