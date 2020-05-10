mod measure;
mod unit;

pub use measure::Measure;
pub use unit::{Unit, UnitConvert};
/// The trait used to define a Measurement System
///
/// This is currently entirely blank; it is just used as PhantomData
pub trait MeasureSystem {}
