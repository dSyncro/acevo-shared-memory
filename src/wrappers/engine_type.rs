//! [`ACEvoEngineType`] — powertrain type of the player car.

use strum::{Display, EnumIter, EnumString, IntoStaticStr};

/// Powertrain type of the player car.
///
/// Maps to the `ACEVO_ENGINE_TYPE` typedef of the raw protocol.
///
/// # Example
///
/// ```no_run
/// use acevo_shared_memory::{ACEvoSharedMemoryMapper, ACEvoEngineType};
///
/// let mapper = ACEvoSharedMemoryMapper::open().unwrap();
/// match mapper.graphics().engine_type() {
///     ACEvoEngineType::InternalCombustion => println!("ICE car — monitor fuel"),
///     ACEvoEngineType::ElectricMotor      => println!("EV — monitor battery"),
///     ACEvoEngineType::Unknown(v)         => println!("Unknown engine type: {v}"),
/// }
/// ```
///
/// Convert back to the raw protocol integer with [`ACEvoEngineType::value`]:
///
/// ```
/// use acevo_shared_memory::ACEvoEngineType;
/// assert_eq!(ACEvoEngineType::InternalCombustion.value(), 0);
/// assert_eq!(ACEvoEngineType::ElectricMotor.value(),      1);
/// assert_eq!(ACEvoEngineType::from(0), ACEvoEngineType::InternalCombustion);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, EnumIter, IntoStaticStr)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ACEvoEngineType {
    /// Traditional petrol/diesel internal combustion engine
    InternalCombustion,
    /// Fully electric powertrain
    ElectricMotor,
    /// Unknown value received from shared memory
    #[strum(disabled)]
    Unknown(i32),
}

impl ACEvoEngineType {
    /// Returns the original integer value as used in the shared-memory protocol.
    ///
    /// ```
    /// use acevo_shared_memory::ACEvoEngineType;
    /// assert_eq!(ACEvoEngineType::InternalCombustion.value(), 0);
    /// assert_eq!(ACEvoEngineType::ElectricMotor.value(),      1);
    /// assert_eq!(ACEvoEngineType::Unknown(5).value(),         5);
    /// ```
    pub fn value(&self) -> i32 {
        match self {
            Self::InternalCombustion => 0,
            Self::ElectricMotor => 1,
            Self::Unknown(v) => *v,
        }
    }
}

impl From<i32> for ACEvoEngineType {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::InternalCombustion,
            1 => Self::ElectricMotor,
            v => Self::Unknown(v),
        }
    }
}
