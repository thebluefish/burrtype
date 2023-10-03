![crates.io](https://img.shields.io/crates/v/burrtype.svg)
![docs.rs](https://img.shields.io/docsrs/burrtype)

Exports your types to other languages, currently supporting TypeScript.

## Install

Add to your `Cargo.toml`

```toml
[dependencies]
burrtype = { version = "0.2", features = ["typescript"] }
```

## Use

Decorate your types with the `Burr` derive macro.

```rust
use burrtype::prelude::*;

#[derive(Burr)]
pub struct Foo(u64);

#[derive(Burr)]
pub struct Bar(Foo);
```

For types that don't derive `Burr`, such as those from third-party crates, use an attribute to treat fields as different types:

```rust
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct PhantomType(pub u64);

    #[derive(Burr)]
    pub struct Foo {
        /// We will treat this type as its inner type for simplicity
        #[burr(type = u64)]
        pub foo: PhantomType,
    }
```

In your `build.rs` or another binary, we can create an exporter, add your types to be exported, and then write those types to your desired language(s).

```rust
Burrxporter::new()
    // auto-registered types depended on by your types will be exported here
    .with_default_mod("common")
    // create a virtual module and add our root type to it
    .with_mod(BurrMod::new("bar")
        .with_type::<Bar>()
    )
    // outputs ./ts/common.ts and ./ts/bar.ts
    .export("ts", TypeScript::new())?
    // bundles all exported types into a single file
    .export("out/bundled.ts", TypeScript::new().with_file_map(ModFileMap::Inline))?
;
```

The exporter and its components behave as builder patterns. Builder options typically use the suffix `with_`.

Inputs are organized into modules, which represent collections of items, including other modules. This module tree typically translates to the file structure being produced, with one file per module, but language constraits, features, or options can change this behavior.

See these concepts in action in [the example](examples/sandbox/).

---

## Desired features going forward

- Replace panics with errors
- tests

### Rust

- Support the same set of common features as TypeScript
- Generic types?