# Pomo 🍅

A simple and elegant Pomodoro Timer built in Rust for the command line.

## Features

- **🍅 Classic Pomodoro Technique**: 25-minute work sessions followed by 5-minute breaks
- **💾 State Persistence**: Resume your timer session even after closing the application
- **⏸️ Pause/Resume**: Control your timer with simple keyboard shortcuts
- **🔔 Desktop Notifications**: Get notified when work/break sessions complete (Linux)
- **🎨 Visual Interface**: Clean terminal UI with emoji indicators and real-time countdown
- **⚡ Fast & Lightweight**: Built in Rust for optimal performance

## Installation

### From Source

```bash
git clone https://github.com/acro5piano/pomo.git
cd pomo
cargo build --release
cargo install --path .
```

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Linux (for desktop notifications)

## Usage

### Start a Pomodoro Session

```bash
pomo
```

This will start a 25-minute work session:

```
25:00 🍅
```

### Controls

During a session, you can:

- **`p`** - Pause the timer
- **`r`** - Resume a paused timer  
- **`q`** - Quit and save current state
- **`Ctrl+C`** - Quit and save current state

### Session Flow

1. **Work Phase** (25:00): Focus time with 🍅 indicator
2. **Break Phase** (05:00): Rest time with 🌴 indicator
3. **Auto-transition**: Automatically switches between work and break
4. **Notifications**: Desktop alerts when each phase completes

### State Persistence

Your timer state is automatically saved to `~/.pomo.json`. This means:

- ✅ Resume where you left off after closing the app
- ✅ Maintain progress across system restarts
- ✅ Never lose your current session

## How It Works

The Pomodoro Technique is a time management method:

1. **Work for 25 minutes** - Focus on a single task
2. **Take a 5-minute break** - Rest and recharge  
3. **Repeat the cycle** - Build sustained productivity

Pomo handles the timing automatically, so you can focus on your work.

## Configuration

The timer state is stored in `~/.pomo.json` with the following structure:

```json
{
  "phase": "Work",
  "remaining_seconds": 1500,
  "is_paused": false,
  "last_update": 1234567890
}
```

## Dependencies

- **clap** - Command line argument parsing
- **crossterm** - Cross-platform terminal manipulation
- **serde/serde_json** - JSON serialization for state persistence
- **tokio** - Async runtime
- **notify-rust** - Desktop notifications (Linux)

## Development

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Test

```bash
cargo test
```

### Release Build

```bash
cargo build --release
```

## Supported Platforms

- ✅ **Linux** - Full feature support including notifications
- ⚠️ **macOS** - Core functionality (notifications may require additional setup)
- ⚠️ **Windows** - Core functionality (notifications may require additional setup)

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Inspiration

Built with inspiration from the original Pomodoro Technique developed by Francesco Cirillo. This implementation focuses on simplicity and productivity for developers who live in the terminal.

---

**Happy focusing! 🍅**