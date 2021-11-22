use super::*;
use core::fmt;
use core::marker::PhantomData;
/// A Simple Unit in a given MeasureSystem
///
/// The equations for Unit are as follows:
///
/// `self.to_base = (val + self.offset) * self.ratio`
///
/// `self.to_self = (val / self.ratio) - self.offset`
///
/// If greater flexibility is required, please see `UnitTrait
#[derive(PartialEq)]
pub struct UnitSimple<'t, S: MS<'t>> {
    pub system: PhantomData<&'t S>,
    pub ratio: Flt,
    pub offset: Flt,
}

impl<'t, S: MS<'t>> UnitTrait<'t, S> for UnitSimple<'t, S> {
    fn from(&self, val: Flt) -> Measure<'t, S> {
        Measure::new(self, val)
    }
    fn to_base(&self, val: Flt) -> Flt {
        (val + self.offset) * self.ratio
    }
    fn to_self(&self, val: Flt) -> Flt {
        (val / self.ratio) - self.offset
    }
}

impl<'t, S: MS<'t>> fmt::Debug for UnitSimple<'t, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UnitSimple")
            .field("ratio", &self.ratio)
            .field("offset", &self.offset)
            .finish()
    }
}
