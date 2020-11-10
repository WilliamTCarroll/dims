use super::*;
use core::fmt;
use core::marker::PhantomData;
/// A Simple Unit in a given MeasureSystem with the info stored for formatting
///
/// The equations for Unit are as follows:
///
/// `self.to_base = (val + self.offset) * self.ratio`
///
/// `self.to_self = (val / self.ratio) - self.offset`
///
/// This also contains:
/// - `abbr`: Abbreviation of the unit (ft)
/// - `singular`: What to write if there is exactly one of this measure (foot)
/// - `plural`: What to write if there is not exactly one (feet)
///
/// If greater flexibility is required, please see `UnitTrait`
pub struct UnitFormat<'t, S: MS> {
    pub system: PhantomData<&'t S>,
    pub ratio: Flt,
    pub offset: Flt,
    pub abbr: &'t str,
    pub singular: &'t str,
    pub plural: &'t str,
}

impl<'t, S: MS> UnitTrait<S> for UnitFormat<'t, S> {
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

#[cfg(not(feature = "no_std"))]
#[cfg(feature = "str")]
impl<'t, S: MS> UnitFormatTrait<S> for UnitFormat<'t, S> {
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

impl<'t, S: MS> fmt::Debug for UnitFormat<'t, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UnitFormat")
            .field("ratio", &self.ratio)
            .field("offset", &self.offset)
            .field("singular", &self.singular)
            .finish()
    }
}

// Only check the ratio and offset; the spelling is of not concern
impl<'t, S: MS> PartialEq for UnitFormat<'t, S> {
    fn eq(&self, other: &Self) -> bool {
        self.ratio == other.ratio && self.offset == other.offset
    }
}
