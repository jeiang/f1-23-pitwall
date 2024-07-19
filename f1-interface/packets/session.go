package packets

const (
	// Session type
	SESSION_TYPE_UNKNOWN    = 0
	SESSION_TYPE_P1         = 1
	SESSION_TYPE_P2         = 2
	SESSION_TYPE_P3         = 3
	SESSION_TYPE_SHORT_P    = 4
	SESSION_TYPE_Q1         = 5
	SESSION_TYPE_Q2         = 6
	SESSION_TYPE_Q3         = 7
	SESSION_TYPE_SHORT_Q    = 8
	SESSION_TYPE_OSQ        = 9
	SESSION_TYPE_R          = 10
	SESSION_TYPE_R2         = 11
	SESSION_TYPE_TIME_TRIAL = 12

	// Weather
	WEATHER_CLEAR       = 0
	WEATHER_LIGHT_CLOUD = 1
	WEATHER_OVERCAST    = 2
	WEATHER_LIGHT_RAIN  = 3
	WEATHER_HEAVY_RAIN  = 4
	WEATHER_STORM       = 5

	// Track ID
	TRACK_ID_MELBOURNE         = 0
	TRACK_ID_PAUL_RICARD       = 1
	TRACK_ID_SHANGHAI          = 2
	TRACK_ID_SAKHIR            = 3
	TRACK_ID_CATALUNYA         = 4
	TRACK_ID_MONACO            = 5
	TRACK_ID_MONTREAL          = 6
	TRACK_ID_SILVERSTONE       = 7
	TRACK_ID_HOCKENHEIM        = 8
	TRACK_ID_HUNGARORING       = 9
	TRACK_ID_SPA               = 10
	TRACK_ID_MONZA             = 11
	TRACK_ID_SINGAPORE         = 12
	TRACK_ID_SUZUKA            = 13
	TRACK_ID_ABU_DHABI         = 14
	TRACK_ID_TEXAS             = 15
	TRACK_ID_BRAZIL            = 16
	TRACK_ID_AUSTRIA           = 17
	TRACK_ID_SOCHI             = 18
	TRACK_ID_MEXICO            = 19
	TRACK_ID_BAKU              = 20
	TRACK_ID_SAKHIR_SHORT      = 21
	TRACK_ID_SILVERSTONE_SHORT = 22
	TRACK_ID_TEXAS_SHORT       = 23
	TRACK_ID_SAO_PAULO_SHORT   = 24
	TRACK_ID_HANOI             = 25
	TRACK_ID_ZANDVOORT         = 26
	TRACK_ID_IMOLA             = 27
	TRACK_ID_PORTIMAO          = 28
	TRACK_ID_JEDDAH            = 29
	TRACK_ID_MIAMI             = 30
	TRACK_ID_LAS_VEGAS         = 31
	TRACK_ID_LOSAIL            = 32

	// Sector
	SECTOR_1 = 0
	SECTOR_2 = 1
	SECTOR_3 = 2

	// Current Lap
	CURRENT_LAP_VALID   = 0
	CURRENT_LAP_INVALID = 1

	// Formula
	FORMULA_F1_MODERN  = 0
	FORMULA_F1_CLASSIC = 1
	FORMULA_F2         = 2
	FORMULA_F1_GENERIC = 3

	// Safety Car Status
	SAFETY_CAR_STATUS_NO_SAFETY_CAR      = 0
	SAFETY_CAR_STATUS_FULL_SAFETY_CAR    = 1
	SAFETY_CAR_STATUS_VIRTUAL_SAFETY_CAR = 2
	SAFETY_CAR_STATUS_FORMATION_LAP      = 3

	// Brake Assist
	BRAKE_ASSIST_OFF    = 0
	BRAKE_ASSIST_LOW    = 1
	BRAKE_ASSIST_MEDIUM = 2
	BRAKE_ASSIST_HIGH   = 3

	// Gear Assist
	GEAR_ASSIST_MANUAL               = 0
	GEAR_ASSIST_MANUAL_AND_SUGGESTED = 1
	GEAR_ASSIST_AUTO                 = 2

	// Dynamic Racing Line
	DYNAMIC_RACING_LINE_OFF         = 0
	DYNAMIC_RACING_LINE_CORNER_ONLY = 1
	DYNAMIC_RACING_LINE_FULL        = 2

	// Game Mode
	GAME_MODE_EVENT                     = 0
	GAME_MODE_GRAND_PRIX                = 3
	GAME_MODE_GRAND_PRIX_2023           = 4
	GAME_MODE_TIME_TRIAL                = 5
	GAME_MODE_SPLITSCREEN               = 6
	GAME_MODE_ONLINE_CUSTOM             = 7
	GAME_MODE_ONLINE_LEAGUE             = 8
	GAME_MODE_CAREER_INVITATIONAL       = 11
	GAME_MODE_CHAMPIONSHIP_INVITATIONAL = 12
	GAME_MODE_CHAMPIONSHIP              = 13
	GAME_MODE_ONLINE_CHAMPIONSHIP       = 14
	GAME_MODE_ONLINE_WEEKLY_EVENT       = 15
	GAME_MODE_STORY_MODE                = 17
	GAME_MODE_CAREER_2022               = 19
	GAME_MODE_CAREER_2022_ONLINE        = 20
	GAME_MODE_CAREER_2023               = 21
	GAME_MODE_CAREER_2023_ONLINE        = 22
	GAME_MODE_BENCHMARK                 = 127

	// Ruleset
	RULESET_PRACTICE_AND_QUALI   = 0
	RULESET_RACE                 = 1
	RULESET_TIME_TRIAL           = 2
	RULESET_TIME_ATTACK          = 3
	RULESET_CHECKPOINT_CHALLENGE = 6
	RULESET_AUTOCROSS            = 8
	RULESET_DRIFT                = 9
	RULESET_AVERAGE_SPEED_ZONE   = 10
	RULESET_RIVAL_DUEL           = 11

	// Session Duration
	SESSION_LENGTH_NONE        = 0
	SESSION_LENGTH_VERY_SHORT  = 2
	SESSION_LENGTH_SHORT       = 3
	SESSION_LENGTH_MEDIUM      = 4
	SESSION_LENGTH_MEDIUM_LONG = 5
	SESSION_LENGTH_LONG        = 6
	SESSION_LENGTH_FULL        = 7

	// Speed Units
	SPEED_UNITS_MPH = 0
	SPEED_UNITS_KPH = 1

	// T
)

