//! Units of volume used for both wet and dry that fall under the "US Customary" label.
//! Units falling under "US Legal"

use crate::systems::VolumeSystem;
use dims_core::unit_creation::*;

pub const TEASPOON: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: FLUID_OUNCE_CUSTOMARY.ratio / 6.0,
};

pub const TABLESPOON: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: FLUID_OUNCE_CUSTOMARY.ratio / 2.0,
};

pub const CUP_CUSTOMARY: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.0002365882365,
};

pub const FLUID_OUNCE_CUSTOMARY: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: CUP_CUSTOMARY.ratio / 8.0,
};
