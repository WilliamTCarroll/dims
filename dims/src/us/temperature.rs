//! Units for measuring temperature
//! The only commonly used modern units are here: Rankine and Farenheit

use crate::systems::TemperatureSystem;
use dims_core::unit_creation::*;

pub const RANKINE: UnitSimple<TemperatureSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 5.0 / 9.0,
};
pub const FAHRENHEIT: UnitSimple<TemperatureSystem> = UnitSimple {
    system: PhantomData,
    offset: 459.67,
    ratio: 5.0 / 9.0,
};
