extern crate raytracer;
use raytracer::math::*;
use raytracer::primitive::Sphere;
use raytracer::{Camera, Screen, World};
use nalgebra_glm as glm;

fn main() {
    let sphere = Sphere::new(glm::zero(), 1.0);
    let world = World { primitives: vec![Box::new(sphere)] };
    let camera = Camera::new(Vec3::new(0.0, 0.0, 2.0), glm::quat_identity(), consts::FRAC_PI_3, 16.0/9.0, None);
    let screen = Screen::new(1920, 1080);
    let image = screen.render(&camera, &world);
    image.save("out/spherecast.png").unwrap();
    println!("done");
}