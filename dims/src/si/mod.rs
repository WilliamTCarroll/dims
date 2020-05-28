//! This module contains a basic set of SI units.
pub mod area;
pub mod length;
pub mod temperature;
pub mod volume;

#[cfg(test)]
mod test {
    use super::*;
    use area::SQMETRE;
    use dims_core::prelude::*;
    use length::METRE;
    use volume::LITRE;
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
        let vol4 = LITRE.from(1200.0);
        assert_eq!(vol1, vol2);
        assert_eq!(vol2, vol3);
        assert_eq!(vol3 / len3, area);
        assert_eq!(vol3 / area, len3);
        assert_eq!(vol3, vol4);
        assert_eq!(vol4 / len2 / len3, len1);
    }
}
