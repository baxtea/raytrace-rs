/**
 * UE4-like material model defined by roughness, metallic, and albedo
 */
use crate::math::Vec3;
use crate::{Ray, Color3};
use nalgebra_glm as glm;
use std::f32::consts::PI;

// TODO: textures
// TODO: emissive
#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub roughness: f32,
    pub metallic: f32,
    pub albedo: Color3,
    pub ior: f32,
}
impl Material {
    pub fn new(roughness: f32, metallic: f32, albedo: Color3, ior: f32) -> Self {
        Material {
            roughness: roughness,
            metallic: metallic,
            albedo: albedo,
            ior: ior,
        }
    }

    fn chi_ggx(v: f32) -> f32 {
        if v > 0.0 {
            1.0
        } else {
            0.0
        }
    }

    // TODO: texture coordinates (2D and 3D)
    pub fn shade(&self, ray: &Ray, normal: &Vec3, dir_to_light: &Vec3) -> Color3 {

        let view_dir = -ray.direction;

        // Cook-Torrance microfacet specular
        let half = glm::normalize(&(dir_to_light + view_dir));
        let alpha = self.roughness * self.roughness;
        let alpha_sq = alpha * alpha;

        let nol = glm::dot(&normal, &dir_to_light) as f32;
        let nov = glm::dot(&normal, &view_dir) as f32;
        let noh = glm::dot(&normal, &half) as f32;
        let noh_sq = noh * noh;
        let voh = glm::dot(&view_dir, &half) as f32;

        // D: normal distribution function
        let d_denom = noh_sq * alpha_sq + (1.0 - noh_sq);
        let d = (Self::chi_ggx(noh) * alpha_sq) / (PI * d_denom * d_denom);

        // G: geometry/self-shadowing
        let k = ((self.roughness + 1.0) * (self.roughness + 1.0)) / 8.0;
        let g1_l = nol / (nol * (1.0 - k) + k);
        let g1_v = nov / (nov * (1.0 - k) + k);
        let g = g1_l * g1_v;

        // F: schlick's fresnel approximation
        let sqrt_f0 = (1.0 - self.ior) / (1.0 + self.ior); // the 1.0 is the
        let f0 = Color3::gray(sqrt_f0 * sqrt_f0).mix(&self.albedo, self.metallic);
        let f = f0 + (Color3::gray(1.0) - f0) * (1.0 - voh).powi(5);

        let specular = (d * f * g) / (4.0 * nol * nov);

        // Lambertian diffuse
        // weight by 1 - metallic because metallic surfaces do not have any diffuse
        let diffuse = (1.0 - self.metallic) * self.albedo / PI;

        let nol = glm::clamp_scalar(nol, 0.0, 1.0);
        // TODO: weights k_s and k_d
        ((diffuse + specular) * nol).clamped()
        //Color3::gray(d).clamped()
    }
}

// iron
impl Default for Material {
    fn default() -> Self {
        Material::new(0.5, 0.0, Color3::new(1.0, 0.0, 1.0), 3.5)
    }
}