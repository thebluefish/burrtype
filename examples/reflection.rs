use std::path::PathBuf;
use burrtype::prelude::*;
use bevy_reflect::prelude::*;
use bevy_reflect::TypeRegistry;

#[derive(Reflect)]
pub struct Foo(pub usize);

pub mod inner {
    pub mod bar {
        #[derive(bevy_reflect::Reflect)]
        pub struct DeepTupleStruct(u64);
    }

    #[derive(bevy_reflect::Reflect)]
    pub struct NamedStruct {
        foo: u64,
        bar: u128,
    }

    #[derive(bevy_reflect::Reflect)]
    pub struct TupleStruct(u32, super::Foo);

    #[derive(bevy_reflect::Reflect)]
    pub struct UnitStruct;

    #[derive(bevy_reflect::Reflect)]
    pub enum Enum {
        Unit,
        Tuple(u32, u64),
        Struct {
            foo: u64,
            bar: String,
        }
    }
}

pub struct BurrMod {
    name: String,
    types: TypeRegistry,
    children: Vec<BurrMod>,
}

pub struct Burrxporter {
    pub mods: Vec<BurrMod>,
    pub root: Option<PathBuf>,
}


/// A test of how we might leverage `bevy_reflect` to achieve similar results
/// Ideally it should cover a broader set of available types than our initial prototype
/// Inline Module-level derive will need to be adapted to support this case, somehow optionally
fn main() -> anyhow::Result<()> {

    Ok(())
}