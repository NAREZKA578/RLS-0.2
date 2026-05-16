use glam::{Vec3, Quat};

pub struct VehicleState {
    pub position: Vec3,
    pub rotation: Quat,
    pub speed: f32,
    pub steer: f32,
    pub max_speed_fwd: f32,
    pub max_speed_rev: f32,
    pub acceleration: f32,
    pub braking: f32,
    pub drag: f32,
    pub max_steer_deg: f32,
    pub steer_speed: f32,
    pub wheelbase: f32,
}

impl Default for VehicleState {
    fn default() -> Self {
        Self {
            position: Vec3::new(100.0, 0.5, 200.0),
            rotation: Quat::IDENTITY,
            speed: 0.0,
            steer: 0.0,
            max_speed_fwd: 33.3,
            max_speed_rev: 5.5,
            acceleration: 8.0,
            braking: 15.0,
            drag: 3.0,
            max_steer_deg: 35.0,
            steer_speed: 2.5,
            wheelbase: 2.7,
        }
    }
}

impl VehicleState {
    pub fn reset(&mut self, position: Vec3) {
        self.position = position;
        self.position.y = 0.5;
        self.rotation = Quat::IDENTITY;
        self.speed = 0.0;
        self.steer = 0.0;
    }
}