type MarshalZone struct {
	ZoneStart float32 // Fraction (0..1) of way through the lap the marshal zone starts
	ZoneFlag  int8    // -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
}

type WeatherForecastSample struct {
	SessionType            uint8 // See SessionType enum
	TimeOffset             uint8 // Time in minutes the forecast is for
	Weather                uint8 // See Weather enum
	TrackTemperature       int8  // Track temp. in degrees celsius
	TrackTemperatureChange int8  // Track temp. change - 0 = up, 1 = down, 2 = no change
	AirTemperature         int8  // Air temp. in degrees celsius
	AirTemperatureChange   int8  // Air temp. change - 0 = up, 1 = down, 2 = no change
	RainPercentage         uint8 // Rain percentage
}

type PacketSessionData struct {
	Header                          PacketHeader
	Weather                         uint8                     // See Weather enum
	TrackTemperature                int8                      // Track temp. in degrees celsius
	AirTemperature                  int8                      // Air temp. in degrees celsius
	TotalLaps                       uint8                     // Total number of laps in this race
	TrackLength                     uint16                    // Track length in metres
	SessionType                     uint8                     // See SessionType enum
	TrackId                         int8                      // -1 for unknown, see Track enum
	Formula                         uint8                     // Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2, 3 = F1 Generic
	SessionTimeLeft                 uint16                    // Time left in session in seconds
	SessionDuration                 uint16                    // Session duration in seconds
	PitSpeedLimit                   uint8                     // Pit speed limit in kilometres per hour
	GamePaused                      uint8                     // Whether the game is paused
	IsSpectating                    uint8                     // Whether the player is spectating
	SpectatorCarIndex               uint8                     // Index of the car being spectated
	SliProNativeSupport             uint8                     // SLI Pro support, 0 = inactive, 1 = active
	NumMarshalZones                 uint8                     // Number of marshal zones to follow
	MarshalZones                    [21]MarshalZone           // List of marshal zones – max 21
	SafetyCarStatus                 uint8                     // See SafetyCarStatus enum
	NetworkGame                     uint8                     // 0 = offline, 1 = online
	NumWeatherForecastSamples       uint8                     // Number of weather samples to follow
	WeatherForecastSamples          [56]WeatherForecastSample // Array of weather forecast samples
	ForecastAccuracy                uint8                     // 0 = Perfect, 1 = Approximate
	AiDifficulty                    uint8                     // AI Difficulty rating – 0-110
	SeasonLinkIdentifier            uint64                    // Identifier for season - persists across saves
	WeekendLinkIdentifier           uint64                    // Identifier for weekend - persists across saves
	SessionLinkIdentifier           uint64                    // Identifier for session - persists across saves
	PitStopWindowIdealLap           uint8                     // Ideal lap to pit within for current strategy
	PitStopWindowLatestLap          uint8                     // Latest lap to pit within for current strategy
	PitStopRejoinPosition           uint8                     // Predicted position to rejoin at after pit stops
	SteeringAssist                  uint8                     // 0 = Off, 1 = On
	BrakingAssist                   uint8                     // See Braking Assist enum
	GearboxAssist                   uint8                     // See Gear Assist enum
	PitAssist                       uint8                     // 0 = Off, 1 = On
	PitReleaseAssist                uint8                     // 0 = Off, 1 = On
	ErsAssist                       uint8                     // ERS Assist - 0 = Off, 1 = On
	DrsAssist                       uint8                     // DRS Assist - 0 = Off, 1 = On
	DynamicRacingLine               uint8                     // See Dynamic Racing Line enum
	DynamicRacingLineType           uint8                     // 0 = 2D, 1 = 3D
	GameMode                        uint8                     // See Game Mode enum
	RuleSet                         uint8                     // See Rule Set enum
	TimeOfDay                       uint8                     // Time of day - minutes since midnight
	SessionLength                   uint8                     // See Session Duration enum
	SpeedUnitsLeadPlayer            uint8                     // See Speed Units enum
	TemperatureUnitsLeadPlayer      uint8                     // See Temperature Units enum
	SpeedUnitsSecondaryPlayer       uint8                     // See Speed Units enum
	TemperatureUnitsSecondaryPlayer uint8                     // See Temperature Units enum
	NumSafetyCarPeriods             uint8                     // Number of safety car periods in this session
	NumVirtualSafetyCarPeriods      uint8                     // Number of virtual safety car periods in this session
	NumRedFlagPeriods               uint8                     // Number of red flag periods in this session
}
