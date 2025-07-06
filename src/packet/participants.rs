use tokio::io::{AsyncRead, AsyncReadExt};
use tracing::trace;

use crate::packet::driver::Driver;
use crate::packet::nationality::Nationality;
use crate::packet::platform::Platform;
use crate::packet::team::Team;
use crate::packet::{DeserializeUDP, DeserializeUDPResult, deserialize_vec};

const PARTICIPANT_DATA_STRUCT_SIZE: usize = 58;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ParticipantsData {
    participants: Vec<ParticipantData>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct ParticipantData {
    is_ai: bool,
    driver: Driver,
    network_id: u8,
    team: Team,
    same_team: bool,
    race_number: u8,
    nationality: Nationality,
    name: String,
    public_telemetry: bool,
    online_names_enabled: bool,
    platform: Platform,
}

impl DeserializeUDP for ParticipantsData {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let length = reader.read_u8().await?;
        let participants = deserialize_vec(&mut reader, length as usize).await?;
        // discard invalid data
        for _ in 0..(22 - length) {
            let mut buf = [0; PARTICIPANT_DATA_STRUCT_SIZE];
            reader.read_exact(&mut buf).await?;
        }
        Ok(Self { participants })
    }
}

impl DeserializeUDP for ParticipantData {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let is_ai = reader.read_u8().await? == 1;
        trace!("deserialized is_ai as {is_ai}");
        let driver = Driver::deserialize(&mut reader).await?;
        trace!("deserialized driver is {driver:?}");
        let network_id = reader.read_u8().await?;
        trace!("deserialized network_id is {network_id}");
        let team = Team::deserialize(&mut reader).await?;
        trace!("deserialized team is {team:?}");
        let same_team = reader.read_u8().await? == 1;
        trace!("deserialized same_team as {same_team}");
        let race_number = reader.read_u8().await?;
        trace!("deserialized race number is {race_number}");
        let nationality = Nationality::deserialize(&mut reader).await?;
        trace!("deserialized nationality is {nationality:?}");
        let name = {
            let mut arr = Vec::with_capacity(48);
            for _ in 0..48 {
                let val = reader.read_u8().await?;
                if val != 0 {
                    arr.push(val);
                }
            }
            String::from_utf8_lossy(&arr).to_string()
        };
        trace!("deserialized name is {name:?}");
        let public_telemetry = reader.read_u8().await? == 1;
        trace!("deserialized public_telemetry is {public_telemetry:?}");
        let online_names_enabled = reader.read_u8().await? == 1;
        trace!("deserialized online_names_enabled is {online_names_enabled:?}");
        let platform = Platform::deserialize(&mut reader).await?;
        trace!("deserialized platform is {platform:?}");
        Ok(Self {
            is_ai,
            driver,
            network_id,
            team,
            same_team,
            race_number,
            nationality,
            name,
            public_telemetry,
            online_names_enabled,
            platform,
        })
    }
}
