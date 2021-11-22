mod common;
use common::*;

#[test]
fn test_mul() {
    let inp = INCH.from(2.0);
    let out = inp * 2.0;
    let exp = INCH.from(4.0);
    assert_eq!(out, exp);
    let inp = INCH.from(6.0);
    let out = inp * -1.0;
    let exp = INCH.from(-6.0);
    assert_eq!(out, exp);
}
#[test]
fn test_div() {
    let inp = INCH.from(2.0);
    let out = inp / 2.0;
    let exp = INCH.from(1.0);
    assert_eq!(out, exp);
}
