use crate::systems::Length;
use dims_core::*;
use std::marker::PhantomData;

pub const MILLIMETRE: UnitSimple<Length> = UnitSimple::<Length> {
    system: PhantomData,
    in_base: 1000.0,
};
