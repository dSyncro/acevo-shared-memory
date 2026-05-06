#ifndef _WRAPPER_HPP
#define _WRAPPER_HPP

#include <cstdint>

namespace ks {

	/** Current operational state of the simulator. */
	typedef int ACEVO_STATUS;
	/** Simulator is not running / no session active */
	#define AC_OFF 0
	/** A replay is currently being played back */
	#define AC_REPLAY 1
	/** Live driving session is active */
	#define AC_LIVE 2
	/** Session is paused */
	#define AC_PAUSE 3

	/** Type of racing session currently loaded. */
	typedef int ACEVO_SESSION_TYPE;
	/** Session type not yet determined */
	#define AC_UNKNOWN -1
	/** Time attack / qualifying session */
	#define AC_TIME_ATTACK 0
	/** Race session */
	#define AC_RACE 1
	/** Hot-stint practice */
	#define AC_HOT_STINT 2
	/** Untimed cruise */
	#define AC_CRUISE 3

	/** Race flag currently shown to the driver. */
	typedef int ACEVO_FLAG_TYPE;
	/** No flag displayed */
	#define AC_NO_FLAG 0
	/** Slow vehicle ahead on track */
	#define AC_WHITE_FLAG 1
	/** Track clear — racing resumed */
	#define AC_GREEN_FLAG 2
	/** Session stopped due to incident or hazard */
	#define AC_RED_FLAG 3
	/** Lapped car must yield to the race leader */
	#define AC_BLUE_FLAG 4
	/** Hazard present — no overtaking */
	#define AC_YELLOW_FLAG 5
	/** Driver disqualified / must pit immediately */
	#define AC_BLACK_FLAG 6
	/** Warning for unsportsmanlike behaviour */
	#define AC_BLACK_WHITE_FLAG 7
	/** Session or race has ended */
	#define AC_CHECKERED_FLAG 8
	/** Mechanical problem — car must pit */
	#define AC_ORANGE_CIRCLE_FLAG 9
	/** Slippery surface ahead on track */
	#define AC_RED_YELLOW_STRIPES_FLAG 10

	/** Where on the circuit the car is currently positioned. */
	typedef int ACEVO_CAR_LOCATION;
	/** Position not yet determined */
	#define ACEVO_UNASSIGNED 0
	/** Car is inside the pit lane */
	#define ACEVO_PITLANE 1
	/** Car is at the pit-lane entry */
	#define ACEVO_PITENTRY 2
	/** Car is at the pit-lane exit */
	#define ACEVO_PITEXIT 3
	/** Car is on the racing circuit */
	#define ACEVO_TRACK 4

	/** Powertrain type of the player car. */
	typedef int ACEVO_ENGINE_TYPE;
	/** Traditional petrol/diesel internal combustion engine */
	#define ACEVO_INTERNAL_COMBUSTION 0
	/** Fully electric powertrain */
	#define ACEVO_ELECTRIC_MOTOR 1

	/** Initial grip conditions at session start. */
	typedef int ACEVO_STARTING_GRIP;
	/** Track grip at minimum */
	#define ACEVO_GREEN 0
	/** Track grip in advanced (fast) stage */
	#define ACEVO_FAST 1
	/** Track conditions starting at optimum grip */
	#define ACEVO_OPTIMUM 2

	#pragma pack(push)
	#pragma pack(4)

