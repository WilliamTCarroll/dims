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
    (
        system: $system:ty,
        base: $base:literal,
        plural: $plural:literal,
        abbr: $abbr:literal$(,)?
    ) => {
        si_unit!(system: $system, prefix: "", base: $base, plural: $plural, abbr: $abbr, repeat: 1, ratio: 1.0);
    };
    // Set up a new si unit system with the given prefix
    (
        system: $system:ty,
        prefix: $prefix:literal,
        base: $base:literal,
        plural: $plural:literal,
        abbr: $abbr:literal,
        repeat: $repeat:literal,
        ratio: $ratio:literal$(,)?
    ) => {
        one_unit!($prefix, "yotta", $base, $system, 1.0e+24 * $ratio, $repeat,$plural, "Y", $abbr);
        one_unit!($prefix, "zetta", $base, $system, 1.0e+21 * $ratio, $repeat, $plural, "Z", $abbr);
        one_unit!($prefix, "exa", $base, $system, 1.0e+18 * $ratio, $repeat, $plural, "E", $abbr);
        one_unit!($prefix, "peta", $base, $system, 1.0e+15 * $ratio, $repeat, $plural, "P", $abbr);
        one_unit!($prefix, "tera", $base, $system, 1.0e+12 * $ratio, $repeat, $plural, "T", $abbr);
        one_unit!($prefix, "giga", $base, $system, 1.0e+9 * $ratio, $repeat, $plural, "G", $abbr);
        one_unit!($prefix, "mega", $base, $system, 1.0e+6 * $ratio, $repeat, $plural, "M", $abbr);
        one_unit!($prefix, "kilo", $base, $system, 1.0e+3 * $ratio, $repeat, $plural, "k", $abbr);
        one_unit!($prefix, "hecto", $base, $system, 1.0e+2 * $ratio, $repeat, $plural, "h", $abbr);
        one_unit!($prefix, "deca", $base, $system, 1.0e+1 * $ratio, $repeat, $plural, "da", $abbr);
        one_unit!($prefix, "", $base, $system, 1.0 * $ratio, $repeat, $plural, "", $abbr);
        one_unit!($prefix, "deci", $base, $system, 1.0e-1 * $ratio, $repeat, $plural, "d", $abbr);
        one_unit!($prefix, "centi", $base, $system, 1.0e-2 * $ratio, $repeat, $plural, "c", $abbr);
        one_unit!($prefix, "milli", $base, $system, 1.0e-3 * $ratio, $repeat, $plural, "m", $abbr);
        one_unit!($prefix, "micro", $base, $system, 1.0e-6 * $ratio, $repeat, $plural, "μ", $abbr);
        one_unit!($prefix, "nano", $base, $system, 1.0e-9 * $ratio, $repeat, $plural, "n", $abbr);
        one_unit!($prefix, "pico", $base, $system, 1.0e-12 * $ratio, $repeat, $plural, "p", $abbr);
        one_unit!($prefix, "femto", $base, $system, 1.0e-15 * $ratio, $repeat, $plural, "f", $abbr);
        one_unit!($prefix, "atto", $base, $system, 1.0e-18 * $ratio, $repeat, $plural, "a", $abbr);
        one_unit!($prefix, "zepto", $base, $system, 1.0e-21 * $ratio, $repeat, $plural, "z", $abbr);
        one_unit!($prefix, "yocto", $base, $system, 1.0e-24 * $ratio, $repeat, $plural, "y", $abbr);
    };
}
