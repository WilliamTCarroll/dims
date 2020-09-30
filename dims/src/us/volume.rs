//! General volume measurements
//! See also: `volume_wet` and `volume_dry`

use crate::systems::VolumeSystem;
use dims_core::unit_creation::*;

pub type VolumeUnit<'t> = super::UnitType<'t, VolumeSystem>;

pub const CUBIC_INCH: VolumeUnit = VolumeUnit {
    system: PhantomData,
    offset: 0.0,
    ratio: CUBIC_FOOT.ratio / 1728.0,
    #[cfg(feature = "str")]
    abbr: "in³",
    #[cfg(feature = "str")]
    singular: "inch",
    #[cfg(feature = "str")]
    plural: "inches",
};

pub const CUBIC_FOOT: VolumeUnit = VolumeUnit {
    system: PhantomData,
    offset: 0.0,
    ratio: CUBIC_YARD.ratio / 27.0,
    #[cfg(feature = "str")]
    abbr: "ft³",
    #[cfg(feature = "str")]
    singular: "foot",
    #[cfg(feature = "str")]
    plural: "feet",
};
pub const CUBIC_YARD: VolumeUnit = VolumeUnit {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.764554857984,
    #[cfg(feature = "str")]
    abbr: "yd³",
    #[cfg(feature = "str")]
    singular: "cubic yard",
    #[cfg(feature = "str")]
    plural: "cubic yards",
};

pub const ACRE_FOOT: VolumeUnit = VolumeUnit {
    system: PhantomData,
    offset: 0.0,
    ratio: CUBIC_FOOT.ratio * 43_560.0,
    #[cfg(feature = "str")]
    abbr: "ac-ft",
    #[cfg(feature = "str")]
    singular: "acre-foot",
    #[cfg(feature = "str")]
    plural: "acre-feet",
};

/// Please note that this may have some floating point Fun. \
/// There is no nice round ratio for this to cubic metres
pub const ACRE_FOOT_SURVEY: VolumeUnit = VolumeUnit {
    system: PhantomData,
    offset: 0.0,
    ratio: 1233.4892384681,
    #[cfg(feature = "str")]
    abbr: "ac-ft",
    #[cfg(feature = "str")]
    singular: "acre-foot",
    #[cfg(feature = "str")]
    plural: "acre-feet",
};
