extern crate raytracer;
use raytracer::math::*;
use raytracer::primitive::Sphere;
use raytracer::{Camera, Screen, World, Material};
use nalgebra_glm as glm;

use std::{path::Path, fs::File, io::BufWriter, error::Error, sync::Arc as Shared};
use png::HasParameters;

fn slice_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            slice.as_ptr() as *const u8,
            slice.len() * std::mem::size_of::<T>())
    }
}

fn write_png<T>(bytes: &[u8], w: u32, h: u32, path: &T) -> Result<(), Box<Error>>
    where T: AsRef<Path> {
    let file = File::create(path)?;
    let ref mut bw = BufWriter::new(file);

    let mut e = png::Encoder::new(bw, w.into(), h.into());
    e.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = e.write_header()?;
    writer.write_image_data(&bytes)?;

    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    let sphere = Sphere::new(glm::zero(), 1.0, &Shared::new(Material::default()));
    let world = World { primitives: vec![Box::new(sphere)] };
    let camera = Camera::new(Vec3::new(0.0, 0.0, 2.0), glm::quat_identity(), consts::FRAC_PI_3, 16.0/9.0, None);
    let screen = Screen::new(1920, 1080);
    let image = screen.render(&camera, &world);
    let image_bytes = slice_bytes(&image);

    const PATH: &'static str = r"out/spherecast.png";
    write_png(&image_bytes, screen.width as u32, screen.height as u32, &PATH)?;

    Ok(())
}