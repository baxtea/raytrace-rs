use crate::math::Scalar;
use crate::{Camera, World, Ray};

use image::{ImageBuffer, RgbImage};

#[cfg(feature="parallel")]
use rayon::{prelude::*, iter::ParallelIterator as Iter};
#[cfg(not(feature="parallel"))]
use std::iter::Iterator as Iter;

pub struct Screen {
    pub width: u32,
    pub height: u32,
}
impl Screen {
    pub fn new(w: u32, h: u32) -> Self {
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

        #[cfg(not(feature="parallel"))]
        let it = (0..self.width*self.height).into_iter();
        #[cfg(feature="parallel")]
        let it = (0..self.width*self.height).into_par_iter();
        it.map(move |i| {
            let px = i % w;
            let py = i / w;
            let x = ox + dx * (px as Scalar);
            let y = oy + dy * (py as Scalar);

            c.primary_ray(x, y)
        })
    }

    // TODO: shade based on material
    // TODO: use a floating-point intermediate format
    // TODO: how to zip with parallel iterators?
    pub fn render(&self, camera: &Camera, world: &World) -> RgbImage {
        let mut im: RgbImage = ImageBuffer::new(self.width, self.height);
        let pixels = im.pixels_mut();
        let rays = self.primary_rays(camera);

        let it = rays.zip(pixels);

        it.for_each(|(r, p)| {
            if let Some(hit) = world.cast(&r) {
                let n = hit.normal;
                p[0] = (255.0 * (n[0] + 1.0) / 2.0) as u8;
                p[1] = (255.0 * (n[1] + 1.0) / 2.0) as u8;
                p[2] = (255.0 * (n[2] + 1.0) / 2.0) as u8;
            } else {
                p[0] = 0;
                p[1] = 0;
                p[2] = 0;
            }
        });

        im
    }
}