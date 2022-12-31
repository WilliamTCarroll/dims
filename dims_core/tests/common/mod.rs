pub use dims_core::unit_creation::*;

pub const INCH: UnitFormat<'static, Length> = UnitFormat {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.0254,
    abbr: "in",
    singular: "inch",
    plural: "inches",
};
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Length;
impl MeasureSystem for Length {
    type N = f32;
    const DEBUG_UNIT: UnitFormat<'static, Self> = INCH;
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Area;
impl MeasureSystem for Area {
    type N = f32;
    const DEBUG_UNIT: UnitFormat<'static, Self> = SQIN;
}

impl MultiplyBy<Length> for Length {
    type Output = Area;
}

impl DivideBy<Length> for Area {
    type Output = Length;
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

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Volume;
impl MeasureSystem for Volume {
    type N = f32;
    const DEBUG_UNIT: UnitFormat<'static, Self> = CBIN;
}

impl MultiplyBy<Length> for Area {
    type Output = Volume;
}

impl DivideBy<Length> for Volume {
    type Output = Area;
}
pub const CBIN: UnitFormat<'static, Volume> = UnitFormat::<'static, Volume> {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.764554857984 / 27.0 / 1728.0,
    #[cfg(feature = "str")]
    abbr: "in³",
    #[cfg(feature = "str")]
    singular: "inch",
    #[cfg(feature = "str")]
    plural: "inches",
};
