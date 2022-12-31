mod common;
use common::*;

#[test]
fn test_mul() {
    let inp = INCH.from(2.0);
    let out = inp * 2.0;
    let exp = INCH.from(4.0);
    assert_eq!(out, exp);
    let inp = INCH.from(6.0);
    let out = inp * -1.0;
    let exp = INCH.from(-6.0);
    assert_eq!(out, exp);
}
#[test]
fn test_div() {
    let inp = INCH.from(2.0);
    let out = inp / 2.0;
    let exp = INCH.from(1.0);
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
#[track_caller]
fn eq<M: MeasureSystem<N = f32> + Clone + Copy>(one: Measure<M>, two: Measure<M>) {
    assert!(
        one.as_base() - two.as_base() < 0.000001,
        "\n{one:#?}\n{two:#?}"
    )
}
