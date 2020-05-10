#[macro_use]
extern crate dims_derive;
use dims_core::*;
use std::marker::PhantomData;

#[derive(MeasureSystem)]
struct Length;

const INCH: UnitSimple<Length> = UnitSimple::<Length> {
    system: PhantomData,
    in_base: 1.0 / 0.0254,
};
const MM: UnitSimple<Length> = UnitSimple::<Length> {
    system: PhantomData,
    in_base: 1000.0,
};
#[test]
fn test_measure_impl() {
    //
    let inch = INCH.from(2.0);
    let mm = MM.from(50.8);
    assert_eq!(inch, mm);
}