	/** Raw physics telemetry updated every simulation step. Contains all low-level vehicle dynamics data. */
	struct SPageFilePhysics {
		/** Incrementing counter — detect new data packets by comparing to previous value */
		int packetId = 0;
		/** Throttle pedal position (0.0 = released, 1.0 = full throttle) */
		float gas = 0;
		/** Brake pedal position (0.0 = released, 1.0 = full brake) */
		float brake = 0;
		/** Remaining fuel in litres */
		float fuel = 0;
		/** Engaged gear: 0 = reverse, 1 = neutral, 2+ = forward gears */
		int gear = 0;
		/** Engine speed in revolutions per minute */
		int rpms = 0;
		/** Normalised steering angle (−1.0 = full left, +1.0 = full right) */
		float steerAngle = 0;
		/** Vehicle speed in km/h */
		float speedKmh = 0;
		/** World-space velocity vector [X, Y, Z] in m/s */
		float velocity[3];
		/** Acceleration in G [lateral X, longitudinal Y, vertical Z] */
		float accG[3];
		/** Tyre slip value per wheel [FL, FR, RL, RR] */
		float wheelSlip[4];
		/** Vertical tyre load in Newtons [FL, FR, RL, RR] */
		float wheelLoad[4];
		/** Tyre inflation pressure in PSI [FL, FR, RL, RR] */
		float wheelsPressure[4];
		/** Wheel rotational speed in rad/s [FL, FR, RL, RR] */
		float wheelAngularSpeed[4];
		/** Tyre wear level (0.0 = new, 1.0 = fully worn) [FL, FR, RL, RR] */
		float tyreWear[4];
		/** Amount of dirt / debris on each tyre surface [FL, FR, RL, RR] */
		float tyreDirtyLevel[4];
		/** Core temperature of each tyre in °C [FL, FR, RL, RR] */
		float tyreCoreTemperature[4];
		/** Wheel camber angle in radians per corner [FL, FR, RL, RR] */
		float camberRAD[4];
		/** Suspension compression travel in metres [FL, FR, RL, RR] */
		float suspensionTravel[4];
		/** DRS flap state (0.0 = closed, 1.0 = fully open) */
		float drs = 0;
		/** Traction control cut intensity (0.0 = inactive, 1.0 = maximum) */
		float tc = 0;
		/** Vehicle heading relative to world north in radians */
		float heading = 0;
		/** Chassis pitch angle in radians (positive = nose up) */
		float pitch = 0;
		/** Chassis roll angle in radians (positive = right side down) */
		float roll = 0;
		/** Height of the centre of gravity above the ground in metres */
		float cgHeight;
		/** Damage level per body zone [front, rear, left, right, centre] (0.0–1.0) */
		float carDamage[5];
		/** Number of tyres currently outside track limits */
		int numberOfTyresOut = 0;
		/** Pit-speed limiter active (0 = off, 1 = on) */
		int pitLimiterOn = 0;
		/** ABS intervention intensity (0.0 = inactive, 1.0 = fully active) */
		float abs = 0;
		/** KERS/ERS battery state of charge (0.0–1.0) */
		float kersCharge = 0;
		/** KERS/ERS power delivery level currently being deployed (0.0–1.0) */
		float kersInput = 0;
		/** Automatic gearshift aid active (0 = manual, 1 = auto) */
		int autoShifterOn = 0;
		/** Ride height at front and rear axle in metres [front, rear] */
		float rideHeight[2];
		/** Current turbo boost pressure in bar */
		float turboBoost = 0;
		/** Additional ballast added to the car in kg */
		float ballast = 0;
		/** Ambient air density in kg/m³ */
		float airDensity = 0;
		/** Ambient air temperature in °C */
		float airTemp = 0;
		/** Road surface temperature in °C */
		float roadTemp = 0;
		/** Angular velocity in the car's local frame [pitch, yaw, roll] in rad/s */
		float localAngularVel[3];
		/** Final force-feedback torque value sent to the wheel (Nm) */
		float finalFF = 0;
		/** Real-time delta vs. best lap (positive = ahead of reference) */
		float performanceMeter = 0;

		/** Engine-braking setting level (higher = more engine braking) */
		int engineBrake = 0;
		/** ERS energy-recovery intensity level */
		int ersRecoveryLevel = 0;
		/** ERS power-deployment level */
		int ersPowerLevel = 0;
		/** ERS heat-charging mode active (0 = off, 1 = on) */
		int ersHeatCharging = 0;
		/** ERS currently recovering energy (0 = deploying, 1 = charging) */
		int ersIsCharging = 0;
		/** Energy stored in the KERS/ERS battery in kilojoules */
		float kersCurrentKJ = 0;

		/** DRS can be activated (0 = no, 1 = yes) */
		int drsAvailable = 0;
		/** DRS is open and active (0 = closed, 1 = open) */
		int drsEnabled = 0;

		/** Brake disc temperature per corner in °C [FL, FR, RL, RR] */
		float brakeTemp[4];
		/** Clutch pedal position (0.0 = engaged, 1.0 = fully disengaged) */
		float clutch = 0;

		/** Tyre inner-edge temperature per wheel in °C [FL, FR, RL, RR] */
		float tyreTempI[4];
		/** Tyre mid-tread temperature per wheel in °C [FL, FR, RL, RR] */
		float tyreTempM[4];
		/** Tyre outer-edge temperature per wheel in °C [FL, FR, RL, RR] */
		float tyreTempO[4];

		/** Car is driven by AI (0 = player, 1 = AI) */
		int isAIControlled;

		/** 3-D world-space contact point of each tyre with the road [FL, FR, RL, RR] [X, Y, Z] */
		float tyreContactPoint[4][3];
		/** Road-surface normal vector at each tyre contact point [FL, FR, RL, RR] [X, Y, Z] */
		float tyreContactNormal[4][3];
		/** Heading vector at each tyre contact point [FL, FR, RL, RR] [X, Y, Z] */
		float tyreContactHeading[4][3];

