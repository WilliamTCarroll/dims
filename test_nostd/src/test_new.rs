extern crate dims_core;

use dims_core::unit_creation::*;
#[derive(Copy, Clone)]
struct Length;
impl<'t> MeasureSystem<'t> for Length {}

/// Size (in bytes) for each floating point number
const SIZE: usize = 8;

static INCH: UnitSimple<Length> = UnitSimple::<Length> {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.0254,
    // The following fields should fail
    // abbr: "in",
    // singular: "inch",
    // plural: "inches",
};

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
