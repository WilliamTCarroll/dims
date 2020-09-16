use crate::systems::TemperatureSystem;
use dims_core::prelude::*;

pub const KELVIN: UnitSimple<TemperatureSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1.0,
    #[cfg(feature = "std")]
    abbr: "K",
    #[cfg(feature = "std")]
    singular: "kelvin",
    #[cfg(feature = "std")]
    plural: "kelvin",
};
pub const CELCIUS: UnitSimple<TemperatureSystem> = UnitSimple {
    system: PhantomData,
    offset: 273.15,
    ratio: 1.0,
    #[cfg(feature = "std")]
    abbr: "°C",
    #[cfg(feature = "std")]
    singular: "celcius",
    #[cfg(feature = "std")]
    plural: "celcius",
};
