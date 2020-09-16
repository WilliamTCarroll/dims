//! Measurements generally used for wet measurements
//! See also `volume` and `volume_dry`

use crate::systems::VolumeSystem;
use dims_core::unit_creation::*;

pub const PINT_WET: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: GALLON_WET.ratio / 8.0,
    #[cfg(feature = "std")]
    abbr: "pt",
    #[cfg(feature = "std")]
    singular: "pint",
    #[cfg(feature = "std")]
    plural: "pints",
};

pub const QUART_WET: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: GALLON_WET.ratio / 4.0,
    #[cfg(feature = "std")]
    abbr: "qt",
    #[cfg(feature = "std")]
    singular: "quart",
    #[cfg(feature = "std")]
    plural: "quarts",
};

pub const POTTLE: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: GALLON_WET.ratio / 2.0,
    #[cfg(feature = "std")]
    abbr: "pot",
    #[cfg(feature = "std")]
    singular: "pottle",
    #[cfg(feature = "std")]
    plural: "pottles",
};

pub const GALLON_WET: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.003785411784,
    #[cfg(feature = "std")]
    abbr: "gal",
    #[cfg(feature = "std")]
    singular: "gallon",
    #[cfg(feature = "std")]
    plural: "gallons",
};

pub const BARREL_WET: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: GALLON_WET.ratio * 31.5,
    #[cfg(feature = "std")]
    abbr: "bbl",
    #[cfg(feature = "std")]
    singular: "barrel",
    #[cfg(feature = "std")]
    plural: "barrels",
};

pub const BARREL_OIL: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: GALLON_WET.ratio * 42.0,
    #[cfg(feature = "std")]
    abbr: "bbl",
    #[cfg(feature = "std")]
    singular: "oil barrel",
    #[cfg(feature = "std")]
    plural: "oil barrels",
};

pub const HOGSHEAD: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: GALLON_WET.ratio * 63.0,
    #[cfg(feature = "std")]
    abbr: "hhd",
    #[cfg(feature = "std")]
    singular: "hogshead",
    #[cfg(feature = "std")]
    plural: "hogsheads",
};
