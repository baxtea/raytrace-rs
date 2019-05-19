extern crate raytracer;
use raytracer::math::*;
use raytracer::primitive::Sphere;
use raytracer::{Camera, Screen, World, Material};
use nalgebra_glm as glm;

use std::{path::Path, fs::File, io::BufWriter, sync::Arc};
use png::HasParameters;

fn slice_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            slice.as_ptr() as *const u8,
            slice.len() * std::mem::size_of::<T>())
    }
}

fn main() {
    let sphere = Sphere::new(glm::zero(), 1.0, &Arc::new(Material::default()));
    let world = World { primitives: vec![Box::new(sphere)] };
    let camera = Camera::new(Vec3::new(0.0, 0.0, 2.0), glm::quat_identity(), consts::FRAC_PI_3, 16.0/9.0, None);
    let screen = Screen::new(1920, 1080);
    let image = screen.render(&camera, &world);
    let image_bytes = slice_bytes(&image);

    let path = Path::new(r"out/spherecast.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut e = png::Encoder::new(w, screen.width as u32, screen.height as u32);
    e.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = e.write_header().unwrap();
    writer.write_image_data(&image_bytes).unwrap();
}