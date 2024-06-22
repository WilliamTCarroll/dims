#[cfg(not(feature = "str"))]
#[macro_export]
macro_rules! measure_system {
    (name: $name:ident, data_type: $data_type:ty $(,)?) => {
        $crate::measure_system! {name: $name, debug_unit: null, data_type: $data_type}
    };
    (name: $name:ident, debug_unit: $debug:ident, data_type: $data_type:ty $(,)?) => {
        #[derive(PartialEq, PartialOrd, Eq, Clone, Copy)]
        pub struct $name;
        impl $crate::dims_core::unit_creation::MeasureSystem for $name {
            type N = $data_type;
        }
    };
}
#[cfg(feature = "str")]
#[macro_export]
macro_rules! measure_system {
    (name: $name:ident, debug_unit: $debug:ident, data_type: $data_type:ty $(,)?) => {
        #[derive(PartialEq, PartialOrd, Eq, Clone, Copy)]
        pub struct $name;
        impl $crate::dims_core::unit_creation::MeasureSystem for $name {
            type N = $data_type;
            #[cfg(feature = "str")]
            const DEBUG_UNIT: $crate::dims_core::unit_creation::UnitFormat<'static, Self> = $debug;
        }
    };
}
