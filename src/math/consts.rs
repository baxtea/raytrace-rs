use lazy_static::lazy_static;
use super::{Scalar, Vec3};

#[cfg(feature="double-precision")]
pub use std::f64::consts::*;
#[cfg(not(feature="double-precision"))]
pub use std::f32::consts::*;

#[cfg(feature="double-precision")]
pub const EPSILON: Scalar = 1.0e-9;
#[cfg(not(feature="double-precision"))]
pub const EPSILON: Scalar = 1.0e-6;

lazy_static! {
    pub static ref ORIGIN  : Vec3 = Vec3::new(0.0, 0.0, 0.0);

    pub static ref RIGHT   : Vec3 = Vec3::new( 1.0,  0.0,  0.0);
    pub static ref LEFT    : Vec3 = Vec3::new(-1.0,  0.0,  0.0);
    pub static ref UP      : Vec3 = Vec3::new( 0.0,  1.0,  0.0);
    pub static ref DOWN    : Vec3 = Vec3::new( 0.0, -1.0,  0.0);
    pub static ref FORWARD : Vec3 = Vec3::new( 0.0,  0.0,  1.0);
    pub static ref BACKWARD: Vec3 = Vec3::new( 0.0,  0.0, -1.0);
}