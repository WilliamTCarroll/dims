use crate::systems::Volume;
use dims_core::unit_creation::*;
pub const LITRE: UnitSimple<Volume> = UnitSimple::<Volume> {
    system: PhantomData,
    ratio: 1.0,
    offset: 0.0,
};
