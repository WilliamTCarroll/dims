//! Units of volume used for both wet and dry that fall under the "US Customary" label.
//! Units falling under "US Legal"

pub use super::volume::VolumeUnit;
use dims_core::unit_creation::*;

pub const TEASPOON: VolumeUnit = VolumeUnit {
    system: PhantomData,
    offset: 0.0,
    ratio: FLUID_OUNCE_CUSTOMARY.ratio / 6.0,
    #[cfg(feature = "str")]
    abbr: "tsp",
    #[cfg(feature = "str")]
    singular: "teaspoon",
    #[cfg(feature = "str")]
    plural: "teaspoons",
};

pub const TABLESPOON: VolumeUnit = VolumeUnit {
    system: PhantomData,
    offset: 0.0,
    ratio: FLUID_OUNCE_CUSTOMARY.ratio / 2.0,
    #[cfg(feature = "str")]
    abbr: "Tbsp",
    #[cfg(feature = "str")]
    singular: "tablespoon",
    #[cfg(feature = "str")]
    plural: "tablespoons",
};

pub const CUP_CUSTOMARY: VolumeUnit = VolumeUnit {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.0002365882365,
    #[cfg(feature = "str")]
    abbr: "cp",
    #[cfg(feature = "str")]
    singular: "cup",
    #[cfg(feature = "str")]
    plural: "cups",
};

pub const FLUID_OUNCE_CUSTOMARY: VolumeUnit = VolumeUnit {
    system: PhantomData,
    offset: 0.0,
    ratio: CUP_CUSTOMARY.ratio / 8.0,
    #[cfg(feature = "str")]
    abbr: "fl oz",
    #[cfg(feature = "str")]
    singular: "fluid ounce",
    #[cfg(feature = "str")]
    plural: "fluid ounces",
};
