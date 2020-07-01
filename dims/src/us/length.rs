//! Basic Length measurements (inch, foot, mile, point, pica, etc)
//! These would be considered the most "common" length measurements for general use.
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
