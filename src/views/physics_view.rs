//! Typed view over the physics shared-memory page (`SPageFilePhysics`).
//!
//! The physics page is updated every simulation step and contains raw vehicle-dynamics
//! data: pedal inputs, speeds, tyre state, suspension, ERS, damage, and more.
//! Boolean-like fields in the underlying C struct are stored as integers; the methods
//! here convert them to proper Rust `bool` values.

use crate::bindings::root::ks::SPageFilePhysics;

use super::view::View;

/// A view over the `SPageFilePhysics` shared-memory page.
///
/// Obtain one from [`ACEvoSharedMemoryMapper::physics`](crate::ACEvoSharedMemoryMapper::physics).
///
/// All accessor methods read directly from the live shared-memory mapping.
/// Call [`PhysicsView::snapshot`](View::snapshot) to obtain an owned copy.
///
/// # Example
///
/// ```no_run
/// use acevo_shared_memory::ACEvoSharedMemoryMapper;
///
/// let mapper = ACEvoSharedMemoryMapper::open().unwrap();
/// let physics = mapper.physics();
///
/// println!("Speed:  {:.1} km/h", physics.raw().speedKmh);
/// println!("Gear:   {}", physics.raw().gear);
/// println!("RPM:    {}", physics.raw().rpms);
/// println!("TC on:  {}", physics.tc_in_action());
/// println!("ABS on: {}", physics.abs_in_action());
/// ```
pub type PhysicsView<'a> = View<'a, SPageFilePhysics>;

impl<'a> PhysicsView<'a> {
    /// Returns `true` when the automatic gearshift aid is active.
    pub fn auto_shifter_on(&self) -> bool {
        self.inner().autoShifterOn != 0
    }

    /// Returns `true` when traction control is currently cutting power.
    pub fn tc_in_action(&self) -> bool {
        self.inner().tcinAction != 0
    }

    /// Returns `true` when ABS is currently modulating the brakes.
    pub fn abs_in_action(&self) -> bool {
        self.inner().absInAction != 0
    }

    /// Returns `true` when DRS can be activated in the current track section.
    pub fn drs_available(&self) -> bool {
        self.inner().drsAvailable != 0
    }

    /// Returns `true` when the DRS flap is open and active.
    pub fn drs_enabled(&self) -> bool {
        self.inner().drsEnabled != 0
    }

    /// Returns `true` when the pit-speed limiter is engaged.
    pub fn pit_limiter_on(&self) -> bool {
        self.inner().pitLimiterOn != 0
    }

    /// Returns `true` when the ERS system is currently recovering energy (charging).
    ///
    /// `false` means energy is being deployed.
    pub fn ers_is_charging(&self) -> bool {
        self.inner().ersIsCharging != 0
    }

    /// Returns `true` when the ignition switch is on.
    pub fn ignition_on(&self) -> bool {
        self.inner().ignitionOn != 0
    }

    /// Returns `true` when the starter motor is currently cranking the engine.
    pub fn starter_engine_on(&self) -> bool {
        self.inner().starterEngineOn != 0
    }

    /// Returns `true` when the engine is running.
    pub fn is_engine_running(&self) -> bool {
        self.inner().isEngineRunning != 0
    }

    /// Returns `true` when the car is driven by the AI rather than the player.
    pub fn is_ai_controlled(&self) -> bool {
        self.inner().isAIControlled != 0
    }
}
