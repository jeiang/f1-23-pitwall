use crate::api::models::{
    Vector3,
    Version,
};
use tokio::io::AsyncRead;

pub trait PacketDeserialize {
    async fn create_from<R>(reader: R) -> color_eyre::eyre::Result<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized;
}

impl PacketDeserialize for u8 {
    async fn create_from<R>(mut reader: R) -> color_eyre::eyre::Result<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        use tokio::io::AsyncReadExt;
        let result = reader.read_u8().await?;
        Ok(result)
    }
}

impl<T> PacketDeserialize for Vector3<T>
where
    T: PacketDeserialize,
{
    async fn create_from<R>(mut reader: R) -> color_eyre::Result<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let x = T::create_from(&mut reader).await?;
        let y = T::create_from(&mut reader).await?;
        let z = T::create_from(&mut reader).await?;
        Ok(Vector3 { x, y, z })
    }
}

impl PacketDeserialize for Version {
    async fn create_from<R>(mut reader: R) -> color_eyre::Result<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let major = u8::create_from(&mut reader).await?;
        let minor = u8::create_from(&mut reader).await?;
        Ok(Version { major, minor })
    }
}

macro_rules! generate_deserialize_primitive {
    ($x:ident) => {
        paste::paste! {
            impl crate::api::packet::reader::PacketDeserialize for $x {
                async fn create_from<R>(mut reader: R) -> color_eyre::eyre::Result<Self>
                where
                    R: tokio::io::AsyncRead + Unpin,
                    Self: Sized,
                {
                    use tokio::io::AsyncReadExt;
                    let result = reader.[<read_ $x _le>]().await?;
                    Ok(result)
                }
            }
        }
    };
}

generate_deserialize_primitive!(i16);
generate_deserialize_primitive!(u16);
generate_deserialize_primitive!(i32);
generate_deserialize_primitive!(u32);
generate_deserialize_primitive!(i64);
generate_deserialize_primitive!(u64);
generate_deserialize_primitive!(f32);
generate_deserialize_primitive!(f64);
