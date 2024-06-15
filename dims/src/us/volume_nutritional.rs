//! Units of volume used for both wet and dry that fall under the "US Customary" label.
//! Units falling under "US Legal"

pub use super::volume::VolumeUnit;

pub const CUP_NUTRITIONAL: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: 0.00024,
    #[cfg(feature = "str")]
    abbr: "cp",
    #[cfg(feature = "str")]
    singular: "cup",
    #[cfg(feature = "str")]
    plural: "cups",
};

pub const FLUID_OUNCE_NUTRITIONAL: VolumeUnit = VolumeUnit {
    offset: 0.0,
    ratio: CUP_NUTRITIONAL.ratio / 8.0,
    #[cfg(feature = "str")]
    abbr: "fl oz",
    #[cfg(feature = "str")]
    singular: "fluid ounce",
    #[cfg(feature = "str")]
    plural: "fluid ounces",
};
