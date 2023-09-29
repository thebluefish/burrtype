use burrtype::prelude::*;
use path_macro::path;
use std::env;

#[derive(Reflect)]
pub struct Foo {
    pub one: u64,
    pub two: String,
}

pub mod inner {
    pub mod bar {
        #[derive(burrtype::Reflect)]
        pub struct DeepTupleStruct(u64);
    }

    #[derive(burrtype::Reflect)]
    pub struct NamedStruct {
        foo: u64,
        bar: u128,
    }

    /// References an item from another module
    #[derive(burrtype::Reflect)]
    pub struct TupleStruct(u32, super::Foo);

    #[derive(burrtype::Reflect)]
    pub struct UnitStruct;

    #[derive(burrtype::Reflect)]
    pub enum Enum {
        Unit,
        Tuple(u32, u64),
        Struct {
            foo: super::Foo,
            bar: String,
        },
        BigStruct {
            one: bar::DeepTupleStruct,
            two: String,
            three: f64,
            four: super::Foo,
        },
    }
}

// todo: re-add proc macro crate, remove most IR generation stuff
// todo: focus the macros to primarily control options not present in Reflect, probably similarly to IR output but outputting a struct of options
// todo: for example, to remap types from one to another during export, at the type level instead of having to specify the remap in a builder


/// We should be able to call an API writer to export items
/// The writer operates via builder patterns, with various options to control export
/// We must provide the builder with all items we wish to export
/// Then export files
fn main() -> anyhow::Result<()> {
    let cwd = if let Ok(dir) = env::var("CARGO_MANIFEST_DIR") { dir.into() } else { env::current_dir().unwrap() };

    println!("--- export ---\n");

    Burrxporter::new()
        // Build inputs
        // The options associated with inputs should correspond to common idioms found in most languages
        .with_mod(BurrMod::new("inner")
            .with_mod(BurrMod::new("deep")
                .with_mod(BurrMod::new("foo")
                    .with_type::<Foo>()
                )
            )
            .with_mod(BurrMod::new("deep2")
                .with_mod(BurrMod::new("bar")
                    // todo: strongly order these
                    // currently the order of their output is random due to hashmap
                    .with_type::<inner::TupleStruct>()
                    .with_type::<inner::bar::DeepTupleStruct>()
                    .with_type::<inner::Enum>()
                )
            )
        )
        // .with_mod(BarMod)
        // Builds and writes outputs
        // The options associated with outputs should correspond to features specific to a language
        .with_root(&path!(cwd / "target" / "api"))
        // exports each root-level mod to root/ts/{target}[.ts]
        .export(&path!("ts"), TypeScript::new())?
        // exports all modules together to root/{target}[.ts]
        .export(&path!("bundled.ts"), TypeScript::new()
            .with_file_map(ModFileMap::Inline)
        )?
    ;

    println!("\n--- done! ---");
    Ok(())
}