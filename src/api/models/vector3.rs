use crate::api::packet::{
    DeserializeUDP,
    DeserializeUDPResult,
};
use tokio::io::AsyncRead;

/// Generic struct for the F1 Vector3
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> DeserializeUDP for Vector3<T>
where
    T: DeserializeUDP,
{
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let x = T::deserialize(&mut reader).await?;
        let y = T::deserialize(&mut reader).await?;
        let z = T::deserialize(&mut reader).await?;
        Ok(Vector3 { x, y, z })
    }
}
