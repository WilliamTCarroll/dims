//! Units generally only used in the nautical context.

use crate::systems::Length;
use dims_core::unit_creation::*;

pub const FATHOM: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1143.0 / 625.0,
};

pub const CABLE: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: FATHOM.ratio * 120.0,
};

pub const MILE_NAUTICAL: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1852.0,
};
