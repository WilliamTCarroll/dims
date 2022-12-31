pub use dims_core::unit_creation::*;

pub const INCH: UnitFormat<'static, Length> = UnitFormat {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.0254,
    abbr: "in",
    singular: "inch",
    plural: "inches",
};
#[derive(Copy, Clone, PartialEq)]
pub struct Length;
impl MeasureSystem for Length {
    type N = f32;
    const DEBUG_UNIT: UnitFormat<'static, Self> = INCH;
}

#[derive(Copy, Clone, PartialEq)]
pub struct Area;
impl MeasureSystem for Area {
    type N = f32;
    const DEBUG_UNIT: UnitFormat<'static, Self> = SQIN;
}
pub const SQIN: UnitFormat<'static, Area> = UnitFormat {
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
pub const SQFT: UnitFormat<'static, Area> = UnitFormat {
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
impl MeasureSystem for Volume {
    type N = f32;
    const DEBUG_UNIT: UnitFormat<'static, Self> = CBIN;
}

pub const CBIN: UnitFormat<'static, Volume> = UnitFormat::<'static, Volume> {
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
