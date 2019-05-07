use crate::math::Scalar;

#[cfg(feature="double-precision")]
pub use std::f64::consts::*;
#[cfg(not(feature="double-precision"))]
pub use std::f32::consts::*;

#[cfg(feature="double-precision")]
pub static EPSILON: Scalar = 1.0e-9;
#[cfg(not(feature="double-precision"))]
pub static EPSILON: Scalar = 1.0e-6;