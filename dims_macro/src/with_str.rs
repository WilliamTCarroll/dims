#[cfg_attr(feature = "str", macro_export)]
macro_rules! one_unit {
    (
        name: $name:ident,
        system: $system:ty,
        ratio: $ratio:expr,
        offset: $offset:expr,
        abbr: $abbr: literal,
        singular: $singular: literal,
        plural: $plural: literal$(,)?
    ) => {
        pub const $name: $crate::dims_core::unit_creation::UnitFormat<$system> =
        $crate::dims_core::unit_creation::UnitFormat::<$system> {
                ratio: $ratio,
                offset: $offset,
                abbr: $abbr,
                singular: $singular,
                plural: $singular,
            };
    };
    (
        name: $name:ident,
        system: $system:ty,
        ratio: $ratio:expr,
        abbr: $abbr: literal,
        singular: $singular: literal$(,)?
    ) => {
        one_unit!(
            name: $name,
            system: $system,
            ratio: $ratio,
            offset: 0.0,
            abbr: $abbr,
            singular: $singular,
            plural: concat!($singular,"s")
        )
    };
    (
        name: $name:ident,
        system: $system:ty,
        ratio: $ratio:expr,
        abbr: $abbr: literal,
        singular: $singular: literal$(,)?
    ) => {
        pub const $name: $crate::dims_core::unit_creation::UnitFormat<$system> =
            $crate::dims_core::unit_creation::UnitFormat::<$system> {
                ratio: $ratio,
                offset: 0.0,
                abbr: $abbr,
                singular: $singular,
                plural: concat!($singular,"s"),
            };
    };
    (
        name: $name:ident,
        system: $system:ty,
        ratio: $ratio:expr,
        abbr: $abbr: literal,
        singular: $singular: literal,
        plural: $plural: literal$(,)?
    ) => {
        one_unit! {
        name: $name,
        system: $system,
        ratio: $ratio,
        offset: 0.0,
        abbr: $abbr,
        singular: $singular,
        plural: $plural
    }
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
        $abbr2:literal$(,)?) => {
        $crate::paste::item! {
            pub const [<$prefix:upper $unit_pre:upper $base:upper>]: $crate::dims_core::unit_creation::UnitFormat<$system> =
            $crate::dims_core::unit_creation::UnitFormat::<$system> {
                ratio: repeat_item!($ratio, $repeat),
                offset: 0.0,
                abbr: concat!($abbr1, $abbr2),
                singular: concat!($prefix, $base),
                plural: concat!($prefix, $plural),
            };
        }
    };
}
