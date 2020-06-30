//! Units of area that are generally used in surveying.
//! The ACRE is re-exported in the general `area` mod
use crate::systems::Area;
use dims_core::unit_creation::*;

pub const SQUARE_FOOT_SURVEY: UnitSimple<Area> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1_440_000.0 / 15_499_969.0,
};

pub const SQUARE_CHAIN: UnitSimple<Area> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: SQUARE_FOOT_SURVEY.ratio * 4356.0,
};

pub const ACRE: UnitSimple<Area> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: SQUARE_CHAIN.ratio * 10.0,
};

pub const SECTION: UnitSimple<Area> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: ACRE.ratio * 640.0,
};

pub const SURVEY_TOWNSHIP: UnitSimple<Area> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: SECTION.ratio * 36.0,
};
