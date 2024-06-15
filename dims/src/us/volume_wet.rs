//! Measurements generally used for wet measurements
//! See also `volume` and `volume_dry`

pub use super::volume::VolumeUnit;

pub const PINT_WET: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: GALLON_WET.ratio / 8.0,
    #[cfg(feature = "str")]
    abbr: "pt",
    #[cfg(feature = "str")]
    singular: "pint",
    #[cfg(feature = "str")]
    plural: "pints",
};

pub const QUART_WET: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: GALLON_WET.ratio / 4.0,
    #[cfg(feature = "str")]
    abbr: "qt",
    #[cfg(feature = "str")]
    singular: "quart",
    #[cfg(feature = "str")]
    plural: "quarts",
};

pub const POTTLE: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: GALLON_WET.ratio / 2.0,
    #[cfg(feature = "str")]
    abbr: "pot",
    #[cfg(feature = "str")]
    singular: "pottle",
    #[cfg(feature = "str")]
    plural: "pottles",
};

pub const GALLON_WET: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: 0.003785411784,
    #[cfg(feature = "str")]
    abbr: "gal",
    #[cfg(feature = "str")]
    singular: "gallon",
    #[cfg(feature = "str")]
    plural: "gallons",
};

pub const BARREL_WET: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: GALLON_WET.ratio * 31.5,
    #[cfg(feature = "str")]
    abbr: "bbl",
    #[cfg(feature = "str")]
    singular: "barrel",
    #[cfg(feature = "str")]
    plural: "barrels",
};

pub const BARREL_OIL: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: GALLON_WET.ratio * 42.0,
    #[cfg(feature = "str")]
    abbr: "bbl",
    #[cfg(feature = "str")]
    singular: "oil barrel",
    #[cfg(feature = "str")]
    plural: "oil barrels",
};

pub const HOGSHEAD: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: GALLON_WET.ratio * 63.0,
    #[cfg(feature = "str")]
    abbr: "hhd",
    #[cfg(feature = "str")]
    singular: "hogshead",
    #[cfg(feature = "str")]
    plural: "hogsheads",
};
