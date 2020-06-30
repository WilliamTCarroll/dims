//! Length units still in common use in the US for surveying.
//! Some of these will vary slightly from "general" units (EX: 1.0 FOOT != 1.0 FOOT_SURVEY)

use crate::systems::Length;
use dims_core::unit_creation::*;

pub const LINK: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 792.0 / 3937.0,
};

pub const FOOT_SURVEY: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 1200.0 / 3937.0,
};

pub const ROD: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: LINK.ratio * 25.0,
};

pub const CHAIN: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: ROD.ratio * 4.0,
};

pub const FURLONG: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: CHAIN.ratio * 10.0,
};

pub const MILE_SURVEY: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: FURLONG.ratio * 8.0,
};

pub const LEAGUE: UnitSimple<Length> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: MILE_SURVEY.ratio * 3.0,
};
