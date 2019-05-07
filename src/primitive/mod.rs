mod sphere;
pub use sphere::Sphere;

use super::raycast::{Hit, Ray};
pub trait Primitive {
    fn nearest_intersection(&self, ray: &Ray) -> Option<Hit>;
}