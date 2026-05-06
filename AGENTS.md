# Agent Guidelines

This is a Windows-only Rust library (`acevo-shared-memory`) that provides a safe,
typed interface for reading the Assetto Corsa Evo simulator's live telemetry via
Windows named shared memory. Read this before making changes.

## Build and check

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo doc --no-deps
cargo test --doc

# run the examples (AC Evo must be running)
cargo run --example monitor
cargo run --example dashboard

# verify serde feature compiles
cargo build --features serde
```

There is no cross-platform CI; the crate is Windows-only and will not compile
on Linux/macOS. Minimum supported Rust version: **1.85** (`rust-version` in
`Cargo.toml`).

## Architecture

The crate is structured around the following layers:

```
src/
  lib.rs                        — public re-exports, crate-level docs
  mapper.rs                     — ACEvoSharedMemoryMapper: opens and owns the three OS handles
  bindings/
    mod.rs                      — include!(concat!(env!("OUT_DIR"), "/bindings.rs"))
    source/wrapper.hpp          — C++ source fed to bindgen; edit this, never the generated file
  views/
    view.rs                     — View<'a, T>: generic borrowed-or-owned page wrapper
    physics_view.rs             — PhysicsView  = View<SPageFilePhysics>
    graphics_view.rs            — GraphicsView = View<SPageFileGraphicEvo>
    static_view.rs              — StaticView   = View<SPageFileStaticEvo>
    storage.rs                  — Storage<'a, T>: internal Borrowed / Owned enum
    utils.rs                    — parse_c_str helper
  wrappers/                     — idiomatic Rust enums wrapping C integer typedef enums
build.rs                        — generates bindings.rs into $OUT_DIR, patches serde attributes
examples/
  monitor.rs                    — terminal text dashboard (polling, ANSI clear-screen)
  dashboard.rs                  — full ratatui TUI with 8 tabs, mouse support
```

`build.rs` runs two steps: `generate_bindings()` writes `$OUT_DIR/bindings.rs` via
bindgen, then `patch_serde()` post-processes the file to inject
`#[cfg_attr(feature = "serde", …)]` attributes before every `pub struct` and before
array fields whose element count exceeds 32 (which require `serde_arrays`).

## Key types

- `ACEvoSharedMemoryMapper` — the single entry point; call `::open()` once per process.
- `View<'a, T>` — foundation for all three views; provides `raw()`, `inner()`, `snapshot()`.
- `PhysicsView`, `GraphicsView`, `StaticView` — type aliases over `View<'a, T>` with
  domain-specific accessor methods.
- `ACEvoStatus`, `ACEvoSessionType`, `ACEvoFlagType`, `ACEvoCarLocation`,
  `ACEvoEngineType`, `ACEvoStartingGrip` — typed Rust enums with strum derives
  and `value() -> i32` round-trip conversion.

## Shared-memory segments

| Named object                 | Struct               | View           | Update rate    |
| ---------------------------- | -------------------- | -------------- | -------------- |
| `Local\acevo_pmf_physics`    | `SPageFilePhysics`   | `PhysicsView`  | Every sim step |
| `Local\acevo_pmf_graphics`   | `SPageFileGraphicEvo`| `GraphicsView` | Every frame    |
| `Local\acevo_pmf_static`     | `SPageFileStaticEvo` | `StaticView`   | Once at load   |

## Invariants to preserve

- **`bindings.rs` lives in `$OUT_DIR`, not in the source tree** — `build.rs` writes
  it to `PathBuf::from(env::var("OUT_DIR")).join("bindings.rs")` and `bindings/mod.rs`
  consumes it with `include!(concat!(env!("OUT_DIR"), "/bindings.rs"))`. Never create
  or edit `src/bindings/bindings.rs`; it does not exist in the source tree.
  To change the generated types, edit `src/bindings/source/wrapper.hpp` and
  re-run `cargo build`.
- **`View` lifetimes** — `View<'a, T>` borrows from the mapper for lifetime `'a`.
  `snapshot()` copies into a `Box<T>` and returns `View<'static, T>`. Do not
  weaken this lifetime relationship.
- **Enum forward-compatibility** — every enum must carry an `Unknown(i32)` or
  `Other(i32)` catch-all with `#[strum(disabled)]` so that unrecognised protocol
  values do not panic and `EnumIter` only yields known variants.
- **`value() -> i32`** — every enum must implement this method for round-trip
  conversion back to the raw protocol integer.
- **`unsafe` inside `Mapper::*_raw()`** — these methods call
  `SharedMemoryLink::get()` which is inherently `unsafe` due to cross-process
  aliasing. The `unsafe` block stays internal; the public safe wrappers
  (`physics()`, `graphics()`, `static_data()`) are the intended surface.
- **serde large-array patching** — `serde` only has built-in impls for arrays up
  to `[T; 32]`. `build.rs`'s `patch_serde()` step detects fields with array size
  > 32 and injects `#[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]`
  before them. If you add new large-array fields to `wrapper.hpp`, verify
  `cargo build --features serde` still compiles.
- **`#pragma pack(4)` alignment** — the C structs use 4-byte maximum alignment.
  If you add new sub-structs or fields to `wrapper.hpp`, verify the static assert
  sizes still hold before committing.

## Enums and protocol constants

C `typedef int ACEVO_*` enumerations are wrapped in `src/wrappers/`. Each file
contains exactly one enum. The naming convention is `ACEvo<Name>` in PascalCase
(e.g. `ACEvoStatus`, `ACEvoFlagType`). All enums must derive:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, EnumIter, IntoStaticStr)]
```

## Features

| Feature | Activates | Effect |
| ------- | --------- | ------ |
| `serde` | `dep:serde`, `dep:serde_arrays` | Derives `Serialize` on all views, raw structs, and enums; derives `Deserialize` on snapshot views (`View<'static, T>`) and enums |

There is no `default` feature and no `serialization` alias — `serde` is the only
non-default feature.

## Dependencies

**Runtime**

- `win-shared-memory = "0.1"` — Windows named file-mapping RAII wrapper.
- `strum = "0.28"` with `features = ["derive"]` — `Display`, `EnumString`,
  `EnumIter`, `IntoStaticStr` derives on all wrapper enums.
- `serde = "1"` *(optional)* — `Serialize`/`Deserialize` derives.
- `serde_arrays = "0.2"` *(optional, activated by `serde`)* — serialization
  support for const-generic arrays larger than 32 elements (e.g. `[i8; 33]`).

**Build**

- `bindgen = "0.72"` — generates `bindings.rs` from `wrapper.hpp` at build time.

**Dev / examples only**

- `ratatui = "0.30"` — TUI framework used by `examples/dashboard.rs`; pulls in
  crossterm as `ratatui::crossterm` (no separate `crossterm` dep needed).

Do not add further dependencies without a strong reason.

## What NOT to do

- Do not add Linux/macOS shims or `cfg` gates; this crate is Windows-only by design.
- Do not change the `unsafe` surface of `Mapper::*_raw()` to safe — the
  cross-process aliasing hazard is real.
- Do not add new public API without corresponding `///` doc comments and at least
  one `# Example` block.
- Do not change the coding style if not explicitly requested.
