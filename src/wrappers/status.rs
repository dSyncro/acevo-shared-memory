//! [`ACEvoStatus`] — operational state of the simulator.

use strum::{Display, EnumIter, EnumString, IntoStaticStr};

/// Current operational state of the simulator.
///
/// Maps to the `ACEVO_STATUS` typedef of the raw protocol.
///
/// # Example
///
/// ```no_run
/// use acevo_shared_memory::{ACEvoSharedMemoryMapper, ACEvoStatus};
///
/// let mapper = ACEvoSharedMemoryMapper::open().unwrap();
/// match mapper.graphics().status() {
///     ACEvoStatus::Live   => println!("Session is live"),
///     ACEvoStatus::Pause  => println!("Session is paused"),
///     ACEvoStatus::Replay => println!("Watching a replay"),
///     ACEvoStatus::Off    => println!("Simulator is idle"),
///     ACEvoStatus::Unknown(v) => println!("Unknown status: {v}"),
/// }
/// ```
///
/// Convert back to the raw protocol integer with [`ACEvoStatus::value`]:
///
/// ```
/// use acevo_shared_memory::ACEvoStatus;
/// assert_eq!(ACEvoStatus::Live.value(), 2);
/// assert_eq!(ACEvoStatus::from(3), ACEvoStatus::Pause);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, EnumIter, IntoStaticStr)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ACEvoStatus {
    /// Simulator is not running / no session active
    Off,
    /// A replay is currently being played back
    Replay,
    /// Live driving session is active
    Live,
    /// Session is paused
    Pause,
    /// Unknown value received from shared memory
    #[strum(disabled)]
    Unknown(i32),
}

impl ACEvoStatus {
    /// Returns the original integer value as used in the shared-memory protocol.
    ///
    /// ```
    /// use acevo_shared_memory::ACEvoStatus;
    /// assert_eq!(ACEvoStatus::Off.value(),    0);
    /// assert_eq!(ACEvoStatus::Replay.value(), 1);
    /// assert_eq!(ACEvoStatus::Live.value(),   2);
    /// assert_eq!(ACEvoStatus::Pause.value(),  3);
    /// assert_eq!(ACEvoStatus::Unknown(99).value(), 99);
    /// ```
    pub fn value(&self) -> i32 {
        match self {
            Self::Off => 0,
            Self::Replay => 1,
            Self::Live => 2,
            Self::Pause => 3,
            Self::Unknown(v) => *v,
        }
    }
}

impl From<i32> for ACEvoStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::Replay,
            2 => Self::Live,
            3 => Self::Pause,
            v => Self::Unknown(v),
        }
    }
}
