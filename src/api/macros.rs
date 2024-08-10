macro_rules! generate_enum_deserialize_impls {
    ($enum_name:path) => {
        impl crate::api::packet::U8Deserializable for $enum_name {}
    };
    ($enum_name:path, $($enum_names:path),+) => {
        generate_enum_deserialize_impls!{ $enum_name }
        generate_enum_deserialize_impls!{ $($enum_names),+ }
    }
}

macro_rules! generate_deserialize_primitive {
    ($x:ident) => {
        paste::paste! {
            impl DeserializeUDP for $x {
                async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
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

pub(crate) use generate_deserialize_primitive;
pub(crate) use generate_enum_deserialize_impls;
