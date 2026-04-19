<div align="center">

# acevo-shared-memory

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/dSyncro/acevo-shared-memory/blob/main/LICENSE)
![Version](https://img.shields.io/badge/version-0.1.0-green)
[![Crates.io](https://img.shields.io/crates/v/acevo-shared-memory.svg)](https://crates.io/crates/acevo-shared-memory)
[![docs.rs](https://docs.rs/acevo-shared-memory/badge.svg)](https://docs.rs/acevo-shared-memory)

A safe Rust interface for reading **Assetto Corsa Evo** live telemetry via
Windows named shared memory.

</div>

## Table of Contents

- [Dependencies and requirements](#dependencies-and-requirements)
- [Getting started](#getting-started)
- [Usage](#usage)
  - [Opening the mapper](#opening-the-mapper)
  - [Reading physics data](#reading-physics-data)
  - [Reading graphics / HUD data](#reading-graphics--hud-data)
  - [Reading static session data](#reading-static-session-data)
- [Views](#views)
- [Typed enums](#typed-enums)
- [Snapshots](#snapshots)
- [Raw access](#raw-access)
- [Feature flags](#feature-flags)
- [Side notes](#side-notes)

## Dependencies and requirements

- **Windows only** — uses Win32 named file-mapping APIs.
- AC Evo must be running before calling `ACEvoSharedMemoryMapper::open()`.
- Requires Rust edition 2024.

## Getting started

Add the library to your project:

```bash
cargo add acevo-shared-memory
```

## Usage

All access goes through `ACEvoSharedMemoryMapper`, which opens the three named
shared-memory segments and returns typed views over each one.

### Opening the mapper

```rust
use acevo_shared_memory::ACEvoSharedMemoryMapper;

let mapper = ACEvoSharedMemoryMapper::open()
    .expect("AC Evo must be running");
```

### Reading physics data

The physics page is updated every simulation step and contains raw vehicle-dynamics
data: speed, gear, RPM, tyre state, G-forces, ERS, damage, and more.

```rust
use acevo_shared_memory::ACEvoSharedMemoryMapper;

let mapper = ACEvoSharedMemoryMapper::open().unwrap();
let physics = mapper.physics();

println!("Speed:  {:.1} km/h", physics.raw().speedKmh);
println!("Gear:   {}", physics.raw().gear);
println!("RPM:    {}", physics.raw().rpms);
println!("TC on:  {}", physics.tc_in_action());
println!("ABS on: {}", physics.abs_in_action());
println!("DRS:    {}", physics.drs_enabled());
```

### Reading graphics / HUD data

The graphics page is updated every rendered frame and contains HUD state, driver
info, flag state, lap timing, electronics settings, and more.

```rust
use acevo_shared_memory::ACEvoSharedMemoryMapper;

let mapper = ACEvoSharedMemoryMapper::open().unwrap();
let g = mapper.graphics();

println!("Driver:   {} {}", g.driver_name(), g.driver_surname());
println!("Car:      {}", g.car_model());
println!("Status:   {:?}", g.status());
println!("Flag:     {:?}", g.flag());
println!("Location: {:?}", g.car_location());
println!("Engine:   {:?}", g.engine_type());
println!("Pos:      {}/{}", g.raw().current_pos, g.raw().total_drivers);
```

### Reading static session data

The static page is written once when a session loads and does not change while
driving. It contains track name, session type, grip condition, and more.

```rust
use acevo_shared_memory::ACEvoSharedMemoryMapper;

let mapper = ACEvoSharedMemoryMapper::open().unwrap();
let s = mapper.static_data();

println!("Interface version: {}", s.sm_version());
println!("Game version:      {}", s.ac_evo_version());
println!("Track:             {} ({})", s.track(), s.track_configuration());
println!("Session:           {:?}", s.session());
println!("Starting grip:     {:?}", s.starting_grip());
println!("Nation:            {}", s.nation());
```

## Views

All three data views share the same `View<T>` foundation and expose these methods:

| Method       | Return type          | Description                                                               |
| ------------ | -------------------- | ------------------------------------------------------------------------- |
| `raw()`      | `&T`                 | Direct reference to the underlying C struct — access every protocol field |
| `inner()`    | `&T`                 | Alias for `raw()`                                                         |
| `snapshot()` | `View<'static, T>`   | Heap-allocates an owned copy that outlives the mapper                     |

### Shared-memory segments

| View          | Segment name                 | Content                                      | Update rate    |
| ------------- | ---------------------------- | -------------------------------------------- | -------------- |
| `PhysicsView` | `Local\acevo_pmf_physics`    | Vehicle dynamics — speed, tyres, suspension  | Every sim step |
| `GraphicsView`| `Local\acevo_pmf_graphics`   | HUD state, tyres, electronics, lap timing    | Every frame    |
| `StaticView`  | `Local\acevo_pmf_static`     | Session metadata — track, session type, grip | Once at load   |

### PhysicsView typed methods

| Method              | Return type | Description                                    |
| ------------------- | ----------- | ---------------------------------------------- |
| `auto_shifter_on()` | `bool`      | Automatic gearshift aid is active              |
| `tc_in_action()`    | `bool`      | Traction control is currently cutting power    |
| `abs_in_action()`   | `bool`      | ABS is currently modulating brakes             |
| `drs_available()`   | `bool`      | DRS can be activated in current track section  |
| `drs_enabled()`     | `bool`      | DRS flap is open and active                    |
| `pit_limiter_on()`  | `bool`      | Pit-speed limiter is engaged                   |
| `ers_is_charging()` | `bool`      | ERS is recovering energy (not deploying)       |
| `ignition_on()`     | `bool`      | Ignition switch is on                          |
| `starter_engine_on()` | `bool`    | Starter motor is cranking the engine           |
| `is_engine_running()` | `bool`    | Engine is running                              |
| `is_ai_controlled()` | `bool`     | Car is driven by AI                            |

### GraphicsView typed methods

| Method                  | Return type        | Description                                     |
| ----------------------- | ------------------ | ----------------------------------------------- |
| `status()`              | `ACEvoStatus`      | Simulator operational state                     |
| `car_location()`        | `ACEvoCarLocation` | Current track zone the car occupies             |
| `flag()`                | `ACEvoFlagType`    | Flag shown to this driver                       |
| `global_flag()`         | `ACEvoFlagType`    | Flag shown to all drivers                       |
| `engine_type()`         | `ACEvoEngineType`  | Powertrain type                                 |
| `driver_name()`         | `&str`             | Driver first name                               |
| `driver_surname()`      | `&str`             | Driver surname                                  |
| `car_model()`           | `&str`             | Car model identifier                            |
| `performance_mode_name()` | `&str`           | Active vehicle performance / power mode         |
| `focused_car_id()`      | `(u64, u64)`       | ID of car shown by camera                       |
| `player_car_id()`       | `(u64, u64)`       | ID of the player's own car                      |
| `g_forces()`            | `(f32, f32, f32)`  | Lateral / longitudinal / vertical G-forces      |
| `time_of_day()`         | `(i32, i32, i32)`  | In-game time as `(hours, minutes, seconds)`     |

### StaticView typed methods

| Method                  | Return type          | Description                                  |
| ----------------------- | -------------------- | -------------------------------------------- |
| `sm_version()`          | `&str`               | Shared-memory interface version              |
| `ac_evo_version()`      | `&str`               | Game build version                           |
| `session()`             | `ACEvoSessionType`   | Type of the current session                  |
| `session_name()`        | `&str`               | Human-readable session name                  |
| `starting_grip()`       | `ACEvoStartingGrip`  | Tyre grip condition at session start         |
| `track()`               | `&str`               | Track identifier                             |
| `track_configuration()` | `&str`               | Track layout variant                         |
| `nation()`              | `&str`               | Country / nation of the event                |

## Typed enums

Protocol integer constants are exposed as Rust enums. All enums derive
`Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Display`, `EnumString`,
`EnumIter`, and `IntoStaticStr` (via [`strum`](https://docs.rs/strum)).
Each enum has an `Unknown(i32)` or `Other(i32)` catch-all variant for
forward-compatibility with future protocol values.

| Enum                 | Maps to           | Variants                                                                      |
| -------------------- | ----------------- | ----------------------------------------------------------------------------- |
| `ACEvoStatus`        | `ACEVO_STATUS`    | `Off`, `Replay`, `Live`, `Pause`                                              |
| `ACEvoSessionType`   | `ACEVO_SESSION_TYPE` | `Unknown` (-1), `TimeAttack`, `Race`, `HotStint`, `Cruise`                 |
| `ACEvoFlagType`      | `ACEVO_FLAG_TYPE` | `NoFlag`, `WhiteFlag`, `GreenFlag`, `RedFlag`, `BlueFlag`, `YellowFlag`, `BlackFlag`, `BlackWhiteFlag`, `CheckeredFlag`, `OrangeCircleFlag`, `RedYellowStripesFlag` |
| `ACEvoCarLocation`   | `ACEVO_CAR_LOCATION` | `Unassigned`, `Pitlane`, `PitEntry`, `PitExit`, `Track`                    |
| `ACEvoEngineType`    | `ACEVO_ENGINE_TYPE` | `InternalCombustion`, `ElectricMotor`                                        |
| `ACEvoStartingGrip`  | `ACEVO_STARTING_GRIP` | `Green`, `Fast`, `Optimum`                                                 |

Every enum also provides a `value() -> i32` method to convert back to the
raw protocol integer:

```rust
use acevo_shared_memory::ACEvoStatus;

assert_eq!(ACEvoStatus::Live.value(), 2);
assert_eq!(ACEvoStatus::from(3), ACEvoStatus::Pause);
```

## Snapshots

Views borrow directly from the shared-memory mapping and carry a lifetime tied
to the `ACEvoSharedMemoryMapper`. Call `.snapshot()` to obtain a `'static`
heap copy that can be stored, queued, or sent across threads:

```rust
use acevo_shared_memory::ACEvoSharedMemoryMapper;

let mapper = ACEvoSharedMemoryMapper::open().unwrap();
let snap = mapper.physics().snapshot(); // Box<SPageFilePhysics> internally
drop(mapper);
println!("Captured speed: {:.1} km/h", snap.raw().speedKmh);
```

## Raw access

Every field defined in the protocol is accessible via `.raw()`, including
fields not yet covered by a typed accessor method:

```rust
let g = mapper.graphics().raw();
println!("Fuel:          {:.2} L",  g.fuel_liter_current_quantity);
println!("Lap time:      {} ms",    g.current_lap_time_ms);
println!("Predicted lap: {} ms",    g.predicted_lap_time_ms);
println!("Gap ahead:     {:.3} s",  g.gap_ahead);
println!("TC level:      {}",       g.electronics.tc_level);
```

## Feature flags

| Feature | Effect |
| ------- | ------ |
| `serde` | Derives `serde::Serialize` on all views, raw C structs, and enums. Derives `serde::Deserialize` on snapshot views (`View<'static, T>`) and enums. Borrowed views can only be serialized — deserializing always produces an owned snapshot. |

## Side notes

Please keep in mind that at the moment this is a side project developed with no
planned continuity nor schedule. Therefore _support, fixes and new features
cannot be guaranteed_.

As stated in the [LICENSE](https://github.com/dSyncro/acevo-shared-memory/blob/main/LICENSE),
_no contributor must be considered liable for the use of this project_.
