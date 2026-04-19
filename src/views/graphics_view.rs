//! Typed view over the graphics shared-memory page (`SPageFileGraphicEvo`).
//!
//! The graphics page is updated every rendered frame and contains HUD state, tyre
//! data, electronics settings, lap timing, session state, and more.
//! This view surfaces commonly used fields through typed accessor methods; use
//! [`GraphicsView::raw`](super::view::View::raw) for direct access to every field.

use crate::bindings::root::ks::SPageFileGraphicEvo;
use crate::wrappers::{ACEvoCarLocation, ACEvoEngineType, ACEvoFlagType, ACEvoStatus};

use super::utils::parse_c_str;
use super::view::View;

/// A view over the `SPageFileGraphicEvo` shared-memory page.
///
/// Obtain one from [`ACEvoSharedMemoryMapper::graphics`](crate::ACEvoSharedMemoryMapper::graphics).
///
/// All accessor methods read directly from the live shared-memory mapping.
/// Call [`GraphicsView::snapshot`](View::snapshot) to obtain an owned copy.
///
/// # Example
///
/// ```no_run
/// use acevo_shared_memory::ACEvoSharedMemoryMapper;
///
/// let mapper = ACEvoSharedMemoryMapper::open().unwrap();
/// let g = mapper.graphics();
///
/// println!("Driver:   {} {}", g.driver_name(), g.driver_surname());
/// println!("Car:      {}", g.car_model());
/// println!("Status:   {:?}", g.status());
/// println!("Flag:     {:?}", g.flag());
/// println!("Location: {:?}", g.car_location());
/// println!("Engine:   {:?}", g.engine_type());
/// ```
pub type GraphicsView<'a> = View<'a, SPageFileGraphicEvo>;

impl<'a> GraphicsView<'a> {
    /// Returns the current simulator operational state.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use acevo_shared_memory::{ACEvoSharedMemoryMapper, ACEvoStatus};
    /// # let mapper = ACEvoSharedMemoryMapper::open().unwrap();
    /// if mapper.graphics().status() == ACEvoStatus::Live {
    ///     println!("Session is live");
    /// }
    /// ```
    pub fn status(&self) -> ACEvoStatus {
        ACEvoStatus::from(self.inner().status)
    }

    /// Returns the unique ID pair `(a, b)` of the car currently shown by the camera.
    pub fn focused_car_id(&self) -> (u64, u64) {
        (self.inner().focused_car_id_a, self.inner().focused_car_id_b)
    }

    /// Returns the unique ID pair `(a, b)` of the player's own car.
    pub fn player_car_id(&self) -> (u64, u64) {
        (self.inner().player_car_id_a, self.inner().player_car_id_b)
    }

    /// Returns the current track zone the car occupies.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use acevo_shared_memory::{ACEvoSharedMemoryMapper, ACEvoCarLocation};
    /// # let mapper = ACEvoSharedMemoryMapper::open().unwrap();
    /// if mapper.graphics().car_location() == ACEvoCarLocation::Pitlane {
    ///     println!("Car is in the pit lane");
    /// }
    /// ```
    pub fn car_location(&self) -> ACEvoCarLocation {
        ACEvoCarLocation::from(self.inner().car_location)
    }

    /// Returns the current G-forces as `(lateral_x, longitudinal_y, vertical_z)`.
    pub fn g_forces(&self) -> (f32, f32, f32) {
        let page = self.inner();
        (page.g_forces_x, page.g_forces_y, page.g_forces_z)
    }

    /// Returns the flag shown specifically to this driver.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use acevo_shared_memory::{ACEvoSharedMemoryMapper, ACEvoFlagType};
    /// # let mapper = ACEvoSharedMemoryMapper::open().unwrap();
    /// match mapper.graphics().flag() {
    ///     ACEvoFlagType::BlueFlag  => println!("Blue flag — let the leader past"),
    ///     ACEvoFlagType::YellowFlag => println!("Yellow flag — danger ahead"),
    ///     _ => {}
    /// }
    /// ```
    pub fn flag(&self) -> ACEvoFlagType {
        ACEvoFlagType::from(self.inner().flag)
    }

    /// Returns the flag shown to all drivers on track.
    pub fn global_flag(&self) -> ACEvoFlagType {
        ACEvoFlagType::from(self.inner().global_flag)
    }

    /// Returns the in-game time of day as `(hours, minutes, seconds)`.
    pub fn time_of_day(&self) -> (i32, i32, i32) {
        let page = self.inner();
        (
            page.time_of_day_hours,
            page.time_of_day_minutes,
            page.time_of_day_seconds,
        )
    }

    /// Returns the powertrain type of the player's car.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use acevo_shared_memory::{ACEvoSharedMemoryMapper, ACEvoEngineType};
    /// # let mapper = ACEvoSharedMemoryMapper::open().unwrap();
    /// if mapper.graphics().engine_type() == ACEvoEngineType::ElectricMotor {
    ///     println!("Car is fully electric — no fuel needed");
    /// }
    /// ```
    pub fn engine_type(&self) -> ACEvoEngineType {
        ACEvoEngineType::from(self.inner().engine_type)
    }

    /// Returns the display name of the active vehicle performance / power mode.
    pub fn performance_mode_name(&self) -> &str {
        let name = &self.inner().performance_mode_name;
        parse_c_str(name)
    }

    /// Returns the driver's first name.
    pub fn driver_name(&self) -> &str {
        let name = &self.inner().driver_name;
        parse_c_str(name)
    }

    /// Returns the driver's surname.
    pub fn driver_surname(&self) -> &str {
        let name = &self.inner().driver_surname;
        parse_c_str(name)
    }

    /// Returns the identifier or display name of the car model.
    pub fn car_model(&self) -> &str {
        let name = &self.inner().car_model;
        parse_c_str(name)
    }
}
