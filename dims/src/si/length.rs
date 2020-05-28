use crate::systems::Length;
use dims_core::prelude::*;

pub const MILLIMETRE: UnitSimple<Length> = UnitSimple::<Length> {
    system: PhantomData,
    ratio: 1.0 / 1000.0,
    offset: 0.0,
};

pub const METRE: UnitSimple<Length> = UnitSimple::<Length> {
    system: PhantomData,
    ratio: 1.0,
    offset: 0.0,
};
