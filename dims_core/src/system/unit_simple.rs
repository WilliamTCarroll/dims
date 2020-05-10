use super::{MeasureSystem as MS, UnitTrait};
use crate::Flt;
use std::marker::PhantomData;

#[repr(transparent)]
pub struct UnitSimple<S: MS> {
    pub system: PhantomData<S>,
    pub in_base: Flt,
}

impl<S: MS> UnitTrait<S> for UnitSimple<S> {
    fn in_base(&self) -> Flt {
        self.in_base
    }
}
