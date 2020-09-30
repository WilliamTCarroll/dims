use crate::systems::TemperatureSystem;
pub use crate::unit_type::UnitType;
use dims_core::prelude::*;

pub type TemperatureUnit<'t> = crate::unit_type::UnitType<'t, TemperatureSystem>;

pub const KELVIN: TemperatureUnit = TemperatureUnit {
    system: PhantomData,
    offset: 0.0,
    ratio: 1.0,
    #[cfg(feature = "str")]
    abbr: "K",
    #[cfg(feature = "str")]
    singular: "kelvin",
    #[cfg(feature = "str")]
    plural: "kelvin",
};
pub const CELCIUS: TemperatureUnit = TemperatureUnit {
    system: PhantomData,
    offset: 273.15,
    ratio: 1.0,
    #[cfg(feature = "str")]
    abbr: "°C",
    #[cfg(feature = "str")]
    singular: "celcius",
    #[cfg(feature = "str")]
    plural: "celcius",
};
