mod sphere;
pub use sphere::Sphere;

use super::{Hit, Ray};
pub trait Primitive {
    fn nearest_intersection(&self, ray: &Ray) -> Option<Hit>;
}