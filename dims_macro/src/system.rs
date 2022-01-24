#[macro_export]
macro_rules! measure_system {
    (name: $name:ident, debug_unit:$debug:ident) => {
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct $name;
        impl<'t> dims_core::unit_creation::MeasureSystem<'t> for $name {
            #[cfg(feature = "str")]
            const DEBUG_UNIT: dims_core::unit_creation::UnitFormat<'t, Self> = $debug;
        }
    };
}
