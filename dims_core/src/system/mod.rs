mod measure;
mod unit_format;
mod unit_simple;
use crate::unit_creation::{Flt, MeasureSystem as MS};

pub use measure::Measure;
pub use unit_format::UnitFormat;
pub use unit_simple::UnitSimple;
/// The trait used to define a Measurement System
///
/// This is currently entirely blank; it is just used as PhantomData
pub trait MeasureSystem {}

/// Allows this MeasureSystem to transform into another via multiplication
///
/// - Output specifies what system will be the result (EX: Area as output for Length * Length)
/// - Other specifies what will be multiplied by this value to get the Output( EX: Length for Length)
pub trait MultiplyBy<OTH: MeasureSystem> {
    type Output: MeasureSystem;
}
/// Allows this MeasureSystem to transform into another via division
///
/// - Output specifies what system will be the result (EX: Length as output for Area / Length)
/// - Other specifies what will be multiplied by this value to get the Output( EX: Length for Area)
pub trait DivideBy<OTH: MeasureSystem> {
    type Output: MeasureSystem;
}

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

#[cfg(not(feature = "no_std"))]
#[cfg(feature = "str")]
pub trait UnitFormatTrait<S: MS> {
    /// Format the given unit as a string with the specified unit suffix abbreviation after it
    fn as_string_abbr(&self, val: Measure<S>) -> String;
    /// Format the given unit as a string with the full unit suffix
    fn as_string_full(&self, val: Measure<S>) -> String;
}
