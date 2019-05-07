#[cfg(feature="double-precision")]
pub type Scalar = f64;
#[cfg(not(feature="double-precision"))]
pub type Scalar = f32;

pub type Vec3 = nalgebra_glm::TVec3<Scalar>;
pub type Quat = nalgebra_glm::Qua<Scalar>;
pub type Mat4 = nalgebra_glm::TMat4<Scalar>;

pub mod consts;