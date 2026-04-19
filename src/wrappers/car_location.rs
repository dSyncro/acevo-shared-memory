//! [`ACEvoCarLocation`] — where on the circuit the car is currently positioned.

use strum::{Display, EnumIter, EnumString, IntoStaticStr};

/// Where on the circuit the car is currently positioned.
///
/// Maps to the `ACEVO_CAR_LOCATION` typedef of the raw protocol.
///
/// # Example
///
/// ```no_run
/// use acevo_shared_memory::{ACEvoSharedMemoryMapper, ACEvoCarLocation};
///
/// let mapper = ACEvoSharedMemoryMapper::open().unwrap();
/// match mapper.graphics().car_location() {
///     ACEvoCarLocation::Track    => println!("On track"),
///     ACEvoCarLocation::Pitlane  => println!("In the pit lane"),
///     ACEvoCarLocation::PitEntry => println!("Entering the pits"),
///     ACEvoCarLocation::PitExit  => println!("Exiting the pits"),
///     ACEvoCarLocation::Unassigned | ACEvoCarLocation::Unknown(_) => {}
/// }
/// ```
///
/// Convert back to the raw protocol integer with [`ACEvoCarLocation::value`]:
///
/// ```
/// use acevo_shared_memory::ACEvoCarLocation;
/// assert_eq!(ACEvoCarLocation::Track.value(),   4);
/// assert_eq!(ACEvoCarLocation::from(1), ACEvoCarLocation::Pitlane);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, EnumIter, IntoStaticStr)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ACEvoCarLocation {
    /// Position not yet determined
    Unassigned,
    /// Car is inside the pit lane
    Pitlane,
    /// Car is at the pit-lane entry
    PitEntry,
    /// Car is at the pit-lane exit
    PitExit,
    /// Car is on the racing circuit
    Track,
    /// Unknown value received from shared memory
    #[strum(disabled)]
    Unknown(i32),
}

impl ACEvoCarLocation {
    /// Returns the original integer value as used in the shared-memory protocol.
    ///
    /// ```
    /// use acevo_shared_memory::ACEvoCarLocation;
    /// assert_eq!(ACEvoCarLocation::Unassigned.value(), 0);
    /// assert_eq!(ACEvoCarLocation::Pitlane.value(),    1);
    /// assert_eq!(ACEvoCarLocation::PitEntry.value(),   2);
    /// assert_eq!(ACEvoCarLocation::PitExit.value(),    3);
    /// assert_eq!(ACEvoCarLocation::Track.value(),      4);
    /// assert_eq!(ACEvoCarLocation::Unknown(7).value(), 7);
    /// ```
    pub fn value(&self) -> i32 {
        match self {
            Self::Unassigned => 0,
            Self::Pitlane => 1,
            Self::PitEntry => 2,
            Self::PitExit => 3,
            Self::Track => 4,
            Self::Unknown(v) => *v,
        }
    }
}

impl From<i32> for ACEvoCarLocation {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Unassigned,
            1 => Self::Pitlane,
            2 => Self::PitEntry,
            3 => Self::PitExit,
            4 => Self::Track,
            v => Self::Unknown(v),
        }
    }
}
