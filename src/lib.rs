#![feature(const_fn)]

pub mod math;
mod ray;
pub mod primitive;
mod camera;
mod screen;
mod world;
mod material;
mod color;

pub use ray::{Hit, Ray};
pub use camera::Camera;
pub use screen::Screen;
pub use world::World;
pub use material::Material;
pub use color::Color3;