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
    pub system: PhantomData<S>,
    pub ratio: Flt,
    pub offset: Flt,
    #[cfg(feature = "std")]
    pub abbr: &'t str,
    #[cfg(feature = "std")]
    pub singular: &'t str,
    #[cfg(feature = "std")]
    pub plural: &'t str,
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
#[cfg(feature = "std")]
impl<'t, S: MS> UnitFormatTrait<S> for UnitSimple<'t, S> {
    fn as_string_abbr(&self, val: Measure<S>) -> String {
        format!("{} {}", val.val_as(self), self.abbr)
    }

    fn as_string_full(&self, val: Measure<S>) -> String {
        let val = val.val_as(self);
        let suffix = if val == 1.0 {
            self.singular
        } else {
            self.plural
        };

        format!("{} {}", val, suffix)
    }
}
