use crate::math::*;
use crate::primitive::Primitive;
use ord_subset::OrdSubsetIterExt;

pub struct World {
    pub primitives: Vec<Box<dyn Primitive>>,
    // TODO: acceleration data structure
}

#[derive(Debug)]
pub struct Hit {
    pub distance: Scalar,
    pub normal: Vec3,
    // TODO: more information (material, primitive id, etc)
}
impl Hit {
    pub fn new(distance: Scalar, normal: Vec3) -> Self {
        Hit {
            distance: distance,
            normal: normal,
        }
    }
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn at(&self, distance: Scalar) -> Vec3 {
        self.origin + self.direction*distance
    }

    pub fn cast(&self, w: &World) -> Option<Hit> {
        w.primitives.iter()
            .filter_map(|p| p.nearest_intersection(self))
            .ord_subset_min_by_key(|h| h.distance)
    }
}