extern crate dims;
use dims::prelude::*;
use dims::si::area::SQUAREMETRE;
use dims::si::length::METRE;
use dims::us::area::SQUARE_INCH as SQIN;
use dims::us::length::INCH;
use dims::us::volume::CUBIC_INCH as CBIN;

/// Size (in bytes) for each floating point number.
const SIZE: usize = 8;
#[test]
fn test_size() {
    use core::mem::{size_of, size_of_val};
    let raw = 256.0;
    let wrap = METRE.from(raw);

    // These should all be the same:
    // The number of bytes equal to the stored value
    assert_eq!(SIZE, size_of::<Length>());
    assert_eq!(SIZE, size_of_val(&wrap));
    assert_eq!(SIZE, size_of_val(&raw));
    // Two floats and no strings
    assert_eq!(SIZE + SIZE, size_of_val(&INCH));
    assert_eq!(SIZE + SIZE, size_of_val(&METRE));
}

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
// Due to the usage of external macros for this, ensure this works in no_std
#[test]
fn test_system_mul_div() {
    let len = INCH.from(2.0);
    let area = len * len;
    assert_eq!(area, SQUAREMETRE.from(0.00258064)); // Floating Point FUN!
    let vol = area * len;
    assert_eq!(vol, CBIN.from(8.0));
    let area = vol / len;
    let len = area / len;
    assert_eq!(len, INCH.from(2.0));
}
