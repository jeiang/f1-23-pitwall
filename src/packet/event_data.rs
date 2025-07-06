use std::any::type_name;
use std::time::Duration;

use bitflags::bitflags;
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::packet::DeserializeUDPError::InvalidBuffer;
use crate::packet::{
    DeserializeUDP,
    DeserializeUDPResult,
    Infringement,
    Penalty,
    deserialize_bool,
    deserialize_option,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(crate) enum EventData {
    #[default]
    /// `SSTA`: Sent when the session starts
    SessionStarted,
    /// `SEND`: Sent when the session ends
    SessionEnded,
    /// `FTLP`: When a driver achieves the fastest lap
    FastestLap {
        vehicle_idx: usize,
        // NOTE: assumed to be in seconds
        lap_time: Duration,
    },
    /// `RTMT`: When a driver retires
    Retirement { vehicle_idx: usize },
    /// `DRSE`: Race control have enabled DRS
    DRSEnabled,
    /// `DRSD`: Race control have disabled DRS
    DRSDisabled,
    /// `TMPT`: Your teammate has entered the pits
    TeamMateInPits { vehicle_idx: usize },
    /// `CHQF`: The chequered flag has been waved
    ChequeredFlag,
    /// `RCWN`: The race winner is announced
    RaceWinner { vehicle_idx: usize },
    /// `PENA`: A penalty has been issued – details in event
    PenaltyIssued {
        penalty_type: Penalty,
        infringement_type: Infringement,
        vehicle_idx: usize,
        other_vehicle_idx: Option<usize>,
        /// Time gained, or time spent doing action in seconds
        time: Option<u8>,
        /// Lap the penalty occurred on
        lap_number: u8,
        /// Number of places gained by this
        places_gained: Option<u8>,
    },
    /// `SPTP`: Speed trap has been triggered by fastest speed
    SpeedTrapTriggered {
        /// index of the car that triggered the speed trap
        vehicle_idx: usize,
        /// speed of the car when it triggered the speed trap
        speed: f32,
        /// is fastest overall speed among all driver
        is_overall_fastest_in_session: bool,
        /// is the fastest speed triggered by driver
        is_driver_fastest_in_session: bool,
        /// index of the fastest vehicle in the session
        fastest_vehicle_idx_in_session: usize,
        /// top speed of the fastest car in the session
        fastest_speed_in_session: f32,
    },
    /// `STLG`: Start lights – number shown
    StartLights {
        /// number of lights showing
        num_lights: u8,
    },
    /// `LGOT`: Lights out
    LightsOut,
    /// `DTSV`: Drive through penalty served
    DriveThroughServed {
        /// idx of the car that served the drive through
        vehicle_idx: usize,
    },
    /// `SGSV`: Stop go penalty served
    StopGoServed {
        /// idx of the car that served the penalty
        vehicle_idx: usize,
    },
    /// `FLBK`: Flashback activated
    Flashback {
        /// the frame identifier that was flashed back to
        flashback_frame_identifier: u32,
        /// the time in the session when the flashback jumped to
        flashback_session_time: f32,
    },
    /// `BUTN`: Button status changed
    ButtonStatus(ButtonFlags),
    /// `RDFL`: Red flag shown
    RedFlag,
    /// `OVTK`: Overtake occurred
    Overtake {
        /// the idx of the overtaking car
        overtaking_vehicle_idx: usize,
        /// the idx of the car being overtaken
        being_overtaken_vehicle_idx: usize,
    },
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ButtonFlags: u32 {
        const CrossOrA = 0x0000_0001;
        const TriangleOrY = 0x0000_0002;
        const CircleOrB = 0x0000_0004;
        const SquareOrX = 0x0000_0008;
        const DPadLeft = 0x0000_0010;
        const DPadRight = 0x0000_0020;
        const DPadUp = 0x0000_0040;
        const DPadDown = 0x0000_0080;
        const OptionsOrMenu = 0x0000_0100;
        const L1OrLB = 0x0000_0200;
        const R1OrRB = 0x0000_0400;
        const L2OrLT = 0x0000_0800;
        const R2OrRT = 0x0000_1000;
        const LeftStickClick = 0x0000_2000;
        const RightStickClick = 0x0000_4000;
        const RightStickLeft = 0x0000_8000;
        const RightStickRight = 0x0001_0000;
        const RightStickUp = 0x0002_0000;
        const RightStickDown = 0x0004_0000;
        const Special = 0x0008_0000;
        const UDPAction1 = 0x0010_0000;
        const UDPAction2 = 0x0020_0000;
        const UDPAction3 = 0x0040_0000;
        const UDPAction4 = 0x0080_0000;
        const UDPAction5 = 0x0100_0000;
        const UDPAction6 = 0x0200_0000;
        const UDPAction7 = 0x0400_0000;
        const UDPAction8 = 0x0800_0000;
        const UDPAction9 = 0x1000_0000;
        const UDPAction10 = 0x2000_0000;
        const UDPAction11 = 0x4000_0000;
        const UDPAction12 = 0x8000_0000;

        // The source may set any bits https://docs.rs/bitflags/latest/bitflags/#externally-defined-flags
        const _ = !0;
    }
}

#[allow(clippy::too_many_lines)]
impl DeserializeUDP for EventData {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf).await?;
        let string = String::from_utf8_lossy(&buf);
        match string.as_ref() {
            "SSTA" => Ok(EventData::SessionStarted),
            "SEND" => Ok(EventData::SessionEnded),
            "FTLP" => {
                let vehicle_idx = usize::from(u8::deserialize(&mut reader).await?);
                let lap_time = Duration::from_secs_f32(f32::deserialize(&mut reader).await?);
                Ok(EventData::FastestLap { vehicle_idx, lap_time })
            }
            "RTMT" => {
                let vehicle_idx = usize::from(u8::deserialize(&mut reader).await?);
                Ok(EventData::Retirement { vehicle_idx })
            }
            "DRSE" => Ok(EventData::DRSEnabled),
            "DRSD" => Ok(EventData::DRSDisabled),
            "TMPT" => {
                let vehicle_idx = usize::from(u8::deserialize(&mut reader).await?);
                Ok(EventData::TeamMateInPits { vehicle_idx })
            }
            "CHQF" => Ok(EventData::ChequeredFlag),
            "RCWN" => {
                let vehicle_idx = usize::from(u8::deserialize(&mut reader).await?);
                Ok(EventData::RaceWinner { vehicle_idx })
            }
            "PENA" => {
                let penalty_type = Penalty::deserialize(&mut reader).await?;
                let infringement_type = Infringement::deserialize(&mut reader).await?;
                let vehicle_idx = usize::from(u8::deserialize(&mut reader).await?);
                let other_vehicle_idx =
                    deserialize_option(&mut reader, 255u8).await?.map(usize::from);
                let time = deserialize_option(&mut reader, 255u8).await?;
                let lap_number = u8::deserialize(&mut reader).await?;
                let places_gained = deserialize_option(&mut reader, 255u8).await?;
                Ok(EventData::PenaltyIssued {
                    penalty_type,
                    infringement_type,
                    vehicle_idx,
                    other_vehicle_idx,
                    time,
                    lap_number,
                    places_gained,
                })
            }
            "SPTP" => {
                let vehicle_idx = usize::from(u8::deserialize(&mut reader).await?);
                let speed = f32::deserialize(&mut reader).await?;
                let is_overall_fastest_in_session = deserialize_bool(&mut reader).await?;
                let is_driver_fastest_in_session = deserialize_bool(&mut reader).await?;
                let fastest_vehicle_idx_in_session =
                    usize::from(u8::deserialize(&mut reader).await?);
                let fastest_speed_in_session = f32::deserialize(&mut reader).await?;
                Ok(EventData::SpeedTrapTriggered {
                    vehicle_idx,
                    speed,
                    is_overall_fastest_in_session,
                    is_driver_fastest_in_session,
                    fastest_vehicle_idx_in_session,
                    fastest_speed_in_session,
                })
            }
            "STLG" => {
                let num_lights = u8::deserialize(&mut reader).await?;
                Ok(EventData::StartLights { num_lights })
            }
            "LGOT" => Ok(EventData::LightsOut),
            "DTSV" => {
                let vehicle_idx = usize::from(u8::deserialize(&mut reader).await?);
                Ok(EventData::DriveThroughServed { vehicle_idx })
            }
            "SGSV" => {
                let vehicle_idx = usize::from(u8::deserialize(&mut reader).await?);
                Ok(EventData::StopGoServed { vehicle_idx })
            }
            "FLBK" => {
                let flashback_frame_identifier = u32::deserialize(&mut reader).await?;
                let flashback_session_time = f32::deserialize(&mut reader).await?;
                Ok(EventData::Flashback { flashback_frame_identifier, flashback_session_time })
            }
            "BUTN" => {
                let value = u32::deserialize(&mut reader).await?;
                // should not panic as u32 is completely filled
                let flags = ButtonFlags::from_bits(value).unwrap();
                Ok(EventData::ButtonStatus(flags))
            }
            "RDFL" => Ok(EventData::RedFlag),
            "OVTK" => {
                let overtaking_vehicle_idx = usize::from(u8::deserialize(&mut reader).await?);
                let being_overtaken_vehicle_idx = usize::from(u8::deserialize(&mut reader).await?);
                Ok(EventData::Overtake { overtaking_vehicle_idx, being_overtaken_vehicle_idx })
            }
            _ => Err(InvalidBuffer { got: Box::new(buf), name: type_name::<EventData>() }),
        }
    }
}
