use crate::math::*;

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