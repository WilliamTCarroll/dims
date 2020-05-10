use crate::systems::Length;
use dims_core::*;
use std::marker::PhantomData;

pub const MILLIMETRE: Unit<Length> = Unit::<Length> {
    system: PhantomData,
    conv_enum: UnitConvert::Direct(1000.0),
};
