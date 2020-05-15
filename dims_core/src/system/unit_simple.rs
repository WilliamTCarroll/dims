use super::*;
use std::marker::PhantomData;
/// A Unit in a given MeasureSystem
///
/// The equations for Unit are as follows:
///
/// `self.to_base = (val + self.offset) * self.ratio`
///
/// `self.to_self = (val / self.ratio) - self.offset`
pub struct UnitSimple<S: MS> {
    pub system: PhantomData<S>,
    pub ratio: Flt,
    pub offset: Flt,
}

impl<S: MS> UnitTrait<S> for UnitSimple<S> {
    /// Generate a new Measure from this unit and value
    fn from(&self, val: Flt) -> Measure<S> {
        Measure::new(self, val)
    }
    /// Convert the value as this unit into the base unit
    ///
    /// EX: KILOGRAM.to_base(12.0) = 12,000.0
    fn to_base(&self, val: Flt) -> Flt {
        (val + self.offset) * self.ratio
    }
    /// Convert the value as this the base unit into this unit
    ///
    /// EX: KILOGRAM.to_self(12,000.0) = 12.0
    fn to_self(&self, val: Flt) -> Flt {
        (val / self.ratio) - self.offset
    }
}
