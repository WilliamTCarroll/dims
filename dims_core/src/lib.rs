#![cfg(not(test))]
#![no_std]
pub mod float;
mod system;
/// ## prelude
///
/// This module contains everything you need to utilize already created `MeasureSystem` and units.
///
/// This and the `unit_creation` module have overlapping contents, so you only need to import one
pub mod prelude {
    pub use super::float::Flt;
    pub use super::system::{Measure, UnitSimple, UnitTrait};
    pub use core::marker::PhantomData;
}
/// ## unit_creation
///
/// This module contains everything needed to make a new `MeasureSystem`, and units for it.
///
/// It also re-exports `PhantomData` which is needed for new Units.
///
/// When making a new `MeasureSystem`, it is advised to use `dims_derive`,
/// so you can quickly apply the required traits.
///
/// This and the `prelude` module have overlapping contents, so you only need to import one
pub mod unit_creation {
    pub use super::float::Flt;
    pub use super::system::{DivideBy, Measure, MeasureSystem, MultiplyBy, UnitSimple, UnitTrait};
    pub use core::marker::PhantomData;
}
