//! Measurements generally used for dry measurements
//! See also `volume` and `volume_wet`

use super::volume::CUBIC_INCH;
use crate::systems::VolumeSystem;
use dims_core::unit_creation::*;

pub const PINT_DRY: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.5506104713575,
};

pub const QUART_DRY: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: PINT_DRY.ratio * 2.0,
};

pub const GALLON_DRY: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: QUART_DRY.ratio * 4.0,
};

pub const PECK: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: GALLON_DRY.ratio * 2.0,
};

pub const BUSHEL: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: PECK.ratio * 4.0,
};

pub const BARREL_DRY: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: CUBIC_INCH.ratio * 7056.0,
};
