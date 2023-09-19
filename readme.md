## Installation

```shell
cargo add burrtype burrtype_derive
```

Or add to your `Cargo.toml`

```toml
burrtype = "0.1"
burrtype_derive = "0.1"
```

## Usage

Decorate your types with the `Burr` derive macro. Currently this macro supports structs.

```rust
#[derive(burrtype_derive::Burr)]
pub struct Foo;
```

In your `build.rs` or another binary, we can create an exporter, add your types to be exported, and then write those types to your desired languages.

```rust
Burrxporter::new()
    .with_mod(BurrMod::new("inner").with_type::<Foo>())
    .with_root(&path!(cwd / "target" / "api"))?
    .export(&path!("ts"), TypeScript::new())?
    ;
```

The exporter and its components behave as builder patterns. Builder options typically use the suffix `with_`.

Inputs are organized into modules, which represent collections of items. Modules can nest other modules. Modules typically represent the file structure being produced, with one file per module, but language constraits, features, or options can change this behavior.

---

## Desired features going forward

- Replace panics with errors

### Typescript

- Generate combined exports for indices
  - Flexible configuration to decide what to re-export
- `bevy_reflect` support - this may or may not end up replacing the primary `Burr` derive macro

### Rust

- Support the same set of common features as TS
- Support generic types