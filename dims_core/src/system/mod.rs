mod measure;
mod unit_simple;
use crate::unit_creation::{Flt, MeasureSystem as MS};

pub use measure::Measure;
pub use unit_simple::UnitSimple;
/// The trait used to define a Measurement System
///
/// This is currently entirely blank; it is just used as PhantomData
pub trait MeasureSystem {}


/// UnitTrait is used to create a unit for a MeasureSystem
///
/// The struct is then used to generate a unit like METRE or KELVIN
pub trait UnitTrait<S: MS> {
    /// Generate a new Measure from this unit and value
    fn from(&self, val: Flt) -> Measure<S>;
    /// Convert the given value as this unit into the base unit
    ///
    /// EX: KILOGRAM.to_base(12.0) = 12,000.0
    fn to_base(&self, val: Flt) -> Flt;
    /// Convert the value as the base unit into this unit
    ///
    /// EX: KILOGRAM.to_self(12,000.0) = 12.0
    fn to_self(&self, val: Flt) -> Flt;
}
