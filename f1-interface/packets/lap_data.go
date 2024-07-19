package packets

const (
	// Driver Status
	DRIVER_STATUS_IN_GARAGE  = 0
	DRIVER_STATUS_FLYING_LAP = 1
	DRIVER_STATUS_IN_LAP     = 2
	DRIVER_STATUS_OUT_LAP    = 3
	DRIVER_STATUS_ON_TRACK   = 4

	// Result Status
	RESULT_STATUS_INVALID        = 0
	RESULT_STATUS_INACTIVE       = 1
	RESULT_STATUS_ACTIVE         = 2
	RESULT_STATUS_FINISHED       = 3
	RESULT_STATUS_DNF            = 4
	RESULT_STATUS_DISQUALIFIED   = 5
	RESULT_STATUS_NOT_CLASSIFIED = 6
	RESULT_STATUS_RETIRED        = 7

	// Pit status
	PIT_STATUS_NONE    = 0
	PIT_STATUS_PITTING = 1
	PIT_STATUS_IN_PIT  = 2
)

type LapData struct {
	LastLapTimeInMS             uint32  // Last lap time in milliseconds
	CurrentLapTimeInMS          uint32  // Current time around the lap in milliseconds
	Sector1TimeInMS             uint16  // Sector 1 time in milliseconds
	ector1TimeMinutes           uint8   // Sector 1 time whole minute part
	Sector2TimeInMS             uint16  // Sector 2 time in milliseconds
	Sector2TimeMinutes          uint8   // Sector 2 time whole minute part
	DeltaToCarInFrontInMS       int16   // Delta in MS to the car in front
	DeltaToCarBehindInMS        int16   // Delta in MS to the car behind
	LapDistance                 float32 // Distance vehicle is around current lap in meters – could be negative if line hasn’t been crossed yet
	TotalDistance               float32 // Total distance travelled in session in meters – could be negative if line hasn’t been crossed yet
	SafetyCarDelta              float32 // Delta in seconds for safety car
	CarPosition                 uint8   // Car
	CurrentLapNum               uint8   // Current lap number
	PitStatus                   uint8   // See PitStatus enum
	Sector                      uint8   // Current sector (0 = sector1, 1 = sector2, 2 = sector3)
	CurrentLapInvalid           uint8   // Current lap invalid - 0 = valid, 1 = invalid
	Penalties                   uint8   // Accumulated time penalties in seconds to be added
	TotalWarnings               uint8   // Accumulated number of warnings issued
	CornerCuttingWarnings       uint8   // Number of warnings for corner cutting
	NumUnservedDriveThroughPens uint8   // Number of unserved drive through penalties
	NumUnservedStopGoPens       uint8   // Number of unserved stop-go penalties
	GridPosition                uint8   // Grid position the vehicle
	DriverStatus                uint8   // See DriverStatus enum
	ResultStatus                uint8   // See ResultStatus enum
	PitLaneTimerActive          uint8   // Pit lane timing, 0 = inactive, 1 = active
	itLaneTimeInLaneInMS        uint16  // If active, the current time spent in the pit lane in ms
	PitStopTimerInMS            uint16  // Time of the actual pit stop in ms
	PitStopShouldServePen       uint8   // Whether the car should serve a penalty at this stop
}

type PacketLapData struct {
	Header                 PacketHeader // Header
	LapData                [22]LapData  // Lap data for all cars on track
	TimeTrialPBCarIndex    uint8        // Index of the Personal Best Car in time trial (255 if invalid)
	TimeTrialRivalCarIndex uint8        // Index of the Rival in time trial (255 if invalid)
}
