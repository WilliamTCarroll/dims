//! Units of volume used for both wet and dry that fall under the "US Customary" label.
//! Units falling under "US Legal"

use crate::systems::VolumeSystem;
use dims_core::unit_creation::*;

pub const CUP_NUTRITIONAL: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.00024,
    #[cfg(feature = "std")]
    abbr: "cp",
    #[cfg(feature = "std")]
    singular: "cup",
    #[cfg(feature = "std")]
    plural: "cups",
};

pub const FLUID_OUNCE_NUTRITIONAL: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: CUP_NUTRITIONAL.ratio / 8.0,
    #[cfg(feature = "std")]
    abbr: "fl oz",
    #[cfg(feature = "std")]
    singular: "fluid ounce",
    #[cfg(feature = "std")]
    plural: "fluid ounces",
};
