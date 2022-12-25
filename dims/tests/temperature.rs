use dims::prelude::*;
use dims::si::temperature::*;
use dims::us::temperature::{FAHRENHEIT, RANKINE};
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
    run(zero, &KELVIN, 0.0);
    run(zero, &CELCIUS, -273.15);
    run(zero, &RANKINE, 0.0);
    run(zero, &FAHRENHEIT, -459.67);
    let freeze = CELCIUS.from(0.0);
    run(freeze, &KELVIN, 273.15);
    run(freeze, &CELCIUS, 0.0);
    run(freeze, &RANKINE, 491.6700);
    run(freeze, &FAHRENHEIT, 31.9999);
    let boil = CELCIUS.from(99.9839);
    run(boil, &KELVIN, 373.1339);
    run(boil, &CELCIUS, 99.98390);
    run(boil, &RANKINE, 671.64102);
    run(boil, &FAHRENHEIT, 211.9710);
}
/// Run the test.\
/// Splitting serves two purposes:
/// 1. Keep the output test consistent
/// 2. Ensure the expected function signature isn't crazy (EX: Generic causing Fun)
#[track_caller]
fn run(inp: Temperature, val_as: &TemperatureUnit, exp: Flt) {
    let out = inp.val_as(val_as);
    assert_eq!(out.round_to(4), exp, "{out:.10}")
}
