use std::{thread::sleep, time::Duration};

use acevo_shared_memory::{ACEvoFlagType, ACEvoSharedMemoryMapper};

const POLL_INTERVAL: Duration = Duration::from_millis(500);
const RETRY_INTERVAL: Duration = Duration::from_secs(2);

fn main() {
    println!("acevo-shared-memory monitor — press Ctrl+C to exit\n");

    loop {
        match ACEvoSharedMemoryMapper::open() {
            Ok(mapper) => run_session(&mapper),
            Err(e) => {
                eprintln!(
                    "Waiting for AC Evo ({e}) — retrying in {}s…",
                    RETRY_INTERVAL.as_secs()
                );
                sleep(RETRY_INTERVAL);
            }
        }
    }
}

/// Polls telemetry and redraws the dashboard until the mapper becomes unavailable.
fn run_session(mapper: &ACEvoSharedMemoryMapper) {
    loop {
        clear_screen();
        print_dashboard(mapper);
        sleep(POLL_INTERVAL);
    }
}

fn print_dashboard(mapper: &ACEvoSharedMemoryMapper) {
    let physics = mapper.physics();
    let graphics = mapper.graphics();
    let session = mapper.static_data();

    // ── Session ──────────────────────────────────────────────────────────────
    println!("=== AC Evo Telemetry Monitor ===");
    println!();
    println!(
        "  Interface : {}  |  Game : {}",
        session.sm_version(),
        session.ac_evo_version()
    );
    println!(
        "  Track     : {}  ({})",
        session.track(),
        session.track_configuration()
    );
    println!(
        "  Session   : {}  |  Starting grip : {:?}",
        session.session_name(),
        session.starting_grip()
    );

    // ── Status & flags ───────────────────────────────────────────────────────
    println!();
    println!("--- Status --------------------------------------------------");
    println!(
        "  Simulator : {:?}  |  Location : {:?}",
        graphics.status(),
        graphics.car_location()
    );
    println!("  Flag      : {}", format_flag(graphics.flag()));
    if graphics.global_flag() != graphics.flag() {
        println!("  Track flag: {}", format_flag(graphics.global_flag()));
    }

    // ── Driver & car ─────────────────────────────────────────────────────────
    println!();
    println!("--- Driver --------------------------------------------------");
    println!(
        "  Name      : {} {}",
        graphics.driver_name(),
        graphics.driver_surname()
    );
    println!(
        "  Car       : {}  |  Engine : {:?}",
        graphics.car_model(),
        graphics.engine_type()
    );
    println!(
        "  Position  : {}/{}",
        graphics.raw().current_pos,
        graphics.raw().total_drivers
    );

    // ── Dynamics ─────────────────────────────────────────────────────────────
    let p = physics.raw();
    println!();
    println!("--- Dynamics ------------------------------------------------");
    println!(
        "  Speed     : {:6.1} km/h  |  Gear : {:2}  |  RPM : {}",
        p.speedKmh, p.gear, p.rpms
    );
    println!(
        "  Throttle  : {:5.1}%  |  Brake : {:5.1}%  |  Steer : {:+.3} rad",
        p.gas * 100.0,
        p.brake * 100.0,
        p.steerAngle
    );

    // ── Driver aids ──────────────────────────────────────────────────────────
    println!();
    println!("--- Aids ----------------------------------------------------");
    println!(
        "  TC : {}  |  ABS : {}  |  DRS : {}  |  Pit limiter : {}",
        flag_indicator(physics.is_tc_in_action()),
        flag_indicator(physics.is_abs_in_action()),
        flag_indicator(physics.is_drs_enabled()),
        flag_indicator(physics.is_pit_limiter_on()),
    );
    println!(
        "  Ignition : {}  |  Engine : {}  |  AI : {}",
        flag_indicator(physics.is_ignition_on()),
        flag_indicator(physics.is_engine_running()),
        flag_indicator(physics.is_ai_controlled()),
    );

    // ── Tyres ────────────────────────────────────────────────────────────────
    println!();
    println!("--- Tyres  (FL  FR  RL  RR) ---------------------------------");
    println!(
        "  Temp core : {:5.1}  {:5.1}  {:5.1}  {:5.1} °C",
        p.tyreCoreTemperature[0],
        p.tyreCoreTemperature[1],
        p.tyreCoreTemperature[2],
        p.tyreCoreTemperature[3],
    );
    println!(
        "  Wear      : {:5.3}  {:5.3}  {:5.3}  {:5.3}",
        p.tyreWear[0], p.tyreWear[1], p.tyreWear[2], p.tyreWear[3],
    );

    // ── Packet freshness ─────────────────────────────────────────────────────
    println!();
    println!("  packetId  : {}", p.packetId);
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn flag_indicator(active: bool) -> &'static str {
    if active { "ON " } else { "off" }
}

fn format_flag(flag: ACEvoFlagType) -> String {
    match flag {
        ACEvoFlagType::NoFlag => "none".to_string(),
        other => format!("{other:?}"),
    }
}
