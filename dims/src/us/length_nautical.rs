//! Units generally only used in the nautical context.

use crate::systems::LengthSystem;
use dims_core::unit_creation::*;

pub const FATHOM: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1143.0 / 625.0,
    #[cfg(feature = "std")]
    abbr: "ftm",
    #[cfg(feature = "std")]
    singular: "fathom",
    #[cfg(feature = "std")]
    plural: "fathoms",
};

pub const CABLE: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: FATHOM.ratio * 120.0,
    #[cfg(feature = "std")]
    abbr: "cb",
    #[cfg(feature = "std")]
    singular: "cable",
    #[cfg(feature = "std")]
    plural: "cables",
};

pub const MILE_NAUTICAL: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1852.0,
    #[cfg(feature = "std")]
    abbr: "nmi",
    #[cfg(feature = "std")]
    singular: "nautical mile",
    #[cfg(feature = "std")]
    plural: "nautical miles",
};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_length_nautical() {
        assert_eq!(FATHOM.from(1.0).as_base(), 1.8288);
        assert_eq!(FATHOM.from(240.0), CABLE.from(2.0));
        assert_eq!(MILE_NAUTICAL.from(1.0).as_base(), 1852.0);
    }
}
