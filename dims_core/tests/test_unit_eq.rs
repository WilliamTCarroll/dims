use dims_core::unit_creation::*;

mod common;
use common::Length;

type Unit<'t> = UnitFormat<'t, Length>;
#[test]
fn test_unit_eq() {
    let same1 = Unit {
        system: PhantomData,
        offset: 0.0,
        ratio: 1.0,
        abbr: "m",
        singular: "metre",
        plural: "metres",
    };
    let same2 = Unit {
        system: PhantomData,
        offset: 0.0,
        ratio: 1.0,
        abbr: "m",
        singular: "meter",
        plural: "meters",
    };
    assert_eq!(same1, same2);
}
