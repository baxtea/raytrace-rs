/**
 * UE4-like material model defined by roughness, metallic, and albedo
 */
use crate::math::Vec3;
use crate::{Ray, Color3};
use nalgebra_glm as glm;

// TODO: ior
// TODO: textures
// TODO: emissive
#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub roughness: f32,
    pub metallic: f32,
    pub albedo: Color3,
}
impl Material {
    pub fn new<T: Into<Color3>>(roughness: f32, metallic: f32, albedo: T) -> Self {
        Material {
            roughness: roughness,
            metallic: metallic,
            albedo: albedo.into(),
        }
    }

    // TODO: texture coordinates (2D and 3D)
    pub fn shade(&self, ray: &Ray, normal: &Vec3, dir_to_light: &Vec3) -> Color3 {
        let dir_to_cam = -ray.direction;
        let nol = (glm::dot(normal, dir_to_light) as f32).max(0.0);

        let diffuse = self.albedo * nol;

        let exponent = 100;

        let reflect = glm::reflect_vec(&(-dir_to_light), normal);
        let vor = (glm::dot(&dir_to_cam, &reflect) as f32).max(0.0);
        let spec = vor.powi(exponent) * nol;
        let specular = Color3::new(spec, spec, spec);

        (diffuse + specular).clamped()

        // // Cook-Torrance microfacet specular
        // let half = glm::normalize(&(light_dir + view_dir));
        // let alpha = self.roughness * self.roughness;
        // let alpha_sq = alpha * alpha;

        // let nol = glm::dot(&normal, &light_dir) as f32;
        // let nov = glm::dot(&normal, &view_dir) as f32;
        // let noh = glm::dot(&normal, &half) as f32;
        // let voh = glm::dot(&view_dir, &half) as f32;

        // // D: normal distribution function
        // let d_denom = noh * noh * (alpha_sq * alpha_sq - 1.0) + 1.0;
        // let d = alpha_sq / (consts::PI * d_denom * d_denom);

        // // G: geometry/self-shadowing
        // let k = ((self.roughness + 1.0) * (self.roughness + 1.0)) / 8.0;
        // let g1_l = nol / (nol * (1.0 - k) + k);
        // let g1_v = nov / (nov * (1.0 - k) + k);
        // let g = g1_l * g1_v;

        // // F: schlick's fresnel approximation with a spherical gaussian
        // let f0 = 0.04;
        // let f = f0 + (1.0 - f0) * (2.0 as f32).powf((-5.55473*voh - 6.98316)*voh);

        // let specular_amount = (d * f * g) / (4.0 * nol * nov);
        // let specular_color = Color3 {
        //     r: f0 * (1.0 - self.metallic) + self.albedo.r * self.metallic,
        //     g: f0 * (1.0 - self.metallic) + self.albedo.g * self.metallic,
        //     b: f0 * (1.0 - self.metallic) + self.albedo.b * self.metallic,
        // };

        // // Lambertian diffuse
        // let diffuse_amount = 1.0 - f;
        // let diffuse_color = self.albedo / consts::PI;

        // diffuse_amount * diffuse_color + specular_amount * specular_color
    }
}

// a rough, non-metallic pink
impl Default for Material {
    fn default() -> Self {
        Material::new(1.0, 0.0, Color3::new(1.0, 0.0, 1.0))
    }
}