		/** Front brake-bias ratio (e.g. 0.56 = 56 % front) */
		float brakeBias = 0;

		/** Velocity in the car's local reference frame [X, Y, Z] in m/s */
		float localVelocity[3];

		/** Remaining Push-to-Pass activations */
		int P2PActivations = 0;
		/** Push-to-Pass status (0 = inactive, 1 = active) */
		int P2PStatus = 0;

		/** Current rev-limiter ceiling in RPM */
		int currentMaxRpm = 0;

		/** Self-aligning tyre torque (Mz) per wheel [FL, FR, RL, RR] in Nm */
		float mz[4];
		/** Longitudinal tyre force (Fx) per wheel [FL, FR, RL, RR] in N */
		float fx[4];
		/** Lateral tyre force (Fy) per wheel [FL, FR, RL, RR] in N */
		float fy[4];
		/** Longitudinal slip ratio per tyre [FL, FR, RL, RR] */
		float slipRatio[4];
		/** Lateral slip angle per tyre in radians [FL, FR, RL, RR] */
		float slipAngle[4];

		/** Traction control currently cutting power (0 = no, 1 = yes) */
		int tcinAction = 0;
		/** ABS currently modulating brakes (0 = no, 1 = yes) */
		int absInAction = 0;
		/** Suspension structural damage per corner (0.0–1.0) [FL, FR, RL, RR] */
		float suspensionDamage[4];
		/** Representative tyre surface temperature per wheel in °C [FL, FR, RL, RR] */
		float tyreTemp[4];
		/** Engine coolant temperature in °C */
		float waterTemp = 0.0f;
		/** Braking torque at each wheel in Nm [FL, FR, RL, RR] */
		float brakeTorque[4];

		/** Front brake-pad compound identifier */
		int frontBrakeCompound = 0;
		/** Rear brake-pad compound identifier */
		int rearBrakeCompound = 0;
		/** Brake-pad remaining life per corner (0.0–1.0) [FL, FR, RL, RR] */
		float padLife[4];
		/** Brake-disc remaining life per corner (0.0–1.0) [FL, FR, RL, RR] */
		float discLife[4];
		/** Ignition switch state (0 = off, 1 = on) */
		int ignitionOn = 0;
		/** Starter motor currently cranking (0 = no, 1 = yes) */
		int starterEngineOn = 0;
		/** Engine is running (0 = stopped, 1 = running) */
		int isEngineRunning = 0;

		/** Vibration intensity transmitted from kerb strikes */
		float kerbVibration = 0.0f;
		/** Vibration intensity caused by tyre slip */
		float slipVibrations = 0.0f;
		/** Vibration intensity from road surface texture */
		float roadVibrations = 0.0f;
		/** Vibration intensity generated by ABS pulsing */
		float absVibrations = 0.0f;
	};

	/** Complete state of a single tyre corner. Embedded four times in SPageFileGraphicEvo (lf, rf, lr, rr). */
	struct SMEvoTyreState {
		/** Combined tyre slip magnitude */
		float slip = 0.f;
		/** Tyre is locked under braking (true = locking) */
		bool lock = 0;
		/** Tyre inflation pressure (PSI) */
		float tyre_pression = 0.f;
		/** Average tyre carcass temperature in °C */
		float tyre_temperature_c = 0.f;
		/** Brake disc temperature in °C */
		float brake_temperature_c = 0.f;
		/** Hydraulic brake pressure applied at this corner */
		float brake_pressure = 0.f;
		/** Inner-edge tyre temperature in °C */
		float tyre_temperature_left = 0.f;
		/** Centre-tread tyre temperature in °C */
		float tyre_temperature_center = 0.f;
		/** Outer-edge tyre temperature in °C */
		float tyre_temperature_right = 0.f;
		/** Name of the compound fitted on the front axle */
		char tyre_compound_front[33];
		/** Name of the compound fitted on the rear axle */
		char tyre_compound_rear[33];

		/** Pressure as a 0–1 fraction of the target range */
		float tyre_normalized_pressure = 0.f;
		/** Inner-edge temperature as a 0–1 fraction of optimal range */
		float tyre_normalized_temperature_left = 0.f;
		/** Centre temperature as a 0–1 fraction of optimal range */
		float tyre_normalized_temperature_center = 0.f;
		/** Outer-edge temperature as a 0–1 fraction of optimal range */
		float tyre_normalized_temperature_right = 0.f;
		/** Brake temperature as a 0–1 fraction of optimal operating range */
		float brake_normalized_temperature = 0.f;
		/** Core tyre temperature as a 0–1 fraction of optimal range */
		float tyre_normalized_temperature_core = 0.f;

