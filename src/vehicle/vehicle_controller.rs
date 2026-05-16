use super::vehicle_state::VehicleState;
use crate::platform::input::InputState;
use crate::world::heightmap::Heightmap;
use glam::{Quat, Vec3};
use std::f32::consts::PI;
use winit::keyboard::KeyCode;

pub struct VehicleController;

impl VehicleController {
    pub fn update(
        vehicle: &mut VehicleState,
        input: &InputState,
        dt: f32,
        heightmap: &Heightmap,
    ) {
        let throttle = if input.is_key_down(KeyCode::KeyW) || input.is_key_down(KeyCode::ArrowUp) {
            1.0_f32
        } else if input.is_key_down(KeyCode::KeyS) || input.is_key_down(KeyCode::ArrowDown) {
            -1.0
        } else {
            0.0
        };

        let steer_input = if input.is_key_down(KeyCode::KeyA) || input.is_key_down(KeyCode::ArrowLeft) {
            -1.0_f32
        } else if input.is_key_down(KeyCode::KeyD) || input.is_key_down(KeyCode::ArrowRight) {
            1.0
        } else {
            0.0
        };

        if throttle > 0.0 {
            vehicle.speed = (vehicle.speed + vehicle.acceleration * dt).min(vehicle.max_speed_fwd);
        } else if throttle < 0.0 {
            if vehicle.speed > 0.05 {
                vehicle.speed = (vehicle.speed - vehicle.braking * dt).max(0.0);
            } else {
                vehicle.speed = (vehicle.speed - vehicle.acceleration * dt).max(-vehicle.max_speed_rev);
            }
        } else {
            if vehicle.speed.abs() > 0.01 {
                vehicle.speed -= vehicle.drag * dt * vehicle.speed.signum();
                if vehicle.speed.abs() < 0.01 {
                    vehicle.speed = 0.0;
                }
            }
        }

        if steer_input.abs() > 0.01 {
            vehicle.steer = (vehicle.steer + steer_input * vehicle.steer_speed * dt).clamp(-1.0, 1.0);
        } else {
            let return_speed = vehicle.steer_speed * 1.5;
            if vehicle.steer > 0.01 {
                vehicle.steer = (vehicle.steer - return_speed * dt).max(0.0);
            } else if vehicle.steer < -0.01 {
                vehicle.steer = (vehicle.steer + return_speed * dt).min(0.0);
            }
        }

        if vehicle.speed.abs() > 0.5 {
            let steer_angle_rad = vehicle.steer * vehicle.max_steer_deg * PI / 180.0;
            let omega = vehicle.speed * steer_angle_rad.tan() / vehicle.wheelbase;
            let delta_yaw = omega * dt * vehicle.speed.signum();
            vehicle.rotation = vehicle.rotation * Quat::from_rotation_y(-delta_yaw);
        }

        if vehicle.speed.abs() > 0.001 {
            let forward = vehicle.rotation * Vec3::NEG_Z;
            vehicle.position += forward * vehicle.speed * dt;
        }

        let terrain_y = heightmap.sample(vehicle.position.x, vehicle.position.z);
        vehicle.position.y = terrain_y + 0.5;
    }

    pub fn model_matrix(vehicle: &VehicleState) -> glam::Mat4 {
        glam::Mat4::from_rotation_translation(vehicle.rotation, vehicle.position)
    }
}