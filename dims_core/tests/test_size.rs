mod common;
use common::*;

#[cfg(feature = "f64")]
const SIZE: usize = 8;
#[cfg(not(feature = "f64"))]
const SIZE: usize = 4;
#[test]
fn check_measure_size() {
    use std::mem::{size_of, size_of_val};
    let raw = 25.4;
    let wrap = INCH.from(raw);

    // These should all be the same:
    // The number of bytes equal to the stored value
    assert_eq!(SIZE, size_of::<Measure<Length>>());
    assert_eq!(SIZE, size_of_val(&wrap));
    assert_eq!(SIZE, size_of_val(&raw));
    // Two floats and the strings
    assert_eq!(SIZE + SIZE + 48, size_of_val(&INCH));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_system_size() {
        use std::mem::size_of_val;
        // We need to ensure these always have a zero size
        assert_eq!(0, size_of_val(&Length));
        assert_eq!(0, size_of_val(&Area));
        assert_eq!(0, size_of_val(&Volume));
    }
}
