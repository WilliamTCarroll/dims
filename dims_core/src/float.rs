#[cfg(feature = "f64")]
pub type Flt = f64;

#[cfg(not(feature = "f64"))]
pub type Flt = f32;
