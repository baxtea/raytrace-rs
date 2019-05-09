use crate::primitive::Primitive;
use crate::ray::{Ray, Hit};
use ord_subset::OrdSubsetIterExt;

pub struct World {
    pub primitives: Vec<Box<dyn Primitive>>,
    // TODO: acceleration data structure
}
impl World {
    pub fn cast(&self, r: &Ray) -> Option<Hit> {
        self.primitives.iter()
            .filter_map(|p| p.nearest_intersection(&r))
            .ord_subset_min_by_key(|h| h.distance)
    }

    // TODO: pub fn trace(...)
}