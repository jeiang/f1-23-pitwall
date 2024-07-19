package packets

const (
	// Driver
	DRIVER_CARLOS_SAINZ          = 0
	DRIVER_DANIIL_KVYAT          = 1
	DRIVER_DANIEL_RICCIARDO      = 2
	DRIVER_FERNANDO_ALONSO       = 3
	DRIVER_FELIPE_MASSA          = 4
	DRIVER_KIMI_RäIKKöNEN        = 6
	DRIVER_LEWIS_HAMILTON        = 7
	DRIVER_MAX_VERSTAPPEN        = 9
	DRIVER_NICO_HULKENBURG       = 10
	DRIVER_KEVIN_MAGNUSSEN       = 11
	DRIVER_ROMAIN_GROSJEAN       = 12
	DRIVER_SEBASTIAN_VETTEL      = 13
	DRIVER_SERGIO_PEREZ          = 14
	DRIVER_VALTTERI_BOTTAS       = 15
	DRIVER_ESTEBAN_OCON          = 17
	DRIVER_LANCE_STROLL          = 19
	DRIVER_ARRON_BARNES          = 20
	DRIVER_MARTIN_GILES          = 21
	DRIVER_ALEX_MURRAY           = 22
	DRIVER_LUCAS_ROTH            = 23
	DRIVER_IGOR_CORREIA          = 24
	DRIVER_SOPHIE_LEVASSEUR      = 25
	DRIVER_JONAS_SCHIFFER        = 26
	DRIVER_ALAIN_FOREST          = 27
	DRIVER_JAY_LETOURNEAU        = 28
	DRIVER_ESTO_SAARI            = 29
	DRIVER_YASAR_ATIYEH          = 30
	DRIVER_CALLISTO_CALABRESI    = 31
	DRIVER_NAOTA_IZUM            = 32
	DRIVER_HOWARD_CLARKE         = 33
	DRIVER_WILHEIM_KAUFMANN      = 34
	DRIVER_MARIE_LAURSEN         = 35
	DRIVER_FLAVIO_NIEVES         = 36
	DRIVER_PETER_BELOUSOV        = 37
	DRIVER_KLIMEK_MICHALSKI      = 38
	DRIVER_SANTIAGO_MORENO       = 39
	DRIVER_BENJAMIN_COPPENS      = 40
	DRIVER_NOAH_VISSER           = 41
	DRIVER_GERT_WALDMULLER       = 42
	DRIVER_JULIAN_QUESADA        = 43
	DRIVER_DANIEL_JONES          = 44
	DRIVER_ARTEM_MARKELOV        = 45
	DRIVER_TADASUKE_MAKINO       = 46
	DRIVER_SEAN_GELAEL           = 47
	DRIVER_NYCK_DE_VRIES         = 48
	DRIVER_JACK_AITKEN           = 49
	DRIVER_GEORGE_RUSSELL        = 50
	DRIVER_MAXIMILIAN_GüNTHER    = 51
	DRIVER_NIREI_FUKUZUMI        = 52
	DRIVER_LUCA_GHIOTTO          = 53
	DRIVER_LANDO_NORRIS          = 54
	DRIVER_SERGIO_SETTE_CâMARA   = 55
	DRIVER_LOUIS_DELéTRAZ_       = 56
	DRIVER_ANTONIO_FUOCO_        = 57
	DRIVER_CHARLES_LECLERC_      = 58
	DRIVER_PIERRE_GASLY          = 59
	DRIVER_ALEXANDER_ALBON_      = 62
	DRIVER_NICHOLAS_LATIFI       = 63
	DRIVER_DORIAN_BOCCOLACCI_    = 64
	DRIVER_NIKO_KARI             = 65
	DRIVER_ROBERTO_MERHI_        = 66
	DRIVER_ARJUN_MAINI           = 67
	DRIVER_ALESSIO_LORANDI_      = 68
	DRIVER_RUBEN_MEIJER_         = 69
	DRIVER_RASHID_NAIR           = 70
	DRIVER_JACK_TREMBLAY         = 71
	DRIVER_DEVON_BUTLER          = 72
	DRIVER_LUKAS_WEBER           = 73
	DRIVER_ANTONIO_GIOVINAZZI_   = 74
	DRIVER_ROBERT_KUBICA         = 75
	DRIVER_ALAIN_PROST           = 76
	DRIVER_AYRTON_SENNA          = 77
	DRIVER_NOBUHARU_MATSUSHITA_  = 78
	DRIVER_NIKITA_MAZEPIN        = 79
	DRIVER_GUANYA_ZHOU           = 80
	DRIVER_MICK_SCHUMACHER       = 81
	DRIVER_CALLUM_ILOTT          = 82
	DRIVER_JUAN_MANUEL_CORREA    = 83
	DRIVER_JORDAN_KING           = 84
	DRIVER_MAHAVEER_RAGHUNATHAN_ = 85
	DRIVER_TATIANA_CALDERON      = 86
	DRIVER_ANTHOINE_HUBERT       = 87
	DRIVER_GUILIANO_ALESI        = 88
	DRIVER_RALPH_BOSCHUNG        = 89
	DRIVER_MICHAEL_SCHUMACHER    = 90
	DRIVER_DAN_TICKTUM           = 91
	DRIVER_MARCUS_ARMSTRONG_     = 92
	DRIVER_CHRISTIAN_LUNDGAARD_  = 93
	DRIVER_YUKI_TSUNODA          = 94
	DRIVER_JEHAN_DARUVALA        = 95
	DRIVER_GULHERME_SAMAIA_      = 96
	DRIVER_PEDRO_PIQUET          = 97
	DRIVER_FELIPE_DRUGOVICH      = 98
	DRIVER_ROBERT_SCHWARTZMAN    = 99
	DRIVER_ROY_NISSANY           = 100
	DRIVER_MARINO_SATO           = 101
	DRIVER_AIDAN_JACKSON         = 102
	DRIVER_CASPER_AKKERMAN       = 103
	DRIVER_JENSON_BUTTON         = 109
	DRIVER_DAVID_COULTHARD       = 110
	DRIVER_NICO_ROSBERG          = 111
	DRIVER_OSCAR_PIASTRI         = 112
	DRIVER_LIAM_LAWSON           = 113
	DRIVER_JURI_VIPS             = 114
	DRIVER_THEO_POURCHAIRE_      = 115
	DRIVER_RICHARD_VERSCHOOR_    = 116
	DRIVER_LIRIM_ZENDELI         = 117
	DRIVER_DAVID_BECKMANN_       = 118
	DRIVER_ALESSIO_DELEDDA_      = 121
	DRIVER_BENT_VISCAAL          = 122
	DRIVER_ENZO_FITTIPALDI       = 123
	DRIVER_MARK_WEBBER           = 125
	DRIVER_JACQUES_VILLENEUVE_   = 126
	DRIVER_CALLIE_MAYER          = 127
	DRIVER_NOAH_BELL             = 128
	DRIVER_JAKE_HUGHES           = 129
	DRIVER_FREDERIK_VESTI        = 130
	DRIVER_OLLI_CALDWELL         = 131
	DRIVER_LOGAN_SARGEANT_       = 132
	DRIVER_CEM_BOLUKBASI_        = 133
	DRIVER_AYUMU_IWASA           = 134
	DRIVER_CLEMENT_NOVALAK_      = 135
	DRIVER_JACK_DOOHAN           = 136
	DRIVER_AMAURY_CORDEEL_       = 137
	DRIVER_DENNIS_HAUGER_        = 138
	DRIVER_CALAN_WILLIAMS_       = 139
	DRIVER_JAMIE_CHADWICK_       = 140
	DRIVER_KAMUI_KOBAYASHI_      = 141
	DRIVER_PASTOR_MALDONADO_     = 142
	DRIVER_MIKA_HAKKINEN_        = 143
	DRIVER_NIGEL_MANSELL         = 144

	// Team
	TEAM_MERCEDES                                = 0
	TEAM_FERRARI                                 = 1
	TEAM_RED_BULL_RACING                         = 2
	TEAM_WILLIAMS                                = 3
	TEAM_ASTON_MARTIN                            = 4
	TEAM_ALPINE                                  = 5
	TEAM_ALPHA_TAURI                             = 6
	TEAM_HAAS                                    = 7
	TEAM_MCLAREN                                 = 8
	TEAM_ALFA_ROMEO                              = 9
	TEAM_MERCEDES_2020                           = 85
	TEAM_FERRARI_2020                            = 86
	TEAM_RED_BULL_2020                           = 87
	TEAM_WILLIAMS_2020                           = 88
	TEAM_RACING_POINT_2020                       = 89
	TEAM_RENAULT_2020                            = 90
	TEAM_ALPHA_TAURI_2020                        = 91
	TEAM_HAAS_2020                               = 92
	TEAM_MCLAREN_2020                            = 93
	TEAM_ALFA_ROMEO_2020                         = 94
	TEAM_ASTON_MARTIN_DB11_V12                   = 95
	TEAM_ASTON_MARTIN_VANTAGE_F1_EDITION         = 96
	TEAM_ASTON_MARTIN_VANTAGE_SAFETY_CAR         = 97
	TEAM_FERRARI_F8_TRIBUTO                      = 98
	TEAM_FERRARI_ROMA                            = 99
	TEAM_MCLAREN_720S                            = 100
	TEAM_MCLAREN_ARTURA                          = 101
	TEAM_MERCEDES_AMG_GT_BLACK_SERIES_SAFETY_CAR = 102
	TEAM_MERCEDES_AMG_GTR_PRO                    = 103
	TEAM_F1_CUSTOM_TEAM                          = 104
	TEAM_PREMA_21                                = 106
	TEAM_UNI_VIRTUOSI_21                         = 107
	TEAM_CARLIN_21                               = 108
	TEAM_HITECH_21                               = 109
	TEAM_ART_GP_21                               = 110
	TEAM_MP_MOTORSPORT_21                        = 111
	TEAM_CHAROUZ_21                              = 112
	TEAM_DAMS_21                                 = 113
	TEAM_CAMPOS_21                               = 114
	TEAM_BWT_21                                  = 115
	TEAM_TRIDENT_21                              = 116
	TEAM_MERCEDES_AMG_GT_BLACK_SERIES            = 117
	TEAM_MERCEDES_22                             = 118
	TEAM_FERRARI_22                              = 119
	TEAM_RED_BULL_RACING_22                      = 120
	TEAM_WILLIAMS_22                             = 121
	TEAM_ASTON_MARTIN_22                         = 122
	TEAM_ALPINE_22                               = 123
	TEAM_ALPHA_TAURI_22                          = 124
	TEAM_HAAS_22                                 = 125
	TEAM_MCLAREN_22                              = 126
	TEAM_ALFA_ROMEO_22                           = 127
	TEAM_KONNERSPORT_22                          = 128
	TEAM_KONNERSPORT                             = 129
	TEAM_PREMA_22                                = 130
	TEAM_VIRTUOSI_22                             = 131
	TEAM_CARLIN_22                               = 132
	TEAM_MP_MOTORSPORT_22                        = 133
	TEAM_CHAROUZ_22                              = 134
	TEAM_DAMS_22                                 = 135
	TEAM_CAMPOS_22                               = 136
	TEAM_VAN_AMERSFOORT_RACING_22                = 137
	TEAM_TRIDENT_22                              = 138
	TEAM_HITECH_22                               = 139
	TEAM_ART_GP_22                               = 140

	// National
	NATIONALITY_AMERICAN       = 1
	NATIONALITY_ARGENTINEAN    = 2
	NATIONALITY_AUSTRALIAN     = 3
	NATIONALITY_AUSTRIAN       = 4
	NATIONALITY_AZERBAIJANI    = 5
	NATIONALITY_BAHRAINI       = 6
	NATIONALITY_BELGIAN        = 7
	NATIONALITY_BOLIVIAN       = 8
	NATIONALITY_BRAZILIAN      = 9
	NATIONALITY_BRITISH        = 10
	NATIONALITY_BULGARIAN      = 11
	NATIONALITY_CAMEROONIAN    = 12
	NATIONALITY_CANADIAN       = 13
	NATIONALITY_CHILEAN        = 14
	NATIONALITY_CHINESE        = 15
	NATIONALITY_COLOMBIAN      = 16
	NATIONALITY_COSTA_RICAN    = 17
	NATIONALITY_CROATIAN       = 18
	NATIONALITY_CYPRIOT        = 19
	NATIONALITY_CZECH          = 20
	NATIONALITY_DANISH         = 21
	NATIONALITY_DUTCH          = 22
	NATIONALITY_ECUADORIAN     = 23
	NATIONALITY_ENGLISH        = 24
	NATIONALITY_EMIRIAN        = 25
	NATIONALITY_ESTONIAN       = 26
	NATIONALITY_FINNISH        = 27
	NATIONALITY_FRENCH         = 28
	NATIONALITY_GERMAN         = 29
	NATIONALITY_GHANAIAN       = 30
	NATIONALITY_GREEK          = 31
	NATIONALITY_GUATEMALAN     = 32
	NATIONALITY_HONDURAN       = 33
	NATIONALITY_HONG_KONGER    = 34
	NATIONALITY_HUNGARIAN      = 35
	NATIONALITY_ICELANDER      = 36
	NATIONALITY_INDIAN         = 37
	NATIONALITY_INDONESIAN     = 38
	NATIONALITY_IRISH          = 39
	NATIONALITY_ISRAELI        = 40
	NATIONALITY_ITALIAN        = 41
	NATIONALITY_JAMAICAN       = 42
	NATIONALITY_JAPANESE       = 43
	NATIONALITY_JORDANIAN      = 44
	NATIONALITY_KUWAITI        = 45
	NATIONALITY_LATVIAN        = 46
	NATIONALITY_LEBANESE       = 47
	NATIONALITY_LITHUANIAN     = 48
	NATIONALITY_LUXEMBOURGER   = 49
	NATIONALITY_MALAYSIAN      = 50
	NATIONALITY_MALTESE        = 51
	NATIONALITY_MEXICAN        = 52
	NATIONALITY_MONEGASQUE     = 53
	NATIONALITY_NEW_ZEALANDER  = 54
	NATIONALITY_NICARAGUAN     = 55
	NATIONALITY_NORTHERN_IRISH = 56
	NATIONALITY_NORWEGIAN      = 57
	NATIONALITY_OMANI          = 58
	NATIONALITY_PAKISTANI      = 59
	NATIONALITY_PANAMANIAN     = 60
	NATIONALITY_PARAGUAYAN     = 61
	NATIONALITY_PERUVIAN       = 62
	NATIONALITY_POLISH         = 63
	NATIONALITY_PORTUGUESE     = 64
	NATIONALITY_QATARI         = 65
	NATIONALITY_ROMANIAN       = 66
	NATIONALITY_RUSSIAN        = 67
	NATIONALITY_SALVADORAN     = 68
	NATIONALITY_SAUDI          = 69
	NATIONALITY_SCOTTISH       = 70
	NATIONALITY_SERBIAN        = 71
	NATIONALITY_SINGAPOREAN    = 72
	NATIONALITY_SLOVAKIAN      = 73
	NATIONALITY_SLOVENIAN      = 74
	NATIONALITY_SOUTH_KOREAN   = 75
	NATIONALITY_SOUTH_AFRICAN  = 76
	NATIONALITY_SPANISH        = 77
	NATIONALITY_SWEDISH        = 78
	NATIONALITY_SWISS          = 79
	NATIONALITY_THAI           = 80
	NATIONALITY_TURKISH        = 81
	NATIONALITY_URUGUAYAN      = 82
	NATIONALITY_UKRAINIAN      = 83
	NATIONALITY_VENEZUELAN     = 84
	NATIONALITY_BARBADIAN      = 85
	NATIONALITY_WELSH          = 86
	NATIONALITY_VIETNAMESE     = 87

	// Platform
	PLATFORM_STEAM       = 1
	PLATFORM_PLAYSTATION = 3
	PLATFORM_XBOX        = 4
	PLATFORM_ORIGIN      = 6
	PLATFORM_UNKOWN      = 255
)

type ParticipantData struct {
	AIControlled    uint8    // Whether the vehicle is AI (1) or Human (0) controlled
	DriverID        uint8    // See Driver enum
	NetworkID       uint8    // Unique driver identifier for network players
	TeamID          uint8    // See Team enum
	MyTeam          uint8    // My team flag – 1 = My Team, 0 = otherwise
	RaceNumber      uint8    // Race number of the car
	Nationality     uint8    // See Nationality enum
	Name            [48]byte // Name of participant in UTF-8 format – null terminated (max 48 bytes, U+2026 for truncation)
	YourTelemetry   uint8    // The player's UDP setting, 0 = restricted, 1 = public
	ShowOnlineNames uint8    // 0 = off, 1 = on
	Platform        uint8    // See platform enum
}

type PacketParticipantsData struct {
	Header        PacketHeader        // Header
	NumActiveCars uint8               // Number of active cars in the data – should match number of cars on HUD
	Participants  [22]ParticipantData // Data for all cars on track
}
