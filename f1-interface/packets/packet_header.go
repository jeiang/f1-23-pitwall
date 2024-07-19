package packets

const (
	// Packet ID
	PACKET_ID_MOTION               = 0
	PACKET_ID_SESSION              = 1
	PACKET_ID_LAP_DATA             = 2
	PACKET_ID_EVENT                = 3
	PACKET_ID_PARTICIPANTS         = 4
	PACKET_ID_CAR_SETUPS           = 5
	PACKET_ID_CAR_TELEMETRY        = 6
	PACKET_ID_CAR_STATUS           = 7
	PACKET_ID_FINAL_CLASSIFICATION = 8
	PACKET_ID_LOBBY_INFO           = 9
	PACKET_ID_CAR_DAMAGE           = 10
	PACKET_ID_SESSION_HISTORY      = 11
	PACKET_ID_TYRE_SETS            = 12
	PACKET_ID_MOTION_EX            = 13
)

type GameVersion struct {
	Major uint8
	Minor uint8
}

type PacketHeader struct {
	PacketFormat            uint16 // 2023
	GameYear                uint8  // Game year - last two digits - 23
	Version                 GameVersion
	packetVersion           uint8   // Version of this packet type, all start from 1
	PacketId                uint8   // Identifier for the packet type, see PacketId above
	SessionUID              uint64  // Unique identifier for the session
	SessionTime             float32 // Session timestamp
	FrameIdentifier         uint32  // Identifier for the frame the data was retrieved on
	PlayerCarIndex          uint8   // Index of player's car in the array
	SecondaryPlayerCarIndex uint8   // Index of secondary player's car in the array (used in split-screen)
	// 255 if no second player
}
