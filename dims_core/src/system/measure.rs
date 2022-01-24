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
#[derive(Copy, Clone, PartialEq)]
pub struct Measure<'t, S: MS<'t>> {
    pub system: PhantomData<&'t S>,
    pub val: Flt,
}

impl<'t, S: MS<'t>> Measure<'t, S> {
    /// Generate a new Measure from the given unit and val
    pub fn new<U: UT<'t, S>>(unit: &U, val: Flt) -> Self {
        let val = unit.to_base(val);
        Self {
            system: PhantomData,
            val,
        }
    }
    /// Convert the stored value to the provided one
    pub fn val_as<U: UT<'t, S>>(self, unit: &U) -> Flt {
        unit.to_self(self.val)
    }
    /// Return the stored value in its base unit
    pub fn as_base(self) -> Flt {
        self.val
    }
}

// Section: External Impls

impl<'t, S: MS<'t> + Clone + Copy> fmt::Debug for Measure<'t, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "str")]
        {
            let val = { S::DEBUG_UNIT.as_string_abbr(*self) };
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

impl<'t, S: MS<'t>> Add for Measure<'t, S> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            system: PhantomData,
            val: self.val + other.val,
        }
    }
}

impl<'t, S: MS<'t>> Add for &Measure<'t, S> {
    type Output = Measure<'t, S>;
    fn add(self, other: Self) -> Measure<'t, S> {
        Measure::<S> {
            system: PhantomData,
            val: self.val + other.val,
        }
    }
}

impl<'t, S: MS<'t>> Sub for Measure<'t, S> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            system: PhantomData,
            val: self.val - other.val,
        }
    }
}

impl<'t, S: MS<'t>> Sub for &Measure<'t, S> {
    type Output = Measure<'t, S>;
    fn sub(self, other: Self) -> Measure<'t, S> {
        Measure::<S> {
            system: PhantomData,
            val: self.val - other.val,
        }
    }
}

// Section: Conditonal Impls
impl<'t, OTH: MS<'t>, S: MS<'t> + MultiplyBy<'t, OTH>> Mul<Measure<'t, OTH>> for Measure<'t, S> {
    type Output = Measure<'t, S::Output>;
    fn mul(self, other: Measure<'t, OTH>) -> Measure<'t, S::Output> {
        Self::Output {
            system: PhantomData,
            val: self.val * other.val,
        }
    }
}
impl<'t, OTH: MS<'t>, S: MS<'t> + DivideBy<'t, OTH>> Div<Measure<'t, OTH>> for Measure<'t, S> {
    type Output = Measure<'t, S::Output>;
    fn div(self, other: Measure<'t, OTH>) -> Self::Output {
        Self::Output {
            system: PhantomData,
            val: self.val / other.val,
        }
    }
}

impl<'t, S: MS<'t>> Mul<Flt> for Measure<'t, S> {
    type Output = Self;
    fn mul(self, other: Flt) -> Self {
        Self {
            system: PhantomData,
            val: self.val * other,
        }
    }
}

impl<'t, S: MS<'t>> Div<Flt> for Measure<'t, S> {
    type Output = Self;
    fn div(self, other: Flt) -> Self {
        Self {
            system: PhantomData,
            val: self.val / other,
        }
    }
}