		char place_holder[128];
	};
	static_assert(sizeof(SMEvoTyreState) == 256, "SMEvoTyreState must be 256 bytes");

	/** Structural damage level for each body zone of the car (0.0 = undamaged, 1.0 = destroyed). */
	struct SMEvoDamageState {
		/** Damage on the front body / nose */
		float damage_front = 0.f;
		/** Damage on the rear body / diffuser */
		float damage_rear = 0.f;
		/** Damage on the left side of the body */
		float damage_left = 0.f;
		/** Damage on the right side of the body */
		float damage_right = 0.f;
		/** Damage on the central / underfloor area */
		float damage_center = 0.f;
		/** Damage on the front-left suspension */
		float damage_suspension_lf = 0.f;
		/** Damage on the front-right suspension */
		float damage_suspension_rf = 0.f;
		/** Damage on the rear-left suspension */
		float damage_suspension_lr = 0.f;
		/** Damage on the rear-right suspension */
		float damage_suspension_rr = 0.f;
		char place_holder[92];
	};
	static_assert(sizeof(SMEvoDamageState) == 128, "SMEvoDamageState must be 128 bytes");

	/** Status of each pit-stop service action. −1 = will not perform, 0 = completed, 1 = in progress. */
	struct SMEvoPitInfo {
		/** Body-repair action state */
		int8_t damage = 0;
		/** Refuelling action state */
		int8_t fuel = 0;
		/** Front-left tyre change state */
		int8_t tyres_lf = 0;
		/** Front-right tyre change state */
		int8_t tyres_rf = 0;
		/** Rear-left tyre change state */
		int8_t tyres_lr = 0;
		/** Rear-right tyre change state */
		int8_t tyres_rr = 0;
		char place_holder[58];
	};
	static_assert(sizeof(SMEvoPitInfo) == 64, "SMEvoPitInfo must be 64 bytes");

	/** All driver-adjustable electronic aid and setup settings. */
	struct SMEvoElectronics {
		/** Traction-control level (0 = off, higher = more aggressive) */
		int8_t tc_level = 0;
		/** TC throttle-cut aggressiveness level */
		int8_t tc_cut_level = 0;
		/** ABS intervention level (0 = off) */
		int8_t abs_level = 0;
		/** Electronic stability-control level (0 = off) */
		int8_t esc_level = 0;
		/** Electronic brake-balance adjustment level */
		int8_t ebb_level = 0;
		/** Front brake-bias ratio (e.g. 0.56 = 56 % front) */
		float brake_bias = 0.0f;
		/** Engine map / power mode selection */
		int8_t engine_map_level = 0;
		/** Turbo wastegate or boost target setting */
		float turbo_level = 0.0f;
		/** ERS power-deployment strategy map */
		int8_t ers_deployment_map = 0;
		/** ERS recharge aggressiveness setting */
		float ers_recharge_map = 0.0f;
		/** ERS heat-based charging is enabled */
		bool is_ers_heat_charging_on = false;
		/** ERS overtake (maximum-deploy) mode is active */
		bool is_ers_overtake_mode_on = false;
		/** DRS flap is currently open */
		bool is_drs_open = false;
		/** Differential lock level under power */
		int8_t diff_power_level = 0;
		/** Differential lock level on lift / coast */
		int8_t diff_coast_level = 0;
		/** Front bump (compression) damper stiffness level */
		int8_t front_bump_damper_level = 0;
		/** Front rebound damper stiffness level */
		int8_t front_rebound_damper_level = 0;
		/** Rear bump (compression) damper stiffness level */
		int8_t rear_bump_damper_level = 0;
		/** Rear rebound damper stiffness level */
		int8_t rear_rebound_damper_level = 0;
		/** Ignition switch is on */
		bool is_ignition_on = false;
		/** Pit-speed limiter is active */
		bool is_pitlimiter_on = false;
		/** Selected vehicle performance / power mode index */
		int8_t active_performance_mode = 0;

		char place_holder[88];
	};
	static_assert(sizeof(SMEvoElectronics) == 128, "SMEvoElectronics must be 128 bytes");

	/** Cockpit light, display, and instrumentation panel states. */
	struct SMEvoInstrumentation {
		/** Main exterior light stage (0 = off) */
		int8_t main_light_stage = 0;
		/** Auxiliary / special lights level */
		int8_t special_light_stage = 0;
		/** Interior cockpit illumination level */
		int8_t cockpit_light_stage = 0;

		/** Windscreen wiper speed (0 = off) */
		int8_t wiper_level = 0;
		/** Rear rain light is on */
		bool rain_lights = false;
		/** Left turn indicator is active */
		bool direction_light_left = false;
		/** Right turn indicator is active */
		bool direction_light_right = false;
		/** Flashing lights are active */
		bool flashing_lights = false;
		/** Hazard lights are illuminated */
		bool warning_lights = false;

