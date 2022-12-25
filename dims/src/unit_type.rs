//! unit_type contains type aliases for each unit. \
//! This lets you avoid re-typing the generic identifier for each of them.\
//! These are all marked `'static` (as all the units defined in this crate are),
//! but you can specify your own lifetimes as needed via the `Measure` struct.
//!
//! Example Use:
//! ```rust
//! use dims::prelude::*;
//! use dims::si::length::METRE;
//! fn your_fn(inp: Length) {
//!     // .. Some operations here
//! }
//! let len = METRE.from(12.0);
//! your_fn(len);
//! ```
use super::systems::*;
use dims_core::prelude::Measure;

pub type Length = Measure<LengthSystem>;

pub type Area = Measure<AreaSystem>;

pub type Volume = Measure<VolumeSystem>;

pub type Mass = Measure<MassSystem>;

pub type Temperature = Measure<TemperatureSystem>;

#[cfg(not(feature = "str"))]
pub type UnitType<'t, S> = dims_core::unit_creation::UnitSimple<'t, S>;
#[cfg(feature = "str")]
pub type UnitType<'t, S> = dims_core::unit_creation::UnitFormat<'t, S>;
