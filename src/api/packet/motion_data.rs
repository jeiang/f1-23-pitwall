use crate::api::models::Vector3;

#[derive(Debug)]
pub struct MotionData {
    pub car_motion: [CarMotionData; 22],
}

#[derive(Debug)]
pub struct CarMotionData {
    pub world_position: Vector3<f32>,
    pub world_velocity: Vector3<f32>,
    pub world_forward_dir: Vector3<i16>,
    pub world_right_dir: Vector3<i16>,
    pub g_force: Vector3<f32>,
    pub rotation: Vector3<f32>,
}
