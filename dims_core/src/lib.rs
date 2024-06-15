mod system;
/// ## prelude
///
/// This module contains everything you need to utilize already created `MeasureSystem` and units.
///
/// This and the `unit_creation` module have overlapping contents, so you only need to import one
pub mod prelude {
    #[cfg(feature = "std")]
    #[cfg(feature = "str")]
    pub use super::system::UnitFormatTrait;
    pub use super::system::{Measure, UnitFormat, UnitSimple, UnitTrait};
}
/// ## unit_creation
///
/// This module contains everything needed to make a new `MeasureSystem`, and units for it.
///
/// ~~It also re-exports `PhantomData` which is needed for new Units.~~
///
/// IMPORTANT: PhantomData is no longer required.
///
/// When making a new `MeasureSystem`, it is advised to use `dims_derive`,
/// so you can quickly apply the required traits.
///
/// This and the `prelude` module have overlapping contents, so you only need to import one
pub mod unit_creation {
    #[cfg(feature = "std")]
    pub use super::system::UnitFormatTrait;
    pub use super::system::{
        DivideBy, Measure, MeasureSystem, MultiplyBy, UnitFormat, UnitSimple, UnitTrait,
    };
    pub use num_traits;
}
