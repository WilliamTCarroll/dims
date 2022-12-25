/// Flt is the Floating Point type requested with crate options
#[cfg(feature = "f64")]
pub type Flt = f64;
/// Flt is the Floating Point type requested with crate options
#[cfg(not(feature = "f64"))]
pub type Flt = f32;
