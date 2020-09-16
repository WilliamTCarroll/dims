use dims_core::unit_creation::*;
static INCH: UnitSimple<Length> = UnitSimple::<Length> {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.0254,
    #[cfg(feature = "std")]
    abbr: "in",
    #[cfg(feature = "std")]
    singular: "inch",
    #[cfg(feature = "std")]
    plural: "inches",
};
#[derive(Copy, Clone)]
struct Length;
impl MeasureSystem for Length {}

#[derive(Copy, Clone)]
struct Area;
impl MeasureSystem for Area {}

#[derive(Copy, Clone)]
struct Volume;
impl MeasureSystem for Volume {}

#[cfg(feature = "f64")]
const SIZE: usize = 8;
#[cfg(not(feature = "f64"))]
const SIZE: usize = 4;
#[test]
fn check_measure_size() {
    use std::mem::{size_of, size_of_val};
    let raw = 25.4;
    let wrap = INCH.from(raw);

    // These should all be the same:
    // The number of bytes equal to the stored value
    assert_eq!(SIZE, size_of::<Measure<Length>>());
    assert_eq!(SIZE, size_of_val(&wrap));
    assert_eq!(SIZE, size_of_val(&raw));
}

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
