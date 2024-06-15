//! Units of area that aren't necessarily used in surveying.
//! The ACRE is re-exported from surveying

// Re-export ACRE (is commonly used outside of surveying)
pub use super::area_survey::ACRE;
use crate::systems::AreaSystem;

pub type AreaUnit<'t> = super::UnitType<'t, AreaSystem>;

pub const SQUARE_INCH: AreaUnit = AreaUnit {
    offset: 0.0,
    ratio: SQUARE_FOOT.ratio / 144.0,
    #[cfg(feature = "str")]
    abbr: "in²",
    #[cfg(feature = "str")]
    singular: "square inch",
    #[cfg(feature = "str")]
    plural: "square inches",
};

pub const SQUARE_FOOT: AreaUnit = AreaUnit {
    offset: 0.0,
    ratio: 0.09290304,
    #[cfg(feature = "str")]
    abbr: "ft²",
    #[cfg(feature = "str")]
    singular: "square foot",
    #[cfg(feature = "str")]
    plural: "square feet",
};

pub const SQUARE_YARD: AreaUnit = AreaUnit {
    offset: 0.0,
    ratio: SQUARE_FOOT.ratio * 9.0,
    #[cfg(feature = "str")]
    abbr: "yd²",
    #[cfg(feature = "str")]
    singular: "square yard",
    #[cfg(feature = "str")]
    plural: "square yards",
};

pub const SQUARE_MILE: AreaUnit = AreaUnit {
    offset: 0.0,
    ratio: 2589988.110336,
    #[cfg(feature = "str")]
    abbr: "mi²",
    #[cfg(feature = "str")]
    singular: "square mile",
    #[cfg(feature = "str")]
    plural: "square miles",
};
#[cfg(test)]
mod test {
    use super::*;
    use dims_core::unit_creation::*;
    #[test]
    fn test_area() {
        assert_eq!(SQUARE_INCH.from(1.0).as_base(), 0.0254_f32.powi(2).into());
        assert_eq!(SQUARE_INCH.from(432.0), SQUARE_FOOT.from(3.0));
        assert_eq!(SQUARE_FOOT.from(18.0), SQUARE_YARD.from(2.0));
        assert_eq!(SQUARE_YARD.from(6_195_200.0), SQUARE_MILE.from(2.0));
    }
}
