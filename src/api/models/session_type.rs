// TODO: determine which code is for a sprint race, and what R3 is
/// The type of session that is currently running
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionType {
    Unknown,
    Practice1,
    Practice2,
    Practice3,
    ShortPractice,
    Qualifying1,
    Qualifying2,
    Qualifying3,
    ShortQualifying,
    OneShotQualifying,
    Race,
    Race2,
    Race3,
    TimeTrial,
}
