use nalgebra_glm as glm;
use crate::math::*;
use crate::raycast::Ray;
use consts::{RIGHT, UP, FORWARD};

#[derive(Debug, Clone)]
enum Age<T> {
    Old(T),
    New(T),
}

#[derive(Debug, Clone)]
enum Projection {
    // Field of view in radians, aspect ratio as width over height
    Perspective(Scalar,Scalar),
    // TODO: Orthographic
}

/**
 * A quaternion-based camera used to manage view and projection matrices
 *
 * While the internal state of the camera is consistent, the view matrix is invalidated with movement or rotation and not reconstructed until asked for.
 * After transformations, make sure to call update_view() before relying on the view matrix for anything.
 */
#[derive(Debug, Clone)]
pub struct Camera {
    position: Vec3,
    orientation: Quat,
    fixed_yaw_axis: Option<Vec3>, // yaw() rotates around this axis (usually global up) if specified, otherwise it uses its local up axis
    view: Age<Mat4>,
    proj: Projection,
}
impl Camera {
    pub fn new(pos: Vec3, orientation: Quat, fov: Scalar, aspect: Scalar, yaw_axis: Option<Vec3>) -> Self {
        Camera {
            position: pos,
            orientation: orientation,
            fixed_yaw_axis: match yaw_axis {
                Some(v) => Some(glm::normalize(&v)),
                None => None,
            },
            view: Age::New(glm::translate(&glm::quat_to_mat4(&orientation), &pos)),
            proj: Projection::Perspective(fov, aspect),
        }
    }
    pub fn new_fps(pos: Vec3, orientation: Quat, fov: Scalar, aspect: Scalar) -> Self {
        Camera {
            position: pos,
            orientation: orientation,
            fixed_yaw_axis: Some(*consts::UP),
            view: Age::New(glm::translate(&glm::quat_to_mat4(&orientation), &pos)),
            proj: Projection::Perspective(fov, aspect),
        }
    }


    pub fn get_position(&self) -> &Vec3 {
        &self.position
    }
    pub fn set_position(&mut self, pos: &Vec3) {
        self.position = *pos;
    }

    pub fn move_global(&mut self, offset: &Vec3) {
        self.position += offset;
    }
    pub fn move_relative(&mut self, offset: &Vec3) {
        self.position += glm::quat_rotate_vec3(&self.orientation, &offset);
    }


    pub fn get_right(&self) -> Vec3 {
        match &self.view {
            Age::New(v) => glm::vec4_to_vec3(&glm::column(v, 0)),
            Age::Old(_) => glm::quat_rotate_vec3(&self.orientation, &RIGHT),
        }
    }
    pub fn get_up(&self) -> Vec3 {
        match &self.view {
            Age::New(v) => glm::vec4_to_vec3(&glm::column(v, 1)),
            Age::Old(_) => glm::quat_rotate_vec3(&self.orientation, &UP),
        }
    }
    pub fn get_direction(&self) -> Vec3 {
        match &self.view {
            Age::New(v) => glm::vec4_to_vec3(&glm::column(v, 2)),
            Age::Old(_) => glm::quat_rotate_vec3(&self.orientation, &FORWARD),
        }
    }
    pub fn set_direction(&mut self, dir: &Vec3) {
        if let Some(up) = self.fixed_yaw_axis {
            self.orientation = glm::quat_look_at(dir, &up);
        } else {
            self.orientation = glm::quat_look_at(dir, &self.get_up());
        }
        self.invalidate_view();
    }
    pub fn look_at(&mut self, target: &Vec3) {
        self.set_direction(&(target - self.position));
    }


    pub fn get_orientation(&self) -> &Quat {
        &self.orientation
    }
    pub fn set_orientation(&mut self, q: &Quat) {
        self.orientation = *q;
        self.invalidate_view();
    }
    pub fn rotate(&mut self, q: &Quat) {
        self.orientation = q * self.orientation;
        self.invalidate_view();
    }
    // Rotates the camera counterclockwise around its local x axis
    pub fn pitch(&mut self, rads: Scalar) {
        self.orientation = glm::quat_angle_axis(rads, &self.get_right()) * self.orientation;
        self.invalidate_view();
    }
    // Rotates the camera counterclockwise around either its local y axis, or its set yaw axis if one is set
    pub fn yaw(&mut self, rads: Scalar) {
        self.orientation = glm::quat_angle_axis(rads, &self.get_yaw_axis()) * self.orientation;
        self.invalidate_view();
    }
    // Rotates the camera counterclockwise around its local z axis
    pub fn roll(&mut self, rads: Scalar) {
        self.orientation = glm::quat_angle_axis(rads, &self.get_direction()) * self.orientation;
        self.invalidate_view();
    }


    pub fn get_yaw_axis(&self) -> Vec3 {
        match self.fixed_yaw_axis {
            Some(a) => a,
            None => self.get_up(),
        }
    }
    pub fn set_yaw_axis(&mut self, axis: Option<Vec3>) {
        self.fixed_yaw_axis = match axis {
            Some(v) => Some(glm::normalize(&v)),
            None => None,
        }
    }


