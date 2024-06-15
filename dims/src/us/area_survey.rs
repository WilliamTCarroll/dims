//! Units of area that are generally used in surveying.
//! The ACRE is re-exported in the general `area` mod
pub use super::area::AreaUnit;

pub const SQUARE_FOOT_SURVEY: AreaUnit = AreaUnit {
    offset: 0.0,
    ratio: 1_440_000.0 / 15_499_969.0,
    #[cfg(feature = "str")]
    abbr: "ft²",
    #[cfg(feature = "str")]
    singular: "square foot",
    #[cfg(feature = "str")]
    plural: "square feet",
};

pub const SQUARE_ROD: AreaUnit = AreaUnit {
    offset: 0.0,
    ratio: SQUARE_CHAIN.ratio / 16.0,
    #[cfg(feature = "str")]
    abbr: "rd²",
    #[cfg(feature = "str")]
    singular: "square rod",
    #[cfg(feature = "str")]
    plural: "square rods",
};

pub const SQUARE_CHAIN: AreaUnit = AreaUnit {
    offset: 0.0,
    ratio: SQUARE_FOOT_SURVEY.ratio * 4356.0,
    #[cfg(feature = "str")]
    abbr: "ch²",
    #[cfg(feature = "str")]
    singular: "square chain",
    #[cfg(feature = "str")]
    plural: "square chains",
};

pub const ACRE: AreaUnit = AreaUnit {
    offset: 0.0,
    ratio: SQUARE_CHAIN.ratio * 10.0,
    #[cfg(feature = "str")]
    abbr: "ac",
    #[cfg(feature = "str")]
    singular: "acre",
    #[cfg(feature = "str")]
    plural: "acres",
};

pub const SECTION: AreaUnit = AreaUnit {
    offset: 0.0,
    ratio: ACRE.ratio * 640.0,
    #[cfg(feature = "str")]
    abbr: "sec",
    #[cfg(feature = "str")]
    singular: "section",
    #[cfg(feature = "str")]
    plural: "sections",
};

pub const SQUARE_LEAGUE: AreaUnit = AreaUnit {
    offset: 0.0,
    ratio: SECTION.ratio * 9.0,
    #[cfg(feature = "str")]
    abbr: "lea²",
    #[cfg(feature = "str")]
    singular: "square league",
    #[cfg(feature = "str")]
    plural: "square leagues",
};

pub const SURVEY_TOWNSHIP: AreaUnit = AreaUnit {
    offset: 0.0,
    ratio: SECTION.ratio * 36.0,
    #[cfg(feature = "str")]
    abbr: "twp",
    #[cfg(feature = "str")]
    singular: "township",
    #[cfg(feature = "str")]
    plural: "townships",
};
#[cfg(test)]
mod test {
    use super::*;
    use dims_core::unit_creation::*;
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
