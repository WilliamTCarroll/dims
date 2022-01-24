use dims_core::unit_creation::*;

#[cfg(not(all(feature = "us", feature = "debug_us")))]
use crate::si::debug_units::*;
#[cfg(all(feature = "us", feature = "debug_us"))]
use crate::us::debug_units::*;

measure_system!(name: LengthSystem, debug_unit: LENGTH_D);
impl<'t> MultiplyBy<'t, LengthSystem> for LengthSystem {
    type Output = AreaSystem;
}
impl<'t> MultiplyBy<'t, AreaSystem> for LengthSystem {
    type Output = VolumeSystem;
}

measure_system!(name: AreaSystem, debug_unit: AREA_D);
impl<'t> MultiplyBy<'t, LengthSystem> for AreaSystem {
    type Output = VolumeSystem;
}
impl<'t> DivideBy<'t, LengthSystem> for AreaSystem {
    type Output = LengthSystem;
}

measure_system!(name: VolumeSystem, debug_unit: VOLUME_D);
impl<'t> DivideBy<'t, LengthSystem> for VolumeSystem {
    type Output = AreaSystem;
}
impl<'t> DivideBy<'t, AreaSystem> for VolumeSystem {
    type Output = LengthSystem;
}

measure_system!(name: TemperatureSystem, debug_unit: TEMPERATURE_D);

measure_system!(name: MassSystem, debug_unit: MASS_D);
