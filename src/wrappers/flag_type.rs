//! [`ACEvoFlagType`] — race flag currently shown to the driver.

use strum::{Display, EnumIter, EnumString, IntoStaticStr};

/// Race flag currently shown to the driver.
///
/// Maps to the `ACEVO_FLAG_TYPE` typedef of the raw protocol.
/// Both a per-driver flag ([`GraphicsView::flag`](crate::GraphicsView::flag)) and a
/// global track flag ([`GraphicsView::global_flag`](crate::GraphicsView::global_flag))
/// are available.
///
/// # Example
///
/// ```no_run
/// use acevo_shared_memory::{ACEvoSharedMemoryMapper, ACEvoFlagType};
///
/// let mapper = ACEvoSharedMemoryMapper::open().unwrap();
/// match mapper.graphics().flag() {
///     ACEvoFlagType::NoFlag       => {}
///     ACEvoFlagType::BlueFlag     => println!("Blue — let the leader past"),
///     ACEvoFlagType::YellowFlag   => println!("Yellow — danger, no overtaking"),
///     ACEvoFlagType::RedFlag      => println!("Red — session stopped"),
///     ACEvoFlagType::CheckeredFlag => println!("Chequered — session ended"),
///     other => println!("Flag: {other}"),
/// }
/// ```
///
/// Convert back to the raw protocol integer with [`ACEvoFlagType::value`]:
///
/// ```
/// use acevo_shared_memory::ACEvoFlagType;
/// assert_eq!(ACEvoFlagType::NoFlag.value(),      0);
/// assert_eq!(ACEvoFlagType::CheckeredFlag.value(), 8);
/// assert_eq!(ACEvoFlagType::from(4), ACEvoFlagType::BlueFlag);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, EnumIter, IntoStaticStr)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ACEvoFlagType {
    /// No flag displayed
    NoFlag,
    /// Slow vehicle ahead on track
    WhiteFlag,
    /// Track clear — racing resumed
    GreenFlag,
    /// Session stopped due to incident or hazard
    RedFlag,
    /// Lapped car must yield to the race leader
    BlueFlag,
    /// Hazard present — no overtaking
    YellowFlag,
    /// Driver disqualified / must pit immediately
    BlackFlag,
    /// Warning for unsportsmanlike behaviour
    BlackWhiteFlag,
    /// Session or race has ended
    CheckeredFlag,
    /// Mechanical problem — car must pit
    OrangeCircleFlag,
    /// Slippery surface ahead on track
    RedYellowStripesFlag,
    /// Unknown value received from shared memory
    #[strum(disabled)]
    Unknown(i32),
}

impl ACEvoFlagType {
    /// Returns the original integer value as used in the shared-memory protocol.
    ///
    /// ```
    /// use acevo_shared_memory::ACEvoFlagType;
    /// assert_eq!(ACEvoFlagType::NoFlag.value(),              0);
    /// assert_eq!(ACEvoFlagType::WhiteFlag.value(),           1);
    /// assert_eq!(ACEvoFlagType::GreenFlag.value(),           2);
    /// assert_eq!(ACEvoFlagType::RedFlag.value(),             3);
    /// assert_eq!(ACEvoFlagType::BlueFlag.value(),            4);
    /// assert_eq!(ACEvoFlagType::YellowFlag.value(),          5);
    /// assert_eq!(ACEvoFlagType::BlackFlag.value(),           6);
    /// assert_eq!(ACEvoFlagType::BlackWhiteFlag.value(),      7);
    /// assert_eq!(ACEvoFlagType::CheckeredFlag.value(),       8);
    /// assert_eq!(ACEvoFlagType::OrangeCircleFlag.value(),    9);
    /// assert_eq!(ACEvoFlagType::RedYellowStripesFlag.value(), 10);
    /// assert_eq!(ACEvoFlagType::Unknown(99).value(),         99);
    /// ```
    pub fn value(&self) -> i32 {
        match self {
            Self::NoFlag => 0,
            Self::WhiteFlag => 1,
            Self::GreenFlag => 2,
            Self::RedFlag => 3,
            Self::BlueFlag => 4,
            Self::YellowFlag => 5,
            Self::BlackFlag => 6,
            Self::BlackWhiteFlag => 7,
            Self::CheckeredFlag => 8,
            Self::OrangeCircleFlag => 9,
            Self::RedYellowStripesFlag => 10,
            Self::Unknown(v) => *v,
        }
    }
}

impl From<i32> for ACEvoFlagType {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::NoFlag,
            1 => Self::WhiteFlag,
            2 => Self::GreenFlag,
            3 => Self::RedFlag,
            4 => Self::BlueFlag,
            5 => Self::YellowFlag,
            6 => Self::BlackFlag,
            7 => Self::BlackWhiteFlag,
            8 => Self::CheckeredFlag,
            9 => Self::OrangeCircleFlag,
            10 => Self::RedYellowStripesFlag,
            v => Self::Unknown(v),
        }
    }
}
