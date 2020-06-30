//! Basic Length measurements (inch, foot, mile, point, pica, etc)
//! These would be considered the most "common" length measurements.
//! See also: `length_survey` and `length_nautical`

use crate::systems::Length;
use dims_core::unit_creation::*;

pub const POINT: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 127.0 / 360_000.0,
};

pub const PICA: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 127.0 / 30_000.0,
};

pub const INCH: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.0254,
};

pub const FOOT: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.3048,
};

pub const YARD: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.9144,
};

pub const MILE: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1609.344,
};
