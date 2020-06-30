//! Units of volume used for both wet and dry that fall under the "US Customary" label.
//! Units falling under "US Legal"

use crate::systems::Volume;
use dims_core::unit_creation::*;

pub const CUP_NUTRITIONAL: UnitSimple<Volume> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.00024,
};

pub const FLUID_OUNCE_NUTRITIONAL: UnitSimple<Volume> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: CUP_NUTRITIONAL.ratio / 8.0,
};
