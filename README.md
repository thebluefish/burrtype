![crates.io](https://img.shields.io/crates/v/burrtype.svg)
![docs.rs](https://img.shields.io/docsrs/burrtype)

Exports your types to other languages, currently supporting TypeScript.

This crate primarily targets compatibility with the `serde` framework and its representations. Compatibility with `#[serde]` derive macro attributes are offered behind the default `serde_compat` feature.

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

Use the optional `#[burr(mod)]` attribute to assign your type to a module automatically. Here, `Foo` will be automatically included in the specified module if not explicitly included during export.

```rust
use burrtype::prelude::*;

#[derive(Burr)]
#[burr(mod = "common/types")]
pub struct Foo(u64);
```

### Exporting your types

In your program's `main()` or (preferably) another binary, we can create an exporter and optionally configure it. The exporter and its components behave as builder patterns.

```rust
Burrxporter::new()
```

Add your types using modules to organize them. This module tree typically translates to the file structure being produced - with one file per module - but language constraits, features, or options can change this behavior.

```rust
    .with_mod(BurrMod::new("bar")
        .with_type::<Bar>()
    )
```

Resolve type dependencies, which exports types your types depend on. They will be added to either the type's chosen module or the given default module. This step is unnecessary if you explicitly include all types needed to describe your data.

```rust
    .resolve_exports("common")
```

Alternatively, you can automatically include **all** types marked with the `#[burr(mod = "path")]` attribute. This can side-step the need to manually build your output modules.

```rust
    .resolve_all("common")
```

Export to one or more targets.

```rust
    // outputs ./ts/common.ts and ./ts/bar.ts
    .export("ts", TypeScript::new())?
    // bundles all exported types into a single file
    .export("out/bundled.ts", TypeScript::new().with_file_map(ModFileMap::Inline))?
```

See these concepts in action in [the examples](examples/).

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
