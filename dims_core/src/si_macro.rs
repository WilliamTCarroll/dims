#[macro_export]
/// Macro to generate an SI unit set.  The following options are valid:
///
/// To set up a system with no prefix (METRE):\
/// `si_unit!{"BASE_UNIT_NAME", MeasureSystem}`
///
/// To set up a system with a prefix (SQMETRE, SQCENTIMETRE):\
/// `si_unit!{"PREFIX", "BASE_UNIT_NAME", MeasureSystem, Adjustment}`\
/// `Adjustment` here means what to adjust each entry by (EX: 1000.0 for area)
/// This is multiplied to the values greater than the base,
/// and divided by values less than the base (no effect on base)
///
/// To set up an INDIVIDUAL unit: \
/// (probably easier to type a literal; this is primarily used by the other macro arms):\
/// `si_unit!{"PREFIX", "UNIT_PRE", "BASE_UNIT_NAME", MeasureSystem, ratio}`
///
/// The meanings are as follows:
/// - prefix: String literal of the prefix (EX: "SQ" or "SQ_")
/// - unit_pre: The unit prefix (EX: "MILLI", "YOCTO")
/// - base: The base unit to be generated (EX: "METRE")
/// - system: The Measurement system (EX: Length)
/// - ratio: The Floating Point ratio to be used (EX: 1.0e-3 for MILLIMETRE)
macro_rules! si_unit {
    // Set up a new si unit system with no prefix
    ($base:literal, $system:ty) => {
        si_unit!("", $base, $system, 1.0);
    };
    // Set up a new si unit system with the given prefix
    ($prefix:literal, $base:literal, $system:ty, $adj: literal) => {
        si_unit!($prefix, "YOTTA", $base, $system, 1.0e+24 * $adj);
        si_unit!($prefix, "ZETTA", $base, $system, 1.0e+21 * $adj);
        si_unit!($prefix, "EXA", $base, $system, 1.0e+18 * $adj);
        si_unit!($prefix, "PETA", $base, $system, 1.0e+15 * $adj);
        si_unit!($prefix, "TERA", $base, $system, 1.0e+12 * $adj);
        si_unit!($prefix, "GIGA", $base, $system, 1.0e+9 * $adj);
        si_unit!($prefix, "MEGA", $base, $system, 1.0e+6 * $adj);
        si_unit!($prefix, "KILO", $base, $system, 1.0e+3 * $adj);
        si_unit!($prefix, "HECTO", $base, $system, 1.0e+2 * $adj);
        si_unit!($prefix, "DECA", $base, $system, 1.0e+1 * $adj);
        si_unit!($prefix, "", $base, $system, 1.0e+0);
        si_unit!($prefix, "DECI", $base, $system, 1.0e-1 / $adj);
        si_unit!($prefix, "CENTI", $base, $system, 1.0e-2 / $adj);
        si_unit!($prefix, "MILLI", $base, $system, 1.0e-3 / $adj);
        si_unit!($prefix, "MICRO", $base, $system, 1.0e-6 / $adj);
        si_unit!($prefix, "NANO", $base, $system, 1.0e-9 / $adj);
        si_unit!($prefix, "PICO", $base, $system, 1.0e-12 / $adj);
        si_unit!($prefix, "FEMTO", $base, $system, 1.0e-15 / $adj);
        si_unit!($prefix, "ATTO", $base, $system, 1.0e-18 / $adj);
        si_unit!($prefix, "ZEPTO", $base, $system, 1.0e-21 / $adj);
        si_unit!($prefix, "YOCTO", $base, $system, 1.0e-24 / $adj);
    };
    // Set up an individual SI unit with the given info
    ($prefix:literal, $unit_pre:literal, $base:literal, $system:ty, $ratio:expr) => {
        $crate::unit_creation::paste::item! {
            pub static [<$prefix $unit_pre $base>]: $crate::unit_creation::UnitSimple<$system> =
            $crate::unit_creation::UnitSimple::<$system> {
                system: $crate::unit_creation::PhantomData,
                ratio: $ratio,
                offset: 0.0,
            };
        }
    };
}
