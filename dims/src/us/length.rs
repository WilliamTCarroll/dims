//! Basic Length measurements (inch, foot, mile, point, pica, etc)
//! These would be considered the most "common" length measurements for general use.
//! See also: `length_survey` and `length_nautical`

use crate::systems::LengthSystem;
use dims_core::unit_creation::*;

pub const POINT: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 127.0 / 360_000.0,
    #[cfg(feature = "std")]
    abbr: "p",
    #[cfg(feature = "std")]
    singular: "point",
    #[cfg(feature = "std")]
    plural: "points",
};

pub const PICA: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 127.0 / 30_000.0,
    #[cfg(feature = "std")]
    abbr: "pc",
    #[cfg(feature = "std")]
    singular: "pica",
    #[cfg(feature = "std")]
    plural: "picas",
};

pub const INCH: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.0254,
    #[cfg(feature = "std")]
    abbr: "in",
    #[cfg(feature = "std")]
    singular: "inch",
    #[cfg(feature = "std")]
    plural: "inches",
};

pub const FOOT: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.3048,
    #[cfg(feature = "std")]
    abbr: "ft",
    #[cfg(feature = "std")]
    singular: "foot",
    #[cfg(feature = "std")]
    plural: "feet",
};

pub const YARD: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.9144,
    #[cfg(feature = "std")]
    abbr: "yd",
    #[cfg(feature = "std")]
    singular: "yard",
    #[cfg(feature = "std")]
    plural: "yards",
};

pub const MILE: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1609.344,
    #[cfg(feature = "std")]
    abbr: "mi",
    #[cfg(feature = "std")]
    singular: "mile",
    #[cfg(feature = "std")]
    plural: "miles",
};

#[cfg(test)]
mod test {
    use super::*;
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
