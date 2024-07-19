package packets

type CarMotionData struct {
	WorldPositionX     float32 // World space X position (in meters)
	WorldPositionY     float32 // World space Y position (in meters)
	WorldPositionZ     float32 // World space Z position (in meters)
	WorldVelocityX     float32 // Velocity in world space X (in meters per second)
	WorldVelocityY     float32 // Velocity in world space Y (in meters per second)
	WorldVelocityZ     float32 // Velocity in world space Z (in meters per second)
	WorldForwardDirX   int16   // World space forward X direction (normalised)
	WorldForwardDirY   int16   // World space forward Y direction (normalised)
	WorldForwardDirZ   int16   // World space forward Z direction (normalised)
	WorldRightDirX     int16   // World space right X direction (normalised)
	WorldRightDirY     int16   // World space right Y direction (normalised)
	WorldRightDirZ     int16   // World space right Z direction (normalised)
	GForceLateral      float32 // G-Force exerted laterally on car where 1.0 is 1g
	GForceLongitudinal float32 // G-Force exerted longitudinally on car where 1.0 is 1g
	GForceVertical     float32 // G-Force exerted vertically on car where 1.0 is 1g
	Yaw                float32 // Yaw angle in radians
	Pitch              float32 // Pitch angle in radians
	Roll               float32 // Roll angle in radians
}

type PacketMotionData struct {
	Header  PacketHeader      // Header
	CarData [22]CarMotionData // Data for all cars on track
}