		/** Index of the currently focused display device */
		int8_t selected_display_index = 0;

		/** Active page index on displays */
		int8_t display_current_page_index[16];

		/** Headlights are on and visible to other drivers */
		bool are_headlights_visible = false;

		char place_holder[101];
	};
	static_assert(sizeof(SMEvoInstrumentation) == 128, "SMEvoInstrumentation must be 128 bytes");

	/** Server-side session lifecycle information. */
	struct SMEvoSessionState {
		/** Name of the current session phase (e.g. 'Race', 'Qualify') */
		char phase_name[33];

		/** Formatted remaining session time (HH:MM:SS) */
		char time_left[15];
		/** Remaining session time in milliseconds */
		int32_t time_left_ms = 0;
		/** Formatted wait time before session start */
		char wait_time[15];
		/** Total laps scheduled for this session */
		int32_t total_lap = 0;
		/** Current lap number being driven */
		int32_t current_lap = 0;
		/** Number of starting lights currently illuminated */
		int32_t lights_on = 0;
		/** Starting-light sequence mode identifier */
		int32_t lights_mode = 0;
		/** Track lap length in kilometres */
		float lap_length_km = 0.f;

		/** Non-zero when the session is ending */
		int32_t end_session_flag = 0;

		/** Formatted countdown to the next session */
		char time_to_next_session[15];
		/** Player has lost connection to the game server */
		bool disconnected_from_server = false;
		/** Season restart option is available to the player */
		bool restart_season_enabled = false;

		/** Drive button is enabled in the UI */
		bool ui_enable_drive = false;
		/** Setup screen is accessible from the UI */
		bool ui_enable_setup = false;

		/** Ready-to-proceed indicator is blinking */
		bool is_ready_to_next_blinking = false;
		/** Waiting-for-players lobby screen is shown */
		bool show_waiting_for_players = false;

		char place_holder[140];
	};
	static_assert(sizeof(SMEvoSessionState) == 256, "SMEvoSessionState must be 256 bytes");

	/** Lap timing and delta values displayed on the HUD. */
	struct SMEvoTimingState {
		/** Current lap time as a formatted string */
		char current_laptime[15];
		/** Delta vs. current reference lap (formatted) */
		char delta_current[15];
		/** Sign of delta_current: +1 slower, −1 faster, 0 = hidden */
		int32_t delta_current_p = 0;
		/** Last completed lap time as a formatted string */
		char last_laptime[15];
		/** Delta vs. last lap (formatted) */
		char delta_last[15];
		/** Sign of delta_last: +1 slower, −1 faster, 0 = hidden */
		int32_t delta_last_p = 0;
		/** Personal best lap time as a formatted string */
		char best_laptime[15];
		/** Theoretical best lap (sum of best sectors) as a formatted string */
		char ideal_laptime[15];
		/** Total elapsed session time as a formatted string */
		char total_time[15];
		/** Current lap has been invalidated (track-limits violation, etc.) */
		bool is_invalid = false;

		char place_holder[137];
	};
	static_assert(sizeof(SMEvoTimingState) == 256, "SMEvoTimingState must be 256 bytes");

	/** Driver-assist settings currently active for the player car. */
	struct SMEvoAssistsState {
		/** Automatic gearshift aid level (0 = off) */
		uint8_t auto_gear = 0;
		/** Automatic throttle blip on downshift (0 = off) */
		uint8_t auto_blip = 0;
		/** Automatic clutch management (0 = off) */
		uint8_t auto_clutch = 0;
		/** Automatic clutch during the rolling start (0 = off) */
		uint8_t auto_clutch_on_start = 0;
		/** Manual ignition and electric start required (0 = automatic) */
		uint8_t manual_ignition_e_start = 0;
		/** Pit-speed limiter activates automatically (0 = manual) */
		uint8_t auto_pit_limiter = 0;
		/** Standing-start launch assistance active (0 = off) */
		uint8_t standing_start_assist = 0;

		/** Auto-steer correction strength (0.0 = off, 1.0 = maximum) */
		float auto_steer = 0.f;
		/** Arcade-style stability aid level (0.0 = off, 1.0 = maximum) */
		float arcade_stability_control = 0.f;

		char place_holder[48];
	};
	static_assert(sizeof(SMEvoAssistsState) == 64, "SMEvoAssistsState must be 64 bytes");

	/** Main HUD and graphics telemetry page. Updated each rendered frame. Contains embedded sub-structs for tyres, damage, electronics, timing, and session state. */
	struct SPageFileGraphicEvo {
		/** Incrementing counter — detect new frames by comparing to previous value */
		int packetId = 0;
		/** Current simulator operational state (see ACEVO_STATUS) */
		ACEVO_STATUS status = AC_OFF;

