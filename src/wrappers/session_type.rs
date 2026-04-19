//! [`ACEvoSessionType`] — type of racing session currently loaded.

use strum::{Display, EnumIter, EnumString, IntoStaticStr};

/// Type of racing session currently loaded.
///
/// Maps to the `ACEVO_SESSION_TYPE` typedef of the raw protocol.
///
/// # Example
///
/// ```no_run
/// use acevo_shared_memory::{ACEvoSharedMemoryMapper, ACEvoSessionType};
///
/// let mapper = ACEvoSharedMemoryMapper::open().unwrap();
/// match mapper.static_data().session() {
///     ACEvoSessionType::Race       => println!("Race session"),
///     ACEvoSessionType::TimeAttack => println!("Qualifying / time attack"),
///     ACEvoSessionType::HotStint   => println!("Hot-stint practice"),
///     ACEvoSessionType::Cruise     => println!("Cruise session"),
///     ACEvoSessionType::Unknown    => println!("Session type not yet known"),
///     ACEvoSessionType::Other(v)   => println!("Unrecognised session type: {v}"),
/// }
/// ```
///
/// Convert back to the raw protocol integer with [`ACEvoSessionType::value`]:
///
/// ```
/// use acevo_shared_memory::ACEvoSessionType;
/// assert_eq!(ACEvoSessionType::Unknown.value(), -1);
/// assert_eq!(ACEvoSessionType::Race.value(),     1);
/// assert_eq!(ACEvoSessionType::from(-1), ACEvoSessionType::Unknown);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, EnumIter, IntoStaticStr)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ACEvoSessionType {
    /// Session type not yet determined
    Unknown,
    /// Time attack / qualifying session
    TimeAttack,
    /// Race session
    Race,
    /// Hot-stint practice
    HotStint,
    /// Untimed cruise
    Cruise,
    /// Unrecognised value received from shared memory
    #[strum(disabled)]
    Other(i32),
}

impl ACEvoSessionType {
    /// Returns the original integer value as used in the shared-memory protocol.
    ///
    /// ```
    /// use acevo_shared_memory::ACEvoSessionType;
    /// assert_eq!(ACEvoSessionType::Unknown.value(),    -1);
    /// assert_eq!(ACEvoSessionType::TimeAttack.value(),  0);
    /// assert_eq!(ACEvoSessionType::Race.value(),        1);
    /// assert_eq!(ACEvoSessionType::HotStint.value(),    2);
    /// assert_eq!(ACEvoSessionType::Cruise.value(),      3);
    /// assert_eq!(ACEvoSessionType::Other(42).value(),  42);
    /// ```
    pub fn value(&self) -> i32 {
        match self {
            Self::Unknown => -1,
            Self::TimeAttack => 0,
            Self::Race => 1,
            Self::HotStint => 2,
            Self::Cruise => 3,
            Self::Other(v) => *v,
        }
    }
}

impl From<i32> for ACEvoSessionType {
    fn from(value: i32) -> Self {
        match value {
            -1 => Self::Unknown,
            0 => Self::TimeAttack,
            1 => Self::Race,
            2 => Self::HotStint,
            3 => Self::Cruise,
            v => Self::Other(v),
        }
    }
}
