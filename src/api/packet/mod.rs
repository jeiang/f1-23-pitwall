mod lap_data;
mod motion_data;
mod reader;
mod session_data;
#[cfg(test)]
mod test;

use crate::api::{
    models::Version,
    packet::lap_data::SessionLapData,
};
pub use motion_data::*;
pub use reader::*;
pub use session_data::*;
use tokio::io::{
    AsyncRead,
    AsyncReadExt,
};
use tracing::trace;

#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    /// The format of the packet. Should be 2023.
    pub format: u16,

    /// The last two digits of the game year. Should be 23.
    pub game_year: u8,

    /// The version of the game.
    pub game_version: Version,

    /// The version of the packet.
    pub version: u8,

    /// Unique identifier for the session.
    pub session_uid: u64,

    /// Session timestamp.
    pub session_time: f32,

    /// Identifier for the frame the data was retrieved on.
    pub frame_identifier: u32,

    /// Same as `frame_identifier`, but does not go back after flashbacks
    pub overall_frame_identifier: u32,

    /// The index of the player car in the array of cars.
    pub player_car_index: u8,

    /// Index of the secondary player car in the array of cars.
    pub secondary_player_car_index: Option<u8>,

    /// The actual data contained in this packet
    pub data: Data,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    Motion(Box<MotionData>),
    Session(Box<SessionData>),
    Lap(Box<SessionLapData>),
    Unknown,
}

impl DeserializeUDP for Packet {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let format = reader.read_u16_le().await?;
        trace!("parsed format as {format:?}");
        let game_year = reader.read_u8().await?;
        trace!("parsed game_year as {game_year:?}");
        let game_version = Version::deserialize(&mut reader).await?;
        trace!("parsed game_version as {game_version:?}");
        let version = reader.read_u8().await?;
        trace!("parsed version as {version:?}");
        let packet_id = reader.read_u8().await?;
        trace!("parsed packet_id as {packet_id:?}");
        let session_uid = reader.read_u64_le().await?;
        trace!("parsed session_uid as {session_uid:?}");
        let session_time = reader.read_f32_le().await?;
        trace!("parsed session_time as {session_time:?}");
        let frame_identifier = reader.read_u32_le().await?;
        trace!("parsed frame_identifier as {frame_identifier:?}");
        let overall_frame_identifier = reader.read_u32_le().await?;
        trace!("parsed overall_frame_identifier as {overall_frame_identifier:?}");
        let player_car_index = reader.read_u8().await?;
        trace!("parsed player_car_index as {player_car_index:?}");
        let secondary_player_car_index = {
            let idx = reader.read_u8().await?;
            if idx == 255 {
                None
            } else {
                Some(idx)
            }
        };
        trace!("parsed secondary_player_car_index as {secondary_player_car_index:?}");
        let data = match packet_id {
            0 => Data::Motion(Box::new(MotionData::deserialize(&mut reader).await?)),
            1 => Data::Session(Box::new(SessionData::deserialize(&mut reader).await?)),
            2 => Data::Lap(Box::new(SessionLapData::deserialize(&mut reader).await?)),
            _ => Data::Unknown,
        };
        trace!("parsed packet data as {data:?}");
        Ok(Self {
            format,
            game_year,
            game_version,
            version,
            session_uid,
            session_time,
            frame_identifier,
            overall_frame_identifier,
            player_car_index,
            secondary_player_car_index,
            data,
        })
    }
}
