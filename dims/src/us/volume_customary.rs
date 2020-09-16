//! Units of volume used for both wet and dry that fall under the "US Customary" label.
//! Units falling under "US Legal"

use crate::systems::VolumeSystem;
use dims_core::unit_creation::*;

pub const TEASPOON: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: FLUID_OUNCE_CUSTOMARY.ratio / 6.0,
    #[cfg(feature = "std")]
    abbr: "tsp",
    #[cfg(feature = "std")]
    singular: "teaspoon",
    #[cfg(feature = "std")]
    plural: "teaspoons",
};

pub const TABLESPOON: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: FLUID_OUNCE_CUSTOMARY.ratio / 2.0,
    #[cfg(feature = "std")]
    abbr: "Tbsp",
    #[cfg(feature = "std")]
    singular: "tablespoon",
    #[cfg(feature = "std")]
    plural: "tablespoons",
};

pub const CUP_CUSTOMARY: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.0002365882365,
    #[cfg(feature = "std")]
    abbr: "cp",
    #[cfg(feature = "std")]
    singular: "cup",
    #[cfg(feature = "std")]
    plural: "cups",
};

pub const FLUID_OUNCE_CUSTOMARY: UnitSimple<VolumeSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: CUP_CUSTOMARY.ratio / 8.0,
    #[cfg(feature = "std")]
    abbr: "fl oz",
    #[cfg(feature = "std")]
    singular: "fluid ounce",
    #[cfg(feature = "std")]
    plural: "fluid ounces",
};
