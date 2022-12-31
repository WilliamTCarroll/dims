use super::*;
use core::fmt;
use {MeasureSystem as MS, UnitTrait as UT};

use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Measure is a wrapped Measurement of a specific System.
///
/// The value is stored in the base unit of the given system. (EX: Metre for length)
///
/// This is `#[repr(transparent)]`, so when compiled,
/// the memory footprint is that of the underlying Float
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Default)]
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
// Section: ops

#[opimps::impl_ops(Add)]
fn add<S: MS>(self: Measure<S>, two: Measure<S>) -> Measure<S> {
    Measure {
        system: PhantomData,
        val: self.val.clone() + two.val.clone(),
    }
}
#[opimps::impl_ops_assign(AddAssign)]
fn add_assign<S: MS>(self: Measure<S>, two: Measure<S>) {
    self.val = self.val.clone() + two.val.clone();
}
#[opimps::impl_ops(Sub)]
fn sub<S: MS>(self: Measure<S>, two: Measure<S>) -> Measure<S> {
    Measure {
        system: PhantomData,
        val: self.val.clone() - two.val.clone(),
    }
}
#[opimps::impl_ops_assign(SubAssign)]
fn sub_assign<S: MS>(self: Measure<S>, two: Measure<S>) {
    self.val = self.val.clone() - two.val.clone();
}
// Impls for multiplying and dividing by primitives or external types
#[opimps::impl_ops_rprim(Mul)]
fn mul<N: NumTrait, S: MS<N = N>>(self: Measure<S>, other: N) -> Measure<S> {
    Measure {
        system: PhantomData,
        val: self.val.clone() * other.clone(),
    }
}
#[opimps::impl_op_assign(MulAssign)]
fn mul_assign<N: NumTrait, S: MS<N = N>>(self: Measure<S>, other: N) {
    self.val = self.val.clone() * other;
}

#[opimps::impl_ops_rprim(Div)]
fn div<N: NumTrait, S: MS<N = N>>(self: Measure<S>, other: N) -> Measure<S> {
    Measure {
        system: PhantomData,
        val: self.val.clone() / other.clone(),
    }
}
#[opimps::impl_op_assign(DivAssign)]
fn div_assign<N: NumTrait, S: MS<N = N>>(self: Measure<S>, other: N) {
    self.val = self.val.clone() * other;
}
// Section: Conditonal Impls

#[opimps::impl_ops(Mul)]
fn mul<N: NumTrait, S1: MS, S2: MS>(self: Measure<S1>, other: Measure<S2>) -> Measure<S1::Output>
where
    S1: MS<N = N> + MultiplyBy<S2>,
    S2: MS<N = N>,
{
    Measure {
        system: PhantomData,
        val: self.val.clone() * other.val.clone(),
    }
}

#[opimps::impl_ops(Div)]
fn div<N: NumTrait, S1: MS, S2: MS>(self: Measure<S1>, other: Measure<S2>) -> Measure<S1::Output>
where
    S1: MS<N = N> + DivideBy<S2>,
    S2: MS<N = N>,
{
    Measure {
        system: PhantomData,
        val: self.val.clone() / other.val.clone(),
    }
}

impl<N: NumTrait + Neg<Output = N>, S: MS<N = N>> Neg for Measure<S> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            system: PhantomData,
            val: -self.val,
        }
    }
}

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
