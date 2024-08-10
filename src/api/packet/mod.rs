#![allow(clippy::module_name_repetitions)]

use crate::api::models::Version;
use tokio::io::{
    AsyncRead,
    AsyncReadExt,
};

pub mod motion_data;
pub mod reader;
pub mod session_data;

use crate::api::packet::reader::PacketDeserialize;
pub use motion_data::MotionData;
pub use session_data::SessionData;

#[derive(Debug)]
pub struct Packet {
    pub header: Header,
    pub data: Data,
}

/// The header of a UDP packet from the F1 2023 game.
#[derive(Debug)]
pub struct Header {
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
}

impl PacketDeserialize for Header {
    async fn create_from<R>(mut reader: R) -> color_eyre::eyre::Result<Self>
    where
        R: AsyncRead + Unpin,
    {
        let format = reader.read_u16_le().await?;
        let game_year = reader.read_u8().await?;
        let game_version = Version::create_from(&mut reader).await?;
        let version = reader.read_u8().await?;
        let session_uid = reader.read_u64_le().await?;
        let session_time = reader.read_f32_le().await?;
        let frame_identifier = reader.read_u32_le().await?;
        let overall_frame_identifier = reader.read_u32_le().await?;
        let player_car_index = reader.read_u8().await?;
        let secondary_player_car_index = {
            let idx = reader.read_u8().await?;
            if idx == 255 {
                None
            } else {
                Some(idx)
            }
        };

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
        })
    }
}

#[derive(Debug)]
pub enum Data {
    Motion(Box<MotionData>),
    Session(Box<SessionData>),
}
