#[macro_use]
extern crate dims_core;
use dims_core::prelude::UnitSimple;
use dims_core::unit_creation::*;
use rand;
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
#[derive(PartialEq, Copy, Clone)]
pub struct Length;
impl MeasureSystem for Length {}
impl MultiplyBy<Length> for Length {
    type Output = Area;
}
#[derive(PartialEq)]
pub struct Area;
impl MeasureSystem for Area {}
impl DivideBy<Length> for Area {
    type Output = Length;
}
#[derive(PartialEq)]
pub struct Mass;
impl MeasureSystem for Mass {}

si_unit! {"METRE", Length}

static INCH: UnitSimple<Length> = UnitSimple::<Length> {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.0254,
};
use KILOMETRE as KM;
use MILLIMETRE as MM;

si_unit! {"SQ", "METRE", Area, 1000.0}
use SQKILOMETRE as SQKM;
use SQMILLIMETRE as SQMM;
si_unit! {"GRAM", Mass}
#[test]
fn test_create() {
    let inch = INCH.from(12.0);
    let gram = GRAM.from(25.0);
    let mm = inch.val_as(&MILLIMETRE);
    // Rounding fun to ensure 64 and 32 bit test okay
    assert_eq!(mm.round_to(4), 304.8);
    let kg = gram.val_as(&KILOGRAM);
    assert_eq!(kg.round_to(3), 0.025);
    // Check addition
    let dbl = &inch + &inch;
    if inch == dbl {}
    assert_eq!(dbl, INCH.from(24.0));
    // And subtraction
    let zero = &inch - &inch;
    assert_eq!(zero, INCH.from(0.0));
    // inch.val_as(&GRAM); // Should not work

    // Grab the pair of random numbers to add
    // These ensure:
    // - Same values are tested direct and wrapped
    // - Compiler cannot aggressively optimize static values
    let list = get_rand_list(1000000);
    let mut list_out1 = Vec::new();
    let time_direct = std::time::Instant::now();
    // loop and add
    for (num1, num2) in &list {
        let output = num1 + num2;
        list_out1.push(output);
    }
    println!("DIRECT: {:?}", time_direct.elapsed().as_nanos());

    // Reset and begin
    let mut list_out2 = Vec::new();

    let time_wrapped = std::time::Instant::now();
    for (num1, num2) in &list {
        let num1 = MM.from(*num1);
        let num2 = MM.from(*num2);
        let output = num1 + num2;
        list_out2.push(output);
    }
    println!("WRAPPED:{:?}", time_wrapped.elapsed().as_nanos());
    // Use the output lists so they aren't skipped by the compiler
    println!("{}", list_out1.len());
    println!("{}", list_out2.len());
    // panic to print output
    // panic!()
    // A word on results:
    // Wrapped time on *Debug* is generally twice as slow as direct
    // Wrapped time on *Release* is fairly exact (sometimes slower, sometimes faster)
    // From what I can tell, there is little to no runtime loss when under release optimizations
}

#[test]
fn test_mul_div() {
    // Try first with the base units
    let len1 = METRE.from(12.5);
    let len2 = METRE.from(10.0);
    let area = len1 * len2;
    assert_eq!(area.val_as(&SQMETRE).round_to(4), 125.0.round_to(4));
    let len1 = MM.from(12.5);
    let len2 = MM.from(10.0);
    let area = len1 * len2;
    println!("{:?}", area);
    // Then below the base
    assert_eq!(area.val_as(&SQMM).round_to(4), 125.0.round_to(4));
    let len3 = area / len2;
    assert_eq!(len3.val_as(&MM).round_to(4), 12.5.round_to(4));
    // And above
    let len1 = KM.from(12.5);
    let len2 = KM.from(10.0);
    let area = len1 * len2;
    assert_eq!(area.val_as(&SQKM).round_to(4), 125.0.round_to(4));
}
/// Get a list of the specified length
///
/// This should help counter aggressive optimization of the compiler
fn get_rand_list(len: usize) -> Vec<(Flt, Flt)> {
    let mut out = Vec::with_capacity(len);

    for _ in 1..len {
        let num1 = rand::random::<Flt>() * 100.0;
        let num2 = rand::random::<Flt>() * 100.0;
        out.push((num1, num2));
    }
    out
}
