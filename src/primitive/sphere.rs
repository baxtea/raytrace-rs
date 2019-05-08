use crate::math::*;
use crate::raycast::{Ray, Hit};
use nalgebra_glm as glm;
use super::Primitive;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: Scalar,
}
impl Sphere {
    pub const fn new(center: Vec3, radius: Scalar) -> Self {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}
impl Primitive for Sphere {
    fn nearest_intersection(&self, ray: &Ray) -> Option<Hit> {
        let translated = ray.origin - self.center;
        // a = 1 because ray.direction is normalized
        let b = glm::dot(&translated, &(2.0*ray.direction));
        let c = glm::length2(&translated) - self.radius*self.radius;
        let mut d = b*b - 4.0*c; // discriminant

        if d >= 0.0 {
            d = d.sqrt();
            let mut dist1 = (-b + d) / 2.0;
            let mut dist2 = (-b - d) / 2.0;
            if dist1 > dist2 {
                std::mem::swap(&mut dist1, &mut dist2);
            }

            // Final collision distance: closest point in front of the ray
            let dist = if dist1 >= 0.0 { dist1 } else { dist2 };
            if dist >= 0.0 {
                Some(
                    Hit::new(dist, glm::normalize(&(ray.at(dist) - self.center)))
                )
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::*;
    use crate::raycast::Ray;
    use super::{Primitive, Sphere};
    use nalgebra_glm as glm;

    #[test]
    fn sphere_at_origin_closest_intersection() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, -2.0), *consts::FORWARD);
        let sphere = Sphere::new(*consts::ORIGIN, 1.0);
        let hit = sphere.nearest_intersection(&ray).unwrap();

        assert!((hit.distance - 1.0).abs() <= consts::EPSILON);
        assert!(glm::distance(&hit.normal, &Vec3::new(0.0, 0.0, -1.0)) <= consts::EPSILON);
    }

    #[test]
    fn sphere_at_origin_miss() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, -2.0), glm::normalize(&Vec3::new(0.0, 1.0, 1.0)));
        let sphere = Sphere::new(*consts::ORIGIN, 1.0);
        let hit = sphere.nearest_intersection(&ray);
        assert!(hit.is_none());
    }

    #[test]
    fn sphere_at_origign_cull_rear_intersections() {
        let ray = Ray::new(*consts::ORIGIN, *consts::FORWARD);
        let sphere = Sphere::new(*consts::ORIGIN, 1.0);
        let hit = sphere.nearest_intersection(&ray).unwrap();

        assert!((hit.distance - 1.0).abs() <= consts::EPSILON);
        assert!(glm::distance(&hit.normal, &*consts::FORWARD) <= consts::EPSILON);
    }

    #[test]
    fn sphere_translated_closest_intersection() {
        let ray = Ray::new(*consts::ORIGIN, *consts::FORWARD);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 11.0), 1.0);
        let hit = sphere.nearest_intersection(&ray).unwrap();

        println!("{:?} {:?}", sphere, hit);
        assert!((hit.distance - 10.0).abs() <= consts::EPSILON);
        assert!(glm::distance(&hit.normal, &Vec3::new(0.0, 0.0, -1.0)) <= consts::EPSILON);
    }
}
