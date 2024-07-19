package packets

const (
	EVENT_SESSION_STARTED              = "SSTA" // Sent when the session starts
	EVENT_SESSION_ENDED                = "SEND" // Sent when the session ends
	EVENT_FASTEST_LAP                  = "FTLP" // When a driver achieves the fastest lap
	EVENT_RETIREMENT                   = "RTMT" // When a driver retires
	EVENT_DRS_ENABLED                  = "DRSE" // Race control have enabled DRS
	EVENT_DRS_DISABLED                 = "DRSD" // Race control have disabled DRS
	EVENT_TEAM_MATE_IN_PITS            = "TMPT" // Your team mate has entered the pits
	EVENT_CHECKERED_FLAG               = "CHQF" // The chequered flag has been waved
	EVENT_RACE_WINNER                  = "RCWN" // The race winner is announced
	EVENT_PENALTY_ISSUED               = "PENA" // A penalty has been issued – details in event
	EVENT_SPEED_TRAP_TRIGGERED         = "SPTP" // Speed trap has been triggered
	EVENT_START_LIGHTS                 = "SGLI" // Start lights are about to go out
	EVENT_LIGHTS_OUT                   = "LGOT" // Lights out signal at the start of the
	EVENT_DRIVE_THROUGH_PENALTY_SERVED = "DTSV" // Drive through penalty served
	EVENT_STOP_GO_PENALTY_SERVED       = "SGPS" // Stop go penalty served
	EVENT_FLASHBACK                    = "FLBK" // Flashback activated
	EVENT_BUTTON_STATUS                = "BUTN" // Button status changed
	EVENT_RED_FLAG                     = "RDFL" // Red flag
	EVENT_OVERTAKE                     = "OVTK" // Overtake

	// Penalty types
	PENALTY_DRIVE_THROUGH                                    = 0
	PENALTY_STOP_GO                                          = 1
	PENALTY_GRID_PENALTY                                     = 2
	PENALTY_PENALTY_REMINDER                                 = 3
	PENALTY_TIME_PENALTY                                     = 4
	PENALTY_WARNING                                          = 5
	PENALTY_DISQUALIFIED                                     = 6
	PENALTY_REMOVED_FROM_FORMATION_LAP                       = 7
	PENALTY_PARKED_TOO_LONG_TIMER                            = 8
	PENALTY_TYRE_REGULATIONS                                 = 9
	PENALTY_THIS_LAP_INVALIDATED                             = 10
	PENALTY_THIS_AND_NEXT_LAP_INVALIDATED                    = 11
	PENALTY_THIS_AND_PREVIOUS_LAP_INVALIDATED                = 12
	PENALTY_THIS_AND_PREVIOUS_LAP_INVALIDATED_WITHOUT_REASON = 13
	PENALTY_RETIRED                                          = 14
	PENALTY_BLACK_FLAG_TIMER                                 = 15

	// Infringement types
	INFRINGEMENT_BLOCKING_BY_SLOW_DRIVING                        = 0
	INFRINGEMENT_BLOCKING_BY_WRONG_WAY_DRIVING                   = 1
	INFRINGEMENT_REVERSING_OFF_THE_START_LINE                    = 2
	INFRINGEMENT_BIG_COLLISION                                   = 3
	INFRINGEMENT_SMALL_COLLISION                                 = 4
	INFRINGEMENT_COLLISION_FAILED_TO_HAND_BACK_POSITION_SINGLE   = 5
	INFRINGEMENT_COLLISION_FAILED_TO_HAND_BACK_POSITION_MULTIPLE = 6
	INFRINGEMENT_CORNER_CUTTING_GAINED_TIME                      = 7
	INFRINGEMENT_CORNER_CUTTING_OVERTAKE_SINGLE                  = 8
	INFRINGEMENT_CORNER_CUTTING_OVERTAKE_MULTIPLE                = 9
	INFRINGEMENT_CROSSING_PIT_EXIT                               = 10
	INFRINGEMENT_IGNORING_BLUE_FLAGS                             = 11
	INFRINGEMENT_IGNORING_YELLOW_FLAGS                           = 12
	INFRINGEMENT_IGNORING_DRIVE_THROUGH                          = 13
	INFRINGEMENT_TOO_MANY_DRIVE_THROUGHS                         = 14
	INFRINGEMENT_DRIVE_THROUGH_REMINDER_SERVE_WITHIN_N_LAPS      = 15
	INFRINGEMENT_DRIVE_THROUGH_REMINDER_SERVE_THIS_LAP           = 16
	INFRINGEMENT_PIT_LANE_SPEEDING                               = 17
	INFRINGEMENT_PARKED_FOR_TOO_LONG                             = 18
	INFRINGEMENT_IGNORING_TYRE_REGULATIONS                       = 19
	INFRINGEMENT_TOO_MANY_PENALTIES                              = 20
	INFRINGEMENT_MULTIPLE_WARNINGS                               = 21
	INFRINGEMENT_APPROACHING_DISQUALIFICATION                    = 22
	INFRINGEMENT_TYRE_REGULATIONS_SELECT_SINGLE                  = 23
	INFRINGEMENT_TYRE_REGULATIONS_SELECT_MULTIPLE                = 24
	INFRINGEMENT_LAP_INVALIDATED_CORNER_CUTTING                  = 25
	INFRINGEMENT_LAP_INVALIDATED_RUNNING_WIDE                    = 26
	INFRINGEMENT_CORNER_CUTTING_RAN_WIDE_GAINED_TIME_MINOR       = 27
	INFRINGEMENT_CORNER_CUTTING_RAN_WIDE_GAINED_TIME_SIGNIFICANT = 28
	INFRINGEMENT_CORNER_CUTTING_RAN_WIDE_GAINED_TIME_EXTREME     = 29
	INFRINGEMENT_LAP_INVALIDATED_WALL_RIDING                     = 30
	INFRINGEMENT_LAP_INVALIDATED_FLASHBACK_USED                  = 31
	INFRINGEMENT_LAP_INVALIDATED_RESET_TO_TRACK                  = 32
	INFRINGEMENT_BLOCKING_PIT_LANE                               = 33
	INFRINGEMENT_JUMP_START                                      = 34
	INFRINGEMENT_SAFETY_CAR_TO_CAR_COLLISION                     = 35
	INFRINGEMENT_SAFETY_CAR_ILLEGAL_OVERTAKE                     = 36
	INFRINGEMENT_SAFETY_CAR_EXCEEDING_ALLOWED_PACE               = 37
	INFRINGEMENT_VIRTUAL_SAFETY_CAR_EXCEEDING_ALLOWED_PACE       = 38
	INFRINGEMENT_FORMATION_LAP_BELOW_ALLOWED_SPEED               = 39
	INFRINGEMENT_FORMATION_LAP_PARKING                           = 40
	INFRINGEMENT_RETIRED_MECHANICAL_FAILURE                      = 41
	INFRINGEMENT_RETIRED_TERMINAL_DAMAGE                         = 42
	INFRINGEMENT_SAFETY_CAR_FALLING_BEHIND                       = 43
	INFRINGEMENT_BLACK_FLAG_TIMER                                = 44
	INFRINGEMENT_UNSERVED_STOP_GO_PENALTY                        = 45
	INFRINGEMENT_UNSERVED_DRIVE_THROUGH_PENALTY                  = 46
	INFRINGEMENT_ENGINE_COMPONENT_CHANGE                         = 47
	INFRINGEMENT_GEARBOX_CHANGE                                  = 48
	INFRINGEMENT_PARC_FERME_CHANGE                               = 49
	INFRIGEMENT_LEAGUE_GRID_PENALTY                              = 50
	INFRINGEMENT_RETRY_PENALTY                                   = 51
	INFRINGEMENT_ILLEGAL_TIME_GAIN                               = 52
	INFRINGEMENT_MANDATORY_PITSTOP                               = 53
	INFRINGEMENT_ATTRIBUTE_ASSIGNED                              = 54
)

