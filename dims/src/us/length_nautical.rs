//! Units generally only used in the nautical context.
pub use super::length::LengthUnit;

pub const FATHOM: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: 1143.0 / 625.0,
    #[cfg(feature = "str")]
    abbr: "ftm",
    #[cfg(feature = "str")]
    singular: "fathom",
    #[cfg(feature = "str")]
    plural: "fathoms",
};

pub const CABLE: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: FATHOM.ratio * 120.0,
    #[cfg(feature = "str")]
    abbr: "cb",
    #[cfg(feature = "str")]
    singular: "cable",
    #[cfg(feature = "str")]
    plural: "cables",
};

pub const MILE_NAUTICAL: LengthUnit = LengthUnit {
    offset: 0.0,
    ratio: 1852.0,
    #[cfg(feature = "str")]
    abbr: "nmi",
    #[cfg(feature = "str")]
    singular: "nautical mile",
    #[cfg(feature = "str")]
    plural: "nautical miles",
};

#[cfg(test)]
mod test {
    use super::*;
    use dims_core::unit_creation::*;
    #[test]
    fn test_length_nautical() {
        assert_eq!(FATHOM.from(1.0).as_base(), 1.8288);
        assert_eq!(FATHOM.from(240.0), CABLE.from(2.0));
        assert_eq!(MILE_NAUTICAL.from(1.0).as_base(), 1852.0);
    }
}
