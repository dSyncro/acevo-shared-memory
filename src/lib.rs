//! # acevo-shared-memory
//!
//! A safe Rust interface for reading the AC Evo simulator's shared-memory telemetry output.
//!
//! AC Evo writes live telemetry into three Windows named shared-memory segments every
//! simulation step / rendered frame. This crate opens those segments, maps them into
//! the process address space, and exposes the data through ergonomic typed views.
//!
//! ## Shared-memory segments
//!
//! | Segment | Named object | Content |
//! |---------|-------------|---------|
//! | Physics | `Local\acevo_pmf_physics` | Low-level vehicle dynamics, updated every sim step |
//! | Graphics | `Local\acevo_pmf_graphics` | HUD state, tyres, electronics, timing, updated each frame |
//! | Static | `Local\acevo_pmf_static` | Session metadata, written once on session load |
//!
//! ## Usage example
//!
//! ```no_run
//! use acevo_shared_memory::ACEvoSharedMemoryMapper;
//!
//! // Open all three shared-memory segments at once.
//! let mapper = ACEvoSharedMemoryMapper::open().expect("AC Evo must be running");
//!
//! // --- Physics ---
//! let physics = mapper.physics();
//! println!("Speed: {:.1} km/h", physics.raw().speedKmh);
//! println!("Gear:  {}", physics.raw().gear);
//!
//! // --- Graphics ---
//! let graphics = mapper.graphics();
//! println!("Driver: {} {}", graphics.driver_name(), graphics.driver_surname());
//! println!("Status: {:?}", graphics.status());
//! println!("Flag:   {:?}", graphics.flag());
//!
//! // --- Static session data ---
//! let static_data = mapper.static_data();
//! println!("Track:   {}", static_data.track());
//! println!("Session: {:?}", static_data.session());
//! ```
//!
//! ## Snapshots
//!
//! Views borrow directly from the shared-memory mapping and carry a lifetime tied to
//! the [`ACEvoSharedMemoryMapper`]. Call [`View::snapshot`] to obtain an
//! owned copy that outlives the mapper:
//!
//! ```no_run
//! use acevo_shared_memory::ACEvoSharedMemoryMapper;
//!
//! let mapper = ACEvoSharedMemoryMapper::open().unwrap();
//! let snap = mapper.physics().snapshot(); // 'static lifetime, heap-allocated copy
//! drop(mapper);
//! println!("Captured speed: {:.1} km/h", snap.raw().speedKmh);
//! ```
//!
//! ## Raw access
//!
//! Every view exposes the underlying C struct through [`View::raw`].
//! This gives access to every field defined in the shared-memory protocol, including
//! ones not yet wrapped by a typed method:
//!
//! ```no_run
//! use acevo_shared_memory::ACEvoSharedMemoryMapper;
//!
//! let mapper = ACEvoSharedMemoryMapper::open().unwrap();
//! let graphics = mapper.graphics();
//! let raw = graphics.raw();
//! println!("Fuel remaining: {:.2} L", raw.fuel_liter_current_quantity);
//! println!("Position: {}/{}", raw.current_pos, raw.total_drivers);
//! ```
//!
//! ## Feature flags
//!
//! | Feature | Effect |
//! |---------|--------|
//! | `serde` | Derives `serde::Serialize` on all views, raw structs, and enums; derives `serde::Deserialize` on snapshot views (`View<'static, T>`) and enums |

mod bindings;
mod mapper;
pub mod views;
pub mod wrappers;

pub use bindings::root::ks::{
    SMEvoAssistsState, SMEvoDamageState, SMEvoElectronics, SMEvoInstrumentation, SMEvoPitInfo,
    SMEvoSessionState, SMEvoTimingState, SMEvoTyreState, SPageFileGraphicEvo, SPageFilePhysics,
    SPageFileStaticEvo,
};
pub use mapper::Mapper as ACEvoSharedMemoryMapper;
pub use views::*;
pub use wrappers::*;
