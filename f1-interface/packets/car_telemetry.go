package packets

const (
	// Driving surface
	DRIVING_SURFACE_TARMAC       = 0
	DRIVING_SURFACE_RUMBLE_STRIP = 1
	DRIVING_SURFACE_CONCRETE     = 2
	DRIVING_SURFACE_ROCK         = 3
	DRIVING_SURFACE_GRAVEL       = 4
	DRIVING_SURFACE_MUD          = 5
	DRIVING_SURFACE_SAND         = 6
	DRIVING_SURFACE_GRASS        = 7
	DRIVING_SURFACE_WATER        = 8
	DRIVING_SURFACE_COBBLESTONE  = 9
	DRIVING_SURFACE_METAL        = 10
	DRIVING_SURFACE_RIDGED       = 11

	// MFD Panel Index
	MFD_PANEL_CAR_SETUP    = 0
	MFD_PANEL_PITS         = 1
	MFD_PANEL_DAMAGE       = 2
	MFD_PANEL_ENGINE       = 3
	MFD_PANEL_TEMPERATURES = 4
	MFD_PANEL_CLOSED       = 255
)

type CarTelemetryData struct {
	Speed                   uint16     // Speed of car in kilometres per hour
	Throttle                float32    // Amount of throttle applied (0.0 to 1.0)
	Steer                   float32    // Steering (-1.0 (full lock left) to 1.0 (full lock right))
	Brake                   float32    // Amount of brake applied (0.0 to 1.0)
	Clutch                  uint8      // Amount of clutch applied (0 to 100)
	Gear                    int8       // Gear selected (1-8, N=0, R=-1)
	EngineRPM               uint16     // Engine RPM
	DRS                     uint8      // 0 = off, 1 = on
	RevLightsPercent        uint8      // Rev lights indicator (percentage)
	RevLightsBitValue       uint16     // Rev lights (bit 0 = 1st lights on, bit 1 = 2nd lights on, etc)
	BrakesTemperature       [4]uint16  // Brakes temperature (celsius)
	TyresSurfaceTemperature [4]uint8   // Tyres surface temperature (celsius)
	TyresInnerTemperature   [4]uint8   // Tyres inner temperature (celsius)
	EngineTemperature       uint16     // Engine temperature (celsius)
	TyresPressure           [4]float32 // Tyres pressure (PSI)
	SurfaceType             [4]uint8   // Driving surface, see appendices
}

type PacketCarTelemetryData struct {
	Header                       PacketHeader
	CarTelemetryData             [22]CarTelemetryData
	MfdPanelIndex                uint8 // See MFD Panel Index
	MfdPanelIndexSecondaryPlayer uint8 // See MFD Panel Index, 255 = closed
	SuggestedGear                int8  // Suggested gear for the player (1-8), 0 if no gear suggested
}