		/** Unique ID of the car currently shown by the camera */
		uint64_t focused_car_id_a = 0;
		uint64_t focused_car_id_b = 0;

		/** Unique ID of the player's own car */
		uint64_t player_car_id_a = 0;
		uint64_t player_car_id_b = 0;

		/** Engine speed in RPM for HUD display */
		unsigned short rpm = 0;

		/** Rev limiter is cutting fuel / ignition (bouncing off limiter) */
		bool is_rpm_limiter_on = false;
		/** Engine RPM is in the upshift window */
		bool is_change_up_rpm = false;
		/** Engine RPM is in the downshift window */
		bool is_change_down_rpm = false;
		/** Traction control is actively intervening this frame */
		bool tc_active = false;
		/** ABS is actively modulating brake pressure this frame */
		bool abs_active = false;
		/** Electronic stability control is intervening this frame */
		bool esc_active = false;
		/** Launch control system is engaged */
		bool launch_active = false;
		/** Ignition switch is on */
		bool is_ignition_on = false;
		/** Engine is running */
		bool is_engine_running = false;
		/** KERS/ERS battery is currently being charged */
		bool kers_is_charging = false;
		/** Car is travelling in the wrong direction on track */
		bool is_wrong_way = false;
		/** DRS activation is permitted in this section */
		bool is_drs_available = false;
		/** High-voltage battery pack is in charging state */
		bool battery_is_charging = false;
		/** Maximum ERS deployment energy for this lap has been consumed */
		bool is_max_kj_per_lap_reached = false;
		/** Maximum ERS charge energy for this lap has been stored */
		bool is_max_charge_kj_per_lap_reached = false;

		/** Displayed speed in km/h */
		short display_speed_kmh = 0;
		/** Displayed speed in mph */
		short display_speed_mph = 0;
		/** Displayed speed in m/s */
		short display_speed_ms = 0;

		/** Speed delta vs. pit-lane limit (negative = under limit) */
		float pitspeeding_delta = 0.f;
		/** Current gear as an integer (same encoding as physics gear) */
		short gear_int = 0;

		/** Engine RPM as a fraction of redline (0.0–1.0) */
		float rpm_percent = 0.f;
		/** Throttle pedal position as a fraction (0.0–1.0) */
		float gas_percent = 0.f;
		/** Brake pressure as a fraction (0.0–1.0) */
		float brake_percent = 0.f;
		/** Handbrake engagement as a fraction (0.0–1.0) */
		float handbrake_percent = 0.f;
		/** Clutch disengagement as a fraction (1.0–0.0) */
		float clutch_percent = 0.f;
		/** Steering wheel position (−1.0 = full left, +1.0 = full right) */
		float steering_percent = 0.f;

		/** Global force-feedback output strength */
		float ffb_strength = 0.f;
		/** Per-car force-feedback gain multiplier */
		float car_ffb_mupliplier = 0.f;

		/** Coolant temperature as a fraction of optimal operating range */
		float water_temperature_percent = 0.f;

		/** Coolant system pressure in bar */
		float water_pressure_bar = 0.f;
		/** Fuel system pressure in bar */
		float fuel_pressure_bar = 0.f;

		/** Coolant temperature in °C */
		int8_t water_temperature_c = 0;
		/** Ambient air temperature in °C */
		int8_t air_temperature_c = 0;
		/** Engine oil temperature in °C */
		float oil_temperature_c = 0.f;
		/** Engine oil pressure in bar */
		float oil_pressure_bar = 0.f;
		/** Exhaust gas temperature in °C */
		float exhaust_temperature_c = 0.f;

		/** Lateral G-force (positive = rightward) */
		float g_forces_x = 0.f;
		/** Longitudinal G-force (positive = under acceleration) */
		float g_forces_y = 0.f;
		/** Vertical G-force (positive = upward) */
		float g_forces_z = 0.f;

		/** Absolute turbo boost pressure in bar */
		float turbo_boost = 0.f;
		/** Current boost stage or map level */
		float turbo_boost_level = 0.f;
		/** Turbo boost as a fraction of maximum (0.0–1.0) */
		float turbo_boost_perc = 0.f;

		/** Steering wheel rotation in degrees from centre */
		int32_t steer_degrees = 0;
		/** Distance driven in the current session in km */
		float current_km = 0.f;
		/** Total odometer / career distance in km */
		uint32_t total_km = 0;
		/** Total driving time accumulated in seconds */
		uint32_t total_driving_time_s = 0;

