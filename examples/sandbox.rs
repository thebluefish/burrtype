use burrtype::prelude::*;
use burrtype_derive::*;
use quote::ToTokens;

/// We should be able to declare a `Burr` derive on types that implements appropriate traits on these types
#[derive(Burr)]
pub struct Foo {
    pub four: u64,
}

/// We should be able to control field representation by using the #[burr] attribute
#[derive(Burr)]
pub struct Bar {
    /// #[burr(ignore)] and optionally #[serde(ignore)] should cause this field to not appear in IR
    #[burr(ignore)]
    pub one: u32,
    /// `#[burr(ty(Y))] name: T` should replace `T` with `Y` in IR
    #[burr(ty(String))]
    pub two: Foo,
    /// #[burr(flatten)] should replace this field with the contents of its type
    /// This will only be valid when the field's type is a Named Struct
    #[burr(flatten)]
    pub three: Foo,
}

/// We should be able to control module representation in final output by using the #[burr] attribute
/// #[burrmod] amd its variants should produce a `struct BurrMod` that exposes information about this module and its items
/// #[burrmod(ir = T)] or #[burrmod(ir = "T")] will instead produce a `struct T`
/// #[burrmod(inline)] should produce its `BurrMod` and related impls inside the module
/// #[burrmod(flatten)] should produce its inner items without the module declaration
#[burrmod]
pub mod foo {
    #[burrtype_derive::burrmod(inline, flatten, ir = WiddlyMod)]
    pub mod bar {
        #[derive(burrtype_derive::Burr)]
        pub struct DeepTupleStruct(u64);
    }

    #[burrtype_derive::burrmod(inline)]
    pub mod deep {
        #[derive(burrtype_derive::Burr)]
        pub struct DeepStruct {
            foo: u64,
            bar: u128,
        }
    }

    pub enum ThisShouldNotFail {

    }

    /// We should be able to reference types with any valid visibility
    #[derive(burrtype_derive::Burr)]
    pub struct TupleStruct(u32, super::Foo);

}

/// We should be able to call an API writer to export items
/// The writer operates via builder patterns, with various options to control export
/// We must provide the builder with all items we wish to export
/// Then consume the builder and export file(s)
/// Example API:
/// ```
/// Burrxporter::new() // Invoke builder to write definitions for our types
///     .root([cwd, "defs"].into()) // Directory to export modules to
///     .with_mod(::common::BurrMod::ts() // We wish to write a TypeScript module
///         .target("shared.ts") // Rename export from default "common.ts"
///         .with_item(::foo::BurrMod) // Add unrelated mod as a child
///         .with_items(::bar::BurrMod::children()) // Merge mod's items
///         .with_items([::child1::BurrMod, ::defs::MyStruct, ::defs::MyEnum]) // Add unrelated items as children
///     )
///     // Exports a module with default settings provided by the mod
///     // Note that since the default export "shared.ts" is already used, this mod's items will be merged
///     .with_mod(::shared::BurrMod::ts())
///     .export()?; Compile and write modules
///```
fn main() {
    let name = <FooModIr as ModExt>::name();
    let items = <FooModIr as ModExt>::items();
    println!("mod {name} {{");
    for item in items {
        println!("  {},", item.name());
    }
    println!("}}");

    let name = <foo::bar::WiddlyMod as ModExt>::name();
    let items = <foo::bar::WiddlyMod as ModExt>::items();
    println!("mod {name} {{");
    for item in items {
        println!("  {},", item.name());
    }
    println!("}}");

    let fields = <Bar as NamedStructExt>::fields();
    for IrNamedField { name, ty} in &fields {
        println!("{name}: {},", ty.to_token_stream());
    }
    // println!("fields: {fields:#?}");
}