use crate::Flt;

use super::{MeasureSystem, Unit};
use std::fmt;

use std::marker::PhantomData;
use std::ops::{Add, Sub};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq)]
pub struct Measure<S: MeasureSystem> {
    system: PhantomData<S>,
    val: Flt,
}
impl<S: MeasureSystem> fmt::Debug for Measure<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Measure")
            .field("as_base", &self.val)
            .finish()
    }
}

impl<S: MeasureSystem> Measure<S> {
    /// Generate a new Measure from the given unit and val
    pub fn new(unit: &Unit<S>, val: Flt) -> Self {
        let val = unit.to_base(val);
        Self {
            system: PhantomData,
            val,
        }
    }
    /// Convert the stored unit to the provided one
    pub fn val_as(&self, unit: &Unit<S>) -> Flt {
        unit.to_self(self.val)
    }
}

impl<S: MeasureSystem> Add for Measure<S> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            system: PhantomData,
            val: self.val + other.val,
        }
    }
}

impl<S: MeasureSystem> Add for &Measure<S> {
    type Output = Measure<S>;
    fn add(self, other: Self) -> Measure<S> {
        Measure::<S> {
            system: PhantomData,
            val: self.val + other.val,
        }
    }
}

impl<S: MeasureSystem> Sub for Measure<S> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            system: PhantomData,
            val: self.val - other.val,
        }
    }
}

impl<S: MeasureSystem> Sub for &Measure<S> {
    type Output = Measure<S>;
    fn sub(self, other: Self) -> Measure<S> {
        Measure::<S> {
            system: PhantomData,
            val: self.val - other.val,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    struct Length {}
    impl MeasureSystem for Length {}
    #[cfg(feature = "f64")]
    const SIZE: usize = 8;
    #[cfg(not(feature = "f64"))]
    const SIZE: usize = 4;
    #[test]
    fn check_measure_size() {
        use std::mem::{size_of, size_of_val};
        let raw = 25.4;
        let wrap = Measure::<Length> {
            system: PhantomData,
            val: raw,
        };

        // These should all be the same:
        // The number of bytes equal to the stored value
        assert_eq!(SIZE, size_of::<Measure<Length>>());
        assert_eq!(SIZE, size_of_val(&wrap));
        assert_eq!(SIZE, size_of_val(&raw));
    }
}
