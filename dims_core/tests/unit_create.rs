use dims_core::prelude::UnitSimple;
use dims_core::unit_creation::*;
use rand;
use std::marker::PhantomData;
#[derive(PartialEq)]
struct Length;
impl MeasureSystem for Length {}

#[derive(PartialEq)]
struct Mass;
impl MeasureSystem for Mass {}

static INCH: UnitSimple<Length> = UnitSimple::<Length> {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.0254,
};
static MM: UnitSimple<Length> = UnitSimple::<Length> {
    system: PhantomData,
    offset: 0.0,
    ratio: 1.0 / 1000.0,
};

static GRAM: UnitSimple<Mass> = UnitSimple::<Mass> {
    system: PhantomData,
    offset: 0.0,
    ratio: 1.0,
};
static KILOGRAM: UnitSimple<Mass> = UnitSimple::<Mass> {
    system: PhantomData,
    offset: 0.0,
    ratio: 1.0 / 1000.0,
};

#[test]
fn test_create() {
    let inch = INCH.from(12.0);
    let gram = GRAM.from(25.0);
    let mm = inch.val_as(&MM);
    // Rounding fun to ensure 64 and 32 bit test okay
    assert_eq!((mm * 1000.0).round(), 304.8 * 1000.0);
    let kg = gram.val_as(&KILOGRAM);
    assert_eq!((kg * 100.0).round(), 25000.0 * 100.0);
    // Check addition
    let dbl = &inch + &inch;
    if inch == dbl {}
    assert_eq!(dbl, INCH.from(24.0));
    // And subtraction
    let no = &inch - &inch;
    assert_eq!(no, INCH.from(0.0));

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
    panic!()
    // A word on results:
    // Wrapped time on *Debug* is generally twice as slow as direct
    // Wrapped time on *Release* is fairly exact (sometimes slower, sometimes faster)
    // From what I can tell, there is little to no runtime loss when under release optimizations
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
