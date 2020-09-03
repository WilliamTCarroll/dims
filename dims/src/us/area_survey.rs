//! Units of area that are generally used in surveying.
//! The ACRE is re-exported in the general `area` mod
use crate::systems::AreaSystem;
use dims_core::unit_creation::*;

pub const SQUARE_FOOT_SURVEY: UnitSimple<AreaSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1_440_000.0 / 15_499_969.0,
};

pub const SQUARE_ROD: UnitSimple<AreaSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: SQUARE_CHAIN.ratio / 16.0,
};

pub const SQUARE_CHAIN: UnitSimple<AreaSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: SQUARE_FOOT_SURVEY.ratio * 4356.0,
};

pub const ACRE: UnitSimple<AreaSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: SQUARE_CHAIN.ratio * 10.0,
};

pub const SECTION: UnitSimple<AreaSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: ACRE.ratio * 640.0,
};

pub const SQUARE_LEAGUE: UnitSimple<AreaSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: SECTION.ratio * 9.0,
};

pub const SURVEY_TOWNSHIP: UnitSimple<AreaSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: SECTION.ratio * 36.0,
};
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_area_survey() {
        assert_eq!(SQUARE_FOOT_SURVEY.from(1.0).as_base(), 0.09290341);
        assert_eq!(SQUARE_ROD.from(32.0), SQUARE_CHAIN.from(2.0));
        assert_eq!(SQUARE_FOOT_SURVEY.from(8712.0), SQUARE_CHAIN.from(2.0));
        assert_eq!(SQUARE_CHAIN.from(20.0), ACRE.from(2.0));
        assert_eq!(ACRE.from(1280.0), SECTION.from(2.0));
        assert_eq!(SECTION.from(72.0), SURVEY_TOWNSHIP.from(2.0));
        assert_eq!(SQUARE_LEAGUE.from(8.0), SURVEY_TOWNSHIP.from(2.0));
    }
}
