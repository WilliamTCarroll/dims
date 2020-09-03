//! Measurements generally used for wet measurements
//! See also `volume` and `volume_dry`

use crate::systems::VolumeSystem;
use dims_core::unit_creation::*;

pub const PINT_WET: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: GALLON_WET.ratio / 8.0,
};

pub const QUART_WET: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: GALLON_WET.ratio / 4.0,
};

pub const POTTLE: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: GALLON_WET.ratio / 2.0,
};

pub const GALLON_WET: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.003785411784,
};

pub const BARREL_WET: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: GALLON_WET.ratio * 31.5,
};

pub const BARREL_OIL: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: GALLON_WET.ratio * 42.0,
};

pub const HOGSHEAD: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: GALLON_WET.ratio * 63.0,
};
