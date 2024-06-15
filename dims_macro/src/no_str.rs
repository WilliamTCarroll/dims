#[cfg_attr(not(feature = "str"), macro_export)]
macro_rules! one_unit {
(
    $name:ident,
    $system:ty,
    $ratio:expr,
    $offset:expr$(,)?
) => {
        pub const $name: dims_core::unit_creation::UnitSimple<$system> =
            dims_core::unit_creation::UnitSimple::<$system> {
                ratio: $ratio,
                offset: $offset,
            };
    };
(
    $name:ident,
    $system:ty,
    $ratio:expr$(,)?
) => {
    one_unit!{name: $name,system: $system, ratio: $ratio, offset: 0.0}
    };
// Set up an individual SI unit with the given info

(
    $prefix:literal,
    $unit_pre:literal,
    $base:literal,
    $system:ty,
    $ratio:expr,
    $repeat:literal,
    $plural:literal,
    $abbr1:literal,
    $abbr2:literal) => {
    $crate::paste::item! {
        pub const [<$prefix:upper $unit_pre:upper $base:upper>]: dims_core::unit_creation::UnitSimple<$system> =
            dims_core::unit_creation::UnitSimple::<$system> {
                ratio: repeat_item!($ratio, $repeat),
                offset: 0.0,
            };
        }
    };
}
