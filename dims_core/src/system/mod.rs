mod measure;
mod unit_simple;
use crate::{Flt, MeasureSystem as MS};

pub use measure::Measure;
pub use unit_simple::UnitSimple;
/// The trait used to define a Measurement System
///
/// This is currently entirely blank; it is just used as PhantomData
pub trait MeasureSystem {}

pub trait UnitTrait<S: MS> {
    fn from(&self, val: Flt) -> Measure<S>;
    fn to_base(&self, val: Flt) -> Flt;
    fn to_self(&self, val: Flt) -> Flt;
}
