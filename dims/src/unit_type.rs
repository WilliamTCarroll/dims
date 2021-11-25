//! unit_type contains type aliases for each unit. \
//! This lets you avoid re-typing the generic identifier for each of them
use super::systems::*;
use dims_core::prelude::Measure;

pub type Length<'t> = Measure<'t, LengthSystem>;

pub type Area<'t> = Measure<'t, AreaSystem>;

pub type Volume<'t> = Measure<'t, VolumeSystem>;

pub type Mass<'t> = Measure<'t, MassSystem>;

pub type Temperature<'t> = Measure<'t, TemperatureSystem>;

#[cfg(not(feature = "str"))]
pub type UnitType<'t, S> = dims_core::unit_creation::UnitSimple<'t, S>;
#[cfg(feature = "str")]
pub type UnitType<'t, S> = dims_core::unit_creation::UnitFormat<'t, S>;
