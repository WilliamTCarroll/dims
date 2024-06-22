use dims::dims_core::unit_creation::*;
use dims_macro::one_unit;
#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct Length;
impl MeasureSystem for Length {
    type N = f64;
}

/// Size (in bytes) for each floating point number
const SIZE: usize = 8;

const INCH: UnitSimple<Length> = UnitSimple::<Length> {
    offset: 0.0,
    ratio: 0.0254,
    // The following fields should fail
    // abbr: "in",
    // singular: "inch",
    // plural: "inches",
};

// The Abbreviations should be ignored
one_unit!(name: FOOT, system: Length, ratio: INCH.ratio * 12.0, abbr: "ft", singular: "foot");
// Try without them
one_unit!(name: YARD, system: Length, ratio: FOOT.ratio * 3.0 );

#[test]
fn test_size() {
    use core::mem::{size_of, size_of_val};
    let raw = 25.4;
    let wrap = INCH.from(raw);

    // These should all be the same:
    // The number of bytes equal to the stored value
    assert_eq!(SIZE, size_of::<Measure<Length>>());
    assert_eq!(SIZE, size_of_val(&wrap));
    assert_eq!(SIZE, size_of_val(&raw));
    // Two floats and no strings
    assert_eq!(SIZE + SIZE, size_of_val(&INCH));
    // These are just marker structs; no memory is allocated
    assert_eq!(0, size_of::<Length>());
}

#[test]
fn test_macro_units() {
    let inch = INCH.from(36.0);
    let foot = FOOT.from(3.0);
    let yard = YARD.from(1.0);
    let diff = (inch - foot).as_base().abs();
    // I'd say this diff is well within acceptable tolerance
    // Unless, maybe, you're measuring the paths of individual photons?
    assert!(diff < 0.000000000000001);
    let diff = (inch - yard).as_base().abs();
    assert!(diff < 0.000000000000001);
}
