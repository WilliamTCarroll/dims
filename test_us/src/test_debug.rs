extern crate dims;
use dims::prelude::*;
use dims::si::length::MILLIMETRE;
use dims::us::length::INCH;

#[test]
fn test_debug_us() {
    let inp = MILLIMETRE.from(25.4);
    assert_eq!(&format!("{:?}", inp), "1.0000001 in");
    let inp = INCH.from(6000.123456789);
    assert_eq!(&format!("{:?}", inp), "6000.1235 in");
    let inp = len_times_two(inp);
    assert_eq!(&format!("{:?}", inp), "12000.247 in");
}
/// Quick fn to ensure
fn len_times_two(inp: Length) -> Length {
    inp * 2.0
}
