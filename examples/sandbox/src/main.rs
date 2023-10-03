use sandbox::*;
use burrtype::prelude::*;

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
        .with_mod(BurrMod::new("things")
            .with_mod(BurrMod::new("inner")
                .with_mod(BurrMod::new("bar")
                    // todo: strongly order these
                    // todo: currently the order of their output is random due to hashmap
                    // We only need to include the root types we are trying to export
                    .with_type::<Bar>()
                    .with_type::<inner::Enum>()
                    .with_type::<inner::UnitStruct>()
                )
            )
        )
        // Builds and writes outputs
        // The options associated with outputs should correspond to features specific to a language
        // exports each root-level mod to root/ts/{target}[.ts]
        .export("test-client/src/api", TypeScript::new().with_file_map(ModFileMap::DecomposeTop))?
        // Root can provide a common output location for future exports
        // todo: Ensure absolute paths and relative paths beginning with an explicit dot don't get the root added
        .with_root("out/api")
        .export("api", TypeScript::new())?
        // exports all modules together to root/{target}[.ts]
        .export("bundled.ts", TypeScript::new().with_file_map(ModFileMap::Inline))?
    ;

    println!("\n--- done! ---");
    Ok(())
}