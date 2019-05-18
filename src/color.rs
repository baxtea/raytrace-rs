use crate::math::Vec3;
use nalgebra_glm as glm;
use std::ops::{Add, Sub, Mul, Div};
use rgb::RGB8;

#[derive(Copy, Clone)]
pub struct Color3 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Color3 {
    #[inline]
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color3 {
            r: r,
            g: g,
            b: b,
        }
    }
    #[inline]
    pub fn from_vec3(v: Vec3) -> Self {
        Color3 {
            r: v[0] as f32,
            g: v[1] as f32,
            b: v[2] as f32,
        }
    }
    #[inline]
    pub fn from_vec3_clamp(v: Vec3) -> Self {
        Color3 {
            r: glm::clamp_scalar(v[0], 0.0, 1.0) as f32,
            g: glm::clamp_scalar(v[1], 0.0, 1.0) as f32,
            b: glm::clamp_scalar(v[2], 0.0, 1.0) as f32,
        }
    }
    #[inline]
    pub fn from_rgb8(rgb: RGB8) -> Self {
        Color3 {
            r: (rgb.r as f32) / 255.0,
            g: (rgb.g as f32) / 255.0,
            b: (rgb.b as f32) / 255.0,
        }
    }
    #[inline]
    pub fn to_rgb8(&self) -> RGB8 {
        RGB8 {
            r: (self.r * 255.0) as u8,
            g: (self.g * 255.0) as u8,
            b: (self.b * 255.0) as u8,
        }
    }
}

impl Add for Color3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Color3 {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
impl Sub for Color3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Color3 {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}
impl Mul for Color3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Color3 {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
impl Mul<f32> for Color3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Color3 {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
impl Mul<Color3> for f32 {
    type Output = Color3;

    fn mul(self, rhs: Color3) -> Color3 {
        rhs * self
    }
}
impl Div for Color3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Color3 {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}
impl Div<f32> for Color3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Color3 {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}
impl Div<Color3> for f32 {
    type Output = Color3;

    fn div(self, rhs: Color3) -> Color3 {
        Color3 {
            r: self / rhs.r,
            g: self / rhs.g,
            b: self / rhs.b,
        }
    }
}