    pub fn get_view(&self) -> &Mat4 {
        match &self.view {
            Age::Old(v) => v,
            Age::New(v) => v,
        }
    }
    pub fn update_view(&mut self) {
        self.view = match self.view {
            Age::New(v) => Age::New(v),
            Age::Old(_) => Age::New(glm::translate(&glm::quat_to_mat4(&self.orientation), &self.position)),
        };
    }
    pub fn is_view_invalid(&self) -> bool {
        match &self.view {
            Age::Old(_) => true,
            Age::New(_) => false,
        }
    }
    pub fn invalidate_view(&mut self) {
        if let Age::New(v) = self.view {
            self.view = Age::Old(v);
        }
    }


    pub fn set_persp(&mut self, fov: Scalar, aspect: Scalar) {
        self.proj = Projection::Perspective(fov, aspect);
    }
    // TODO: pub fn set_ortho(...)


    /// x and y are in *screen space*: (-1,-1) is bottom left, (0,0) is center, (1,1) is top right
    /// Does not rely on the view matrix being up-to-date, only uses the quaternion
    pub fn primary_ray(&self, x: Scalar, y: Scalar) -> Ray {
        match &self.proj {
            Projection::Perspective(fov, aspect) => {
                let tan_fov_over_2: Scalar = (fov / 2.0).tan();
                let px = x * aspect * tan_fov_over_2;
                let py = y * tan_fov_over_2;
                let dir = glm::quat_rotate_vec3(&self.orientation, &glm::normalize(&Vec3::new(px, py, -1.0)));
                Ray::new(self.position, dir)
            }
        }
    }
}

static DEFAULT_ASPECT: Scalar = 16.0/9.0;
static DEFAULT_FOV: Scalar = consts::FRAC_PI_3; // 60 degrees

/// An FPS camera at the origin, facing down the z axis with y up
/// Fov is 60 degrees, aspect ratio is 16:9
impl Default for Camera {
    fn default() -> Self {
        Camera::new_fps(glm::zero(), glm::quat_identity(), DEFAULT_FOV, DEFAULT_ASPECT)
    }
}

#[cfg(test)]
mod tests {
    use super::Camera;
    use crate::math::*;
    use nalgebra_glm as glm;

    #[test]
    fn extract_axes_from_matrix() {
        // TODO: check that this works with a non-default starting orientation
        let camera = Camera::default();

        // The view matrix is created fresh with the new camera
        assert!(!camera.is_view_invalid());
        assert!(glm::length(&(camera.get_right() - *consts::RIGHT)) < consts::EPSILON);
        assert!(glm::length(&(camera.get_up() - *consts::UP)) < consts::EPSILON);
        assert!(glm::length(&(camera.get_direction() - *consts::FORWARD)) < consts::EPSILON);
    }

    #[test]
    fn extract_axes_from_quat() {
        // TODO: check that this works with a non-default starting orientation
        let mut camera = Camera::default();

        // Force the camera to recreate the view matrix from its position and quaternion
        camera.invalidate_view();
        assert!(glm::length(&(camera.get_right() - *consts::RIGHT)) < consts::EPSILON);
        assert!(glm::length(&(camera.get_up() - *consts::UP)) < consts::EPSILON);
        assert!(glm::length(&(camera.get_direction() - *consts::FORWARD)) < consts::EPSILON);
    }

    #[test]
    fn fixed_yaw_rotation() {
        let mut camera = Camera::default();
        assert!(camera.fixed_yaw_axis.is_some());

        // For an FPS camera, the yaw axis should remain (0,1,0) regardless of any transformations
        camera.pitch(-consts::FRAC_PI_2);
        assert!(glm::length(&(camera.get_yaw_axis() - *consts::UP)) < consts::EPSILON);
        camera.yaw(consts::PI);

        assert!(camera.is_view_invalid());
        camera.update_view();
        assert!(!camera.is_view_invalid());

        assert!(glm::length(&(camera.get_right() - *consts::LEFT)) < consts::EPSILON);
        assert!(glm::length(&(camera.get_up() - *consts::FORWARD)) < consts::EPSILON);
        assert!(glm::length(&(camera.get_direction() - *consts::UP)) < consts::EPSILON);
    }

    #[test]
    fn unfixed_yaw_rotation() {
        let mut camera = Camera::new(glm::zero(), glm::quat_identity(), super::DEFAULT_FOV, super::DEFAULT_ASPECT, None);
        assert!(camera.fixed_yaw_axis.is_none());

        // For a freeform camera, the yaw axis should change to reflect the camera's local coordinate system
        camera.pitch(-consts::FRAC_PI_2);
        assert!(glm::length(&(camera.get_yaw_axis() - *consts::BACKWARD)) < consts::EPSILON);
        camera.yaw(consts::PI);

        assert!(camera.is_view_invalid());
        camera.update_view();
        assert!(!camera.is_view_invalid());

        assert!(glm::length(&(camera.get_right() - *consts::LEFT)) < consts::EPSILON);
        assert!(glm::length(&(camera.get_up() - *consts::BACKWARD)) < consts::EPSILON);
        assert!(glm::length(&(camera.get_direction() - *consts::DOWN)) < consts::EPSILON);
    }

    #[test]
    fn primary_ray_cast() {
        let camera = Camera::default();
        let ray = camera.primary_ray(0.0, 0.0);
        assert!(glm::length(&ray.origin) < consts::EPSILON);
        assert!(glm::length(&(ray.direction - Vec3::new(0.0, 0.0, -1.0))) < consts::EPSILON);
    }
}