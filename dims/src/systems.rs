use dims_core::unit_creation::*;
#[derive(MeasureSystem)]
/// Length measures simple linear units
///
/// The base unit for this is the Metre
///
/// The following operations are supported (as well as addition and subtraction):
/// - Length * Length = Area
/// - Length * Area = Volume
pub struct Length;
impl MultiplyBy<Length> for Length {
    type Output = Area;
}
impl MultiplyBy<Area> for Length {
    type Output = Volume;
}

#[derive(MeasureSystem)]
/// Unit measures two dimensional length
///
/// The base unit for this is the Square Metre
///
/// The following operations are supported (as well as addition and subtraction):
/// - Length * Area = Volume
/// - Area / Length = Length
pub struct Area;
impl MultiplyBy<Length> for Area {
    type Output = Volume;
}
impl DivideBy<Length> for Area {
    type Output = Length;
}

#[derive(MeasureSystem)]
/// Volume measures three dimensional length
///
/// The base unit for this is the Cubic Metre
///
/// The following operations are supported (as well as addition and subtraction):
/// - Volume / Length = Area
/// - Volume / Area = Length
pub struct Volume;
impl DivideBy<Length> for Volume {
    type Output = Area;
}
impl DivideBy<Area> for Volume {
    type Output = Length;
}

#[derive(MeasureSystem)]
/// Temperature measures what it sounds like.
///
/// The base unit is Kelvin
///
/// There is no check for negative values,
/// as something like an endothermic reaction
/// would subtract from the current temperature
pub struct Temperature;

#[derive(MeasureSystem)]
/// Mass measures the amount of matter in an object.
///
/// The base unit of this is Gram.
pub struct Mass;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_system_size() {
        use std::mem::size_of_val;
        // We need to ensure these always have a zero size
        assert_eq!(0, size_of_val(&Length));
        assert_eq!(0, size_of_val(&Area));
        assert_eq!(0, size_of_val(&Volume));
    }
}
