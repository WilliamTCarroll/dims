#[macro_use]
extern crate dims_macro;
use core::marker::PhantomData;
use dims_core::prelude::UnitFormat;
use dims_core::unit_creation::*;
use rand;
use std::hint::black_box;

/// Specify a function to round a numeric to the specified number of
pub trait RoundTo {
    /// Round the given value to the number of decimals specified
    fn round_to(&self, decimals: i32) -> Self;
}
impl RoundTo for f32 {
    fn round_to(&self, decimals: i32) -> Self {
        let decimals: f32 = (10.0 as f32).powi(decimals);
        (self * decimals).round() / decimals
    }
}

measure_system! {name: Length, debug_unit: INCH,data_type: f32}

impl MultiplyBy<Length> for Length {
    type Output = Area;
}
measure_system! {name: Area, debug_unit: SQINCH, data_type: f32}

impl DivideBy<Length> for Area {
    type Output = Length;
}
si_unit! {system: Length,base: "metre", plural: "metres", abbr: "m"}

// const INCH: UnitFormat<Length> = UnitFormat::<Length> {
//     system: PhantomData,
//     offset: 0.0,
//     ratio: 0.0254,
//     abbr: "in",
//     singular: "inch",
//     plural: "inches",
// };
one_unit! {
    name: INCH,
    system: Length,
    ratio: 0.0254,
    abbr: "in",
    singular: "inch",
    plural: "inches",
}

const SQINCH: UnitFormat<Area> = UnitFormat {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.09290304 / 144.0,
    abbr: "in²",
    singular: "square inch",
    plural: "square inches",
};
use KILOMETRE as KM;
use MILLIMETRE as MM;
si_unit! {system: Area, prefix: "sq",base: "metre", plural: "metres", abbr:"m²", repeat: 2, ratio: 1.0}
use SQKILOMETRE as SQKM;
use SQMILLIMETRE as SQMM;

measure_system! {name: Mass, debug_unit: GRAM, data_type: f32}
si_unit! {system: Mass,base: "gram", plural: "grams", abbr:"g"}

measure_system! {name: Massi32, debug_unit: GRAM_I32, data_type: i32}
one_unit! {
    name: GRAM_I32,
    system: Massi32,
    ratio: 1,
    offset: 0,
    abbr: "in",
    singular: "inch",
    plural: "inches",
}
one_unit! {
    name: KILOGRAM_I32,
    system: Massi32,
    ratio: 1000,
    offset: 0,
    abbr: "in",
    singular: "inch",
    plural: "inches",
}
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

    // Double check the i32
    assert_eq!(GRAM_I32.from(32_000), KILOGRAM_I32.from(32));

    // inch.val_as(&GRAM); // Should not work

    // Grab the pair of random numbers to add
    // These ensure:
    // - Same values are tested direct and wrapped
    // - Compiler cannot aggressively optimize static values
    // This benchmark also utilizes `black_box` added in 1.66 to discourage optimization
    let list = get_rand_list(1000000);
    let mut list_out1 = Vec::new();
    let time_direct = std::time::Instant::now();
    // loop and add
    for (num1, num2) in &list {
        let output = num1 + num2;
        list_out1.push(output);
        black_box(list_out1.as_ptr());
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
        black_box(list_out2.as_ptr());
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
fn get_rand_list(len: usize) -> Vec<(f32, f32)> {
    let mut out = Vec::with_capacity(len);

    for _ in 1..len {
        let num1 = rand::random::<f32>() * 100.0;
        let num2 = rand::random::<f32>() * 100.0;
        out.push((num1, num2));
    }
    out
}
