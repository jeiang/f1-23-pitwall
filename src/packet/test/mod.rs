use super::*;
use crate::packet::Vector3;

#[tokio::test]
async fn deserialize_zeroed_car_motion_data() {
    let bytes = [0; 20 * 3];
    let sample_data = CarMotionData {
        world_position: Vector3::default(),
        world_velocity: Vector3::default(),
        world_forward_dir: Vector3::default(),
        world_right_dir: Vector3::default(),
        g_force: Vector3::default(),
        rotation: Vector3::default(),
    };
    let deserialized = CarMotionData::deserialize(&bytes[..]).await;
    assert!(deserialized.is_ok());
    assert_eq!(sample_data, deserialized.unwrap());
}