type PacketEventData struct {
	Header      PacketHeader // Header
	EventString string       // Event string code, see below
	// Union of all event data types
	EventData interface{}
}

// EventString: FLTP
type FastestLap struct {
	VehicleIdx uint8   // Vehicle index of car achieving fastest lap
	LapTime    float32 // Lap time is in seconds
}

// EventString: RTMT
type Retirement struct {
	VehicleIdx uint8 // Vehicle index of car retiring
}

// EventString: TMPT
type TeamMateInPits struct {
	VehicleIdx uint8 // Vehicle index of team mate
}

// EventString: RCWN
type RaceWinner struct {
	VehicleIdx uint8 // Vehicle index of the race winner
}

// EventString: PENA
type Penalty struct {
	PenaltyType      uint8 // Penalty type, see below
	InfringementType uint8 // Infringement type, see below
	VehicleIdx       uint8 // Vehicle index of the car the penalty is applied to
	OtherVehicleIdx  uint8 // Vehicle index of the other car involved
	Time             uint8 // Time gained, or time spent doing action in seconds
	LapNum           uint8 // Lap the penalty occurred on
	PlacesGained     uint8 // Number of places gained by this
}

// EventString: SPTP
type SpeedTrap struct {
	VehicleIdx                 uint8   // Vehicle index of the vehicle triggering speed trap
	Speed                      float32 // Top speed achieved by vehicle in m/s
	IsOverallFastestInSession  uint8   // Whether the vehicle has the fastest speed overall in this session
	IsDriverFastestInSession   uint8   // Whether the vehicle has the fastest speed of all drivers in this session
	FastestVehicleIdxInSession uint8   // Vehicle index of the fastest speeder in this session
	FastestSpeedInSession      float32 // Overall fastest speed in this session
}

// EventString: SGLI
type StartLights struct {
	NumLights uint8 // Number of lights showing
}

// EventString: DTSV
type DriveThroughPenaltyServed struct {
	VehicleIdx uint8 // Vehicle index of the car serving drive through
}

// EventString: SGPS
type StopGoPenaltyServed struct {
	VehicleIdx uint8 // Vehicle index of the car serving stop go
}

// EventString: FLBK
type Flashback struct {
	FlashbackFrameIdentifier uint32  // Flashback frame identifier
	FlashbackSessionTime     float32 // Time into the session of the flashback
}

// EventString: BUTN
type ButtonStatus struct {
	ButtonStatus uint32 // Bit flags specifying which buttons are being pressed currently
}

// EventString: OVTK
type Overtake struct {
	OvertakingVehicleIdx     uint8 // Vehicle index of the car being overtaken
	BeingOvertakenVehicleIdx uint8 // Vehicle index of the car being overtaken
}
