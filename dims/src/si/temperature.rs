use crate::systems::TemperatureSystem;
use dims_core::prelude::*;

pub const KELVIN: UnitSimple<TemperatureSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1.0,
};
pub const CELCIUS: UnitSimple<TemperatureSystem> = UnitSimple {
    system: PhantomData,
    offset: 273.15,
    ratio: 1.0,
};
