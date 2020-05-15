#[macro_use]
extern crate dims_derive;
use dims_core::prelude::*;
use std::marker::PhantomData;

#[derive(MeasureSystem)]
struct Length;

const INCH: UnitSimple<Length> = UnitSimple::<Length> {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.0254,
};
const MM: UnitSimple<Length> = UnitSimple::<Length> {
    system: PhantomData,
    offset: 0.0,
    ratio: 1.0 / 1000.0,
};
#[test]
fn test_measure_impl() {
    //
    let inch = INCH.from(2.0);
    let mm = MM.from(50.8);
    assert_eq!(inch, mm);
}
