//! Basic Length measurements (inch, foot, mile, point, pica, etc)
//! These would be considered the most "common" length measurements for general use.
//! See also: `length_survey` and `length_nautical`

use crate::systems::LengthSystem;

pub type LengthUnit<'t> = super::UnitType<'t, LengthSystem>;

pub const POINT: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: 127.0 / 360_000.0,
    #[cfg(feature = "str")]
    abbr: "p",
    #[cfg(feature = "str")]
    singular: "point",
    #[cfg(feature = "str")]
    plural: "points",
};

pub const PICA: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: 127.0 / 30_000.0,
    #[cfg(feature = "str")]
    abbr: "pc",
    #[cfg(feature = "str")]
    singular: "pica",
    #[cfg(feature = "str")]
    plural: "picas",
};

pub const INCH: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: 0.0254,
    #[cfg(feature = "str")]
    abbr: "in",
    #[cfg(feature = "str")]
    singular: "inch",
    #[cfg(feature = "str")]
    plural: "inches",
};

pub const FOOT: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: 0.3048,
    #[cfg(feature = "str")]
    abbr: "ft",
    #[cfg(feature = "str")]
    singular: "foot",
    #[cfg(feature = "str")]
    plural: "feet",
};

pub const YARD: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: 0.9144,
    #[cfg(feature = "str")]
    abbr: "yd",
    #[cfg(feature = "str")]
    singular: "yard",
    #[cfg(feature = "str")]
    plural: "yards",
};

pub const MILE: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: 1609.344,
    #[cfg(feature = "str")]
    abbr: "mi",
    #[cfg(feature = "str")]
    singular: "mile",
    #[cfg(feature = "str")]
    plural: "miles",
};

#[cfg(test)]
mod test {
    use super::*;
    use dims_core::unit_creation::*;
    #[test]
    fn test_length() {
        assert_eq!(INCH.from(1.0).as_base(), 0.0254);
        assert_eq!(POINT.from(24.0), PICA.from(2.0));
        assert_eq!(PICA.from(12.0), INCH.from(2.0));
        assert_eq!(INCH.from(24.0), FOOT.from(2.0));
        assert_eq!(FOOT.from(6.0), YARD.from(2.0));
        assert_eq!(YARD.from(3520.0), MILE.from(2.0));
    }
}
