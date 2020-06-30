//! Units of area  that aren't necessarily used in surveying.
//! The ACRE is re-exported from surveying

// Re-export ACRE (is commonly used outside of surveying)
pub use super::area_survey::ACRE;
use crate::systems::Area;
use dims_core::unit_creation::*;

pub const SQUARE_INCH: UnitSimple<Area> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: SQUARE_FOOT.ratio / 144.0,
};

pub const SQUARE_FOOT: UnitSimple<Area> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.09290304,
};

pub const SQUARE_YARD: UnitSimple<Area> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: SQUARE_FOOT.ratio * 9.0,
};

pub const SQUARE_MILE: UnitSimple<Area> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 2589988.110336,
};
