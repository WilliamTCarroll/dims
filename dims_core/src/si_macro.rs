#[macro_export]
/// Macro to generate an SI unit set.  The following options are valid:
///
/// To set up a system with no prefix, repeats, or offset (METRE):\
/// `si_unit!{"BASE_UNIT_NAME", MeasureSystem}`
///
/// To set up a system with a prefix (SQMETRE, CUBICMETRE):\
/// `si_unit!{"PREFIX", "BASE_UNIT_NAME", MeasureSystem, repeat, offset}`\
/// - `repeat` here means how many times to repeat the literal (EX: 1.0e+18).
/// This is utilized for area (2) or volume(3).  (4) is the max; this will be expanded as required.
/// `LITRE` is the exception to volume; its repeat is 1, and uses the `offset`
/// - `offset` is the ratio to be multiplied by the offset from base.
/// In 90% of situations, this is 1.0.
/// For `LITRE`, this is 0.001 (as 1 `LITRE` != 1 `CUBICMETRE`)
///
/// To set up an INDIVIDUAL unit: \
/// (probably easier to type a literal; this is primarily used by the other macro arms):\
/// `si_unit!{"PREFIX", "UNIT_PRE", "BASE_UNIT_NAME", MeasureSystem, ratio, repeat}`
///
/// The meanings are as follows:
/// - prefix: String literal of the prefix (EX: "SQ" or "SQ_")
/// - unit_pre: The unit prefix (EX: "MILLI", "YOCTO")
/// - base: The base unit to be generated (EX: "METRE")
/// - system: The Measurement system (EX: Length)
/// - ratio: The Floating Point ratio to be used (EX: 1.0e-3 for MILLIMETRE)
/// - repeat: Number of times to repeat the ratio (EX: 3 for volume, excluding litre)
macro_rules! si_unit {
    // Set up a new si unit system with no prefix
    ($base:literal, $system:ty) => {
        si_unit!("", $base, $system, 1, 1.0);
    };
    // Set up a new si unit system with the given prefix
    ($prefix:literal, $base:literal, $system:ty, $repeat:literal, $off:literal) => {
        si_unit!($prefix, "YOTTA", $base, $system, 1.0e+24 * $off, $repeat);
        si_unit!($prefix, "ZETTA", $base, $system, 1.0e+21 * $off, $repeat);
        si_unit!($prefix, "EXA", $base, $system, 1.0e+18 * $off, $repeat);
        si_unit!($prefix, "PETA", $base, $system, 1.0e+15 * $off, $repeat);
        si_unit!($prefix, "TERA", $base, $system, 1.0e+12 * $off, $repeat);
        si_unit!($prefix, "GIGA", $base, $system, 1.0e+9 * $off, $repeat);
        si_unit!($prefix, "MEGA", $base, $system, 1.0e+6 * $off, $repeat);
        si_unit!($prefix, "KILO", $base, $system, 1.0e+3 * $off, $repeat);
        si_unit!($prefix, "HECTO", $base, $system, 1.0e+2 * $off, $repeat);
        si_unit!($prefix, "DECA", $base, $system, 1.0e+1 * $off, $repeat);
        si_unit!($prefix, "", $base, $system, 1.0 * $off, $repeat);
        si_unit!($prefix, "DECI", $base, $system, 1.0e-1 * $off, $repeat);
        si_unit!($prefix, "CENTI", $base, $system, 1.0e-2 * $off, $repeat);
        si_unit!($prefix, "MILLI", $base, $system, 1.0e-3 * $off, $repeat);
        si_unit!($prefix, "MICRO", $base, $system, 1.0e-6 * $off, $repeat);
        si_unit!($prefix, "NANO", $base, $system, 1.0e-9 * $off, $repeat);
        si_unit!($prefix, "PICO", $base, $system, 1.0e-12 * $off, $repeat);
        si_unit!($prefix, "FEMTO", $base, $system, 1.0e-15 * $off, $repeat);
        si_unit!($prefix, "ATTO", $base, $system, 1.0e-18 * $off, $repeat);
        si_unit!($prefix, "ZEPTO", $base, $system, 1.0e-21 * $off, $repeat);
        si_unit!($prefix, "YOCTO", $base, $system, 1.0e-24 * $off, $repeat);
    };
    // Set up an individual SI unit with the given info
    ($prefix:literal, $unit_pre:literal, $base:literal, $system:ty, $ratio:expr, $repeat:literal) => {
        $crate::unit_creation::paste::item! {
            pub static [<$prefix $unit_pre $base>]: $crate::unit_creation::UnitSimple<$system> =
            $crate::unit_creation::UnitSimple::<$system> {
                system: $crate::unit_creation::PhantomData,
                ratio: repeat_item!($ratio, $repeat),
                offset: 0.0,
            };
        }
    };
}
#[macro_export]
/// Repeat the item presented x number of times. \
/// This is for internal use for generating si units. \
/// Each set must explicitly be noted, so this only goes up to 4 (will be expanded if required/requested).
macro_rules! repeat_item {
    ($item: expr, 4) => {
        $item * repeat_item!($item, 3)
    };
    ($item: expr, 3) => {
        $item * repeat_item!($item, 2)
    };
    ($item: expr, 2) => {
        $item * $item
    };
    ($item: expr, 1) => {
        $item
    };
}