		/** In-game time of day — hours (0–23) */
		int32_t time_of_day_hours = 0;
		/** In-game time of day — minutes (0–59) */
		int32_t time_of_day_minutes = 0;
		/** In-game time of day — seconds (0–59) */
		int32_t time_of_day_seconds = 0;

		/** Delta vs. reference lap in milliseconds (signed) */
		int32_t delta_time_ms = 0;
		/** Current lap time in milliseconds */
		int32_t current_lap_time_ms = 0;
		/** Predicted final lap time in milliseconds */
		int32_t predicted_lap_time_ms = 0;

		/** Fuel remaining in the tank in litres */
		float fuel_liter_current_quantity = 0.f;
		/** Fuel remaining as a fraction of tank capacity */
		float fuel_liter_current_quantity_percent = 0.f;
		/** Average fuel consumption rate in litres per km */
		float fuel_liter_per_km = 0.f;
		/** Average fuel economy in km per litre */
		float km_per_fuel_liter = 0.f;

		/** Engine output torque in Nm */
		float current_torque = 0.f;
		/** Engine output power in brake horsepower */
		int32_t current_bhp = 0;

		/** Full tyre state for the front-left corner */
		SMEvoTyreState tyre_lf{};
		/** Full tyre state for the front-right corner */
		SMEvoTyreState tyre_rf{};
		/** Full tyre state for the rear-left corner */
		SMEvoTyreState tyre_lr{};
		/** Full tyre state for the rear-right corner */
		SMEvoTyreState tyre_rr{};

		/** Normalised track position (0.0 = start/finish line, 1.0 = one full lap) */
		float npos = 0.f;

		/** KERS/ERS charge level as a fraction (0.0–1.0) */
		float kers_charge_perc = 0.f;
		/** KERS/ERS power currently being deployed as a fraction */
		float kers_current_perc = 0.f;

		/** Seconds driver input remains locked (e.g. after collision penalty) */
		float control_lock_time = 0.f;

		/** Damage levels for each body zone of the car */
		SMEvoDamageState car_damage{};

		/** Current track zone the car occupies (see ACEVO_CAR_LOCATION) */
		ACEVO_CAR_LOCATION car_location = 0;

		/** Status of each pit-stop service item */
		SMEvoPitInfo pit_info{};

		/** Fuel consumed since session start in litres */
		float fuel_liter_used = 0.f;
		/** Average fuel consumed per lap in litres */
		float fuel_liter_per_lap = 0.f;
		/** Estimated number of laps achievable with remaining fuel */
		float laps_possible_with_fuel = 0.f;

		/** High-voltage battery temperature in °C */
		float battery_temperature = 0.f;
		/** High-voltage battery pack voltage in V */
		float battery_voltage = 0.f;

		/** Instantaneous fuel consumption in litres per km */
		float instantaneous_fuel_liter_per_km = 0.f;
		/** Instantaneous fuel economy in km per litre */
		float instantaneous_km_per_fuel_liter = 0.f;

		/** How well current RPM suits the engaged gear (1.0 = ideal window) */
		float gear_rpm_window = 0.f;

		/** Current state of all cockpit lights and displays */
		SMEvoInstrumentation instrumentation{};
		/** Minimum allowed setting for each instrumentation item */
		SMEvoInstrumentation instrumentation_min_limit{};
		/** Maximum allowed setting for each instrumentation item */
		SMEvoInstrumentation instrumentation_max_limit{};

		/** Current electronic aid and setup values */
		SMEvoElectronics electronics{};
		/** Minimum allowed value for each electronics setting */
		SMEvoElectronics electronics_min_limit{};
		/** Maximum allowed value for each electronics setting */
		SMEvoElectronics electronics_max_limit{};
		/** Flags which electronics fields the driver can adjust in-session */
		SMEvoElectronics electronics_is_modifiable{};

		/** Total laps completed in the session */
		int32_t total_lap_count = 0;
		/** Current race position (1 = leader) */
		uint32_t current_pos = 0;
		/** Total number of cars in the session */
		uint32_t total_drivers = 0;

		/** Last completed lap time in milliseconds */
		int32_t last_laptime_ms = 0;
		/** Personal best lap time in milliseconds */
		int32_t best_laptime_ms = 0;

		/** Flag shown specifically to this driver */
		ACEVO_FLAG_TYPE flag = 0;
		/** Flag shown to all drivers on track */
		ACEVO_FLAG_TYPE global_flag = 0;

		/** Number of forward gears the car has */
		uint32_t max_gears = 0;
		/** Powertrain type of the car (see ACEVO_ENGINE_TYPE) */
		ACEVO_ENGINE_TYPE engine_type = 0;
		/** Car is equipped with a KERS/ERS system */
		bool has_kers = false;
		/** This is the final scheduled lap of the race */
		bool is_last_lap = false;

