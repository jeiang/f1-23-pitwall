/// Game mode enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameMode {
    EventMode,
    GrandPrix,
    GrandPrix23,
    TimeTrial,
    Splitscreen,
    OnlineCustom,
    OnlineLeague,
    CareerInvitational,
    ChampionshipInvitational,
    Championship,
    OnlineChampionship,
    OnlineWeeklyEvent,
    StoryMode,
    Career22,
    Career22Online,
    Career23,
    Career23Online,
    Benchmark,
}
