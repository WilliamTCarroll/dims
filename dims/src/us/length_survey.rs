//! Length units still in common use in the US for surveying.
//! Some of these will vary slightly from "general" units (EX: 1.0 FOOT != 1.0 FOOT_SURVEY)

use crate::systems::Length;
use dims_core::unit_creation::*;

pub const LINK: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 792.0 / 3937.0,
};

pub const FOOT_SURVEY: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1200.0 / 3937.0,
};

pub const ROD: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: LINK.ratio * 25.0,
};

pub const CHAIN: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: ROD.ratio * 4.0,
};

pub const FURLONG: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: CHAIN.ratio * 10.0,
};

pub const MILE_SURVEY: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: FURLONG.ratio * 8.0,
};

pub const LEAGUE: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: MILE_SURVEY.ratio * 3.0,
};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_length_survey() {
        assert_eq!(LINK.from(1.0).as_base(), 0.2011684);
        assert_eq!(FOOT_SURVEY.from(1.0).as_base(), 0.3048006);
        assert_eq!(LINK.from(50.0), ROD.from(2.0));
        assert_eq!(ROD.from(8.0), CHAIN.from(2.0));
        assert_eq!(CHAIN.from(20.0), FURLONG.from(2.0));
        assert_eq!(FURLONG.from(16.0), MILE_SURVEY.from(2.0));
        assert_eq!(MILE_SURVEY.from(6.0), LEAGUE.from(2.0));
    }
}
