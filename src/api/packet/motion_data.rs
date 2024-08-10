use crate::api::{
    models::Vector3,
    packet::{
        reader::DeserializeUDP,
        DeserializeUDPResult,
    },
};
use tokio::io::AsyncRead;
use tracing::trace;

#[derive(Debug, Clone, PartialEq)]
pub struct MotionData {
    pub car_motion: Vec<CarMotionData>,
}

impl DeserializeUDP for MotionData {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let mut car_motion = vec![];
        for i in 0..22 {
            let value = CarMotionData::deserialize(&mut reader).await?;
            trace!("deserialized car {i} as {value:?}");
            car_motion.push(value);
        }
        Ok(MotionData { car_motion })
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq)]
pub struct CarMotionData {
    pub world_position: Vector3<f32>,
    pub world_velocity: Vector3<f32>,
    pub world_forward_dir: Vector3<i16>,
    pub world_right_dir: Vector3<i16>,
    pub g_force: Vector3<f32>,
    pub rotation: Vector3<f32>,
}

impl DeserializeUDP for CarMotionData {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let world_position = Vector3::deserialize(&mut reader).await?;
        trace!("parsed world_position as {world_position:?}");
        let world_velocity = Vector3::deserialize(&mut reader).await?;
        trace!("parsed world_velocity as {world_velocity:?}");
        let world_forward_dir = Vector3::deserialize(&mut reader).await?;
        trace!("parsed world_forward_dir as {world_forward_dir:?}");
        let world_right_dir = Vector3::deserialize(&mut reader).await?;
        trace!("parsed world_right_dir as {world_right_dir:?}");
        let g_force = Vector3::deserialize(&mut reader).await?;
        trace!("parsed g_force as {g_force:?}");
        let rotation = Vector3::deserialize(&mut reader).await?;
        trace!("parsed rotation as {rotation:?}");
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
