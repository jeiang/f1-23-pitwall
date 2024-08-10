use crate::api::{
    models::Vector3,
    packet::reader::PacketDeserialize,
};
use tokio::io::AsyncRead;

#[derive(Debug)]
pub struct MotionData {
    pub car_motion: Vec<CarMotionData>,
}

impl PacketDeserialize for MotionData {
    async fn create_from<R>(mut reader: R) -> color_eyre::Result<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let mut car_motion = vec![];
        for _ in 0..22 {
            car_motion.push(CarMotionData::create_from(&mut reader).await?);
        }
        Ok(MotionData { car_motion })
    }
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

impl PacketDeserialize for CarMotionData {
    async fn create_from<R>(mut reader: R) -> color_eyre::Result<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let world_position = Vector3::create_from(&mut reader).await?;
        let world_velocity = Vector3::create_from(&mut reader).await?;
        let world_forward_dir = Vector3::create_from(&mut reader).await?;
        let world_right_dir = Vector3::create_from(&mut reader).await?;
        let g_force = Vector3::create_from(&mut reader).await?;
        let rotation = Vector3::create_from(&mut reader).await?;
        Ok(CarMotionData {
            world_position,
            world_velocity,
            world_forward_dir,
            world_right_dir,
            g_force,
            rotation,
        })
    }
}
