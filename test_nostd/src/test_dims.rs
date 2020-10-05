extern crate dims;
use dims::prelude::*;
use dims::si::length::METRE;
use dims::us::length::INCH;

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
