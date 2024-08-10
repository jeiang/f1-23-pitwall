use crate::api::packet::{
    DeserializeUDP,
    DeserializeUDPResult,
};
use tokio::io::AsyncRead;

/// Generic struct for the F1 Version number
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Version {
    /// The major component of the version
    pub major: u8,
    /// The minor component of the version
    pub minor: u8,
}

impl DeserializeUDP for Version {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let major = u8::deserialize(&mut reader).await?;
        let minor = u8::deserialize(&mut reader).await?;
        Ok(Version { major, minor })
    }
}
