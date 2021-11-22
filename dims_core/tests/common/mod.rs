pub use dims_core::unit_creation::*;

pub const INCH: UnitFormat<Length> = UnitFormat::<Length> {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.0254,
    abbr: "in",
    singular: "inch",
    plural: "inches",
};
#[derive(Copy, Clone, PartialEq)]
pub struct Length;
impl<'t> MeasureSystem<'t> for Length {
    const DEBUG_UNIT: UnitFormat<'t, Self> = INCH;
}

#[derive(Copy, Clone, PartialEq)]
pub struct Area;
impl<'t> MeasureSystem<'t> for Area {
    const DEBUG_UNIT: UnitFormat<'t, Self> = SQIN;
}
pub const SQIN: UnitFormat<Area> = UnitFormat::<Area> {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.09290304 / 144.0,
    #[cfg(feature = "str")]
    abbr: "in²",
    #[cfg(feature = "str")]
    singular: "square inch",
    #[cfg(feature = "str")]
    plural: "square inches",
};
pub const SQFT: UnitFormat<Area> = UnitFormat::<Area> {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.09290304,
    #[cfg(feature = "str")]
    abbr: "ft²",
    #[cfg(feature = "str")]
    singular: "square foot",
    #[cfg(feature = "str")]
    plural: "square feet",
};

#[derive(Copy, Clone, PartialEq)]
pub struct Volume;
impl<'t> MeasureSystem<'t> for Volume {
    const DEBUG_UNIT: UnitFormat<'t, Self> = CBIN;
}

pub const CBIN: UnitFormat<Volume> = UnitFormat::<Volume> {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.764554857984 / 144.0 / 1728.0,
    #[cfg(feature = "str")]
    abbr: "in³",
    #[cfg(feature = "str")]
    singular: "inch",
    #[cfg(feature = "str")]
    plural: "inches",
};