		/** Display name of the active vehicle performance / power mode */
		char performance_mode_name[33];
		/** Raw differential coast-lock value from setup */
		float diff_coast_raw_value = 0.f;
		/** Raw differential power-lock value from setup */
		float diff_power_raw_value = 0.f;

		/** Cumulative time penalty from track-limit cuts in ms */
		int32_t race_cut_gained_time_ms = 0;
		/** Distance to the penalty trigger in metres */
		int32_t distance_to_deadline = 0;
		/** Running delta time accrued from track-limit violations */
		float race_cut_current_delta = 0.f;

		/** Session lifecycle and countdown information */
		SMEvoSessionState session_state{};
		/** HUD lap times and delta display values */
		SMEvoTimingState timing_state{};

		/** Network round-trip ping to the server in ms */
		int32_t player_ping = 0;
		/** Measured network latency in ms */
		int32_t player_latency = 0;
		/** Client CPU usage in percent */
		int32_t player_cpu_usage = 0;
		/** Average client CPU usage in percent */
		int32_t player_cpu_usage_avg = 0;
		/** Network Quality-of-Service score */
		int32_t player_qos = 0;
		/** Average QoS score over the session */
		int32_t player_qos_avg = 0;
		/** Current rendered frames per second */
		int32_t player_fps = 0;
		/** Average FPS over the session */
		int32_t player_fps_avg = 0;

		/** Driver's first name */
		char driver_name[33];
		/** Driver's surname */
		char driver_surname[33];
		/** Identifier or display name of the car model */
		char car_model[33];

		/** Car is stationary inside its assigned pit box */
		bool is_in_pit_box = 0;
		/** Car is anywhere within the pit lane */
		bool is_in_pit_lane = 0;
		/** Current lap is valid and counts for timing */
		bool is_valid_lap = false;

		/** World-space position of up to 60 cars \[car_index\]\[X, Y, Z\] */
		float car_coordinates[60][3];

		/** Time gap to the car immediately ahead in seconds */
		float gap_ahead = 0;
		/** Time gap to the car immediately behind in seconds */
		float gap_behind = 0;

		/** Number of cars actively participating in the session */
		uint8_t active_cars = 0;
		/** Target fuel consumption per lap in litres */
		float fuel_per_lap = 0;
		/** Estimated laps remaining with current fuel */
		float fuel_estimated_laps = 0;

		/** All driver-assist levels currently active */
		SMEvoAssistsState assists_state{};

		/** Maximum fuel tank capacity of the car in litres */
		float max_fuel = 0;
		/** Maximum turbo boost pressure in bar */
		float max_turbo_boost = 0;
		/** Car is restricted to a single tyre compound for both axles */
		bool use_single_compound = false;

		/** Car UID mapping for indexing car_coordinates */
		uint64_t car_ids[60][2];
	};

	/** Static session metadata. Written once when a session loads and does not change while driving. */
	struct SPageFileStaticEvo {
		/** Shared-memory interface version string */
		char sm_version[15];
		/** AC Evo game build version string */
		char ac_evo_version[15];

		/** Type of the current session (see ACEVO_SESSION_TYPE) */
		ACEVO_SESSION_TYPE session = -1;

		/** Human-readable session name (e.g. 'Race 1') */
		char session_name[33];
		/** Unique identifier of the event within the championship */
		uint8_t event_id = 0;
		/** Unique identifier of this session within the event */
		uint8_t session_id = 0;

		/** Tyre grip condition at session start (see ACEVO_STARTING_GRIP) */
		ACEVO_STARTING_GRIP starting_grip = 0;
		/** Ambient air temperature at session start in °C */
		float starting_ambient_temperature_c = 0.f;
		/** Road surface temperature at session start in °C */
		float starting_ground_temperature_c = 0.f;

		/** Weather is fixed and will not change during the session */
		bool is_static_weather = false;
		/** Session ends by elapsed time rather than lap count */
		bool is_timed_race = 0;
		/** Session is an online multiplayer event */
		bool is_online = 0;
		/** Total sessions in this event (e.g. 3 = practice + qualify + race) */
		int number_of_sessions = 0;

		/** Country / nation name associated with the event or track */
		char nation[33];
		/** Geographic longitude of the track location in decimal degrees */
		float longitude = 0.f;
		/** Geographic latitude of the track location in decimal degrees */
		float latitude = 0.f;

		/** Track identifier or name */
		char track[33];
		/** Track layout variant or configuration name */
		char track_configuration[33];
		/** Total lap length of the track in metres */
		float track_length_m = 0;
	};

	#pragma pack(pop)

}

#endif
