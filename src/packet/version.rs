use tokio::io::AsyncRead;

use crate::packet::{
    DeserializeUDP,
    DeserializeUDPResult,
};

/// Generic struct for the F1 Version number
#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct Version {
    /// The major component of the version
    pub(crate) major: u8,
    /// The minor component of the version
    pub(crate) minor: u8,
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
