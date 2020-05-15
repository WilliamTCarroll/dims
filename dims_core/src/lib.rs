mod float;
mod system;

pub use float::Flt;
pub mod prelude {
    pub use super::float::Flt;
    pub use super::system::{Measure, UnitSimple, UnitTrait};
}

pub mod unit_creation {
    pub use super::float::Flt;
    pub use super::system::{Measure, MeasureSystem, UnitTrait};
}
