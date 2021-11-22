#[cfg_attr(feature = "str", macro_export)]
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
            pub const [<$prefix:upper $unit_pre:upper $base:upper>]: dims_core::unit_creation::UnitFormat<$system> =
            dims_core::unit_creation::UnitFormat::<$system> {
                system: dims_core::unit_creation::PhantomData,
                ratio: repeat_item!($ratio, $repeat),
                offset: 0.0,
                abbr: concat!($abbr1, $abbr2),
                singular: concat!($prefix, $base),
                plural: concat!($prefix, $plural),
            };
        }
    };
}
