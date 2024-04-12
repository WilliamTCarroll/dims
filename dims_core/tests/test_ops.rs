mod common;
use common::*;

#[test]
fn test_mul() {
    let inp = INCH.from(2.0);
    let out = inp * 2.0;
    let exp = INCH.from(4.0);
    assert_eq!(out, exp);
    // Try both the multiply = and a negative
    let mut out = INCH.from(6.0);
    out *= -1.0;
    let exp = INCH.from(-6.0);
    assert_eq!(out, exp);
}
#[test]
fn test_div() {
    let inp = INCH.from(2.0);
    let out = inp / 2.0;
    let exp = INCH.from(1.0);
    assert_eq!(out, exp);
    let mut out = inp;
    out /= 2.0;
    assert_eq!(out, exp);
}
#[test]
fn test_system_mul_div() {
    let len = INCH.from(2.0);
    let area = len * len;
    eq(area, SQIN.from(4.0));
    let vol = area * &len;
    eq(vol, CBIN.from(7.999999)); // Floating Point FUN!
    let area = &vol / len;
    let len = area / len;
    eq(len, INCH.from(2.0));
    let mut len = INCH.from(16.0);
    eq(INCH.from(-16.0), -len);
    len *= -1.0;
    eq(INCH.from(-16.0), len);
    len /= -2.0;
    eq(INCH.from(8.0), len);
}

#[test]
fn test_add_sub() {
    let len1 = INCH.from(3.0);
    let len2 = INCH.from(5.0);
    eq(INCH.from(8.0), &len1 + len2);
    let len3 = INCH.from(24.0) - &len2;
    eq(INCH.from(19.0), len3);
    let mut len4 = len3;
    len4 -= len1;
    eq(INCH.from(16.0), len4);
    eq(INCH.from(-16.0), -len4);
}
#[test]
fn test_ord() {
    assert!(INCH.from(2.0) < INCH.from(9.0));
    assert!(INCH.from(2.0) > -INCH.from(9.0));
    // Just for the sake of my own sanity, check ordering in a vec
    let mut inp = vec![INCH.from(2.0), INCH.from(1.0), INCH.from(3.0)];
    let exp = vec![INCH.from(1.0), INCH.from(2.0), INCH.from(3.0)];
    inp.sort_by(|one, two| one.partial_cmp(two).unwrap_or(core::cmp::Ordering::Equal));
    assert_eq!(inp, exp);
}
#[track_caller]
fn eq<M: MeasureSystem<N = f32> + Clone + Copy>(one: Measure<M>, two: Measure<M>) {
    assert!(
        one.as_base() - two.as_base() < 0.000001,
        "\n{one:#?}\n{two:#?}"
    )
}
