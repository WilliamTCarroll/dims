/// Flt is the Floating Point type requested with crate options
#[cfg(feature = "f64")]
pub type Flt = f64;
/// Flt is the Floating Point type requested with crate options
#[cfg(not(feature = "f64"))]
pub type Flt = f32;
/// Specify a function to round a numeric to the specified number of decimals
pub trait RoundTo {
    /// Round the given value to the number of decimals specified
    fn round_to(&self, decimals: i32) -> Self;
}
impl RoundTo for Flt {
    fn round_to(&self, decimals: i32) -> Self {
        let decimals: Flt = (10.0 as Flt).powi(decimals);
        (self * decimals).round() / decimals
    }
}
