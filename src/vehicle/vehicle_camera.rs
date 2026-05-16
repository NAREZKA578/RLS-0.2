use crate::graphics::game_renderer::camera::SceneCamera;
use super::vehicle_state::VehicleState;
use glam::Vec3;

pub enum CameraMode {
    Walking,
    InCabin { offset: Vec3 },
    ThirdPerson { distance: f32, height: f32 },
}

impl CameraMode {
    pub fn cabin() -> Self {
        Self::InCabin {
            offset: Vec3::new(-0.4, 1.3, 0.5),
        }
    }

    pub fn third_person() -> Self {
        Self::ThirdPerson {
            distance: 8.0,
            height: 3.0,
        }
    }
}

pub fn apply_camera(
    camera: &mut SceneCamera,
    mode: &CameraMode,
    vehicle: &VehicleState,
    player_pos: Vec3,
) {
    match mode {
        CameraMode::Walking => {
            camera.position = player_pos;
        }
        CameraMode::InCabin { offset } => {
            let world_offset = vehicle.rotation * *offset;
            camera.position = vehicle.position + world_offset;
            let forward = vehicle.rotation * Vec3::NEG_Z;
            camera.position.y = camera.position.y.max(vehicle.position.y + 0.5);
        }
        CameraMode::ThirdPerson { distance, height } => {
            let back = vehicle.rotation * Vec3::Z;
            camera.position = vehicle.position + back * *distance + Vec3::Y * *height;
            let to_vehicle = (vehicle.position - camera.position).normalize();
            camera.yaw = (-to_vehicle.x).atan2(-to_vehicle.z);
            camera.pitch = to_vehicle.y.asin().clamp(-1.5, 1.5);
        }
    }
}