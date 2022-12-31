#![no_std]
// In the event of f64, the numbers are not excessive
#![allow(clippy::excessive_precision)]

#[macro_use]
extern crate dims_macro;
// Re-Export this for potential use using custom units
pub extern crate dims_core;

#[cfg(feature = "si")]
pub mod si;
#[cfg(feature = "us")]
pub mod us;

mod float;
pub use float::Flt;

pub mod unit_type;

pub mod prelude {
    // Re-export the core items
    pub use dims_core::prelude::*;
    // Re-export unit_type mod
    pub use crate::unit_type::*;
    // Re-export the float
    pub use crate::Flt;
}

pub mod systems;
