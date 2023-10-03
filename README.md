![crates.io](https://img.shields.io/crates/v/burrtype.svg)
![docs.rs](https://img.shields.io/docsrs/burrtype)

Exports your types to other languages, currently supporting TypeScript.

## Install

Add to your `Cargo.toml`

```toml
[dependencies]
burrtype = { version = "0.2", features = ["typescript"] }
```

## Usage

### Preparing your types

Decorate your types with the `Burr` derive macro.

```rust
use burrtype::prelude::*;

#[derive(Burr)]
pub struct Foo(u64);

#[derive(Burr)]
pub struct Bar(Foo);
```

### Exporting your types

In your `build.rs` or another binary, we can create an exporter and optionally configure it. The exporter and its components behave as builder patterns.

```rust
Burrxporter::new()
    // auto-registered types depended on by your types will be exported here
    .with_default_mod("common")
```

Add your types using modules to organize them. This module tree typically translates to the file structure being produced - with one file per module - but language constraits, features, or options can change this behavior.

```rust
    .with_mod(BurrMod::new("bar")
        .with_type::<Bar>()
    )
```

Export to one or more targets.

```rust
    // outputs ./ts/common.ts and ./ts/bar.ts
    .export("ts", TypeScript::new())?
    // bundles all exported types into a single file
    .export("out/bundled.ts", TypeScript::new().with_file_map(ModFileMap::Inline))?
```

See these concepts in action in [the example](examples/sandbox/).

### 3rd-party types

For types that don't derive `Burr`, such as those from third-party crates, use an attribute to treat fields as another type:

```rust
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PhantomType(pub u64);

#[derive(Burr)]
pub struct Foo {
    #[burr(type = u64)]
    pub foo: PhantomType,
}
```

Alternatively, you can register a 3rd-party type as a string name representing that type in the target language.

```rust
    .export("api", TypeScript::new()
        .with_type_name::<rust_decimal::Decimal>("number")
    )?
```

---

## Desired features going forward

- Replace panics with errors
- tests

- Rust target
  - Support the same set of common features as TypeScript
  - Generic types?