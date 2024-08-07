/// The ruleset in use for the current game session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleSet {
    PracticeAndQualifying,
    Race,
    TimeTrial,
    TimeAttack,
    CheckpointChallenge,
    Autocross,
    Drift,
    AverageSpeedZone,
    RivalDuel,
}
