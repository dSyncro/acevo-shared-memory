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
```

There is no cross-platform CI; the crate is Windows-only and will not compile
on Linux/macOS.

## Architecture

The crate is structured around three layers:

```
src/
  lib.rs              — public re-exports, crate-level docs
  mapper.rs           — ACEvoSharedMemoryMapper: opens and owns the three OS handles
  bindings/           — bindgen output (auto-generated from wrapper.hpp, do not edit)
  views/
    view.rs           — View<'a, T>: generic borrowed-or-owned page wrapper
    physics_view.rs   — PhysicsView  = View<SPageFilePhysics>
    graphics_view.rs  — GraphicsView = View<SPageFileGraphicEvo>
    static_view.rs    — StaticView   = View<SPageFileStaticEvo>
    storage.rs        — Storage<'a, T>: internal Borrowed / Owned enum
    utils.rs          — parse_c_str helper
  wrappers/           — idiomatic Rust enums wrapping C integer typedef enums
```

The `examples/monitor.rs` binary is a standalone polling loop intended for
manual testing — it is not part of the library API.

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

- **`bindings/bindings.rs` is auto-generated** — it is produced by `build.rs` via
  `bindgen`. Never edit it by hand; changes will be overwritten on the next build.
  Edit `src/bindings/source/wrapper.hpp` instead and re-run `cargo build`.
- **`View` lifetimes** — `View<'a, T>` borrows from the mapper for lifetime `'a`.
  `snapshot()` copies into a `Box<T>` and returns `View<'static, T>`. Do not
  weaken this lifetime relationship.
- **Enum forward-compatibility** — every enum must carry an `Unknown(i32)` or
  `Other(i32)` catch-all with `#[strum(disabled)]` so that unrecognised protocol
  values do not panic and `EnumIter` only yields known variants.
- **`value() -> i32`** — every enum must implement this method for round-trip
  conversion back to the raw protocol integer.
- **`unsafe` on `Mapper::*_raw()`** — these methods call `SharedMemoryLink::get()`
  which is inherently `unsafe` due to cross-process aliasing. Keep the `unsafe`
  block internal; the public safe methods (`physics()`, `graphics()`, `static_data()`)
  are the correct surface.
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

## Dependencies

- `win-shared-memory = "0.1"` — Windows named file-mapping RAII wrapper.
- `strum = "0.28"` with `features = ["derive"]` — enum string/iter derives.
- `thiserror = "2"` — error derivation (re-exported via `win-shared-memory`).
- `serde = "1"` — optional, gated behind the `serde` / `serialization` features.
- `bindgen = "0.72"` — build dependency for generating `bindings.rs`.

Do not add further dependencies without a strong reason.

## What NOT to do

- Do not edit `src/bindings/bindings.rs` directly — it is auto-generated.
- Do not add Linux/macOS shims or `cfg` gates; this crate is Windows-only by design.
- Do not change the `unsafe` surface of `Mapper::*_raw()` to safe — the
  cross-process aliasing hazard is real.
- Do not add new public API without corresponding `///` doc comments and at least
  one `# Example` block.
- Do not change the coding style if not explicitly requested.
