//! Units for measuring temperature
//! The only commonly used modern units are here: Rankine and Farenheit

use crate::systems::TemperatureSystem;

pub type TemperatureUnit<'t> = super::UnitType<'t, TemperatureSystem>;

pub const RANKINE: TemperatureUnit = TemperatureUnit {
    offset: 0.0,
    ratio: 5.0 / 9.0,
    #[cfg(feature = "str")]
    abbr: "°R",
    #[cfg(feature = "str")]
    singular: "rankine",
    #[cfg(feature = "str")]
    plural: "rankine",
};
pub const FAHRENHEIT: TemperatureUnit = TemperatureUnit {
    offset: 459.67,
    ratio: 5.0 / 9.0,
    #[cfg(feature = "str")]
    abbr: "°F",
    #[cfg(feature = "str")]
    singular: "fahrenheit",
    #[cfg(feature = "str")]
    plural: "fahrenheit",
};
