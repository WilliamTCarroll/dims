use super::{DivideBy, Flt, MeasureSystem as MS, MultiplyBy, UnitTrait as UT};
use core::fmt;

use core::marker::PhantomData;
use core::ops::{Add, Div, Mul, Sub};

/// Measure is a wrapped Measurement of a specific System.
///
/// The value is stored in the base unit of the given system. (EX: Metre for length)
///
/// This is `#[repr(transparent)]`, so when compiled,
/// the memory footprint is that of the underlying Float
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq)]
pub struct Measure<S: MS> {
    pub system: PhantomData<S>,
    pub val: Flt,
}

impl<S: MS> Measure<S> {
    /// Generate a new Measure from the given unit and val
    pub fn new<U: UT<S>>(unit: &U, val: Flt) -> Self {
        let val = unit.to_base(val);
        Self {
            system: PhantomData,
            val,
        }
    }
    /// Convert the stored value to the provided one
    pub fn val_as<U: UT<S>>(&self, unit: &U) -> Flt {
        unit.to_self(self.val)
    }
    /// Return the stored value in its base unit
    pub fn as_base(&self) -> Flt {
        self.val
    }
}

// Section: External Impls

impl<S: MS> fmt::Debug for Measure<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Measure")
            .field("as_base", &self.val)
            .finish()
    }
}

impl<S: MS> Add for Measure<S> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            system: PhantomData,
            val: self.val + other.val,
        }
    }
}

impl<S: MS> Add for &Measure<S> {
    type Output = Measure<S>;
    fn add(self, other: Self) -> Measure<S> {
        Measure::<S> {
            system: PhantomData,
            val: self.val + other.val,
        }
    }
}

impl<S: MS> Sub for Measure<S> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            system: PhantomData,
            val: self.val - other.val,
        }
    }
}

impl<S: MS> Sub for &Measure<S> {
    type Output = Measure<S>;
    fn sub(self, other: Self) -> Measure<S> {
        Measure::<S> {
            system: PhantomData,
            val: self.val - other.val,
        }
    }
}

// Section: Conditonal Impls
impl<OTH: MS, S: MS + MultiplyBy<OTH>> Mul<Measure<OTH>> for Measure<S> {
    type Output = Measure<S::Output>;
    fn mul(self, other: Measure<OTH>) -> Measure<S::Output> {
        Self::Output {
            system: PhantomData,
            val: self.val * other.val,
        }
    }
}
impl<OTH: MS, S: MS + DivideBy<OTH>> Div<Measure<OTH>> for Measure<S> {
    type Output = Measure<S::Output>;
    fn div(self, other: Measure<OTH>) -> Self::Output {
        Self::Output {
            system: PhantomData,
            val: self.val / other.val,
        }
    }
}
