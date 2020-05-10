mod measure;
mod unit_simple;
use crate::Flt;

use crate::MeasureSystem as MS;
pub use measure::Measure;
pub use unit_simple::UnitSimple;
/// The trait used to define a Measurement System
///
/// This is currently entirely blank; it is just used as PhantomData
pub trait MeasureSystem {}

pub trait UnitTrait<S: MS>
where
    Self: Sized,
{
    /// Generate a new Measure from this unit and value
    fn from(&self, val: Flt) -> Measure<S> {
        Measure::new(self, val)
    }
    fn in_base(&self) -> Flt;
    fn to_base(&self, val: Flt) -> Flt {
        val / self.in_base()
    }
    fn to_self(&self, val: Flt) -> Flt {
        self.in_base() * val
    }
}
