use dims::imperial::temperature::*;
use dims::prelude::*;
use dims::si::temperature::*;
/// Specify a function to round a numeric to the specified number of
pub trait RoundTo {
    /// Round the given value to the number of decimals specified
    fn round_to(&self, decimals: i32) -> Self;
}
impl RoundTo for Flt {
    fn round_to(&self, decimals: i32) -> Self {
        let decimals: Flt = (10.0 as Flt).powi(decimals);
        (self * decimals).round() / decimals
    }
}
#[test]
fn temperature_test() {
    let zero = KELVIN.from(0.0);
    assert_eq!(0.0, zero.val_as(&KELVIN));
    assert_eq!(-273.15, zero.val_as(&CELCIUS));
    assert_eq!(0.0, zero.val_as(&RANKINE));
    assert_eq!(-459.67, zero.val_as(&FAHRENHEIT));
    let freeze = CELCIUS.from(0.0);
    assert_eq!(273.15, freeze.val_as(&KELVIN));
    assert_eq!(0.0, freeze.val_as(&CELCIUS));
    assert_eq!(491.6700, freeze.val_as(&RANKINE).round_to(4)); // Yay floating point errors
    assert_eq!(32.0000, freeze.val_as(&FAHRENHEIT).round_to(3));
    let boil = CELCIUS.from(99.9839);
    assert_eq!(373.1339, boil.val_as(&KELVIN));
    assert_eq!(99.98390, boil.val_as(&CELCIUS).round_to(4));
    assert_eq!(671.64102, boil.val_as(&RANKINE));
    assert_eq!(211.9710, boil.val_as(&FAHRENHEIT).round_to(4));
}
