use crate::math::*;
use crate::{Camera, World, Ray};
use crate::Color3;
use rgb::RGB8;
#[cfg(feature="parallel")]
use rayon::prelude::*;

use std::iter::Iterator as Iter;

pub struct Screen {
    pub width: usize,
    pub height: usize,
}
impl Screen {
    pub fn new(w: usize, h: usize) -> Self {
        Screen {
            width: w,
            height: h,
        }
    }

    /// Generates rays in a scanline fashion, left to right then top to bottom.
    /// Returned as either a standard iterator or rayon parallel iterator of Rays, depending on the enabled features.
    // TODO: antialiasing
    fn primary_rays(&self, camera: &Camera) -> impl Iter<Item = Ray> {
        // Pixel size
        let dx =  2.0 / (self.width as Scalar);
        let dy = -2.0 / (self.height as Scalar);
        // Offset
        let ox = -1.0 + dx / 2.0;
        let oy =  1.0 - dy / 2.0;
        // Need to create a copy of these too, so they can be safely moved into the closure
        let w = self.width;
        let c = camera.clone();

        let it = (0..self.width*self.height).into_iter();
        it.map(move |i| {
            let px = i % w;
            let py = i / w;
            let x = ox + dx * (px as Scalar);
            let y = oy + dy * (py as Scalar);

            c.primary_ray(x, y)
        })
    }

    // TODO: trace rays instead of only casting
    // TODO: shade based on material
    pub fn render(&self, camera: &Camera, world: &World) -> Vec<RGB8> {
        // floating-point intermediate format, 0-1
        let black = Color3::new(0.0, 0.0, 0.0);
        let mut pixels = vec![black; self.width * self.height];
        let rays = self.primary_rays(camera);

        #[cfg(feature="parallel")]
        let it = rays.zip_eq(pixels.as_mut_slice().into_par_iter());
        #[cfg(not(feature="parallel"))]
        let it = rays.zip(pixels.as_mut_slice().into_iter()); // TODO: zip_eq for std iterators?

        let one = Vec3::new(1.0,1.0,1.0);
        it.for_each(|(r, p): (Ray, &mut Color3)| {
            if let Some(hit) = world.cast(&r) {
                // Rescale hit.normal, which is in the range [-1, 1], into the range [0, 1]
                let n = Color3::from_vec3((hit.normal + one) / 2.0);
                *p = n;
            } else {
                *p = black;
            }
        });

        // convert to u8, 0-255
        pixels.into_iter().map(|v| v.to_rgb8()).collect()
    }
}