//! Length units still in common use in the US for surveying.
//! Some of these will vary slightly from "general" units (EX: 1.0 FOOT != 1.0 FOOT_SURVEY)
pub use super::length::LengthUnit;

pub const LINK: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: 792.0 / 3937.0,
    #[cfg(feature = "str")]
    abbr: "li",
    #[cfg(feature = "str")]
    singular: "link",
    #[cfg(feature = "str")]
    plural: "links",
};

pub const FOOT_SURVEY: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: 1200.0 / 3937.0,
    #[cfg(feature = "str")]
    abbr: "ft",
    #[cfg(feature = "str")]
    singular: "foot",
    #[cfg(feature = "str")]
    plural: "feet",
};

pub const ROD: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: LINK.ratio * 25.0,
    #[cfg(feature = "str")]
    abbr: "rd",
    #[cfg(feature = "str")]
    singular: "rod",
    #[cfg(feature = "str")]
    plural: "rods",
};

pub const CHAIN: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: ROD.ratio * 4.0,
    #[cfg(feature = "str")]
    abbr: "ch",
    #[cfg(feature = "str")]
    singular: "chain",
    #[cfg(feature = "str")]
    plural: "chains",
};

pub const FURLONG: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: CHAIN.ratio * 10.0,
    #[cfg(feature = "str")]
    abbr: "fur",
    #[cfg(feature = "str")]
    singular: "furlong",
    #[cfg(feature = "str")]
    plural: "furlongs",
};

pub const MILE_SURVEY: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: FURLONG.ratio * 8.0,
    #[cfg(feature = "str")]
    abbr: "mi",
    #[cfg(feature = "str")]
    singular: "mile",
    #[cfg(feature = "str")]
    plural: "miles",
};

pub const LEAGUE: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: MILE_SURVEY.ratio * 3.0,
    #[cfg(feature = "str")]
    abbr: "lea",
    #[cfg(feature = "str")]
    singular: "league",
    #[cfg(feature = "str")]
    plural: "leagues",
};

#[cfg(test)]
mod test {
    use super::*;
    use dims_core::unit_creation::*;
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
