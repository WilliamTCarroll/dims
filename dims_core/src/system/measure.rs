use super::*;
use core::fmt;
use {MeasureSystem as MS, UnitTrait as UT};

use core::marker::PhantomData;
use core::ops::{Add, Div, Mul, Sub};

/// Measure is a wrapped Measurement of a specific System.
///
/// The value is stored in the base unit of the given system. (EX: Metre for length)
///
/// This is `#[repr(transparent)]`, so when compiled,
/// the memory footprint is that of the underlying Float
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Default, Hash)]
pub struct Measure<S: MS> {
    pub system: PhantomData<S>,
    pub val: S::N,
}

impl<S: MS> Measure<S> {
    /// Generate a new Measure from the given unit and val
    pub fn new<U: UT<System = S>>(unit: &U, val: S::N) -> Self {
        let val = unit.to_base(val);
        Self {
            system: PhantomData,
            val,
        }
    }
    /// Convert the stored value to the provided one
    pub fn val_as<U: UT<System = S>>(self, unit: &U) -> S::N {
        unit.to_self(self.val)
    }
    /// Return the stored value in its base unit
    pub fn as_base(self) -> S::N {
        self.val
    }
}

// Section: External Impls

impl<S: MS + Clone> fmt::Debug for Measure<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "str")]
        {
            let val = { S::DEBUG_UNIT.as_string_abbr(self.clone()) };
            f.write_str(&val)
        }
        #[cfg(not(feature = "str"))]
        {
            f.debug_struct("Measure")
                .field("as_base", &self.val)
                .finish()
        }
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
            val: self.val.clone() + &other.val,
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
            val: self.val.clone() - &other.val,
        }
    }
}

// Section: Conditonal Impls
impl<N: NumTrait, OTH: MS<N = N>, S: MS<N = N> + MultiplyBy<OTH>> Mul<Measure<OTH>> for Measure<S> {
    type Output = Measure<S::Output>;
    fn mul(self, other: Measure<OTH>) -> Measure<S::Output> {
        Self::Output {
            system: PhantomData,
            val: self.val * other.val,
        }
    }
}
impl<N: NumTrait, OTH: MS<N = N>, S: MS<N = N> + DivideBy<OTH>> Div<Measure<OTH>> for Measure<S> {
    type Output = Measure<S::Output>;
    fn div(self, other: Measure<OTH>) -> Self::Output {
        Self::Output {
            system: PhantomData,
            val: self.val / other.val,
        }
    }
}

impl<N: NumTrait, S: MS<N = N>> Mul<N> for Measure<S> {
    type Output = Self;
    fn mul(self, other: N) -> Self {
        Self {
            system: PhantomData,
            val: self.val * other,
        }
    }
}

impl<N: NumTrait, S: MS<N = N>> Div<N> for Measure<S> {
    type Output = Self;
    fn div(self, other: N) -> Self {
        Self {
            system: PhantomData,
            val: self.val / other,
        }
    }
}
