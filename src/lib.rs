#![feature(const_fn)]

pub mod math;
mod ray;
pub mod primitive;
mod camera;
mod screen;
mod world;

pub use ray::{Hit, Ray};
pub use camera::Camera;
pub use screen::Screen;
pub use world::World;