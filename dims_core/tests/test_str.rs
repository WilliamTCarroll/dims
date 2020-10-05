use dims_core::unit_creation::*;
#[derive(Copy, Clone)]
struct Length;
impl MeasureSystem for Length {}

static INCH: UnitFormat<Length> = UnitFormat::<Length> {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.0254,
    abbr: "in",
    singular: "inch",
    plural: "inches",
};

#[test]
fn test_str() {
    let val = INCH.from(32.0);
    assert_eq!(&INCH.as_string_abbr(val), "32 in");
    assert_eq!(&INCH.as_string_full(val), "32 inches");
    let val = INCH.from(1.0);
    assert_eq!(&INCH.as_string_abbr(val), "1 in");
    assert_eq!(&INCH.as_string_full(val), "1 inch");
}
