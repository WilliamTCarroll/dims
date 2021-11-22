//! unit_type contains type aliases for each unit. \
//! This lets you avoid re-typing the generic identifier for each of them
use super::systems::*;
use dims_core::prelude::Measure;

pub type Length = Measure<'static, LengthSystem>;

pub type Area = Measure<'static, AreaSystem>;

pub type Volume = Measure<'static, VolumeSystem>;

pub type Mass = Measure<'static, MassSystem>;

pub type Temperature = Measure<'static, TemperatureSystem>;

#[cfg(not(feature = "str"))]
pub type UnitType<'t, S> = dims_core::unit_creation::UnitSimple<'t, S>;
#[cfg(feature = "str")]
pub type UnitType<'t, S> = dims_core::unit_creation::UnitFormat<'t, S>;
