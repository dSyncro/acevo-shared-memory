use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

use acevo_shared_memory::{ACEvoFlagType, ACEvoSharedMemoryMapper, SMEvoTyreState};
use ratatui::{
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
            MouseButton, MouseEventKind,
        },
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::*,
    widgets::*,
};

const POLL_INTERVAL: Duration = Duration::from_millis(100);
const RETRY_INTERVAL: Duration = Duration::from_secs(2);

// ─── Tab ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Timing,
    Session,
    Driver,
    Powertrain,
    Tyres,
    Chassis,
    Aids,
    FuelNet,
}

impl Tab {
    const ALL: &'static [Tab] = &[
        Tab::Timing,
        Tab::Session,
        Tab::Driver,
        Tab::Powertrain,
        Tab::Tyres,
        Tab::Chassis,
        Tab::Aids,
        Tab::FuelNet,
    ];

    fn title(self) -> &'static str {
        match self {
            Tab::Timing => "Timing",
            Tab::Session => "Session",
            Tab::Driver => "Driver",
            Tab::Powertrain => "Powertrain",
            Tab::Tyres => "Tyres",
            Tab::Chassis => "Chassis",
            Tab::Aids => "Aids",
            Tab::FuelNet => "Fuel & Net",
        }
    }

    fn index(self) -> usize {
        Self::ALL.iter().position(|&t| t == self).unwrap_or(0)
    }

    fn next(self) -> Self {
        Self::ALL[(self.index() + 1) % Self::ALL.len()]
    }

    fn prev(self) -> Self {
        Self::ALL[(self.index() + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

// ─── App ─────────────────────────────────────────────────────────────────────

struct App {
    tab: Tab,
    mapper: Option<ACEvoSharedMemoryMapper>,
    last_attempt: Instant,
    quit: bool,
    terminal_area: Rect,
}

impl App {
    fn new() -> Self {
        Self {
            tab: Tab::Timing,
            mapper: None,
            last_attempt: Instant::now() - RETRY_INTERVAL - Duration::from_secs(1),
            quit: false,
            terminal_area: Rect::default(),
        }
    }

    fn try_connect(&mut self) {
        if self.mapper.is_none() && self.last_attempt.elapsed() >= RETRY_INTERVAL {
            self.last_attempt = Instant::now();
            if let Ok(m) = ACEvoSharedMemoryMapper::open() {
                self.mapper = Some(m);
            }
        }
    }

    fn run(&mut self, terminal: &mut Terminal<impl Backend<Error = io::Error>>) -> io::Result<()> {
        while !self.quit {
            self.try_connect();
            terminal.draw(|f| self.render(f))?;
            if event::poll(POLL_INTERVAL)? {
                match event::read()? {
                    Event::Key(key) if key.kind == KeyEventKind::Press => {
                        self.handle_key(key.code);
                    }
                    Event::Mouse(mouse) => self.handle_mouse(mouse.kind, mouse.column, mouse.row),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.quit = true,
            KeyCode::Right | KeyCode::Tab => self.tab = self.tab.next(),
            KeyCode::Left | KeyCode::BackTab => self.tab = self.tab.prev(),
            _ => {}
        }
    }

    fn handle_mouse(&mut self, kind: MouseEventKind, col: u16, row: u16) {
        match kind {
            MouseEventKind::Down(MouseButton::Left) if row < 3 => {
                if let Some(tab) = self.tab_at_column(col) {
                    self.tab = tab;
                }
            }
            MouseEventKind::ScrollDown => self.tab = self.tab.next(),
            MouseEventKind::ScrollUp => self.tab = self.tab.prev(),
            _ => {}
        }
    }

    fn tab_at_column(&self, col: u16) -> Option<Tab> {
        let mut x = self.terminal_area.x + 1;
        for &t in Tab::ALL {
            let w = 1 + t.title().len() as u16 + 1;
            if col >= x && col < x + w {
                return Some(t);
            }
            x += w + 3;
        }
        None
    }

    fn render(&mut self, frame: &mut Frame) {
        self.terminal_area = frame.area();
        let [tabs_area, content_area, status_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        self.render_tabs(frame, tabs_area);

        if let Some(mapper) = &self.mapper {
            match self.tab {
                Tab::Timing => render_timing(frame, content_area, mapper),
                Tab::Session => render_session(frame, content_area, mapper),
                Tab::Driver => render_driver(frame, content_area, mapper),
                Tab::Powertrain => render_powertrain(frame, content_area, mapper),
                Tab::Tyres => render_tyres(frame, content_area, mapper),
                Tab::Chassis => render_chassis(frame, content_area, mapper),
                Tab::Aids => render_aids(frame, content_area, mapper),
                Tab::FuelNet => render_fuel_net(frame, content_area, mapper),
            }
        } else {
            render_connecting(frame, content_area);
        }

        render_statusbar(frame, status_area, self.mapper.is_some());
    }

    fn render_tabs(&mut self, frame: &mut Frame, area: Rect) {
        let titles: Vec<Line> = Tab::ALL.iter().map(|t| Line::from(t.title())).collect();
        let tabs = Tabs::new(titles)
            .block(Block::bordered().title(" AC Evo Dashboard "))
            .select(self.tab.index())
            .highlight_style(Style::default().bold().fg(Color::Yellow));
        frame.render_widget(tabs, area);
    }
}

fn render_statusbar(frame: &mut Frame, area: Rect, connected: bool) {
    let (status_str, status_color) = if connected {
        ("● Connected", Color::Green)
    } else {
        ("○ Waiting for AC Evo…", Color::Yellow)
    };
    let bar = Line::from(vec![
        Span::raw("  ← → / Tab / Scroll: navigate  │  Click tab to jump  │  q / Esc: quit  │  "),
        Span::styled(status_str, Style::default().fg(status_color)),
    ]);
    frame.render_widget(bar, area);
}

fn render_connecting(frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "○  Waiting for AC Evo…",
            Style::default().fg(Color::Yellow).bold(),
        )),
        Line::from(""),
        Line::from("  Make sure the game is running with an active session."),
        Line::from("  The dashboard will connect automatically."),
    ];
    frame.render_widget(
        Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(Block::bordered()),
        area,
    );
}

// ─── Timing Tab ──────────────────────────────────────────────────────────────

fn render_timing(frame: &mut Frame, area: Rect, mapper: &ACEvoSharedMemoryMapper) {
    let g = mapper.graphics();
    let gr = g.raw();
    let ts = &gr.timing_state;
    let ss = &gr.session_state;

    let [left_area, right_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

    let delta_curr = match ts.delta_current_p {
        0 => "-".to_owned(),
        1 => format!("▲ {} (slower)", c_str(&ts.delta_current)),
        _ => format!("▼ {} (faster)", c_str(&ts.delta_current)),
    };
    let delta_last = match ts.delta_last_p {
        0 => "-".to_owned(),
        1 => format!("▲ {} (slower)", c_str(&ts.delta_last)),
        _ => format!("▼ {} (faster)", c_str(&ts.delta_last)),
    };
    let valid_str = if gr.is_valid_lap {
        "Yes".to_owned()
    } else {
        "INVALID".to_owned()
    };

    let lap_data: Vec<(&str, String)> = vec![
        ("Current lap", c_str(&ts.current_laptime).to_owned()),
        ("Delta (ref)", delta_curr),
        ("Last lap", c_str(&ts.last_laptime).to_owned()),
        ("Delta (last)", delta_last),
        ("Best lap", c_str(&ts.best_laptime).to_owned()),
        ("Theoretical", c_str(&ts.ideal_laptime).to_owned()),
        ("Total time", c_str(&ts.total_time).to_owned()),
        ("Predicted", fmt_ms(gr.predicted_lap_time_ms)),
        ("Delta ms", format!("{:+} ms", gr.delta_time_ms)),
        ("Last lap ms", fmt_ms(gr.last_laptime_ms)),
        ("Best lap ms", fmt_ms(gr.best_laptime_ms)),
        ("Valid lap", valid_str),
        ("Is last lap", bool_str(gr.is_last_lap)),
    ];

    let lap_rows: Vec<Row> = lap_data
        .iter()
        .map(|(k, v)| {
            let vs = if *k == "Valid lap" && v == "INVALID" {
                Style::default().fg(Color::Red).bold()
            } else {
                Style::default()
            };
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()).style(vs),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(lap_rows, [Constraint::Length(14), Constraint::Fill(1)])
            .block(Block::bordered().title(" Lap Timing "))
            .column_spacing(2),
        left_area,
    );

    let session_data: Vec<(&str, String)> = vec![
        ("Phase", c_str(&ss.phase_name).to_owned()),
        ("Time left", c_str(&ss.time_left).to_owned()),
        ("Time left ms", format!("{} ms", ss.time_left_ms)),
        ("Wait time", c_str(&ss.wait_time).to_owned()),
        ("Next session", c_str(&ss.time_to_next_session).to_owned()),
        ("Current lap", ss.current_lap.to_string()),
        ("Total laps", ss.total_lap.to_string()),
        ("Laps done", gr.total_lap_count.to_string()),
        (
            "Lights on",
            format!("{} (mode {})", ss.lights_on, ss.lights_mode),
        ),
        ("Lap length", format!("{:.3} km", ss.lap_length_km)),
        ("Session ends", bool_str(ss.end_session_flag != 0)),
        ("Disconnected", bool_str(ss.disconnected_from_server)),
        ("Waiting lobby", bool_str(ss.show_waiting_for_players)),
        ("Ready blink", bool_str(ss.is_ready_to_next_blinking)),
    ];

    let session_rows: Vec<Row> = session_data
        .iter()
        .map(|(k, v)| {
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(session_rows, [Constraint::Length(16), Constraint::Fill(1)])
            .block(Block::bordered().title(" Session State "))
            .column_spacing(2),
        right_area,
    );
}

// ─── Session Tab ─────────────────────────────────────────────────────────────

fn render_session(frame: &mut Frame, area: Rect, mapper: &ACEvoSharedMemoryMapper) {
    let s = mapper.static_data();
    let sr = s.raw();
    let g = mapper.graphics();
    let gr = g.raw();

    let [left_area, right_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

    let track_data: Vec<(&str, String)> = vec![
        ("Track", s.track().to_owned()),
        ("Configuration", s.track_configuration().to_owned()),
        ("Nation", s.nation().to_owned()),
        (
            "Length",
            format!(
                "{:.3} m ({:.3} km)",
                sr.track_length_m,
                sr.track_length_m / 1000.0
            ),
        ),
        ("Latitude", format!("{:.6}°", sr.latitude)),
        ("Longitude", format!("{:.6}°", sr.longitude)),
        ("Session type", format!("{}", s.session())),
        ("Session name", s.session_name().to_owned()),
        ("Event ID", sr.event_id.to_string()),
        ("Session ID", sr.session_id.to_string()),
        ("Is online", bool_str(sr.is_online)),
        ("Is timed race", bool_str(sr.is_timed_race)),
        ("# Sessions", sr.number_of_sessions.to_string()),
        ("Interface ver", s.sm_version().to_owned()),
        ("Game version", s.ac_evo_version().to_owned()),
    ];

    let track_rows: Vec<Row> = track_data
        .iter()
        .map(|(k, v)| {
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(track_rows, [Constraint::Length(16), Constraint::Fill(1)])
            .block(Block::bordered().title(" Track & Session "))
            .column_spacing(2),
        left_area,
    );

    let cond_data: Vec<(&str, String)> = vec![
        ("Starting grip", format!("{}", s.starting_grip())),
        (
            "Start air temp",
            format!("{:.1}°C", sr.starting_ambient_temperature_c),
        ),
        (
            "Start road temp",
            format!("{:.1}°C", sr.starting_ground_temperature_c),
        ),
        ("Static weather", bool_str(sr.is_static_weather)),
        ("", "".to_owned()),
        ("Performance", g.performance_mode_name().to_owned()),
        ("Status", format!("{}", g.status())),
        ("Time of day", {
            let (h, m, s2) = g.time_of_day();
            format!("{h:02}:{m:02}:{s2:02}")
        }),
        ("", "".to_owned()),
        ("Current km", format!("{:.3} km", gr.current_km)),
        ("Total km", format!("{} km", gr.total_km)),
        ("Drive time", fmt_seconds(gr.total_driving_time_s)),
        ("Max gears", gr.max_gears.to_string()),
        ("Engine type", format!("{}", g.engine_type())),
        ("Has KERS", bool_str(gr.has_kers)),
    ];

    let cond_rows: Vec<Row> = cond_data
        .iter()
        .map(|(k, v)| {
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(cond_rows, [Constraint::Length(16), Constraint::Fill(1)])
            .block(Block::bordered().title(" Conditions & Car "))
            .column_spacing(2),
        right_area,
    );
}

// ─── Driver Tab ──────────────────────────────────────────────────────────────

fn render_driver(frame: &mut Frame, area: Rect, mapper: &ACEvoSharedMemoryMapper) {
    let g = mapper.graphics();
    let gr = g.raw();
    let (gx, gy, gz) = g.g_forces();
    let p = mapper.physics();
    let pr = p.raw();

    let [left_area, right_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

    let driver_data: Vec<(&str, String)> = vec![
        (
            "Name",
            format!("{} {}", g.driver_name(), g.driver_surname()),
        ),
        ("Car", g.car_model().to_owned()),
        ("Engine", format!("{}", g.engine_type())),
        ("", "".to_owned()),
        (
            "Position",
            format!("{} / {}", gr.current_pos, gr.total_drivers),
        ),
        ("Active cars", gr.active_cars.to_string()),
        ("Gap ahead", format!("{:.3} s", gr.gap_ahead)),
        ("Gap behind", format!("{:.3} s", gr.gap_behind)),
        ("", "".to_owned()),
        ("Location", format!("{}", g.car_location())),
        ("Track pos", format!("{:.4} (npos)", gr.npos)),
        ("In pit box", bool_str(gr.is_in_pit_box)),
        ("In pit lane", bool_str(gr.is_in_pit_lane)),
        ("Wrong way", bool_str(gr.is_wrong_way)),
        ("", "".to_owned()),
        ("Driver flag", format_flag(g.flag())),
        ("Track flag", format_flag(g.global_flag())),
    ];

    let driver_rows: Vec<Row> = driver_data
        .iter()
        .map(|(k, v)| {
            let vs = if *k == "Wrong way" && v == "Yes" {
                Style::default().fg(Color::Red).bold()
            } else {
                Style::default()
            };
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()).style(vs),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(driver_rows, [Constraint::Length(14), Constraint::Fill(1)])
            .block(Block::bordered().title(" Driver & Position "))
            .column_spacing(2),
        left_area,
    );

    let gforce_data: [(&str, f32); 3] = [("Lateral", gx), ("Longitudinal", gy), ("Vertical", gz)];
    let gforce_rows: Vec<Row> = gforce_data
        .iter()
        .map(|(label, val)| {
            let pct = (val.abs() / 3.0).clamp(0.0, 1.0);
            let bar = filled_bar((pct * 100.0) as u16, 18);
            let color = if pct > 0.7 { Color::Red } else { Color::Cyan };
            Row::new([
                Cell::from(*label).style(Style::default().fg(Color::Cyan)),
                Cell::from(format!("{:+.2}g", val)),
                Cell::from(bar).style(Style::default().fg(color)),
            ])
        })
        .collect();

    let penalty_data: Vec<(&str, String)> = vec![
        (
            "Cut penalty ms",
            format!("{} ms", gr.race_cut_gained_time_ms),
        ),
        ("Cut delta", format!("{:.3}", gr.race_cut_current_delta)),
        ("Dist deadline", format!("{} m", gr.distance_to_deadline)),
        ("Lock time", format!("{:.2} s", gr.control_lock_time)),
        (
            "P2P status",
            if p.is_push_to_pass_active() {
                "Active".to_owned()
            } else {
                "Inactive".to_owned()
            },
        ),
        ("P2P remaining", pr.P2PActivations.to_string()),
    ];

    let [gf_area, pen_area] =
        Layout::vertical([Constraint::Length(7), Constraint::Min(0)]).areas(right_area);

    frame.render_widget(
        Table::new(
            gforce_rows,
            [
                Constraint::Length(13),
                Constraint::Length(8),
                Constraint::Fill(1),
            ],
        )
        .block(Block::bordered().title(" G-Forces "))
        .column_spacing(1),
        gf_area,
    );

    let pen_rows: Vec<Row> = penalty_data
        .iter()
        .map(|(k, v)| {
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(pen_rows, [Constraint::Length(14), Constraint::Fill(1)])
            .block(Block::bordered().title(" Penalties & P2P "))
            .column_spacing(2),
        pen_area,
    );
}

// ─── Powertrain Tab ──────────────────────────────────────────────────────────

fn render_powertrain(frame: &mut Frame, area: Rect, mapper: &ACEvoSharedMemoryMapper) {
    let p = mapper.physics();
    let pr = p.raw();
    let g = mapper.graphics();
    let gr = g.raw();

    let [stats_area, body_area] =
        Layout::vertical([Constraint::Length(5), Constraint::Min(0)]).areas(area);

    // Big stat boxes
    let [speed_area, gear_area, rpm_area] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(2),
    ])
    .areas(stats_area);

    let speed_str = format!("{:.1}", pr.speedKmh);
    let gear_str = match p.actual_gear() {
        -1 => "R".to_owned(),
        0 => "N".to_owned(),
        n => n.to_string(),
    };
    let rpm_str = pr.rpms.to_string();

    frame.render_widget(big_stat(&speed_str, "km/h", " Speed "), speed_area);
    frame.render_widget(big_stat(&gear_str, "gear", " Gear "), gear_area);

    // RPM with progress bar inside box
    let rpm_pct = (gr.rpm_percent.clamp(0.0, 1.0) * 100.0) as u16;
    let rpm_max_str = pr.currentMaxRpm.to_string();
    frame.render_widget(
        Gauge::default()
            .block(Block::bordered().title(" Engine RPM "))
            .gauge_style(Style::default().fg(Color::LightCyan))
            .percent(rpm_pct)
            .label(format!("{rpm_str} / {rpm_max_str}")),
        rpm_area,
    );

    let [inputs_area, right_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(body_area);

    // Input gauges
    let [thr, brk, clt, steer_a, ff_a] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
    ])
    .areas(inputs_area);

    let throttle_pct = (pr.gas.clamp(0.0, 1.0) * 100.0) as u16;
    let brake_pct = (pr.brake.clamp(0.0, 1.0) * 100.0) as u16;
    let clutch_pct = (pr.clutch.clamp(0.0, 1.0) * 100.0) as u16;
    let steer_ratio = ((pr.steerAngle.clamp(-1.0, 1.0) + 1.0) / 2.0) as f64;

    frame.render_widget(
        Gauge::default()
            .block(Block::bordered().title(" Throttle "))
            .gauge_style(Style::default().fg(Color::Green))
            .percent(throttle_pct)
            .label(format!("{:.1}%", pr.gas * 100.0)),
        thr,
    );
    frame.render_widget(
        Gauge::default()
            .block(Block::bordered().title(" Brake "))
            .gauge_style(Style::default().fg(Color::Red))
            .percent(brake_pct)
            .label(format!("{:.1}%", pr.brake * 100.0)),
        brk,
    );
    frame.render_widget(
        Gauge::default()
            .block(Block::bordered().title(" Clutch "))
            .gauge_style(Style::default().fg(Color::Magenta))
            .percent(clutch_pct)
            .label(format!("{:.1}%", pr.clutch * 100.0)),
        clt,
    );
    frame.render_widget(
        LineGauge::default()
            .block(Block::bordered().title(format!(
                " Steering  {}°  {:+.3} rad ",
                gr.steer_degrees, pr.steerAngle
            )))
            .filled_style(Style::default().fg(Color::Yellow))
            .ratio(steer_ratio),
        steer_a,
    );
    frame.render_widget(
        LineGauge::default()
            .block(Block::bordered().title(format!(" Force Feedback  {:.2} ", pr.finalFF)))
            .filled_style(Style::default().fg(Color::DarkGray))
            .ratio(((pr.finalFF.abs()).min(1.0)) as f64),
        ff_a,
    );

    // Right: engine + ERS tables
    let [engine_area, ers_area] =
        Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(right_area);

    let engine_data: Vec<(&str, String)> = vec![
        (
            "Torque / BHP",
            format!("{:.0} Nm  /  {} BHP", gr.current_torque, gr.current_bhp),
        ),
        (
            "Water temp",
            format!(
                "{:.0}°C  ({:.0}%)",
                pr.waterTemp,
                gr.water_temperature_percent * 100.0
            ),
        ),
        (
            "Water pressure",
            format!("{:.2} bar", gr.water_pressure_bar),
        ),
        ("Oil temp", format!("{:.1}°C", gr.oil_temperature_c)),
        ("Oil pressure", format!("{:.2} bar", gr.oil_pressure_bar)),
        ("Exhaust temp", format!("{:.0}°C", gr.exhaust_temperature_c)),
        (
            "Turbo boost",
            format!(
                "{:.3} bar  ({:.1}%  max {:.2})",
                gr.turbo_boost,
                gr.turbo_boost_perc * 100.0,
                gr.max_turbo_boost
            ),
        ),
        ("Fuel pressure", format!("{:.2} bar", gr.fuel_pressure_bar)),
        ("Perf delta", format!("{:+.3} s", pr.performanceMeter)),
        (
            "Air temp",
            format!("{:.1}°C  density {:.4} kg/m³", pr.airTemp, pr.airDensity),
        ),
        ("Road temp", format!("{:.1}°C", pr.roadTemp)),
    ];

    let engine_rows: Vec<Row> = engine_data
        .iter()
        .map(|(k, v)| {
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(engine_rows, [Constraint::Length(16), Constraint::Fill(1)])
            .block(Block::bordered().title(" Engine & Temps "))
            .column_spacing(2),
        engine_area,
    );

    let ers_data: Vec<(&str, String)> = vec![
        (
            "KERS charge",
            format!("{:.1}%", gr.kers_charge_perc * 100.0),
        ),
        (
            "KERS deploy",
            format!("{:.1}%", gr.kers_current_perc * 100.0),
        ),
        ("KERS KJ", format!("{:.2} kJ", pr.kersCurrentKJ)),
        ("KERS input", format!("{:.3}", pr.kersInput)),
        ("Recovery lvl", pr.ersRecoveryLevel.to_string()),
        ("Power lvl", pr.ersPowerLevel.to_string()),
        ("Heat charging", bool_str(pr.ersHeatCharging != 0)),
        ("Battery chg", bool_str(gr.battery_is_charging)),
        ("Battery temp", format!("{:.1}°C", gr.battery_temperature)),
        ("Battery V", format!("{:.1} V", gr.battery_voltage)),
        ("Max KJ/lap", bool_str(gr.is_max_kj_per_lap_reached)),
        ("Engine brake", pr.engineBrake.to_string()),
        ("RPM limiter", bool_str(gr.is_rpm_limiter_on)),
        ("Shift up", bool_str(gr.is_change_up_rpm)),
        ("Shift down", bool_str(gr.is_change_down_rpm)),
    ];

    let ers_rows: Vec<Row> = ers_data
        .iter()
        .map(|(k, v)| {
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(ers_rows, [Constraint::Length(14), Constraint::Fill(1)])
            .block(Block::bordered().title(" ERS / KERS "))
            .column_spacing(2),
        ers_area,
    );
}

// ─── Tyres Tab ───────────────────────────────────────────────────────────────

fn render_tyres(frame: &mut Frame, area: Rect, mapper: &ACEvoSharedMemoryMapper) {
    let physics = mapper.physics();
    let pr = physics.raw();
    let graphics = mapper.graphics();
    let gr = graphics.raw();

    let [top_area, bot_area] =
        Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);
    let [fl_area, fr_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(top_area);
    let [rl_area, rr_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(bot_area);

    render_tyre_corner(
        frame,
        fl_area,
        "FL",
        &gr.tyre_lf,
        pr.tyreWear[0],
        pr.padLife[0],
        pr.discLife[0],
        pr.tyreDirtyLevel[0],
        pr.suspensionTravel[0],
    );
    render_tyre_corner(
        frame,
        fr_area,
        "FR",
        &gr.tyre_rf,
        pr.tyreWear[1],
        pr.padLife[1],
        pr.discLife[1],
        pr.tyreDirtyLevel[1],
        pr.suspensionTravel[1],
    );
    render_tyre_corner(
        frame,
        rl_area,
        "RL",
        &gr.tyre_lr,
        pr.tyreWear[2],
        pr.padLife[2],
        pr.discLife[2],
        pr.tyreDirtyLevel[2],
        pr.suspensionTravel[2],
    );
    render_tyre_corner(
        frame,
        rr_area,
        "RR",
        &gr.tyre_rr,
        pr.tyreWear[3],
        pr.padLife[3],
        pr.discLife[3],
        pr.tyreDirtyLevel[3],
        pr.suspensionTravel[3],
    );
}

fn render_tyre_corner(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    t: &SMEvoTyreState,
    wear: f32,
    pad_life: f32,
    disc_life: f32,
    dirty: f32,
    susp_travel: f32,
) {
    let compound_f = c_str(&t.tyre_compound_front);
    let compound_r = c_str(&t.tyre_compound_rear);
    let lock_str = if t.lock { "LOCKED" } else { "free" };
    let lock_style = if t.lock {
        Style::default().fg(Color::Red).bold()
    } else {
        Style::default()
    };

    let data: Vec<(&str, String, Style)> = vec![
        (
            "Compound F/R",
            format!("{} / {}", compound_f, compound_r),
            Style::default(),
        ),
        (
            "Core temp",
            format!(
                "{:.1}°C  (norm {:.3})",
                t.tyre_temperature_c, t.tyre_normalized_temperature_core
            ),
            Style::default().fg(temp_color(t.tyre_temperature_c)),
        ),
        (
            "In/Ctr/Out °C",
            format!(
                "{:.1} / {:.1} / {:.1}",
                t.tyre_temperature_left, t.tyre_temperature_center, t.tyre_temperature_right
            ),
            Style::default().fg(temp_color(
                (t.tyre_temperature_left + t.tyre_temperature_center + t.tyre_temperature_right)
                    / 3f32,
            )),
        ),
        (
            "Wear",
            format!("{:.4}", wear),
            Style::default().fg(wear_color(wear)),
        ),
        (
            "Pressure PSI",
            format!(
                "{:.2} PSI  (norm {:.3})",
                t.tyre_pression, t.tyre_normalized_pressure
            ),
            Style::default(),
        ),
        (
            "Slip / Lock",
            format!("{:.4}  {lock_str}", t.slip),
            lock_style,
        ),
        (
            "Brake disc",
            format!(
                "{:.1}°C  (norm {:.3})",
                t.brake_temperature_c, t.brake_normalized_temperature
            ),
            Style::default(),
        ),
        (
            "Brake pressure",
            format!("{:.2}", t.brake_pressure),
            Style::default(),
        ),
        (
            "Pad / Disc life",
            format!("{:.3} / {:.3}", pad_life, disc_life),
            Style::default(),
        ),
        ("Dirty level", format!("{:.4}", dirty), Style::default()),
        (
            "Susp travel",
            format!("{:.4} m", susp_travel),
            Style::default(),
        ),
    ];

    let rows: Vec<Row> = data
        .iter()
        .map(|(k, v, vs)| {
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()).style(*vs),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(rows, [Constraint::Length(15), Constraint::Fill(1)])
            .block(Block::bordered().title(format!(" {label} ")))
            .column_spacing(1),
        area,
    );
}

// ─── Chassis Tab ─────────────────────────────────────────────────────────────

fn render_chassis(frame: &mut Frame, area: Rect, mapper: &ACEvoSharedMemoryMapper) {
    let p = mapper.physics();
    let pr = p.raw();
    let g = mapper.graphics();
    let gr = g.raw();
    let dmg = &gr.car_damage;

    let [left_area, right_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

    let [susp_area, orient_area] =
        Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(left_area);

    let susp_data: Vec<(&str, String)> = vec![
        ("Susp travel FL", format!("{:.4} m", pr.suspensionTravel[0])),
        ("Susp travel FR", format!("{:.4} m", pr.suspensionTravel[1])),
        ("Susp travel RL", format!("{:.4} m", pr.suspensionTravel[2])),
        ("Susp travel RR", format!("{:.4} m", pr.suspensionTravel[3])),
        (
            "Ride height F",
            format!(
                "{:.4} m  ({:.1} mm)",
                pr.rideHeight[0],
                pr.rideHeight[0] * 1000.0
            ),
        ),
        (
            "Ride height R",
            format!(
                "{:.4} m  ({:.1} mm)",
                pr.rideHeight[1],
                pr.rideHeight[1] * 1000.0
            ),
        ),
        ("Camber FL", format!("{:.4} rad", pr.camberRAD[0])),
        ("Camber FR", format!("{:.4} rad", pr.camberRAD[1])),
        ("Camber RL", format!("{:.4} rad", pr.camberRAD[2])),
        ("Camber RR", format!("{:.4} rad", pr.camberRAD[3])),
        ("Brake bias", format!("{:.1}%", pr.brakeBias * 100.0)),
        ("Diff power", format!("{:.3}", gr.diff_power_raw_value)),
        ("Diff coast", format!("{:.3}", gr.diff_coast_raw_value)),
        ("CG height", format!("{:.4} m", pr.cgHeight)),
        ("Ballast", format!("{:.1} kg", pr.ballast)),
    ];

    let susp_rows: Vec<Row> = susp_data
        .iter()
        .map(|(k, v)| {
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(susp_rows, [Constraint::Length(15), Constraint::Fill(1)])
            .block(Block::bordered().title(" Suspension & Setup "))
            .column_spacing(2),
        susp_area,
    );

    let orient_data: Vec<(&str, String)> = vec![
        ("Heading", format!("{:.4} rad", pr.heading)),
        ("Pitch", format!("{:.4} rad", pr.pitch)),
        ("Roll", format!("{:.4} rad", pr.roll)),
        ("Ang vel P", format!("{:+.4} rad/s", pr.localAngularVel[0])),
        ("Ang vel Y", format!("{:+.4} rad/s", pr.localAngularVel[1])),
        ("Ang vel R", format!("{:+.4} rad/s", pr.localAngularVel[2])),
        ("Velocity X", format!("{:+.3} m/s", pr.localVelocity[0])),
        ("Velocity Y", format!("{:+.3} m/s", pr.localVelocity[1])),
        ("Velocity Z", format!("{:+.3} m/s", pr.localVelocity[2])),
        ("Wheel load FL", format!("{:.1} N", pr.wheelLoad[0])),
        ("Wheel load FR", format!("{:.1} N", pr.wheelLoad[1])),
        ("Wheel load RL", format!("{:.1} N", pr.wheelLoad[2])),
        ("Wheel load RR", format!("{:.1} N", pr.wheelLoad[3])),
        ("Tyres out", pr.numberOfTyresOut.to_string()),
        ("Gear rpm win", format!("{:.3}", gr.gear_rpm_window)),
    ];

    let orient_rows: Vec<Row> = orient_data
        .iter()
        .map(|(k, v)| {
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(orient_rows, [Constraint::Length(15), Constraint::Fill(1)])
            .block(Block::bordered().title(" Orientation & Loads "))
            .column_spacing(2),
        orient_area,
    );

    // Right: damage
    let dmg_data: Vec<(&str, f32)> = vec![
        ("Body front", dmg.damage_front),
        ("Body rear", dmg.damage_rear),
        ("Body left", dmg.damage_left),
        ("Body right", dmg.damage_right),
        ("Body centre", dmg.damage_center),
        ("Susp FL", dmg.damage_suspension_lf),
        ("Susp FR", dmg.damage_suspension_rf),
        ("Susp RL", dmg.damage_suspension_lr),
        ("Susp RR", dmg.damage_suspension_rr),
    ];

    let dmg_rows: Vec<Row> = dmg_data
        .iter()
        .map(|(k, v)| {
            let pct = (v * 100.0) as u16;
            let bar = filled_bar(pct, 16);
            let color = if *v > 0.5 {
                Color::Red
            } else if *v > 0.2 {
                Color::Yellow
            } else {
                Color::Green
            };
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(format!("{:.3}", v)),
                Cell::from(bar).style(Style::default().fg(color)),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(
            dmg_rows,
            [
                Constraint::Length(12),
                Constraint::Length(6),
                Constraint::Fill(1),
            ],
        )
        .block(Block::bordered().title(" Damage "))
        .column_spacing(1),
        right_area,
    );
}

// ─── Aids Tab ────────────────────────────────────────────────────────────────

fn render_aids(frame: &mut Frame, area: Rect, mapper: &ACEvoSharedMemoryMapper) {
    let p = mapper.physics();
    let pr = p.raw();
    let g = mapper.graphics();
    let gr = g.raw();
    let el = &gr.electronics;
    let ast = &gr.assists_state;

    let [left_area, right_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

    let [active_area, assists_area] =
        Layout::vertical([Constraint::Fill(2), Constraint::Fill(1)]).areas(left_area);

    let active: Vec<(&str, bool)> = vec![
        ("TC in action", p.is_tc_in_action()),
        ("ABS in action", p.is_abs_in_action()),
        ("TC active (HUD)", gr.tc_active),
        ("ABS active (HUD)", gr.abs_active),
        ("ESC active", gr.esc_active),
        ("Launch control", gr.launch_active),
        ("DRS available", p.is_drs_available()),
        ("DRS enabled", p.is_drs_enabled()),
        ("Pit limiter", p.is_pit_limiter_on()),
        ("Auto shifter", p.is_auto_shifter_on()),
        ("ERS charging", p.is_ers_charging()),
        ("Ignition", p.is_ignition_on()),
        ("Starter running", p.starter_engine_on()),
        ("Engine running", p.is_engine_running()),
        ("AI controlled", p.is_ai_controlled()),
        ("Wrong way", gr.is_wrong_way),
        ("In pit box", gr.is_in_pit_box),
        ("In pit lane", gr.is_in_pit_lane),
    ];

    let active_rows: Vec<Row> = active
        .iter()
        .map(|(label, on)| {
            let (ind, style) = if *on {
                ("● ON", Style::default().fg(Color::Green).bold())
            } else {
                ("○ off", Style::default().fg(Color::DarkGray))
            };
            Row::new([
                Cell::from(*label).style(Style::default().fg(Color::Cyan)),
                Cell::from(ind).style(style),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(active_rows, [Constraint::Length(18), Constraint::Fill(1)])
            .block(Block::bordered().title(" Active Aids & Flags "))
            .column_spacing(2),
        active_area,
    );

    let pit = &gr.pit_info;
    let pit_data: Vec<(&str, String)> = vec![
        ("Auto gear", ast.auto_gear.to_string()),
        ("Auto blip", ast.auto_blip.to_string()),
        ("Auto clutch", ast.auto_clutch.to_string()),
        ("Clch on start", ast.auto_clutch_on_start.to_string()),
        ("Auto pit lim", ast.auto_pit_limiter.to_string()),
        ("Standing start", ast.standing_start_assist.to_string()),
        ("Auto steer", format!("{:.3}", ast.auto_steer)),
        (
            "Arcade stab",
            format!("{:.3}", ast.arcade_stability_control),
        ),
        ("Pit: damage", pit_action(pit.damage)),
        ("Pit: fuel", pit_action(pit.fuel)),
        ("Pit: tyre FL", pit_action(pit.tyres_lf)),
        ("Pit: tyre FR", pit_action(pit.tyres_rf)),
        ("Pit: tyre RL", pit_action(pit.tyres_lr)),
        ("Pit: tyre RR", pit_action(pit.tyres_rr)),
    ];

    let assist_rows: Vec<Row> = pit_data
        .iter()
        .map(|(k, v)| {
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(assist_rows, [Constraint::Length(14), Constraint::Fill(1)])
            .block(Block::bordered().title(" Assists & Pit Plan "))
            .column_spacing(2),
        assists_area,
    );

    // Electronics settings
    let elec_data: Vec<(&str, String)> = vec![
        ("TC level", el.tc_level.to_string()),
        ("TC cut level", el.tc_cut_level.to_string()),
        ("ABS level", el.abs_level.to_string()),
        ("ESC level", el.esc_level.to_string()),
        ("EBB level", el.ebb_level.to_string()),
        ("Brake bias", format!("{:.1}%", el.brake_bias * 100.0)),
        ("Engine map", el.engine_map_level.to_string()),
        ("Turbo level", format!("{:.3}", el.turbo_level)),
        ("ERS deploy map", el.ers_deployment_map.to_string()),
        ("ERS recharge", format!("{:.3}", el.ers_recharge_map)),
        ("ERS heat chg", bool_str(el.is_ers_heat_charging_on)),
        ("ERS overtake", bool_str(el.is_ers_overtake_mode_on)),
        ("DRS open", bool_str(el.is_drs_open)),
        ("Diff power", el.diff_power_level.to_string()),
        ("Diff coast", el.diff_coast_level.to_string()),
        ("F bump damp", el.front_bump_damper_level.to_string()),
        ("F rebound damp", el.front_rebound_damper_level.to_string()),
        ("R bump damp", el.rear_bump_damper_level.to_string()),
        ("R rebound damp", el.rear_rebound_damper_level.to_string()),
        ("Perf mode idx", el.active_performance_mode.to_string()),
        ("Ignition", bool_str(el.is_ignition_on)),
        ("Pit limiter", bool_str(el.is_pitlimiter_on)),
    ];

    let elec_rows: Vec<Row> = elec_data
        .iter()
        .map(|(k, v)| {
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(elec_rows, [Constraint::Length(16), Constraint::Fill(1)])
            .block(Block::bordered().title(" Electronics Settings "))
            .column_spacing(2),
        right_area,
    );
}

// ─── Fuel & Net Tab ──────────────────────────────────────────────────────────

fn render_fuel_net(frame: &mut Frame, area: Rect, mapper: &ACEvoSharedMemoryMapper) {
    let g = mapper.graphics();
    let gr = g.raw();

    let [fuel_area, net_area] =
        Layout::horizontal([Constraint::Fill(2), Constraint::Fill(1)]).areas(area);

    let [gauge_area, table_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(fuel_area);

    let fuel_pct = if gr.max_fuel > 0.0 {
        ((gr.fuel_liter_current_quantity / gr.max_fuel) * 100.0).clamp(0.0, 100.0) as u16
    } else {
        0
    };
    let fuel_label = format!(
        "{:.2} / {:.2} L  ({:.1}%)",
        gr.fuel_liter_current_quantity,
        gr.max_fuel,
        gr.fuel_liter_current_quantity_percent * 100.0
    );

    frame.render_widget(
        Gauge::default()
            .block(Block::bordered().title(" Fuel Level "))
            .gauge_style(Style::default().fg(Color::LightYellow))
            .percent(fuel_pct)
            .label(fuel_label),
        gauge_area,
    );

    let fuel_data: Vec<(&str, String)> = vec![
        ("Max tank", format!("{:.2} L", gr.max_fuel)),
        (
            "Current",
            format!(
                "{:.3} L  ({:.1}%)",
                gr.fuel_liter_current_quantity,
                gr.fuel_liter_current_quantity_percent * 100.0
            ),
        ),
        ("Used total", format!("{:.3} L", gr.fuel_liter_used)),
        (
            "Per lap (avg)",
            format!("{:.3} L/lap", gr.fuel_liter_per_lap),
        ),
        ("Target per lap", format!("{:.3} L/lap", gr.fuel_per_lap)),
        ("Per km (avg)", format!("{:.4} L/km", gr.fuel_liter_per_km)),
        (
            "Per km (now)",
            format!("{:.4} L/km", gr.instantaneous_fuel_liter_per_km),
        ),
        (
            "km per L (avg)",
            format!("{:.3} km/L", gr.km_per_fuel_liter),
        ),
        (
            "km per L (now)",
            format!("{:.3} km/L", gr.instantaneous_km_per_fuel_liter),
        ),
        (
            "Laps remaining",
            format!("{:.2}", gr.laps_possible_with_fuel),
        ),
        ("Laps estimated", format!("{:.2}", gr.fuel_estimated_laps)),
        ("Single compound", bool_str(gr.use_single_compound)),
    ];

    let fuel_rows: Vec<Row> = fuel_data
        .iter()
        .map(|(k, v)| {
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(fuel_rows, [Constraint::Length(18), Constraint::Fill(1)])
            .block(Block::bordered().title(" Fuel Calculator "))
            .column_spacing(2),
        table_area,
    );

    let net_data: Vec<(&str, String)> = vec![
        ("FPS", format!("{}", gr.player_fps)),
        ("FPS avg", format!("{}", gr.player_fps_avg)),
        ("Ping", format!("{} ms", gr.player_ping)),
        ("Latency", format!("{} ms", gr.player_latency)),
        ("CPU usage", format!("{}%", gr.player_cpu_usage)),
        ("CPU avg", format!("{}%", gr.player_cpu_usage_avg)),
        ("QoS", format!("{}", gr.player_qos)),
        ("QoS avg", format!("{}", gr.player_qos_avg)),
        ("Active cars", gr.active_cars.to_string()),
        ("Is online", bool_str(mapper.static_data().raw().is_online)),
        (
            "Disconnected",
            bool_str(gr.session_state.disconnected_from_server),
        ),
        (
            "Waiting",
            bool_str(gr.session_state.show_waiting_for_players),
        ),
    ];

    let net_rows: Vec<Row> = net_data
        .iter()
        .map(|(k, v)| {
            Row::new([
                Cell::from(*k).style(Style::default().fg(Color::Cyan)),
                Cell::from(v.as_str()),
            ])
        })
        .collect();

    frame.render_widget(
        Table::new(net_rows, [Constraint::Length(12), Constraint::Fill(1)])
            .block(Block::bordered().title(" Network & Performance "))
            .column_spacing(2),
        net_area,
    );
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn c_str(chars: &[i8]) -> &str {
    let bytes = unsafe { &*(chars as *const [i8] as *const [u8]) };
    let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    std::str::from_utf8(&bytes[..len]).unwrap_or("?")
}

fn fmt_ms(ms: i32) -> String {
    if ms <= 0 {
        return "--:--.---".to_owned();
    }
    let frac = ms % 1000;
    let total_s = ms / 1000;
    let secs = total_s % 60;
    let mins = total_s / 60;
    format!("{mins}:{secs:02}.{frac:03}")
}

fn fmt_seconds(secs: u32) -> String {
    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = secs / 3600;
    format!("{h:02}:{m:02}:{s:02}")
}

fn bool_str(b: bool) -> String {
    if b { "Yes".to_owned() } else { "No".to_owned() }
}

fn pit_action(v: i8) -> String {
    match v {
        -1 => "skip".to_owned(),
        0 => "done".to_owned(),
        1 => "in progress".to_owned(),
        n => n.to_string(),
    }
}

fn big_stat<'a>(value: &'a str, unit: &'a str, title: &'a str) -> Paragraph<'a> {
    Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            value,
            Style::default().bold().fg(Color::White),
        )),
        Line::from(Span::styled(unit, Style::default().fg(Color::DarkGray))),
    ])
    .alignment(Alignment::Center)
    .block(Block::bordered().title(title))
}

fn filled_bar(percent: u16, width: usize) -> String {
    let filled = (percent as usize * width / 100).min(width);
    format!("{}{}", "█".repeat(filled), "░".repeat(width - filled))
}

fn format_flag(flag: ACEvoFlagType) -> String {
    match flag {
        ACEvoFlagType::NoFlag => "None".to_owned(),
        other => format!("{other}"),
    }
}

fn wear_color(wear: f32) -> Color {
    if wear > 0.8 {
        Color::Green
    } else if wear > 0.5 {
        Color::Yellow
    } else if wear > 0.2 {
        Color::LightRed
    } else {
        Color::Red
    }
}

fn temp_color(temp: f32) -> Color {
    if temp < 70.0 {
        Color::Blue
    } else if temp < 85.0 {
        Color::Green
    } else if temp < 100.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}

// ─── Entry point ─────────────────────────────────────────────────────────────

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen, EnableMouseCapture)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(out))?;
    let result = App::new().run(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    result
}
