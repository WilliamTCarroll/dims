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
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_area() {
        assert_eq!(SQUARE_INCH.from(1.0).as_base(), 0.0254_f32.powi(2));
        assert_eq!(SQUARE_INCH.from(432.0), SQUARE_FOOT.from(3.0));
        assert_eq!(SQUARE_FOOT.from(18.0), SQUARE_YARD.from(2.0));
        assert_eq!(SQUARE_YARD.from(6_195_200.0), SQUARE_MILE.from(2.0));
    }
}
