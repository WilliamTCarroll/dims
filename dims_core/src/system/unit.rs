use super::{Measure, MeasureSystem};
use crate::Flt;
use std::marker::PhantomData;

type Conv = Box<dyn Fn(Flt) -> Flt>;

#[repr(transparent)]
pub struct Unit<S: MeasureSystem> {
    pub system: PhantomData<S>,
    pub conv_enum: UnitConvert, // TODO: What about Temperature?  Fn Pointer instead?
}
impl<S: MeasureSystem> Unit<S> {
    /// Generate a new Measure from this unit and value
    pub fn from(&self, val: Flt) -> Measure<S> {
        Measure::new(self, val)
    }
    pub fn to_base(&self, val: Flt) -> Flt {
        self.conv_enum.to_base(val)
    }
    pub fn to_self(&self, val: Flt) -> Flt {
        self.conv_enum.to_self(val) // TODO: NOPE, NOT RIGHT.  GOING TO BED
    }
}

pub enum UnitConvert {
    Direct(Flt),
    Function(Conv, Conv),
}
impl UnitConvert {
    fn to_base(&self, other: Flt) -> Flt {
        match self {
            Self::Direct(val) => other / val,
            Self::Function(to_base, _) => to_base(other),
        }
    }
    fn to_self(&self, other: Flt) -> Flt {
        match self {
            Self::Direct(val) => other * val,
            Self::Function(_, to_self) => to_self(other),
        }
    }
}
