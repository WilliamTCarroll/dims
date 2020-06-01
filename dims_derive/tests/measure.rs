#[macro_use]
extern crate dims_derive;
use dims_core::prelude::*;
use std::marker::PhantomData;

/// Specify a function to round a numeric to the specified number of
pub trait RoundTo {
    /// Round the given value to the number of decimals specified
    fn round_to(&self, decimals: i32) -> Self;
}

impl RoundTo for Flt {
    fn round_to(&self, decimals: i32) -> Self {
        let decimals: Flt = (10.0 as Flt).powi(decimals);
        (self * decimals).round() / decimals
    }
}

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
    ratio: 1.0e-3,
};

#[test]
fn test_measure_impl() {
    //
    let inch = INCH.from(2.0);
    let mm = MM.from(50.8);
    assert_eq!(inch.as_base().round_to(6), mm.as_base().round_to(6));
}
