//! Measurements generally used for dry measurements
//! See also `volume` and `volume_wet`

use super::volume::CUBIC_INCH;
use crate::systems::VolumeSystem;
use dims_core::unit_creation::*;

pub const PINT_DRY: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.5506104713575,
    #[cfg(feature = "std")]
    abbr: "pt",
    #[cfg(feature = "std")]
    singular: "pint",
    #[cfg(feature = "std")]
    plural: "pints",
};

pub const QUART_DRY: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: PINT_DRY.ratio * 2.0,
    #[cfg(feature = "std")]
    abbr: "qt",
    #[cfg(feature = "std")]
    singular: "quart",
    #[cfg(feature = "std")]
    plural: "quarts",
};

pub const GALLON_DRY: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: QUART_DRY.ratio * 4.0,
    #[cfg(feature = "std")]
    abbr: "gal",
    #[cfg(feature = "std")]
    singular: "gallon",
    #[cfg(feature = "std")]
    plural: "gallons",
};

pub const PECK: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: GALLON_DRY.ratio * 2.0,
    #[cfg(feature = "std")]
    abbr: "pk",
    #[cfg(feature = "std")]
    singular: "peck",
    #[cfg(feature = "std")]
    plural: "pecks",
};

pub const BUSHEL: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: PECK.ratio * 4.0,
    #[cfg(feature = "std")]
    abbr: "bu",
    #[cfg(feature = "std")]
    singular: "bushel",
    #[cfg(feature = "std")]
    plural: "bushels",
};

pub const BARREL_DRY: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: CUBIC_INCH.ratio * 7056.0,
    #[cfg(feature = "std")]
    abbr: "bbl",
    #[cfg(feature = "std")]
    singular: "barrel",
    #[cfg(feature = "std")]
    plural: "barrels",
};
