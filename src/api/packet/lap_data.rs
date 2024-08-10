use crate::api::{
    models::{
        DriverStatus,
        PitStatus,
        ResultStatus,
        Sector,
    },
    packet::{
        deserialize_bool,
        deserialize_option,
        DeserializeUDP,
        DeserializeUDPResult,
    },
};
use std::{
    ops::Add,
    time::Duration,
};
use tokio::io::AsyncRead;
use tracing::trace;

#[derive(Debug, Clone, PartialEq)]
pub struct LapTimes {
    /// Current lap time
    pub current: Duration,
    /// Last lap time
    pub last: Duration,
    /// Sector 1 time
    pub sector1: Duration,
    /// Sector 2 time
    pub sector2: Duration,
}

impl DeserializeUDP for LapTimes {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let last_lap_time_ms = u32::deserialize(&mut reader).await?;
        trace!("parsed last_lap_time_ms as {last_lap_time_ms}");
        let current_lap_time_ms = u32::deserialize(&mut reader).await?;
        trace!("parsed current_lap_time_ms as {current_lap_time_ms}");
        let sector1_time_ms = u16::deserialize(&mut reader).await?;
        trace!("parsed sector1_time_ms as {sector1_time_ms}");
        let sector1_time_min = u8::deserialize(&mut reader).await?;
        trace!("parsed sector1_time_min as {sector1_time_min}");
        let sector2_time_ms = u16::deserialize(&mut reader).await?;
        trace!("parsed sector2_time_ms as {sector2_time_ms}");
        let sector2_time_min = u8::deserialize(&mut reader).await?;
        trace!("parsed sector2_time_min as {sector2_time_min}");

        let current = Duration::from_millis(u64::from(current_lap_time_ms));
        trace!("computed current lap time as {current:#?}");
        let last = Duration::from_millis(u64::from(last_lap_time_ms));
        trace!("computed last lap time as {last:#?}");
        let sector1 =
            Duration::from_mins(u64::from(sector1_time_min)).add(Duration::from_millis(u64::from(sector1_time_ms)));
        trace!("computed sector1 lap time as {sector1:#?}");
        let sector2 =
            Duration::from_mins(u64::from(sector2_time_min)).add(Duration::from_millis(u64::from(sector2_time_ms)));
        trace!("computed sector2 lap time as {sector2:#?}");
        Ok(Self {
            current,
            last,
            sector1,
            sector2,
        })
    }
}

/// Unserved Penalties
#[derive(Debug, Clone, PartialEq)]
pub struct Penalties {
    /// Accumulated time penalties in seconds to be added
    time: Duration,
    /// Accumulated number of warnings issued
    total_warnings: u8,
    /// Accumulated number of corner cutting warnings issued
    corner_cutting_warnings: u8,
    /// Num drive-through pens left to serve
    drive_through: u8,
    /// Num stop-go pens left to serve
    stop_go: u8,
}

