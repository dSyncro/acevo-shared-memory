//! Typed view over the static shared-memory page (`SPageFileStaticEvo`).
//!
//! The static page is written once when a session loads and does not change while
//! driving. It contains session metadata such as track name, session type, weather
//! conditions, and geographic coordinates.

use crate::bindings::root::ks::SPageFileStaticEvo;
use crate::wrappers::{ACEvoSessionType, ACEvoStartingGrip};

use super::utils::parse_c_str;
use super::view::View;

/// A view over the `SPageFileStaticEvo` shared-memory page.
///
/// Obtain one from [`ACEvoSharedMemoryMapper::static_data`](crate::ACEvoSharedMemoryMapper::static_data).
///
/// Because this page is written only at session load, it is safe to snapshot once
/// and hold for the duration of the session.
///
/// # Example
///
/// ```no_run
/// use acevo_shared_memory::ACEvoSharedMemoryMapper;
///
/// let mapper = ACEvoSharedMemoryMapper::open().unwrap();
/// let s = mapper.static_data();
///
/// println!("Interface version: {}", s.sm_version());
/// println!("Game version:      {}", s.ac_evo_version());
/// println!("Track:             {} ({})", s.track(), s.track_configuration());
/// println!("Session:           {:?}", s.session());
/// println!("Starting grip:     {:?}", s.starting_grip());
/// println!("Nation:            {}", s.nation());
/// ```
pub type StaticView<'a> = View<'a, SPageFileStaticEvo>;

impl<'a> StaticView<'a> {
    /// Returns the shared-memory interface version string.
    pub fn sm_version(&self) -> &str {
        let version = &self.inner().sm_version;
        parse_c_str(version)
    }

    /// Returns the AC Evo game build version string.
    pub fn ac_evo_version(&self) -> &str {
        let version = &self.inner().ac_evo_version;
        parse_c_str(version)
    }

    /// Returns the type of the current session.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use acevo_shared_memory::{ACEvoSharedMemoryMapper, ACEvoSessionType};
    /// # let mapper = ACEvoSharedMemoryMapper::open().unwrap();
    /// match mapper.static_data().session() {
    ///     ACEvoSessionType::Race      => println!("It's race day!"),
    ///     ACEvoSessionType::TimeAttack => println!("Qualifying in progress"),
    ///     _ => {}
    /// }
    /// ```
    pub fn session(&self) -> ACEvoSessionType {
        ACEvoSessionType::from(self.inner().session)
    }

    /// Returns the human-readable session name (e.g. `"Race 1"`).
    pub fn session_name(&self) -> &str {
        let name = &self.inner().session_name;
        parse_c_str(name)
    }

    /// Returns the tyre grip condition at session start.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use acevo_shared_memory::{ACEvoSharedMemoryMapper, ACEvoStartingGrip};
    /// # let mapper = ACEvoSharedMemoryMapper::open().unwrap();
    /// if mapper.static_data().starting_grip() == ACEvoStartingGrip::Green {
    ///     println!("Track is green — grip will build through the session");
    /// }
    /// ```
    pub fn starting_grip(&self) -> ACEvoStartingGrip {
        ACEvoStartingGrip::from(self.inner().starting_grip)
    }

    /// Returns the track identifier or name.
    pub fn track(&self) -> &str {
        let track = &self.inner().track;
        parse_c_str(track)
    }

    /// Returns the track layout variant or configuration name.
    pub fn track_configuration(&self) -> &str {
        let config = &self.inner().track_configuration;
        parse_c_str(config)
    }

    /// Returns the country / nation name associated with the event or track.
    pub fn nation(&self) -> &str {
        let nation = &self.inner().nation;
        parse_c_str(nation)
    }
}
