#[cfg_attr(not(feature = "str"), macro_export)]
macro_rules! one_unit {
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
        pub static [<$prefix:upper $unit_pre:upper $base:upper>]: dims_core::unit_creation::UnitSimple<$system> =
            dims_core::unit_creation::UnitSimple::<$system> {
                system: dims_core::unit_creation::PhantomData,
                ratio: repeat_item!($ratio, $repeat),
                offset: 0.0,
            };
        }
    };
}
