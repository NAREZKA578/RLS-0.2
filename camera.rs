use glam::{Mat4, Vec3};
use std::f32::consts::FRAC_PI_2;

#[derive(Debug, Clone)]
pub struct SceneCamera {
    pub position: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub aspect_ratio: f32,
}

impl Default for SceneCamera {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneCamera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 1.7, 0.0),
            pitch: 0.0,
            yaw: 0.0,
            fov: 1.2,
            near: 0.1,
            far: 10000.0,
            aspect_ratio: 16.0 / 9.0,
        }
    }

    pub fn set_aspect(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width / height;
    }

    pub fn view_matrix(&self) -> Mat4 {
        let forward = self.forward();
        let up = self.up();
        let target = self.position + forward;

        Mat4::look_at_rh(self.position, target, up)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        let fov_rad = self.fov;
        Mat4::perspective_rh(fov_rad, self.aspect_ratio, self.near, self.far)
    }

    pub fn view_projection(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    pub fn forward(&self) -> Vec3 {
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        Vec3::new(-sin_yaw * cos_pitch, sin_pitch, -cos_yaw * cos_pitch).normalize()
    }

    pub fn right(&self) -> Vec3 {
        let forward = self.forward();
        forward.cross(Vec3::Y).normalize()
    }

    pub fn up(&self) -> Vec3 {
        self.right().cross(self.forward()).normalize()
    }

    pub fn move_forward(&mut self, amount: f32) {
        let forward = self.forward();
        self.position += forward * amount;
    }

    pub fn move_right(&mut self, amount: f32) {
        let right = self.right();
        self.position += right * amount;
    }

    pub fn rotate(&mut self, d_pitch: f32, d_yaw: f32) {
        self.pitch = (self.pitch + d_pitch).clamp(-FRAC_PI_2 + 0.01, FRAC_PI_2 - 0.01);
        self.yaw += d_yaw;
    }

    pub fn world_up() -> Vec3 {
        Vec3::Y
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }
}

impl Transform {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    pub fn matrix(&self) -> Mat4 {
        let t = Mat4::from_translation(self.position);
        let rx = Mat4::from_rotation_x(self.rotation.x);
        let ry = Mat4::from_rotation_y(self.rotation.y);
        let rz = Mat4::from_rotation_z(self.rotation.z);
        let s = Mat4::from_scale(self.scale);
        t * rz * ry * rx * s
    }
}
