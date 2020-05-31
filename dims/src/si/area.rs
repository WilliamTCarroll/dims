use crate::systems::Area;
use dims_core::unit_creation::*;
pub const SQMETRE: UnitSimple<Area> = UnitSimple::<Area> {
    system: PhantomData,
    ratio: 1.0,
    offset: 0.0,
};
