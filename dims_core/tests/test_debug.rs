mod common;
use common::*;

#[test]
fn test_debug() {
    // Floating Point Fun
    let val = INCH.from(30.0);
    let res = format!("{:?}", val);
    assert_eq!(&res, "29.999998 in");
    // Ensure different units still resolve correctly
    let val = SQFT.from(2.0);
    let res = format!("{:?}", val);
    assert_eq!(&res, "288 in²");
}
