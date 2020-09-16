//! Length units still in common use in the US for surveying.
//! Some of these will vary slightly from "general" units (EX: 1.0 FOOT != 1.0 FOOT_SURVEY)

use crate::systems::LengthSystem;
use dims_core::unit_creation::*;

pub const LINK: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 792.0 / 3937.0,
    #[cfg(feature = "std")]
    abbr: "li",
    #[cfg(feature = "std")]
    singular: "link",
    #[cfg(feature = "std")]
    plural: "links",
};

pub const FOOT_SURVEY: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1200.0 / 3937.0,
    #[cfg(feature = "std")]
    abbr: "ft",
    #[cfg(feature = "std")]
    singular: "foot",
    #[cfg(feature = "std")]
    plural: "feet",
};

pub const ROD: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: LINK.ratio * 25.0,
    #[cfg(feature = "std")]
    abbr: "rd",
    #[cfg(feature = "std")]
    singular: "rod",
    #[cfg(feature = "std")]
    plural: "rods",
};

pub const CHAIN: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: ROD.ratio * 4.0,
    #[cfg(feature = "std")]
    abbr: "ch",
    #[cfg(feature = "std")]
    singular: "chain",
    #[cfg(feature = "std")]
    plural: "chains",
};

pub const FURLONG: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: CHAIN.ratio * 10.0,
    #[cfg(feature = "std")]
    abbr: "fur",
    #[cfg(feature = "std")]
    singular: "furlong",
    #[cfg(feature = "std")]
    plural: "furlongs",
};

pub const MILE_SURVEY: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: FURLONG.ratio * 8.0,
    #[cfg(feature = "std")]
    abbr: "mi",
    #[cfg(feature = "std")]
    singular: "mile",
    #[cfg(feature = "std")]
    plural: "miles",
};

pub const LEAGUE: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: MILE_SURVEY.ratio * 3.0,
    #[cfg(feature = "std")]
    abbr: "lea",
    #[cfg(feature = "std")]
    singular: "league",
    #[cfg(feature = "std")]
    plural: "leagues",
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
