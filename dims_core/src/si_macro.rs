#[macro_export]
macro_rules! si_unit {
    ($base:ident, $system:ty) => {
        si_unit!(YOTTA, $base, $system, 1.0e+24);
        si_unit!(ZETTA, $base, $system, 1.0e+21);
        si_unit!(EXA, $base, $system, 1.0e+18);
        si_unit!(PETA, $base, $system, 1.0e+15);
        si_unit!(TERA, $base, $system, 1.0e+12);
        si_unit!(GIGA, $base, $system, 1.0e+9);
        si_unit!(MEGA, $base, $system, 1.0e+6);
        si_unit!(KILO, $base, $system, 1.0e+3);
        si_unit!(HECTO, $base, $system, 1.0e+2);
        si_unit!(DECA, $base, $system, 1.0e+1);
        si_unit!($base, $system, 1.0e+0);
        si_unit!(DECI, $base, $system, 1.0e-1);
        si_unit!(CENTI, $base, $system, 1.0e-2);
        si_unit!(MILLI, $base, $system, 1.0e-3);
        si_unit!(MICRO, $base, $system, 1.0e-6);
        si_unit!(NANO, $base, $system, 1.0e-9);
        si_unit!(PICO, $base, $system, 1.0e-12);
        si_unit!(FEMTO, $base, $system, 1.0e-15);
        si_unit!(ATTO, $base, $system, 1.0e-18);
        si_unit!(ZEPTO, $base, $system, 1.0e-21);
        si_unit!(YOCTO, $base, $system, 1.0e-24);
    };

    ($prefix:ident, $base:ident, $system:ty, $ratio:literal) => {
        $crate::unit_creation::paste::item! {pub static [<$prefix $base>]: $crate::unit_creation::UnitSimple<$system> =
                $crate::unit_creation::UnitSimple::<$system> {
                    system: $crate::unit_creation::PhantomData,
                    ratio: $ratio,
                    offset: 0.0,
                };
        }
    };
    ($base:ident, $system:ty, $ratio:literal) => {
        pub static $base: $crate::unit_creation::UnitSimple<$system> =
            $crate::unit_creation::UnitSimple::<$system> {
                system: $crate::unit_creation::PhantomData,
                ratio: $ratio,
                offset: 0.0,
            };
    };
}
// si_unit! {METRE, Length}
