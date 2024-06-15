use super::*;
use core::fmt;
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
pub struct UnitSimple<S: MS> {
    pub ratio: S::N,
    pub offset: S::N,
}

impl<S: MS> UnitTrait for UnitSimple<S> {
    type System = S;
    fn from(&self, val: S::N) -> Measure<S> {
        Measure::new(self, val)
    }
    fn to_base(&self, val: S::N) -> S::N {
        (val + self.offset.clone()) * self.ratio.clone()
    }
    fn to_self(&self, val: S::N) -> S::N {
        (val / self.ratio.clone()) - self.offset.clone()
    }
}

impl<S: MS> fmt::Debug for UnitSimple<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UnitSimple")
            .field("ratio", &self.ratio)
            .field("offset", &self.offset)
            .finish()
    }
}
