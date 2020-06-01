use crate::systems::Temperature;
use dims_core::unit_creation::*;
pub const RANKINE: UnitSimple<Temperature> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 5.0 / 9.0,
};
pub const FAHRENHEIT: UnitSimple<Temperature> = UnitSimple {
    system: PhantomData,
    offset: 459.67,
    ratio: 5.0 / 9.0,
};
