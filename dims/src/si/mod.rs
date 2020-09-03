//! This module contains a basic set of SI units.
pub mod temperature;

pub mod length {
    si_unit! {"METRE", crate::systems::Length}
}
pub mod area {
    si_unit! {"SQUARE","METRE", crate::systems::Area,2,1.0}
}
pub mod volume {
    si_unit! {"CUBIC","METRE", crate::systems::Volume,3,1.0}
    si_unit! {"","LITRE", crate::systems::Volume,1,1.0e-3}
}
pub mod mass {
    si_unit! {"GRAM", crate::systems::Mass}
}
#[cfg(test)]
mod test {
    use super::*;
    use area::*;
    use dims_core::prelude::*;
    use length::*;
    use volume::{CUBICDECIMETRE, CUBICMETRE, KILOLITRE, LITRE};
    #[test]
    fn test_mult_div() {
        let len1 = METRE.from(12.0);
        let len2 = METRE.from(10.0);
        let len3 = METRE.from(10.0);
        let area = len1 * len2;
        // These various methods should be equal
        let vol1 = len1 * len2 * len3;
        let vol2 = area * len3;
        let vol3 = len3 * area;
        let vol4 = CUBICMETRE.from(1200.0);
        assert_eq!(vol1, vol2);
        assert_eq!(vol2, vol3);
        assert_eq!(vol3 / len3, area);
        assert_eq!(vol3 / area, len3);
        assert_eq!(vol3, vol4);
        assert_eq!(vol4 / len2 / len3, len1);
        let vol5 = LITRE.from(1_200_000.0);
        let vol6 = CUBICDECIMETRE.from(1_200_000.0);
        assert_eq!(vol5, vol6);
        assert_eq!(vol4, vol5);
        let vol5 = KILOLITRE.from(120_000.0);
        let vol6 = CUBICMETRE.from(120_000.0);
        assert_eq!(vol5, vol6);
        let vol5 = volume::CUBICDECAMETRE.from(1.325);
        let vol6 = volume::MEGALITRE.from(1.325);
        assert_eq!(vol5.as_base().round(), vol6.as_base().round()); // Floating point fun

        // Check more non-base units
        let area1 = SQUAREKILOMETRE.from(1.4);
        let len1 = DECAMETRE.from(100.0);
        let len2 = DECAMETRE.from(140.0); // 1_400_000
        let area2 = len1 * len2;
        assert_eq!(area1, area2);
        let area3 = SQUAREMETRE.from(1_400_000.0);
        assert_eq!(area2, area3);
    }
}
