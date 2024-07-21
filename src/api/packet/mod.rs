#![allow(clippy::module_name_repetitions)]
use crate::api::models::Version;

mod car_damage_data;
mod car_setup_data;
mod car_status_data;
mod car_telemetry_data;
mod event_data;
mod final_classification_data;
mod lap_data;
mod lobby_info_data;
mod motion_data;
mod motion_ex_data;
mod participants_data;
mod session_data;
mod session_history_data;
mod tyre_set_data;

pub use car_damage_data::CarDamageData;
pub use car_setup_data::CarSetupData;
pub use car_status_data::CarStatusData;
pub use car_telemetry_data::CarTelemetryData;
pub use event_data::EventData;
pub use final_classification_data::FinalClassificationData;
pub use lap_data::LapData;
pub use lobby_info_data::LobbyInfoData;
pub use motion_data::MotionData;
pub use motion_ex_data::MotionExData;
pub use participants_data::ParticipantsData;
pub use session_data::SessionData;
pub use session_history_data::SessionHistoryData;
pub use tyre_set_data::TyreSetData;

#[derive(Debug)]
pub struct Packet {
    pub header: Header,
    pub data: Data,
}

/// The header of a UDP packet from the F1 2023 game.
#[derive(Debug, Default)]
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

#[derive(Debug)]
pub enum Data {
    Motion(Box<MotionData>),
    Session(Box<SessionData>),
    Lap(Box<LapData>),
    Event(Box<EventData>),
    Participants(Box<ParticipantsData>),
    CarSetup(Box<CarSetupData>),
    CarTelemetry(Box<CarTelemetryData>),
    CarStatus(Box<CarStatusData>),
    FinalClassification(Box<FinalClassificationData>),
    LobbyInfo(Box<LobbyInfoData>),
    CarDamage(Box<CarDamageData>),
    SessionHistory(Box<SessionHistoryData>),
    TyreSet(Box<TyreSetData>),
    MotionEx(Box<MotionExData>),
}
