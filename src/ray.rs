use crate::math::*;
use crate::Material;
use std::sync::Arc;

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
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub distance: Scalar,
    pub normal: Vec3,
    pub material: Arc<Material>,
    // TODO: more information (primitive id, etc)
}
impl Hit {
    pub fn new(distance: Scalar, normal: Vec3, material: &Arc<Material>) -> Self {
        Hit {
            distance: distance,
            normal: normal,
            material: material.clone(),
        }
    }
}