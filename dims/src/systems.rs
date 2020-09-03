use dims_core::unit_creation::*;
#[derive(MeasureSystem)]
/// LengthSystem measures simple linear units
///
/// The base unit for this is the Metre
///
/// The following operations are supported (as well as addition and subtraction):
/// - LengthSystem * LengthSystem = AreaSystem
/// - LengthSystem * AreaSystem = VolumeSystem
pub struct LengthSystem;
impl MultiplyBy<LengthSystem> for LengthSystem {
    type Output = AreaSystem;
}
impl MultiplyBy<AreaSystem> for LengthSystem {
    type Output = VolumeSystem;
}

#[derive(MeasureSystem)]
/// Unit measures two dimensional LengthSystem
///
/// The base unit for this is the Square Metre
///
/// The following operations are supported (as well as addition and subtraction):
/// - LengthSystem * AreaSystem = VolumeSystem
/// - AreaSystem / LengthSystem = LengthSystem
pub struct AreaSystem;
impl MultiplyBy<LengthSystem> for AreaSystem {
    type Output = VolumeSystem;
}
impl DivideBy<LengthSystem> for AreaSystem {
    type Output = LengthSystem;
}

#[derive(MeasureSystem)]
/// VolumeSystem measures three dimensional LengthSystem
///
/// The base unit for this is the Cubic Metre
///
/// The following operations are supported (as well as addition and subtraction):
/// - VolumeSystem / LengthSystem = AreaSystem
/// - VolumeSystem / AreaSystem = LengthSystem
pub struct VolumeSystem;
impl DivideBy<LengthSystem> for VolumeSystem {
    type Output = AreaSystem;
}
impl DivideBy<AreaSystem> for VolumeSystem {
    type Output = LengthSystem;
}

#[derive(MeasureSystem)]
/// TemperatureSystem measures what it sounds like.
///
/// The base unit is Kelvin
///
/// There is no check for negative values,
/// as something like an endothermic reaction
/// would subtract from the current temperature
pub struct TemperatureSystem;

#[derive(MeasureSystem)]
/// MassSystem measures the amount of matter in an object.
///
/// The base unit of this is Gram.
pub struct MassSystem;
