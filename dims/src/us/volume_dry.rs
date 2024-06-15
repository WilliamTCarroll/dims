//! Measurements generally used for dry measurements
//! See also `volume` and `volume_wet`

pub use super::volume::VolumeUnit;
use super::volume::CUBIC_INCH;

pub const PINT_DRY: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: 0.5506104713575,
    #[cfg(feature = "str")]
    abbr: "pt",
    #[cfg(feature = "str")]
    singular: "pint",
    #[cfg(feature = "str")]
    plural: "pints",
};

pub const QUART_DRY: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: PINT_DRY.ratio * 2.0,
    #[cfg(feature = "str")]
    abbr: "qt",
    #[cfg(feature = "str")]
    singular: "quart",
    #[cfg(feature = "str")]
    plural: "quarts",
};

pub const GALLON_DRY: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: QUART_DRY.ratio * 4.0,
    #[cfg(feature = "str")]
    abbr: "gal",
    #[cfg(feature = "str")]
    singular: "gallon",
    #[cfg(feature = "str")]
    plural: "gallons",
};

pub const PECK: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: GALLON_DRY.ratio * 2.0,
    #[cfg(feature = "str")]
    abbr: "pk",
    #[cfg(feature = "str")]
    singular: "peck",
    #[cfg(feature = "str")]
    plural: "pecks",
};

pub const BUSHEL: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: PECK.ratio * 4.0,
    #[cfg(feature = "str")]
    abbr: "bu",
    #[cfg(feature = "str")]
    singular: "bushel",
    #[cfg(feature = "str")]
    plural: "bushels",
};

pub const BARREL_DRY: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: CUBIC_INCH.ratio * 7056.0,
    #[cfg(feature = "str")]
    abbr: "bbl",
    #[cfg(feature = "str")]
    singular: "barrel",
    #[cfg(feature = "str")]
    plural: "barrels",
};
