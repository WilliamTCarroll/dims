use crate::systems::Temperature;
use dims_core::prelude::*;

pub const KELVIN: UnitSimple<Temperature> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1.0,
};
pub const CELCIUS: UnitSimple<Temperature> = UnitSimple {
    system: PhantomData,
    offset: 273.15,
    ratio: 1.0,
};
