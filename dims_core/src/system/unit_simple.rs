use super::*;
use core::marker::PhantomData;
/// A Simple Unit in a given MeasureSystem
///
/// The equations for Unit are as follows:
///
/// `self.to_base = (val + self.offset) * self.ratio`
///
/// `self.to_self = (val / self.ratio) - self.offset`
///
/// If greater flexibility is required, please see `UnitTrait`
pub struct UnitSimple<'t, S: MS> {
    pub system: PhantomData<&'t S>,
    pub ratio: Flt,
    pub offset: Flt,
}

impl<'t, S: MS> UnitTrait<S> for UnitSimple<'t, S> {
    fn from(&self, val: Flt) -> Measure<S> {
        Measure::new(self, val)
    }
    fn to_base(&self, val: Flt) -> Flt {
        (val + self.offset) * self.ratio
    }
    fn to_self(&self, val: Flt) -> Flt {
        (val / self.ratio) - self.offset
    }
}
