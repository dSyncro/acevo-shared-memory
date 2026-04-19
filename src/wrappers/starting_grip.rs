//! [`ACEvoStartingGrip`] — initial grip conditions at session start.

use strum::{Display, EnumIter, EnumString, IntoStaticStr};

/// Initial grip conditions at session start.
///
/// Maps to the `ACEVO_STARTING_GRIP` typedef of the raw protocol.
///
/// # Example
///
/// ```no_run
/// use acevo_shared_memory::{ACEvoSharedMemoryMapper, ACEvoStartingGrip};
///
/// let mapper = ACEvoSharedMemoryMapper::open().unwrap();
/// match mapper.static_data().starting_grip() {
///     ACEvoStartingGrip::Green   => println!("Green track — grip builds during the session"),
///     ACEvoStartingGrip::Fast    => println!("Track already in advanced grip stage"),
///     ACEvoStartingGrip::Optimum => println!("Track starts at peak grip"),
///     ACEvoStartingGrip::Unknown(v) => println!("Unknown grip condition: {v}"),
/// }
/// ```
///
/// Convert back to the raw protocol integer with [`ACEvoStartingGrip::value`]:
///
/// ```
/// use acevo_shared_memory::ACEvoStartingGrip;
/// assert_eq!(ACEvoStartingGrip::Green.value(),   0);
/// assert_eq!(ACEvoStartingGrip::Fast.value(),    1);
/// assert_eq!(ACEvoStartingGrip::Optimum.value(), 2);
/// assert_eq!(ACEvoStartingGrip::from(2), ACEvoStartingGrip::Optimum);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, EnumIter, IntoStaticStr)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ACEvoStartingGrip {
    /// Track grip at minimum
    Green,
    /// Track grip in advanced (fast) stage
    Fast,
    /// Track conditions starting at optimum grip
    Optimum,
    /// Unknown value received from shared memory
    #[strum(disabled)]
    Unknown(i32),
}

impl ACEvoStartingGrip {
    /// Returns the original integer value as used in the shared-memory protocol.
    ///
    /// ```
    /// use acevo_shared_memory::ACEvoStartingGrip;
    /// assert_eq!(ACEvoStartingGrip::Green.value(),      0);
    /// assert_eq!(ACEvoStartingGrip::Fast.value(),       1);
    /// assert_eq!(ACEvoStartingGrip::Optimum.value(),    2);
    /// assert_eq!(ACEvoStartingGrip::Unknown(9).value(), 9);
    /// ```
    pub fn value(&self) -> i32 {
        match self {
            Self::Green => 0,
            Self::Fast => 1,
            Self::Optimum => 2,
            Self::Unknown(v) => *v,
        }
    }
}

impl From<i32> for ACEvoStartingGrip {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Green,
            1 => Self::Fast,
            2 => Self::Optimum,
            v => Self::Unknown(v),
        }
    }
}
