use dims_core::unit_creation::*;

#[cfg(not(all(feature = "us", feature = "debug_us")))]
use crate::si::debug_units::*;
#[cfg(all(feature = "us", feature = "debug_us"))]
use crate::us::debug_units::*;
use crate::Flt;

measure_system!(name: LengthSystem, debug_unit: LENGTH_D, data_type: Flt);
impl MultiplyBy<LengthSystem> for LengthSystem {
    type Output = AreaSystem;
}
impl MultiplyBy<AreaSystem> for LengthSystem {
    type Output = VolumeSystem;
}

measure_system!(name: AreaSystem, debug_unit: AREA_D, data_type: Flt);
impl MultiplyBy<LengthSystem> for AreaSystem {
    type Output = VolumeSystem;
}
impl DivideBy<LengthSystem> for AreaSystem {
    type Output = LengthSystem;
}

measure_system!(name: VolumeSystem, debug_unit: VOLUME_D, data_type: Flt);
impl DivideBy<LengthSystem> for VolumeSystem {
    type Output = AreaSystem;
}
impl DivideBy<AreaSystem> for VolumeSystem {
    type Output = LengthSystem;
}

measure_system!(
    name: TemperatureSystem,
    debug_unit: TEMPERATURE_D,
    data_type: Flt
);

measure_system!(name: MassSystem, debug_unit: MASS_D, data_type: Flt);
