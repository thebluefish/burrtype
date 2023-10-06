// `#[warn(unused_imports)]` false positive - this is needed to bring types into scope
#[allow(unused_imports)]
use auto_types::*;
use burrtype::prelude::*;

/// This example demonstrates exporting your types via their configured target modules at the type-level
/// Unlike `sandbox`, it includes no root types in the module tree
/// All types are collected directly from the crate by the exporter
fn main() -> anyhow::Result<()> {
    println!("--- export ---\n");

    Burrxporter::new()
        .resolve_all("common")
        .with_root("out/api")
        .export("ts", TypeScript::new()
            .with_type_name::<rust_decimal::Decimal>("number")
        )?
        .export("bundled.ts", TypeScript::new()
            .with_file_map(ModFileMap::Inline)
            .with_type_name::<rust_decimal::Decimal>("number")
        )?
    ;

    println!("\n--- done! ---");
    Ok(())
}