impl DeserializeUDP for Penalties {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let time = Duration::from_secs(u64::from(u8::deserialize(&mut reader).await?));
        trace!("parsed penalties_time as {time:#?}");
        let total_warnings = u8::deserialize(&mut reader).await?;
        trace!("parsed lap data total_warnings as {total_warnings}");
        let corner_cutting_warnings = u8::deserialize(&mut reader).await?;
        trace!("parsed lap data corner_cutting_warnings as {corner_cutting_warnings}");
        let drive_through = u8::deserialize(&mut reader).await?;
        trace!("parsed penalties_drive_through as {drive_through}");
        let stop_go = u8::deserialize(&mut reader).await?;
        trace!("parsed penalties_stop_go as {stop_go}");
        Ok(Self {
            time,
            total_warnings,
            corner_cutting_warnings,
            drive_through,
            stop_go,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PitLapInfo {
    timer_active: bool,
    /// If active, the current time spent in the pit lane
    time_in_lane: Duration,
    /// Time of the actual pit stop
    time_of_stop: Duration,
    /// Whether the car should serve a penalty at this stop
    should_serve_penalty: bool,
}

impl DeserializeUDP for PitLapInfo {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let timer_active = deserialize_bool(&mut reader).await?;
        trace!("parsed pit timer_active as {timer_active}");
        let time_in_lane = Duration::from_millis(u64::from(u16::deserialize(&mut reader).await?));
        trace!("parsed pit time_in_lane as {time_in_lane:#?}");
        let time_of_stop = Duration::from_millis(u64::from(u16::deserialize(&mut reader).await?));
        trace!("parsed pit time_of_stop as {time_of_stop:#?}");
        let should_serve_penalty = deserialize_bool(&mut reader).await?;
        trace!("parsed pit should_serve_penalty as {should_serve_penalty}");
        Ok(Self {
            timer_active,
            time_in_lane,
            time_of_stop,
            should_serve_penalty,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LapData {
    pub lap_times: LapTimes,
    pub delta_to_car_in_front: Duration,
    pub delta_to_race_leader: Duration,
    /// Distance vehicle is around current lap in metres – could be negative if line hasn't been
    /// crossed yet
    pub lap_distance: f32,
    /// Total distance travelled in session in metres – could be negative if line hasn't been
    /// crossed yet
    pub total_distance: f32,
    pub safety_car_delta: Duration,
    pub race_position: u8,
    pub current_lap: u8,
    pub pit_status: PitStatus,
    pub num_pit_stops_taken: u8,
    pub sector: Sector,
    pub is_current_lap_invalid: bool,
    pub penalties: Penalties,
    pub starting_grid_position: u8,
    pub driver_status: DriverStatus,
    pub result_status: ResultStatus,
    pub pit_lap_info: PitLapInfo,
}

impl DeserializeUDP for LapData {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let lap_times = LapTimes::deserialize(&mut reader).await?;
        trace!("parsed lap data lap_times as {lap_times:?}");
        let delta_to_car_in_front = Duration::from_millis(u64::from(u16::deserialize(&mut reader).await?));
        trace!("parsed lap data delta_to_car_in_front as {delta_to_car_in_front:#?}");
        let delta_to_race_leader = Duration::from_millis(u64::from(u16::deserialize(&mut reader).await?));
        trace!("parsed lap data delta_to_race_leader as {delta_to_race_leader:#?}");
        let lap_distance = f32::deserialize(&mut reader).await?;
        trace!("parsed lap data lap_distance as {lap_distance}");
        let total_distance = f32::deserialize(&mut reader).await?;
        trace!("parsed lap data total_distance as {total_distance}");
        let safety_car_delta = Duration::from_secs_f32(f32::deserialize(&mut reader).await?);
        trace!("parsed lap data safety_car_delta as {safety_car_delta:#?}");
        let race_position = u8::deserialize(&mut reader).await?;
        trace!("parsed lap data race_position as {race_position}");
        let current_lap = u8::deserialize(&mut reader).await?;
        trace!("parsed lap data current_lap as {current_lap}");
        let pit_status = PitStatus::deserialize(&mut reader).await?;
        trace!("parsed lap data pit_status as {pit_status:?}");
        let num_pit_stops_taken = u8::deserialize(&mut reader).await?;
        trace!("parsed lap data num_pit_stops_taken as {num_pit_stops_taken}");
        let sector = Sector::deserialize(&mut reader).await?;
        trace!("parsed lap data sector as {sector:?}");
        let is_current_lap_invalid = deserialize_bool(&mut reader).await?;
        trace!("parsed lap data is_current_lap_invalid as {is_current_lap_invalid}");
        let penalties = Penalties::deserialize(&mut reader).await?;
        trace!("parsed lap data penalties as {penalties:?}");
        let starting_grid_position = u8::deserialize(&mut reader).await?;
        trace!("parsed lap data starting_grid_position as {starting_grid_position}");
        let driver_status = DriverStatus::deserialize(&mut reader).await?;
        trace!("parsed lap data driver_status as {driver_status:?}");
        let result_status = ResultStatus::deserialize(&mut reader).await?;
        trace!("parsed lap data result_status as {result_status:?}");
        let pit_lap_info = PitLapInfo::deserialize(&mut reader).await?;
        trace!("parsed lap data pit_lap_info as {pit_lap_info:?}");

        Ok(Self {
            lap_times,
            delta_to_car_in_front,
            delta_to_race_leader,
            lap_distance,
            total_distance,
            safety_car_delta,
            race_position,
            current_lap,
            pit_status,
            num_pit_stops_taken,
            sector,
            is_current_lap_invalid,
            penalties,
            starting_grid_position,
            driver_status,
            result_status,
            pit_lap_info,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SessionLapData {
    /// Lap data for all the cars in the session
    lap_data: Vec<LapData>,
    /// Index of the personal best car in `lap_data`
    time_trial_personal_best_car_idx: Option<usize>,
    /// Index of the rival car in `lap_data`
    time_trial_rival_car_idx: Option<usize>,
}

impl DeserializeUDP for SessionLapData {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let mut lap_data = Vec::with_capacity(22);
        for i in 0..22 {
            let value = LapData::deserialize(&mut reader).await?;
            trace!("deserialized car {i} lap data as {value:?}");
            lap_data.push(value);
        }
        let time_trial_personal_best_car_idx = deserialize_option(&mut reader, 255u8).await?.map(usize::from);
        trace!("parsed time_trial_personal_best_car_idx as {time_trial_personal_best_car_idx:?}");
        let time_trial_rival_car_idx = deserialize_option(&mut reader, 255u8).await?.map(usize::from);
        trace!("parsed time_trial_rival_car_idx as {time_trial_rival_car_idx:?}");
        Ok(Self {
            lap_data,
            time_trial_personal_best_car_idx,
            time_trial_rival_car_idx,
        })
    }
}
