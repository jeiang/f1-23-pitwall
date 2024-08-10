use crate::api::{
    macros::generate_deserialize_primitive,
    packet::DeserializeUDPError::ExceededValidRange,
};
use num_traits::FromPrimitive;
use std::any::type_name;
use thiserror::Error;
use tokio::io::{
    AsyncRead,
    AsyncReadExt,
};

#[derive(Error, Debug)]
pub enum DeserializeUDPError {
    #[error("failed to deserialize the udp packet because of {source}")]
    ReadError {
        #[from]
        #[backtrace]
        source: std::io::Error,
    },
    #[error("tried to deserialize {got} which is outside of the expected range for {name}")]
    ExceededValidRange { got: u8, name: &'static str },
}

pub type DeserializeUDPResult<T> = Result<T, DeserializeUDPError>;

pub trait DeserializeUDP {
    async fn deserialize<R>(reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized;
}

pub trait U8Deserializable {}

impl<T: U8Deserializable + FromPrimitive> DeserializeUDP for T {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let level = reader.read_u8().await?;
        Self::from_u8(level).ok_or(ExceededValidRange {
            got: level,
            name: type_name::<T>(),
        })
    }
}

impl DeserializeUDP for u8 {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        Ok(reader.read_u8().await?)
    }
}

impl DeserializeUDP for i8 {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        Ok(reader.read_i8().await?)
    }
}

generate_deserialize_primitive!(i16);
generate_deserialize_primitive!(u16);
generate_deserialize_primitive!(i32);
generate_deserialize_primitive!(u32);
generate_deserialize_primitive!(i64);
generate_deserialize_primitive!(u64);
generate_deserialize_primitive!(f32);
generate_deserialize_primitive!(f64);

pub async fn deserialize_bool<R>(mut reader: R) -> DeserializeUDPResult<bool>
where
    R: AsyncRead + Unpin,
{
    let value = u8::deserialize(&mut reader).await?;
    match value {
        0 => Ok(false),
        1 => Ok(true),
        x => Err(ExceededValidRange {
            got: x,
            name: type_name::<bool>(),
        }),
    }
}

pub async fn deserialize_option<R, T>(mut reader: R, none_value: T) -> DeserializeUDPResult<Option<T>>
where
    R: AsyncRead + Unpin,
    T: DeserializeUDP + PartialEq,
{
    let value = T::deserialize(&mut reader).await?;
    if value == none_value {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

pub async fn deserialize_vec<R, T>(mut reader: R, limit: usize) -> DeserializeUDPResult<Vec<T>>
where
    R: AsyncRead + Unpin,
    T: DeserializeUDP,
{
    let mut vec = Vec::with_capacity(limit);
    for _ in 0..limit {
        let value = T::deserialize(&mut reader).await?;
        vec.push(value);
    }
    Ok(vec)
}
