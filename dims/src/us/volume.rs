//! General volume measurements
//! See also: `volume_wet` and `volume_dry`

use crate::systems::VolumeSystem;
use dims_core::unit_creation::*;

pub const CUBIC_INCH: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: CUBIC_FOOT.ratio / 1728.0,
};

pub const CUBIC_FOOT: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: CUBIC_YARD.ratio / 27.0,
};

pub const ACRE_FOOT: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: CUBIC_FOOT.ratio * 43_560.0,
};

/// Please note that this may have some floating point Fun. \
/// There is no nice round ratio for this to cubic metres
pub const ACRE_FOOT_SURVEY: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1233.4892384681,
};

pub const CUBIC_YARD: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.764554857984,
};
