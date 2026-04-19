//! Idiomatic Rust wrappers that mirror the C raw symbols defined in the
//! AC Evo shared-memory protocol.

mod car_location;
mod engine_type;
mod flag_type;
mod session_type;
mod starting_grip;
mod status;

pub use car_location::ACEvoCarLocation;
pub use engine_type::ACEvoEngineType;
pub use flag_type::ACEvoFlagType;
pub use session_type::ACEvoSessionType;
pub use starting_grip::ACEvoStartingGrip;
pub use status::ACEvoStatus;
