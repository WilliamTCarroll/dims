use crate::systems::MassSystem;

pub type MassUnit<'t> = super::UnitType<'t, MassSystem>;

pub const OUNCE: MassUnit = MassUnit {
    offset: 0.0,
    ratio: POUND.ratio / 16.0,
    #[cfg(feature = "str")]
    abbr: "oz",
    #[cfg(feature = "str")]
    singular: "ounce",
    #[cfg(feature = "str")]
    plural: "ounces",
};
pub const POUND: MassUnit = MassUnit {
    offset: 0.0,
    ratio: 453.59237,
    #[cfg(feature = "str")]
    abbr: "lb",
    #[cfg(feature = "str")]
    singular: "pound",
    #[cfg(feature = "str")]
    plural: "pounds",
};
pub const KIP: MassUnit = MassUnit {
    offset: 0.0,
    ratio: POUND.ratio * 1000.0,
    #[cfg(feature = "str")]
    abbr: "kip",
    #[cfg(feature = "str")]
    singular: "kip",
    #[cfg(feature = "str")]
    plural: "kips",
};

pub const TON: MassUnit = MassUnit {
    offset: 0.0,
    ratio: POUND.ratio * 2000.0,
    #[cfg(feature = "str")]
    abbr: "ton",
    #[cfg(feature = "str")]
    singular: "ton",
    #[cfg(feature = "str")]
    plural: "tons",
};
