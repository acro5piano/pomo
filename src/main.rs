use clap::Parser;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    time::{Duration, Instant},
};
use tokio::time;

#[derive(Parser)]
#[command(name = "pomo")]
#[command(about = "A simple Pomodoro timer")]
struct Cli {}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum TimerPhase {
    Work,
    Break,
}

#[derive(Serialize, Deserialize)]
struct TimerState {
    phase: TimerPhase,
    remaining_seconds: u32,
    is_paused: bool,
    last_update: Option<u64>,
}

impl Default for TimerState {
    fn default() -> Self {
        Self {
            phase: TimerPhase::Work,
            remaining_seconds: 25 * 60, // 25 minutes
            is_paused: false,
            last_update: None,
        }
    }
}

impl TimerState {
    fn work_duration() -> u32 {
        25 * 60 // 25 minutes
    }

    fn break_duration() -> u32 {
        5 * 60 // 5 minutes
    }

    fn reset_to_work(&mut self) {
        self.phase = TimerPhase::Work;
        self.remaining_seconds = Self::work_duration();
        self.is_paused = false;
        self.last_update = None;
    }

    fn reset_to_break(&mut self) {
        self.phase = TimerPhase::Break;
        self.remaining_seconds = Self::break_duration();
        self.is_paused = false;
        self.last_update = None;
    }

    fn format_time(&self) -> String {
        let minutes = self.remaining_seconds / 60;
        let seconds = self.remaining_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    fn emoji(&self) -> &'static str {
        match self.phase {
            TimerPhase::Work => "🍅",
            TimerPhase::Break => "🌴",
        }
    }

    fn update(&mut self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if let Some(last_update) = self.last_update {
            if !self.is_paused {
                let elapsed = now - last_update;
                if elapsed > 0 {
                    if self.remaining_seconds > elapsed as u32 {
                        self.remaining_seconds -= elapsed as u32;
                    } else {
                        self.remaining_seconds = 0;
                    }
                }
            }
        }

        self.last_update = Some(now);
    }

    fn is_finished(&self) -> bool {
        self.remaining_seconds == 0
    }

    fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }
}

fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(".pomo.json")
}

fn load_state() -> TimerState {
    let config_path = get_config_path();
    if let Ok(contents) = fs::read_to_string(&config_path) {
        if let Ok(mut state) = serde_json::from_str::<TimerState>(&contents) {
            state.update();
            return state;
        }
    }
    TimerState::default()
}

fn save_state(state: &TimerState) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path();
    let contents = serde_json::to_string_pretty(state)?;
    fs::write(&config_path, contents)?;
    Ok(())
}

fn show_notification(message: &str) {
    let _ = Notification::new()
        .summary("Pomodoro Timer")
        .body(message)
        .show();
}

async fn run_timer() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = load_state();
    let mut last_save = Instant::now();
    let save_interval = Duration::from_secs(5);

    enable_raw_mode()?;
    let mut stdout = io::stdout();

    loop {
        // Clear screen and move cursor to top
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        // Update state
        state.update();

        // Display timer
        println!("{} {}", state.format_time(), state.emoji());
        println!();
        if state.is_paused {
            println!("PAUSED - Press 'r' to resume, 'q' to quit");
        } else {
            println!("Press 'p' to pause, 'q' to quit");
        }

        stdout.flush()?;

        // Check if timer finished
        if state.is_finished() {
            match state.phase {
                TimerPhase::Work => {
                    show_notification("Work session completed! Time for a break.");
                    state.reset_to_break();
                }
                TimerPhase::Break => {
                    show_notification("Break time over! Ready for work?");
                    state.reset_to_work();
                }
            }
        }

        // Save state periodically
        if last_save.elapsed() >= save_interval {
            save_state(&state)?;
            last_save = Instant::now();
        }

        // Check for input (non-blocking)
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('p') if !state.is_paused => {
                        state.toggle_pause();
                    }
                    KeyCode::Char('r') if state.is_paused => {
                        state.toggle_pause();
                    }
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        // Wait a bit before next update
        time::sleep(Duration::from_millis(100)).await;
    }

    disable_raw_mode()?;
    save_state(&state)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _cli = Cli::parse();

    // Set up Ctrl+C handler
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        original_hook(panic_info);
    }));

    if let Err(e) = run_timer().await {
        disable_raw_mode()?;
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
